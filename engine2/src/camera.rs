//! Cameras managed asks for pixels (CCameraRenderer): the scene worlds each
//! is given, which scene objects are shown, and, when managed asks for a
//! camera's view as a bitmap (CameraComponent.RenderToBitmap), the scene
//! objects in its worlds drawn from where managed put it (render/).
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

static WORLDS: Mutex<Option<HashMap<usize, Vec<usize>>>> = Mutex::new(None);
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

    use crate::render::{EngineFiles, Instance, Model, Renderer, View};
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
        let mut placed: Vec<(Arc<Model>, Mat4, [f32; 4], Vec<Mat4>)> = Vec::new();
        for world in super::worlds(camera) {
            for obj in crate::handles::made_with("SceneObject", world) {
                if !super::shown(obj) { continue; }
                let model = crate::animation::model_of(obj);
                let Some(name) = crate::models::name_of(model) else { continue };
                let Some(g) = gpu_model(r, &name) else { continue };
                let m = mat(&crate::animation::Xf::from_managed(&crate::state::get::<S_Transform>(obj, PROP_Transform)));
                let tint = crate::state::get::<S_Vector4>(obj, PROP_TintRGBA).f__vec;
                let tint = if tint.f_X + tint.f_Y + tint.f_Z + tint.f_W == 0.0 { [1.0; 4] } else { [tint.f_X, tint.f_Y, tint.f_Z, tint.f_W] };
                placed.push((g, m, tint, skin(obj, model)));
            }
        }
        // and the maps in them: their world nodes' objects
        let mut world_objects = 0;
        for scene_world in super::worlds(camera) {
            for world in crate::world::in_scene_world(scene_world) {
                for (name, rows, tint) in crate::world::scene_objects(world) {
                    let Some(g) = gpu_model(r, &name) else { continue };
                    let m = Mat4::from_cols_array_2d(&[
                        [rows[0][0], rows[1][0], rows[2][0], 0.0],
                        [rows[0][1], rows[1][1], rows[2][1], 0.0],
                        [rows[0][2], rows[1][2], rows[2][2], 0.0],
                        [rows[0][3], rows[1][3], rows[2][3], 1.0],
                    ]);
                    placed.push((g, m, tint, Vec::new()));
                    world_objects += 1;
                }
            }
        }
        let instances: Vec<Instance> = placed.iter().map(|(g, m, tint, bones)| Instance { model: g, transform: *m, tint: *tint, bones: bones.clone() }).collect();
        let image = r.draw(&instances, &view, width as u32, height as u32);
        let out = unsafe { std::slice::from_raw_parts_mut(pixels as *mut u8, (width * height * 4) as usize) };
        out.copy_from_slice(&image);
        crate::trace::note(&format!("render: camera at {:?}, {} objects ({world_objects} the map's) into {width}x{height}", view.eye, instances.len()));
    }
}

#[cfg(feature = "render")]
pub use draw::to_bitmap;

#[cfg(not(feature = "render"))]
pub fn to_bitmap(_camera: usize, _pixels: usize, _width: i32, _height: i32, _bpp: i32) {}
