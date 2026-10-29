//! The engine's file system, as far as reading compiled resources needs it:
//! managed links each file it knows of (a project's, one downloaded from
//! sbox.game) to where it is on disk (AddSymLink), and names project
//! folders (AddProjectPath); a resource is looked for in those, then in the
//! game's own folders.
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Default)]
pub struct Fs {
    links: HashMap<String, String>,
    projects: Vec<(String, String)>,
    cloud: Vec<(String, String)>,
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
/// cloud folder or game folder (`game`, `game/core`, `game/addons/*`) that has it.
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
    let rel = path.replace('\\', "/");
    let rel = rel.trim_start_matches('/');
    for r in roots {
        let p = r.join(rel);
        if p.is_file() { return Some(p); }
        // the engine's names are case blind; disks may not be
        let p = r.join(&key);
        if p.is_file() { return Some(p); }
    }
    None
}

/// A compiled resource by its name (`models/x.vmdl` reads `models/x.vmdl_c`).
pub fn read_compiled(name: &str) -> Option<Vec<u8>> {
    let compiled = if name.ends_with("_c") { name.to_string() } else { format!("{name}_c") };
    let path = find(&compiled)?;
    std::fs::read(path).ok()
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
        std::fs::remove_dir_all(dir).unwrap();
    }
}
