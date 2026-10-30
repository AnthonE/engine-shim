//! A map's world (`maps/<name>/world.vwrld_c`): its nodes (`.vwnod_c`),
//! each listing scene objects, the level's geometry batched by material:
//! a model in the pak, where it is and its tint.
use super::kv3::Value;
use super::resource::Resource;

/// A world node's object: its model, its transform (3x4, row by row), its tint.
pub type Placed = (String, [[f32; 4]; 3], [f32; 4]);

/// The scene objects of the world in `folder` (`maps/flatgrass`), read
/// through `read` (game paths).
pub fn scene_objects(folder: &str, read: impl Fn(&str) -> Option<Vec<u8>>) -> Vec<Placed> {
    let kv3 = |path: &str| read(path).and_then(|b| Resource::parse(b).ok()).and_then(|r| r.kv3("DATA")?.ok());
    let mut out = Vec::new();
    let Some(world) = kv3(&format!("{folder}/world.vwrld_c")) else { return out };
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
    out
}
