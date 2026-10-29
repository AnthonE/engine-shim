//! Elo Pros, the server's side: what a game server owes the store
//! (elo-pros-launcher's docs/ROOMS-SERVER.md §4, LAUNCHER.md §6), as Gates'
//! shards do it.
//!
//! - **The shard list.** A row (`elo-shardlist-v1`) naming this server, its
//!   address, map and cap, written to `ENGINE2_SHARDLIST` whenever managed
//!   changes them (the Steam game server calls s&box already makes). The
//!   operator drops it where the origin serves it,
//!   `$SCRY_DEPOTS_DIR/<slug>/servers.json`. Needs `ENGINE2_PUBLIC_ADDR`
//!   (`host:port`): an address we can't know from in here.
//! - **Status.** `GET /status.json` on `ENGINE2_STATUS_ADDR`, plain HTTP:
//!   `{"players":3,"max_players":8,"tick":123456}`, and beside it
//!   `GET /online.json`, who is signed in and since when, which the store's
//!   playtime and friends' presence read. The row's `status_url`
//!   (`ENGINE2_STATUS_URL`) names where the world reaches them.
//! - **The ticket.** A paid title's copy is a token; `ENGINE2_ELO_ORIGIN`
//!   (`https://elopros.com`) arms the check. At join a signed-in player's
//!   wallet is asked about (`GET /api/ticket/{slug}/of/{wallet}`), and the
//!   roster on an interval (`POST /api/ticket/{slug}/check`), because a copy
//!   can be sold mid-session. Three answers, never two: only a definite
//!   `entitled: false` refuses or kicks; "could not look" keeps the player.
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

/// Engine frames run: `status.json`'s tick, and its liveness.
pub static TICK: AtomicU64 = AtomicU64::new(0);

const TIMEOUT: Duration = Duration::from_secs(6);
const SWEEP: Duration = Duration::from_secs(120);
/// The far end's page cap (meter/tickets.py's CHECK_MAX_WALLETS).
pub const CHECK_MAX_WALLETS: usize = 100;
/// A response past this is refused, never truncated: a truncated page reads
/// as fewer wallets checked.
const MAX_RESPONSE_BYTES: usize = 64 * 1024;

fn env(k: &str) -> Option<String> {
    std::env::var(k).ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

/// The title's slug in the catalog: what the launcher signs for, too.
pub fn slug() -> String {
    env("ENGINE2_ELO_GAME").unwrap_or_else(|| "engine-shim".into())
}

// ---- the server, as managed describes it

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Server {
    pub name: String,
    pub map: String,
    pub max_players: i32,
}

fn server() -> &'static Mutex<Server> {
    static S: OnceLock<Mutex<Server>> = OnceLock::new();
    S.get_or_init(Default::default)
}

/// A change managed made (ISteamGameServer_Set…): the row follows it.
pub fn describe(change: impl FnOnce(&mut Server)) {
    let now = {
        let mut s = server().lock().unwrap();
        let before = s.clone();
        change(&mut s);
        (*s != before).then(|| s.clone())
    };
    if let (Some(s), Some(path), Some(addr)) = (now, env("ENGINE2_SHARDLIST"), env("ENGINE2_PUBLIC_ADDR")) {
        let doc = shard_row(&s, &addr, env("ENGINE2_STATUS_URL").as_deref());
        // whole or not at all: a reader never sees half a file
        let tmp = format!("{path}.tmp");
        if let Err(e) = std::fs::write(&tmp, doc).and_then(|_| std::fs::rename(&tmp, &path)) {
            crate::trace::note(&format!("elo: can't write the shard list to {path}: {e}"));
        }
    }
}

fn quote(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// This server as `elo-shardlist-v1`: one row. No `players`: a count in a
/// file is stale when written; `status_url` is asked instead.
pub fn shard_row(s: &Server, addr: &str, status_url: Option<&str>) -> String {
    let id = env("ENGINE2_SHARD_ID").unwrap_or_else(|| addr.to_string());
    let name = if s.name.is_empty() { addr } else { &s.name };
    let mut row = format!("{{\"id\":{},\"name\":{},\"addr\":{}", quote(&id), quote(name), quote(addr));
    if s.max_players > 0 { row.push_str(&format!(",\"max_players\":{}", s.max_players)); }
    if !s.map.is_empty() { row.push_str(&format!(",\"map\":{}", quote(&s.map))); }
    if let Some(u) = status_url { row.push_str(&format!(",\"status_url\":{}", quote(u))); }
    format!("{{\"servers\":[{row}}}]}}\n")
}

/// `status.json`: an unknown cap is left out, never sent as 0.
pub fn status(players: usize, s: &Server, tick: u64) -> String {
    let max = if s.max_players > 0 { format!(",\"max_players\":{}", s.max_players) } else { String::new() };
    format!("{{\"players\":{players}{max},\"tick\":{tick}}}")
}

/// `online.json` (Gates' online.rs, read by scry's playtime.py): signed-in
/// wallets and when they came; a guest has no wallet, so no row.
pub fn online(at: i64, rows: &[(String, i64)]) -> String {
    let rows: Vec<String> = rows.iter().map(|(w, since)| format!("{{\"wallet\":{},\"since\":{since}}}", quote(w))).collect();
    format!("{{\"at\":{at},\"online\":[{}]}}", rows.join(","))
}

/// The status responder, once, if `ENGINE2_STATUS_ADDR` says where:
/// `players` counts the connections, `signed_in` who proved a wallet.
pub fn serve_status(players: fn() -> usize, signed_in: fn() -> Vec<(String, i64)>) {
    static ONCE: OnceLock<()> = OnceLock::new();
    let Some(addr) = env("ENGINE2_STATUS_ADDR") else { return };
    ONCE.get_or_init(|| {
        let l = match TcpListener::bind(&addr) {
            Ok(l) => l,
            Err(e) => return crate::trace::note(&format!("elo: can't serve status on {addr}: {e}")),
        };
        crate::trace::note(&format!("elo: status on http://{}/status.json", l.local_addr().map(|a| a.to_string()).unwrap_or(addr)));
        std::thread::Builder::new().name("elo status".into()).spawn(move || {
            for c in l.incoming().flatten() {
                let _ = c.set_read_timeout(Some(Duration::from_secs(2)));
                let mut line = String::new();
                let _ = BufReader::new(&c).read_line(&mut line);
                let (code, body) = if line.starts_with("GET /status.json ") {
                    ("200 OK", status(players(), &server().lock().unwrap(), TICK.load(Ordering::Relaxed)))
                } else if line.starts_with("GET /online.json ") {
                    let at = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
                    ("200 OK", online(at, &signed_in()))
                } else {
                    ("404 Not Found", "{}".to_string())
                };
                let _ = write!(&c, "HTTP/1.1 {code}\r\ncontent-type: application/json\r\ncontent-length: {}\r\naccess-control-allow-origin: *\r\nconnection: close\r\n\r\n{body}", body.len());
            }
        }).expect("elo: a thread");
    });
}

/// The link a player pastes to come here (LAUNCHER.md §6b), said once.
pub fn announce() {
    static ONCE: OnceLock<()> = OnceLock::new();
    if let Some(addr) = env("ENGINE2_PUBLIC_ADDR") {
        ONCE.get_or_init(|| {
            let line = format!("elo: players join with elo://join/{}/{addr}", slug());
            eprintln!("engine2: {line}");
            crate::trace::note(&line);
        });
    }
}

// ---- the ticket

/// What the origin said about one wallet. No bool: "the chain says zero"
/// and "we could not ask" are opposite facts down one wire.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// `entitled: true`, or the title sells no ticket.
    Owns,
    /// `entitled: false`: the only answer that may cost a player their seat.
    Nope,
    /// `entitled: null`, a failure, a timeout, a malformed body.
    Unknown,
}

/// The origin that answers, if the check is armed.
pub fn origin() -> Option<String> {
    env("ENGINE2_ELO_ORIGIN").map(|o| o.trim_end_matches('/').to_string())
}

fn is_wallet(w: &str) -> bool {
    w.len() == 42 && w.starts_with("0x") && w[2..].bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder().timeout_global(Some(TIMEOUT)).build().into()
}

fn read(mut res: ureq::http::Response<ureq::Body>) -> Option<String> {
    res.body_mut().with_config().limit(MAX_RESPONSE_BYTES as u64 + 1).read_to_string().ok().filter(|b| b.len() <= MAX_RESPONSE_BYTES)
}

/// `true`, `false` or `null` after `"key":`, else None.
fn tri(body: &str, key: &str) -> Option<Option<bool>> {
    let at = body.find(key)? + key.len();
    let rest = body[at..].trim_start().strip_prefix(':')?.trim_start();
    if rest.starts_with("true") { Some(Some(true)) } else if rest.starts_with("false") { Some(Some(false)) } else if rest.starts_with("null") { Some(None) } else { None }
}

fn verdict(v: Option<Option<bool>>) -> Verdict {
    match v {
        Some(Some(true)) => Verdict::Owns,
        Some(Some(false)) => Verdict::Nope,
        _ => Verdict::Unknown,
    }
}

/// One wallet, at join. Blocking: never on the main thread.
pub fn check_one(origin: &str, slug: &str, wallet: &str) -> Verdict {
    if !is_wallet(wallet) { return Verdict::Unknown; }
    match agent().get(&format!("{origin}/api/ticket/{slug}/of/{wallet}")).call().ok().and_then(read) {
        Some(body) => verdict(tri(&body, "\"entitled\"")),
        None => Verdict::Unknown,
    }
}

/// The roster in one call: a verdict per wallet, in order. A wallet the far
/// end didn't answer about is Unknown, never dropped.
pub fn check_many(origin: &str, slug: &str, wallets: &[String]) -> Vec<Verdict> {
    let unknown = vec![Verdict::Unknown; wallets.len()];
    if wallets.is_empty() || wallets.len() > CHECK_MAX_WALLETS || !wallets.iter().all(|w| is_wallet(w)) {
        return unknown;
    }
    let payload = format!("{{\"wallets\":[{}]}}", wallets.iter().map(|w| quote(w)).collect::<Vec<_>>().join(","));
    let Some(body) = agent().post(&format!("{origin}/api/ticket/{slug}/check")).header("content-type", "application/json").send(payload.as_str()).ok().and_then(read) else {
        return unknown;
    };
    if tri(&body, "\"every_wallet_entitled\"") == Some(Some(true)) {
        return vec![Verdict::Owns; wallets.len()];
    }
    let Some(map) = body.find("\"wallets\"").map(|at| &body[at..]) else { return unknown };
    wallets
        .iter()
        .map(|w| match map.find(&quote(w)) {
            Some(at) => verdict(tri(&map[at..], "\"entitled\"")),
            None => Verdict::Unknown,
        })
        .collect()
}

/// The sweep, once, if the check is armed: every `SWEEP`, the signed-in
/// roster (player, wallet) is asked about, and `kick` gets each player whose
/// copy is definitely gone.
pub fn sweep(roster: fn() -> Vec<(u64, String)>, kick: fn(u64)) {
    static ONCE: OnceLock<()> = OnceLock::new();
    let Some(origin) = origin() else { return };
    ONCE.get_or_init(|| {
        let slug = slug();
        crate::trace::note(&format!("elo: copies of {slug} checked at {origin}, the roster every {}s", SWEEP.as_secs()));
        std::thread::Builder::new().name("elo sweep".into()).spawn(move || loop {
            std::thread::sleep(SWEEP);
            let r = roster();
            for page in r.chunks(CHECK_MAX_WALLETS) {
                let wallets: Vec<String> = page.iter().map(|(_, w)| w.clone()).collect();
                for ((player, _), v) in page.iter().zip(check_many(&origin, &slug, &wallets)) {
                    if v == Verdict::Nope {
                        crate::trace::note(&format!("elo: {player} no longer holds a copy"));
                        kick(*player);
                    }
                }
            }
        }).expect("elo: a thread");
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    /// An origin that answers every request with `body` (and `status`),
    /// and hands back what it was asked.
    fn origin_answering(status: &'static str, body: impl Into<String>) -> (String, std::sync::mpsc::Receiver<String>) {
        let body: String = body.into();
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for c in l.incoming().flatten() {
                let mut c = c;
                let _ = c.set_read_timeout(Some(Duration::from_millis(300)));
                let mut req = Vec::new();
                let mut buf = [0u8; 4096];
                while let Ok(n) = c.read(&mut buf) {
                    if n == 0 { break; }
                    req.extend_from_slice(&buf[..n]);
                    let s = String::from_utf8_lossy(&req);
                    if let Some(h) = s.find("\r\n\r\n") {
                        let len = s.lines().find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse::<usize>().unwrap_or(0))).unwrap_or(0);
                        if req.len() >= h + 4 + len { break; }
                    }
                }
                let _ = tx.send(String::from_utf8_lossy(&req).into_owned());
                let _ = write!(c, "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}", body.len());
            }
        });
        (format!("http://{addr}"), rx)
    }

    const W1: &str = "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf";
    const W2: &str = "0x2b5ad5c4795c026514f8317c7a215e218dccd6cf";

    #[test]
    fn one_wallet_three_answers() {
        let (o, asked) = origin_answering("200 OK", r#"{"game":"g","wallet":"x","entitled":true}"#);
        assert_eq!(check_one(&o, "g", W1), Verdict::Owns);
        assert!(asked.recv().unwrap().starts_with(&format!("GET /api/ticket/g/of/{W1} ")));
        let (o, _) = origin_answering("200 OK", r#"{"entitled": false}"#);
        assert_eq!(check_one(&o, "g", W1), Verdict::Nope);
        let (o, _) = origin_answering("200 OK", r#"{"entitled":null}"#);
        assert_eq!(check_one(&o, "g", W1), Verdict::Unknown);
        // the origin's own chain read failed: could not look, not "no"
        let (o, _) = origin_answering("503 Service Unavailable", r#"{"entitled":false}"#);
        assert_eq!(check_one(&o, "g", W1), Verdict::Unknown);
        // nobody there
        assert_eq!(check_one("http://127.0.0.1:9", "g", W1), Verdict::Unknown);
        // not a wallet: our bug, never the player's refusal
        assert_eq!(check_one(&o, "g", "0xABC"), Verdict::Unknown);
    }

    #[test]
    fn the_roster_in_one_call() {
        let (o, asked) = origin_answering("200 OK", format!(r#"{{"wallets":{{"{W1}":{{"entitled":true}},"{W2}":{{"entitled":false}}}}}}"#));
        let w3 = "0x0000000000000000000000000000000000000003".to_string();
        assert_eq!(check_many(&o, "g", &[W1.into(), W2.into(), w3]), [Verdict::Owns, Verdict::Nope, Verdict::Unknown]);
        let req = asked.recv().unwrap();
        assert!(req.starts_with("POST /api/ticket/g/check "), "{req}");
        assert!(req.ends_with(&format!(r#"{{"wallets":["{W1}","{W2}","0x0000000000000000000000000000000000000003"]}}"#)), "{req}");
        let (o, _) = origin_answering("200 OK", r#"{"every_wallet_entitled":true,"wallets":{}}"#);
        assert_eq!(check_many(&o, "g", &[W1.into(), W2.into()]), [Verdict::Owns, Verdict::Owns]);
        let (o, _) = origin_answering("502 Bad Gateway", "");
        assert_eq!(check_many(&o, "g", &[W1.into()]), [Verdict::Unknown]);
    }

    #[test]
    fn the_row_and_the_status() {
        let s = Server { name: "EU \"one\"".into(), map: "sweeper".into(), max_players: 16 };
        assert_eq!(
            shard_row(&s, "play.example:27015", Some("https://play.example/status.json")),
            "{\"servers\":[{\"id\":\"play.example:27015\",\"name\":\"EU \\\"one\\\"\",\"addr\":\"play.example:27015\",\"max_players\":16,\"map\":\"sweeper\",\"status_url\":\"https://play.example/status.json\"}]}\n"
        );
        // nothing known: the address names it, and no count is made up
        assert_eq!(shard_row(&Server::default(), "a:1", None), "{\"servers\":[{\"id\":\"a:1\",\"name\":\"a:1\",\"addr\":\"a:1\"}]}\n");
        assert_eq!(status(3, &s, 42), r#"{"players":3,"max_players":16,"tick":42}"#);
        assert_eq!(status(0, &Server::default(), 1), r#"{"players":0,"tick":1}"#);
        assert_eq!(online(100, &[(W1.into(), 40), (W2.into(), 90)]), format!(r#"{{"at":100,"online":[{{"wallet":"{W1}","since":40}},{{"wallet":"{W2}","since":90}}]}}"#));
        assert_eq!(online(5, &[]), r#"{"at":5,"online":[]}"#);
    }
}
