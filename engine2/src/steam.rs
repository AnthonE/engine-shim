//! Steam's part, without Steam: a game server logs on to nobody, players are
//! who their tickets say, and the answers come back as Steam's callbacks
//! would (Steamworks.Dispatch.OnClientCallback), when managed runs them.
//!
//! Identity is a u64 where Steam's is a SteamID: `ENGINE2_IDENTITY`, or one
//! made up at start. A ticket is what a client hands the server to prove
//! who it is (`ENGINE2_TICKET`, or one made here naming our identity);
//! `verify` decides. For now it checks only that a ticket made here names
//! the player it came with, and takes any other at its word: a development
//! server's rule, and the place a real sign-in (the Elo Pros wallet's) goes.
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

/// CallbackType, Steamworks' numbering.
pub const STEAM_SERVERS_CONNECTED: i32 = 101;
pub const VALIDATE_AUTH_TICKET_RESPONSE: i32 = 143;

/// EBeginAuthSessionResult
pub const BEGIN_AUTH_OK: i64 = 0;
pub const BEGIN_AUTH_INVALID_TICKET: i64 = 1;
pub const BEGIN_AUTH_DUPLICATE: i64 = 2;

/// EAuthSessionResponse
pub const AUTH_OK: i32 = 0;
pub const AUTH_TICKET_INVALID: i32 = 8;

/// The prefix of the tickets made here.
const TICKET: &str = "engine-shim/1 ";

/// This process's identity, as peers and tickets name it.
pub fn identity() -> u64 {
    static ID: OnceLock<u64> = OnceLock::new();
    *ID.get_or_init(|| {
        if let Some(id) = std::env::var("ENGINE2_IDENTITY").ok().and_then(|v| v.trim().parse().ok()) {
            return id;
        }
        // made up: the clock and the process, in the individual-account range
        // SteamIDs use, so nothing reads it as an anonymous server
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
        let r = (t.as_nanos() as u64 ^ ((std::process::id() as u64) << 20)) & 0xffff_ffff;
        0x0110_0001_0000_0000 | r.max(1)
    })
}

/// A ticket for this process's identity: what `verify` takes.
pub fn ticket() -> Vec<u8> {
    match std::env::var("ENGINE2_TICKET") {
        Ok(t) if !t.is_empty() => t.into_bytes(),
        _ => format!("{TICKET}{}", identity()).into_bytes(),
    }
}

/// Who a ticket proves `claimed` is: the owning account, or the
/// EAuthSessionResponse that says why not.
pub fn verify(claimed: u64, ticket: &[u8]) -> Result<u64, i32> {
    static OPEN: AtomicBool = AtomicBool::new(false);
    if !OPEN.swap(true, Ordering::Relaxed) {
        crate::trace::note("steam: tickets are taken at their word (a development server's rule)");
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
        assert_eq!(verify(me, &ticket()), Ok(me));
        assert_eq!(verify(me + 1, &ticket()), Err(AUTH_TICKET_INVALID));
        assert_eq!(verify(me, b""), Err(AUTH_TICKET_INVALID));
        assert_eq!(verify(me, b"engine-shim/1 nonsense"), Err(AUTH_TICKET_INVALID));
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
