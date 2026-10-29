//! Compiled Source 2 assets, read: the resource container, binary KV3 in
//! it, and what physics needs of a model (models.rs makes the model).
pub mod kv3;
pub mod resource;

#[cfg(test)]
mod tests {
    use super::*;

    /// A downloaded model, if this machine has one (s&box's own download
    /// cache: Facepunch's content, read where it lies, never copied here).
    fn sample(name: &str) -> Option<Vec<u8>> {
        let dir = std::env::var("ENGINE2_SAMPLE_ASSETS").ok()?;
        let path = std::fs::read_dir(format!("{dir}/{}", name.rsplit_once('/')?.0)).ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| p.file_name().is_some_and(|f| f.to_string_lossy().starts_with(name.rsplit_once('/').unwrap().1) && f.to_string_lossy().ends_with(".vmdl_c")))?;
        std::fs::read(path).ok()
    }

    #[test]
    fn a_downloaded_models_blocks_read_as_kv3() {
        let Some(bytes) = sample("models/props/washing_machine/washing_machine") else {
            eprintln!("no ENGINE2_SAMPLE_ASSETS: skipped");
            return;
        };
        let r = resource::Resource::parse(bytes).unwrap();
        for tag in ["DATA", "PHYS", "CTRL"] {
            let v = r.kv3(tag).unwrap().unwrap_or_else(|e| panic!("{tag}: {e}"));
            if std::env::var("ENGINE2_DUMP").is_ok_and(|d| d == tag) {
                let mut s = String::new();
                kv3::text(&v, 0, &mut s);
                eprintln!("{tag}:\n{s}");
            }
        }
    }
}

#[cfg(test)]
mod sweep {
    /// Every compiled resource in a cache, every KV3 block read
    /// (`ENGINE2_SAMPLE_ASSETS=... cargo test --release -- --ignored every_block`).
    #[test]
    #[ignore]
    fn every_block() {
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let mut files = Vec::new();
        let mut stack = vec![std::path::PathBuf::from(dir)];
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(d).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() { stack.push(p) } else if p.to_string_lossy().ends_with("_c") { files.push(p) }
            }
        }
        let (mut blocks, mut failed) = (0, std::collections::BTreeMap::<String, usize>::new());
        for f in &files {
            let Ok(r) = super::resource::Resource::parse(std::fs::read(f).unwrap()) else { continue };
            for &(tag, at, n) in &r.blocks {
                let b = &r.bytes[at..at + n];
                if b.len() < 4 || u32::from_le_bytes(b[..4].try_into().unwrap()) & 0xFFFF_FF00 != 0x4B56_3300 { continue; }
                blocks += 1;
                if let Err(e) = super::kv3::read(b) {
                    *failed.entry(format!("{} {}", String::from_utf8_lossy(&tag), e)).or_default() += 1;
                }
            }
        }
        eprintln!("{} files, {blocks} KV3 blocks, failures: {failed:#?}", files.len());
        assert!(failed.is_empty());
    }
}
