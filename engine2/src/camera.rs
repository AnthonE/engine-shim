//! Cameras managed asks for pixels (CCameraRenderer): the scene worlds each
//! is given, which scene objects are shown, and, when managed asks for a
//! camera's view as a bitmap (CameraComponent.RenderToBitmap), the scene
//! objects in its worlds drawn from where managed put it (render/).
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

static WORLDS: Mutex<Option<HashMap<usize, Vec<usize>>>> = Mutex::new(None);

/// Materials by name, each one object (and its name to give back).
static MATERIALS: Mutex<Option<(HashMap<String, usize>, HashMap<usize, std::ffi::CString>)>> = Mutex::new(None);
/// Each sky's material, once managed changes it; each directional
/// light's direction (towards it) as made or set.
static SKIES: Mutex<Option<HashMap<usize, usize>>> = Mutex::new(None);
static SUNS: Mutex<Option<HashMap<usize, crate::slots::S_Vector3>>> = Mutex::new(None);
/// Each light's bake index and scale (managed's map loader gives a map's
/// lights the ones its lightmaps name them by).
static BAKED: Mutex<Option<HashMap<usize, (i32, f32)>>> = Mutex::new(None);

/// What each scene object shows of its model: the mesh groups (its body
/// groups' choices) once managed set them, and its material group (skin).
static MESH_GROUPS: Mutex<Option<HashMap<usize, u64>>> = Mutex::new(None);
static SKINS: Mutex<Option<HashMap<usize, usize>>> = Mutex::new(None);

/// Each camera's clear colour, as managed configured it
/// (CameraRenderer.Configure: `clearColor`, the camera's BackgroundColor, or
/// transparent where it doesn't clear colour), RGBA as authored.
static CLEAR: Mutex<Option<HashMap<usize, [f32; 4]>>> = Mutex::new(None);

/// A camera's render attributes (SetRenderAttributes): what a frame reads
/// of them is kept, as the attributes are managed's to free.
pub fn set_view_attributes(camera: usize, attrs: usize) {
    if let Some(crate::gpu::Val::Vec4(c)) = crate::gpu::get(attrs, crate::string_token("clearColor")) {
        CLEAR.lock().unwrap().get_or_insert_with(Default::default).insert(camera, c);
    }
}

/// A camera's clear colour, if managed gave it one.
pub fn clear_color(camera: usize) -> Option<[f32; 4]> {
    CLEAR.lock().unwrap().as_ref()?.get(&camera).copied()
}

/// An object's mesh groups: as set, else its model's default.
pub fn mesh_groups(obj: usize) -> u64 {
    if let Some(&m) = MESH_GROUPS.lock().unwrap().as_ref().and_then(|g| g.get(&obj)) { return m; }
    crate::models::file(crate::animation::model_of(obj)).map(|f| f.default_mask).unwrap_or(u64::MAX)
}

/// An object's mesh groups if managed chose them (none: its model's default).
pub fn chosen_mesh_groups(obj: usize) -> Option<u64> {
    MESH_GROUPS.lock().unwrap().as_ref().and_then(|g| g.get(&obj).copied())
}

pub fn set_mesh_groups(obj: usize, mask: u64) {
    MESH_GROUPS.lock().unwrap().get_or_insert_with(Default::default).insert(obj, mask);
}

/// A body group of the object's model (by its name's token) set to a
/// choice: that choice's mesh groups in place of the others'.
pub fn set_body_group(obj: usize, token: u32, choice: i32) {
    let Some(f) = crate::models::file(crate::animation::model_of(obj)) else { return };
    let Some(part) = f.body_parts.iter().find(|p| crate::handles::murmur2(&p.name.to_string_lossy()) == token) else { return };
    let Some(&(_, chosen)) = part.choices.get(choice.max(0) as usize) else { return };
    let all = part.choices.iter().fold(0, |a, c| a | c.1);
    set_mesh_groups(obj, (mesh_groups(obj) & !all) | chosen);
}

/// An object's material group (skin): its index among its model's.
pub fn skin(obj: usize) -> usize {
    SKINS.lock().unwrap().as_ref().and_then(|s| s.get(&obj).copied()).unwrap_or(0)
}

/// A skin set by its name's token (one the model doesn't have: its first).
pub fn set_skin(obj: usize, token: u32) {
    let groups = crate::models::file(crate::animation::model_of(obj)).map(|f| &f.material_groups[..]).unwrap_or(&[]);
    let i = groups.iter().position(|g| crate::handles::murmur2(&g.to_string_lossy()) == token).unwrap_or(0);
    SKINS.lock().unwrap().get_or_insert_with(Default::default).insert(obj, i);
}

pub fn set_bake_index(light: usize, index: i32) {
    BAKED.lock().unwrap().get_or_insert_with(Default::default).entry(light).or_insert((-1, 1.0)).0 = index;
}

pub fn set_bake_scale(light: usize, scale: f32) {
    BAKED.lock().unwrap().get_or_insert_with(Default::default).entry(light).or_insert((-1, 1.0)).1 = scale;
}

/// A light's bake index and scale, if it was given one.
pub fn baked(light: usize) -> Option<(i32, f32)> {
    BAKED.lock().unwrap().as_ref()?.get(&light).copied().filter(|b| b.0 >= 0)
}

pub fn material(name: &str) -> usize {
    let key = crate::fs::normal(name);
    let mut m = MATERIALS.lock().unwrap();
    let (by_name, names) = m.get_or_insert_with(Default::default);
    *by_name.entry(key).or_insert_with(|| {
        let p = crate::handles::object();
        names.insert(p, std::ffi::CString::new(name.replace('\0', "")).unwrap_or_default());
        p
    })
}

/// A copy of a material (CreateProceduralMaterialCopy: managed's
/// Material.CreateCopy): an object of its own with its source's name and
/// shader, and its source's texture parameters, which it can then change.
pub fn copy_material(src: usize) -> usize {
    let p = crate::handles::object();
    {
        let mut m = MATERIALS.lock().unwrap();
        let (_, names) = m.get_or_insert_with(Default::default);
        let name = names.get(&src).cloned().unwrap_or_default();
        names.insert(p, name);
    }
    if let Some(shader) = material_shader(src) { set_material_shader(p, &shader); }
    let params = MATERIAL_TEXTURES.lock().unwrap().as_ref().and_then(|t| t.get(&src).cloned());
    if let Some(params) = params { MATERIAL_TEXTURES.lock().unwrap().get_or_insert_with(Default::default).insert(p, params); }
    p
}

/// Textures managed set on a material by parameter name (Material.Set:
/// a line's `g_tColor`).
static MATERIAL_TEXTURES: Mutex<Option<HashMap<usize, HashMap<String, usize>>>> = Mutex::new(None);

pub fn set_material_texture(m: usize, name: &str, t: usize) {
    MATERIAL_TEXTURES.lock().unwrap().get_or_insert_with(Default::default).entry(m).or_default().insert(name.to_string(), t);
}

pub fn material_texture(m: usize, name: &str) -> Option<usize> {
    MATERIAL_TEXTURES.lock().unwrap().as_ref()?.get(&m)?.get(name).copied()
}

/// Values managed set on a material by parameter name (Material.Set with a
/// number, a vector or a colour: Grubs' charge gauge sets how far it's
/// filled), and how many times any has been set. A shader graph's material
/// reads them over its own (render: `set_graph_values`), by the material's
/// name: a copy of a material shares its name, so shares them.
static MATERIAL_VALUES: Mutex<Option<HashMap<usize, HashMap<String, [f32; 4]>>>> = Mutex::new(None);
static MATERIAL_VALUES_SET: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

pub fn set_material_value(m: usize, name: &str, v: [f32; 4]) {
    MATERIAL_VALUES.lock().unwrap().get_or_insert_with(Default::default).entry(m).or_default().insert(name.to_ascii_lowercase(), v);
    MATERIAL_VALUES_SET.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

pub fn material_value(m: usize, name: &str) -> Option<[f32; 4]> {
    MATERIAL_VALUES.lock().unwrap().as_ref()?.get(&m)?.get(&name.to_ascii_lowercase()).copied()
}

/// Every value set on a material, by the material's name (as files name
/// it) and the parameter's in lower case; and how many sets there have been.
pub fn material_values() -> (u64, HashMap<String, HashMap<String, [f32; 4]>>) {
    let set = MATERIAL_VALUES_SET.load(std::sync::atomic::Ordering::Relaxed);
    let values = MATERIAL_VALUES.lock().unwrap().clone().unwrap_or_default();
    let mut out: HashMap<String, HashMap<String, [f32; 4]>> = HashMap::new();
    for (m, v) in values {
        if let Some(name) = material_name(m) { out.entry(crate::fs::normal(&name)).or_default().extend(v); }
    }
    (set, out)
}

/// Textures by name, each one object (as materials are).
static TEXTURES: Mutex<Option<(HashMap<String, usize>, HashMap<usize, String>)>> = Mutex::new(None);

pub fn texture(name: &str) -> usize {
    let mut t = TEXTURES.lock().unwrap();
    let (by_name, names) = t.get_or_insert_with(Default::default);
    *by_name.entry(crate::fs::normal(name)).or_insert_with(|| {
        let p = crate::handles::object();
        names.insert(p, name.to_string());
        p
    })
}

pub fn texture_name(t: usize) -> Option<String> {
    TEXTURES.lock().unwrap().as_ref()?.1.get(&t).cloned()
}

/// Each light probe volume's textures: its probes, their lights' indices
/// and strengths.
static PROBE_TEXTURES: Mutex<Option<HashMap<usize, [usize; 3]>>> = Mutex::new(None);

pub fn set_probe_texture(volume: usize, which: usize, t: usize) {
    PROBE_TEXTURES.lock().unwrap().get_or_insert_with(Default::default).entry(volume).or_default()[which] = t;
}

pub fn probe_texture(volume: usize, which: usize) -> usize {
    PROBE_TEXTURES.lock().unwrap().as_ref().and_then(|p| p.get(&volume)).map(|t| t[which]).unwrap_or(0)
}

pub fn material_name(m: usize) -> Option<String> {
    MATERIALS.lock().unwrap().as_ref()?.1.get(&m).map(|n| n.to_string_lossy().into_owned())
}

/// Materials made from a shader (CreateRawMaterial: managed's
/// Material.FromShader, a compute shader's): the shader each was made with.
static RAW_SHADERS: Mutex<Option<HashMap<usize, String>>> = Mutex::new(None);

pub fn set_material_shader(m: usize, shader: &str) {
    RAW_SHADERS.lock().unwrap().get_or_insert_with(Default::default).insert(m, shader.to_string());
}

/// Shaders by name, each one object (CVfx), and its name to give back.
static SHADERS_BY_NAME: Mutex<Option<(HashMap<String, usize>, HashMap<usize, std::ffi::CString>)>> = Mutex::new(None);

pub fn shader(name: &str) -> usize {
    let key = crate::fs::normal(name);
    let mut m = SHADERS_BY_NAME.lock().unwrap();
    let (by_name, names) = m.get_or_insert_with(Default::default);
    *by_name.entry(key).or_insert_with(|| {
        let p = crate::handles::object();
        names.insert(p, std::ffi::CString::new(name.replace('\0', "")).unwrap_or_default());
        p
    })
}

pub fn shader_name_ptr(v: usize) -> usize {
    static EMPTY: &std::ffi::CStr = c"";
    SHADERS_BY_NAME.lock().unwrap().as_ref().and_then(|x| x.1.get(&v).map(|n| n.as_ptr() as usize)).unwrap_or(EMPTY.as_ptr() as usize)
}

/// A material's shader: the one it was made from, else its file's (read once).
pub fn material_shader(m: usize) -> Option<String> {
    if let Some(s) = RAW_SHADERS.lock().unwrap().as_ref().and_then(|r| r.get(&m)) { return Some(s.clone()); }
    static SHADERS: Mutex<Option<HashMap<usize, Option<String>>>> = Mutex::new(None);
    if let Some(s) = SHADERS.lock().unwrap().as_ref().and_then(|s| s.get(&m)) { return s.clone(); }
    let shader = material_name(m)
        .and_then(|n| crate::fs::read_compiled(&n))
        .and_then(|b| crate::assets::resource::Resource::parse(b).ok())
        .and_then(|r| crate::assets::material::read(&r).ok())
        .map(|x| x.shader);
    SHADERS.lock().unwrap().get_or_insert_with(Default::default).insert(m, shader.clone());
    shader
}

pub fn material_name_ptr(m: usize) -> usize {
    static EMPTY: &std::ffi::CStr = c"";
    MATERIALS.lock().unwrap().as_ref().and_then(|x| x.1.get(&m).map(|n| n.as_ptr() as usize)).unwrap_or(EMPTY.as_ptr() as usize)
}

pub fn set_sky_material(sky: usize, m: usize) {
    SKIES.lock().unwrap().get_or_insert_with(Default::default).insert(sky, m);
}

/// A sky object's material: the one it was set, else the one it was made with.
pub fn sky_material(sky: usize) -> usize {
    SKIES.lock().unwrap().as_ref().and_then(|s| s.get(&sky).copied()).or_else(|| crate::handles::links(sky).first().copied()).unwrap_or(0)
}

/// A directional light made (CreateDirectionalLight), towards it.
pub fn add_sun(light: usize, towards: crate::slots::S_Vector3) {
    SUNS.lock().unwrap().get_or_insert_with(Default::default).insert(light, towards);
}

/// A light turned: a directional one's new direction (every kind of light
/// is told).
pub fn turn_light(light: usize, towards: crate::slots::S_Vector3) {
    if let Some(d) = SUNS.lock().unwrap().as_mut().and_then(|s| s.get_mut(&light)) { *d = towards; }
}

/// A scene object gone: whatever was kept of it here.
pub fn forget(obj: usize) {
    set_shown(obj, true);
    if let Some(t) = TAGS.lock().unwrap().as_mut() { t.remove(&obj); }
    if let Some(s) = SUNS.lock().unwrap().as_mut() { s.remove(&obj); }
    if let Some(s) = SKIES.lock().unwrap().as_mut() { s.remove(&obj); }
    if let Some(s) = BAKED.lock().unwrap().as_mut() { s.remove(&obj); }
    if let Some(s) = PROBE_TEXTURES.lock().unwrap().as_mut() { s.remove(&obj); }
}

/// The directional lights, with the direction each was made or set with.
pub fn suns() -> Vec<(usize, crate::slots::S_Vector3)> {
    SUNS.lock().unwrap().as_ref().map(|s| s.iter().map(|(&k, &v)| (k, v)).collect()).unwrap_or_default()
}
static HIDDEN: Mutex<Option<HashSet<usize>>> = Mutex::new(None);

pub fn add_world(camera: usize, world: usize) {
    let mut w = WORLDS.lock().unwrap();
    let list = w.get_or_insert_with(Default::default).entry(camera).or_default();
    if !list.contains(&world) { list.push(world); }
}

pub fn clear_worlds(camera: usize) {
    if let Some(w) = WORLDS.lock().unwrap().as_mut() { w.remove(&camera); }
}

pub fn worlds(camera: usize) -> Vec<usize> {
    WORLDS.lock().unwrap().as_ref().and_then(|w| w.get(&camera).cloned()).unwrap_or_default()
}

/// A scene object shown or not (SetRenderingEnabled): shown unless told.
pub fn set_shown(obj: usize, shown: bool) {
    let mut h = HIDDEN.lock().unwrap();
    let h = h.get_or_insert_with(Default::default);
    if shown { h.remove(&obj); } else { h.insert(obj); }
}

pub fn shown(obj: usize) -> bool {
    !HIDDEN.lock().unwrap().as_ref().is_some_and(|h| h.contains(&obj))
}

/// Each scene object's tags (StringToken values, in the order given), and
/// each camera's render and exclude tags.
static TAGS: Mutex<Option<HashMap<usize, Vec<u32>>>> = Mutex::new(None);
static CAMERA_TAGS: Mutex<Option<HashMap<usize, (Vec<u32>, Vec<u32>)>>> = Mutex::new(None);

pub fn with_tags<T>(obj: usize, f: impl FnOnce(&mut Vec<u32>) -> T) -> T {
    let mut t = TAGS.lock().unwrap();
    let t = t.get_or_insert_with(Default::default);
    let tags = t.entry(obj).or_default();
    let out = f(tags);
    if tags.is_empty() { t.remove(&obj); }
    out
}

pub fn add_tag(obj: usize, tag: u32) { with_tags(obj, |t| if !t.contains(&tag) { t.push(tag) }) }
pub fn remove_tag(obj: usize, tag: u32) { with_tags(obj, |t| t.retain(|&x| x != tag)) }
pub fn tags(obj: usize) -> Vec<u32> { with_tags(obj, |t| t.clone()) }

/// A camera's tags changed: `exclude` picks which set; None clears it.
pub fn camera_tag(camera: usize, exclude: bool, tag: Option<u32>) {
    let mut c = CAMERA_TAGS.lock().unwrap();
    let (render, excluded) = c.get_or_insert_with(Default::default).entry(camera).or_default();
    let set = if exclude { excluded } else { render };
    match tag { Some(t) => if !set.contains(&t) { set.push(t) }, None => set.clear() }
}

/// Tags as names where they're ones a game's camera commonly sets, for what
/// a frame dump says (a token is a hash: the rest are said as numbers).
pub fn tag_names(tags: &[u32]) -> String {
    const KNOWN: [&str; 10] = ["firstperson", "viewmodel", "viewer", "ui", "player", "light", "skybox", "world", "debris", "hide_in_game"];
    tags.iter().map(|&t| KNOWN.iter().find(|n| crate::string_token(n) == t).map(|n| n.to_string()).unwrap_or_else(|| format!("{t:#x}"))).collect::<Vec<_>>().join(",")
}

/// A camera's render and exclude tags, said (a frame dump).
pub fn camera_tag_names(camera: usize) -> String {
    let c = CAMERA_TAGS.lock().unwrap();
    let (r, e) = c.as_ref().and_then(|c| c.get(&camera)).cloned().unwrap_or_default();
    format!("render [{}] exclude [{}]", tag_names(&r), tag_names(&e))
}

/// Whether a camera draws an object by their tags, as native's views
/// filter them: one with any excluded tag isn't, and where the camera
/// names tags to draw, only one with at least one of them is.
pub fn tags_drawn(camera: usize, obj: usize) -> bool {
    let c = CAMERA_TAGS.lock().unwrap();
    let Some((render, excluded)) = c.as_ref().and_then(|c| c.get(&camera)) else { return true };
    if render.is_empty() && excluded.is_empty() { return true; }
    let t = TAGS.lock().unwrap();
    let tags = t.as_ref().and_then(|t| t.get(&obj)).map(|v| v.as_slice()).unwrap_or(&[]);
    !tags.iter().any(|x| excluded.contains(x)) && (render.is_empty() || tags.iter().any(|x| render.contains(x)))
}

#[cfg(feature = "render")]
mod draw {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, OnceLock};

    use glam::{Mat4, Quat, Vec3};

    use crate::render::{BakedLight, EngineFiles, Instance, Lightmaps, Model, ProbePlace, ProbeVolume, Renderer, View};
    use crate::slots::*;

    /// The renderer, made the first time a camera draws (None: no adapter).
    fn renderer() -> Option<&'static Renderer> {
        static R: OnceLock<Option<Renderer>> = OnceLock::new();
        R.get_or_init(|| Renderer::new().map_err(|e| crate::trace::note(&format!("render: {e}"))).ok()).as_ref()
    }

    /// Each model on the GPU, by name, the first time it's seen.
    fn gpu_model(r: &Renderer, name: &str) -> Option<Arc<Model>> {
        static M: OnceLock<Mutex<HashMap<String, Option<Arc<Model>>>>> = OnceLock::new();
        let m = M.get_or_init(Default::default);
        let key = crate::fs::normal(name);
        if let Some(g) = m.lock().unwrap().get(&key) { return g.clone(); }
        // a file's, or (the default player's, with no file) the stand-in's meshes
        let drawn = r.model(&EngineFiles, name).or_else(|e| match crate::standin::draw_meshes(name) {
            Some((meshes, default, skins)) => r.model_of(&EngineFiles, name, &meshes, default, &skins),
            None => Err(e),
        });
        let g = drawn.map_err(|e| crate::trace::note(&format!("render: {e}"))).ok().map(Arc::new);
        if let Some(g) = &g { for n in &g.notes { crate::trace::note(&format!("render: {n}")); } }
        m.lock().unwrap().insert(key, g.clone());
        g
    }

    /// A model built at runtime on the GPU (its meshes: rendermesh.rs), made
    /// again whenever managed has written to their buffers since.
    fn gpu_runtime_model(r: &Renderer, model: usize, meshes: &[usize]) -> Option<Arc<Model>> {
        static M: OnceLock<Mutex<HashMap<usize, (u64, Option<Arc<Model>>)>>> = OnceLock::new();
        let m = M.get_or_init(Default::default);
        let version = crate::rendermesh::version(meshes);
        if let Some((v, g)) = m.lock().unwrap().get(&model) { if *v == version { return g.clone(); } }
        let file = crate::models::file(model);
        let masks = file.map(|f| f.mesh_group_masks.clone()).unwrap_or_default();
        let drawn = crate::rendermesh::for_drawing(meshes, &masks, |mat| super::material_name(mat).unwrap_or_default());
        let default = file.map(|f| f.default_mask).unwrap_or(u64::MAX);
        let g = r.model_of(&EngineFiles, &format!("a model built at runtime ({model:#x})"), &drawn, default, &[]).map_err(|e| crate::trace::note(&format!("render: {e}"))).ok().map(Arc::new);
        if let Some(g) = &g { for n in &g.notes { crate::trace::note(&format!("render: {n}")); } }
        m.lock().unwrap().insert(model, (version, g.clone()));
        g
    }

    /// Each object's graph materials as its own attributes make them, made
    /// again only when what it was given (or a texture among it) changes.
    fn object_materials(r: &Renderer, placed: &[(Arc<Model>, Mat4, [f32; 4], Vec<Mat4>, bool, Option<u64>, usize)], with: &[(usize, usize)]) -> HashMap<usize, Arc<HashMap<String, wgpu::BindGroup>>> {
        use crate::gpu::Val;
        use std::hash::{Hash, Hasher};
        type Kept = HashMap<usize, (u64, Option<Arc<HashMap<String, wgpu::BindGroup>>>)>;
        static KEPT: OnceLock<Mutex<Kept>> = OnceLock::new();
        let mut kept = KEPT.get_or_init(Default::default).lock().unwrap();
        let mut out = HashMap::new();
        for &(k, obj) in with {
            let Some(values) = crate::gpu::object_values(obj) else { continue };
            let mut numbers: HashMap<u32, [f32; 4]> = HashMap::new();
            let mut textures: HashMap<u32, usize> = HashMap::new();
            for (key, v) in &values {
                match v {
                    Val::Float(f) => { numbers.insert(*key, [*f, 0.0, 0.0, 0.0]); }
                    Val::Vec2(v) => { numbers.insert(*key, [v[0], v[1], 0.0, 0.0]); }
                    Val::Vec3(v) => { numbers.insert(*key, [v[0], v[1], v[2], 0.0]); }
                    Val::Vec4(v) => { numbers.insert(*key, *v); }
                    Val::Int(i) => { numbers.insert(*key, [*i as f32, 0.0, 0.0, 0.0]); }
                    Val::Bool(b) => { numbers.insert(*key, [*b as i32 as f32, 0.0, 0.0, 0.0]); }
                    Val::Texture(t) => { textures.insert(*key, *t); }
                    _ => {}
                }
            }
            // what it was given, and each texture as it is now, as one number
            let mut h = std::collections::hash_map::DefaultHasher::new();
            (Arc::as_ptr(&placed[k].0) as usize).hash(&mut h);
            let mut keys: Vec<_> = numbers.iter().map(|(k, v)| (*k, v.map(f32::to_bits))).collect();
            keys.sort();
            keys.hash(&mut h);
            let mut texs: Vec<_> = textures.iter().map(|(k, t)| (*k, *t, crate::gpu::texture_version(*t))).collect();
            texs.sort();
            texs.hash(&mut h);
            let key = h.finish();
            let groups = match kept.get(&obj) {
                Some((was, groups)) if *was == key => groups.clone(),
                _ => {
                    let groups = r.object_materials(&placed[k].0, &numbers, &textures);
                    kept.insert(obj, (key, groups.clone()));
                    groups
                }
            };
            if let Some(g) = groups { out.insert(k, g); }
        }
        out
    }

    fn mat(x: &crate::animation::Xf) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::new(x.scale.x, x.scale.y, x.scale.z),
            Quat::from_xyzw(x.rot.x, x.rot.y, x.rot.z, x.rot.w),
            Vec3::new(x.pos.x, x.pos.y, x.pos.z),
        )
    }

    /// A model scene object's bones as it's posed now (animation.rs, in the
    /// model's space), each over its bind pose's inverse: none if it has no
    /// skeleton.
    fn skin(obj: usize, model: usize) -> Vec<Mat4> {
        let Some(f) = crate::models::file(model) else { return Vec::new() };
        if f.bones.is_empty() { return Vec::new(); }
        let pose = crate::animation::with(obj, |p| p.model_space.clone()).unwrap_or_default();
        if pose.len() != f.bones.len() { return Vec::new(); }
        pose.iter().zip(&f.bones).map(|(p, b)| mat(p) * mat(&crate::animation::Xf::of(b.model)).inverse()).collect()
    }

    fn v3(v: S_Vector3) -> Vec3 { Vec3::new(v.f__vec.f_X, v.f__vec.f_Y, v.f__vec.f_Z) }

    /// A map's baked lighting on the GPU, lit by these lights, read once
    /// for each set of them.
    fn lightmaps(r: &Renderer, folder: &str, lights: &[BakedLight]) -> Option<Arc<Lightmaps>> {
        static L: OnceLock<Mutex<HashMap<String, Option<Arc<Lightmaps>>>>> = OnceLock::new();
        let key = format!("{folder} {lights:?}");
        let l = L.get_or_init(Default::default);
        if let Some(g) = l.lock().unwrap().get(&key) { return g.clone(); }
        let g = r.lightmaps(&EngineFiles, folder, lights).map_err(|e| crate::trace::note(&format!("render: {e}"))).ok().flatten().map(Arc::new);
        let mut l = l.lock().unwrap();
        // a light moved or recoloured: the lightmaps lit by the old ones go
        l.retain(|k, _| !k.starts_with(&format!("{folder} ")));
        l.insert(key, g.clone());
        g
    }

    /// The light probe volumes managed made in these worlds (SceneLightProbe:
    /// a map's), lit by these lights: each read once for each set of them.
    pub fn probe_volumes(r: &Renderer, worlds: &[usize], lights: &[BakedLight]) -> Vec<Arc<ProbeVolume>> {
        static V: OnceLock<Mutex<HashMap<String, Option<Arc<ProbeVolume>>>>> = OnceLock::new();
        let cache = V.get_or_init(Default::default);
        let mut out = Vec::new();
        for world in worlds {
            for v in crate::handles::made_with("SceneLightProbeVolumeObject", *world) {
                let names = [0, 1, 2].map(|i| super::texture_name(super::probe_texture(v, i)));
                let [Some(t), Some(i), Some(s)] = names else { continue };
                let place = ProbePlace {
                    transform: mat(&crate::animation::Xf::from_managed(&crate::state::get::<S_Transform>(v, PROP_Transform))),
                    mins: v3(crate::state::get::<S_Vector3>(v, PROP__CScnLghtPrbVlmbj_m_vBoxMins)),
                    maxs: v3(crate::state::get::<S_Vector3>(v, PROP__CScnLghtPrbVlmbj_m_vBoxMaxs)),
                    priority: crate::state::get::<i32>(v, PROP__CScnLghtPrbVlmbj_m_nRenderPriority),
                };
                let key = format!("{v} {t} {place:?} {lights:?}");
                let built = cache.lock().unwrap().get(&key).cloned();
                let built = built.unwrap_or_else(|| {
                    let b = r.probe_volume(&EngineFiles, &t, &i, &s, place, lights).map_err(|e| crate::trace::note(&format!("render: {e}"))).ok().map(Arc::new);
                    let mut c = cache.lock().unwrap();
                    c.retain(|k, _| !k.starts_with(&format!("{v} ")));
                    c.insert(key, b.clone());
                    b
                });
                out.extend(built);
            }
        }
        out
    }

    /// Camera `camera`'s view, `width` by `height`, into managed's pixels.
    pub fn to_bitmap(camera: usize, pixels: usize, width: i32, height: i32, bpp: i32) {
        if pixels == 0 || width <= 1 || height <= 1 || bpp != 4 { return; }
        let Some((image, said)) = frame(camera, width, height) else { return };
        let out = unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, (width * height * 4) as usize) };
        out.copy_from_slice(&image);
        // ENGINE2_FRAME_DUMP=<folder>: the first frames as raw RGBA, for
        // `convert -size WxH -depth 8 rgba:x.rgba x.png`
        if let Some(dir) = std::env::var_os("ENGINE2_FRAME_DUMP") {
            static DUMPED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            let n = DUMPED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            if n < 32 {
                let path = std::path::Path::new(&dir).join(format!("frame-{n:02}-{width}x{height}.rgba"));
                let _ = std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(&path, &image));
            }
        }
        crate::trace::note(&said);
    }

    /// Camera `camera`'s view drawn into a texture managed made (a render
    /// target: a ScenePanel's, a camera's `RenderTarget`), at the texture's
    /// size. Drawn and read back as `to_bitmap` is, then written into the
    /// texture in its own format, which bumps its version so the UI and the
    /// renderer take the new pixels. False where the texture isn't one
    /// managed made or its format isn't one written here.
    pub fn to_texture(camera: usize, texture: usize) -> bool {
        let Some((width, height)) = crate::gpu::texture_size(texture) else { return false };
        let Some((image, said)) = frame(camera, width, height) else { return false };
        crate::trace::note(&said);
        crate::gpu::write_rgba(texture, &image)
    }

    /// The renderer the cameras draw with (a window's surface shares its device).
    pub fn shared_renderer() -> Option<&'static Renderer> { renderer() }

    /// Camera `camera`'s view, `width` by `height`, as RGBA (top row first),
    /// and a line saying what was in it.
    /// The camera's custom scene objects (managed's SceneCustomObject: a
    /// sprite batch, say) asked to render, as native asks in its translucent
    /// layer (SceneCustomObjectRender.RenderObject), each with a render
    /// context that records what it draws; what they drew as sprite
    /// batches and lines, and what's said of it.
    fn custom_objects(camera: usize, view: &View, width: i32, height: i32) -> (Vec<crate::sprites::Batch>, Vec<crate::lines::Line>, String) {
        let f = crate::managed(M_Sandbox_ScnCstmbjctRndr_RenderObject);
        let objects: Vec<usize> = super::worlds(camera).into_iter()
            .flat_map(|w| crate::handles::made_with("ManagedSceneObject", w))
            .filter(|&o| super::shown(o) && super::tags_drawn(camera, o))
            .collect();
        if f == 0 || objects.is_empty() { return (Vec::new(), Vec::new(), String::new()); }
        #[repr(C)]
        struct Setup { context: usize, view: usize, layer: usize, format: i32, msaa: i32, stats: usize }
        // managed finds the object by its handle (HandleIndex.Get<SceneCustomObject>)
        let render: extern "C" fn(Setup, i32) = unsafe { std::mem::transmute(f) };
        let angles = crate::state::get::<S_Angles>(camera, PROP__CCameraRenderer_CameraRotation);
        let row = |k: usize| view.view.row(k).truncate().to_array();
        let back = row(2);
        crate::sprites::set_eye(Some(crate::sprites::Eye { position: view.eye.to_array(), forward: back.map(|v| -v), right: row(0), up: row(1), angles: [angles.f_pitch, angles.f_yaw, angles.f_roll] }));
        let scene_view = crate::gpu::new_view(crate::impls::camera_id(camera), width, height);
        let layer = crate::gpu::add_layer(scene_view, "Translucent", 0);
        // SceneLayerType.Translucent: what a sprite batch with refraction asks
        crate::state::set::<i64>(layer, PROP__ISceneLayer_LayerEnum, 1);
        let mut draws = Vec::new();
        for obj in &objects {
            let context = crate::gpu::new_layer_context(layer);
            crate::gpu::with_context(context, |c| c.viewport = [0, 0, width, height]);
            render(Setup { context, view: scene_view, layer, format: 0, msaa: 0, stats: 0 }, crate::handles::of(*obj));
            if let Some(c) = crate::gpu::take_context(context) { draws.extend(c.draws); }
        }
        crate::gpu::free_view(scene_view);
        crate::sprites::set_eye(None);
        let batches: Vec<crate::sprites::Batch> = draws.iter().filter_map(crate::sprites::batch).collect();
        let lines: Vec<crate::lines::Line> = draws.iter().filter_map(crate::lines::line).collect();
        let other: Vec<String> = draws.iter().filter(|d| crate::sprites::batch(d).is_none() && crate::lines::line(d).is_none() && d.grab.is_none() && d.clear.is_none())
            .map(|d| super::material_shader(d.material).or_else(|| super::material_name(d.material)).unwrap_or_else(|| format!("{:#x}", d.material))).collect();
        let n: usize = batches.iter().map(|b| b.len()).sum();
        let triangles: usize = lines.iter().map(|l| l.indices.len() / 3).sum();
        let mut said = format!("{} custom objects: {} sprite batches, {n} sprites; {} lines, {triangles} triangles", objects.len(), batches.len(), lines.len());
        if !other.is_empty() { said += &format!(", and {} draws not drawn here ({})", other.len(), other.join(", ")); }
        if std::env::var_os("ENGINE2_FRAME_DUMP").is_some() {
            use crate::sprites::at;
            for (k, b) in batches.iter().enumerate() {
                let f = |i: usize, w: usize| f32::from_bits(b.word(i, w));
                let first: Vec<String> = (0..b.len().min(4)).map(|i| format!("at {:.1},{:.1},{:.1} scale {:.1}x{:.1} billboard {} texture {:?}",
                    f(i, at::POSITION), f(i, at::POSITION + 1), f(i, at::POSITION + 2), f(i, at::SCALE), f(i, at::SCALE + 1), b.word(i, at::BILLBOARD),
                    crate::gpu::texture_at(b.word(i, at::TEXTURE) as i32).and_then(super::texture_name))).collect();
                said += &format!("; batch {k} ({} sprites, {}): {}", b.len(), if b.additive { "added" } else { "blended" }, first.join(" | "));
            }
            for (k, l) in lines.iter().enumerate() {
                let ends = l.indices.first().zip(l.indices.last()).map(|(&a, &b)| (l.vertices[a as usize].position, l.vertices[b as usize].position));
                said += &format!("; line {k} ({} triangles, {}{}, material {:?}, texture {:?}{}): from {:?}", l.indices.len() / 3,
                    if l.additive { "added" } else if l.opaque { "opaque" } else { "blended" }, if l.lit { ", lit" } else { "" },
                    l.material.and_then(super::material_name), l.texture.and_then(super::texture_name), if l.missing_texture { " (not found: a band in its place)" } else { "" }, ends);
            }
        }
        (batches, lines, said)
    }

    pub fn frame(camera: usize, width: i32, height: i32) -> Option<(Vec<u8>, String)> {
        frame_into(camera, width, height, None)
    }

    /// A sky by its material. One that can't be had is said once, and looked
    /// for again every few seconds (a package may yet bring it), not each frame.
    fn sky_of(r: &Renderer, name: &str) -> Option<Arc<crate::render::Sky>> {
        static MISSING: OnceLock<Mutex<HashMap<String, std::time::Instant>>> = OnceLock::new();
        let missing = MISSING.get_or_init(Default::default);
        if missing.lock().unwrap().get(name).is_some_and(|since| since.elapsed().as_secs() < 5) { return None; }
        match r.sky(&EngineFiles, name) {
            Ok(s) => Some(s),
            Err(e) => {
                if missing.lock().unwrap().insert(name.to_string(), std::time::Instant::now()).is_none() { crate::trace::note(&format!("render: {e}")); }
                None
            }
        }
    }

    /// The view drawn into `target` (a window's frame) rather than read back:
    /// the image is empty then.
    pub fn frame_into(camera: usize, width: i32, height: i32, target: Option<(&wgpu::TextureView, wgpu::TextureFormat)>) -> Option<(Vec<u8>, String)> {
        if width <= 1 || height <= 1 { return None; }
        let r = renderer()?;
        let get = |prop| crate::state::get::<f32>(camera, prop);
        let angles = crate::state::get::<S_Angles>(camera, PROP__CCameraRenderer_CameraRotation);
        let (fov, near, far) = (get(PROP__CCameraRenderer_FieldOfView), get(PROP__CCameraRenderer_ZNear), get(PROP__CCameraRenderer_ZFar));
        let view = View::camera(
            v3(crate::state::get::<S_Vector3>(camera, PROP__CCameraRenderer_CameraPosition)),
            [angles.f_pitch, angles.f_yaw, angles.f_roll],
            if fov > 1.0 { fov } else { 60.0 },
            if near > 0.0 { near } else { 1.0 },
            if far > near { far } else { 20000.0 },
            width as f32 / height as f32,
        );
        // what's in its worlds and shown: each object's model, where, how tinted
        let mut placed: Vec<(Arc<Model>, Mat4, [f32; 4], Vec<Mat4>, bool, Option<u64>, usize)> = Vec::new();
        // the objects among them with attributes of their own, for their graph materials
        let mut with_attributes: Vec<(usize, usize)> = Vec::new();
        // ENGINE2_FRAME_DUMP: what each frame placed, said with it
        let dump = std::env::var_os("ENGINE2_FRAME_DUMP").is_some();
        let mut described: Vec<String> = Vec::new();
        for world in super::worlds(camera) {
            for obj in crate::handles::made_with("SceneObject", world) {
                if !super::shown(obj) { continue; }
                if !super::tags_drawn(camera, obj) {
                    if dump { described.push(format!("({} left out by its tags [{}])", crate::models::name_of(crate::animation::model_of(obj)).unwrap_or_else(|| "(built)".into()), super::tag_names(&super::tags(obj)))); }
                    continue;
                }
                let model = crate::animation::model_of(obj);
                let g = match (crate::models::name_of(model), crate::models::meshes_of(model)) {
                    (Some(name), _) => gpu_model(r, &name),
                    (None, Some(meshes)) => gpu_runtime_model(r, model, &meshes),
                    _ => None,
                };
                let Some(g) = g else { continue };
                let m = mat(&crate::animation::Xf::from_managed(&crate::state::get::<S_Transform>(obj, PROP_Transform)));
                let tint = crate::state::get::<S_Vector4>(obj, PROP_TintRGBA).f__vec;
                let tint = if tint.f_X + tint.f_Y + tint.f_Z + tint.f_W == 0.0 { [1.0; 4] } else { [tint.f_X, tint.f_Y, tint.f_Z, tint.f_W] };
                let bones = skin(obj, model);
                if dump { described.push(format!("{} at {:?} scale {:?}, {} bones posed, bounds {:?}..{:?}, tags [{}]", crate::models::name_of(model).unwrap_or_else(|| "(built)".into()), m.w_axis.truncate(), m.x_axis.truncate().length(), bones.len(), g.min, g.max, super::tag_names(&super::tags(obj)))); }
                placed.push((g, m, tint, bones, false, super::chosen_mesh_groups(obj), super::skin(obj)));
                with_attributes.push((placed.len() - 1, obj));
            }
        }
        // the lights in its worlds that a map's lightmaps name, by their bake index
        let worlds = super::worlds(camera);
        let suns: HashMap<usize, S_Vector3> = super::suns().into_iter().collect();
        let mut baked = Vec::new();
        for world in &worlds {
            for light in crate::handles::made_with("SceneLightObject", *world) {
                let Some((index, scale)) = super::baked(light) else { continue };
                if !super::shown(light) || index > 254 { continue; }
                baked.push(BakedLight {
                    index: index as u8,
                    sun: suns.contains_key(&light),
                    position: v3(crate::state::get::<S_Vector3>(light, PROP_WorldPosition)),
                    towards: suns.get(&light).map(|&t| v3(t)).unwrap_or(Vec3::ZERO),
                    color: v3(crate::state::get::<S_Vector3>(light, PROP_Color)) * scale,
                });
            }
        }
        baked.sort_by_key(|l| l.index);
        // and the maps in them: their world nodes' objects, lit by their lightmaps
        let mut world_objects = 0;
        let mut lit = None;
        for scene_world in super::worlds(camera) {
            for world in crate::world::in_scene_world(scene_world) {
                if lit.is_none() {
                    if let Some(folder) = crate::world::with(world, |w| w.folder.to_string_lossy().into_owned()) { lit = lightmaps(r, &folder, &baked); }
                }
                for (name, rows, tint) in crate::world::scene_objects(world) {
                    let Some(g) = gpu_model(r, &name) else { continue };
                    let m = Mat4::from_cols_array_2d(&[
                        [rows[0][0], rows[1][0], rows[2][0], 0.0],
                        [rows[0][1], rows[1][1], rows[2][1], 0.0],
                        [rows[0][2], rows[1][2], rows[2][2], 0.0],
                        [rows[0][3], rows[1][3], rows[2][3], 1.0],
                    ]);
                    placed.push((g, m, tint, Vec::new(), true, None, 0));
                    world_objects += 1;
                }
            }
        }
        // the sun and sky of its worlds: a directional light, a sky object
        let in_worlds = |obj: usize| crate::handles::links(obj).iter().any(|l| worlds.contains(l));
        let mut view = view;
        // cleared to the camera's background colour (authored in sRGB, drawn in linear
        // light), transparent where managed asks for that: a thumbnail's backdrop
        if let Some([r, g, b, a]) = super::clear_color(camera) {
            let linear = |c: f32| if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
            view.clear = [linear(r) as f64, linear(g) as f64, linear(b) as f64, a.clamp(0.0, 1.0) as f64];
        }
        view.lightmaps = lit.clone();
        view.probes = probe_volumes(r, &worlds, &baked);
        for (light, made) in super::suns() {
            let towards = v3(made);
            let c = v3(crate::state::get::<S_Vector3>(light, PROP_Color));
            if !in_worlds(light) || !super::shown(light) { continue; }
            if towards.length_squared() > 1e-6 { view.sun = Some((towards, if c.length_squared() > 0.0 { c } else { Vec3::ONE })); }
        }
        for world in &worlds {
            for sky in crate::handles::made_with("SceneSkyBoxObject", *world) {
                if !super::shown(sky) { continue; }
                let Some(name) = super::material_name(super::sky_material(sky)) else { continue };
                if let Some(s) = sky_of(r, &name) {
                    view.sky = Some(s);
                    let t = crate::state::get::<S_Transform>(sky, PROP_Transform).f_Rotation.f__quat;
                    if t.f_X != 0.0 || t.f_Y != 0.0 || t.f_Z != 0.0 || t.f_W != 0.0 { view.sky_rotation = Quat::from_xyzw(t.f_X, t.f_Y, t.f_Z, t.f_W).normalize(); }
                }
            }
        }
        // what managed draws itself (its custom scene objects: sprite batches), asked for now
        let (sprites, lines, custom) = custom_objects(camera, &view, width, height);
        view.sprites = sprites;
        view.lines = lines;
        // what a game set on an object itself (SceneObject.Attributes), for its graph materials
        view.object_materials = object_materials(r, &placed, &with_attributes);
        let instances: Vec<Instance> = placed.iter().map(|(g, m, tint, bones, lightmapped, mask, skin)| Instance { model: g, transform: *m, tint: *tint, bones: bones.clone(), lightmapped: *lightmapped, origin: None, mask: *mask, skin: *skin }).collect();
        crate::trace::mark("gather");
        // what managed has set on materials since the last frame, for the graphs' materials that read it
        {
            static SEEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
            let (set, values) = super::material_values();
            if SEEN.swap(set, std::sync::atomic::Ordering::Relaxed) != set { r.set_graph_values(&values); }
        }
        let image = r.draw_into(&instances, &view, width as u32, height as u32, target);
        let mut said = format!("render: camera at {:?}, {} objects ({world_objects} the map's) into {width}x{height}; sun {:?}, sky {}, lightmaps {} ({} baked lights, {} probe volumes); {custom}", view.eye, instances.len(), view.sun, view.sky.is_some(), view.lightmaps.is_some(), baked.len(), view.probes.len());
        if dump {
            let a = crate::state::get::<S_Angles>(camera, PROP__CCameraRenderer_CameraRotation);
            said += &format!("; looking {:?} (fov {fov}, planes {near}..{far}), clear {:?}, {}: {}", [a.f_pitch, a.f_yaw, a.f_roll], view.clear, super::camera_tag_names(camera), described.join("; "));
        }
        Some((image, said))
    }
}

#[cfg(feature = "render")]
pub use draw::{frame, frame_into, shared_renderer, to_bitmap, to_texture};
#[cfg(feature = "render")]
#[allow(unused_imports)]
pub use draw::probe_volumes;

#[cfg(not(feature = "render"))]
pub fn to_bitmap(_camera: usize, _pixels: usize, _width: i32, _height: i32, _bpp: i32) {}
#[cfg(not(feature = "render"))]
pub fn to_texture(_camera: usize, _texture: usize) -> bool { false }

#[cfg(test)]
mod tag_tests {
    #[test]
    fn a_camera_draws_by_its_tags() {
        let (camera, viewmodel, prop, player) = (crate::handles::object(), crate::handles::object(), crate::handles::object(), crate::handles::object());
        let (firstperson, ui, viewer) = (crate::string_token("firstperson"), crate::string_token("ui"), crate::string_token("viewer"));
        super::add_tag(viewmodel, firstperson);
        super::add_tag(player, viewer);
        assert!(super::tags_drawn(camera, viewmodel) && super::tags_drawn(camera, prop), "a camera with no tags draws everything");
        // FreeCam: the viewmodel and the UI out, the player's own body in
        super::camera_tag(camera, true, Some(firstperson));
        super::camera_tag(camera, true, Some(ui));
        assert!(!super::tags_drawn(camera, viewmodel), "an excluded tag hides it");
        assert!(super::tags_drawn(camera, prop) && super::tags_drawn(camera, player));
        super::camera_tag(camera, true, None);
        assert!(super::tags_drawn(camera, viewmodel), "cleared, it's drawn again");
        // render tags: only what has one of them
        super::camera_tag(camera, false, Some(viewer));
        assert!(super::tags_drawn(camera, player) && !super::tags_drawn(camera, prop));
        super::remove_tag(player, viewer);
        assert!(!super::tags_drawn(camera, player) && super::tags(player).is_empty());
    }
}

#[cfg(all(test, feature = "render"))]
mod tests {
    use crate::slots::*;

    fn v3(x: f32, y: f32, z: f32) -> S_Vector3 { S_Vector3 { f__vec: S_System_Numerics_Vector3 { f_X: x, f_Y: y, f_Z: z } } }

    /// A map from sbox.game (ENGINE2_SAMPLE_MAP its pak, ENGINE2_CONTENT
    /// its materials), if this machine has one, drawn as managed asks a
    /// camera for it: its world, its sun made and given its bake index as
    /// managed's map loader does, a camera looking straight down at the
    /// edge of a building's shadow (flatgrass's, north of its hall). The
    /// shadow is in the lightmaps alone: the ground in it is dark, beside
    /// it sunlit; and hardly darker than beside it when no light has the
    /// sun's index (each frame exposed on its own, as s&box would).
    #[test]
    fn a_map_is_lit_by_its_lightmaps() {
        let Ok(path) = std::env::var("ENGINE2_SAMPLE_MAP") else { return };
        if std::env::var("ENGINE2_CONTENT").is_err() || !path.ends_with("flatgrass.vpk") { return; }
        if crate::render::Renderer::new().is_err() { eprintln!("no adapter: skipped"); return; }
        crate::fs::add_link("maps/flatgrass.vpk", &path);
        let scene_world = crate::handles::object();
        let world = crate::world::create("maps/flatgrass.vpk", scene_world).expect("mounted");
        let ents = crate::world::lump(world, "default_ents");
        let env = ents.iter().find(|e| e.get(crate::string_token("classname")) == Some("light_environment")).expect("a sun");
        let lights = crate::render::BakedLight::from_entities(&[(*env).clone()]);
        let (towards, color) = (lights[0].towards, lights[0].color);
        unsafe { crate::impls::g_pSceneSystem_CreateDirectionalLight(scene_world, &v3(towards.x, towards.y, towards.z) as *const S_Vector3 as usize) };
        let sun = crate::handles::made_with("SceneLightObject", scene_world)[0];
        crate::state::set(sun, PROP_Color, v3(color.x, color.y, color.z));

        let camera = crate::handles::object();
        crate::state::set(camera, PROP__CCameraRenderer_CameraPosition, v3(100.0, 600.0, 2000.0));
        crate::state::set(camera, PROP__CCameraRenderer_CameraRotation, S_Angles { f_pitch: 89.9, f_yaw: 0.0, f_roll: 0.0 });
        crate::state::set(camera, PROP__CCameraRenderer_FieldOfView, 75f32);
        crate::state::set(camera, PROP__CCameraRenderer_ZNear, 4f32);
        crate::state::set(camera, PROP__CCameraRenderer_ZFar, 40000f32);
        super::add_world(camera, scene_world);
        let frame = || {
            let mut pixels = vec![0u8; 240 * 240 * 4];
            super::to_bitmap(camera, pixels.as_mut_ptr() as usize, 240, 240, 4);
            let mean = |c: std::ops::Range<usize>| {
                let v: Vec<f32> = c.flat_map(|x| (90..150).map(move |y| (x, y))).map(|(x, y)| pixels[(y * 240 + x) * 4..][..3].iter().map(|&p| p as f32).sum::<f32>() / 3.0).collect();
                v.iter().sum::<f32>() / v.len() as f32
            };
            // the strip of shadow beside the hall, and the grass beyond it
            (mean(86..94), mean(30..70))
        };
        let (dark, lit) = frame();
        eprintln!("unindexed: shadow {dark:.0}, grass {lit:.0}");
        // (each frame exposed as s&box would expose it: compare within one)
        assert!(dark > lit * 0.75, "no light has the sun's index: no sun on the grass, so hardly a shadow ({dark:.0} against {lit:.0})");
        unsafe { crate::impls::CScnLghtbjct_SetBakeLightIndex(sun, lights.iter().find(|l| l.sun).map(|l| l.index as i32).unwrap()) };
        let (dark, lit) = frame();
        eprintln!("the sun baked: shadow {dark:.0}, grass {lit:.0}");
        assert!(dark < lit * 0.5, "the sun on the grass, and the hall's shadow on the ground ({dark:.0} against {lit:.0})");

        // its light probe volumes, made as managed's map loader makes them
        // (SceneLightProbe): the one in the hall lights what's in the hall
        let volumes = crate::render::ProbeVolume::from_entities(&ents.iter().map(|e| (*e).clone()).collect::<Vec<_>>());
        for (place, [t, i, s]) in &volumes {
            let v = crate::handles::object();
            crate::handles::register(v, "SceneLightProbeVolumeObject", &[scene_world]);
            let (_, q, p) = place.transform.to_scale_rotation_translation();
            crate::state::set(v, PROP_Transform, S_Transform { f_Position: v3(p.x, p.y, p.z), f_Scale: v3(1.0, 1.0, 1.0), f_Rotation: S_Rotation { f__quat: S_System_Numerics_Quaternion { f_X: q.x, f_Y: q.y, f_Z: q.z, f_W: q.w } } });
            crate::state::set(v, PROP__CScnLghtPrbVlmbj_m_vBoxMins, v3(place.mins.x, place.mins.y, place.mins.z));
            crate::state::set(v, PROP__CScnLghtPrbVlmbj_m_vBoxMaxs, v3(place.maxs.x, place.maxs.y, place.maxs.z));
            crate::state::set(v, PROP__CScnLghtPrbVlmbj_m_nRenderPriority, place.priority);
            let name = |n: &String| std::ffi::CString::new(n.as_str()).unwrap();
            let (t, i, s) = (name(t), name(i), name(s));
            unsafe {
                crate::impls::Set__CScnLghtPrbVlmbj_m_hLightProbeTexture(v, crate::impls::Glue_Resources_GetTexture(t.as_ptr() as usize, core::mem::zeroed()));
                crate::impls::Set__CScnLghtPrbVlmbj_m_hLightProbeDirectLightIndicesTexture(v, crate::impls::Glue_Resources_GetTexture(i.as_ptr() as usize, core::mem::zeroed()));
                crate::impls::Set__CScnLghtPrbVlmbj_m_hLightProbeDirectLightScalarsTexture(v, crate::impls::Glue_Resources_GetTexture(s.as_ptr() as usize, core::mem::zeroed()));
            }
        }
        let built = super::probe_volumes(&crate::render::Renderer::new().unwrap(), &[scene_world], &lights);
        eprintln!("{} of {} probe volumes built", built.len(), volumes.len());
        assert_eq!(built.len(), volumes.len());
        let places: Vec<crate::render::ProbePlace> = built.iter().map(|v| v.place).collect();
        let tv = glam::Vec3::new(170.0, 460.0, 42.0);
        let chosen = crate::render::ProbePlace::choose(&places, tv, tv, glam::Vec3::splat(13.0)).expect("one lights the hall");
        assert_eq!(places[chosen].transform.w_axis.truncate(), glam::Vec3::new(160.0, 496.0, 96.0), "the hall's own, over the map's");
    }
}
