//! Valve's pak files (`.vpk`): a map comes as one (`maps/flatgrass.vpk`,
//! its world, collision, entities and meshes inside). A directory of
//! files by extension, folder and name, each with a few bytes kept in the
//! directory itself and the rest in an archive: this file (index 0x7fff),
//! or a numbered one beside it (`x_000.vpk` for `x_dir.vpk`). Versions 1
//! and 2 (2 adds sections after the data this reader has no need of).
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const SIGNATURE: u32 = 0x55aa_1234;
const IN_THIS_FILE: u16 = 0x7fff;

struct Entry {
    preload: Vec<u8>,
    archive: u16,
    offset: u32,
    length: u32,
}

pub struct Vpk {
    path: PathBuf,
    /// where this file's own data starts (after the header and directory)
    data: u64,
    /// by path, as the engine compares them (fs::normal)
    entries: HashMap<String, Entry>,
}

impl Vpk {
    pub fn open(path: &Path) -> Result<Vpk, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let u32_at = |at: usize| bytes.get(at..at + 4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).ok_or("vpk: too short");
        if u32_at(0)? != SIGNATURE { return Err(format!("{}: not a vpk", path.display())); }
        let header = match u32_at(4)? {
            1 => 12,
            2 => 28,
            v => return Err(format!("vpk: version {v}")),
        };
        let tree = u32_at(8)? as usize;
        let end = header + tree;
        if end > bytes.len() { return Err("vpk: directory past the end".into()); }
        let mut at = header;
        let cstr = |at: &mut usize| -> Result<String, String> {
            let n = bytes[*at..end].iter().position(|&c| c == 0).ok_or("vpk: unterminated name")?;
            let s = String::from_utf8_lossy(&bytes[*at..*at + n]).into_owned();
            *at += n + 1;
            Ok(s)
        };
        let mut entries = HashMap::new();
        loop {
            let ext = cstr(&mut at)?;
            if ext.is_empty() { break; }
            loop {
                let folder = cstr(&mut at)?;
                if folder.is_empty() { break; }
                loop {
                    let name = cstr(&mut at)?;
                    if name.is_empty() { break; }
                    // crc, preload size, archive, offset, length, terminator
                    let e = bytes.get(at..at + 18).ok_or("vpk: entry past the end")?;
                    let preload_len = u16::from_le_bytes([e[4], e[5]]) as usize;
                    let entry = Entry {
                        archive: u16::from_le_bytes([e[6], e[7]]),
                        offset: u32::from_le_bytes(e[8..12].try_into().unwrap()),
                        length: u32::from_le_bytes(e[12..16].try_into().unwrap()),
                        preload: bytes.get(at + 18..at + 18 + preload_len).ok_or("vpk: preload past the end")?.to_vec(),
                    };
                    at += 18 + preload_len;
                    // a folder of " " is the root; an extension of " " is none
                    let mut file = if folder.trim().is_empty() { name } else { format!("{folder}/{name}") };
                    if !ext.trim().is_empty() { file = format!("{file}.{ext}"); }
                    entries.insert(crate::fs::normal(&file), entry);
                }
            }
        }
        Ok(Vpk { path: path.to_path_buf(), data: end as u64, entries })
    }

    /// The paths it holds.
    pub fn paths(&self) -> impl Iterator<Item = &str> {
        self.entries.keys().map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// A file's bytes.
    pub fn read(&self, path: &str) -> Option<Vec<u8>> {
        use std::io::{Read, Seek, SeekFrom};
        let e = self.entries.get(&crate::fs::normal(path))?;
        let mut out = e.preload.clone();
        if e.length > 0 {
            let (file, start) = if e.archive == IN_THIS_FILE {
                (self.path.clone(), self.data + e.offset as u64)
            } else {
                let name = self.path.file_name()?.to_string_lossy().replace("_dir.vpk", &format!("_{:03}.vpk", e.archive));
                (self.path.with_file_name(name), e.offset as u64)
            };
            let mut f = std::fs::File::open(file).ok()?;
            f.seek(SeekFrom::Start(start)).ok()?;
            let mut rest = vec![0u8; e.length as usize];
            f.read_exact(&mut rest).ok()?;
            out.extend(rest);
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A version 2 pak, its files in itself: (path, preload, data).
    pub fn pak(files: &[(&str, &[u8], &[u8])]) -> Vec<u8> {
        let mut tree = Vec::new();
        let mut data = Vec::new();
        for (path, preload, body) in files {
            let (stem, ext) = path.rsplit_once('.').unwrap();
            let (folder, name) = stem.rsplit_once('/').unwrap_or((" ", stem));
            for s in [ext, folder, name] { tree.extend_from_slice(s.as_bytes()); tree.push(0); }
            tree.extend_from_slice(&0u32.to_le_bytes());
            tree.extend_from_slice(&(preload.len() as u16).to_le_bytes());
            tree.extend_from_slice(&IN_THIS_FILE.to_le_bytes());
            tree.extend_from_slice(&(data.len() as u32).to_le_bytes());
            tree.extend_from_slice(&(body.len() as u32).to_le_bytes());
            tree.extend_from_slice(&0xffffu16.to_le_bytes());
            tree.extend_from_slice(preload);
            data.extend_from_slice(body);
            // each file its own extension and folder: end both lists
            tree.extend_from_slice(&[0, 0]);
        }
        tree.push(0);
        let mut out = Vec::new();
        for v in [SIGNATURE, 2, tree.len() as u32, data.len() as u32, 0, 0, 0] { out.extend_from_slice(&v.to_le_bytes()); }
        out.extend(tree);
        out.extend(data);
        out
    }

    #[test]
    fn a_pak_reads_its_files() {
        let dir = std::env::temp_dir().join(format!("engine2-vpk-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.vpk");
        std::fs::write(&path, pak(&[("maps/test/world_physics.vphys_c", b"", b"physics"), ("maps/test.vmap_c", b"pre", b"loaded"), ("readme.txt", b"", b"")])).unwrap();
        let v = Vpk::open(&path).unwrap();
        assert_eq!(v.len(), 3);
        assert_eq!(v.read("Maps/Test/World_Physics.vphys_c").unwrap(), b"physics");
        assert_eq!(v.read("/maps/test.vmap_c").unwrap(), b"preloaded", "its preload, then its data");
        assert_eq!(v.read("readme.txt").unwrap(), b"");
        assert!(v.read("maps/nothing.vmap_c").is_none());
        assert!(Vpk::open(&dir.join("missing.vpk")).is_err());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_downloaded_map_reads() {
        // a map from sbox.game (Facepunch's, read where it lies), if this machine has one
        let Ok(path) = std::env::var("ENGINE2_SAMPLE_MAP") else { return };
        let v = Vpk::open(Path::new(&path)).unwrap();
        let name = Path::new(&path).file_stem().unwrap().to_string_lossy().to_string();
        let phys = v.read(&format!("maps/{name}/world_physics.vphys_c")).expect("its collision");
        crate::assets::resource::Resource::parse(phys).unwrap().kv3("DATA").unwrap().unwrap();
    }
}
