//! The relay, on its own (engine2/src/relay.rs): players reach a server by
//! its identity through it. `ENGINE2_RELAY_PORT` (default 27080, udp), and
//! the certificate as a server's: `ENGINE2_TLS_CERT`/`ENGINE2_TLS_KEY`, or a
//! self-signed one whose hash it prints.
fn main() {
    let port = std::env::var("ENGINE2_RELAY_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(27080);
    if let Err(e) = engine2::relay::run(port) {
        eprintln!("relay: {e}");
        std::process::exit(1);
    }
}
