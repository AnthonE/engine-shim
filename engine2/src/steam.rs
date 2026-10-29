//! Steam's part, without Steam: a game server logs on to nobody, players are
//! who they prove they are, and the answers come back as Steam's callbacks
//! would (Steamworks.Dispatch.OnClientCallback), when managed runs them.
//!
//! Identity is a u64 where Steam's is a SteamID: `ENGINE2_IDENTITY`, else
//! the address the running Elo Pros launcher holds (its first 8 bytes), else
//! one made up at start.
//!
//! A ticket is what a player hands a server to prove who they are. The
//! server sends each connection a nonce when it welcomes it (`challenge`);
//! the player's launcher signs it in with Ethereum (`prove`, EIP-4361) for
//! the address they dialled, and that proof is the ticket. `verify` checks
//! it: the signer, the domain, the nonce (once), its age, and that the id
//! claimed is the signer's. With no launcher a player has a guest ticket,
//! which a server takes at its word unless `ENGINE2_SIWE_DOMAINS` names the
//! domains it answers to: then only a proof gets in.
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use crate::elo_overlay::Overlay;
use crate::siwe;

/// CallbackType, Steamworks' numbering.
pub const STEAM_SERVERS_CONNECTED: i32 = 101;
pub const VALIDATE_AUTH_TICKET_RESPONSE: i32 = 143;

/// EBeginAuthSessionResult
pub const BEGIN_AUTH_OK: i64 = 0;
pub const BEGIN_AUTH_INVALID_TICKET: i64 = 1;
pub const BEGIN_AUTH_DUPLICATE: i64 = 2;

/// EAuthSessionResponse
pub const AUTH_OK: i32 = 0;
pub const AUTH_TICKET_USED: i32 = 7;
pub const AUTH_TICKET_INVALID: i32 = 8;

/// The prefix of a guest's ticket.
const TICKET: &str = "engine-shim/1 ";
/// How far a proof's `Issued At` may be from our clock (the launcher's own bound).
const SKEW: i64 = 15 * 60;
/// How long a nonce waits for its proof.
const CHALLENGE_LIFE: Duration = Duration::from_secs(10 * 60);

/// The launcher, if one is running: the game's slug is what it signs for.
fn launcher() -> Option<Overlay> {
    let game = std::env::var("ENGINE2_ELO_GAME").unwrap_or_else(|_| "engine-shim".into());
    Overlay::connect(&game, env!("CARGO_PKG_VERSION")).map_err(|e| crate::trace::note(&format!("steam: no elo launcher: {e}"))).ok()
}

/// This process's identity, as peers and tickets name it.
pub fn identity() -> u64 {
    static ID: OnceLock<u64> = OnceLock::new();
    *ID.get_or_init(|| {
        if let Some(id) = std::env::var("ENGINE2_IDENTITY").ok().and_then(|v| v.trim().parse().ok()) {
            return id;
        }
        let address = launcher().and_then(|mut l| l.address());
        if let Some(a) = address.as_deref().and_then(|a| siwe::parse_address(a)) {
            crate::trace::note(&format!("steam: we are {}", siwe::hex(&a)));
            return siwe::id_of(&a);
        }
        // made up: the clock and the process, in the individual-account range
        // SteamIDs use, so nothing reads it as an anonymous server
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
        let r = (t.as_nanos() as u64 ^ ((std::process::id() as u64) << 20)) & 0xffff_ffff;
        0x0110_0001_0000_0000 | r.max(1)
    })
}

/// A ticket for the server that sent `challenge` (the address we dialled,
/// its nonce): the launcher's proof, or a guest's ticket.
pub fn ticket(challenge: Option<(String, String)>) -> Vec<u8> {
    if let Ok(t) = std::env::var("ENGINE2_TICKET") {
        if !t.is_empty() { return t.into_bytes(); }
    }
    if let Some((server, nonce)) = challenge {
        if let Some(t) = launcher().and_then(|l| proved(l, &server, &nonce)) {
            return t;
        }
    }
    format!("{TICKET}{}", identity()).into_bytes()
}

/// The launcher's sign-in for `server`, over its nonce, as a ticket.
fn proved(mut launcher: Overlay, server: &str, nonce: &str) -> Option<Vec<u8>> {
    match launcher.prove(server, nonce) {
        Ok(p) => Some(siwe::ticket(&p.message, &p.signature)),
        Err(e) => {
            crate::trace::note(&format!("steam: the launcher didn't prove us to {server}: {e}"));
            None
        }
    }
}

/// The domains this server answers to (`ENGINE2_SIWE_DOMAINS`, commas):
/// none means a development server, which takes guests.
fn domains() -> &'static [String] {
    static D: OnceLock<Vec<String>> = OnceLock::new();
    D.get_or_init(|| {
        std::env::var("ENGINE2_SIWE_DOMAINS").unwrap_or_default().split(',').map(|d| d.trim().to_ascii_lowercase()).filter(|d| !d.is_empty()).collect()
    })
}

/// Who a ticket proves `claimed` is: the owning account, or the
/// EAuthSessionResponse that says why not.
pub fn verify(claimed: u64, ticket: &[u8]) -> Result<u64, i32> {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() as i64;
    verify_with(claimed, ticket, domains(), now)
}

fn verify_with(claimed: u64, ticket: &[u8], domains: &[String], now: i64) -> Result<u64, i32> {
    if let Some((message, signature)) = siwe::parse_ticket(ticket) {
        return verify_proof(claimed, &message, &signature, domains, now);
    }
    if !domains.is_empty() {
        return Err(AUTH_TICKET_INVALID); // a guest, where only a proof gets in
    }
    static OPEN: AtomicBool = AtomicBool::new(false);
    if !OPEN.swap(true, Ordering::Relaxed) {
        crate::trace::note("steam: guests are taken at their word (no ENGINE2_SIWE_DOMAINS: a development server)");
    }
    let t = std::str::from_utf8(ticket).map_err(|_| AUTH_TICKET_INVALID)?;
    match t.strip_prefix(TICKET).map(|id| id.trim().parse::<u64>()) {
        Some(Ok(id)) if id == claimed => Ok(claimed),
        Some(_) => Err(AUTH_TICKET_INVALID),
        // someone else's ticket (ENGINE2_TICKET): taken
        None if !t.is_empty() => Ok(claimed),
        None => Err(AUTH_TICKET_INVALID),
    }
}

/// A SIWE proof: every field we rely on checked against what we know.
fn verify_proof(claimed: u64, message: &str, signature: &[u8], domains: &[String], now: i64) -> Result<u64, i32> {
    let refuse = |why: &str| {
        crate::trace::note(&format!("steam: {claimed}'s proof refused: {why}"));
        AUTH_TICKET_INVALID
    };
    let signer = siwe::signer(message, signature).ok_or_else(|| refuse("not a signature"))?;
    let f = siwe::fields(message).ok_or_else(|| refuse("not a sign-in message"))?;
    if f.address != signer { return Err(refuse("signed by someone else than it names")); }
    if siwe::id_of(&signer) != claimed { return Err(refuse("for another player")); }
    if !domains.is_empty() {
        // the domain line names us, or the site a browser wallet signed on;
        // the URI names us either way
        let uri = f.uri.strip_prefix("https://").unwrap_or("").split('/').next().unwrap_or("").to_ascii_lowercase();
        if !domains.contains(&f.domain.to_ascii_lowercase()) || !domains.contains(&uri) {
            return Err(refuse(&format!("for {} ({}), not here", f.domain, f.uri)));
        }
    }
    if (now - f.issued_at).abs() > SKEW { return Err(refuse("too old, or from the future")); }
    // last, so a bad proof doesn't spend the nonce: one we issued, to this
    // player's connection, once
    let mut c = challenges().lock().unwrap();
    match c.get(&f.nonce) {
        None => {
            drop(c);
            crate::trace::note(&format!("steam: {claimed}'s proof refused: a nonce not ours, or used"));
            Err(AUTH_TICKET_USED)
        }
        Some(ch) if ch.peer != claimed => { drop(c); Err(refuse("a nonce we gave someone else")) }
        Some(_) => {
            c.remove(&f.nonce);
            crate::trace::note(&format!("steam: {claimed} is {}", siwe::hex(&signer)));
            Ok(claimed)
        }
    }
}

// ---- challenges: a nonce for each connection welcomed, until its proof

struct Challenge {
    peer: u64,
    at: Instant,
}

fn challenges() -> &'static Mutex<HashMap<String, Challenge>> {
    static C: OnceLock<Mutex<HashMap<String, Challenge>>> = OnceLock::new();
    C.get_or_init(Default::default)
}

/// A fresh nonce for the player `peer` says it is: 32 hex digits, which
/// EIP-4361's alphabet takes.
pub fn challenge(peer: u64) -> String {
    use std::hash::{BuildHasher, Hasher};
    static N: AtomicU32 = AtomicU32::new(0);
    // SipHash under keys the OS chose for this process: unpredictable outside it
    let s = std::collections::hash_map::RandomState::new();
    let n = N.fetch_add(1, Ordering::Relaxed);
    let half = |salt: u8| {
        let mut h = s.build_hasher();
        h.write_u32(n);
        h.write_u8(salt);
        h.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos());
        h.finish()
    };
    let nonce = format!("{:016x}{:016x}", half(0), half(1));
    let mut c = challenges().lock().unwrap();
    c.retain(|_, ch| ch.at.elapsed() < CHALLENGE_LIFE);
    c.insert(nonce.clone(), Challenge { peer, at: Instant::now() });
    nonce
}

// ---- the game server

static LOGGED_ON: AtomicBool = AtomicBool::new(false);

pub fn log_on() {
    if !LOGGED_ON.swap(true, Ordering::Relaxed) {
        queue(STEAM_SERVERS_CONNECTED, vec![0], true); // an empty struct: one byte, as managed sizes it
    }
}

pub fn log_off() { LOGGED_ON.store(false, Ordering::Relaxed) }

pub fn logged_on() -> bool { LOGGED_ON.load(Ordering::Relaxed) }

// ---- auth sessions: one per player the server is checking or has checked

fn sessions() -> &'static Mutex<HashSet<u64>> {
    static S: OnceLock<Mutex<HashSet<u64>>> = OnceLock::new();
    S.get_or_init(Default::default)
}

/// A player's ticket, checked: the answer is a ValidateAuthTicketResponse_t,
/// queued for the next callbacks.
pub fn begin_auth(player: u64, ticket: &[u8], server: bool) -> i64 {
    if ticket.is_empty() { return BEGIN_AUTH_INVALID_TICKET; }
    if !sessions().lock().unwrap().insert(player) { return BEGIN_AUTH_DUPLICATE; }
    let (response, owner) = match verify(player, ticket) {
        Ok(owner) => (AUTH_OK, owner),
        Err(r) => (r, 0),
    };
    crate::trace::note(&format!("steam: {player}'s ticket: {}", if response == AUTH_OK { "ok".into() } else { format!("refused ({response})") }));
    queue(VALIDATE_AUTH_TICKET_RESPONSE, validate_response(player, response, owner), server);
    BEGIN_AUTH_OK
}

pub fn end_auth(player: u64) {
    sessions().lock().unwrap().remove(&player);
}

/// ValidateAuthTicketResponse_t, Pack 4: SteamID, the response, the owner.
fn validate_response(player: u64, response: i32, owner: u64) -> Vec<u8> {
    let mut b = player.to_le_bytes().to_vec();
    b.extend_from_slice(&response.to_le_bytes());
    b.extend_from_slice(&owner.to_le_bytes());
    b
}

/// Ticket handles, as GetAuthSessionTicket gives them out.
pub fn next_ticket_handle() -> u32 {
    static N: AtomicU32 = AtomicU32::new(0);
    N.fetch_add(1, Ordering::Relaxed) + 1
}

// ---- callbacks

struct Callback {
    kind: i32,
    data: Vec<u8>,
    server: bool,
}

fn callbacks() -> &'static Mutex<VecDeque<Callback>> {
    static Q: OnceLock<Mutex<VecDeque<Callback>>> = OnceLock::new();
    Q.get_or_init(Default::default)
}

fn queue(kind: i32, data: Vec<u8>, server: bool) {
    callbacks().lock().unwrap().push_back(Callback { kind, data, server });
}

/// The callbacks queued for the server's or the client's API, each handed
/// to `dispatch(kind, data, size, server)`: Steam's RunCallbacks.
pub fn run_callbacks(server: bool, mut dispatch: impl FnMut(i32, *const u8, i32, bool)) {
    loop {
        let next = {
            let mut q = callbacks().lock().unwrap();
            let i = q.iter().position(|c| c.server == server);
            i.and_then(|i| q.remove(i))
        };
        let Some(c) = next else { break };
        dispatch(c.kind, c.data.as_ptr(), c.data.len() as i32, c.server);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tickets() {
        let me = identity();
        assert_eq!(verify(me, &ticket(None)), Ok(me));
        assert_eq!(verify(me + 1, &ticket(None)), Err(AUTH_TICKET_INVALID));
        assert_eq!(verify(me, b""), Err(AUTH_TICKET_INVALID));
        assert_eq!(verify(me, b"engine-shim/1 nonsense"), Err(AUTH_TICKET_INVALID));
    }

    /// A proof from the launcher for `domain`, over `nonce`, issued at `at`.
    fn proof(key: u8, domain: &str, nonce: &str, at: &str) -> (u64, Vec<u8>) {
        let (sk, a) = siwe::test_key(key);
        let m = siwe::test_message(domain, &siwe::hex(&a), nonce, at);
        (siwe::id_of(&a), siwe::ticket(&m, &siwe::test_sign(&sk, &m)))
    }

    const AT: &str = "2026-08-07T12:00:00Z";
    const NOW: i64 = 1_786_104_030;

    #[test]
    fn a_sign_in_proves_a_player_once() {
        let here = vec!["shard.example:27015".to_string()];
        let (_, a) = siwe::test_key(3);
        let me = siwe::id_of(&a);
        let nonce = challenge(me);
        let (id, t) = proof(3, "shard.example:27015", &nonce, AT);
        assert_eq!(id, me);
        assert_eq!(verify_with(me, &t, &here, NOW), Ok(me));
        // the same proof again: replayed
        assert_eq!(verify_with(me, &t, &here, NOW), Err(AUTH_TICKET_USED));
        // a development server checks a proof all the same, but for its domain
        let nonce = challenge(me);
        let (_, t) = proof(3, "elsewhere.example", &nonce, AT);
        assert_eq!(verify_with(me, &t, &[], NOW), Ok(me));
    }

    /// A launcher as elo-broker answers: hello, then `prove` signed by `key`.
    fn launcher_at(path: &std::path::Path, key: u8) {
        use std::io::{BufRead, BufReader, Write};
        let l = std::os::unix::net::UnixListener::bind(path).unwrap();
        std::thread::spawn(move || {
            let (s, _) = l.accept().unwrap();
            let mut w = s.try_clone().unwrap();
            for line in BufReader::new(s).lines() {
                let line = line.unwrap();
                let reply = if line.contains(r#""op":"hello""#) {
                    r#"{"ok":true,"protocol":1,"launcher":"elo-launcher","signer":"local"}"#.to_string()
                } else if line.contains(r#""op":"prove""#) {
                    let field = |k: &str| line.split(&format!(r#""{k}":""#)).nth(1).unwrap().split('"').next().unwrap().to_string();
                    let (sk, a) = siwe::test_key(key);
                    let m = siwe::test_message(&field("server"), &siwe::hex(&a), &field("nonce"), AT);
                    format!(r#"{{"ok":true,"signature":"{}","address":"{}","scheme":"eip4361","message":"{}"}}"#, siwe::test_sign(&sk, &m), siwe::hex(&a), m.replace('\n', "\\n"))
                } else {
                    r#"{"ok":false,"reason":"not here"}"#.to_string()
                };
                writeln!(w, "{reply}").unwrap();
            }
        });
    }

    #[test]
    fn the_launcher_signs_a_player_in() {
        let dir = std::env::temp_dir().join(format!("engine2-launcher-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("launcher.sock");
        let _ = std::fs::remove_file(&path);
        launcher_at(&path, 9);
        let ov = Overlay::connect_at(&path, "engine-shim", "0", Duration::from_secs(5)).unwrap();
        let me = siwe::id_of(&siwe::test_key(9).1);
        let nonce = challenge(me);
        let t = proved(ov, "127.0.0.1:27015", &nonce).unwrap();
        assert_eq!(verify_with(me, &t, &["127.0.0.1:27015".into()], NOW), Ok(me));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn proofs_refused() {
        let here = vec!["shard.example".to_string()];
        let (_, a) = siwe::test_key(4);
        let me = siwe::id_of(&a);
        let nonce = challenge(me);
        let refused = |t: &[u8], claimed| verify_with(claimed, t, &here, NOW);
        // for another server
        assert_eq!(refused(&proof(4, "evil.example", &nonce, AT).1, me), Err(AUTH_TICKET_INVALID));
        // stale
        assert_eq!(refused(&proof(4, "shard.example", &nonce, "2026-08-06T12:00:00Z").1, me), Err(AUTH_TICKET_INVALID));
        // claiming to be someone else
        assert_eq!(refused(&proof(4, "shard.example", &nonce, AT).1, me ^ 1), Err(AUTH_TICKET_INVALID));
        // someone else's key, signing for a nonce issued to us
        let (other, t) = proof(5, "shard.example", &nonce, AT);
        assert_eq!(refused(&t, other), Err(AUTH_TICKET_INVALID));
        // a nonce we never issued
        assert_eq!(refused(&proof(4, "shard.example", "0123456789abcdef", AT).1, me), Err(AUTH_TICKET_USED));
        // a guest, where only proofs get in
        assert_eq!(refused(format!("{TICKET}{me}").as_bytes(), me), Err(AUTH_TICKET_INVALID));
        // a forged signature: the message's address isn't the signer's
        let (sk, _) = siwe::test_key(6);
        let m = siwe::test_message("shard.example", &siwe::hex(&a), &nonce, AT);
        assert_eq!(refused(&siwe::ticket(&m, &siwe::test_sign(&sk, &m)), me), Err(AUTH_TICKET_INVALID));
        // and after all that, the nonce is still good for the real proof
        assert_eq!(refused(&proof(4, "shard.example", &nonce, AT).1, me), Ok(me));
    }

    #[test]
    fn a_player_is_checked_once_and_answered() {
        let player = 0x0110_0001_0000_0042;
        let t = format!("{TICKET}{player}");
        assert_eq!(begin_auth(player, t.as_bytes(), true), BEGIN_AUTH_OK);
        assert_eq!(begin_auth(player, t.as_bytes(), true), BEGIN_AUTH_DUPLICATE);
        let mut got = Vec::new();
        run_callbacks(true, |k, d, n, s| got.push((k, unsafe { core::slice::from_raw_parts(d, n as usize) }.to_vec(), s)));
        let mine: Vec<_> = got.iter().filter(|(k, d, _)| *k == VALIDATE_AUTH_TICKET_RESPONSE && d[..8] == player.to_le_bytes()).collect();
        assert_eq!(mine.len(), 1);
        let (_, d, server) = mine[0];
        assert!(*server);
        assert_eq!(d.len(), 20);
        assert_eq!(i32::from_le_bytes(d[8..12].try_into().unwrap()), AUTH_OK);
        assert_eq!(u64::from_le_bytes(d[12..20].try_into().unwrap()), player);
        end_auth(player);
        assert_eq!(begin_auth(player, b"engine-shim/1 7", true), BEGIN_AUTH_OK);
        let mut refused = None;
        run_callbacks(true, |k, d, n, _| if k == VALIDATE_AUTH_TICKET_RESPONSE {
            let d = unsafe { core::slice::from_raw_parts(d, n as usize) };
            if d[..8] == player.to_le_bytes() { refused = Some(i32::from_le_bytes(d[8..12].try_into().unwrap())) }
        });
        assert_eq!(refused, Some(AUTH_TICKET_INVALID));
        end_auth(player);
    }
}
