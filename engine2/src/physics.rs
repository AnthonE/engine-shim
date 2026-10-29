//! Physics: s&box's worlds, bodies and shapes, simulated by Rapier where
//! s&box's native engine runs Box3D. A world steps when managed says
//! (`StepSimulation`), tells managed which bodies moved (`OnActive`), and
//! hands over what touched what (`ProcessIntersections`); traces sweep a
//! ray or a shape through it, filtered by shape tags as managed asks.
//!
//! Units are s&box's: inches, kilograms, seconds. Rapier's tolerances are
//! told so (`length_unit`).
//!
//! Objects are managed's handles (handles.rs) and are found here by their
//! pointer. Nothing calls managed while the lock is held: a callback may
//! call straight back in.
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, MutexGuard, OnceLock};

use rapier3d::parry::bounding_volume::{Aabb, BoundingVolume};
use rapier3d::parry::mass_properties::MassProperties;
use rapier3d::parry::query::ShapeCastOptions;
use rapier3d::parry::shape::{FeatureId, HeightField, SharedShape, TriMeshFlags};
use rapier3d::prelude::*;

pub mod joints;

use crate::handles::murmur2;
use crate::surfaces::{self, Props};
use crate::slots::{S_BBox, S_Capsule, S_PhysicsTrace_Result, S_Rotation, S_Sandbox_Sphere, S_Transform, S_Vector3};

/// Inches in a metre: s&box's length unit, for Rapier's tolerances.
const INCHES_PER_METRE: f32 = 39.37;
/// A new world's gravity, s&box's default project's (managed sets its own).
const DEFAULT_GRAVITY: Vector = Vector::new(0.0, 0.0, -850.0);

// PhysicsBodyType
pub const STATIC: i64 = 0;
pub const KEYFRAMED: i64 = 1;
pub const DYNAMIC: i64 = 2;

// PhysicsShapeType
pub const SHAPE_SPHERE: i64 = 0;
pub const SHAPE_CAPSULE: i64 = 1;
pub const SHAPE_HULL: i64 = 2;
pub const SHAPE_MESH: i64 = 3;
pub const SHAPE_HEIGHTFIELD: i64 = 4;

// CollisionFunctionMask
const SOLID_CONTACT: u8 = 1;
const TOUCH_EVENT: u8 = 2;
const TOUCH_PERSISTS: u8 = 8;

// PhysicsBodyCreationFlags
const BODY_GRAVITY: u32 = 1;
const BODY_AUTO_SLEEP: u32 = 2;
const BODY_AWAKE: u32 = 4;
const BODY_BULLET: u32 = 8;
const BODY_OVERRIDE_MASS_CENTER: u32 = 16;

// PhysicsShapeCreationFlags
const SHAPE_SENSOR: u32 = 1;
const SHAPE_IGNORE_TRACES: u32 = 2;
const SHAPE_NO_MASS: u32 = 4;
const SHAPE_TAGS: u32 = 8;
const SHAPE_DEFAULT_SOLID_TAG: u32 = 32;

/// Sandbox.PhysicsBodySettings, as managed lays it out.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BodySettings {
    pub body_type: i32,
    pub position: S_Vector3,
    pub rotation: S_Rotation,
    pub velocity: S_Vector3,
    pub angular_velocity: S_Vector3,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub gravity_scale: f32,
    pub sleep_threshold: f32,
    pub mass: f32,
    pub inertia_scale: f32,
    pub mass_center: S_Vector3,
    pub flags: u32,
    pub motion_locks: u32,
}
const _: () = assert!(core::mem::size_of::<BodySettings>() == 100);

impl Default for BodySettings {
    /// PhysicsBodySettings' own defaults: a static body, awake, with gravity.
    fn default() -> Self {
        BodySettings {
            body_type: STATIC as i32,
            position: v3(Vector::ZERO),
            rotation: rot(Rotation::IDENTITY),
            velocity: v3(Vector::ZERO),
            angular_velocity: v3(Vector::ZERO),
            linear_damping: 0.0,
            angular_damping: 0.0,
            gravity_scale: 1.0,
            sleep_threshold: 2.0,
            mass: 0.0,
            inertia_scale: 1.0,
            mass_center: v3(Vector::ZERO),
            flags: BODY_GRAVITY | BODY_AUTO_SLEEP | BODY_AWAKE,
            motion_locks: 0,
        }
    }
}

/// Sandbox.PhysicsShapeSettings, as managed lays it out.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShapeSettings {
    pub surface_indices: usize,
    pub tags: [u32; 16],
    pub surface_velocity: S_Vector3,
    pub friction: f32,
    pub elasticity: f32,
    pub rolling_resistance: f32,
    pub surface_index: i32,
    pub collision_function_mask: u32,
    pub flags: u32,
    pub surface_count: i32,
}
const _: () = assert!(core::mem::size_of::<ShapeSettings>() == 112);

impl Default for ShapeSettings {
    fn default() -> Self {
        ShapeSettings {
            surface_indices: 0,
            tags: [0; 16],
            surface_velocity: v3(Vector::ZERO),
            friction: -1.0,
            elasticity: -1.0,
            rolling_resistance: -1.0,
            surface_index: -1,
            collision_function_mask: 3,
            flags: 16,
            surface_count: 0,
        }
    }
}

/// PhysicsTrace.Request: what to sweep, from where to where, and what it
/// may hit.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TraceRequest {
    pub world: usize,
    pub body: usize,
    pub start: S_Vector3,
    pub end: S_Vector3,
    pub tag_require: [u32; 8],
    pub tag_any: [u32; 8],
    pub tag_exclude: [u32; 8],
    pub filter: usize,
    pub trigger_filter: u8,
    pub object_set_mask: u16,
    pub use_hit_position: u8,
    pub shape: TraceShape,
}
const _: () = assert!(core::mem::size_of::<TraceRequest>() == 208);

/// PhysicsTrace.Request.Shape: a sphere (radius 0: a ray), a box, a capsule
/// or a cylinder (a cone when its radii differ), turned by `rotation`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TraceShape {
    pub kind: i32,
    pub rotation: S_Rotation,
    pub mins: S_Vector3,
    pub maxs: S_Vector3,
    pub radius: [f32; 2],
}

/// PhysicsWorld3d.VPhysIntersectionNotification_t: one touch begun, ended,
/// kept, or a trigger entered or left.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Intersection {
    pub reason: i32,
    pub left: Side,
    pub right: Side,
    pub point: [f32; 3],
    pub speed: [f32; 3],
    pub normal: [f32; 3],
    pub normal_speed: f32,
    pub impulse: f32,
}
const _: () = assert!(core::mem::size_of::<Intersection>() == 120);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Side {
    pub shape: usize,
    pub body: usize,
    pub surface: i32,
    pub shape_handle: i32,
    pub body_handle: i32,
}

// IntersectionEventType_t
pub const TOUCH_BEGIN: i32 = 0;
pub const TOUCH_END: i32 = 1;
pub const TOUCH_PERSISTS_EVENT: i32 = 2;
pub const TRIGGER_BEGIN: i32 = 4;
pub const TRIGGER_END: i32 = 5;

// ---- conversions: managed's structs are glam's in another dress

pub fn vec(v: S_Vector3) -> Vector {
    Vector::new(v.f__vec.f_X, v.f__vec.f_Y, v.f__vec.f_Z)
}

pub fn v3(v: Vector) -> S_Vector3 {
    S_Vector3 { f__vec: crate::slots::S_System_Numerics_Vector3 { f_X: v.x, f_Y: v.y, f_Z: v.z } }
}

pub fn quat(r: S_Rotation) -> Rotation {
    let q = r.f__quat;
    let q = Rotation::from_xyzw(q.f_X, q.f_Y, q.f_Z, q.f_W);
    // managed's default Rotation is all zeroes: no rotation, not a degenerate one
    if q.length_squared() < 1e-12 { Rotation::IDENTITY } else { q.normalize() }
}

pub fn rot(q: Rotation) -> S_Rotation {
    S_Rotation { f__quat: crate::slots::S_System_Numerics_Quaternion { f_X: q.x, f_Y: q.y, f_Z: q.z, f_W: q.w } }
}

fn transform(p: &Pose) -> S_Transform {
    S_Transform { f_Position: v3(p.translation), f_Scale: v3(Vector::ONE), f_Rotation: rot(p.rotation) }
}

fn pose(t: &S_Transform) -> Pose {
    Pose::from_parts(vec(t.f_Position), quat(t.f_Rotation))
}

/// A value managed passed by pointer (null: zeroes).
///
/// # Safety
/// `p` is null or points at a `T`.
pub unsafe fn read<T: Copy + Default>(p: usize) -> T {
    if p == 0 { T::default() } else { unsafe { core::ptr::read_unaligned(p as *const T) } }
}

/// # Safety
/// `p` is null or points at a Vector3.
pub unsafe fn read_vec(p: usize) -> Vector {
    if p == 0 { Vector::ZERO } else { vec(unsafe { core::ptr::read_unaligned(p as *const S_Vector3) }) }
}

/// # Safety
/// `p` is null or points at a Rotation.
pub unsafe fn read_rot(p: usize) -> Rotation {
    if p == 0 { Rotation::IDENTITY } else { quat(unsafe { core::ptr::read_unaligned(p as *const S_Rotation) }) }
}

/// # Safety
/// `p` is null or points at a Transform.
pub unsafe fn read_pose(p: usize) -> Pose {
    if p == 0 { Pose::IDENTITY } else { pose(&unsafe { core::ptr::read_unaligned(p as *const S_Transform) }) }
}

/// `n` Vector3s at `p`.
///
/// # Safety
/// `p` is null or points at `n` Vector3s.
pub unsafe fn read_points(p: usize, n: i32) -> Vec<Vector> {
    if p == 0 || n <= 0 { return Vec::new(); }
    let s = unsafe { core::slice::from_raw_parts(p as *const S_Vector3, n as usize) };
    s.iter().map(|&v| vec(v)).collect()
}

// ---- collision rules: which tags collide, trigger or ignore each other

const RULE_UNSET: u8 = 0;
const RULE_COLLIDE: u8 = 1;
const RULE_TRIGGER: u8 = 2;
const RULE_IGNORE: u8 = 3;

/// Sandbox.Physics.CollisionRules, read from the JSON managed sends: the
/// least colliding rule of any pair of the two shapes' tags, else of their
/// tags' defaults, else collide.
#[derive(Default)]
struct Rules {
    defaults: HashMap<u32, u8>,
    pairs: HashMap<(u32, u32), u8>,
}

fn rule_of(v: &serde_json::Value) -> u8 {
    match v {
        serde_json::Value::Number(n) => n.as_u64().unwrap_or(0).min(3) as u8,
        serde_json::Value::String(s) => match s.to_ascii_lowercase().as_str() {
            "collide" => RULE_COLLIDE,
            "trigger" => RULE_TRIGGER,
            "ignore" => RULE_IGNORE,
            _ => RULE_UNSET,
        },
        _ => RULE_UNSET,
    }
}

impl Rules {
    fn from_json(json: &str) -> Rules {
        let mut r = Rules::default();
        let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else { return r };
        if let Some(d) = v.get("Defaults").and_then(|d| d.as_object()) {
            for (tag, rule) in d {
                r.defaults.insert(murmur2(tag), rule_of(rule));
            }
        }
        if let Some(p) = v.get("Pairs").and_then(|p| p.as_array()) {
            for pair in p {
                let (Some(a), Some(b)) = (pair.get("a").and_then(|a| a.as_str()), pair.get("b").and_then(|b| b.as_str())) else { continue };
                let rule = pair.get("r").map(rule_of).unwrap_or(RULE_UNSET);
                if rule == RULE_UNSET { continue; }
                let key = Rules::key(murmur2(a), murmur2(b));
                let was = r.pairs.get(&key).copied().unwrap_or(RULE_UNSET);
                r.pairs.insert(key, was.max(rule));
            }
        }
        r
    }

    fn key(a: u32, b: u32) -> (u32, u32) {
        (a.min(b), a.max(b))
    }

    /// CollisionRules.GetCollisionRule over two tag sets.
    fn between(&self, left: &[u32], right: &[u32]) -> u8 {
        if self.pairs.is_empty() && self.defaults.is_empty() { return RULE_COLLIDE; }
        let mut rule = RULE_UNSET;
        for &a in left {
            for &b in right {
                if let Some(&p) = self.pairs.get(&Rules::key(a, b)) {
                    rule = rule.max(p);
                    if rule == RULE_IGNORE { return rule; }
                }
            }
        }
        if rule == RULE_UNSET {
            for t in left.iter().chain(right) {
                rule = rule.max(self.defaults.get(t).copied().unwrap_or(RULE_UNSET));
            }
        }
        rule.max(RULE_COLLIDE)
    }
}

// ---- the objects

pub struct Shape {
    pub body: usize,
    pub collider: ColliderHandle,
    pub kind: i64,
    pub tags: Vec<u32>,
    pub mask: u8,
    pub trigger: bool,
    pub ignore_traces: bool,
    pub no_mass: bool,
    /// the surface it is made of (surfaces.rs; -1: none named, the default)
    pub surface: i32,
    /// the surfaces a mesh's or height field's triangles name, by their material
    pub parts: Vec<i32>,
    /// each triangle's material (by parry's triangle id), if they have their own
    pub materials: Vec<u8>,
    /// friction, elasticity and rolling resistance set over its surface's
    pub own_friction: Option<f32>,
    pub own_elasticity: Option<f32>,
    pub own_rolling_resistance: Option<f32>,
    pub local_velocity: Vector,
    /// a sphere's or capsule's own description, for AsSphere/AsCapsule
    pub ends: (Vector, Vector, f32),
}

pub struct Body {
    pub handle: RigidBodyHandle,
    pub kind: i64,
    pub shapes: Vec<usize>,
    pub mass: Option<f32>,
    pub mass_center: Option<Vector>,
    pub inertia: Option<(Vector, Rotation)>,
    pub inertia_scale: f32,
    pub deferred: bool,
    pub gravity: bool,
    pub gravity_scale: f32,
    pub auto_sleep: bool,
    pub sleep_threshold: f32,
    pub touch_events: bool,
    pub asleep: bool,
}

/// What the narrow phase's filter reads: each collider's shape, and the rules.
#[derive(Default)]
struct Meta {
    rules: Rules,
    /// the surfaces, as of the last change: the step reads them, and may not
    /// wait on surfaces.rs's lock
    surfaces: Vec<Props>,
    shapes: HashMap<usize, Shape>,
    by_collider: HashMap<ColliderHandle, usize>,
}

impl Shape {
    /// The surface at `triangle` (-1: anywhere): its material's, if it has one.
    pub fn surface_at(&self, triangle: i32) -> i32 {
        if let Ok(t) = usize::try_from(triangle) {
            if !self.materials.is_empty() {
                // past the count: the same triangle's back face
                let m = self.materials[t % self.materials.len()];
                if let Some(&s) = self.parts.get(m as usize).filter(|&&s| s >= 0) { return s; }
            }
        }
        self.surface
    }

    pub fn friction(&self, t: &[Props], triangle: i32) -> f32 {
        self.own_friction.unwrap_or_else(|| surface_props(t, self.surface_at(triangle)).friction)
    }

    pub fn elasticity(&self, t: &[Props], triangle: i32) -> f32 {
        self.own_elasticity.unwrap_or_else(|| surface_props(t, self.surface_at(triangle)).elasticity)
    }

    /// Its friction and elasticity on its collider (a triangle's own surface
    /// is the contact hook's), as Box3D mixes them: friction the geometric
    /// mean of two shapes', elasticity the greater.
    fn apply(&self, t: &[Props], co: &mut Collider) {
        co.set_friction(self.friction(t, -1));
        co.set_restitution(self.elasticity(t, -1).max(0.0));
        co.set_friction_combine_rule(CoefficientCombineRule::GeometricMean);
        co.set_restitution_combine_rule(CoefficientCombineRule::Max);
        let mut hooks = ActiveHooks::FILTER_CONTACT_PAIRS | ActiveHooks::FILTER_INTERSECTION_PAIR;
        if !self.materials.is_empty() { hooks |= ActiveHooks::MODIFY_SOLVER_CONTACTS; }
        co.set_active_hooks(hooks);
    }
}

/// Surface `i`'s props from a world's copy of the table (the default's for -1).
fn surface_props(t: &[Props], i: i32) -> Props {
    usize::try_from(i).ok().and_then(|i| t.get(i)).or(t.first()).copied().unwrap_or(surfaces::DEFAULT)
}

impl Meta {
    fn shape(&self, c: ColliderHandle) -> Option<&Shape> {
        self.by_collider.get(&c).and_then(|p| self.shapes.get(p))
    }
}

impl PhysicsHooks for Meta {
    fn filter_contact_pair(&self, cx: &PairFilterContext) -> Option<SolverFlags> {
        let (Some(a), Some(b)) = (self.shape(cx.collider1), self.shape(cx.collider2)) else {
            return Some(SolverFlags::COMPUTE_RIGID_IMPULSES);
        };
        if a.body == b.body || a.mask & SOLID_CONTACT == 0 || b.mask & SOLID_CONTACT == 0 { return None; }
        (self.rules.between(&a.tags, &b.tags) == RULE_COLLIDE).then_some(SolverFlags::COMPUTE_RIGID_IMPULSES)
    }

    fn filter_intersection_pair(&self, cx: &PairFilterContext) -> bool {
        let (Some(a), Some(b)) = (self.shape(cx.collider1), self.shape(cx.collider2)) else { return true };
        a.body != b.body && !(a.trigger && b.trigger) && self.rules.between(&a.tags, &b.tags) != RULE_IGNORE
    }

    /// A contact with a mesh or height field whose triangles have surfaces of
    /// their own: the touching triangle's friction and elasticity.
    fn modify_solver_contacts(&self, cx: &mut ContactModificationContext) {
        let (c1, c2) = (cx.collider1, cx.collider2);
        let Some(m) = cx.rigid_mut() else { return };
        let side = |c: ColliderHandle, tri: u32| self.shape(c).map(|s| (s.friction(&self.surfaces, tri as i32), s.elasticity(&self.surfaces, tri as i32)));
        let (Some((f1, e1)), Some((f2, e2))) = (side(c1, m.manifold.subshape1), side(c2, m.manifold.subshape2)) else { return };
        *m.friction = (f1.max(0.0) * f2.max(0.0)).sqrt();
        *m.restitution = e1.max(e2).max(0.0);
    }
}

pub struct World {
    sim: rapier3d::pipeline::PhysicsWorld,
    meta: Meta,
    bodies: HashMap<usize, Body>,
    by_handle: HashMap<RigidBodyHandle, usize>,
    reference: usize,
    sleeping: bool,
    max_speed: f32,
    simulation: i64,
    touching: HashSet<(ColliderHandle, ColliderHandle)>,
    triggering: HashSet<(ColliderHandle, ColliderHandle)>,
    pending: Vec<Intersection>,
    /// bodies moved or reshaped since the last step: queries see them anyway
    stale: HashSet<RigidBodyHandle>,
    joints: HashMap<usize, joints::Joint>,
}

impl World {
    fn new() -> World {
        let mut sim = rapier3d::pipeline::PhysicsWorld::new();
        sim.gravity = DEFAULT_GRAVITY;
        sim.integration_parameters.length_unit = INCHES_PER_METRE;
        World {
            sim,
            meta: Meta::default(),
            bodies: HashMap::new(),
            by_handle: HashMap::new(),
            reference: 0,
            sleeping: true,
            max_speed: f32::INFINITY,
            simulation: 0,
            touching: HashSet::new(),
            triggering: HashSet::new(),
            pending: Vec::new(),
            stale: HashSet::new(),
            joints: HashMap::new(),
        }
    }

    /// Brings the colliders of bodies changed since the last step up to date
    /// in the query tree, which a step would otherwise do.
    fn refresh(&mut self) {
        for h in self.stale.drain() {
            let Some(rb) = self.sim.bodies.get(h) else { continue };
            let at = *rb.position();
            for &c in rb.colliders() {
                let Some(co) = self.sim.colliders.get_mut(c) else { continue };
                if let Some(local) = co.position_wrt_parent().copied() { co.set_position(at * local); }
                let aabb = co.compute_broad_phase_aabb(&self.sim.integration_parameters, &self.sim.bodies);
                self.sim.broad_phase.set_aabb(&self.sim.integration_parameters, c, aabb);
            }
        }
    }
}

#[derive(Default)]
struct Physics {
    worlds: HashMap<usize, World>,
    /// body pointer -> its world's
    body_world: HashMap<usize, usize>,
    /// shape pointer -> its world's
    shape_world: HashMap<usize, usize>,
    /// joint pointer -> its world's
    joint_world: HashMap<usize, usize>,
}

fn physics() -> MutexGuard<'static, Physics> {
    static P: OnceLock<Mutex<Physics>> = OnceLock::new();
    P.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

impl Physics {
    fn body(&mut self, ptr: usize) -> Option<(&mut World, &mut Body)> {
        let w = *self.body_world.get(&ptr)?;
        let world = self.worlds.get_mut(&w)?;
        let body = world.bodies.get_mut(&ptr)? as *mut Body;
        // the body lives in the world's map; the two borrows touch different fields
        Some((world, unsafe { &mut *body }))
    }

    fn shape(&mut self, ptr: usize) -> Option<(&mut World, usize)> {
        let w = *self.shape_world.get(&ptr)?;
        Some((self.worlds.get_mut(&w)?, ptr))
    }
}

// ---- worlds

/// A new world at `ptr` (g_pPhysicsSystem.CreateWorld).
pub fn create_world(ptr: usize) {
    physics().worlds.insert(ptr, World::new());
}

/// A world and everything in it, gone. The handles of its bodies and shapes
/// to free, the world's last.
pub fn destroy_world(ptr: usize) -> Vec<usize> {
    let mut p = physics();
    let Some(w) = p.worlds.remove(&ptr) else { return Vec::new() };
    let mut gone = Vec::new();
    for j in w.joints.into_keys() {
        p.joint_world.remove(&j);
        gone.push(j);
    }
    for (b, body) in w.bodies {
        p.body_world.remove(&b);
        for s in body.shapes {
            p.shape_world.remove(&s);
            gone.push(s);
        }
        gone.push(b);
    }
    gone.push(ptr);
    gone
}

pub fn world_count() -> i32 {
    physics().worlds.len() as i32
}

pub fn set_gravity(world: usize, g: Vector) {
    if let Some(w) = physics().worlds.get_mut(&world) {
        w.sim.gravity = g;
        w.sim.wake_up_all(false);
    }
}

pub fn gravity(world: usize) -> Vector {
    physics().worlds.get(&world).map(|w| w.sim.gravity).unwrap_or(DEFAULT_GRAVITY)
}

pub fn set_simulation(world: usize, mode: i64) {
    if let Some(w) = physics().worlds.get_mut(&world) { w.simulation = mode; }
}

pub fn simulation(world: usize) -> i64 {
    physics().worlds.get(&world).map(|w| w.simulation).unwrap_or(0)
}

pub fn set_sleeping(world: usize, on: bool) {
    let mut p = physics();
    let Some(w) = p.worlds.get_mut(&world) else { return };
    w.sleeping = on;
    let ptrs: Vec<usize> = w.bodies.keys().copied().collect();
    for b in ptrs { apply_sleeping(w, b); }
}

pub fn sleeping(world: usize) -> bool {
    physics().worlds.get(&world).map(|w| w.sleeping).unwrap_or(true)
}

pub fn set_max_speed(world: usize, speed: f32) {
    if let Some(w) = physics().worlds.get_mut(&world) {
        w.max_speed = if speed > 0.0 { speed } else { f32::INFINITY };
    }
}

pub fn set_reference_body(world: usize, body: usize) {
    if let Some(w) = physics().worlds.get_mut(&world) { w.reference = body; }
}

pub fn reference_body(world: usize) -> usize {
    physics().worlds.get(&world).map(|w| w.reference).unwrap_or(0)
}

pub fn set_rules(world: usize, json: &str) {
    if let Some(w) = physics().worlds.get_mut(&world) {
        w.meta.rules = Rules::from_json(json);
        w.sim.wake_up_all(false);
    }
}

/// A body that moved this step, for managed's OnActive.
#[derive(Clone, Copy)]
pub struct Active {
    pub body: usize,
    pub transform: S_Transform,
    pub velocity: Vector,
    pub angular_velocity: Vector,
    pub fell_asleep: bool,
}

/// What a step did: the bodies that moved (and any that fell asleep), and
/// the joints that broke.
#[derive(Default)]
pub struct Stepped {
    pub active: Vec<Active>,
    pub broken: Vec<usize>,
}

/// Steps `world` by `dt` in `steps` equal steps. Forces applied are spent
/// (Box3D's rule).
pub fn step(world: usize, dt: f32, steps: i32) -> Stepped {
    let mut p = physics();
    let Some(w) = p.worlds.get_mut(&world) else { return Stepped::default() };
    if !(dt > 0.0) { return Stepped::default(); }
    let steps = steps.max(1);
    w.sim.integration_parameters.dt = dt / steps as f32;
    w.stale.clear();
    w.tension_springs();
    for _ in 0..steps {
        w.sim.step_with_events(&w.meta, &());
        if w.max_speed.is_finite() {
            for (_, rb) in w.sim.bodies.iter_mut() {
                let v = rb.linvel();
                if v.length() > w.max_speed { rb.set_linvel(v.normalize() * w.max_speed, false); }
            }
        }
    }
    for (_, rb) in w.sim.bodies.iter_mut() {
        if rb.is_dynamic() && (rb.user_force() != Vector::ZERO || rb.user_torque() != Vector::ZERO) {
            rb.reset_forces(false);
            rb.reset_torques(false);
        }
    }
    collect_intersections(w);
    let broken = w.broken_joints(dt / steps as f32);
    let mut active = Vec::new();
    for (&ptr, body) in w.bodies.iter_mut() {
        let Some(rb) = w.sim.bodies.get(body.handle) else { continue };
        if rb.is_fixed() { continue; }
        let asleep = rb.is_sleeping();
        let fell_asleep = asleep && !body.asleep;
        body.asleep = asleep;
        if asleep && !fell_asleep { continue; }
        active.push(Active {
            body: ptr,
            transform: transform(rb.position()),
            velocity: rb.linvel(),
            angular_velocity: rb.angvel(),
            fell_asleep,
        });
    }
    Stepped { active, broken }
}

/// The pair of colliders in a set's order.
fn pair(a: ColliderHandle, b: ColliderHandle) -> (ColliderHandle, ColliderHandle) {
    if a.into_raw_parts() <= b.into_raw_parts() { (a, b) } else { (b, a) }
}

/// One shape of a touch, `triangle` the one it touches at (-1: unknown).
fn side(w: &World, c: ColliderHandle, triangle: i32) -> Option<Side> {
    let s = *w.meta.by_collider.get(&c)?;
    let shape = w.meta.shapes.get(&s)?;
    Some(Side {
        shape: s,
        body: shape.body,
        // every shape is made of something: none named is the default
        surface: shape.surface_at(triangle).max(0),
        shape_handle: crate::handles::of(s),
        body_handle: crate::handles::of(shape.body),
    })
}

/// What began, persisted and ended touching since the last step, for
/// ProcessIntersections: each shape's own mask says whether it hears.
fn collect_intersections(w: &mut World) {
    let mut touching = HashSet::new();
    let mut events = Vec::new();
    for cp in w.sim.narrow_phase.contact_pairs() {
        if !cp.has_any_active_contact() { continue; }
        let key = pair(cp.collider1, cp.collider2);
        touching.insert(key);
        let (Some(a), Some(b)) = (w.meta.shape(cp.collider1), w.meta.shape(cp.collider2)) else { continue };
        let mask = a.mask | b.mask;
        let reason = if !w.touching.contains(&key) { TOUCH_BEGIN } else if mask & TOUCH_PERSISTS != 0 { TOUCH_PERSISTS_EVENT } else { continue };
        if mask & TOUCH_EVENT == 0 { continue; }
        let deepest = cp.find_deepest_contact().map(|(m, _)| (m.subshape1 as i32, m.subshape2 as i32)).unwrap_or((-1, -1));
        let (Some(left), Some(right)) = (side(w, cp.collider1, deepest.0), side(w, cp.collider2, deepest.1)) else { continue };
        let mut e = Intersection { reason, left, right, impulse: cp.total_impulse_magnitude(), ..Default::default() };
        if let (Some((m, c)), Some(co)) = (cp.find_deepest_contact(), w.sim.colliders.get(cp.collider1)) {
            let point = co.position().transform_point(c.local_p1);
            let normal = m.data.normal;
            let v1 = co.parent().and_then(|b| w.sim.bodies.get(b)).map(|b| b.velocity_at_point(point)).unwrap_or(Vector::ZERO);
            let v2 = w.sim.colliders.get(cp.collider2).and_then(|c| c.parent()).and_then(|b| w.sim.bodies.get(b)).map(|b| b.velocity_at_point(point)).unwrap_or(Vector::ZERO);
            let speed = v2 - v1;
            e.point = point.to_array();
            e.normal = normal.to_array();
            e.speed = speed.to_array();
            e.normal_speed = speed.dot(normal);
        }
        events.push(e);
    }
    for &(c1, c2) in w.touching.difference(&touching) {
        let (Some(a), Some(b)) = (w.meta.shape(c1), w.meta.shape(c2)) else { continue };
        if (a.mask | b.mask) & TOUCH_EVENT == 0 { continue; }
        let (Some(left), Some(right)) = (side(w, c1, -1), side(w, c2, -1)) else { continue };
        events.push(Intersection { reason: TOUCH_END, left, right, ..Default::default() });
    }
    let mut triggering = HashSet::new();
    for (c1, c2, on) in w.sim.narrow_phase.intersection_pairs() {
        if !on { continue; }
        let key = pair(c1, c2);
        triggering.insert(key);
        if w.triggering.contains(&key) { continue; }
        let (Some(left), Some(right)) = (side(w, c1, -1), side(w, c2, -1)) else { continue };
        events.push(Intersection { reason: TRIGGER_BEGIN, left, right, ..Default::default() });
    }
    for &(c1, c2) in w.triggering.difference(&triggering) {
        let (Some(left), Some(right)) = (side(w, c1, -1), side(w, c2, -1)) else { continue };
        events.push(Intersection { reason: TRIGGER_END, left, right, ..Default::default() });
    }
    w.touching = touching;
    w.triggering = triggering;
    w.pending.extend(events);
}

/// The intersections gathered since the last time they were asked for.
pub fn take_intersections(world: usize) -> Vec<Intersection> {
    physics().worlds.get_mut(&world).map(|w| std::mem::take(&mut w.pending)).unwrap_or_default()
}

// ---- bodies

/// A new body at `ptr` in `world`, as `s` describes it.
pub fn add_body(world: usize, ptr: usize, s: &BodySettings) {
    let mut p = physics();
    let Some(w) = p.worlds.get_mut(&world) else { return };
    let kind = s.body_type as i64;
    let flags = s.flags;
    let rb = RigidBodyBuilder::new(body_type(kind))
        .pose(Pose::from_parts(vec(s.position), quat(s.rotation)))
        .linvel(vec(s.velocity))
        .angvel(vec(s.angular_velocity))
        .linear_damping(s.linear_damping)
        .angular_damping(s.angular_damping)
        .gravity_scale(if flags & BODY_GRAVITY != 0 { s.gravity_scale } else { 0.0 })
        .ccd_enabled(flags & BODY_BULLET != 0)
        .sleeping(flags & BODY_AWAKE == 0)
        .build();
    let handle = w.sim.insert_body(rb);
    w.by_handle.insert(handle, ptr);
    w.bodies.insert(ptr, Body {
        handle,
        kind,
        shapes: Vec::new(),
        mass: (s.mass > 0.0).then_some(s.mass),
        mass_center: (flags & BODY_OVERRIDE_MASS_CENTER != 0).then(|| vec(s.mass_center)),
        inertia: None,
        inertia_scale: if s.inertia_scale > 0.0 { s.inertia_scale } else { 1.0 },
        deferred: false,
        gravity: flags & BODY_GRAVITY != 0,
        gravity_scale: s.gravity_scale,
        auto_sleep: flags & BODY_AUTO_SLEEP != 0,
        sleep_threshold: s.sleep_threshold,
        touch_events: false,
        asleep: flags & BODY_AWAKE == 0,
    });
    apply_sleeping(w, ptr);
    let locks = s.motion_locks;
    if locks != 0 {
        if let Some(rb) = w.sim.bodies.get_mut(handle) { lock(rb, locks); }
    }
    p.body_world.insert(ptr, world);
}

fn body_type(kind: i64) -> RigidBodyType {
    match kind {
        KEYFRAMED => RigidBodyType::KinematicPositionBased,
        DYNAMIC => RigidBodyType::Dynamic,
        _ => RigidBodyType::Fixed,
    }
}

/// MotionLocks: x, y, z, then pitch (about y), yaw (about z), roll (about x).
fn lock(rb: &mut RigidBody, locks: u32) {
    let free = |bit: u32| locks & bit == 0;
    rb.set_enabled_translations(free(1), free(2), free(4), true);
    rb.set_enabled_rotations(free(32), free(8), free(16), true);
}

/// Whether a body may sleep: its world allows it and so does it.
fn apply_sleeping(w: &mut World, ptr: usize) {
    let Some(body) = w.bodies.get(&ptr) else { return };
    let can = w.sleeping && body.auto_sleep;
    let threshold = body.sleep_threshold;
    let Some(rb) = w.sim.bodies.get_mut(body.handle) else { return };
    let a = rb.activation_mut();
    if can {
        // managed's threshold is a speed in inches; Rapier's is in its length unit
        let d = RigidBodyActivation::default_normalized_linear_threshold();
        a.normalized_linear_threshold = if threshold > 0.0 { threshold / INCHES_PER_METRE } else { d };
        a.angular_threshold = RigidBodyActivation::default_angular_threshold();
    } else {
        a.normalized_linear_threshold = -1.0;
        a.angular_threshold = -1.0;
    }
}

/// A body and its shapes, gone from its world: the pointers whose handles
/// to free, its shapes' then its own.
pub fn remove_body(ptr: usize) -> Vec<usize> {
    let mut p = physics();
    let Some(w) = p.body_world.remove(&ptr) else { return Vec::new() };
    let Some(world) = p.worlds.get_mut(&w) else { return Vec::new() };
    let Some(body) = world.bodies.remove(&ptr) else { return Vec::new() };
    world.by_handle.remove(&body.handle);
    for s in &body.shapes {
        if let Some(shape) = world.meta.shapes.remove(s) { world.meta.by_collider.remove(&shape.collider); }
    }
    // Rapier takes the body's joints with it; their handles go too
    let joints: Vec<usize> = world.joints.iter().filter(|(_, j)| j.body1 == ptr || j.body2 == ptr).map(|(&j, _)| j).collect();
    for j in &joints { world.joints.remove(j); }
    world.sim.remove_body(body.handle);
    if world.reference == ptr { world.reference = 0; }
    for s in &body.shapes { p.shape_world.remove(s); }
    for j in &joints { p.joint_world.remove(j); }
    let mut gone = joints;
    gone.extend(body.shapes);
    gone.push(ptr);
    gone
}

/// Reads a body's simulated state.
pub fn with_body<R>(ptr: usize, f: impl FnOnce(&Body, &RigidBody) -> R) -> Option<R> {
    let mut p = physics();
    let (w, body) = p.body(ptr)?;
    let rb = w.sim.bodies.get(body.handle)?;
    Some(f(body, rb))
}

/// Changes a body's simulated state.
pub fn with_body_mut<R>(ptr: usize, f: impl FnOnce(&mut Body, &mut RigidBody) -> R) -> Option<R> {
    let mut p = physics();
    let (w, body) = p.body(ptr)?;
    let rb = w.sim.bodies.get_mut(body.handle)?;
    Some(f(body, rb))
}

pub fn world_of_body(ptr: usize) -> usize {
    physics().body_world.get(&ptr).copied().unwrap_or(0)
}

pub fn body_kind(ptr: usize) -> i64 {
    with_body(ptr, |b, _| b.kind).unwrap_or(STATIC)
}

pub fn set_body_kind(ptr: usize, kind: i64) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    body.kind = kind;
    let h = body.handle;
    if let Some(rb) = w.sim.bodies.get_mut(h) { rb.set_body_type(body_type(kind), true); }
    rebuild_mass(w, ptr);
}

pub fn set_transform(ptr: usize, position: Option<Vector>, rotation: Option<Rotation>) {
    mark_stale(ptr);
    with_body_mut(ptr, |_, rb| {
        let mut pose = *rb.position();
        if let Some(t) = position { pose.translation = t; }
        if let Some(r) = rotation { pose.rotation = r; }
        rb.set_position(pose, true);
        if rb.is_kinematic() { rb.set_next_kinematic_position(pose); }
    });
}

/// Keyframed: arrive at `target` over the next step. Anything else that
/// moves: the velocity that gets it there in `dt`.
pub fn set_target(ptr: usize, target: Pose, dt: f32) {
    mark_stale(ptr);
    with_body_mut(ptr, |_, rb| {
        if rb.is_kinematic() {
            rb.set_next_kinematic_position(target);
        } else if rb.is_dynamic() && dt > 0.0 {
            let from = *rb.position();
            rb.set_linvel((target.translation - from.translation) / dt, true);
            let (axis, angle) = (target.rotation * from.rotation.inverse()).to_axis_angle();
            let angle = if angle > core::f32::consts::PI { angle - 2.0 * core::f32::consts::PI } else { angle };
            rb.set_angvel(axis * (angle / dt), true);
        } else {
            rb.set_position(target, true);
        }
    });
}

fn mark_stale(ptr: usize) {
    let mut p = physics();
    if let Some((w, b)) = p.body(ptr) { w.stale.insert(b.handle); }
}

pub fn set_gravity_enabled(ptr: usize, on: bool) {
    with_body_mut(ptr, |b, rb| {
        b.gravity = on;
        rb.set_gravity_scale(if on { b.gravity_scale } else { 0.0 }, true);
    });
}

pub fn set_gravity_scale(ptr: usize, scale: f32) {
    with_body_mut(ptr, |b, rb| {
        b.gravity_scale = scale;
        if b.gravity { rb.set_gravity_scale(scale, true); }
    });
}

pub fn set_auto_sleep(ptr: usize, on: bool) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    body.auto_sleep = on;
    apply_sleeping(w, ptr);
}

pub fn set_sleep_threshold(ptr: usize, t: f32) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    body.sleep_threshold = t;
    apply_sleeping(w, ptr);
}

pub fn set_sleeping_body(ptr: usize, asleep: bool) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    let h = body.handle;
    body.asleep = asleep;
    if asleep {
        if let Some(rb) = w.sim.bodies.get_mut(h) { rb.sleep(); }
    } else {
        w.sim.wake_up(h, true);
    }
}

pub fn set_touch_events(ptr: usize, on: bool) {
    with_body_mut(ptr, |b, _| b.touch_events = on);
}

pub fn set_motion_locks(ptr: usize, locks: u32) {
    with_body_mut(ptr, |_, rb| lock(rb, locks));
}

/// Mass, its centre, inertia: what was set, else what the shapes make.
pub fn set_mass(ptr: usize, mass: Option<f32>) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    body.mass = mass;
    rebuild_mass(w, ptr);
}

/// SetLocalMassCenter: the centre to use once overriding is on.
pub fn set_local_mass_center(ptr: usize, center: Vector) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    if body.mass_center.is_some() {
        body.mass_center = Some(center);
        rebuild_mass(w, ptr);
    }
}

pub fn set_override_mass_center(ptr: usize, on: bool) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    let current = w.sim.bodies.get(body.handle).map(|rb| rb.local_center_of_mass()).unwrap_or(Vector::ZERO);
    body.mass_center = if on { Some(body.mass_center.unwrap_or(current)) } else { None };
    rebuild_mass(w, ptr);
}

pub fn set_inertia(ptr: usize, inertia: Option<(Vector, Rotation)>) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    body.inertia = inertia;
    rebuild_mass(w, ptr);
}

pub fn scale_inertia(ptr: usize, scale: f32) {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return };
    body.inertia_scale = if scale > 0.0 { scale } else { 1.0 };
    rebuild_mass(w, ptr);
}

/// Defers mass updates while shapes are added (true) or catches up (false):
/// whether they were deferred before.
pub fn defer_mass(ptr: usize, defer: bool) -> bool {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return false };
    let was = body.deferred;
    body.deferred = defer;
    if was && !defer { rebuild_mass(w, ptr); }
    was
}

pub fn build_mass(ptr: usize) {
    let mut p = physics();
    let Some((w, _)) = p.body(ptr) else { return };
    let was = w.bodies.get(&ptr).map(|b| b.deferred).unwrap_or(false);
    if let Some(b) = w.bodies.get_mut(&ptr) { b.deferred = false; }
    rebuild_mass(w, ptr);
    if let Some(b) = w.bodies.get_mut(&ptr) { b.deferred = was; }
}

/// A body's mass properties from its shapes (Rapier's colliders weigh
/// nothing; the body carries the sum), then what was set over them.
fn rebuild_mass(w: &mut World, ptr: usize) {
    let Some(body) = w.bodies.get(&ptr) else { return };
    if body.deferred { return; }
    let mut props: Option<MassProperties> = None;
    for s in &body.shapes {
        let Some(shape) = w.meta.shapes.get(s) else { continue };
        if shape.no_mass || shape.trigger { continue; }
        let Some(co) = w.sim.colliders.get(shape.collider) else { continue };
        let local = co.position_wrt_parent().copied().unwrap_or(Pose::IDENTITY);
        let density = surface_props(&w.meta.surfaces, shape.surface).density_per_cubic_inch();
        let m = co.shape().mass_properties(density).transform_by(&local);
        props = Some(match props { Some(p) => p + m, None => m });
    }
    let mut props = props.unwrap_or_else(|| MassProperties::new(Vector::ZERO, 0.0, Vector::ZERO));
    if let Some(m) = body.mass {
        if props.mass() > 0.0 {
            props.set_mass(m, true);
        } else {
            // no shapes to weigh: a point mass, a solid inch-ball's inertia
            props = MassProperties::new(props.local_com, m, Vector::splat(0.4 * m));
        }
    }
    if let Some(c) = body.mass_center {
        props.local_com = c;
    }
    if let Some((i, frame)) = body.inertia {
        props = MassProperties::with_principal_inertia_frame(props.local_com, props.mass(), i, frame);
    }
    if body.inertia_scale != 1.0 {
        let frame = props.principal_inertia_local_frame;
        props = MassProperties::with_principal_inertia_frame(props.local_com, props.mass(), props.principal_inertia() * body.inertia_scale, frame);
    }
    if let Some(rb) = w.sim.bodies.get_mut(body.handle) {
        rb.set_additional_mass_properties(props, true);
        rb.recompute_mass_properties_from_colliders(&w.sim.colliders);
    }
}

pub fn principal_inertia(ptr: usize) -> (Vector, Rotation) {
    with_body(ptr, |_, rb| {
        let m = &rb.mass_properties().local_mprops;
        (m.principal_inertia(), m.principal_inertia_local_frame)
    })
    .unwrap_or((Vector::ZERO, Rotation::IDENTITY))
}

/// The world-space box around every shape of a body.
pub fn body_bounds(ptr: usize) -> S_BBox {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return bbox(None) };
    let mut aabb: Option<Aabb> = None;
    for s in &body.shapes {
        let Some(co) = w.meta.shapes.get(s).and_then(|s| w.sim.colliders.get(s.collider)) else { continue };
        let b = co.compute_aabb();
        aabb = Some(match aabb { Some(a) => a.merged(&b), None => b });
    }
    bbox(aabb)
}

fn bbox(a: Option<Aabb>) -> S_BBox {
    match a {
        Some(a) => S_BBox { f_Mins: v3(a.mins), f_Maxs: v3(a.maxs) },
        None => S_BBox { f_Mins: v3(Vector::ZERO), f_Maxs: v3(Vector::ZERO) },
    }
}

/// The point on any of a body's shapes nearest `point`.
pub fn closest_point(ptr: usize, point: Vector) -> Vector {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return point };
    let mut best: Option<(f32, Vector)> = None;
    for s in &body.shapes {
        let Some(co) = w.meta.shapes.get(s).and_then(|s| w.sim.colliders.get(s.collider)) else { continue };
        let proj = co.shape().project_point(co.position(), point, true);
        let d = (proj.point - point).length_squared();
        if best.is_none_or(|(bd, _)| d < bd) { best = Some((d, proj.point)); }
    }
    best.map(|(_, p)| p).unwrap_or(point)
}

/// The shapes of `other`, and of `ptr` placed at `at`: whether they overlap,
/// and the way and depth to push `ptr` out (ComputePenetration's answer).
pub fn penetration(ptr: usize, other: usize, at: Pose) -> Option<(Vector, f32)> {
    let mut p = physics();
    let (w, body) = p.body(ptr)?;
    let current = *w.sim.bodies.get(body.handle)?.position();
    let shapes = body.shapes.clone();
    let others = w.bodies.get(&other)?.shapes.clone();
    let dispatcher = w.sim.narrow_phase.query_dispatcher();
    let mut deepest: Option<(Vector, f32)> = None;
    for s in &shapes {
        let Some(a) = w.meta.shapes.get(s).and_then(|s| w.sim.colliders.get(s.collider)) else { continue };
        let pa = at * current.inv_mul(a.position());
        for o in &others {
            let Some(b) = w.meta.shapes.get(o).and_then(|s| w.sim.colliders.get(s.collider)) else { continue };
            let pos12 = pa.inv_mul(b.position());
            let Ok(Some(c)) = dispatcher.contact(&pos12, a.shape(), b.shape(), 0.0) else { continue };
            if c.dist > 0.0 { continue; }
            let normal = pa.rotation * c.normal1;
            if deepest.is_none_or(|(_, d)| -c.dist > d) { deepest = Some((-normal, -c.dist)); }
        }
    }
    deepest
}

/// Whether a body touches another (its shapes in contact, or overlapping
/// when triggers count).
pub fn touching(ptr: usize, other_shapes: impl Fn(&World) -> Vec<usize>, triggers: bool) -> bool {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return false };
    let mine: Vec<ColliderHandle> = body.shapes.iter().filter_map(|s| w.meta.shapes.get(s)).map(|s| s.collider).collect();
    let theirs: Vec<ColliderHandle> = other_shapes(w).iter().filter_map(|s| w.meta.shapes.get(s)).map(|s| s.collider).collect();
    mine.iter().any(|&a| {
        theirs.iter().any(|&b| {
            w.touching.contains(&pair(a, b)) || (triggers && w.triggering.contains(&pair(a, b)))
        })
    })
}

pub fn shapes_of_body(ptr: usize) -> Vec<usize> {
    let mut p = physics();
    p.body(ptr).map(|(_, b)| b.shapes.clone()).unwrap_or_default()
}

pub fn touching_shape(ptr: usize, shape: usize, triggers: bool) -> bool {
    touching(ptr, |_| vec![shape], triggers)
}

pub fn touching_body(ptr: usize, other: usize, triggers: bool) -> bool {
    touching(ptr, |w| w.bodies.get(&other).map(|b| b.shapes.clone()).unwrap_or_default(), triggers)
}

pub fn shape_touching(shape: usize, other: usize, triggers: bool) -> bool {
    let mut p = physics();
    let Some((w, _)) = p.shape(shape) else { return false };
    let (Some(a), Some(b)) = (w.meta.shapes.get(&shape), w.meta.shapes.get(&other)) else { return false };
    let k = pair(a.collider, b.collider);
    w.touching.contains(&k) || (triggers && w.triggering.contains(&k))
}

// ---- shapes

/// What a shape is made of.
pub enum Geometry {
    Sphere(Vector, f32),
    Capsule(Vector, Vector, f32),
    Box(Pose, Vector),
    Hull(Pose, Vec<Vector>),
    Mesh(Pose, Vec<Vector>, Vec<[u32; 3]>),
    /// heights (row-major, `x` by `y` cells' corners), spacing, height scale
    Height(Vec<u16>, usize, usize, f32, f32),
}

impl Geometry {
    fn kind(&self) -> i64 {
        match self {
            Geometry::Sphere(..) => SHAPE_SPHERE,
            Geometry::Capsule(..) => SHAPE_CAPSULE,
            Geometry::Box(..) | Geometry::Hull(..) => SHAPE_HULL,
            Geometry::Mesh(..) => SHAPE_MESH,
            Geometry::Height(..) => SHAPE_HEIGHTFIELD,
        }
    }

    /// The shape, and where it sits on its body.
    fn build(&self) -> Option<(SharedShape, Pose)> {
        Some(match self {
            Geometry::Sphere(c, r) => (SharedShape::ball(r.max(0.0)), Pose::from_translation(*c)),
            Geometry::Capsule(a, b, r) => (SharedShape::capsule(*a, *b, r.max(0.0)), Pose::IDENTITY),
            Geometry::Box(at, half) => {
                let h = half.abs().max(Vector::splat(1e-3));
                (SharedShape::cuboid(h.x, h.y, h.z), *at)
            }
            Geometry::Hull(at, points) => (SharedShape::convex_hull(points)?, *at),
            Geometry::Mesh(at, vertices, triangles) => {
                if triangles.is_empty() { return None; }
                (SharedShape::trimesh_with_flags(vertices.clone(), triangles.clone(), TriMeshFlags::FIX_INTERNAL_EDGES).ok()?, *at)
            }
            Geometry::Height(heights, x, y, spacing, scale) => {
                let (nx, ny) = (*x, *y);
                if nx < 2 || ny < 2 || heights.len() < nx * ny { return None; }
                // parry's field lies on x-z, its rows along z and its heights
                // along y, centred; s&box's on x-y from the body's origin,
                // heights along z. Turned a quarter about x (z becomes -y, so
                // the rows go in backwards) and moved to its corner.
                let mut m = rapier3d::parry::utils::Array2::zeros(ny, nx);
                for j in 0..ny {
                    for i in 0..nx {
                        m[(ny - 1 - j, i)] = heights[j * nx + i] as f32 * scale;
                    }
                }
                let (w, d) = ((nx - 1) as f32 * spacing, (ny - 1) as f32 * spacing);
                let field = HeightField::new(m, Vector::new(w, 1.0, d));
                let turn = Rotation::from_rotation_x(core::f32::consts::FRAC_PI_2);
                (SharedShape::new(field), Pose::from_parts(Vector::new(w / 2.0, d / 2.0, 0.0), turn))
            }
        })
    }

    fn ends(&self) -> (Vector, Vector, f32) {
        match self {
            Geometry::Sphere(c, r) => (*c, *c, *r),
            Geometry::Capsule(a, b, r) => (*a, *b, *r),
            _ => (Vector::ZERO, Vector::ZERO, 0.0),
        }
    }
}

/// Each triangle's material by parry's triangle id: a mesh's are its
/// triangles' own, in order; a height field's are its samples' (`x` by `y`,
/// row-major), each cell taking its first corner's.
fn triangle_materials(g: &Geometry, materials: &[u8]) -> Vec<u8> {
    match g {
        Geometry::Mesh(_, _, triangles) if materials.len() >= triangles.len() && !triangles.is_empty() => materials[..triangles.len()].to_vec(),
        &Geometry::Height(_, nx, ny, ..) if nx >= 2 && ny >= 2 && materials.len() >= nx * ny => {
            // build()'s field: its rows run backwards along s&box's y, its
            // columns along x; a cell's two triangles are `id` and `id + half`
            let (rows, cols) = (ny - 1, nx - 1);
            let half = rows * cols;
            let mut out = vec![0u8; half * 2];
            for c in 0..cols {
                for r in 0..rows {
                    let (x, y) = (c, ny - 2 - r);
                    let m = materials[y * nx + x];
                    let id = c * rows + r;
                    out[id] = m;
                    out[id + half] = m;
                }
            }
            out
        }
        _ => Vec::new(),
    }
}

/// A new shape at `ptr` on `body`: false if the body is unknown or the
/// shape can't be made (a hull of too few points). `materials` are its
/// triangles' (a mesh's) or samples' (a height field's), naming the
/// surfaces its settings list.
pub fn add_shape(body: usize, ptr: usize, g: &Geometry, s: &ShapeSettings, materials: &[u8]) -> bool {
    let table = surfaces::table();
    let mut p = physics();
    let Some(w) = p.body_world.get(&body).copied() else { return false };
    let Some(world) = p.worlds.get_mut(&w) else { return false };
    let Some(b) = world.bodies.get(&body) else { return false };
    let bh = b.handle;
    let Some((shape, at)) = g.build() else { return false };
    let flags = s.flags;
    let trigger = flags & SHAPE_SENSOR != 0;
    let mut co = ColliderBuilder::new(shape)
        .position(at)
        .density(0.0)
        .sensor(trigger)
        .active_collision_types(if trigger { ActiveCollisionTypes::all() } else { ActiveCollisionTypes::default() })
        .build();
    let mut tags: Vec<u32> = if flags & SHAPE_TAGS != 0 { s.tags.iter().copied().filter(|&t| t != 0).collect() } else { Vec::new() };
    if flags & SHAPE_DEFAULT_SOLID_TAG != 0 && tags.is_empty() { tags.push(murmur2("solid")); }
    let parts = if s.surface_indices != 0 && s.surface_count > 0 {
        unsafe { core::slice::from_raw_parts(s.surface_indices as *const i32, s.surface_count as usize) }.to_vec()
    } else {
        Vec::new()
    };
    let own = |v: f32| (v >= 0.0).then_some(v);
    let shape = Shape {
        body,
        collider: ColliderHandle::invalid(),
        kind: g.kind(),
        tags,
        mask: s.collision_function_mask as u8,
        trigger,
        ignore_traces: flags & SHAPE_IGNORE_TRACES != 0,
        no_mass: flags & SHAPE_NO_MASS != 0,
        surface: s.surface_index,
        parts,
        materials: triangle_materials(g, materials),
        own_friction: own(s.friction),
        own_elasticity: own(s.elasticity),
        own_rolling_resistance: own(s.rolling_resistance),
        local_velocity: vec(s.surface_velocity),
        ends: g.ends(),
    };
    world.meta.surfaces = table;
    shape.apply(&world.meta.surfaces, &mut co);
    let collider = world.sim.insert_collider(co, Some(bh));
    world.meta.by_collider.insert(collider, ptr);
    world.meta.shapes.insert(ptr, Shape { collider, ..shape });
    if let Some(b) = world.bodies.get_mut(&body) { b.shapes.push(ptr); }
    world.stale.insert(bh);
    rebuild_mass(world, body);
    p.shape_world.insert(ptr, w);
    true
}

/// A shape made anew from `g`, kept where it is on its body.
pub fn update_shape(ptr: usize, g: &Geometry) {
    let mut p = physics();
    let Some((w, _)) = p.shape(ptr) else { return };
    let Some((shape, at)) = g.build() else { return };
    let Some(s) = w.meta.shapes.get_mut(&ptr) else { return };
    s.kind = g.kind();
    s.ends = g.ends();
    // new triangles: the old ones' materials don't fit them
    s.materials.clear();
    let (c, body) = (s.collider, s.body);
    if let Some(co) = w.sim.colliders.get_mut(c) {
        co.set_shape(shape);
        co.set_position_wrt_parent(at);
    }
    if let Some(h) = w.bodies.get(&body).map(|b| b.handle) { w.stale.insert(h); }
    rebuild_mass(w, body);
}

/// A shape gone from its body: whether it was there.
pub fn remove_shape(ptr: usize) -> bool {
    let mut p = physics();
    let Some(w) = p.shape_world.remove(&ptr) else { return false };
    let Some(world) = p.worlds.get_mut(&w) else { return false };
    let Some(s) = world.meta.shapes.remove(&ptr) else { return false };
    world.meta.by_collider.remove(&s.collider);
    world.sim.remove_collider(s.collider);
    world.touching.retain(|&(a, b)| a != s.collider && b != s.collider);
    world.triggering.retain(|&(a, b)| a != s.collider && b != s.collider);
    if let Some(b) = world.bodies.get_mut(&s.body) { b.shapes.retain(|&x| x != ptr); }
    rebuild_mass(world, s.body);
    true
}

/// Reads a shape.
pub fn with_shape<R>(ptr: usize, f: impl FnOnce(&Shape, &Collider) -> R) -> Option<R> {
    let mut p = physics();
    let (w, _) = p.shape(ptr)?;
    let s = w.meta.shapes.get(&ptr)?;
    let co = w.sim.colliders.get(s.collider)?;
    Some(f(s, co))
}

/// Changes a shape; `rebuild` if its body's mass may change with it.
pub fn with_shape_mut<R>(ptr: usize, rebuild: bool, f: impl FnOnce(&mut Shape, &mut Collider) -> R) -> Option<R> {
    let mut p = physics();
    let (w, _) = p.shape(ptr)?;
    let s = w.meta.shapes.get_mut(&ptr)?;
    let co = w.sim.colliders.get_mut(s.collider)?;
    let r = f(s, co);
    if rebuild {
        let body = w.meta.shapes.get(&ptr)?.body;
        rebuild_mass(w, body);
    }
    Some(r)
}

pub fn shape_kind(ptr: usize) -> i64 {
    with_shape(ptr, |s, _| s.kind).unwrap_or(SHAPE_SPHERE)
}

pub fn shape_tags(ptr: usize) -> Vec<u32> {
    with_shape(ptr, |s, _| s.tags.clone()).unwrap_or_default()
}

/// Changes a shape's tags: whether they changed. Contacts are found again
/// under the new tags.
pub fn edit_tags(ptr: usize, f: impl FnOnce(&mut Vec<u32>) -> bool) -> bool {
    let mut p = physics();
    let Some((w, _)) = p.shape(ptr) else { return false };
    let Some(s) = w.meta.shapes.get_mut(&ptr) else { return false };
    let changed = f(&mut s.tags);
    if changed {
        let body = s.body;
        if let Some(h) = w.bodies.get(&body).map(|b| b.handle) { w.sim.wake_up(h, true); }
    }
    changed
}

pub fn set_mask(ptr: usize, f: impl FnOnce(u8) -> u8) {
    with_shape_mut(ptr, false, |s, _| s.mask = f(s.mask));
}

pub fn set_trigger(ptr: usize, on: bool) {
    with_shape_mut(ptr, true, |s, co| {
        s.trigger = on;
        co.set_sensor(on);
        co.set_active_collision_types(if on { ActiveCollisionTypes::all() } else { ActiveCollisionTypes::default() });
    });
}

pub fn set_friction(ptr: usize, f: f32) {
    with_shape_mut(ptr, false, |s, co| {
        s.own_friction = Some(f);
        co.set_friction(f);
    });
}

pub fn set_elasticity(ptr: usize, e: f32) {
    with_shape_mut(ptr, false, |s, co| {
        s.own_elasticity = Some(e);
        co.set_restitution(e.max(0.0));
    });
}

pub fn set_rolling_resistance(ptr: usize, r: f32) {
    with_shape_mut(ptr, false, |s, _| s.own_rolling_resistance = Some(r));
}

/// A shape's friction: its own, or its surface's.
pub fn friction(ptr: usize) -> f32 {
    let mut p = physics();
    let Some((w, _)) = p.shape(ptr) else { return 0.0 };
    w.meta.shapes.get(&ptr).map(|s| s.friction(&w.meta.surfaces, -1)).unwrap_or(0.0)
}

/// A shape is made of surface `index` now (-1: the default): `part` 0 of a
/// shape whose triangles have no materials, otherwise the surface its
/// triangles of material `part` are.
pub fn set_surface(ptr: usize, part: i32, index: i32) {
    let table = surfaces::table();
    let mut p = physics();
    let Some((w, _)) = p.shape(ptr) else { return };
    w.meta.surfaces = table;
    let Some(s) = w.meta.shapes.get_mut(&ptr) else { return };
    if let Ok(k) = usize::try_from(part) {
        if s.parts.len() <= k { s.parts.resize(k + 1, -1); }
        s.parts[k] = index;
    }
    if part < 0 || (part == 0 && s.materials.is_empty()) { s.surface = index; }
    let body = s.body;
    if let Some(co) = w.sim.colliders.get_mut(s.collider) { s.apply(&w.meta.surfaces, co); }
    rebuild_mass(w, body);
}

/// The surface each of a body's shapes is made of now.
pub fn set_body_surface(body: usize, index: i32) {
    for s in shapes_of_body(body) { set_surface(s, -1, index); }
}

/// A shape's surface (-1: none named).
pub fn surface(ptr: usize) -> i32 {
    with_shape(ptr, |s, _| s.surface).unwrap_or(-1)
}

/// A body's mass over its volume: kilograms per cubic metre.
pub fn body_density(ptr: usize) -> f32 {
    let mut p = physics();
    let Some((w, body)) = p.body(ptr) else { return 0.0 };
    let (mut mass, mut volume) = (0.0, 0.0);
    for s in &body.shapes {
        let Some(shape) = w.meta.shapes.get(s) else { continue };
        if shape.no_mass || shape.trigger { continue; }
        let Some(co) = w.sim.colliders.get(shape.collider) else { continue };
        let v = co.shape().mass_properties(1.0).mass();
        volume += v;
        mass += v * surface_props(&w.meta.surfaces, shape.surface).density_per_cubic_inch();
    }
    if volume > 0.0 { mass / volume * surfaces::CUBIC_INCHES_PER_CUBIC_METRE } else { 0.0 }
}

/// A surface changed (or was added): every shape takes its friction and
/// elasticity again, and bodies their mass.
pub fn surfaces_changed() {
    let table = surfaces::table();
    let mut p = physics();
    for w in p.worlds.values_mut() {
        w.meta.surfaces = table.clone();
        let mut bodies = HashSet::new();
        for s in w.meta.shapes.values() {
            if let Some(co) = w.sim.colliders.get_mut(s.collider) { s.apply(&w.meta.surfaces, co); }
            bodies.insert(s.body);
        }
        for b in bodies { rebuild_mass(w, b); }
    }
}

pub fn shape_bounds(ptr: usize, local: bool) -> S_BBox {
    bbox(with_shape(ptr, |_, co| {
        if local { co.shape().compute_aabb(co.position_wrt_parent().unwrap_or(&Pose::IDENTITY)) } else { co.compute_aabb() }
    }))
}

pub fn as_sphere(ptr: usize) -> S_Sandbox_Sphere {
    let (c, _, r) = with_shape(ptr, |s, _| s.ends).unwrap_or((Vector::ZERO, Vector::ZERO, 0.0));
    S_Sandbox_Sphere { f_Center: v3(c), f_Radius: r }
}

pub fn as_capsule(ptr: usize) -> S_Capsule {
    let (a, b, r) = with_shape(ptr, |s, _| s.ends).unwrap_or((Vector::ZERO, Vector::ZERO, 0.0));
    S_Capsule { f_CenterA: v3(a), f_CenterB: v3(b), f_Radius: r }
}

/// A shape's triangles in its body's space: its vertices and indices.
pub fn triangulation(ptr: usize) -> (Vec<Vector>, Vec<u32>) {
    with_shape(ptr, |_, co| {
        let at = co.position_wrt_parent().copied().unwrap_or(Pose::IDENTITY);
        let (v, i) = match co.shape().as_typed_shape() {
            TypedShape::TriMesh(m) => (m.vertices().to_vec(), m.indices().to_vec()),
            TypedShape::HeightField(h) => h.to_trimesh(),
            TypedShape::Ball(b) => b.to_trimesh(12, 6),
            TypedShape::Capsule(c) => c.to_trimesh(12, 6),
            TypedShape::Cuboid(c) => c.to_trimesh(),
            TypedShape::ConvexPolyhedron(c) => c.to_trimesh(),
            _ => (Vec::new(), Vec::new()),
        };
        (v.into_iter().map(|p| at.transform_point(p)).collect(), i.into_iter().flatten().collect())
    })
    .unwrap_or_default()
}

// ---- queries

/// The shapes of `world` touching a sphere, a box or the hull of some
/// points; `mask` drops static (1), dynamic (2) or keyframed (4) ones.
pub fn query(world: usize, shape: SharedShape, at: Pose, mask: u16) -> Vec<usize> {
    let mut p = physics();
    let Some(w) = p.worlds.get_mut(&world) else { return Vec::new() };
    w.refresh();
    let filter = QueryFilter { flags: set_flags(mask), ..Default::default() };
    w.sim
        .intersect_shape(at, shape.as_ref(), filter)
        .filter_map(|(c, _)| w.meta.by_collider.get(&c).copied())
        .collect()
}

fn set_flags(mask: u16) -> QueryFilterFlags {
    let mut f = QueryFilterFlags::empty();
    if mask & 1 != 0 { f |= QueryFilterFlags::EXCLUDE_FIXED; }
    if mask & 2 != 0 { f |= QueryFilterFlags::EXCLUDE_DYNAMIC; }
    if mask & 4 != 0 { f |= QueryFilterFlags::EXCLUDE_KINEMATIC; }
    f
}

/// The shape a trace sweeps, placed at its start (None: a ray).
fn trace_shape(t: &TraceShape) -> Option<(SharedShape, Pose)> {
    let turn = quat(t.rotation);
    let (mins, maxs) = (vec(t.mins), vec(t.maxs));
    match t.kind {
        0 if t.radius[0] > 0.0 => Some((SharedShape::ball(t.radius[0]), Pose::from_rotation(turn))),
        0 => None,
        1 => {
            let half = ((maxs - mins) * 0.5).abs();
            if half.max_element() <= 0.0 { return None; }
            let h = half.max(Vector::splat(1e-3));
            // the box is placed by its centre, which a lopsided one moves
            Some((SharedShape::cuboid(h.x, h.y, h.z), Pose::from_parts(turn * ((mins + maxs) * 0.5), turn)))
        }
        2 => Some((SharedShape::capsule(mins, maxs, t.radius[0].max(0.0)), Pose::from_rotation(turn))),
        _ => {
            // a cylinder along z from mins to maxs, a cone if the top's radius differs
            let (bottom, top) = (t.radius[0].max(0.0), t.radius[1]);
            let half = ((maxs - mins).length() * 0.5).max(1e-3);
            let up = Rotation::from_rotation_arc(Vector::Y, (maxs - mins).try_normalize().unwrap_or(Vector::Z));
            let centre = (mins + maxs) * 0.5;
            let s = if (top - bottom).abs() < 1e-4 {
                SharedShape::cylinder(half, bottom)
            } else if top <= 0.0 {
                SharedShape::cone(half, bottom)
            } else {
                let mut points = Vec::new();
                for i in 0..16 {
                    let a = i as f32 * core::f32::consts::TAU / 16.0;
                    let (s, c) = a.sin_cos();
                    points.push(Vector::new(c * bottom, -half, s * bottom));
                    points.push(Vector::new(c * top, half, s * top));
                }
                SharedShape::convex_hull(&points)?
            };
            Some((s, Pose::from_parts(turn * centre, turn * up)))
        }
    }
}

/// Whether a shape's tags pass a trace's: all of `require`, one of `any`
/// (if it names any), none of `exclude`.
fn tags_pass(tags: &[u32], r: &TraceRequest) -> bool {
    let (require, any, exclude) = (r.tag_require, r.tag_any, r.tag_exclude);
    require.iter().filter(|&&t| t != 0).all(|t| tags.contains(t))
        && (any.iter().all(|&t| t == 0) || any.iter().any(|t| *t != 0 && tags.contains(t)))
        && !exclude.iter().any(|t| *t != 0 && tags.contains(t))
}

fn empty_result(r: &TraceRequest) -> S_PhysicsTrace_Result {
    S_PhysicsTrace_Result {
        f_StartPos: r.start,
        f_EndPos: r.end,
        f_HitPos: r.end,
        f_Normal: v3(Vector::ZERO),
        f_Fraction: 1.0,
        f_StartedInSolid: 0,
        f_PhysicsBodyHandle: 0,
        f_PhysicsShapeHandle: 0,
        f_SurfaceProperty: -1,
        f_TriangleIndex: -1,
        f_Tags: [0; 16],
    }
}

/// A hit of `shape` at `fraction` of the way, at `point`, facing `normal`.
fn hit_result(w: &World, r: &TraceRequest, shape: usize, fraction: f32, point: Vector, normal: Vector, triangle: i32) -> S_PhysicsTrace_Result {
    let (start, end) = (vec(r.start), vec(r.end));
    let mut out = empty_result(r);
    let fraction = fraction.clamp(0.0, 1.0);
    out.f_Fraction = fraction;
    out.f_EndPos = v3(start + (end - start) * fraction);
    out.f_HitPos = v3(point);
    out.f_Normal = v3(normal);
    out.f_TriangleIndex = triangle;
    if let Some(s) = w.meta.shapes.get(&shape) {
        out.f_PhysicsShapeHandle = crate::handles::of(shape);
        out.f_PhysicsBodyHandle = crate::handles::of(s.body);
        out.f_SurfaceProperty = s.surface_at(triangle).max(0);
        for (i, t) in s.tags.iter().take(16).enumerate() { out.f_Tags[i] = *t; }
    }
    out
}

/// The triangle a ray met: a mesh's or height field's own (the hit's
/// sub-shape; its feature is then the triangle's own face), else the face's.
fn triangle_of(w: &World, c: ColliderHandle, hit: &RayIntersection) -> i32 {
    let composite = w.sim.colliders.get(c).is_some_and(|co| matches!(co.shape().shape_type(), ShapeType::TriMesh | ShapeType::HeightField));
    match hit.feature {
        _ if composite => hit.subshape as i32,
        FeatureId::Face(i) => i as i32,
        _ => -1,
    }
}

/// The trace's own body's shapes, swept as one, in that body's space.
fn body_sweep(w: &World, body: usize) -> Option<SharedShape> {
    let b = w.bodies.get(&body)?;
    let parts: Vec<(Pose, SharedShape)> = b
        .shapes
        .iter()
        .filter_map(|s| w.meta.shapes.get(s))
        .filter(|s| !s.trigger)
        .filter_map(|s| w.sim.colliders.get(s.collider))
        .map(|co| (co.position_wrt_parent().copied().unwrap_or(Pose::IDENTITY), co.shared_shape().clone()))
        .collect();
    match parts.len() {
        0 => None,
        1 if parts[0].0 == Pose::IDENTITY => Some(parts[0].1.clone()),
        _ => Some(SharedShape::compound(parts)),
    }
}

/// Sweeps a trace through its world: every hit, nearest first (`all`), or
/// only the nearest. A filter function, if the trace has one, is managed's
/// and is called after the lock is let go.
pub fn trace(r: &TraceRequest, all: bool) -> Vec<S_PhysicsTrace_Result> {
    let candidates = {
        let mut p = physics();
        let Some(w) = p.worlds.get_mut(&r.world) else { return if all { Vec::new() } else { vec![empty_result(r)] } };
        sweep(w, r, all || r.filter != 0)
    };
    let mut hits = Vec::new();
    for (shape_handle, res) in candidates {
        if r.filter != 0 && shape_handle != 0 {
            let f: extern "C" fn(i32) -> u8 = unsafe { std::mem::transmute(r.filter) };
            if f(shape_handle) == 0 { continue; }
        }
        hits.push(res);
        if !all { break; }
    }
    if !all && hits.is_empty() { hits.push(empty_result(r)); }
    hits
}

/// Every shape the trace may hit along its way, nearest first (or only the
/// nearest unless `many`), with the managed handle of the shape for the filter.
fn sweep(w: &mut World, r: &TraceRequest, many: bool) -> Vec<(i32, S_PhysicsTrace_Result)> {
    w.refresh();
    let (start, end) = (vec(r.start), vec(r.end));
    let delta = end - start;
    let own = w.bodies.get(&r.body).map(|b| b.handle);
    let triggers = r.trigger_filter;
    let shapes = &w.meta;
    let pass = |c: ColliderHandle, co: &Collider| -> bool {
        let Some(s) = shapes.shape(c) else { return false };
        if s.ignore_traces { return false; }
        let trigger = co.is_sensor();
        match triggers {
            0 if trigger => return false,
            2 if !trigger => return false,
            _ => {}
        }
        tags_pass(&s.tags, r)
    };
    let filter = QueryFilter { flags: set_flags(r.object_set_mask), exclude_rigid_body: own, predicate: Some(&pass), ..Default::default() };
    let swept: Option<(SharedShape, Pose)> = if r.body != 0 {
        let from = Pose::from_parts(start, quat(r.shape.rotation));
        body_sweep(w, r.body).map(|s| (s, from))
    } else {
        trace_shape(&r.shape).map(|(s, at)| (s, Pose::from_parts(start + at.translation, at.rotation)))
    };
    let mut out: Vec<(f32, ColliderHandle, f32, Vector, Vector, i32)> = Vec::new();
    match swept {
        None => {
            let len = delta.length();
            let ray = Ray::new(start, if len > 0.0 { delta / len } else { Vector::Z });
            let max = len.max(0.0);
            if many {
                for (c, _, hit) in w.sim.intersect_ray(ray, max, true, filter) {
                    let point = ray.point_at(hit.time_of_impact);
                    out.push((hit.time_of_impact, c, if len > 0.0 { hit.time_of_impact / len } else { 0.0 }, point, hit.normal, triangle_of(w, c, &hit)));
                }
            } else if let Some((c, hit)) = w.sim.cast_ray_and_get_normal(&ray, max, true, filter) {
                let point = ray.point_at(hit.time_of_impact);
                out.push((hit.time_of_impact, c, if len > 0.0 { hit.time_of_impact / len } else { 0.0 }, point, hit.normal, triangle_of(w, c, &hit)));
            }
        }
        Some((shape, from)) => {
            let options = ShapeCastOptions { max_time_of_impact: 1.0, target_distance: 0.0, stop_at_penetration: true, compute_impact_geometry_on_penetration: true };
            if many {
                // every shape it would meet, each cast alone
                let sweep_aabb = shape.compute_aabb(&from).merged(&shape.compute_aabb(&Pose::from_parts(from.translation + delta, from.rotation)));
                let found: Vec<ColliderHandle> = w.sim.intersect_aabb_conservative(sweep_aabb, filter).map(|(c, _)| c).collect();
                let dispatcher = w.sim.narrow_phase.query_dispatcher();
                for c in found {
                    let Some(co) = w.sim.colliders.get(c) else { continue };
                    let pos12 = co.position().inv_mul(&from);
                    let vel12 = co.position().rotation.inverse() * delta;
                    let Ok(Some(hit)) = dispatcher.cast_shapes(&pos12, vel12, co.shape(), shape.as_ref(), options) else { continue };
                    let point = co.position().transform_point(hit.witness1);
                    let normal = co.position().rotation * hit.normal1;
                    out.push((hit.time_of_impact, c, hit.time_of_impact, point, normal, -1));
                }
            } else if let Some((c, hit)) = w.sim.cast_shape(&from, delta, shape.as_ref(), options, filter) {
                out.push((hit.time_of_impact, c, hit.time_of_impact, hit.witness1, hit.normal1, -1));
            }
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    let start_solid = |frac: f32, normal: Vector| frac <= 0.0 && normal == Vector::ZERO;
    out.into_iter()
        .filter_map(|(_, c, frac, point, normal, tri)| {
            let shape = *w.meta.by_collider.get(&c)?;
            let mut res = hit_result(w, r, shape, frac, point, normal, tri);
            if frac <= 0.0 && (start_solid(frac, normal) || swept_starts_inside(w, r, c)) {
                res.f_StartedInSolid = 1;
            }
            Some((crate::handles::of(shape), res))
        })
        .collect()
}

/// Whether a trace begins inside the collider `c`.
fn swept_starts_inside(w: &World, r: &TraceRequest, c: ColliderHandle) -> bool {
    let Some(co) = w.sim.colliders.get(c) else { return false };
    let start = vec(r.start);
    if r.body == 0 && trace_shape(&r.shape).is_none() {
        return co.shape().contains_point(co.position(), start);
    }
    true
}

/// A trace against one shape placed at `at`, not a world (TraceAgainst…).
pub fn trace_against(r: &TraceRequest, target: SharedShape, at: Pose) -> S_PhysicsTrace_Result {
    let (start, end) = (vec(r.start), vec(r.end));
    let delta = end - start;
    let mut out = empty_result(r);
    let hit = match trace_shape(&r.shape) {
        None => {
            let len = delta.length();
            let ray = Ray::new(start, if len > 0.0 { delta / len } else { Vector::Z });
            target.cast_ray_and_get_normal(&at, &ray, len, true).map(|h| {
                let inside = h.time_of_impact <= 0.0 && target.contains_point(&at, start);
                (if len > 0.0 { h.time_of_impact / len } else { 0.0 }, ray.point_at(h.time_of_impact), h.normal, inside)
            })
        }
        Some((shape, local)) => {
            let from = Pose::from_parts(start + local.translation, local.rotation);
            let options = ShapeCastOptions { max_time_of_impact: 1.0, target_distance: 0.0, stop_at_penetration: true, compute_impact_geometry_on_penetration: true };
            let pos12 = at.inv_mul(&from);
            let vel12 = at.rotation.inverse() * delta;
            rapier3d::parry::query::cast_shapes(&Pose::IDENTITY, Vector::ZERO, target.as_ref(), &pos12, vel12, shape.as_ref(), options)
                .ok()
                .flatten()
                .map(|h| (h.time_of_impact, at.transform_point(h.witness1), at.rotation * h.normal1, h.time_of_impact <= 0.0))
        }
    };
    if let Some((frac, point, normal, inside)) = hit {
        let frac = frac.clamp(0.0, 1.0);
        out.f_Fraction = frac;
        out.f_EndPos = v3(start + delta * frac);
        out.f_HitPos = v3(point);
        out.f_Normal = v3(normal);
        out.f_StartedInSolid = inside as u8;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    static SERIAL: Mutex<()> = Mutex::new(());

    fn ptr() -> usize {
        Box::into_raw(Box::new([0u64; 4])) as usize
    }

    fn ground_and_ball() -> (usize, usize, usize) {
        let w = ptr();
        create_world(w);
        let ground = ptr();
        add_body(w, ground, &BodySettings::default());
        assert!(add_shape(ground, ptr(), &Geometry::Box(Pose::IDENTITY, Vector::new(500.0, 500.0, 10.0)), &ShapeSettings::default(), &[]));
        let ball = ptr();
        let mut s = BodySettings { body_type: DYNAMIC as i32, ..Default::default() };
        s.position = v3(Vector::new(0.0, 0.0, 200.0));
        add_body(w, ball, &s);
        assert!(add_shape(ball, ptr(), &Geometry::Sphere(Vector::ZERO, 8.0), &ShapeSettings::default(), &[]));
        (w, ground, ball)
    }

    #[test]
    fn a_ball_falls_and_comes_to_rest_on_the_ground() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (w, _, ball) = ground_and_ball();
        let mut moved = 0;
        let mut slept = false;
        for _ in 0..600 {
            let active = step(w, 1.0 / 60.0, 2).active;
            moved += active.iter().filter(|a| a.body == ball).count();
            slept |= active.iter().any(|a| a.body == ball && a.fell_asleep);
        }
        let z = with_body(ball, |_, rb| rb.translation().z).unwrap();
        assert!((z - 18.0).abs() < 1.0, "resting on the ground's top (10) by its radius (8): {z}");
        assert!(moved > 10, "managed heard it move");
        assert!(slept, "and fall asleep");
        // a sphere of radius 8 in at the default surface's density
        let m = with_body(ball, |_, rb| rb.mass()).unwrap();
        let expected = 4.0 / 3.0 * core::f32::consts::PI * 512.0 * surfaces::DEFAULT.density_per_cubic_inch();
        assert!((m - expected).abs() < 1e-3 * expected, "{m} vs {expected}");
    }

    #[test]
    fn mass_set_by_managed_wins_over_the_shapes() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (_, _, ball) = ground_and_ball();
        set_mass(ball, Some(500.0));
        assert!((with_body(ball, |_, rb| rb.mass()).unwrap() - 500.0).abs() < 1e-3);
        set_mass(ball, None);
        assert!(with_body(ball, |_, rb| rb.mass()).unwrap() < 500.0);
    }

    fn ray(world: usize, from: Vector, to: Vector) -> TraceRequest {
        TraceRequest {
            world,
            body: 0,
            start: v3(from),
            end: v3(to),
            tag_require: [0; 8],
            tag_any: [0; 8],
            tag_exclude: [0; 8],
            filter: 0,
            trigger_filter: 0,
            object_set_mask: 0,
            use_hit_position: 0,
            shape: TraceShape { kind: 0, rotation: rot(Rotation::IDENTITY), mins: v3(Vector::ZERO), maxs: v3(Vector::ZERO), radius: [0.0; 2] },
        }
    }

    #[test]
    fn traces_hit_what_is_there() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (w, _, ball) = ground_and_ball();
        step(w, 1.0 / 60.0, 1);
        // a ray down from 100 meets the ground's top at 10
        let r = ray(w, Vector::new(100.0, 0.0, 100.0), Vector::new(100.0, 0.0, -100.0));
        let hit = trace(&r, false)[0];
        assert!((hit.f_Fraction - 0.45).abs() < 1e-3, "{}", hit.f_Fraction);
        assert!((vec(hit.f_Normal) - Vector::Z).length() < 1e-3);
        assert!((vec(hit.f_HitPos).z - 10.0).abs() < 1e-3);
        assert_eq!(hit.f_StartedInSolid, 0);
        // a box 20 tall swept down stops 10 short of it
        let mut b = r;
        b.shape = TraceShape { kind: 1, rotation: rot(Rotation::IDENTITY), mins: v3(Vector::splat(-10.0)), maxs: v3(Vector::splat(10.0)), radius: [0.0; 2] };
        let hit = trace(&b, false)[0];
        assert!((vec(hit.f_EndPos).z - 20.0).abs() < 0.1, "{}", vec(hit.f_EndPos).z);
        assert!((vec(hit.f_Normal) - Vector::Z).length() < 1e-2, "{:?}", vec(hit.f_Normal));
        // a sphere straight down through the ball: the ball first, then the ground
        let mut s = ray(w, Vector::new(0.0, 0.0, 400.0), Vector::new(0.0, 0.0, -100.0));
        s.shape.radius = [2.0, 0.0];
        let all = trace(&s, true);
        assert_eq!(all.len(), 2);
        assert!(all[0].f_Fraction < all[1].f_Fraction);
        // ignoring dynamic bodies, only the ground
        s.object_set_mask = 2;
        assert_eq!(trace(&s, true).len(), 1);
        // a ray that starts inside the ground
        let inside = trace(&ray(w, Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, -100.0)), false)[0];
        assert_eq!(inside.f_StartedInSolid, 1);
        assert_eq!(inside.f_Fraction, 0.0);
        // the ball's own sweep down, from where it is, stops on the ground
        let mut own = ray(w, Vector::new(0.0, 0.0, 200.0), Vector::new(0.0, 0.0, -100.0));
        own.body = ball;
        let hit = trace(&own, false)[0];
        assert!((vec(hit.f_EndPos).z - 18.0).abs() < 0.1, "{}", vec(hit.f_EndPos).z);
    }

    #[test]
    fn tags_and_rules_decide_what_is_hit_and_what_collides() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let w = ptr();
        create_world(w);
        let ground = ptr();
        add_body(w, ground, &BodySettings::default());
        let mut tagged = ShapeSettings { flags: SHAPE_TAGS | 16, ..Default::default() };
        tagged.tags[0] = murmur2("world");
        tagged.tags[1] = murmur2("ghostfloor");
        let floor = ptr();
        assert!(add_shape(ground, floor, &Geometry::Box(Pose::IDENTITY, Vector::new(500.0, 500.0, 10.0)), &tagged, &[]));
        // a trace that must hit "player" finds nothing; one that excludes "world" too
        let mut r = ray(w, Vector::new(0.0, 0.0, 100.0), Vector::new(0.0, 0.0, -100.0));
        r.tag_require[0] = murmur2("player");
        assert_eq!(trace(&r, false)[0].f_Fraction, 1.0);
        r.tag_require[0] = 0;
        r.tag_exclude[0] = murmur2("world");
        assert_eq!(trace(&r, false)[0].f_Fraction, 1.0);
        r.tag_exclude[0] = 0;
        r.tag_any = [murmur2("nothing"), murmur2("ghostfloor"), 0, 0, 0, 0, 0, 0];
        let hit = trace(&r, false)[0];
        assert!(hit.f_Fraction < 1.0);
        assert_eq!(hit.f_Tags[0], murmur2("world"));
        // a ball tagged "ghost", which the rules say ignores "ghostfloor": it falls through
        set_rules(w, r#"{"Defaults":{"solid":"Collide"},"Pairs":[{"a":"ghost","b":"ghostfloor","r":"Ignore"}]}"#);
        let ball = ptr();
        let mut s = BodySettings { body_type: DYNAMIC as i32, ..Default::default() };
        s.position = v3(Vector::new(0.0, 0.0, 50.0));
        add_body(w, ball, &s);
        let mut ghost = ShapeSettings { flags: SHAPE_TAGS | 16, ..Default::default() };
        ghost.tags[0] = murmur2("ghost");
        assert!(add_shape(ball, ptr(), &Geometry::Sphere(Vector::ZERO, 8.0), &ghost, &[]));
        for _ in 0..60 { step(w, 1.0 / 60.0, 1); }
        assert!(with_body(ball, |_, rb| rb.translation().z).unwrap() < -50.0);
    }

    #[test]
    fn touching_and_triggers_are_told() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (w, ground, ball) = ground_and_ball();
        // a trigger volume around the ball's path
        let zone = ptr();
        add_body(w, zone, &BodySettings::default());
        let trigger = ShapeSettings { flags: SHAPE_SENSOR | 16, collision_function_mask: 2, ..Default::default() };
        let volume = ptr();
        assert!(add_shape(zone, volume, &Geometry::Box(Pose::from_translation(Vector::new(0.0, 0.0, 100.0)), Vector::splat(20.0)), &trigger, &[]));
        let mut seen = Vec::new();
        for _ in 0..300 {
            step(w, 1.0 / 60.0, 1);
            seen.extend(take_intersections(w).into_iter().map(|e| (e.reason, e.left.body, e.right.body)));
        }
        let has = |reason, a, b| seen.iter().any(|&(r, l, rr)| r == reason && ((l, rr) == (a, b) || (l, rr) == (b, a)));
        assert!(has(TRIGGER_BEGIN, ball, zone), "{seen:?}");
        assert!(has(TRIGGER_END, ball, zone), "{seen:?}");
        assert!(has(TOUCH_BEGIN, ball, ground), "{seen:?}");
        assert!(touching_body(ball, ground, false));
        // nothing is told twice to begin
        assert_eq!(seen.iter().filter(|&&(r, l, rr)| r == TOUCH_BEGIN && (l == ball || rr == ball)).count(), 1);
        assert!(!remove_body(ball).is_empty());
        assert!(!touching_body(ball, ground, false));
    }

    #[test]
    fn keyframed_bodies_go_where_they_are_sent() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let w = ptr();
        create_world(w);
        let k = ptr();
        add_body(w, k, &BodySettings { body_type: KEYFRAMED as i32, ..Default::default() });
        add_shape(k, ptr(), &Geometry::Box(Pose::IDENTITY, Vector::splat(5.0)), &ShapeSettings::default(), &[]);
        set_target(k, Pose::from_translation(Vector::new(10.0, 0.0, 0.0)), 1.0 / 60.0);
        let active = step(w, 1.0 / 60.0, 1).active;
        assert!(active.iter().any(|a| a.body == k && (vec(a.transform.f_Position).x - 10.0).abs() < 1e-3));
    }

    #[test]
    fn collision_rules_read_as_managed_writes_them() {
        let r = Rules::from_json(r#"{"Defaults":{"solid":"Collide","trigger":"Trigger","ladder":3},"Pairs":[{"a":"trigger","b":"playerclip","r":"Ignore"},{"a":"solid","b":"solid","r":1}]}"#);
        let (solid, trigger, clip, ladder) = (murmur2("solid"), murmur2("trigger"), murmur2("playerclip"), murmur2("ladder"));
        assert_eq!(r.between(&[solid], &[solid]), RULE_COLLIDE);
        assert_eq!(r.between(&[trigger], &[clip]), RULE_IGNORE);
        assert_eq!(r.between(&[clip], &[trigger]), RULE_IGNORE);
        assert_eq!(r.between(&[ladder], &[clip]), RULE_IGNORE);
        assert_eq!(r.between(&[trigger], &[murmur2("other")]), RULE_TRIGGER);
        assert_eq!(r.between(&[], &[]), RULE_COLLIDE);
    }

    /// A surface of its own for a test (the list is the process's): its index.
    fn surface(name: &str, friction: f32, elasticity: f32, density: f32) -> i32 {
        let p = surfaces::add(name, "default", "");
        surfaces::update(p, Props { friction, elasticity, density, rolling_resistance: 0.0, bounce_threshold: 35.0 }).unwrap()
    }

    fn made_of(surface: i32) -> ShapeSettings {
        ShapeSettings { surface_index: surface, ..Default::default() }
    }

    /// A body at `at` with one shape: the body.
    fn body(w: usize, kind: i64, at: Vector, g: Geometry, s: &ShapeSettings) -> usize {
        let b = ptr();
        add_body(w, b, &BodySettings { body_type: kind as i32, position: v3(at), ..Default::default() });
        assert!(add_shape(b, ptr(), &g, s, &[]));
        b
    }

    #[test]
    fn a_ball_bounces_as_its_surface_does() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let rubber = surface("test_superball", 0.8, 0.9, 1200.0);
        // the highest each ball comes back up to after its first bounce
        let peak = |surface: i32| {
            let w = ptr();
            create_world(w);
            body(w, STATIC, Vector::ZERO, Geometry::Box(Pose::IDENTITY, Vector::new(500.0, 500.0, 10.0)), &made_of(-1));
            let ball = body(w, DYNAMIC, Vector::new(0.0, 0.0, 200.0), Geometry::Sphere(Vector::ZERO, 8.0), &made_of(surface));
            let (mut bounced, mut top) = (false, 0.0f32);
            for _ in 0..180 {
                step(w, 1.0 / 60.0, 4);
                let (z, vz) = with_body(ball, |_, rb| (rb.translation().z, rb.linvel().z)).unwrap();
                bounced |= vz > 0.0;
                if bounced { top = top.max(z); }
            }
            top - 18.0
        };
        // elasticity: the greater of the two (the ground's default is 0.05)
        let (high, low) = (peak(rubber), peak(-1));
        assert!(high > 100.0, "0.9 of its speed back: most of its 182 drop ({high})");
        assert!(low < 10.0, "the default barely bounces ({low})");
    }

    #[test]
    fn a_box_slides_as_far_as_its_surface_lets_it() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let ice = surface("test_ice", 0.01, 0.0, 900.0);
        let slide = |surface: i32| {
            let w = ptr();
            create_world(w);
            body(w, STATIC, Vector::ZERO, Geometry::Box(Pose::IDENTITY, Vector::new(5000.0, 500.0, 10.0)), &made_of(-1));
            let b = body(w, DYNAMIC, Vector::new(0.0, 0.0, 15.0), Geometry::Box(Pose::IDENTITY, Vector::splat(5.0)), &made_of(surface));
            with_body_mut(b, |_, rb| rb.set_linvel(Vector::new(300.0, 0.0, 0.0), true));
            for _ in 0..180 { step(w, 1.0 / 60.0, 4); }
            with_body(b, |_, rb| rb.translation().x).unwrap()
        };
        // friction: the geometric mean, sqrt(0.01 * 0.8) on ice against the
        // default's 0.8: v²/2μg, 595 and 66
        let (far, near) = (slide(ice), slide(-1));
        assert!(far > 400.0, "on ice ({far})");
        assert!(near < 100.0 && near > 30.0, "on the default ({near})");
    }

    #[test]
    fn a_shape_weighs_what_its_surface_does_and_follows_it() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let wood = surface("test_wood", 0.8, 0.05, 540.0);
        let w = ptr();
        create_world(w);
        let b = body(w, DYNAMIC, Vector::ZERO, Geometry::Box(Pose::IDENTITY, Vector::splat(5.0)), &made_of(wood));
        let mass = || with_body(b, |_, rb| rb.mass()).unwrap();
        // a thousand cubic inches of wood
        let expected = 1000.0 * 540.0 / surfaces::CUBIC_INCHES_PER_CUBIC_METRE;
        assert!((mass() - expected).abs() < 1e-3 * expected, "{} vs {expected}", mass());
        assert!((body_density(b) - 540.0).abs() < 0.5, "{}", body_density(b));
        // the surface reloaded heavier: the shapes made of it follow
        surfaces::update(surfaces::object(wood), Props { friction: 0.6, elasticity: 0.05, density: 1080.0, rolling_resistance: 0.0, bounce_threshold: 35.0 });
        surfaces_changed();
        assert!((mass() - 2.0 * expected).abs() < 1e-3 * expected, "{}", mass());
        let shape = shapes_of_body(b)[0];
        assert_eq!(friction(shape), 0.6);
        // a friction of its own wins over its surface's, and stays
        set_friction(shape, 0.3);
        surfaces_changed();
        assert_eq!(friction(shape), 0.3);
        // made of another by name
        set_body_surface(b, -1);
        assert_eq!(super::surface(shape), -1);
        assert!((body_density(b) - 2000.0).abs() < 1.0, "the default's: {}", body_density(b));
    }

    #[test]
    fn a_mesh_is_made_of_what_each_triangle_names() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (glue, ice) = (surface("test_glue", 1.0, 0.0, 1000.0), surface("test_black_ice", 0.0, 0.0, 1000.0));
        let w = ptr();
        create_world(w);
        // a floor of two halves: x < 0 material 0 (glue), x > 0 material 1 (ice)
        let v = [(-1000.0, -500.0), (0.0, -500.0), (0.0, 500.0), (-1000.0, 500.0), (1000.0, -500.0), (1000.0, 500.0)].map(|(x, y)| Vector::new(x, y, 0.0)).to_vec();
        let t = vec![[0, 1, 2], [0, 2, 3], [1, 4, 5], [1, 5, 2]];
        let parts = [glue, ice];
        let settings = ShapeSettings { surface_indices: parts.as_ptr() as usize, surface_count: 2, ..Default::default() };
        let floor = ptr();
        add_body(w, floor, &BodySettings::default());
        assert!(add_shape(floor, ptr(), &Geometry::Mesh(Pose::IDENTITY, v, t), &settings, &[0, 0, 1, 1]));
        // a trace tells which
        let at = |x: f32| trace(&ray(w, Vector::new(x, 0.0, 50.0), Vector::new(x, 0.0, -50.0)), false)[0];
        assert_eq!(at(-300.0).f_SurfaceProperty, glue);
        assert_eq!(at(300.0).f_SurfaceProperty, ice);
        // and a box slides on one and not the other
        let a = body(w, DYNAMIC, Vector::new(-500.0, 0.0, 5.0), Geometry::Box(Pose::IDENTITY, Vector::splat(5.0)), &made_of(-1));
        let b = body(w, DYNAMIC, Vector::new(500.0, 0.0, 5.0), Geometry::Box(Pose::IDENTITY, Vector::splat(5.0)), &made_of(-1));
        for x in [a, b] { with_body_mut(x, |_, rb| rb.set_linvel(Vector::new(0.0, 200.0, 0.0), true)); }
        let mut touched = Vec::new();
        for _ in 0..60 {
            step(w, 1.0 / 60.0, 4);
            touched.extend(take_intersections(w).into_iter().filter(|e| e.reason == TOUCH_BEGIN));
        }
        let y = |x: usize| with_body(x, |_, rb| rb.translation().y).unwrap();
        assert!(y(a) < 40.0, "held on glue ({})", y(a));
        assert!(y(b) > 150.0, "slid on ice ({})", y(b));
        // and a touch says which it began on
        let on = |body: usize| touched.iter().find_map(|e| [(e.left, e.right), (e.right, e.left)].into_iter().find(|(me, _)| me.body == body).map(|(_, floor)| floor.surface));
        assert_eq!(on(a), Some(glue));
        assert_eq!(on(b), Some(ice));
    }

    #[test]
    fn a_height_field_is_made_of_what_each_sample_names() {
        let _s = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let (grass, rock) = (surface("test_grass", 0.8, 0.0, 1000.0), surface("test_rock", 0.8, 0.0, 1000.0));
        let w = ptr();
        create_world(w);
        // 3 by 3 samples, 100 apart, flat: the lower-left cell (x 0..100, y 0..100) rock, the rest grass
        let mut materials = [0u8; 9];
        materials[0] = 1;
        let parts = [grass, rock];
        let settings = ShapeSettings { surface_indices: parts.as_ptr() as usize, surface_count: 2, ..Default::default() };
        let ground = ptr();
        add_body(w, ground, &BodySettings::default());
        assert!(add_shape(ground, ptr(), &Geometry::Height(vec![0; 9], 3, 3, 100.0, 1.0), &settings, &materials));
        let at = |x: f32, y: f32| trace(&ray(w, Vector::new(x, y, 50.0), Vector::new(x, y, -50.0)), false)[0];
        for (x, y, s) in [(30.0, 20.0, rock), (70.0, 80.0, rock), (150.0, 30.0, grass), (40.0, 160.0, grass), (150.0, 150.0, grass)] {
            let hit = at(x, y);
            assert!(hit.f_Fraction < 1.0, "the ground is at ({x}, {y})");
            assert_eq!(hit.f_SurfaceProperty, s, "at ({x}, {y})");
        }
    }
}
