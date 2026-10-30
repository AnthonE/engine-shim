//! A model in the scene (CSceneAnimatableObject, managed's SceneModel): its
//! bones. Each is kept in its parent's space, as the animation leaves it
//! (the model's bind pose, until animation is read) and managed's
//! procedural bones set it; carried down to the model's space, where a
//! physics bone (a ragdoll's) overrides it; and placed in the world by the
//! object's transform. That last is where a renderer finds a bone, and so
//! where ModelHitboxes puts a hitbox.
//!
//! The animation is the object's animation graph run (animgraph.rs: the
//! model's own, or one managed sets), when managed has it use one; else a
//! sequence played straight (assets/anim.rs), as a SkinnedModelRenderer
//! with a sequence asks; with none, the bind pose.
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

use rapier3d::math::{Pose, Rotation, Vector};

use crate::slots::S_Transform;

/// A transform with managed's scale: position, rotation, scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xf {
    pub pos: Vector,
    pub rot: Rotation,
    pub scale: Vector,
}

impl Xf {
    pub const IDENTITY: Xf = Xf { pos: Vector::ZERO, rot: Rotation::IDENTITY, scale: Vector::ONE };

    pub fn of(p: Pose) -> Xf {
        Xf { pos: p.translation, rot: p.rotation, scale: Vector::ONE }
    }

    /// `local` in this transform's space, in the space this one is in.
    pub fn to_world(&self, local: &Xf) -> Xf {
        Xf { pos: self.pos + self.rot * (self.scale * local.pos), rot: (self.rot * local.rot).normalize(), scale: self.scale * local.scale }
    }

    pub fn to_local(&self, world: &Xf) -> Xf {
        let inv = self.rot.inverse();
        let s = self.scale.map(|c| if c.abs() < 1e-12 { 0.0 } else { 1.0 / c });
        Xf { pos: s * (inv * (world.pos - self.pos)), rot: (inv * world.rot).normalize(), scale: s * world.scale }
    }

    pub fn from_managed(t: &S_Transform) -> Xf {
        let s = crate::physics::vec(t.f_Scale);
        // managed's Transform.Zero has no scale; a transform without one is a plain one
        Xf { pos: crate::physics::vec(t.f_Position), rot: crate::physics::quat(t.f_Rotation), scale: if s == Vector::ZERO { Vector::ONE } else { s } }
    }

    pub fn managed(&self) -> S_Transform {
        S_Transform { f_Position: crate::physics::v3(self.pos), f_Scale: crate::physics::v3(self.scale), f_Rotation: crate::physics::rot(self.rot) }
    }
}

/// One scene model's bones.
pub struct Posed {
    pub model: usize,
    /// in each bone's parent's space
    pub local: Vec<Xf>,
    /// in the model's space
    pub model_space: Vec<Xf>,
    /// in the world, and as they were before the last update
    pub world: Vec<Xf>,
    pub previous: Vec<Xf>,
    /// physics bones, in the model's space
    pub overrides: HashMap<usize, Xf>,
    pub delta: f32,
    /// the sequence playing (its index in the model's), where in it (0 to 1), how
    pub sequence: Option<usize>,
    pub cycle: f32,
    pub looping: Option<bool>,
    pub rate: Option<f32>,
    /// the sequence has just begun: an event at its very start fires
    begun: bool,
    /// events passed, (clip, event, seconds into the clip), and tags
    /// changed, (name, AnimTagStatus), for managed to hear
    pub events: Vec<(usize, usize, f32)>,
    pub tag_events: Vec<(usize, i32)>,
    /// its animation graph (animgraph.rs; 0: none) and the graph running
    pub graph: usize,
    pub instance: Option<crate::animgraph::Instance>,
    /// what managed asked to hear when the graph changes (InitAnimGraph)
    pub on_graph_changed: usize,
}

fn all() -> MutexGuard<'static, HashMap<usize, Posed>> {
    static A: OnceLock<Mutex<HashMap<usize, Posed>>> = OnceLock::new();
    A.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// The model's bones in bind pose: parent space, and model space.
fn bind_pose(model: usize) -> (Vec<Xf>, Vec<Xf>) {
    let Some(f) = crate::models::file(model) else { return (Vec::new(), Vec::new()) };
    f.bones.iter().map(|b| (Xf::of(Pose::from_parts(b.pos, b.rot)), Xf::of(b.model))).unzip()
}

/// The object's transform, as managed last set it.
fn transform(obj: usize) -> Xf {
    Xf::from_managed(&crate::state::get::<S_Transform>(obj, crate::slots::PROP_Transform))
}

impl Posed {
    fn new(model: usize) -> Posed {
        let (local, model_space) = bind_pose(model);
        let n = local.len();
        Posed {
            model, local, model_space, world: vec![Xf::IDENTITY; n], previous: vec![Xf::IDENTITY; n], overrides: HashMap::new(), delta: 0.0,
            sequence: None, cycle: 0.0, looping: None, rate: None, begun: false, events: Vec::new(), tag_events: Vec::new(),
            graph: crate::models::file(model).map(crate::animgraph::of_model).unwrap_or(0), instance: None, on_graph_changed: 0,
        }
    }

    fn parents(&self) -> Vec<i32> {
        crate::models::file(self.model).map(|f| f.bones.iter().map(|b| b.parent).collect()).unwrap_or_default()
    }

    /// Model space from parent space, physics bones over it; then the world.
    fn calculate(&mut self, at: Xf) {
        let parents = self.parents();
        for i in 0..self.local.len() {
            self.model_space[i] = match self.overrides.get(&i) {
                Some(o) => *o,
                None => match parents.get(i).copied().unwrap_or(-1) {
                    p if p >= 0 && (p as usize) < i => self.model_space[p as usize].to_world(&self.local[i]),
                    _ => self.local[i],
                },
            };
        }
        self.previous = std::mem::take(&mut self.world);
        self.world = self.model_space.iter().map(|m| at.to_world(m)).collect();
        if self.previous.len() != self.world.len() { self.previous = self.world.clone(); }
    }

    fn reset(&mut self) {
        let (local, _) = bind_pose(self.model);
        if local.len() == self.local.len() { self.local = local; }
    }

    /// The sequence playing: its duration, and whether it loops.
    pub fn playing(&self) -> Option<(&'static crate::assets::anim::Sequence, f32, bool)> {
        let f = crate::models::file(self.model)?;
        let s = f.anim.sequences.get(self.sequence?)?;
        let duration = f.anim.clips.get(s.clip).map(|c| c.duration()).unwrap_or(0.0);
        Some((s, duration, self.looping.unwrap_or(s.looping)))
    }

    pub fn finished(&self) -> bool {
        self.playing().is_none_or(|(_, d, looping)| !looping && (d <= 0.0 || self.cycle >= 1.0))
    }

    /// The animation's step: the sequence on by `delta` seconds (at the
    /// playback rate), and the bones in its pose; with none, the bind pose.
    fn step(&mut self, delta: f32) {
        let Some((s, duration, looping)) = self.playing() else { return self.reset() };
        let from = (!std::mem::take(&mut self.begun)).then_some(self.cycle);
        let step = if duration > 0.0 { delta * self.rate.unwrap_or(1.0) / duration } else { 0.0 };
        let wrapped = looping && self.cycle + step >= 1.0;
        if duration > 0.0 {
            self.cycle += step;
            self.cycle = if looping { self.cycle.rem_euclid(1.0) } else { self.cycle.clamp(0.0, 1.0) };
        }
        if let Some(clip) = crate::models::file(self.model).and_then(|f| f.anim.clips.get(s.clip)).filter(|_| from.is_none() || step > 0.0) {
            self.events.extend(clip.events_between(from, self.cycle, wrapped).map(|e| (s.clip, e, clip.events[e].cycle * duration)));
        }
        self.pose_at(s.clip, self.cycle);
    }

    fn pose_at(&mut self, clip: usize, cycle: f32) {
        let Some(f) = crate::models::file(self.model) else { return };
        let bind: Vec<crate::assets::anim::BonePose> = f.bones.iter().map(|b| crate::assets::anim::BonePose { pos: b.pos, rot: b.rot, scale: 1.0 }).collect();
        let pose = f.anim.sample(clip, cycle, &bind);
        if pose.len() != self.local.len() { return; }
        self.local = pose.iter().map(|p| Xf { pos: p.pos, rot: p.rot, scale: Vector::splat(p.scale) }).collect();
    }

    /// The graph run on by `delta`: the bones in its pose.
    fn run_graph(&mut self, delta: f32) {
        let (Some(g), Some(f)) = (crate::animgraph::graph(self.graph), crate::models::file(self.model)) else { return self.reset() };
        let instance = self.instance.get_or_insert_with(|| crate::animgraph::Instance::new(g));
        let pose = instance.run(g, f, delta * self.rate.unwrap_or(1.0));
        self.events.append(&mut instance.events);
        self.tag_events.extend(instance.tag_events.drain(..).filter_map(|(t, status)| Some((g.tag_names.get(&t)?.as_ptr() as usize, status))));
        if pose.len() != self.local.len() { return; }
        self.local = pose.iter().map(|p| Xf { pos: p.pos, rot: p.rot, scale: Vector::splat(p.scale) }).collect();
    }

    /// Play a sequence by name from its start (an unknown name, or none: stop).
    pub fn play(&mut self, name: &str) {
        self.sequence = crate::models::file(self.model).and_then(|f| f.anim.sequences.iter().position(|s| s.name.eq_ignore_ascii_case(name)));
        self.cycle = 0.0;
        self.begun = true;
    }
}

/// A scene object made of `model`, at `at` (managed's transform), in bind pose.
pub fn create(obj: usize, model: usize) {
    let mut p = Posed::new(model);
    p.calculate(transform(obj));
    all().insert(obj, p);
}

/// The object shows another model now: its bones, in that one's bind pose,
/// and that one's graph.
pub fn change_model(obj: usize, model: usize) {
    let callback = all().get(&obj).map(|p| p.on_graph_changed).unwrap_or(0);
    create(obj, model);
    with(obj, |p| p.on_graph_changed = callback);
    graph_changed(obj);
}

/// Managed's callback for a graph changed, and a first call of it.
pub fn init_graph(obj: usize, callback: usize) {
    with(obj, |p| p.on_graph_changed = callback);
    graph_changed(obj);
}

/// The object runs `graph` now (0: none), from its start.
pub fn set_graph(obj: usize, graph: usize) {
    with(obj, |p| { p.graph = graph; p.instance = None; });
    graph_changed(obj);
}

/// Tells managed the graph changed, if it asked to hear (never under the lock).
fn graph_changed(obj: usize) {
    let callback = all().get(&obj).map(|p| p.on_graph_changed).unwrap_or(0);
    if callback != 0 {
        let f: extern "C" fn(usize) = unsafe { std::mem::transmute(callback) };
        f(obj);
    }
}

pub fn graph_of(obj: usize) -> usize {
    all().get(&obj).map(|p| p.graph).unwrap_or(0)
}

pub fn model_of(obj: usize) -> usize {
    all().get(&obj).map(|p| p.model).unwrap_or(0)
}

#[cfg(test)]
pub fn find_by_model(model: usize) -> Option<usize> {
    all().iter().find(|(_, p)| p.model == model).map(|(&o, _)| o)
}

pub fn with<R>(obj: usize, f: impl FnOnce(&mut Posed) -> R) -> Option<R> {
    all().get_mut(&obj).map(f)
}

/// The animation's step: the bones take the animation's pose, for managed's
/// procedural bones and the world placement to follow.
pub fn update(obj: usize, delta: f32) {
    let use_graph = crate::state::get::<i32>(obj, crate::slots::PROP_ShouldUseAnimGraph) != 0;
    with(obj, |p| {
        p.delta = delta;
        if use_graph && p.graph != 0 { p.run_graph(delta) } else { p.step(delta) }
        // nobody listening: keep the newest
        let n = p.events.len().saturating_sub(256);
        p.events.drain(..n);
        let n = p.tag_events.len().saturating_sub(256);
        p.tag_events.drain(..n);
    });
}

// a graph's parameters on an object, as managed holds them: each (object,
// parameter) at an address of its own

fn parameters() -> MutexGuard<'static, (HashMap<(usize, usize), usize>, HashMap<usize, (usize, usize)>)> {
    static P: OnceLock<Mutex<(HashMap<(usize, usize), usize>, HashMap<usize, (usize, usize)>)>> = OnceLock::new();
    P.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// Parameter `index` of the object's graph: its object (0: none such).
pub fn parameter(obj: usize, index: usize) -> usize {
    let n = crate::animgraph::graph(graph_of(obj)).map(|g| g.params.len()).unwrap_or(0);
    if index >= n { return 0; }
    let mut p = parameters();
    if let Some(&ptr) = p.0.get(&(obj, index)) { return ptr; }
    let ptr = crate::handles::object();
    p.0.insert((obj, index), ptr);
    p.1.insert(ptr, (obj, index));
    ptr
}

pub fn parameter_named(obj: usize, name: &str) -> usize {
    let Some(g) = crate::animgraph::graph(graph_of(obj)) else { return 0 };
    g.params.iter().position(|p| p.name.to_bytes().eq_ignore_ascii_case(name.as_bytes())).map(|i| parameter(obj, i)).unwrap_or(0)
}

/// A parameter object's graph parameter, and its value on its object (set with `set`).
pub fn with_parameter<R>(ptr: usize, f: impl FnOnce(&'static crate::animgraph::Param, &mut crate::animgraph::Variant) -> R) -> Option<R> {
    let (obj, index) = *parameters().1.get(&ptr)?;
    let g = crate::animgraph::graph(graph_of(obj))?;
    let param = g.params.get(index)?;
    with(obj, |p| {
        let i = p.instance.get_or_insert_with(|| crate::animgraph::Instance::new(g));
        i.values.get_mut(index).map(|v| f(param, v))
    }).flatten()
}

pub fn reset_parameters(obj: usize) {
    let Some(g) = crate::animgraph::graph(graph_of(obj)) else { return };
    with(obj, |p| if let Some(i) = p.instance.as_mut() { i.reset_parameters(g) });
}

pub fn set_bind_pose(obj: usize) {
    with(obj, Posed::reset);
}

pub fn calculate(obj: usize) {
    let at = transform(obj);
    with(obj, |p| p.calculate(at));
}

/// Bone merge: this object's bones take `from`'s pose wherever their names
/// match (clothing on a body), the rest carried along from their parents.
pub fn merge_from(obj: usize, from: usize) {
    let mut a = all();
    let Some(src) = a.get(&from) else { return };
    let (Some(fs), Some(fd)) = (crate::models::file(src.model), a.get(&obj).and_then(|p| crate::models::file(p.model))) else { return };
    let src_model: HashMap<String, Xf> = fs.bones.iter().zip(&src.model_space).map(|(b, x)| (b.name.to_string_lossy().to_ascii_lowercase(), *x)).collect();
    let Some(dst) = a.get_mut(&obj) else { return };
    for (i, b) in fd.bones.iter().enumerate() {
        if i >= dst.local.len() { break; }
        if let Some(m) = src_model.get(&b.name.to_string_lossy().to_ascii_lowercase()) {
            let parent = match b.parent { p if p >= 0 && (p as usize) < i => dst.model_space[p as usize], _ => Xf::IDENTITY };
            dst.local[i] = parent.to_local(m);
            dst.model_space[i] = *m;
        } else if b.parent >= 0 && (b.parent as usize) < i {
            dst.model_space[i] = dst.model_space[b.parent as usize].to_world(&dst.local[i]);
        }
    }
    drop(a);
    calculate(obj);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transforms_nest_and_undo() {
        let a = Xf { pos: Vector::new(10.0, 0.0, 0.0), rot: Rotation::from_rotation_z(std::f32::consts::FRAC_PI_2), scale: Vector::splat(2.0) };
        let b = Xf { pos: Vector::new(1.0, 0.0, 0.0), rot: Rotation::IDENTITY, scale: Vector::ONE };
        let w = a.to_world(&b);
        // 1 along a's turned x, doubled: 2 along world y
        assert!((w.pos - Vector::new(10.0, 2.0, 0.0)).length() < 1e-5, "{:?}", w.pos);
        let back = a.to_local(&w);
        assert!((back.pos - b.pos).length() < 1e-5 && (back.scale - b.scale).length() < 1e-5);
    }

    /// What managed hears from an object's animation events: each one's
    /// kind, and what it carries, read through the KeyValues3 slots.
    static HEARD: std::sync::Mutex<Vec<(String, f32, String, i32)>> = std::sync::Mutex::new(Vec::new());
    extern "C" fn heard(name: usize, cycle: f32, _time: f32, data: usize) {
        let s = |p: usize| unsafe { std::ffi::CStr::from_ptr(p as *const std::ffi::c_char) }.to_string_lossy().into_owned();
        let bodygroup = s(unsafe { crate::impls::KeyValues3_GetMemberString(data, c"bodygroup".as_ptr() as usize) });
        let value = unsafe { crate::impls::KeyValues3_GetMemberInt(data, c"value".as_ptr() as usize, -1) };
        HEARD.lock().unwrap().push((s(name), cycle, bodygroup, value));
    }

    #[test]
    fn a_downloaded_reload_sets_its_body_group() {
        // s&box's download cache, if this machine has one: Facepunch's content, read where it lies
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let dir = std::path::Path::new(&dir).join("models/weapons/sbox_shotgun_spaghellim4");
        let Some(file) = std::fs::read_dir(&dir).ok().and_then(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| {
            let n = p.file_name().unwrap().to_string_lossy().to_string();
            n.starts_with("v_spaghellim4.") && n.ends_with(".vmdl_c")
        })) else { return };
        crate::fs::add_link("models/weapons/sbox_shotgun_spaghellim4/v_spaghellim4.vmdl_c", file.to_str().unwrap());
        let model = crate::models::load("models/weapons/sbox_shotgun_spaghellim4/v_spaghellim4.vmdl");
        let obj = crate::handles::object();
        super::create(obj, model);
        super::with(obj, |p| p.play("Reload_Shell"));
        let (_, duration, looping) = super::with(obj, |p| p.playing().map(|(s, d, l)| (s.clip, d, l))).flatten().unwrap();
        assert!(!looping && duration > 0.0);
        // played through and on past its end, a frame at a time: each event once
        for _ in 0..((duration * 60.0) as usize + 30) { super::update(obj, 1.0 / 60.0); }
        assert_eq!(unsafe { crate::impls::CScnnmtblbjct_PendingAnimationEvents(obj) }, 2);
        unsafe { crate::impls::CScnnmtblbjct_RunAnimationEvents(obj, heard as *const () as usize) };
        assert_eq!(unsafe { crate::impls::CScnnmtblbjct_PendingAnimationEvents(obj) }, 0, "heard once");
        let h = HEARD.lock().unwrap().clone();
        eprintln!("{h:?}");
        assert_eq!(h.len(), 2);
        assert!(h.iter().all(|e| e.0 == "AE_CL_BODYGROUP_SET_VALUE" && e.2 == "shells_animated"), "{h:?}");
        assert_eq!((h[0].3, h[0].1 < 0.05), (1, true), "the shell shows as the reload starts");
        assert!(h[1].1 > h[0].1, "then the next, later on");
    }
}
