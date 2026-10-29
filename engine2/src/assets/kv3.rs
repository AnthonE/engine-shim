//! Binary KeyValues 3, Source 2's tree of typed values in which compiled
//! resources keep their data (a model's description, its collision).
//! Read as ValveResourceFormat reads it (MIT, BinaryKV3.cs): versions 1 to
//! 5, uncompressed or LZ4 (zstd is not read yet).
//!
//! The values sit in lanes by size (one byte, two, four, eight), with a
//! table of strings and a list of type bytes that says what to take next.

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Double(f64),
    String(String),
    Blob(Vec<u8>),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

impl Value {
    /// A member of an object, by name.
    pub fn get(&self, name: &str) -> Option<&Value> {
        match self {
            Value::Object(m) => m.iter().rev().find(|(k, _)| k == name).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::Int(i) => Some(i as f64),
            Value::UInt(u) => Some(u as f64),
            Value::Double(d) => Some(d),
            Value::Bool(b) => Some(b as u8 as f64),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Int(i) => Some(i),
            Value::UInt(u) => Some(u as i64),
            Value::Double(d) => Some(d as i64),
            Value::Bool(b) => Some(b as i64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> &[Value] {
        match self {
            Value::Array(a) => a,
            _ => &[],
        }
    }

    /// A member's number (0 if absent).
    pub fn f32(&self, name: &str) -> f32 {
        self.get(name).and_then(Value::as_f64).unwrap_or(0.0) as f32
    }

    pub fn i64(&self, name: &str) -> i64 {
        self.get(name).and_then(Value::as_i64).unwrap_or(0)
    }

    /// An array of numbers as floats (a vector, a matrix's rows).
    pub fn floats(&self) -> Vec<f32> {
        self.as_array().iter().filter_map(Value::as_f64).map(|f| f as f32).collect()
    }

    /// The bytes of a blob, or an array of small numbers taken as bytes.
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Value::Blob(b) => b.clone(),
            Value::Array(a) => a.iter().filter_map(Value::as_i64).map(|i| i as u8).collect(),
            _ => Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn err<T>(s: impl Into<String>) -> Result<T, Error> {
    Err(Error(s.into()))
}

/// A byte cursor that never panics: running out is an error.
struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    fn new(b: &'a [u8]) -> Self {
        Cursor { b, at: 0 }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.b.len()).ok_or_else(|| Error(format!("KV3: {n} bytes past the end at {}", self.at)))?;
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }

    fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }

    fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    fn i32(&mut self) -> Result<i32, Error> {
        Ok(self.u32()? as i32)
    }

    fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn rest(&self) -> &'a [u8] {
        &self.b[self.at..]
    }
}

/// One lane of values of one size.
#[derive(Clone, Copy)]
struct Lane<'a> {
    b: &'a [u8],
}

impl<'a> Lane<'a> {
    fn next<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        if self.b.len() < N { return err("KV3: a value lane ran out"); }
        let (v, rest) = self.b.split_at(N);
        self.b = rest;
        Ok(v.try_into().unwrap())
    }
}

#[derive(Clone, Copy, Default)]
struct Lanes<'a> {
    l1: Lane<'a>,
    l2: Lane<'a>,
    l4: Lane<'a>,
    l8: Lane<'a>,
}

impl Default for Lane<'_> {
    fn default() -> Self {
        Lane { b: &[] }
    }
}

struct Context<'a> {
    version: u32,
    types: &'a [u8],
    object_lengths: Lane<'a>,
    strings: Vec<String>,
    blobs: Vec<Vec<u8>>,
    next_blob: usize,
    main: Lanes<'a>,
    aux: Lanes<'a>,
}

const NULL: u8 = 1;
const BOOLEAN: u8 = 2;
const INT64: u8 = 3;
const UINT64: u8 = 4;
const DOUBLE: u8 = 5;
const STRING: u8 = 6;
const BINARY_BLOB: u8 = 7;
const ARRAY: u8 = 8;
const OBJECT: u8 = 9;
const ARRAY_TYPED: u8 = 10;
const INT32: u8 = 11;
const UINT32: u8 = 12;
const BOOLEAN_TRUE: u8 = 13;
const BOOLEAN_FALSE: u8 = 14;
const INT64_ZERO: u8 = 15;
const INT64_ONE: u8 = 16;
const DOUBLE_ZERO: u8 = 17;
const DOUBLE_ONE: u8 = 18;
const FLOAT: u8 = 19;
const INT16: u8 = 20;
const UINT16: u8 = 21;
const INT8: u8 = 22;
const UINT8: u8 = 23;
const ARRAY_TYPE_BYTE_LENGTH: u8 = 24;
const ARRAY_TYPE_AUXILIARY_BUFFER: u8 = 25;
const UINT32_ZERO: u8 = 26;
const UINT32_ONE: u8 = 27;

const TRAILER: u32 = 0xFFEE_DD00;

impl<'a> Context<'a> {
    fn read_type(&mut self) -> Result<u8, Error> {
        let (&t, rest) = self.types.split_first().ok_or(Error("KV3: out of types".into()))?;
        self.types = rest;
        let mut t = t;
        if self.version >= 3 {
            if t & 0x80 != 0 { self.skip_type_byte()?; }
            if t & 0x40 != 0 { self.skip_type_byte()?; }
            t &= 0x3F;
        } else {
            if t & 0x80 != 0 { self.skip_type_byte()?; }
            t &= 0x7F;
        }
        Ok(t)
    }

    /// A flag (resource, resource name...): not kept.
    fn skip_type_byte(&mut self) -> Result<(), Error> {
        self.types = self.types.get(1..).ok_or(Error("KV3: out of types".into()))?;
        Ok(())
    }

    fn string(&self, id: i32) -> String {
        if id >= 0 { self.strings.get(id as usize).cloned().unwrap_or_default() } else { String::new() }
    }

    fn value(&mut self, t: u8, aux: bool) -> Result<Value, Error> {
        // before version 5 the auxiliary lanes are the main ones
        let aux = aux && self.version >= 5;
        let lane = |c: &mut Self| if aux { c.aux } else { c.main };
        macro_rules! take {
            ($l:ident, $n:literal) => {{
                let mut l = lane(self);
                let v = l.$l.next::<$n>()?;
                if aux { self.aux = l } else { self.main = l }
                v
            }};
        }
        Ok(match t {
            NULL => Value::Null,
            BOOLEAN_TRUE => Value::Bool(true),
            BOOLEAN_FALSE => Value::Bool(false),
            INT64_ZERO => Value::Int(0),
            INT64_ONE => Value::Int(1),
            DOUBLE_ZERO => Value::Double(0.0),
            DOUBLE_ONE => Value::Double(1.0),
            UINT32_ZERO => Value::UInt(0),
            UINT32_ONE => Value::UInt(1),
            BOOLEAN => Value::Bool(take!(l1, 1)[0] != 0),
            INT8 => Value::Int(take!(l1, 1)[0] as i8 as i64),
            UINT8 => Value::UInt(take!(l1, 1)[0] as u64),
            INT16 => Value::Int(i16::from_le_bytes(take!(l2, 2)) as i64),
            UINT16 => Value::UInt(u16::from_le_bytes(take!(l2, 2)) as u64),
            INT32 => Value::Int(i32::from_le_bytes(take!(l4, 4)) as i64),
            UINT32 => Value::UInt(u32::from_le_bytes(take!(l4, 4)) as u64),
            FLOAT => Value::Double(f32::from_le_bytes(take!(l4, 4)) as f64),
            INT64 => Value::Int(i64::from_le_bytes(take!(l8, 8))),
            UINT64 => Value::UInt(u64::from_le_bytes(take!(l8, 8))),
            DOUBLE => Value::Double(f64::from_le_bytes(take!(l8, 8))),
            STRING => {
                let id = i32::from_le_bytes(self.main.l4.next::<4>()?);
                Value::String(self.string(id))
            }
            BINARY_BLOB if self.version < 2 => {
                let n = i32::from_le_bytes(self.main.l4.next::<4>()?).max(0) as usize;
                if self.main.l1.b.len() < n { return err("KV3: a blob ran out"); }
                let (v, rest) = self.main.l1.b.split_at(n);
                self.main.l1.b = rest;
                Value::Blob(v.to_vec())
            }
            BINARY_BLOB => {
                let b = self.blobs.get(self.next_blob).cloned().ok_or(Error("KV3: out of blobs".into()))?;
                self.next_blob += 1;
                Value::Blob(b)
            }
            ARRAY => {
                let n = i32::from_le_bytes(self.main.l4.next::<4>()?).max(0) as usize;
                let mut a = Vec::with_capacity(n.min(1 << 16));
                for _ in 0..n {
                    let t = self.read_type()?;
                    a.push(self.value(t, false)?);
                }
                Value::Array(a)
            }
            ARRAY_TYPED | ARRAY_TYPE_BYTE_LENGTH | ARRAY_TYPE_AUXILIARY_BUFFER => {
                let n = if t == ARRAY_TYPED { i32::from_le_bytes(self.main.l4.next::<4>()?).max(0) as usize } else { self.main.l1.next::<1>()?[0] as usize };
                let sub = self.read_type()?;
                let elements_aux = t == ARRAY_TYPE_AUXILIARY_BUFFER;
                let mut a = Vec::with_capacity(n.min(1 << 16));
                for _ in 0..n { a.push(self.value(sub, elements_aux)?); }
                Value::Array(a)
            }
            OBJECT => {
                let n = if self.version >= 5 {
                    i32::from_le_bytes(self.object_lengths.next::<4>()?)
                } else {
                    i32::from_le_bytes(self.main.l4.next::<4>()?)
                }
                .max(0) as usize;
                let mut m = Vec::with_capacity(n.min(1 << 16));
                for _ in 0..n {
                    let t = self.read_type()?;
                    let id = i32::from_le_bytes(self.main.l4.next::<4>()?);
                    let name = self.string(id);
                    m.push((name, self.value(t, false)?));
                }
                Value::Object(m)
            }
            other => return err(format!("KV3: unknown type {other}")),
        })
    }
}

fn align(x: usize, a: usize) -> usize {
    x.div_ceil(a) * a
}

/// `n` values of `size` bytes from `at` in `b`, aligned to their size: the lane, and where it ends.
fn lane_at(b: &[u8], at: usize, n: usize, size: usize) -> Result<(Lane<'_>, usize), Error> {
    if n == 0 { return Ok((Lane::default(), at)); }
    let start = align(at, size);
    let end = start + n * size;
    if end > b.len() { return err("KV3: a lane is past its buffer"); }
    Ok((Lane { b: &b[start..end] }, end))
}

fn strings(b: &[u8], n: usize) -> Result<(Vec<String>, usize), Error> {
    let mut out = Vec::with_capacity(n);
    let mut at = 0;
    for _ in 0..n {
        let end = b[at..].iter().position(|&c| c == 0).ok_or(Error("KV3: an unterminated string".into()))? + at;
        out.push(String::from_utf8_lossy(&b[at..end]).into_owned());
        at = end + 1;
    }
    Ok((out, at))
}

fn decompress(method: u32, input: &[u8], size: usize) -> Result<Vec<u8>, Error> {
    match method {
        0 => input.get(..size).map(<[u8]>::to_vec).ok_or(Error("KV3: a buffer is short".into())),
        1 => {
            let mut out = vec![0u8; size];
            let n = lz4_flex::block::decompress_into(input, &mut out).map_err(|e| Error(format!("KV3: LZ4: {e}")))?;
            if n != size { return err(format!("KV3: LZ4 gave {n} bytes of {size}")); }
            Ok(out)
        }
        2 => err("KV3: zstd compression is not read yet"),
        m => err(format!("KV3: unknown compression {m}")),
    }
}

/// A block of binary KV3: its root value.
pub fn read(block: &[u8]) -> Result<Value, Error> {
    let mut c = Cursor::new(block);
    let magic = c.u32()?;
    if magic == 0x0356_4B56 { return err("KV3: version 0 (VKV3) is not read"); }
    if magic & 0xFFFF_FF00 != 0x4B56_3300 { return err(format!("KV3: not KV3 (magic {magic:#x})")); }
    let version = magic & 0xFF;
    if !(1..=5).contains(&version) { return err(format!("KV3: version {version}")); }
    c.take(16)?; // the format's GUID
    let method = c.u32()?;
    let (frame_size, n1, n4, n8, n_types, total, compressed_total, n_blocks, blob_bytes);
    if version == 1 {
        frame_size = 0u16;
        n1 = c.i32()? as usize;
        n4 = c.i32()? as usize;
        n8 = c.i32()? as usize;
        n_types = 0usize;
        total = c.i32()? as usize;
        compressed_total = c.rest().len();
        n_blocks = 0usize;
        blob_bytes = 0usize;
    } else {
        let dictionary = c.u16()?;
        if dictionary != 0 { return err("KV3: a compression dictionary"); }
        frame_size = c.u16()?;
        n1 = c.i32()? as usize;
        n4 = c.i32()? as usize;
        n8 = c.i32()? as usize;
        n_types = c.i32()? as usize;
        c.u16()?;
        c.u16()?;
        total = c.i32()? as usize;
        compressed_total = c.i32()? as usize;
        n_blocks = c.i32()? as usize;
        blob_bytes = c.i32()? as usize;
    }
    let mut n2 = 0usize;
    if version >= 4 {
        n2 = c.i32()? as usize;
        c.i32()?; // the size of the blobs' compressed frame sizes
    }
    let (mut size1, mut csize1, mut size2, mut csize2) = (total, compressed_total, 0usize, 0usize);
    let (mut b2n1, mut b2n2, mut b2n4, mut b2n8, mut b2objects) = (0usize, 0, 0, 0, 0);
    if version >= 5 {
        size1 = c.i32()? as usize;
        csize1 = c.i32()? as usize;
        size2 = c.i32()? as usize;
        csize2 = c.i32()? as usize;
        b2n1 = c.i32()? as usize;
        b2n2 = c.i32()? as usize;
        b2n4 = c.i32()? as usize;
        b2n8 = c.i32()? as usize;
        c.i32()?;
        b2objects = c.i32()? as usize;
        c.i32()?;
        c.i32()?;
    }

    // buffer 1: the lanes (and before version 5 everything else)
    let in1 = if method == 0 { c.take(size1)? } else { c.take(csize1)? };
    let buf1 = decompress(method, in1, size1)?;
    let (l1, at) = lane_at(&buf1, 0, n1, 1)?;
    let (l2, at) = lane_at(&buf1, at, n2, 2)?;
    let (l4, at) = lane_at(&buf1, at, n4, 4)?;
    let (l8, mut at) = lane_at(&buf1, at, n8, 8)?;
    if n8 == 0 && version < 5 { at = align(at, 8); }
    let mut main = Lanes { l1, l2, l4, l8 };
    let n_strings = i32::from_le_bytes(main.l4.next::<4>()?).max(0) as usize;

    let buf2;
    let mut ctx_types: &[u8];
    let mut object_lengths = Lane::default();
    let sizes_tail: &[u8];
    let strings_list;
    let aux;
    if version >= 5 {
        let (s, _) = strings(main.l1.b, n_strings)?;
        // the strings sit at the end of lane 1
        let used: usize = s.iter().map(|x| x.len() + 1).sum();
        main.l1.b = &main.l1.b[used..];
        strings_list = s;
        aux = main;
        let in2 = if method == 0 { c.take(size2)? } else { c.take(csize2)? };
        buf2 = decompress(method, in2, size2)?;
        let end = b2objects * 4;
        if end > buf2.len() { return err("KV3: object lengths past the buffer"); }
        object_lengths = Lane { b: &buf2[..end] };
        let (m1, at2) = lane_at(&buf2, end, b2n1, 1)?;
        let (m2, at2) = lane_at(&buf2, at2, b2n2, 2)?;
        let (m4, at2) = lane_at(&buf2, at2, b2n4, 4)?;
        let (m8, at2) = lane_at(&buf2, at2, b2n8, 8)?;
        main = Lanes { l1: m1, l2: m2, l4: m4, l8: m8 };
        ctx_types = buf2.get(at2..at2 + n_types).ok_or(Error("KV3: types past the buffer".into()))?;
        sizes_tail = &buf2[at2 + n_types..];
    } else {
        let (s, used) = strings(&buf1[at..], n_strings)?;
        strings_list = s;
        let types_start = at + used;
        let types_len = if version == 1 { total.saturating_sub(types_start + 4) } else { n_types.saturating_sub(used) };
        ctx_types = buf1.get(types_start..types_start + types_len).ok_or(Error("KV3: types past the buffer".into()))?;
        sizes_tail = &buf1[types_start + types_len..];
        aux = main;
    }

    // the binary blobs, after the buffers
    let mut blobs = Vec::new();
    if n_blocks == 0 {
        if sizes_tail.len() >= 4 && u32::from_le_bytes(sizes_tail[..4].try_into().unwrap()) != TRAILER {
            return err("KV3: no trailer");
        }
    } else {
        let mut t = Cursor::new(sizes_tail);
        let lengths: Vec<usize> = (0..n_blocks).map(|_| t.i32().map(|n| n.max(0) as usize)).collect::<Result<_, _>>()?;
        if t.u32()? != TRAILER { return err("KV3: no trailer after the blob lengths"); }
        if lengths.iter().sum::<usize>() != blob_bytes { return err("KV3: blob lengths don't add up"); }
        match method {
            0 => {
                for &n in &lengths { blobs.push(c.take(n)?.to_vec()); }
            }
            1 => {
                // each blob in frames of at most frame_size, chained: a frame may refer back
                let mut out = Vec::with_capacity(blob_bytes);
                for &n in &lengths {
                    let start = out.len();
                    while out.len() - start < n {
                        let frame = t.u16()? as usize;
                        let input = c.take(frame)?;
                        let want = (n - (out.len() - start)).min(frame_size.max(1) as usize);
                        let decoded = lz4_flex::block::decompress_with_dict(input, want, &out[out.len().saturating_sub(65536)..])
                            .map_err(|e| Error(format!("KV3: LZ4 blob: {e}")))?;
                        if decoded.is_empty() { return err("KV3: an empty LZ4 blob frame"); }
                        out.extend_from_slice(&decoded);
                    }
                    blobs.push(out[start..start + n].to_vec());
                }
            }
            _ => return err("KV3: compressed blobs of this kind are not read yet"),
        }
        if c.u32()? != TRAILER { return err("KV3: no trailer after the blobs"); }
    }

    let mut ctx = Context { version, types: ctx_types, object_lengths, strings: strings_list, blobs, next_blob: 0, main, aux };
    let t = ctx.read_type()?;
    let v = ctx.value(t, false)?;
    ctx_types = ctx.types;
    let _ = ctx_types;
    Ok(v)
}

/// A value as KV3's text, for looking at.
pub fn text(v: &Value, indent: usize, out: &mut String) {
    let pad = "\t".repeat(indent);
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(&b.to_string()),
        Value::Int(i) => out.push_str(&i.to_string()),
        Value::UInt(u) => out.push_str(&u.to_string()),
        Value::Double(d) => out.push_str(&format!("{d}")),
        Value::String(s) => out.push_str(&format!("{s:?}")),
        Value::Blob(b) => out.push_str(&format!("#[{} bytes]", b.len())),
        Value::Array(a) => {
            if a.iter().all(|x| x.as_f64().is_some()) && a.len() <= 16 {
                let items: Vec<String> = a.iter().map(|x| { let mut s = String::new(); text(x, 0, &mut s); s }).collect();
                out.push_str(&format!("[{}]", items.join(", ")));
            } else {
                out.push_str("[\n");
                for x in a {
                    out.push_str(&pad);
                    out.push('\t');
                    text(x, indent + 1, out);
                    out.push_str(",\n");
                }
                out.push_str(&pad);
                out.push(']');
            }
        }
        Value::Object(m) => {
            out.push_str("{\n");
            for (k, x) in m {
                out.push_str(&format!("{pad}\t{k} = "));
                text(x, indent + 1, out);
                out.push('\n');
            }
            out.push_str(&pad);
            out.push('}');
        }
    }
}
