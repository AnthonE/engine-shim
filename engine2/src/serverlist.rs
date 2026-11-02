//! The server browser: s&box's `CServerList` (what `Networking.QueryLobbies`,
//! `JoinBestLobby` and the `servers` command ask), answered from Elo Pros
//! shard lists rather than Steam's master servers.
//!
//! `ENGINE2_SERVER_LIST` names where to look, commas between: files, or
//! `http(s)://` URLs of an `elo-shardlist-v1` document (`{"servers":[...]}`,
//! what a server writes to `ENGINE2_SHARDLIST`, elo.rs, and the origin
//! serves as `<slug>/servers.json`). Each row is a server; its `status_url`,
//! if it has one, is asked for its players, and how long that took is its
//! ping. A row past the store's fields carries what the browser filters on:
//! the server's `identity`, `game`, `tags` and `password` (elo.rs writes them).
//!
//! Managed's filters are Steam's: `gametagsand` (every tag named is the
//! server's), `gametagsnor` (none is) and `map`. A server's identity, once
//! listed, is where `ConnectToSteamId` goes: straight to its address, with
//! no relay between (impls.rs).
//!
//! A query runs on its own thread; managed hears it start at once, and each
//! server and the end when it runs Steam's callbacks (`pump`), as Steam's
//! server list answers from its own.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

/// How long one source or one server's status may take.
const TIMEOUT: Duration = Duration::from_secs(3);
/// A list or a status past this is refused.
const MAX_BYTES: u64 = 1 << 20;

/// One server, as a row says and its status answers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Row {
    pub identity: u64,
    pub addr: String,
    pub name: String,
    pub map: String,
    pub game: String,
    pub tags: Vec<String>,
    pub password: bool,
    pub players: i32,
    pub max_players: i32,
    pub status_url: Option<String>,
    /// Milliseconds its status took; 0, not asked.
    pub ping: i32,
}

/// Rows from one `elo-shardlist-v1` document. A row without an address is
/// nobody's to join, and left out; one without an identity is given one
/// made from its address, which only this process knows it by.
pub fn rows(doc: &str) -> Vec<Row> {
    let Ok(v) = serde_json::from_str::<serde_json::Value>(doc) else { return vec![] };
    let Some(list) = v.get("servers").and_then(|s| s.as_array()) else { return vec![] };
    let s = |r: &serde_json::Value, k: &str| r.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    list.iter()
        .filter_map(|r| {
            let addr = s(r, "addr");
            if addr.is_empty() { return None; }
            let identity = match r.get("identity") {
                Some(serde_json::Value::String(t)) => t.trim().parse().ok(),
                Some(serde_json::Value::Number(n)) => n.as_u64(),
                _ => None,
            }
            .filter(|&i| i != 0)
            .unwrap_or_else(|| made_up(&addr));
            let tags = s(r, "tags").split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect();
            Some(Row {
                identity,
                name: Some(s(r, "name")).filter(|n| !n.is_empty()).unwrap_or_else(|| addr.clone()),
                map: s(r, "map"),
                game: s(r, "game"),
                tags,
                password: r.get("password").and_then(|p| p.as_bool()).unwrap_or(false),
                max_players: r.get("max_players").and_then(|m| m.as_i64()).unwrap_or(0) as i32,
                status_url: Some(s(r, "status_url")).filter(|u| !u.is_empty()),
                addr,
                ..Default::default()
            })
        })
        .collect()
}

/// An identity for a row that names none: its address hashed (FNV-1a), in
/// the individual-account range steam.rs makes its own up in.
fn made_up(addr: &str) -> u64 {
    let h = addr.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3));
    0x0110_0001_0000_0000 | ((h ^ (h >> 32)) & 0xffff_ffff).max(1)
}

/// Whether a server passes managed's filters (Steam's keys). A filter we
/// don't know is said once and passes: Steam's list has more than a shard
/// row can answer.
pub fn passes(r: &Row, filters: &[(String, String)]) -> bool {
    let named = |v: &str| v.split(',').map(str::trim).filter(|t| !t.is_empty()).map(String::from).collect::<Vec<_>>();
    filters.iter().all(|(k, v)| match k.as_str() {
        "gametagsand" => named(v).iter().all(|t| r.tags.contains(t)),
        "gametagsnor" => !named(v).iter().any(|t| r.tags.contains(t)),
        "map" => r.map.eq_ignore_ascii_case(v),
        "password" => (v == "1") == r.password,
        _ => {
            static SAID: Mutex<Vec<String>> = Mutex::new(Vec::new());
            let mut said = SAID.lock().unwrap();
            if !said.contains(k) {
                said.push(k.clone());
                crate::trace::note(&format!("serverlist: the filter {k} isn't one a shard row answers; passed"));
            }
            true
        }
    })
}

/// The sources `ENGINE2_SERVER_LIST` names.
fn sources() -> Vec<String> {
    std::env::var("ENGINE2_SERVER_LIST").unwrap_or_default().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder().timeout_global(Some(TIMEOUT)).build().into()
}

fn fetch(source: &str) -> Result<String, String> {
    if source.starts_with("http://") || source.starts_with("https://") {
        let mut res = agent().get(source).call().map_err(|e| e.to_string())?;
        res.body_mut().with_config().limit(MAX_BYTES).read_to_string().map_err(|e| e.to_string())
    } else {
        std::fs::read_to_string(source).map_err(|e| e.to_string())
    }
}

/// A row's players and cap from its `status.json`, and how long it took.
fn ask_status(r: &mut Row) {
    let Some(url) = r.status_url.clone() else { return };
    let began = Instant::now();
    let Ok(body) = fetch(&url) else { return };
    let ms = began.elapsed().as_millis().clamp(1, i32::MAX as u128) as i32;
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&body) else { return };
    r.ping = ms;
    if let Some(p) = v.get("players").and_then(|p| p.as_i64()) { r.players = p as i32; }
    if let Some(m) = v.get("max_players").and_then(|m| m.as_i64()).filter(|&m| m > 0) { r.max_players = m as i32; }
}

/// Every server the sources list, each once (by identity, the first
/// source's row winning), statuses asked side by side.
pub fn gather() -> Vec<Row> {
    let mut all: Vec<Row> = Vec::new();
    for src in sources() {
        match fetch(&src) {
            Ok(doc) => {
                for r in rows(&doc) {
                    if !all.iter().any(|a| a.identity == r.identity) { all.push(r); }
                }
            }
            Err(e) => crate::trace::note(&format!("serverlist: can't read {src}: {e}")),
        }
    }
    std::thread::scope(|s| {
        for r in all.iter_mut() { s.spawn(move || ask_status(r)); }
    });
    let mut known = known().lock().unwrap();
    for r in &all { known.insert(r.identity, r.addr.clone()); }
    all
}

/// Servers' addresses by identity, as lists named them.
fn known() -> &'static Mutex<HashMap<u64, String>> {
    static K: OnceLock<Mutex<HashMap<u64, String>>> = OnceLock::new();
    K.get_or_init(Default::default)
}

/// Where the server of this identity is, if a list says: the last query's
/// answer, else the lists asked now (a `connect <identity>` with none run).
pub fn address_of(identity: u64) -> Option<String> {
    if let Some(a) = known().lock().unwrap().get(&identity) { return Some(a.clone()); }
    if sources().is_empty() { return None; }
    gather();
    known().lock().unwrap().get(&identity).cloned()
}

// ---- managed's CServerList

/// `gameserveritem_t` as managed reads it (Sandbox.Services.ServerList):
/// Steam's, without the trailing SteamID, which comes beside it.
#[repr(C)]
pub struct Item {
    pub conn_port: u16,
    pub query_port: u16,
    /// IPv4, a.b.c.d as a << 24 | b << 16 | c << 8 | d.
    pub ip: u32,
    pub ping: i32,
    pub had_successful_response: u8,
    pub do_not_refresh: u8,
    pub game_dir: [u8; 32],
    pub map: [u8; 32],
    pub game_description: [u8; 64],
    pub app_id: u32,
    pub players: i32,
    pub max_players: i32,
    pub bot_players: i32,
    pub password: u8,
    pub secure: u8,
    pub time_last_played: u32,
    pub server_version: i32,
    pub server_name: [u8; 64],
    pub game_tags: [u8; 128],
}

/// `s` into a fixed buffer, NUL-terminated, cut at a character's edge.
fn fixed<const N: usize>(s: &str) -> [u8; N] {
    let mut b = [0u8; N];
    let mut end = s.len().min(N - 1);
    while !s.is_char_boundary(end) { end -= 1; }
    b[..end].copy_from_slice(&s.as_bytes()[..end]);
    b
}

/// A row as Steam's server list describes a server.
pub fn item(r: &Row) -> Item {
    use std::net::ToSocketAddrs;
    let at = r.addr.to_socket_addrs().ok().and_then(|mut a| a.find(|a| a.is_ipv4()));
    let (ip, port) = match at {
        Some(std::net::SocketAddr::V4(a)) => (u32::from(*a.ip()), a.port()),
        _ => (0, r.addr.rsplit_once(':').and_then(|(_, p)| p.parse().ok()).unwrap_or(0)),
    };
    Item {
        conn_port: port,
        query_port: port,
        ip,
        ping: r.ping,
        had_successful_response: 1,
        do_not_refresh: 0,
        game_dir: fixed("sbox"),
        map: fixed(&r.map),
        game_description: fixed(&r.game),
        app_id: 0,
        players: r.players,
        max_players: r.max_players,
        bot_players: 0,
        password: r.password as u8,
        secure: 0,
        time_last_played: 0,
        server_version: 0,
        server_name: fixed(&r.name),
        game_tags: fixed(&r.tags.join(",")),
    }
}

struct Query {
    /// The managed ServerList's interop handle.
    owner: u32,
    filters: Vec<(String, String)>,
}

#[derive(Default)]
struct Lists {
    queries: HashMap<usize, Query>,
    /// Queries answered, waiting for managed: (query, its servers).
    answered: Vec<(usize, Vec<Row>)>,
    next: usize,
}

fn lists() -> &'static Mutex<Lists> {
    static L: OnceLock<Mutex<Lists>> = OnceLock::new();
    L.get_or_init(Default::default)
}

/// A server list for the managed ServerList `owner`: a handle (never 0).
pub fn create(owner: u32) -> usize {
    let mut l = lists().lock().unwrap();
    l.next += 1;
    let id = l.next;
    l.queries.insert(id, Query { owner, filters: Vec::new() });
    id
}

pub fn destroy(list: usize) {
    let mut l = lists().lock().unwrap();
    l.queries.remove(&list);
    l.answered.retain(|(q, _)| *q != list);
}

pub fn add_filter(list: usize, key: &str, value: &str) {
    if let Some(q) = lists().lock().unwrap().queries.get_mut(&list) {
        q.filters.push((key.to_string(), value.to_string()));
    }
}

/// Managed's callbacks: OnStarted, OnServerResponded, OnFinished.
pub struct Managed {
    pub started: extern "C" fn(u32),
    pub responded: extern "C" fn(u32, *const Item, u64),
    pub finished: extern "C" fn(u32),
}

/// A query begun: managed hears it start now, the rest from `pump`.
pub fn start(list: usize, m: &Managed) {
    let Some((owner, filters)) = lists().lock().unwrap().queries.get(&list).map(|q| (q.owner, q.filters.clone())) else { return };
    (m.started)(owner);
    std::thread::Builder::new()
        .name("server list".into())
        .spawn(move || {
            let found: Vec<Row> = gather().into_iter().filter(|r| passes(r, &filters)).collect();
            crate::trace::note(&format!("serverlist: {} servers for {filters:?}", found.len()));
            let mut l = lists().lock().unwrap();
            if l.queries.contains_key(&list) { l.answered.push((list, found)); }
        })
        .expect("serverlist: a thread");
}

/// The answers ready, told to managed (on its thread: Steam's callbacks).
pub fn pump(m: &Managed) {
    let ready: Vec<(u32, Vec<Row>)> = {
        let mut l = lists().lock().unwrap();
        let answered = std::mem::take(&mut l.answered);
        answered.into_iter().filter_map(|(q, rows)| l.queries.get(&q).map(|q| (q.owner, rows))).collect()
    };
    for (owner, rows) in ready {
        for r in &rows {
            let it = item(r);
            (m.responded)(owner, &it, r.identity);
        }
        (m.finished)(owner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"{"servers":[
        {"id":"a","name":"EU one","addr":"127.0.0.1:27015","max_players":16,"map":"Sweeper","identity":"18446744073709551615","game":"Sweeper","tags":"protocol:3,gameident:local.sweeper,hdn:0","password":true},
        {"id":"b","name":"","addr":"10.1.2.3:27016"},
        {"id":"c","name":"nowhere"}
    ]}"#;

    #[test]
    fn rows_as_a_server_writes_them() {
        let r = rows(DOC);
        assert_eq!(r.len(), 2, "a row with no address is nobody's to join");
        assert_eq!(r[0].identity, u64::MAX);
        assert_eq!((r[0].name.as_str(), r[0].map.as_str(), r[0].game.as_str(), r[0].max_players, r[0].password), ("EU one", "Sweeper", "Sweeper", 16, true));
        assert_eq!(r[0].tags, ["protocol:3", "gameident:local.sweeper", "hdn:0"]);
        // nothing but an address: named by it, an identity made from it, the same each time
        assert_eq!(r[1].name, "10.1.2.3:27016");
        assert_eq!(r[1].identity, made_up("10.1.2.3:27016"));
        assert_eq!(r[1].identity >> 32, 0x0110_0001);
        assert!(rows("not json").is_empty() && rows(r#"{"rooms":[]}"#).is_empty());

        // and the row elo.rs writes reads back as it was
        let s = crate::elo::Server { name: "x".into(), map: "m".into(), max_players: 4, game: "g".into(), tags: "a:1,b:2".into(), password: false };
        let back = rows(&crate::elo::shard_row(&s, "1.2.3.4:5", Some("http://h/status.json"), 42));
        assert_eq!(back, [Row { identity: 42, addr: "1.2.3.4:5".into(), name: "x".into(), map: "m".into(), game: "g".into(), tags: vec!["a:1".into(), "b:2".into()], max_players: 4, status_url: Some("http://h/status.json".into()), ..Default::default() }]);
    }

    #[test]
    fn filters_as_steams() {
        let r = &rows(DOC)[0];
        let f = |k: &str, v: &str| vec![(k.to_string(), v.to_string())];
        assert!(passes(r, &f("gametagsand", "protocol:3")));
        assert!(passes(r, &f("gametagsand", "protocol:3,hdn:0")));
        assert!(!passes(r, &f("gametagsand", "protocol:4")));
        assert!(!passes(r, &f("gametagsand", "gameident:local.sweeper,mapident:x")));
        assert!(passes(r, &f("gametagsnor", "hdn:1")) && !passes(r, &f("gametagsnor", "hdn:0")));
        assert!(passes(r, &f("map", "sweeper")) && !passes(r, &f("map", "flatgrass")));
        assert!(passes(r, &f("secure", "1")), "one a row can't answer passes");
        // managed's own: protocol, game, not hidden; a row with no tags is no s&box server's
        let managed = vec![("gametagsand".to_string(), "protocol:3".to_string()), ("gametagsand".to_string(), "gameident:local.sweeper".to_string()), ("gametagsand".to_string(), "hdn:0".to_string())];
        assert!(passes(r, &managed));
        assert!(!passes(&rows(DOC)[1], &managed));
    }

    #[test]
    fn an_item_as_managed_reads_it() {
        // gameserveritem_t's layout, as Sandbox.Services.ServerList declares it
        assert_eq!(std::mem::size_of::<Item>(), 364);
        assert_eq!(std::mem::offset_of!(Item, game_dir), 14);
        assert_eq!(std::mem::offset_of!(Item, app_id), 144);
        assert_eq!(std::mem::offset_of!(Item, password), 160);
        assert_eq!(std::mem::offset_of!(Item, time_last_played), 164);
        assert_eq!(std::mem::offset_of!(Item, server_name), 172);
        assert_eq!(std::mem::offset_of!(Item, game_tags), 236);
        let mut r = rows(DOC)[0].clone();
        r.players = 3;
        let it = item(&r);
        // managed: new IPAddress(BitConverter.GetBytes(HostToNetworkOrder((int)ip)))
        assert_eq!(it.ip, 0x7f00_0001);
        assert_eq!((it.conn_port, it.players, it.max_players, it.password), (27015, 3, 16, 1));
        assert_eq!(&it.server_name[..7], b"EU one\0");
        assert_eq!(&it.game_tags[..41], b"protocol:3,gameident:local.sweeper,hdn:0\0");
        // a long name is cut, never left unterminated, never mid-character
        let long: [u8; 8] = fixed("abcdefé");
        assert_eq!(&long, b"abcdef\0\0");
    }
}
