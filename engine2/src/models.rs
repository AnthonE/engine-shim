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
}

pub enum Obj {
    Builder { physics: usize },
    Bodies(Bodies),
    Part(Part),
    Sphere(Vector, f32),
    Capsule(Vector, Vector, f32),
    Hull(Hull),
    Mesh(Mesh),
    Model { physics: usize },
}

pub struct Models {
    pub objs: HashMap<usize, Obj>,
}

pub fn models() -> MutexGuard<'static, Models> {
    static M: OnceLock<Mutex<Models>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(Models { objs: HashMap::new() })).lock().unwrap_or_else(|e| e.into_inner())
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
            Some(Obj::Model { physics }) => *physics,
            _ => 0,
        }
    }
}

/// A body array of `n` bodies and `joints` joints, each its own object.
pub fn new_bodies(n: i32, joints: i32) -> usize {
    let parts = (0..n.max(0)).map(|_| make(Obj::Part(Part::default()))).collect();
    let joints = (0..joints.max(0)).map(|_| crate::handles::object()).collect();
    make(Obj::Bodies(Bodies { parts, joints, surfaces: Vec::new() }))
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
    make(Obj::Model { physics })
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
