//! The relay: players reach a server by its identity, not its address, as
//! Steam's relays let them. A server behind a NAT, or one that hides its
//! address, registers here; a player dials the relay and names the server.
//!
//! It speaks WebTransport, as servers do (net.rs). Two kinds of session:
//!
//! - `/host`: a server. On the stream it opens it says who it is (Hosting:
//!   u64 identity, i32 virtual port); the relay answers with a nonce
//!   (Challenge), the server signs `host_text` with its host key (Proof, 65
//!   bytes, steam::host_key), and the relay checks the signer's identity is
//!   the one claimed before it says Ready. A server reconnecting replaces
//!   its old registration: only its key could. No identity is squatted.
//! - `/to/<identity>/<virtual port>`: a player. The relay opens a stream to
//!   that server, writes the player's tag (u32) on it, and copies bytes both
//!   ways without reading them: net.rs's frames, the hello, the welcome's
//!   nonce and the player's sign-in pass end to end. Datagrams cross with
//!   the tag in front on the server's side.
//!
//! The relay holds no key and checks no player: the server does, as if they
//! had dialled it. It sees who talks to whom, and how much.
//!
//! Who pays: with `ENGINE2_RELAY_METER` (`https://elopros.com/api`) and
//! `ENGINE2_RELAY_SECRET`, the Elo Pros meter (scry-forge's
//! `meter/gamerelay.py`) says whether a server may register, by its host
//! address, before it is Ready, and the relay beats every minute with its
//! players; "stop" drops it. A meter that can't be reached lets servers be:
//! an outage there never takes players off a server. With no meter, every
//! server is carried free: a relay for your own servers. A developer puts a
//! server on their bill with `claim`.
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use wtransport::endpoint::endpoint_side::Server;
use wtransport::endpoint::IncomingSession;
use wtransport::{Connection, Endpoint};

use crate::net::{frame, le_i32, le_u64, read_frame, rt, server_endpoint};

pub(crate) const CHALLENGE: u8 = 10;
pub(crate) const HOSTING: u8 = 11;
pub(crate) const PROOF: u8 = 12;
pub(crate) const READY: u8 = 13;
pub(crate) const REFUSED: u8 = 14;

const HANDSHAKE: Duration = Duration::from_secs(10);
/// Players at once through one server's registration.
const MAX_PLAYERS: usize = 1024;

/// What a server signs to be registered as `id`: the nonce makes it this
/// registration's and no other's.
pub(crate) fn host_text(id: u64, vport: i32, nonce: &str) -> String {
    format!("engine-shim relay: host {id}:{vport}\nnonce: {nonce}")
}

struct Host {
    session: Connection,
    players: Mutex<HashMap<u32, Connection>>,
    next: AtomicU32,
    /// The most players at once since the last beat.
    peak: AtomicUsize,
}

/// The Elo Pros meter: who pays for the servers carried here.
#[derive(Clone)]
pub(crate) struct Meter {
    pub url: String,
    pub secret: String,
    pub beat: Duration,
}

impl Meter {
    fn from_env() -> Option<Meter> {
        let get = |k: &str| std::env::var(k).ok().map(|v| v.trim().to_string()).filter(|v| !v.is_empty());
        Some(Meter { url: get("ENGINE2_RELAY_METER")?.trim_end_matches('/').to_string(), secret: get("ENGINE2_RELAY_SECRET")?, beat: Duration::from_secs(60) })
    }

    /// A box call: the reply's JSON, whatever its status; None if nobody answered.
    fn call(&self, what: &str, body: String) -> Option<crate::elo_overlay::json::Value> {
        let secret: String = self.secret.bytes().map(|b| if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") }).collect();
        let agent: ureq::Agent = ureq::Agent::config_builder().timeout_global(Some(Duration::from_secs(6))).http_status_as_error(false).build().into();
        let mut res = agent.post(&format!("{}/gamerelay/box/{what}?secret={secret}", self.url)).header("content-type", "application/json").send(body.as_str()).ok()?;
        let text = res.body_mut().with_config().limit(64 * 1024).read_to_string().ok()?;
        crate::elo_overlay::json::parse(&text).ok()
    }

    /// May the server at `host` (its address) register: Ok(its session), or
    /// Err(why not). Ok(None): the meter didn't answer, so it may.
    fn auth(&self, host: &str) -> Result<Option<String>, String> {
        let Some(r) = self.call("auth", format!(r#"{{"host":"{host}"}}"#)) else { return Ok(None) };
        if r.get("ok").map(|v| v.as_bool()) == Some(true) {
            return Ok(r.get("session").and_then(|s| s.as_str()).map(String::from));
        }
        Err(r.get("why").or(r.get("error")).and_then(|w| w.as_str()).unwrap_or("the meter said no").to_string())
    }

    /// A minute of carrying `host`: Err(why) when the meter says stop.
    fn beat(&self, host: &str, session: &str, players: usize) -> Result<(), String> {
        let Some(r) = self.call("beat", format!(r#"{{"host":"{host}","session":"{session}","players":{players}}}"#)) else { return Ok(()) };
        match r.get("go").map(|v| v.as_bool()) {
            Some(false) => Err(r.get("why").and_then(|w| w.as_str()).unwrap_or("the meter said stop").to_string()),
            _ => Ok(()),
        }
    }
}

/// What a server's host key signs to put that server on `account`'s bill
/// (scry-forge's `gamerelay.claim_text`, byte for byte).
pub fn claim_text(address: &str, account: &str) -> String {
    format!("elo game relay\nhost: {}\naccount: {account}", address.to_lowercase())
}

/// A server's claim for `account`, from its host key file: (its address,
/// the signature), for the dev console.
pub fn claim(key_file: &str, account: &str) -> Result<(String, String), String> {
    let text = std::fs::read_to_string(key_file).map_err(|e| format!("{key_file}: {e}"))?;
    let key = crate::siwe::parse_secret(text.trim()).ok_or_else(|| format!("{key_file}: not a host key (64 hex digits)"))?;
    let address = crate::siwe::hex(&crate::siwe::address_of_key(&key));
    let sig = crate::siwe::sign(&key, &claim_text(&address, account));
    Ok((address, crate::siwe::hex(&sig)))
}

type Hosts = Arc<Mutex<HashMap<(u64, i32), Arc<Host>>>>;

/// The relay on `port` until the process ends (the `relay` binary).
pub fn run(port: u16) -> Result<(), String> {
    let endpoint = server_endpoint(port)?;
    let addr = endpoint.local_addr().map(|a| a.to_string()).unwrap_or_default();
    eprintln!("relay: on {addr}; servers register at /host, players dial /to/<identity>/<virtual port>");
    let meter = Meter::from_env();
    eprintln!("relay: {}", meter.as_ref().map_or("no meter: every server carried free".to_string(), |m| format!("servers paid for at {}", m.url)));
    rt().block_on(serve(endpoint, meter));
    Ok(())
}

pub(crate) async fn serve(endpoint: Endpoint<Server>, meter: Option<Meter>) {
    let hosts = Hosts::default();
    loop {
        let incoming = endpoint.accept().await;
        rt().spawn(session(hosts.clone(), meter.clone(), incoming));
    }
}

async fn session(hosts: Hosts, meter: Option<Meter>, incoming: IncomingSession) {
    let Ok(Ok(request)) = tokio::time::timeout(HANDSHAKE, incoming).await else { return };
    let path = request.path().to_string();
    if path == "/host" {
        if let Ok(conn) = request.accept().await { host(hosts, meter, conn).await; }
        return;
    }
    let to = path.strip_prefix("/to/").and_then(|r| r.split_once('/')).and_then(|(id, vport)| Some((id.parse::<u64>().ok()?, vport.parse::<i32>().ok()?)));
    let host = to.and_then(|to| hosts.lock().unwrap().get(&to).cloned());
    match host {
        Some(h) if h.players.lock().unwrap().len() < MAX_PLAYERS => {
            if let Ok(conn) = request.accept().await { player(h, conn).await; }
        }
        Some(_) => request.too_many_requests().await,
        None => request.not_found().await,
    }
}

/// A server: who it is, proven, paid for, then registered until its session
/// ends, carrying its players' datagrams back to them.
async fn host(hosts: Hosts, meter: Option<Meter>, session: Connection) {
    let proven = tokio::time::timeout(HANDSHAKE, async {
        let (mut send, mut recv) = session.accept_bi().await.map_err(|e| e.to_string())?;
        let (k, b) = read_frame(&mut recv).await?;
        if k != HOSTING { return Err("not a server".to_string()); }
        let (id, vport) = (le_u64(&b, 0), le_i32(&b, 8));
        let nonce = crate::steam::random_hex();
        send.write_all(&frame(CHALLENGE, nonce.as_bytes())).await.map_err(|e| e.to_string())?;
        let (k, sig) = read_frame(&mut recv).await?;
        let signer = (k == PROOF).then(|| crate::siwe::signer(&host_text(id, vport, &nonce), &sig)).flatten();
        let Some(address) = signer.filter(|a| crate::siwe::id_of(a) == id).map(|a| crate::siwe::hex(&a)) else {
            let _ = send.write_all(&frame(REFUSED, b"that is not this identity's key")).await;
            let _ = send.finish().await;
            return Err(format!("{id} not proven"));
        };
        // paid for, or refused with why
        let paid = match meter.clone() {
            Some(m) => {
                let a = address.clone();
                tokio::task::spawn_blocking(move || m.auth(&a)).await.unwrap_or(Ok(None))
            }
            None => Ok(None),
        };
        let billed = match paid {
            Ok(s) => s,
            Err(why) => {
                let _ = send.write_all(&frame(REFUSED, why.as_bytes())).await;
                let _ = send.finish().await;
                return Err(format!("{address}: {why}"));
            }
        };
        send.write_all(&frame(READY, &[])).await.map_err(|e| e.to_string())?;
        Ok((id, vport, address, billed, send, recv))
    });
    let (id, vport, address, billed, _send, _recv) = match proven.await {
        Ok(Ok(p)) => p,
        Ok(Err(e)) => return crate::trace::note(&format!("relay: a server didn't register: {e}")),
        Err(_) => return crate::trace::note("relay: a server didn't register in time"),
    };
    let me = Arc::new(Host { session: session.clone(), players: Mutex::default(), next: AtomicU32::new(0), peak: AtomicUsize::new(0) });
    hosts.lock().unwrap().insert((id, vport), me.clone());
    crate::trace::note(&format!("relay: {id}:{vport} is here"));
    // a minute at a time, the players carried, until the meter says stop
    let beats = meter.zip(billed).map(|(m, bill)| {
        let me = me.clone();
        rt().spawn(async move {
            loop {
                tokio::time::sleep(m.beat).await;
                let now = me.players.lock().unwrap().len();
                let players = me.peak.swap(now, Ordering::Relaxed).max(now);
                let (m2, a, b) = (m.clone(), address.clone(), bill.clone());
                if let Ok(Err(why)) = tokio::task::spawn_blocking(move || m2.beat(&a, &b, players)).await {
                    crate::trace::note(&format!("relay: {id}:{vport} dropped: {why}"));
                    me.session.close(0u32.into(), why.as_bytes());
                    return;
                }
            }
        })
    });
    while let Ok(d) = session.receive_datagram().await {
        let tag = u32::from_le_bytes(d.get(..4).and_then(|t| t.try_into().ok()).unwrap_or_default());
        let to = me.players.lock().unwrap().get(&tag).cloned();
        if let Some(p) = to { let _ = p.send_datagram(&d[4..]); }
    }
    if let Some(b) = beats { b.abort(); }
    let mut h = hosts.lock().unwrap();
    if h.get(&(id, vport)).is_some_and(|now| Arc::ptr_eq(now, &me)) {
        h.remove(&(id, vport));
        crate::trace::note(&format!("relay: {id}:{vport} left"));
    }
}

/// A player: their stream piped to a stream of the server's, both ways,
/// and their datagrams tagged, until either end is done.
async fn player(h: Arc<Host>, conn: Connection) {
    let tag = h.next.fetch_add(1, Ordering::Relaxed) + 1;
    let n = {
        let mut p = h.players.lock().unwrap();
        p.insert(tag, conn.clone());
        p.len()
    };
    h.peak.fetch_max(n, Ordering::Relaxed);
    let piped = async {
        let (mut psend, mut precv) = tokio::time::timeout(HANDSHAKE, conn.accept_bi()).await.map_err(|_| "no stream".to_string())?.map_err(|e| e.to_string())?;
        let (mut hsend, mut hrecv) = h.session.open_bi().await.map_err(|e| e.to_string())?.await.map_err(|e| e.to_string())?;
        hsend.write_all(&tag.to_le_bytes()).await.map_err(|e| e.to_string())?;
        let up = async {
            let _ = tokio::io::copy(&mut precv, &mut hsend).await;
            let _ = hsend.finish().await;
        };
        let down = async {
            let _ = tokio::io::copy(&mut hrecv, &mut psend).await;
            let _ = psend.finish().await;
        };
        let dgrams = async {
            while let Ok(d) = conn.receive_datagram().await {
                let mut t = tag.to_le_bytes().to_vec();
                t.extend_from_slice(&d);
                let _ = h.session.send_datagram(t);
            }
        };
        tokio::select! {
            _ = async { tokio::join!(up, down) } => {}
            _ = dgrams => {}
        }
        Ok::<_, String>(())
    };
    if let Err(e) = piped.await { crate::trace::note(&format!("relay: a player didn't get through: {e}")); }
    h.players.lock().unwrap().remove(&tag);
    conn.close(0u32.into(), b"");
}
