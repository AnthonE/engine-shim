//! Compiled Source 2 assets, read: the resource container, binary KV3 in
//! it, and what physics needs of a model (models.rs makes the model).
pub mod anim;
pub mod entities;
pub mod kv3;
pub mod kv3text;
pub mod material;
pub mod mesh;
pub mod model;
pub mod resource;
pub mod texture;
pub mod vpk;

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

    /// Every model in a cache read whole: its skeleton, meshes, hitboxes,
    /// attachments and sequences (`... -- --ignored every_model`).
    #[test]
    #[ignore]
    fn every_model() {
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let mut stack = vec![std::path::PathBuf::from(dir)];
        let (mut models, mut bones, mut hitboxes, mut attachments, mut sequences, mut meshes, mut clips, mut playable) = (0, 0, 0, 0, 0, 0, 0, 0);
        let (mut keyed, mut props, mut breaks) = (0, 0, 0);
        while let Some(d) = stack.pop() {
            for e in std::fs::read_dir(d).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() { stack.push(p); continue; }
                if !p.to_string_lossy().ends_with(".vmdl_c") { continue; }
                let r = super::resource::Resource::parse(std::fs::read(&p).unwrap()).unwrap();
                let m = super::model::read(&r, |_| None).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
                models += 1;
                bones += m.bones.len();
                hitboxes += m.hitbox_sets(0).iter().map(|s| s.boxes.len()).sum::<usize>();
                attachments += m.attachments().len();
                sequences += m.sequences.len();
                let names: Vec<String> = m.bones.iter().map(|b| b.name.to_string_lossy().into_owned()).collect();
                let bind: Vec<super::anim::BonePose> = m.bones.iter().map(|b| super::anim::BonePose { pos: b.pos, rot: b.rot, scale: 1.0 }).collect();
                let a = super::anim::read(&r, &names);
                for c in 0..a.clips.len() {
                    for p in a.sample(c, 0.5, &bind) {
                        assert!(p.pos.is_finite() && p.rot.is_finite(), "{}: clip {c}", p.pos);
                    }
                }
                clips += a.clips.len();
                playable += a.sequences.len();
                meshes += m.meshes.iter().filter(|x| !x.objects.is_empty()).count();
                // key values written as text read, every one
                let text = r.kv3("DATA").and_then(|d| d.ok()).and_then(|d| d.get("m_modelInfo")?.get("m_keyValueText")?.as_str().map(String::from)).unwrap_or_default();
                if !text.trim().is_empty() {
                    let kv = m.key_values.as_ref().unwrap_or_else(|| panic!("{}: {:?}", p.display(), super::kv3text::read(&text).err()));
                    keyed += 1;
                    props += kv.get("prop_data").is_some() as usize;
                    breaks += kv.get("break_list").map(|b| b.as_array().len()).unwrap_or(0);
                }
                assert!(m.render_bounds().is_some(), "{}: no meshes", p.display());
            }
        }
        eprintln!("{models} models: {meshes} meshes, {bones} bones, {hitboxes} hitboxes, {attachments} attachments, {sequences} sequences ({playable} playing one of {clips} clips)");
        eprintln!("{keyed} with key values: {props} with prop_data, {breaks} break pieces");
    }
}

