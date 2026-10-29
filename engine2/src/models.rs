//! Models made at runtime (managed's ModelBuilder: an editor mesh's
//! collision, a procedural prop), and what physics reads back from them.
//!
//! The builder hands native a description of each body (CPhysBodyDesc:
//! spheres, capsules, boxes, hulls from points, triangle meshes) in an
//! array (CPhysBodyDescArray); native makes a model whose physics container
//! (CPhysicsData) gives the parts back (VPhysXBodyPart_t), each hull and
//! mesh an object of its own (RnHull_t, RnMesh_t) that a body is then given
//! as a shape (IPhysicsBody's AddHullShape, AddMeshShape).
//!
//! Here a body's description and the part it becomes are the same object;
//! a hull's or mesh's description (RnHullDesc_t, RnMeshDesc_t) and the
//! hull or mesh itself are too. Objects are found by pointer; none is
//! freed (managed's handles may outlive what it deletes).
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock};

use rapier3d::math::{Pose, Vector};

/// A convex hull: its vertices, and its edges as vertex pairs.
#[derive(Clone, Default)]
pub struct Hull {
    pub points: Vec<Vector>,
    pub edges: Vec<(u32, u32)>,
}

impl Hull {
    /// The hull of `points` (None for fewer than four, or all in a plane).
    pub fn of(points: &[Vector]) -> Option<Hull> {
        let (points, triangles) = rapier3d::parry::transformation::try_convex_hull(points).ok()?;
        if points.len() < 4 { return None; }
        let mut edges: Vec<(u32, u32)> = triangles
            .iter()
            .flat_map(|t| [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])])
            .map(|(a, b)| (a.min(b), a.max(b)))
            .collect();
        edges.sort_unstable();
        edges.dedup();
        Some(Hull { points, edges })
    }

    pub fn bounds(&self) -> (Vector, Vector) {
        bounds(&self.points)
    }
}

/// A triangle mesh and each triangle's material.
#[derive(Clone, Default)]
pub struct Mesh {
    pub vertices: Vec<Vector>,
    pub triangles: Vec<[u32; 3]>,
    pub materials: Vec<u8>,
}

impl Mesh {
    pub fn bounds(&self) -> (Vector, Vector) {
        bounds(&self.vertices)
    }
}

fn bounds(points: &[Vector]) -> (Vector, Vector) {
    if points.is_empty() { return (Vector::ZERO, Vector::ZERO); }
    points.iter().fold((Vector::splat(f32::MAX), Vector::splat(f32::MIN)), |(lo, hi), &p| (lo.min(p), hi.max(p)))
}

/// One body of a model, as its builder described it.
#[derive(Clone)]
pub struct Part {
    pub spheres: Vec<usize>,
    pub capsules: Vec<usize>,
    pub hulls: Vec<usize>,
    pub meshes: Vec<usize>,
    pub mass: f32,
    pub inertia_scale: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub gravity_scale: f32,
    pub flags: u32,
    pub override_mass_center: bool,
    pub mass_center: Vector,
    pub bone: String,
    pub bind_pose: Pose,
    pub surface: u32,
}

impl Default for Part {
    fn default() -> Self {
        Part {
            spheres: Vec::new(),
            capsules: Vec::new(),
            hulls: Vec::new(),
            meshes: Vec::new(),
            mass: 0.0,
            inertia_scale: 1.0,
            linear_damping: 0.0,
            angular_damping: 0.0,
            gravity_scale: 1.0,
            flags: 0,
            override_mass_center: false,
            mass_center: Vector::ZERO,
            bone: String::new(),
            bind_pose: Pose::IDENTITY,
            surface: 0,
        }
    }
}

/// Bodies, their joints and their surfaces: a builder's description, and
/// the physics container of the model made from it.
#[derive(Clone, Default)]
pub struct Bodies {
    pub parts: Vec<usize>,
    pub joints: Vec<usize>,
    pub surfaces: Vec<i32>,
    /// the bone each part follows, and its bind pose (a model file's; none for a built one)
    pub bones: Vec<(String, Pose)>,
}

pub enum Obj {
    Builder { physics: usize },
    Bodies(Bodies),
    Part(Part),
    Sphere(Vector, f32),
    Capsule(Vector, Vector, f32),
    Hull(Hull),
    Mesh(Mesh),
    Model { physics: usize, file: Option<&'static ModelFile> },
}

/// What a model file holds that managed reads by pointer: a hitbox set, a
/// hitbox, an attachment (each the model's own, at its own address).
#[derive(Clone, Copy)]
pub enum Piece {
    HitBoxSet(&'static HitBoxSet),
    HitBox(&'static HitBox),
    Attachment(&'static Attachment),
}

pub struct Models {
    pub objs: HashMap<usize, Obj>,
    /// models loaded from files, by name, and each one's name
    pub by_name: HashMap<String, usize>,
    pub names: HashMap<usize, String>,
    pub pieces: HashMap<usize, Piece>,
}

pub fn models() -> MutexGuard<'static, Models> {
    static M: OnceLock<Mutex<Models>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(Models { objs: HashMap::new(), by_name: HashMap::new(), names: HashMap::new(), pieces: HashMap::new() })).lock().unwrap_or_else(|e| e.into_inner())
}

/// A new object: its pointer.
pub fn make(o: Obj) -> usize {
    let p = crate::handles::object();
    models().objs.insert(p, o);
    p
}

impl Models {
    pub fn part(&mut self, p: usize) -> Option<&mut Part> {
        match self.objs.get_mut(&p) {
            Some(Obj::Part(x)) => Some(x),
            _ => None,
        }
    }

    pub fn bodies(&mut self, p: usize) -> Option<&mut Bodies> {
        match self.objs.get_mut(&p) {
            Some(Obj::Bodies(x)) => Some(x),
            _ => None,
        }
    }

    pub fn hull(&self, p: usize) -> Option<&Hull> {
        match self.objs.get(&p) {
            Some(Obj::Hull(x)) => Some(x),
            _ => None,
        }
    }

    pub fn mesh(&self, p: usize) -> Option<&Mesh> {
        match self.objs.get(&p) {
            Some(Obj::Mesh(x)) => Some(x),
            _ => None,
        }
    }

    /// The physics container of a model (0 if none).
    pub fn physics_of(&self, model: usize) -> usize {
        match self.objs.get(&model) {
            Some(Obj::Model { physics, .. }) => *physics,
            _ => 0,
        }
    }

    /// What a model's file holds (None for a model built at runtime, or one
    /// whose file isn't here).
    pub fn file(&self, model: usize) -> Option<&'static ModelFile> {
        match self.objs.get(&model) {
            Some(Obj::Model { file, .. }) => *file,
            _ => None,
        }
    }
}

/// What a model's file holds, by the model's pointer.
pub fn file(model: usize) -> Option<&'static ModelFile> {
    models().file(model)
}

/// A piece of a model file, by the pointer managed was given for it.
pub fn piece(p: usize) -> Option<Piece> {
    models().pieces.get(&p).copied()
}

/// A model file's pieces, known by their addresses.
fn index_pieces(m: &mut Models, f: &'static ModelFile) {
    for mesh in &f.meshes {
        for set in &mesh.hitbox_sets {
            m.pieces.insert(set as *const HitBoxSet as usize, Piece::HitBoxSet(set));
            for b in &set.boxes { m.pieces.insert(b as *const HitBox as usize, Piece::HitBox(b)); }
        }
        for a in &mesh.attachments { m.pieces.insert(a as *const Attachment as usize, Piece::Attachment(a)); }
    }
}

/// A body array of `n` bodies and `joints` joints, each its own object.
pub fn new_bodies(n: i32, joints: i32) -> usize {
    let parts = (0..n.max(0)).map(|_| make(Obj::Part(Part::default()))).collect();
    let joints = (0..joints.max(0)).map(|_| crate::handles::object()).collect();
    make(Obj::Bodies(Bodies { parts, joints, ..Default::default() }))
}

/// The model a builder makes: its physics container a copy of the bodies
/// it was given (managed deletes the builder's array once the model is made).
pub fn build(builder: usize) -> usize {
    let mut m = models();
    let physics = match m.objs.get(&builder) {
        Some(Obj::Builder { physics }) => *physics,
        _ => 0,
    };
    let copy = m.bodies(physics).cloned();
    drop(m);
    let physics = copy.map(|b| make(Obj::Bodies(b))).unwrap_or(0);
    make(Obj::Model { physics, file: None })
}

// ---- models from files: a compiled model (`.vmdl_c`) as the same objects

use crate::assets::kv3::Value;
use crate::assets::model::{Attachment, HitBox, HitBoxSet, ModelFile};

/// The model a name names: loaded once, the same object after. A name with
/// no file yet (one still downloading), or one that can't be read, is a
/// model with no collision, and is looked for again next time.
pub fn load(name: &str) -> usize {
    let key = crate::fs::normal(name);
    if let Some(&m) = models().by_name.get(&key) { return m; }
    let (m, error) = match read_model(name) {
        Ok((physics, file)) => (from_file(physics, file), None),
        Err(e) => (from_file(0, None), Some(e)),
    };
    let mut all = models();
    all.names.insert(m, name.to_string());
    match error {
        None => { all.by_name.insert(key, m); }
        Some(e) => crate::trace::note(&format!("models: {name}: {e}")),
    }
    m
}

/// A model of a file's collision and the rest of what it holds, kept for
/// good: managed holds a model's pieces by pointer.
pub fn from_file(physics: usize, file: Option<ModelFile>) -> usize {
    let file: Option<&'static ModelFile> = file.map(|f| &*Box::leak(Box::new(f)));
    let m = make(Obj::Model { physics, file });
    if let Some(f) = file { index_pieces(&mut models(), f); }
    m
}

pub fn name_of(model: usize) -> Option<String> {
    models().names.get(&model).cloned()
}

/// A model file: its collision (the physics container, 0 for none) and the
/// rest of what it holds (assets/model.rs), if that reads.
fn read_model(name: &str) -> Result<(usize, Option<ModelFile>), String> {
    let bytes = crate::fs::read_compiled(name).ok_or("no file")?;
    let r = crate::assets::resource::Resource::parse(bytes)?;
    let file = match crate::assets::model::read(&r, crate::fs::read_compiled) {
        Ok(f) => Some(f),
        Err(e) => { crate::trace::note(&format!("models: {name}: {e}")); None }
    };
    Ok((read_collision(&r)?, file))
}

/// A model file's collision: its PHYS block, or the physics resources its
/// DATA names (`m_refPhysicsData`). The physics container (0: none).
fn read_collision(r: &crate::assets::resource::Resource) -> Result<usize, String> {
    if let Some(phys) = r.kv3("PHYS") {
        return Ok(physics(&phys.map_err(|e| e.to_string())?));
    }
    let data = r.kv3("DATA").ok_or("no DATA")?.map_err(|e| e.to_string())?;
    for refd in data.get("m_refPhysicsData").map(Value::as_array).unwrap_or(&[]) {
        let Some(path) = refd.as_str() else { continue };
        let Some(bytes) = crate::fs::read_compiled(path) else { continue };
        let r = crate::assets::resource::Resource::parse(bytes)?;
        if let Some(Ok(v)) = r.kv3("DATA") { return Ok(physics(&v)); }
    }
    Ok(0)
}

/// A vector: an array of three numbers.
fn vec3(v: Option<&Value>) -> Vector {
    let f = v.map(Value::floats).unwrap_or_default();
    Vector::new(f.first().copied().unwrap_or(0.0), f.get(1).copied().unwrap_or(0.0), f.get(2).copied().unwrap_or(0.0))
}

/// Points kept as an array of vectors, or packed as floats in a blob.
fn points(v: Option<&Value>) -> Vec<Vector> {
    match v {
        Some(Value::Blob(b)) => b.chunks_exact(12).map(|c| {
            let f = |i: usize| f32::from_le_bytes(c[i * 4..i * 4 + 4].try_into().unwrap());
            Vector::new(f(0), f(1), f(2))
        }).collect(),
        Some(Value::Array(a)) => a.iter().map(|p| vec3(Some(p))).collect(),
        _ => Vec::new(),
    }
}

/// Triangles kept as `{ m_nIndex = [a, b, c] }`s, or packed as ints in a blob.
fn triangles(v: Option<&Value>) -> Vec<[u32; 3]> {
    match v {
        Some(Value::Blob(b)) => b.chunks_exact(12).map(|c| core::array::from_fn(|i| u32::from_le_bytes(c[i * 4..i * 4 + 4].try_into().unwrap()))).collect(),
        Some(Value::Array(a)) => a.iter().filter_map(|t| {
            let f = t.get("m_nIndex")?.as_array().iter().filter_map(Value::as_i64).collect::<Vec<_>>();
            (f.len() == 3).then(|| [f[0] as u32, f[1] as u32, f[2] as u32])
        }).collect(),
        _ => Vec::new(),
    }
}

/// A bind pose: a 3x4 matrix (rows of rotation and translation) as twelve
/// numbers, or as three rows of four.
fn bind_pose(v: &Value) -> Pose {
    let f: Vec<f32> = match v {
        Value::Array(rows) if rows.iter().all(|r| matches!(r, Value::Array(_))) => rows.iter().flat_map(Value::floats).collect(),
        _ => v.floats(),
    };
    if f.len() < 12 { return Pose::IDENTITY; }
    let m = rapier3d::math::glamx::Mat3::from_cols(Vector::new(f[0], f[4], f[8]), Vector::new(f[1], f[5], f[9]), Vector::new(f[2], f[6], f[10]));
    Pose::from_parts(Vector::new(f[3], f[7], f[11]), rapier3d::math::Rotation::from_mat3(&m).normalize())
}

/// VPhysXAggregateData_t as the objects a built model's physics is.
fn physics(agg: &Value) -> usize {
    let mut parts = Vec::new();
    for p in agg.get("m_parts").map(Value::as_array).unwrap_or(&[]) {
        let shape = p.get("m_rnShape");
        let list = |name: &str| shape.and_then(|s| s.get(name)).map(Value::as_array).unwrap_or(&[]).to_vec();
        let mut part = Part {
            mass: p.f32("m_flMass"),
            inertia_scale: p.get("m_flInertiaScale").and_then(Value::as_f64).map(|f| f as f32).unwrap_or(1.0),
            linear_damping: p.f32("m_flLinearDamping"),
            angular_damping: p.f32("m_flAngularDamping"),
            flags: p.i64("m_nFlags") as u32,
            override_mass_center: p.i64("m_bOverrideMassCenter") != 0,
            mass_center: vec3(p.get("m_vMassCenterOverride")),
            ..Default::default()
        };
        for s in list("m_spheres") {
            let sphere = s.get("m_Sphere");
            let o = Obj::Sphere(vec3(sphere.and_then(|x| x.get("m_vCenter"))), sphere.map(|x| x.f32("m_flRadius")).unwrap_or(0.0));
            part.spheres.push(make(o));
        }
        for c in list("m_capsules") {
            let cap = c.get("m_Capsule");
            let ends = cap.and_then(|x| x.get("m_vCenter")).map(Value::as_array).unwrap_or(&[]);
            let o = Obj::Capsule(vec3(ends.first()), vec3(ends.get(1)), cap.map(|x| x.f32("m_flRadius")).unwrap_or(0.0));
            part.capsules.push(make(o));
        }
        for h in list("m_hulls") {
            let Some(hull) = h.get("m_Hull") else { continue };
            // explicit vertex indices (2023 on): the positions are apart
            let pts = if hull.get("m_VertexPositions").is_some() { points(hull.get("m_VertexPositions")) } else { points(hull.get("m_Vertices")) };
            if let Some(hull) = Hull::of(&pts) { part.hulls.push(make(Obj::Hull(hull))); }
        }
        for m in list("m_meshes") {
            let Some(mesh) = m.get("m_Mesh") else { continue };
            let materials = mesh.get("m_Materials").map(Value::bytes).unwrap_or_default();
            let o = Obj::Mesh(Mesh { vertices: points(mesh.get("m_Vertices")), triangles: triangles(mesh.get("m_Triangles")), materials });
            part.meshes.push(make(o));
        }
        parts.push(make(Obj::Part(part)));
    }
    let names: Vec<String> = agg.get("m_boneNames").map(Value::as_array).unwrap_or(&[]).iter().filter_map(|n| n.as_str().map(String::from)).collect();
    let poses: Vec<Pose> = agg.get("m_bindPose").map(Value::as_array).unwrap_or(&[]).iter().map(bind_pose).collect();
    let bones = names.into_iter().enumerate().map(|(i, n)| (n, poses.get(i).copied().unwrap_or(Pose::IDENTITY))).collect();
    make(Obj::Bodies(Bodies { parts, bones, ..Default::default() }))
}

/// A planar polygon (convex or not, wound either way) as triangles: indices
/// into its points, three to a triangle. Ears are clipped in the plane the
/// polygon faces most nearly (its Newell normal's largest axis dropped).
pub fn triangulate(points: &[Vector]) -> Vec<u32> {
    let n = points.len();
    if n < 3 { return Vec::new(); }
    let mut normal = Vector::ZERO;
    for i in 0..n {
        let (a, b) = (points[i], points[(i + 1) % n]);
        normal += Vector::new((a.y - b.y) * (a.z + b.z), (a.z - b.z) * (a.x + b.x), (a.x - b.x) * (a.y + b.y));
    }
    let abs = normal.abs();
    let (u, v, sign) = if abs.x >= abs.y && abs.x >= abs.z {
        (1, 2, normal.x)
    } else if abs.y >= abs.z {
        (2, 0, normal.y)
    } else {
        (0, 1, normal.z)
    };
    let flat: Vec<(f32, f32)> = points.iter().map(|p| (p[u], p[v])).collect();
    // counter-clockwise in the projection when the normal points along the dropped axis
    let ccw = sign >= 0.0;
    let cross = |a: (f32, f32), b: (f32, f32), c: (f32, f32)| (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
    let convex = |a, b, c| if ccw { cross(a, b, c) > 0.0 } else { cross(a, b, c) < 0.0 };
    let inside = |p: (f32, f32), a, b, c| {
        let (d1, d2, d3) = (cross(a, b, p), cross(b, c, p), cross(c, a, p));
        let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
        let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
        !(neg && pos)
    };
    let mut left: Vec<usize> = (0..n).collect();
    let mut out = Vec::with_capacity((n - 2) * 3);
    let mut guard = 0;
    while left.len() > 3 && guard < n * n {
        guard += 1;
        let m = left.len();
        let mut clipped = false;
        for i in 0..m {
            let (ia, ib, ic) = (left[(i + m - 1) % m], left[i], left[(i + 1) % m]);
            let (a, b, c) = (flat[ia], flat[ib], flat[ic]);
            if !convex(a, b, c) { continue; }
            if left.iter().any(|&k| k != ia && k != ib && k != ic && inside(flat[k], a, b, c)) { continue; }
            out.extend([ia as u32, ib as u32, ic as u32]);
            left.remove(i);
            clipped = true;
            break;
        }
        if !clipped { break; }
    }
    // what's left (a triangle, or a degenerate run): as a fan
    for i in 1..left.len().saturating_sub(1) {
        out.extend([left[0] as u32, left[i] as u32, left[i + 1] as u32]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polygons_triangulate_whichever_way_they_face_or_wind() {
        // a square in z, wound both ways: two triangles covering it
        let sq = [Vector::new(0.0, 0.0, 5.0), Vector::new(10.0, 0.0, 5.0), Vector::new(10.0, 10.0, 5.0), Vector::new(0.0, 10.0, 5.0)];
        let area = |pts: &[Vector], t: &[u32]| -> f32 {
            t.chunks(3).map(|c| (pts[c[1] as usize] - pts[c[0] as usize]).cross(pts[c[2] as usize] - pts[c[0] as usize]).length() / 2.0).sum()
        };
        let t = triangulate(&sq);
        assert_eq!(t.len(), 6);
        assert!((area(&sq, &t) - 100.0).abs() < 1e-3);
        let mut rev = sq;
        rev.reverse();
        assert!((area(&rev, &triangulate(&rev)) - 100.0).abs() < 1e-3);
        // an L in a wall (facing y): concave, six corners, four triangles, area 75
        let l = [(0.0, 0.0), (10.0, 0.0), (10.0, 5.0), (5.0, 5.0), (5.0, 10.0), (0.0, 10.0)].map(|(x, z)| Vector::new(x, 3.0, z));
        let t = triangulate(&l);
        assert_eq!(t.len(), 12);
        assert!((area(&l, &t) - 75.0).abs() < 1e-3, "{}", area(&l, &t));
    }

    #[test]
    fn a_hull_of_a_cube_has_its_eight_corners() {
        let mut points = Vec::new();
        for i in 0..8 {
            points.push(Vector::new((i & 1) as f32, ((i >> 1) & 1) as f32, ((i >> 2) & 1) as f32) * 10.0);
        }
        points.push(Vector::splat(5.0)); // inside: not a vertex
        let h = Hull::of(&points).unwrap();
        assert_eq!(h.points.len(), 8);
        // a cube's faces split into triangles: its 12 edges and a diagonal on each of its 6 faces
        assert_eq!(h.edges.len(), 18);
        assert_eq!(h.bounds(), (Vector::ZERO, Vector::splat(10.0)));
        assert!(Hull::of(&points[..3]).is_none());
    }

    #[test]
    fn a_downloaded_model_has_its_collision() {
        // s&box's own download cache, if this machine has one: Facepunch's
        // content, read where it lies, never copied here
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let dir = std::path::Path::new(&dir).join("models/props/washing_machine");
        let Some(file) = std::fs::read_dir(&dir).ok().and_then(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| p.to_string_lossy().ends_with(".vmdl_c") && p.to_string_lossy().contains("washing_machine."))) else { return };
        crate::fs::add_link("models/props/washing_machine/washing_machine.vmdl_c", file.to_str().unwrap());
        let m = load("models/props/washing_machine/washing_machine.vmdl");
        assert_eq!(load("Models/Props/Washing_Machine/washing_machine.vmdl"), m, "loaded once");
        let mut all = models();
        let physics = all.physics_of(m);
        let parts = all.bodies(physics).unwrap().parts.clone();
        assert_eq!(parts.len(), 1);
        let part = all.part(parts[0]).unwrap().clone();
        assert_eq!(part.mass, 80.0);
        assert!(!part.hulls.is_empty());
        let (lo, hi) = part.hulls.iter().map(|&h| all.hull(h).unwrap().bounds()).fold((Vector::splat(f32::MAX), Vector::splat(f32::MIN)), |(a, b), (c, d)| (a.min(c), b.max(d)));
        // a washing machine: about 30 wide, 28 deep, 40 tall
        assert!((hi - lo).z > 30.0 && (hi - lo).x > 20.0, "{lo:?} {hi:?}");
    }

    #[test]
    fn a_model_keeps_its_bodies_after_its_builder_lets_go() {
        let builder = make(Obj::Builder { physics: 0 });
        let bodies = new_bodies(2, 0);
        if let Some(Obj::Builder { physics }) = models().objs.get_mut(&builder) { *physics = bodies; }
        let model = build(builder);
        let physics = models().physics_of(model);
        assert_ne!(physics, bodies, "a copy of its own");
        assert_eq!(models().bodies(physics).unwrap().parts.len(), 2);
    }
}
