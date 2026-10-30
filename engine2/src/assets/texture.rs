//! Textures (`.vtex_c`): a DATA block that describes the texture (its size,
//! format, mip count, flags and some extra data), and after it the pixels,
//! the smallest mip first (so a file's beginning is a usable texture, as
//! sbox.game's web streaming takes it), each mip LZ4- or zstd-compressed
//! where the extra data says so. Decoded here to RGBA8.
use super::resource::Resource;

/// The formats a texture's pixels come in (VTexFormat).
pub mod format {
    pub const DXT1: u8 = 1;
    pub const DXT5: u8 = 2;
    pub const I8: u8 = 3;
    pub const RGBA8888: u8 = 4;
    pub const BC6H: u8 = 19;
    pub const BC7: u8 = 20;
    pub const ATI2N: u8 = 21;
    pub const IA88: u8 = 22;
    pub const ATI1N: u8 = 27;
    pub const BGRA8888: u8 = 28;
}

const CUBE: u16 = 0x4;
const COMPRESSED_MIP_SIZE: u32 = 4;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub format: u8,
    pub mips: usize,
    pub flags: u16,
    /// how its mips are compressed (1 LZ4, 2 zstd), and each one's size on
    /// disk by level (one no smaller than it was is stored as it is)
    compressed: Option<(u32, Vec<usize>)>,
    /// where the pixels start
    data: usize,
    bytes: Vec<u8>,
}

fn u16_at(b: &[u8], at: usize) -> Result<u16, String> {
    b.get(at..at + 2).map(|x| u16::from_le_bytes(x.try_into().unwrap())).ok_or_else(|| "vtex: too short".to_string())
}

fn u32_at(b: &[u8], at: usize) -> Result<u32, String> {
    b.get(at..at + 4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).ok_or_else(|| "vtex: too short".to_string())
}

pub fn read(r: Resource) -> Result<Texture, String> {
    let &(_, at, size) = r.blocks.iter().find(|b| &b.0 == b"DATA").ok_or("vtex: no DATA")?;
    let b = &r.bytes;
    if u16_at(b, at)? != 1 { return Err(format!("vtex: version {}", u16_at(b, at)?)); }
    let mut t = Texture {
        flags: u16_at(b, at + 2)?,
        width: u16_at(b, at + 20)? as usize,
        height: u16_at(b, at + 22)? as usize,
        depth: u16_at(b, at + 24)?.max(1) as usize,
        format: *b.get(at + 26).ok_or("vtex: too short")?,
        mips: (*b.get(at + 27).ok_or("vtex: too short")?).max(1) as usize,
        compressed: None,
        data: at + size,
        bytes: Vec::new(),
    };
    // the extra data: each entry its type, where it is (from that field) and its size
    let extra = at + 32 + u32_at(b, at + 32)? as usize;
    for i in 0..u32_at(b, at + 36)? as usize {
        let e = extra + i * 12;
        if u32_at(b, e)? != COMPRESSED_MIP_SIZE { continue; }
        let x = e + 4 + u32_at(b, e + 4)? as usize;
        let method = u32_at(b, x)?;
        let sizes = x + 4 + u32_at(b, x + 4)? as usize;
        let n = u32_at(b, x + 8)? as usize;
        if method != 0 { t.compressed = Some((method, (0..n).map(|k| u32_at(b, sizes + k * 4).map(|s| s as usize)).collect::<Result<_, _>>()?)); }
    }
    t.bytes = r.bytes;
    Ok(t)
}

impl Texture {
    /// 6 for a cube, else 1.
    pub fn faces(&self) -> usize { if self.flags & CUBE != 0 { 6 } else { 1 } }

    /// A mip's size, in pixels.
    pub fn size(&self, mip: usize) -> (usize, usize) {
        ((self.width >> mip).max(1), (self.height >> mip).max(1))
    }

    /// A mip's bytes, decompressed: every face and slice of it.
    fn mip_len(&self, mip: usize) -> usize {
        let (w, h) = self.size(mip);
        let d = (self.depth >> mip).max(1);
        let blocks = w.div_ceil(4) * h.div_ceil(4);
        d * self.faces() * match self.format {
            format::DXT1 | format::ATI1N => blocks * 8,
            format::DXT5 | format::ATI2N | format::BC6H | format::BC7 => blocks * 16,
            format::I8 => w * h,
            format::IA88 => w * h * 2,
            _ => w * h * 4,
        }
    }

    /// The largest mip the file holds (a web-streamed prefix holds only the
    /// small ones): the smallest mip it can give that's at least `mip`.
    pub fn available(&self, mip: usize) -> Option<usize> {
        let mut at = self.data;
        let mut best = None;
        for level in (0..self.mips).rev() {
            at += self.stored_len(level);
            if at > self.bytes.len() { break; }
            best = Some(level);
            if level <= mip { break; }
        }
        best
    }

    fn stored_len(&self, mip: usize) -> usize {
        self.compressed.as_ref().and_then(|c| c.1.get(mip).copied()).unwrap_or_else(|| self.mip_len(mip))
    }

    /// A mip's raw pixels (its first face and slice), decompressed: blocks,
    /// for the block formats, as a GPU takes them.
    pub fn raw(&self, mip: usize) -> Result<Vec<u8>, String> {
        let mut at = self.data;
        for level in (mip + 1..self.mips).rev() { at += self.stored_len(level); }
        let stored = self.bytes.get(at..at + self.stored_len(mip)).ok_or("vtex: mip past the end")?;
        let n = self.mip_len(mip);
        match self.compressed.as_ref().map(|c| c.0) {
            _ if stored.len() >= n => Ok(stored.to_vec()),
            Some(1) => lz4_flex::block::decompress(stored, n).map_err(|e| format!("vtex: {e}")),
            Some(2) => {
                let mut out = Vec::with_capacity(n);
                let mut d = ruzstd::decoding::StreamingDecoder::new(stored).map_err(|e| format!("vtex: {e}"))?;
                std::io::Read::read_to_end(&mut d, &mut out).map_err(|e| format!("vtex: {e}"))?;
                Ok(out)
            }
            m => Err(format!("vtex: mips compressed by method {m:?}")),
        }
    }

    /// A mip as RGBA8, top row first: (width, height, pixels).
    pub fn rgba(&self, mip: usize) -> Result<(usize, usize, Vec<u8>), String> {
        let mip = self.available(mip).ok_or("vtex: no mip in the file")?;
        let (w, h) = self.size(mip);
        let raw = self.raw(mip)?;
        let mut words = vec![0u32; w * h];
        use texture2ddecoder as t;
        let decoded = match self.format {
            format::DXT1 => t::decode_bc1(&raw, w, h, &mut words),
            format::DXT5 => t::decode_bc3(&raw, w, h, &mut words),
            format::ATI1N => t::decode_bc4(&raw, w, h, &mut words),
            format::ATI2N => t::decode_bc5(&raw, w, h, &mut words),
            format::BC6H => t::decode_bc6(&raw, w, h, &mut words, false),
            format::BC7 => t::decode_bc7(&raw, w, h, &mut words),
            format::RGBA8888 => return Ok((w, h, raw[..w * h * 4].to_vec())),
            format::BGRA8888 => return Ok((w, h, raw[..w * h * 4].chunks_exact(4).flat_map(|p| [p[2], p[1], p[0], p[3]]).collect())),
            format::I8 => return Ok((w, h, raw[..w * h].iter().flat_map(|&l| [l, l, l, 255]).collect())),
            format::IA88 => return Ok((w, h, raw[..w * h * 2].chunks_exact(2).flat_map(|p| [p[0], p[0], p[0], p[1]]).collect())),
            f => return Err(format!("vtex: format {f} not read yet")),
        };
        decoded.map_err(|e| format!("vtex: {e}"))?;
        // the decoder's words are BGRA in memory
        Ok((w, h, words.iter().flat_map(|p| { let [b, g, r, a] = p.to_le_bytes(); [r, g, b, a] }).collect()))
    }
}
