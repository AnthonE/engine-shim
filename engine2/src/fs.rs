//! The engine's file system, as far as reading compiled resources needs it:
//! managed links each file it knows of (a project's, one downloaded from
//! sbox.game) to where it is on disk (AddSymLink), and names project
//! folders (AddProjectPath); a resource is looked for in those, then in the
//! game's own folders, then in the paks mounted (a map's `.vpk`).
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use crate::assets::vpk::Vpk;

#[derive(Default)]
pub struct Fs {
    links: HashMap<String, String>,
    projects: Vec<(String, String)>,
    cloud: Vec<(String, String)>,
    /// mounted paks, by the name they were mounted as
    paks: Vec<(String, Arc<Vpk>)>,
}

fn fs() -> MutexGuard<'static, Fs> {
    static F: OnceLock<Mutex<Fs>> = OnceLock::new();
    F.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// A path as the engine compares them: forward slashes, lower case, no
/// leading slash.
pub fn normal(path: &str) -> String {
    path.replace('\\', "/").trim_start_matches('/').to_ascii_lowercase()
}

pub fn add_link(path: &str, target: &str) {
    fs().links.insert(normal(path), target.to_string());
}

pub fn remove_link(path: &str) {
    fs().links.remove(&normal(path));
}

pub fn link(path: &str) -> Option<String> {
    fs().links.get(&normal(path)).cloned()
}

pub fn add_project(ident: &str, path: &str) {
    let mut f = fs();
    if !f.projects.iter().any(|(_, p)| p == path) { f.projects.push((ident.to_string(), path.to_string())); }
}

pub fn add_cloud(ident: &str, path: &str) {
    let mut f = fs();
    if !f.cloud.iter().any(|(_, p)| p == path) { f.cloud.push((ident.to_string(), path.to_string())); }
}

pub fn reset_projects(including_cloud: bool) {
    let mut f = fs();
    f.projects.clear();
    if including_cloud { f.cloud.clear(); }
}

/// Where a game-relative file is: its link, else the first project folder,
/// cloud folder or game folder (`game`, `game/core`, `game/addons/*`) that
/// has it, else the content folders named in `ENGINE2_CONTENT` (a player's
/// copy of a map pak, which a player of a local project has no other way to).
pub fn find(path: &str) -> Option<PathBuf> {
    let key = normal(path);
    let (link, folders) = {
        let f = fs();
        let folders: Vec<String> = f.projects.iter().chain(&f.cloud).map(|(_, p)| p.clone()).collect();
        (f.links.get(&key).cloned(), folders)
    };
    if let Some(l) = link {
        let p = PathBuf::from(l);
        if p.is_file() { return Some(p); }
    }
    let root = PathBuf::from(crate::impls::game_root());
    let mut roots: Vec<PathBuf> = folders.into_iter().map(PathBuf::from).collect();
    roots.push(root.join("core"));
    roots.push(root.clone());
    if let Ok(addons) = std::fs::read_dir(root.join("addons")) {
        let mut a: Vec<PathBuf> = addons.filter_map(|e| e.ok()).map(|e| e.path().join("Assets")).collect();
        a.sort();
        roots.extend(a);
    }
    if let Ok(more) = std::env::var("ENGINE2_CONTENT") {
        roots.extend(more.split(':').filter(|d| !d.is_empty()).map(PathBuf::from));
    }
    let rel = path.replace('\\', "/");
    let rel = rel.trim_start_matches('/');
    for r in roots {
        let p = r.join(rel);
        if p.is_file() { return Some(p); }
        // the engine's names are case blind (managed names a project's
        // folder in lower case); disks may not be
        if let Some(p) = case_blind(&p).filter(|p| p.is_file()) { return Some(p); }
    }
    None
}

/// `path` as it is on disk, each part matched without regard to case.
fn case_blind(path: &std::path::Path) -> Option<PathBuf> {
    let mut at = PathBuf::new();
    for part in path.components() {
        let next = at.join(part);
        if next.exists() || matches!(part, std::path::Component::RootDir | std::path::Component::Prefix(_)) {
            at = next;
            continue;
        }
        let want = part.as_os_str().to_string_lossy().to_lowercase();
        let found = std::fs::read_dir(if at.as_os_str().is_empty() { std::path::Path::new(".") } else { &at }).ok()?
            .filter_map(|e| e.ok())
            .find(|e| e.file_name().to_string_lossy().to_lowercase() == want)?;
        at = at.join(found.file_name());
    }
    Some(at)
}

/// Mounts the pak at the game-relative `path` (found as any file is) as
/// `name`: its files are found after the disk's. Mounted again, it's the
/// same pak.
pub fn mount(name: &str, path: &str) -> Result<usize, String> {
    let key = normal(name);
    if let Some((_, p)) = fs().paks.iter().find(|(n, _)| *n == key) { return Ok(p.len()); }
    let disk = find(path).ok_or_else(|| format!("{path}: not found"))?;
    let pak = Arc::new(Vpk::open(&disk)?);
    let n = pak.len();
    fs().paks.push((key, pak));
    Ok(n)
}

pub fn unmount(name: &str) -> bool {
    let key = normal(name);
    let mut f = fs();
    let n = f.paks.len();
    f.paks.retain(|(k, _)| *k != key);
    f.paks.len() != n
}

/// A game-relative file's bytes: from disk, else from a mounted pak.
pub fn read(path: &str) -> Option<Vec<u8>> {
    if let Some(p) = find(path) { return std::fs::read(p).ok(); }
    let paks: Vec<Arc<Vpk>> = fs().paks.iter().map(|(_, p)| p.clone()).collect();
    paks.iter().find_map(|p| p.read(path))
}

/// A compiled resource by its name (`models/x.vmdl` reads `models/x.vmdl_c`).
pub fn read_compiled(name: &str) -> Option<Vec<u8>> {
    let compiled = if name.ends_with("_c") { name.to_string() } else { format!("{name}_c") };
    read(&compiled)
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_link_wins_and_names_are_case_blind() {
        let dir = std::env::temp_dir().join(format!("engine2-fs-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("models")).unwrap();
        let file = dir.join("models").join("box.4f2a.vmdl_c");
        std::fs::write(&file, b"x").unwrap();
        super::add_link("/Models/Box.vmdl_c", file.to_str().unwrap());
        assert_eq!(super::read_compiled("models/box.vmdl").unwrap(), b"x");
        super::remove_link("models/box.vmdl_c");
        assert!(super::read_compiled("models/box.vmdl").is_none());
        // a project folder named in lower case, on a disk that isn't case blind
        std::fs::create_dir_all(dir.join("Assets").join("Maps")).unwrap();
        std::fs::write(dir.join("Assets").join("Maps").join("Test.vpk"), b"pak").unwrap();
        super::add_project("test", &dir.join("assets").to_string_lossy());
        assert_eq!(super::read("maps/test.vpk").unwrap(), b"pak");
        super::reset_projects(false);
        std::fs::remove_dir_all(dir).unwrap();
    }
}
