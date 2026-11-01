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

/// A material's shader, from its file (read once).
pub fn material_shader(m: usize) -> Option<String> {
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
        let g = r.model(&EngineFiles, name).map_err(|e| crate::trace::note(&format!("render: {e}"))).ok().map(Arc::new);
        if let Some(g) = &g { for n in &g.notes { crate::trace::note(&format!("render: {n}")); } }
        m.lock().unwrap().insert(key, g.clone());
        g
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
        let Some(r) = renderer() else { return };
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
        let mut placed: Vec<(Arc<Model>, Mat4, [f32; 4], Vec<Mat4>, bool)> = Vec::new();
        for world in super::worlds(camera) {
            for obj in crate::handles::made_with("SceneObject", world) {
                if !super::shown(obj) { continue; }
                let model = crate::animation::model_of(obj);
                let Some(name) = crate::models::name_of(model) else { continue };
                let Some(g) = gpu_model(r, &name) else { continue };
                let m = mat(&crate::animation::Xf::from_managed(&crate::state::get::<S_Transform>(obj, PROP_Transform)));
                let tint = crate::state::get::<S_Vector4>(obj, PROP_TintRGBA).f__vec;
                let tint = if tint.f_X + tint.f_Y + tint.f_Z + tint.f_W == 0.0 { [1.0; 4] } else { [tint.f_X, tint.f_Y, tint.f_Z, tint.f_W] };
                placed.push((g, m, tint, skin(obj, model), false));
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
                    placed.push((g, m, tint, Vec::new(), true));
                    world_objects += 1;
                }
            }
        }
        // the sun and sky of its worlds: a directional light, a sky object
        let in_worlds = |obj: usize| crate::handles::links(obj).iter().any(|l| worlds.contains(l));
        let mut view = view;
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
                match r.sky(&EngineFiles, &name) {
                    Ok(s) => {
                        view.sky = Some(s);
                        let t = crate::state::get::<S_Transform>(sky, PROP_Transform).f_Rotation.f__quat;
                        if t.f_X != 0.0 || t.f_Y != 0.0 || t.f_Z != 0.0 || t.f_W != 0.0 { view.sky_rotation = Quat::from_xyzw(t.f_X, t.f_Y, t.f_Z, t.f_W).normalize(); }
                    }
                    Err(e) => crate::trace::note(&format!("render: {e}")),
                }
            }
        }
        let instances: Vec<Instance> = placed.iter().map(|(g, m, tint, bones, lightmapped)| Instance { model: g, transform: *m, tint: *tint, bones: bones.clone(), lightmapped: *lightmapped, origin: None }).collect();
        let image = r.draw(&instances, &view, width as u32, height as u32);
        let out = unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, (width * height * 4) as usize) };
        out.copy_from_slice(&image);
        crate::trace::note(&format!("render: camera at {:?}, {} objects ({world_objects} the map's) into {width}x{height}; sun {:?}, sky {}, lightmaps {} ({} baked lights, {} probe volumes)", view.eye, instances.len(), view.sun, view.sky.is_some(), view.lightmaps.is_some(), baked.len(), view.probes.len()));
    }
}

#[cfg(feature = "render")]
pub use draw::to_bitmap;
#[cfg(feature = "render")]
pub use draw::probe_volumes;

#[cfg(not(feature = "render"))]
pub fn to_bitmap(_camera: usize, _pixels: usize, _width: i32, _height: i32, _bpp: i32) {}

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
    /// it sunlit; and dark everywhere when no light has the sun's index.
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
        assert!(lit < 120.0, "no light has the sun's index: no sun on the grass ({lit:.0})");
        unsafe { crate::impls::CScnLghtbjct_SetBakeLightIndex(sun, lights.iter().find(|l| l.sun).map(|l| l.index as i32).unwrap()) };
        let (dark, lit) = frame();
        eprintln!("the sun baked: shadow {dark:.0}, grass {lit:.0}");
        assert!(lit > 140.0, "the sun on the grass ({lit:.0})");
        assert!(dark < lit * 0.5, "the hall's shadow on the ground ({dark:.0} against {lit:.0})");

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
