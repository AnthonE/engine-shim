//! Source 2's compiled resource container (a `.vmdl_c`, a `.vphys_c`...): a
//! header and a list of blocks by four-letter tag (DATA, PHYS, MDAT...),
//! most of them binary KV3.

pub struct Resource {
    pub bytes: Vec<u8>,
    pub version: u16,
    /// tag, offset, size
    pub blocks: Vec<([u8; 4], usize, usize)>,
}

impl Resource {
    pub fn parse(bytes: Vec<u8>) -> Result<Resource, String> {
        let u32_at = |b: &[u8], at: usize| b.get(at..at + 4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).ok_or("resource: too short".to_string());
        let header_size = bytes.get(4..6).map(|x| u16::from_le_bytes(x.try_into().unwrap())).ok_or("resource: too short")?;
        let version = bytes.get(6..8).map(|x| u16::from_le_bytes(x.try_into().unwrap())).ok_or("resource: too short")?;
        if header_size != 12 { return Err(format!("resource: a header of {header_size} bytes, not 12")); }
        let block_offset = u32_at(&bytes, 8)? as usize;
        let count = u32_at(&bytes, 12)? as usize;
        let mut blocks = Vec::with_capacity(count);
        let mut at = 8 + block_offset;
        for _ in 0..count {
            let tag: [u8; 4] = bytes.get(at..at + 4).ok_or("resource: block list past the end")?.try_into().unwrap();
            let offset = u32_at(&bytes, at + 4)? as usize;
            let size = u32_at(&bytes, at + 8)? as usize;
            let start = at + 4 + offset;
            if start + size > bytes.len() { return Err(format!("resource: block {} past the end", String::from_utf8_lossy(&tag))); }
            blocks.push((tag, start, size));
            at += 12;
        }
        Ok(Resource { bytes, version, blocks })
    }

    /// The first block of a tag.
    pub fn block(&self, tag: &str) -> Option<&[u8]> {
        self.blocks.iter().find(|(t, _, _)| t == tag.as_bytes()).map(|&(_, at, n)| &self.bytes[at..at + n])
    }

    /// The first block of a tag, as KV3.
    pub fn kv3(&self, tag: &str) -> Option<Result<super::kv3::Value, super::kv3::Error>> {
        self.block(tag).map(super::kv3::read)
    }
}
