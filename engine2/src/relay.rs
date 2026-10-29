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
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
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
}

type Hosts = Arc<Mutex<HashMap<(u64, i32), Arc<Host>>>>;

/// The relay on `port` until the process ends (the `relay` binary).
pub fn run(port: u16) -> Result<(), String> {
    let endpoint = server_endpoint(port)?;
    let addr = endpoint.local_addr().map(|a| a.to_string()).unwrap_or_default();
    eprintln!("relay: on {addr}; servers register at /host, players dial /to/<identity>/<virtual port>");
    rt().block_on(serve(endpoint));
    Ok(())
}

pub(crate) async fn serve(endpoint: Endpoint<Server>) {
    let hosts = Hosts::default();
    loop {
        let incoming = endpoint.accept().await;
        rt().spawn(session(hosts.clone(), incoming));
    }
}

async fn session(hosts: Hosts, incoming: IncomingSession) {
    let Ok(Ok(request)) = tokio::time::timeout(HANDSHAKE, incoming).await else { return };
    let path = request.path().to_string();
    if path == "/host" {
        if let Ok(conn) = request.accept().await { host(hosts, conn).await; }
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

/// A server: who it is, proven, then registered until its session ends,
/// carrying its players' datagrams back to them.
async fn host(hosts: Hosts, session: Connection) {
    let proven = tokio::time::timeout(HANDSHAKE, async {
        let (mut send, mut recv) = session.accept_bi().await.map_err(|e| e.to_string())?;
        let (k, b) = read_frame(&mut recv).await?;
        if k != HOSTING { return Err("not a server".to_string()); }
        let (id, vport) = (le_u64(&b, 0), le_i32(&b, 8));
        let nonce = crate::steam::random_hex();
        send.write_all(&frame(CHALLENGE, nonce.as_bytes())).await.map_err(|e| e.to_string())?;
        let (k, sig) = read_frame(&mut recv).await?;
        let signer = (k == PROOF).then(|| crate::siwe::signer(&host_text(id, vport, &nonce), &sig)).flatten();
        if signer.map(|a| crate::siwe::id_of(&a)) != Some(id) {
            let _ = send.write_all(&frame(REFUSED, b"that is not this identity's key")).await;
            let _ = send.finish().await;
            return Err(format!("{id} not proven"));
        }
        send.write_all(&frame(READY, &[])).await.map_err(|e| e.to_string())?;
        Ok((id, vport, send, recv))
    });
    let (id, vport, _send, _recv) = match proven.await {
        Ok(Ok(p)) => p,
        Ok(Err(e)) => return crate::trace::note(&format!("relay: a server didn't register: {e}")),
        Err(_) => return crate::trace::note("relay: a server didn't register in time"),
    };
    let me = Arc::new(Host { session: session.clone(), players: Mutex::default(), next: AtomicU32::new(0) });
    hosts.lock().unwrap().insert((id, vport), me.clone());
    crate::trace::note(&format!("relay: {id}:{vport} is here"));
    while let Ok(d) = session.receive_datagram().await {
        let tag = u32::from_le_bytes(d.get(..4).and_then(|t| t.try_into().ok()).unwrap_or_default());
        let to = me.players.lock().unwrap().get(&tag).cloned();
        if let Some(p) = to { let _ = p.send_datagram(&d[4..]); }
    }
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
    h.players.lock().unwrap().insert(tag, conn.clone());
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
