//! Maps: a world managed creates from a map's pak (`maps/flatgrass.vpk`),
//! mounted so its files are found (`maps/flatgrass/...`), with its folder
//! and its entity lumps (`entities/<lump>.vents_c`), each entity's keys
//! handed to managed as a `CEntityKeyValues`. Its collision is a physics
//! resource managed loads by name (models::load_physics), its game objects
//! a scene it reads itself (`world.scene_c`). Nothing here draws it.
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::assets::entities::Entity;

pub struct World {
    /// `maps/flatgrass`
    pub folder: CString,
    /// the scene world it is in
    pub scene_world: usize,
    /// its lumps, read when first asked for: each entity at an address of its own
    lumps: HashMap<String, Vec<&'static Entity>>,
}

fn worlds() -> MutexGuard<'static, HashMap<usize, World>> {
    static W: OnceLock<Mutex<HashMap<usize, World>>> = OnceLock::new();
    W.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// Every entity handed out, so only these are read as one.
fn entities() -> MutexGuard<'static, std::collections::HashSet<usize>> {
    static E: OnceLock<Mutex<std::collections::HashSet<usize>>> = OnceLock::new();
    E.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// A world for the map pak `map` (`maps/flatgrass.vpk`): mounted, its
/// object. None if the pak can't be found or read.
pub fn create(map: &str, scene_world: usize) -> Option<usize> {
    let folder = map.trim_end_matches(".vpk").trim_end_matches(".vmap").trim_start_matches('/').to_string();
    if let Err(e) = crate::fs::mount(&folder, &format!("{folder}.vpk")) {
        crate::trace::note(&format!("world: {map}: {e}"));
        return None;
    }
    let ptr = crate::handles::object();
    worlds().insert(ptr, World { folder: CString::new(folder).ok()?, scene_world, lumps: HashMap::new() });
    Some(ptr)
}

pub fn release(ptr: usize) {
    worlds().remove(&ptr);
}

pub fn with<R>(ptr: usize, f: impl FnOnce(&World) -> R) -> Option<R> {
    worlds().get(&ptr).map(f)
}

/// A lump's entities, read from the world's pak the first time.
pub fn lump(ptr: usize, name: &str) -> Vec<&'static Entity> {
    let mut w = worlds();
    let Some(world) = w.get_mut(&ptr) else { return Vec::new() };
    if let Some(l) = world.lumps.get(name) { return l.clone(); }
    let path = format!("{}/entities/{name}.vents_c", world.folder.to_string_lossy());
    let read: Vec<&'static Entity> = crate::fs::read(&path)
        .and_then(|b| crate::assets::resource::Resource::parse(b).ok())
        .and_then(|r| r.kv3("DATA")?.ok())
        .map(|d| crate::assets::entities::read_lump(&d))
        .unwrap_or_default()
        .into_iter()
        .map(|e| &*Box::leak(Box::new(e)))
        .collect();
    entities().extend(read.iter().map(|e| *e as *const Entity as usize));
    world.lumps.insert(name.to_string(), read.clone());
    read
}

/// The entity at `ptr`, if it is one handed out.
pub fn entity(ptr: usize) -> Option<&'static Entity> {
    entities().contains(&ptr).then(|| unsafe { &*(ptr as *const Entity) })
}

/// What a world draws (its nodes' scene objects, from `world.vwrld_c` and
/// each node's `.vwnod_c`): each a model in the pak, where it is (a 3x4
/// transform, row by row) and its tint. Read when first asked for.
pub fn scene_objects(ptr: usize) -> Vec<(String, [[f32; 4]; 3], [f32; 4])> {
    let Some(folder) = with(ptr, |w| w.folder.to_string_lossy().into_owned()) else { return Vec::new() };
    static READ: OnceLock<Mutex<HashMap<String, Vec<(String, [[f32; 4]; 3], [f32; 4])>>>> = OnceLock::new();
    let read = READ.get_or_init(Default::default);
    if let Some(o) = read.lock().unwrap().get(&folder) { return o.clone(); }
    let kv3 = |path: &str| crate::fs::read(path).and_then(|b| crate::assets::resource::Resource::parse(b).ok()).and_then(|r| r.kv3("DATA")?.ok());
    use crate::assets::kv3::Value;
    let mut out = Vec::new();
    if let Some(world) = kv3(&format!("{folder}/world.vwrld_c")) {
        for node in world.get("m_worldNodes").map(Value::as_array).unwrap_or(&[]) {
            let Some(prefix) = node.get("m_worldNodePrefix").and_then(Value::as_str) else { continue };
            let Some(n) = kv3(&format!("{}.vwnod_c", prefix.replace('\\', "/"))) else { continue };
            for o in n.get("m_sceneObjects").map(Value::as_array).unwrap_or(&[]) {
                let Some(model) = o.get("m_renderableModel").and_then(Value::as_str).filter(|m| !m.is_empty()) else { continue };
                let rows = o.get("m_vTransform").map(Value::as_array).unwrap_or(&[]);
                let row = |i: usize| { let f = rows.get(i).map(Value::floats).unwrap_or_default(); core::array::from_fn(|k| f.get(k).copied().unwrap_or(if k == i { 1.0 } else { 0.0 })) };
                let tint = o.get("m_vTintColor").map(Value::floats).unwrap_or_default();
                let tint = if tint.iter().take(3).all(|&t| t == 0.0) { [1.0; 4] } else { core::array::from_fn(|k| tint.get(k).copied().unwrap_or(1.0)) };
                out.push((model.to_string(), [row(0), row(1), row(2)], tint));
            }
        }
    }
    read.lock().unwrap().insert(folder, out.clone());
    out
}

/// The worlds made in a scene world.
pub fn in_scene_world(scene_world: usize) -> Vec<usize> {
    worlds().iter().filter(|(_, w)| w.scene_world == scene_world).map(|(&p, _)| p).collect()
}

#[cfg(test)]
mod tests {
    use crate::handles::murmur2;

    #[test]
    fn a_downloaded_map_has_its_entities_and_collision() {
        // a map from sbox.game (Facepunch's, read where it lies), if this machine has one
        let Ok(path) = std::env::var("ENGINE2_SAMPLE_MAP") else { return };
        let name = std::path::Path::new(&path).file_stem().unwrap().to_string_lossy().to_string();
        crate::fs::add_link(&format!("maps/{name}.vpk"), &path);
        let w = super::create(&format!("maps/{name}.vpk"), 0).expect("mounted");
        assert_eq!(super::with(w, |x| x.folder.to_string_lossy().into_owned()).unwrap(), format!("maps/{name}"));
        let ents = super::lump(w, "default_ents");
        assert!(!ents.is_empty());
        assert_eq!(ents[0].get(murmur2("classname")), Some("worldspawn"));
        let classes: Vec<&str> = ents.iter().filter_map(|e| e.get(murmur2("classname"))).collect();
        assert!(classes.iter().all(|c| !c.is_empty()), "every entity has a class");
        eprintln!("{} entities: {:?}", ents.len(), { let mut c = classes.clone(); c.sort(); c.dedup(); c });
        assert!(super::entity(ents[0] as *const _ as usize).is_some());
        assert!(super::entity(0x1234).is_none(), "only what was handed out");

        let physics = crate::models::load_physics(&format!("maps/{name}/world_physics.vphys"));
        assert_ne!(physics, 0);
        let mut m = crate::models::models();
        let b = m.bodies(physics).unwrap().clone();
        let hulls: usize = b.parts.iter().map(|&p| m.part(p).unwrap().hulls.len()).sum();
        let meshes: usize = b.parts.iter().map(|&p| m.part(p).unwrap().meshes.len()).sum();
        eprintln!("collision: {} parts, {hulls} hulls, {meshes} meshes, {} surfaces, attributes {:?}", b.parts.len(), b.surface_hashes.len(), b.attributes);
        assert!(hulls + meshes > 0);
        assert!(b.attributes.iter().flatten().any(|t| t.to_str() == Ok("solid")));
    }
}
