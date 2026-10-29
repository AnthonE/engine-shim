//! The relay, on its own (engine2/src/relay.rs): players reach a server by
//! its identity through it. `ENGINE2_RELAY_PORT` (default 27080, udp), and
//! the certificate as a server's: `ENGINE2_TLS_CERT`/`ENGINE2_TLS_KEY`, or a
//! self-signed one whose hash it prints. With `ENGINE2_RELAY_METER` and
//! `ENGINE2_RELAY_SECRET`, the Elo Pros meter says who pays for each server.
//!
//!   relay                            run it
//!   relay claim <host key> <account> a server's claim for an Elo Pros
//!                                    account's bill, for the dev console
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("claim") {
        let (Some(key), Some(account)) = (args.get(1), args.get(2)) else {
            eprintln!("relay claim <host key file> <account (acct_…)>");
            std::process::exit(2);
        };
        match engine2::relay::claim(key, account) {
            Ok((address, signature)) => {
                println!("address: {address}");
                println!("signature: {signature}");
                eprintln!("on the dev console's game relay, as {account}: claim this server with both");
            }
            Err(e) => {
                eprintln!("relay claim: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    let port = std::env::var("ENGINE2_RELAY_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(27080);
    if let Err(e) = engine2::relay::run(port) {
        eprintln!("relay: {e}");
        std::process::exit(1);
    }
}
