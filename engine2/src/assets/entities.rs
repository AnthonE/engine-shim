//! A map's entities (`maps/<map>/entities/<lump>.vents_c`): a KV3 lump, each
//! entity's keys a blob in Source 2's entity format. The blob: a version
//! (1), how many keys are known by hash and how many by name, then each
//! key's hash (StringToken's: MurmurHash2 of its name, lowercased), its
//! type and its value; a named key has its name too. Values are given to
//! managed as the strings the engine formats them as ("1.000000 0.000000
//! 0.000000"), which it converts to what it wants.
use super::kv3::Value;

/// An entity: its keys, by hash, and their values as text.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Entity {
    pub keys: Vec<(u32, String)>,
}

impl Entity {
    pub fn get(&self, hash: u32) -> Option<&str> {
        self.keys.iter().find(|(h, _)| *h == hash).map(|(_, v)| v.as_str())
    }
}

/// The entities of a lump's DATA block.
pub fn read_lump(data: &Value) -> Vec<Entity> {
    data.get("m_entityKeyValues").map(Value::as_array).unwrap_or(&[]).iter().filter_map(|e| {
        match (e.get("m_keyValuesData"), e.get("m_keyValues3Data")) {
            (Some(Value::Blob(b)), _) => read_blob(b),
            (_, Some(kv3)) => Some(from_kv3(kv3)),
            _ => None,
        }
    }).collect()
}

/// An entity whose keys are KV3 already (newer lumps): its `values`.
fn from_kv3(v: &Value) -> Entity {
    let values = v.get("values").unwrap_or(v);
    let Value::Object(m) = values else { return Entity::default() };
    Entity { keys: m.iter().map(|(k, x)| (crate::handles::murmur2(k), text(x))).collect() }
}

fn text(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::Bool(b) => (if *b { "1" } else { "0" }).into(),
        Value::Int(i) => i.to_string(),
        Value::UInt(u) => u.to_string(),
        Value::Double(d) => format!("{d:.6}"),
        Value::String(s) => s.clone(),
        Value::Array(a) => a.iter().map(text).collect::<Vec<_>>().join(" "),
        Value::Blob(_) | Value::Object(_) => String::new(),
    }
}

// EntityFieldType, as the blob names its values' types
const FLOAT: u32 = 1;
const STRING: u32 = 2;
const VECTOR: u32 = 3;
const QUATERNION: u32 = 4;
const INTEGER: u32 = 5;
const BOOLEAN: u32 = 6;
const SHORT: u32 = 7;
const CHARACTER: u32 = 8;
const COLOR32: u32 = 9;
const POSITION_VECTOR: u32 = 14;
const TIME: u32 = 15;
const VECTOR2D: u32 = 25;
const INTEGER64: u32 = 26;
const VECTOR4D: u32 = 27;
const CSTRING: u32 = 30;
const UINT64: u32 = 33;
const FLOAT64: u32 = 34;
const UINT: u32 = 37;
const QANGLE: u32 = 39;

struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl Cursor<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        let s = self.b.get(self.at..self.at + n)?;
        self.at += n;
        Some(s)
    }
    fn u32(&mut self) -> Option<u32> {
        self.take(4).map(|x| u32::from_le_bytes(x.try_into().unwrap()))
    }
    fn f32(&mut self) -> Option<f32> {
        self.take(4).map(|x| f32::from_le_bytes(x.try_into().unwrap()))
    }
    fn floats(&mut self, n: usize) -> Option<String> {
        (0..n).map(|_| self.f32().map(|f| format!("{f:.6}"))).collect::<Option<Vec<_>>>().map(|v| v.join(" "))
    }
    fn cstr(&mut self) -> Option<String> {
        let n = self.b.get(self.at..)?.iter().position(|&c| c == 0)?;
        let s = String::from_utf8_lossy(&self.b[self.at..self.at + n]).into_owned();
        self.at += n + 1;
        Some(s)
    }

    /// A value of type `ty`, as text: none for a type not known, after
    /// which nothing more of the entity can be read.
    fn value(&mut self, ty: u32) -> Option<String> {
        Some(match ty {
            FLOAT | TIME => format!("{:.6}", self.f32()?),
            VECTOR | POSITION_VECTOR | QANGLE => self.floats(3)?,
            VECTOR2D => self.floats(2)?,
            QUATERNION | VECTOR4D => self.floats(4)?,
            INTEGER => (self.u32()? as i32).to_string(),
            UINT => self.u32()?.to_string(),
            BOOLEAN => (if self.take(1)?[0] != 0 { "1" } else { "0" }).into(),
            SHORT => i16::from_le_bytes(self.take(2)?.try_into().unwrap()).to_string(),
            CHARACTER => (self.take(1)?[0] as i8).to_string(),
            COLOR32 => self.take(4)?.iter().map(u8::to_string).collect::<Vec<_>>().join(" "),
            INTEGER64 => i64::from_le_bytes(self.take(8)?.try_into().unwrap()).to_string(),
            UINT64 => u64::from_le_bytes(self.take(8)?.try_into().unwrap()).to_string(),
            FLOAT64 => format!("{:.6}", f64::from_le_bytes(self.take(8)?.try_into().unwrap())),
            CSTRING | STRING => self.cstr()?,
            _ => return None,
        })
    }
}

/// An entity's keys from its blob: as many as can be read.
pub fn read_blob(b: &[u8]) -> Option<Entity> {
    let mut c = Cursor { b, at: 0 };
    if c.u32()? != 1 { return None; }
    let (hashed, named) = (c.u32()?, c.u32()?);
    let mut e = Entity::default();
    for i in 0..hashed + named {
        let hash = c.u32()?;
        let ty = c.u32()?;
        if i >= hashed { c.cstr()?; }
        let Some(v) = c.value(ty) else { break };
        e.keys.push((hash, v));
    }
    Some(e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handles::murmur2;

    #[test]
    fn an_entitys_keys_read_as_text() {
        // as a map keeps them: hashed keys, then one by name
        let mut b = Vec::new();
        for v in [1u32, 4, 1] { b.extend_from_slice(&v.to_le_bytes()); }
        let key = |b: &mut Vec<u8>, name: &str, ty: u32| { b.extend_from_slice(&murmur2(name).to_le_bytes()); b.extend_from_slice(&ty.to_le_bytes()); };
        key(&mut b, "classname", CSTRING);
        b.extend_from_slice(b"info_player_start\0");
        key(&mut b, "origin", VECTOR);
        for f in [10.0f32, -2.5, 64.0] { b.extend_from_slice(&f.to_le_bytes()); }
        key(&mut b, "enabled", BOOLEAN);
        b.push(1);
        key(&mut b, "health", INTEGER);
        b.extend_from_slice(&(-5i32).to_le_bytes());
        key(&mut b, "custom", FLOAT);
        b.extend_from_slice(b"custom\0");
        b.extend_from_slice(&0.5f32.to_le_bytes());
        let e = read_blob(&b).unwrap();
        assert_eq!(e.get(murmur2("classname")), Some("info_player_start"));
        assert_eq!(e.get(murmur2("Origin")), Some("10.000000 -2.500000 64.000000"));
        assert_eq!(e.get(murmur2("enabled")), Some("1"));
        assert_eq!(e.get(murmur2("health")), Some("-5"));
        assert_eq!(e.get(murmur2("custom")), Some("0.500000"));
        // a type not known stops it, keeping what came before
        let mut short = b[..12].to_vec();
        short[4] = 2;
        key(&mut short, "classname", CSTRING);
        short.extend_from_slice(b"light\0");
        key(&mut short, "odd", 99);
        assert_eq!(read_blob(&short).unwrap().keys.len(), 1);
    }

    #[test]
    fn worldspawn_from_flatgrass() {
        // the first entity of a map from sbox.game, its first keys as they are on disk
        let b: &[u8] = b"\x01\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00i~\x9c8\x01\x00\x00\x00\x00\x00\x80?b\x1c\x1b\xc6\x1e\x00\x00\x00worldspawn\x00";
        let e = read_blob(b).unwrap();
        assert_eq!(e.get(murmur2("classname")), Some("worldspawn"));
        assert_eq!(e.keys[0].1, "1.000000");
    }
}
