//! Sign-In With Ethereum (EIP-4361): who a player is, proved. The Elo Pros
//! launcher's `prove` signs a standard SIWE message over a nonce the server
//! issued (sdk/PROTOCOL.md in elo-pros-launcher); here it is checked with
//! arithmetic alone, as Gates' shards check it: keccak the EIP-191 envelope,
//! recover the public key, hash it to an address. No network call.
//!
//! The launcher writes the message with its own clock, so the server does
//! not rebuild it: it verifies the signature over the bytes as sent, then
//! reads each field it relies on and checks it against what it knows.
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use tiny_keccak::{Hasher, Keccak};

/// A ticket's first line when it carries a proof.
const SCHEME: &str = "eip4361";

pub type Address = [u8; 20];

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut k = Keccak::v256();
    let mut out = [0u8; 32];
    k.update(data);
    k.finalize(&mut out);
    out
}

/// The `personal_sign` digest: the message under EIP-191's envelope.
fn digest(message: &[u8]) -> [u8; 32] {
    let mut e = format!("\x19Ethereum Signed Message:\n{}", message.len()).into_bytes();
    e.extend_from_slice(message);
    keccak256(&e)
}

/// The last 20 bytes of keccak over the uncompressed point, without its tag.
fn address_of(vk: &VerifyingKey) -> Address {
    let p = vk.to_encoded_point(false);
    let h = keccak256(&p.as_bytes()[1..]);
    h[12..].try_into().unwrap()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    if s.len() % 2 != 0 { return None; }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(s.get(i..i + 2)?, 16).ok()).collect()
}

pub fn hex(b: &[u8]) -> String {
    let mut s = String::from("0x");
    for x in b { s.push_str(&format!("{x:02x}")); }
    s
}

/// `0x` and 40 hex digits, either case.
pub fn parse_address(s: &str) -> Option<Address> {
    unhex(s.trim())?.try_into().ok()
}

/// A player's id where Steam has a SteamID: their address's first 8 bytes,
/// so an operator can read one off the other.
pub fn id_of(a: &Address) -> u64 {
    u64::from_be_bytes(a[..8].try_into().unwrap())
}

/// Who signed `message` (a 65-byte r‖s‖v signature, v 27/28 or 0/1).
pub fn signer(message: &str, signature: &[u8]) -> Option<Address> {
    if signature.len() != 65 { return None; }
    let v = match signature[64] { 27 | 0 => 0, 28 | 1 => 1, _ => return None };
    let sig = Signature::from_slice(&signature[..64]).ok()?;
    let vk = VerifyingKey::recover_from_prehash(&digest(message.as_bytes()), &sig, RecoveryId::from_byte(v)?).ok()?;
    Some(address_of(&vk))
}

/// A proof as a ticket: the scheme, the signature, the message, a line each.
pub fn ticket(message: &str, signature_hex: &str) -> Vec<u8> {
    format!("{SCHEME}\n{signature_hex}\n{message}").into_bytes()
}

/// A ticket's message and signature, if it carries a proof.
pub fn parse_ticket(t: &[u8]) -> Option<(String, Vec<u8>)> {
    let t = std::str::from_utf8(t).ok()?;
    let rest = t.strip_prefix(SCHEME)?.strip_prefix('\n')?;
    let (sig, message) = rest.split_once('\n')?;
    Some((message.to_string(), unhex(sig.trim())?))
}

/// The fields of a SIWE message a server checks.
#[derive(Debug, PartialEq)]
pub struct Fields {
    pub domain: String,
    pub address: Address,
    pub uri: String,
    pub chain: String,
    pub nonce: String,
    /// Unix seconds.
    pub issued_at: i64,
}

/// EIP-4361's layout: the domain's line, the address, then `Key: value`
/// lines after a statement. None if it is not one.
pub fn fields(message: &str) -> Option<Fields> {
    let mut lines = message.lines();
    let domain = lines.next()?.strip_suffix(" wants you to sign in with your Ethereum account:")?;
    let address = parse_address(lines.next()?)?;
    let (mut uri, mut chain, mut nonce, mut issued) = (None, None, None, None);
    for l in lines {
        let Some((k, v)) = l.split_once(": ") else { continue };
        let slot = match k {
            "URI" => &mut uri,
            "Chain ID" => &mut chain,
            "Nonce" => &mut nonce,
            "Issued At" => &mut issued,
            _ => continue,
        };
        // a key twice is a message built to be read two ways
        if slot.replace(v.to_string()).is_some() { return None; }
    }
    Some(Fields {
        domain: domain.to_string(),
        address,
        uri: uri?,
        chain: chain.unwrap_or_default(),
        nonce: nonce?,
        issued_at: rfc3339(&issued?)?,
    })
}

/// `2026-08-07T12:00:00Z`, fractions and offsets allowed: unix seconds.
fn rfc3339(s: &str) -> Option<i64> {
    let n = |r: std::ops::Range<usize>| s.get(r)?.parse::<i64>().ok();
    if s.len() < 20 || &s[4..5] != "-" || &s[7..8] != "-" || !matches!(&s[10..11], "T" | "t") || &s[13..14] != ":" || &s[16..17] != ":" {
        return None;
    }
    let (y, mo, d, h, mi, sec) = (n(0..4)?, n(5..7)?, n(8..10)?, n(11..13)?, n(14..16)?, n(17..19)?);
    let mut rest = &s[19..];
    if let Some(f) = rest.strip_prefix('.') {
        rest = f.trim_start_matches(|c: char| c.is_ascii_digit());
    }
    let offset = match rest {
        "Z" | "z" => 0,
        o if o.len() == 6 && (o.starts_with('+') || o.starts_with('-')) && &o[3..4] == ":" => {
            let m = o[1..3].parse::<i64>().ok()? * 60 + o[4..6].parse::<i64>().ok()?;
            if o.starts_with('-') { -m } else { m }
        }
        _ => return None,
    };
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) || h > 23 || mi > 59 || sec > 60 { return None; }
    // days from the civil date (Howard Hinnant's)
    let y = if mo <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (if mo > 2 { mo - 3 } else { mo + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400 + h * 3600 + mi * 60 + sec - offset * 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::ecdsa::SigningKey;

    fn key(byte: u8) -> SigningKey {
        let mut b = [0u8; 32];
        b[31] = byte;
        SigningKey::from_slice(&b).unwrap()
    }

    pub fn sign(sk: &SigningKey, message: &str) -> String {
        let (sig, rec) = sk.sign_prehash_recoverable(&digest(message.as_bytes())).unwrap();
        let mut b = sig.to_bytes().to_vec();
        b.push(27 + rec.to_byte());
        hex(&b)
    }

    pub fn message(domain: &str, address: &str, nonce: &str, issued: &str) -> String {
        format!("{domain} wants you to sign in with your Ethereum account:\n{address}\n\nProve your identity to play gates. This signature authorises nothing and moves no funds.\n\nURI: https://{domain}\nVersion: 1\nChain ID: 4663\nNonce: {nonce}\nIssued At: {issued}")
    }

    #[test]
    fn private_key_one_is_its_published_address() {
        assert_eq!(hex(&address_of(key(1).verifying_key())), "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf");
    }

    #[test]
    fn a_proof_recovers_its_signer_and_reads_back() {
        let sk = key(7);
        let me = address_of(sk.verifying_key());
        let m = message("shard.example:27015", "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf", "8f14e45fceea167a", "2026-08-07T12:00:00Z");
        let t = ticket(&m, &sign(&sk, &m));
        let (m2, sig) = parse_ticket(&t).unwrap();
        assert_eq!(m2, m);
        assert_eq!(signer(&m2, &sig), Some(me));
        // another message, the same signature: someone else
        assert_ne!(signer(&m.replace("8f14", "9f14"), &sig), Some(me));
        let f = fields(&m).unwrap();
        assert_eq!(f.domain, "shard.example:27015");
        assert_eq!(hex(&f.address), "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf");
        assert_eq!(f.uri, "https://shard.example:27015");
        assert_eq!(f.nonce, "8f14e45fceea167a");
        assert_eq!(f.issued_at, 1_786_104_000);
    }

    #[test]
    fn times() {
        assert_eq!(rfc3339("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(rfc3339("2026-08-07T12:00:00.123Z"), Some(1_786_104_000));
        assert_eq!(rfc3339("2026-08-07T14:00:00+02:00"), Some(1_786_104_000));
        assert_eq!(rfc3339("2026-08-07 12:00:00Z"), None);
        assert_eq!(rfc3339("yesterday"), None);
    }

    #[test]
    fn not_proofs() {
        assert_eq!(parse_ticket(b"engine-shim/1 42"), None);
        assert_eq!(fields("hello"), None);
        let twice = message("a", "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf", "n1n1n1n1", "2026-08-07T12:00:00Z") + "\nNonce: other";
        assert_eq!(fields(&twice), None);
        assert_eq!(signer("x", &[0u8; 65]), None);
        assert_eq!(signer("x", &[1u8; 3]), None);
    }
}

#[cfg(test)]
pub use tests::{message as test_message, sign as test_sign};
#[cfg(test)]
pub fn test_key(byte: u8) -> (k256::ecdsa::SigningKey, Address) {
    let mut b = [0u8; 32];
    b[31] = byte;
    let sk = k256::ecdsa::SigningKey::from_slice(&b).unwrap();
    let a = address_of(sk.verifying_key());
    (sk, a)
}
