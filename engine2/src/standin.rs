//! The default player's stand-in. s&box's player model (Citizen: its mesh,
//! its animations, its animation graph, its clothing) comes only with
//! Facepunch's closed download, under their EULA, and is never read here. A
//! game built on s&box's `PlayerController` or `CitizenAnimationHelper`
//! asks for it by name all the same: a model at `models/citizen/...` or
//! `models/citizen_human/...`, bones and attachments by name, and a graph
//! with the parameters the helper sets. This module answers those names
//! with a figure of our own, made here in code and copied from nothing:
//!
//! - **a model**: a jointed mannequin of ellipsoids on a skeleton whose
//!   bones carry the names games ask for, with hitboxes, attachments and
//!   three materials (`materials/standin/*.vmat`, plain colours);
//! - **its animations**: standing, walking and running in four directions,
//!   crouched, in the air, seated, swimming, eight ways to hold something,
//!   and looking up and down, each computed from a gait, none recorded;
//! - **its graph**: the parameters `CitizenAnimationHelper` and
//!   `PlayerController` set (sbox-public, MIT), with their kinds and their
//!   options, driving those animations through the nodes animgraph.rs runs.
//!
//! What it takes from Citizen is names only: paths, parameter names and
//! their options, and bone and attachment names, the interface a game
//! compiles against, as the slot table is. Its shape, its motion and its
//! look are its own. Clothing isn't answered: a figure is drawn bare.
//!
//! It answers only when the file asked for isn't there (models.rs,
//! animgraph.rs, camera.rs and the renderer each ask after their own read
//! fails), so a game that ships its own model at one of these paths gets
//! its own. `ENGINE2_STANDIN=0` turns it off.
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::OnceLock;

use rapier3d::math::{Pose, Rotation, Vector};

use crate::animgraph::{Cmp, Condition, Damping, Graph, Node, Param, State, Transition, Variant, BOOL, ENUM, FLOAT, INT, VECTOR};
use crate::assets::anim::{Animations, BonePose, Clip, Event, Sequence};
use crate::assets::kv3::Value;
use crate::assets::material::Material;
use crate::assets::mesh::{Attribute, Buffers, Draw, IndexBuffer, Mesh as DrawMesh, VertexBuffer};
use crate::assets::model::{Attachment, Bone, DrawCall, HitBox, HitBoxSet, ModelFile, RenderMesh, SceneObject};

/// Its graph's name: what its model's key values name, and what a game
/// that names Citizen's own graph is given.
pub const GRAPH: &str = "models/standin/player.vanmgrph";
const BODY: &str = "materials/standin/body.vmat";
const SKIN: &str = "materials/standin/skin.vmat";
const EYES: &str = "materials/standin/eyes.vmat";
const MATERIALS: [&str; 3] = [BODY, SKIN, EYES];

fn on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("ENGINE2_STANDIN").map(|v| v != "0").unwrap_or(true))
}

fn key(name: &str) -> String {
    let n = crate::fs::normal(name);
    n.strip_suffix("_c").unwrap_or(&n).to_string()
}

/// Is `name` the default player's model: one the stand-in answers for?
pub fn is_model(name: &str) -> bool {
    let n = key(name);
    on() && n.ends_with(".vmdl") && (n.starts_with("models/citizen/") || n.starts_with("models/citizen_human/"))
}

/// Is `name` the default player's graph (or the stand-in's own)?
pub fn is_graph(name: &str) -> bool {
    let n = key(name);
    on() && (n == GRAPH || (n.ends_with(".vanmgrph") && (n.starts_with("models/citizen/") || n.starts_with("models/citizen_human/"))))
}

/// One of the stand-in's materials: a plain colour on `complex`.
pub fn material(name: &str) -> Option<Material> {
    if !on() { return None; }
    let n = key(name);
    let tint = match n.as_str() {
        BODY => [0.36, 0.42, 0.50, 1.0],
        SKIN => [0.80, 0.74, 0.66, 1.0],
        EYES => [0.05, 0.05, 0.06, 1.0],
        _ => return None,
    };
    Some(Material {
        name: n,
        shader: "shaders/complex.shader".into(),
        vectors: HashMap::from([("g_vColorTint".to_string(), tint)]),
        floats: HashMap::from([("g_flMetalness".to_string(), 0.0)]),
        ..Default::default()
    })
}

// ---- the skeleton

// x forward, y left, z up, in s&box's units (inches): a figure 70 tall,
// its eyes at 64. Every bone's bind rotation is the model's own axes, so a
// pose is rotations about them.
const N: usize = 22;
const PELVIS: usize = 0;
const SPINE0: usize = 1;
const SPINE1: usize = 2;
const SPINE2: usize = 3;
const NECK: usize = 4;
const HEAD: usize = 5;
#[allow(dead_code)]
const CLAV_L: usize = 6;
const UARM_L: usize = 7;
const LARM_L: usize = 8;
const HAND_L: usize = 9;
#[allow(dead_code)]
const CLAV_R: usize = 10;
const UARM_R: usize = 11;
const LARM_R: usize = 12;
const HAND_R: usize = 13;
const ULEG_L: usize = 14;
const LLEG_L: usize = 15;
const ANKLE_L: usize = 16;
const BALL_L: usize = 17;
const ULEG_R: usize = 18;
const LLEG_R: usize = 19;
const ANKLE_R: usize = 20;
const BALL_R: usize = 21;

/// name, parent, where it is in the model
const BONES: [(&str, i32, [f32; 3]); N] = [
    ("pelvis", -1, [0.0, 0.0, 37.0]),
    ("spine_0", 0, [0.0, 0.0, 40.5]),
    ("spine_1", 1, [0.0, 0.0, 45.0]),
    ("spine_2", 2, [0.0, 0.0, 49.5]),
    ("neck_0", 3, [0.0, 0.0, 56.5]),
    ("head", 4, [0.0, 0.0, 59.5]),
    ("clavicle_L", 3, [0.0, 2.0, 55.0]),
    ("arm_upper_L", 6, [0.0, 7.6, 55.0]),
    ("arm_lower_L", 7, [0.0, 7.6, 44.0]),
    ("hand_L", 8, [0.0, 7.6, 33.5]),
    ("clavicle_R", 3, [0.0, -2.0, 55.0]),
    ("arm_upper_R", 10, [0.0, -7.6, 55.0]),
    ("arm_lower_R", 11, [0.0, -7.6, 44.0]),
    ("hand_R", 12, [0.0, -7.6, 33.5]),
    ("leg_upper_L", 0, [0.0, 3.6, 36.5]),
    ("leg_lower_L", 14, [0.0, 3.6, 19.5]),
    ("ankle_L", 15, [0.0, 3.6, 3.0]),
    ("ball_L", 16, [5.5, 3.6, 0.8]),
    ("leg_upper_R", 0, [0.0, -3.6, 36.5]),
    ("leg_lower_R", 18, [0.0, -3.6, 19.5]),
    ("ankle_R", 19, [0.0, -3.6, 3.0]),
    ("ball_R", 20, [5.5, -3.6, 0.8]),
];
const THIGH: f32 = 17.0;
const SHIN: f32 = 16.5;

fn v(a: [f32; 3]) -> Vector { Vector::new(a[0], a[1], a[2]) }
fn cs(s: &str) -> CString { CString::new(s).unwrap_or_default() }

fn local_pos(i: usize) -> Vector {
    let (_, parent, at) = BONES[i];
    if parent < 0 { v(at) } else { v(at) - v(BONES[parent as usize].2) }
}

fn bones() -> Vec<Bone> {
    (0..N).map(|i| {
        let (name, parent, at) = BONES[i];
        Bone { name: cs(name), parent, pos: local_pos(i), rot: Rotation::IDENTITY, flags: 0, model: Pose::from_parts(v(at), Rotation::IDENTITY) }
    }).collect()
}

// ---- the mesh: ellipsoids, each moved by one bone

/// bone, centre in the model, radii, material
const PARTS: [(usize, [f32; 3], [f32; 3], usize); 25] = [
    (HEAD, [0.4, 0.0, 64.6], [4.5, 4.1, 5.3], 1),
    (HEAD, [4.3, 1.7, 65.4], [0.55, 0.85, 0.9], 2),
    (HEAD, [4.3, -1.7, 65.4], [0.55, 0.85, 0.9], 2),
    (NECK, [0.0, 0.0, 58.2], [1.9, 1.9, 2.6], 1),
    (SPINE2, [0.0, 0.0, 52.0], [4.4, 7.0, 6.2], 0),
    (SPINE1, [0.2, 0.0, 45.5], [3.9, 5.8, 5.0], 0),
    (PELVIS, [0.0, 0.0, 38.8], [4.1, 6.3, 4.6], 0),
    (UARM_L, [0.0, 7.7, 55.0], [2.4, 2.4, 2.4], 0),
    (UARM_L, [0.0, 7.7, 49.6], [1.9, 1.9, 6.2], 0),
    (LARM_L, [0.0, 7.7, 39.0], [1.6, 1.6, 5.9], 1),
    (HAND_L, [0.3, 7.7, 31.2], [1.3, 1.0, 3.0], 1),
    (UARM_R, [0.0, -7.7, 55.0], [2.4, 2.4, 2.4], 0),
    (UARM_R, [0.0, -7.7, 49.6], [1.9, 1.9, 6.2], 0),
    (LARM_R, [0.0, -7.7, 39.0], [1.6, 1.6, 5.9], 1),
    (HAND_R, [0.3, -7.7, 31.2], [1.3, 1.0, 3.0], 1),
    (ULEG_L, [0.0, 3.6, 28.0], [2.9, 2.9, 9.4], 0),
    (LLEG_L, [0.0, 3.6, 11.4], [2.2, 2.2, 8.9], 0),
    (LLEG_L, [0.2, 3.6, 19.5], [2.3, 2.3, 2.3], 0),
    (ANKLE_L, [1.6, 3.6, 1.7], [3.2, 1.9, 1.7], 0),
    (BALL_L, [5.6, 3.6, 1.2], [2.2, 1.8, 1.2], 0),
    (ULEG_R, [0.0, -3.6, 28.0], [2.9, 2.9, 9.4], 0),
    (LLEG_R, [0.0, -3.6, 11.4], [2.2, 2.2, 8.9], 0),
    (LLEG_R, [0.2, -3.6, 19.5], [2.3, 2.3, 2.3], 0),
    (ANKLE_R, [1.6, -3.6, 1.7], [3.2, 1.9, 1.7], 0),
    (BALL_R, [5.6, -3.6, 1.2], [2.2, 1.8, 1.2], 0),
];
const RINGS: usize = 8;
const SEGMENTS: usize = 12;
const STRIDE: usize = 40;

// DXGI's formats, as assets/mesh.rs reads them
const R32G32B32_FLOAT: u32 = 6;
const R32G32_FLOAT: u32 = 16;
const R8G8B8A8_UNORM: u32 = 28;
const R8G8B8A8_UINT: u32 = 30;

struct Built {
    vertices: Vec<u8>,
    count: usize,
    indices: Vec<u32>,
    /// each material's run of the indices: start, count
    runs: [(usize, usize); 3],
    min: Vector,
    max: Vector,
}

fn build() -> Built {
    let mut vertices = Vec::new();
    let mut count = 0usize;
    let mut indices = Vec::new();
    let mut runs = [(0, 0); 3];
    let (mut min, mut max) = (Vector::splat(f32::MAX), Vector::splat(f32::MIN));
    for material in 0..3 {
        let start = indices.len();
        for &(bone, centre, radii, _) in PARTS.iter().filter(|p| p.3 == material) {
            let base = count as u32;
            for ring in 0..=RINGS {
                let lat = std::f32::consts::PI * ring as f32 / RINGS as f32;
                for seg in 0..=SEGMENTS {
                    let lon = std::f32::consts::TAU * seg as f32 / SEGMENTS as f32;
                    let unit = [lat.sin() * lon.cos(), lat.sin() * lon.sin(), lat.cos()];
                    let p = [centre[0] + radii[0] * unit[0], centre[1] + radii[1] * unit[1], centre[2] + radii[2] * unit[2]];
                    let n = v([unit[0] / radii[0], unit[1] / radii[1], unit[2] / radii[2]]).normalize_or_zero();
                    min = min.min(v(p));
                    max = max.max(v(p));
                    for f in p { vertices.extend_from_slice(&f.to_le_bytes()); }
                    for f in [n.x, n.y, n.z] { vertices.extend_from_slice(&f.to_le_bytes()); }
                    for f in [seg as f32 / SEGMENTS as f32, ring as f32 / RINGS as f32] { vertices.extend_from_slice(&f.to_le_bytes()); }
                    vertices.extend_from_slice(&[bone as u8, 0, 0, 0]);
                    vertices.extend_from_slice(&[255, 0, 0, 0]);
                    count += 1;
                }
            }
            let row = (SEGMENTS + 1) as u32;
            for ring in 0..RINGS as u32 {
                for seg in 0..SEGMENTS as u32 {
                    let (a, b) = (base + ring * row + seg, base + (ring + 1) * row + seg);
                    indices.extend_from_slice(&[a, b, a + 1, a + 1, b, b + 1]);
                }
            }
        }
        runs[material] = (start, indices.len() - start);
    }
    Built { vertices, count, indices, runs, min, max }
}

fn built() -> &'static Built {
    static B: OnceLock<Built> = OnceLock::new();
    B.get_or_init(build)
}

/// What the renderer draws of it: one mesh, a draw to each material.
pub fn draw_meshes(name: &str) -> Option<(Vec<DrawMesh>, u64, Vec<Vec<String>>)> {
    if !is_model(name) { return None; }
    let b = built();
    let attribute = |name: &str, format: u32, offset: usize| Attribute { name: name.to_string(), index: 0, format, offset };
    let buffers = Buffers {
        vertices: vec![VertexBuffer {
            count: b.count,
            stride: STRIDE,
            attributes: vec![
                attribute("POSITION", R32G32B32_FLOAT, 0),
                attribute("NORMAL", R32G32B32_FLOAT, 12),
                attribute("TEXCOORD", R32G32_FLOAT, 24),
                attribute("BLENDINDICES", R8G8B8A8_UINT, 32),
                attribute("BLENDWEIGHT", R8G8B8A8_UNORM, 36),
            ],
            data: b.vertices.clone(),
        }],
        indices: vec![IndexBuffer { count: b.indices.len(), size: 4, data: b.indices.iter().flat_map(|i| i.to_le_bytes()).collect() }],
    };
    let draws = (0..3).map(|m| Draw {
        material: MATERIALS[m].to_string(), vertex_buffer: 0, index_buffer: 0, base_vertex: 0,
        start_index: b.runs[m].0, index_count: b.runs[m].1, tint: [1.0; 3],
    }).collect();
    Some((vec![DrawMesh { draws, buffers, bones: Vec::new(), group_mask: 1 }], 1, vec![MATERIALS.iter().map(|m| m.to_string()).collect()]))
}

// ---- hitboxes and attachments

fn hitboxes() -> HitBoxSet {
    // bone, from and to along it (in the bone's space), radius, a tag
    let capsules: [(usize, [f32; 3], [f32; 3], f32, &str); 15] = [
        (HEAD, [0.4, 0.0, 3.0], [0.4, 0.0, 7.0], 4.4, "head"),
        (SPINE2, [0.0, -2.5, 2.5], [0.0, 2.5, 2.5], 5.6, "chest"),
        (SPINE1, [0.0, -1.5, 0.5], [0.0, 1.5, 0.5], 5.0, "stomach"),
        (PELVIS, [0.0, -2.0, 1.8], [0.0, 2.0, 1.8], 4.8, "stomach"),
        (UARM_L, [0.0, 0.0, -1.0], [0.0, 0.0, -10.5], 2.2, "arm"),
        (LARM_L, [0.0, 0.0, -1.0], [0.0, 0.0, -10.0], 1.9, "arm"),
        (HAND_L, [0.0, 0.0, 0.0], [0.0, 0.0, -4.5], 1.7, "hand"),
        (UARM_R, [0.0, 0.0, -1.0], [0.0, 0.0, -10.5], 2.2, "arm"),
        (LARM_R, [0.0, 0.0, -1.0], [0.0, 0.0, -10.0], 1.9, "arm"),
        (HAND_R, [0.0, 0.0, 0.0], [0.0, 0.0, -4.5], 1.7, "hand"),
        (ULEG_L, [0.0, 0.0, -1.0], [0.0, 0.0, -16.0], 3.0, "leg"),
        (LLEG_L, [0.0, 0.0, -1.0], [0.0, 0.0, -17.0], 2.4, "leg"),
        (ULEG_R, [0.0, 0.0, -1.0], [0.0, 0.0, -16.0], 3.0, "leg"),
        (LLEG_R, [0.0, 0.0, -1.0], [0.0, 0.0, -17.0], 2.4, "leg"),
        (NECK, [0.0, 0.0, 0.0], [0.0, 0.0, 3.0], 2.2, "neck"),
    ];
    HitBoxSet {
        name: cs("default"),
        source: cs(""),
        boxes: capsules.iter().enumerate().map(|(i, &(bone, from, to, radius, tag))| HitBox {
            name: cs(BONES[bone].0),
            surface: cs("flesh"),
            bone: cs(BONES[bone].0),
            min: v(from),
            max: v(to),
            radius,
            shape: 2,
            bone_hash: crate::handles::murmur2(BONES[bone].0),
            color: [255, 255, 255, 255],
            index: i as u16,
            translation_only: false,
            visible: true,
            tags: vec![cs(tag)],
        }).collect(),
    }
}

fn attachments() -> Vec<Attachment> {
    let on = |name: &str, bone: usize, offset: [f32; 3]| Attachment { name: cs(name), influences: vec![(cs(BONES[bone].0), v(offset), Rotation::IDENTITY)], ignore_rotation: false };
    vec![
        on("eyes", HEAD, [3.8, 0.0, 4.5]),
        on("hat", HEAD, [0.0, 0.0, 10.4]),
        on("forward_reference", PELVIS, [10.0, 0.0, 0.0]),
        on("hand_L", HAND_L, [0.6, 0.0, -2.6]),
        on("hand_R", HAND_R, [0.6, 0.0, -2.6]),
        on("hold_L", HAND_L, [0.6, 0.0, -2.6]),
        on("hold_R", HAND_R, [0.6, 0.0, -2.6]),
        on("foot_L", ANKLE_L, [2.0, 0.0, -3.0]),
        on("foot_R", ANKLE_R, [2.0, 0.0, -3.0]),
    ]
}

// ---- its poses

fn rx(deg: f32) -> Rotation { Rotation::from_rotation_x(deg.to_radians()) }
fn ry(deg: f32) -> Rotation { Rotation::from_rotation_y(deg.to_radians()) }
fn rz(deg: f32) -> Rotation { Rotation::from_rotation_z(deg.to_radians()) }

/// A figure posed: each bone's rotation in its parent's space (the bind
/// pose is none), and how far its pelvis is from where it stands.
#[derive(Clone)]
struct Fig {
    r: [Rotation; N],
    lift: Vector,
}

impl Fig {
    /// Standing, arms a little out from its sides and bent a little.
    fn rest() -> Fig {
        let mut f = Fig { r: [Rotation::IDENTITY; N], lift: Vector::ZERO };
        f.r[UARM_L] = rx(7.0);
        f.r[UARM_R] = rx(-7.0);
        f.r[LARM_L] = ry(-10.0);
        f.r[LARM_R] = ry(-10.0);
        f
    }

    fn pose(&self) -> Vec<BonePose> {
        (0..N).map(|i| BonePose { pos: local_pos(i) + if i == PELVIS { self.lift } else { Vector::ZERO }, rot: self.r[i], scale: 1.0 }).collect()
    }

    /// Only how each bone is turned from the bind pose: a delta.
    fn delta(&self) -> Vec<BonePose> {
        (0..N).map(|i| BonePose { pos: if i == PELVIS { self.lift } else { Vector::ZERO }, rot: self.r[i], scale: 1.0 }).collect()
    }
}

/// How far the pelvis drops when its thighs are `hip` degrees forward and
/// its shins `knee` degrees bent back from them.
fn drop(hip: f32, knee: f32) -> f32 {
    THIGH + SHIN - (THIGH * hip.to_radians().cos() + SHIN * (knee - hip).to_radians().cos())
}

/// A leg: its thigh `hip` degrees forward and `side` degrees out toward
/// the left, its knee bent `knee`, its foot kept near level.
fn leg(f: &mut Fig, upper: usize, hip: f32, side: f32, knee: f32) {
    f.r[upper] = rx(side) * ry(-hip);
    f.r[upper + 1] = ry(knee);
    f.r[upper + 2] = ry(0.8 * (hip - knee));
}

/// One moment of a gait: `t` through its cycle (the left foot lands at a
/// quarter, the right at three), going `dir` (forward, right; each -1 to
/// 1), `effort` from a walk (0) to a run (1), `crouch` from standing (0)
/// to crouched (1).
fn gait(t: f32, dir: [f32; 2], effort: f32, crouch: f32) -> Fig {
    let mut f = Fig::rest();
    let tau = std::f32::consts::TAU;
    let moving = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt().min(1.0);
    let (base_hip, base_knee) = (58.0 * crouch, 104.0 * crouch);
    let swing = (22.0 + 20.0 * effort) * (1.0 - 0.35 * crouch);
    let bend = 42.0 + 34.0 * effort;
    let mut lowest = f32::MAX;
    for (upper, phase) in [(ULEG_L, t), (ULEG_R, t + 0.5)] {
        let s = (tau * phase).sin() * swing;
        // bent most as it passes under the body going forward
        let k = 5.0 * moving + bend * moving * (0.5 + 0.5 * (tau * phase).cos()).powi(2);
        let hip = base_hip + s * dir[0];
        // sideways its legs open and close, less far than they swing
        leg(&mut f, upper, hip, -s * dir[1] * 0.45, base_knee + k);
        // how far below the pelvis this foot reaches: the pelvis rides on the lower one
        lowest = lowest.min(drop(hip, base_knee + k));
    }
    let bob = (0.5 - 0.5 * (2.0 * tau * t).cos()) * (0.5 + 1.0 * effort) * moving;
    f.lift = Vector::new(0.0, (tau * t).cos() * 0.6 * moving, -lowest.max(0.0) - bob);
    // the arms, each against its own side's leg
    let arm = (16.0 + 24.0 * effort) * (0.35 + 0.65 * dir[0].abs()) * moving * (1.0 - 0.5 * crouch);
    let elbow = 10.0 + 55.0 * effort * moving;
    let l = (tau * t).sin() * arm * if dir[0] < 0.0 { -1.0 } else { 1.0 };
    f.r[UARM_L] = rx(7.0) * ry(l);
    f.r[UARM_R] = rx(-7.0) * ry(-l);
    f.r[LARM_L] = ry(-elbow - 0.3 * l.min(0.0).abs());
    f.r[LARM_R] = ry(-elbow - 0.3 * (-l).min(0.0).abs());
    // leaning into it, the shoulders against the hips
    let lean = (3.0 + 9.0 * effort) * dir[0] * moving;
    let twist = (tau * t).sin() * (4.0 + 4.0 * effort) * dir[0] * moving;
    f.r[PELVIS] = rz(twist);
    f.r[SPINE0] = ry(18.0 * crouch + lean * 0.5);
    f.r[SPINE1] = ry(8.0 * crouch + lean * 0.5) * rz(-twist);
    f.r[SPINE2] = ry(6.0 * crouch) * rz(-twist * 0.8);
    f.r[NECK] = ry(-(24.0 * crouch + lean * 0.6));
    f
}

fn idle(t: f32, crouch: f32) -> Fig {
    let mut f = gait(0.0, [0.0, 0.0], 0.0, crouch);
    let breath = (std::f32::consts::TAU * t).sin();
    f.r[SPINE1] = f.r[SPINE1] * ry(1.2 * breath);
    f.r[SPINE2] = f.r[SPINE2] * ry(0.9 * breath);
    f.r[NECK] = f.r[NECK] * ry(-1.4 * breath);
    f.r[UARM_L] = rx(7.0 + 0.8 * breath);
    f.r[UARM_R] = rx(-7.0 - 0.8 * breath);
    f
}

fn in_the_air(t: f32) -> Fig {
    let mut f = Fig::rest();
    let w = (std::f32::consts::TAU * t).sin();
    leg(&mut f, ULEG_L, 26.0 + 3.0 * w, 4.0, 42.0);
    leg(&mut f, ULEG_R, 8.0 - 3.0 * w, -4.0, 24.0);
    f.r[UARM_L] = rx(38.0 + 4.0 * w) * ry(-12.0);
    f.r[UARM_R] = rx(-38.0 - 4.0 * w) * ry(-12.0);
    f.r[LARM_L] = ry(-28.0);
    f.r[LARM_R] = ry(-28.0);
    f.r[SPINE1] = ry(5.0);
    f
}

fn seated(t: f32) -> Fig {
    let mut f = Fig::rest();
    let breath = (std::f32::consts::TAU * t).sin();
    for upper in [ULEG_L, ULEG_R] {
        f.r[upper] = ry(-86.0);
        f.r[upper + 1] = ry(84.0);
        f.r[upper + 2] = ry(2.0);
    }
    f.lift = Vector::new(0.0, 0.0, -drop(86.0, 84.0));
    f.r[UARM_L] = rx(6.0) * ry(-28.0);
    f.r[UARM_R] = rx(-6.0) * ry(-28.0);
    f.r[LARM_L] = ry(-58.0);
    f.r[LARM_R] = ry(-58.0);
    f.r[SPINE1] = ry(2.0 + 1.0 * breath);
    f
}

fn swimming(t: f32) -> Fig {
    let mut f = Fig::rest();
    let tau = std::f32::consts::TAU;
    f.r[PELVIS] = ry(68.0);
    f.lift = Vector::new(0.0, 0.0, -8.0);
    let kick = (tau * t * 2.0).sin() * 16.0;
    leg(&mut f, ULEG_L, kick, 2.0, 14.0 + kick.max(0.0) * 0.6);
    leg(&mut f, ULEG_R, -kick, -2.0, 14.0 + (-kick).max(0.0) * 0.6);
    let reach = (tau * t).sin();
    f.r[UARM_L] = rx(18.0 + 30.0 * reach.max(0.0)) * ry(-(150.0 - 40.0 * reach.max(0.0)));
    f.r[UARM_R] = rx(-18.0 - 30.0 * (-reach).max(0.0)) * ry(-(150.0 - 40.0 * (-reach).max(0.0)));
    f.r[LARM_L] = ry(-14.0);
    f.r[LARM_R] = ry(-14.0);
    f.r[NECK] = ry(-34.0);
    f.r[HEAD] = ry(-22.0);
    f
}

/// How its arms hold something: by NativeEngine's hold types, 1 to 8
/// (pistol, rifle, shotgun, an item, fists, a swing, a launcher, the physgun).
fn hold(kind: usize) -> Fig {
    let mut f = Fig::rest();
    // right upper arm (out, forward), right elbow, left upper arm (in, forward), left elbow
    let (ro, rf, re, lo, lf, le): (f32, f32, f32, f32, f32, f32) = match kind {
        1 => (8.0, 82.0, 8.0, -22.0, 68.0, 30.0),
        2 => (-12.0, 40.0, 86.0, -28.0, 72.0, 25.0),
        3 => (-12.0, 32.0, 80.0, -27.0, 62.0, 30.0),
        4 => (-4.0, 30.0, 76.0, 7.0, 0.0, 10.0),
        5 => (-15.0, 46.0, 120.0, 15.0, 56.0, 114.0),
        6 => (-30.0, 72.0, 100.0, 7.0, 8.0, 24.0),
        7 => (-26.0, 96.0, 110.0, -20.0, 100.0, 70.0),
        _ => (-5.0, 36.0, 70.0, -18.0, 46.0, 60.0),
    };
    f.r[UARM_R] = rx(ro) * ry(-rf);
    f.r[LARM_R] = ry(-re);
    f.r[UARM_L] = rx(lo) * ry(-lf);
    f.r[LARM_L] = ry(-le);
    f
}

/// Bent to look `pitch` degrees down (up, below zero): a delta.
fn looking(pitch: f32) -> Fig {
    let mut f = Fig { r: [Rotation::IDENTITY; N], lift: Vector::ZERO };
    f.r[SPINE1] = ry(pitch * 0.25);
    f.r[SPINE2] = ry(pitch * 0.25);
    f.r[NECK] = ry(pitch * 0.2);
    f.r[HEAD] = ry(pitch * 0.2);
    f
}

// ---- its clips

/// A step: the foot (0 left, 1 right), as managed's OnFootstepEvent reads it.
fn footstep(cycle: f32, foot: i64) -> Event {
    Event {
        cycle,
        name: cs("AE_FOOTSTEP"),
        data: Value::Object(vec![
            ("Foot".to_string(), Value::Int(foot)),
            ("attachment".to_string(), Value::String(if foot == 0 { "foot_L" } else { "foot_R" }.to_string())),
            ("volume".to_string(), Value::Double(1.0)),
        ]),
    }
}

/// A cycle's length, the same for every gait so they blend in step.
const STRIDE_SECONDS: f32 = 0.72;
const HOLDS: [&str; 8] = ["hold_pistol", "hold_rifle", "hold_shotgun", "hold_item", "hold_punch", "hold_swing", "hold_rpg", "hold_physgun"];
/// name, direction (forward, right), effort, crouch
const GAITS: [(&str, [f32; 2], f32, f32); 12] = [
    ("walk_n", [1.0, 0.0], 0.0, 0.0), ("walk_s", [-1.0, 0.0], 0.0, 0.0), ("walk_e", [0.0, 1.0], 0.0, 0.0), ("walk_w", [0.0, -1.0], 0.0, 0.0),
    ("run_n", [1.0, 0.0], 1.0, 0.0), ("run_s", [-1.0, 0.0], 0.8, 0.0), ("run_e", [0.0, 1.0], 0.9, 0.0), ("run_w", [0.0, -1.0], 0.9, 0.0),
    ("crouch_walk_n", [1.0, 0.0], 0.0, 1.0), ("crouch_walk_s", [-1.0, 0.0], 0.0, 1.0), ("crouch_walk_e", [0.0, 1.0], 0.0, 1.0), ("crouch_walk_w", [0.0, -1.0], 0.0, 1.0),
];

fn animations() -> Animations {
    let mut clips = Vec::new();
    // a loop: its last frame its first again
    let looped = |name: &str, seconds: f32, frames: usize, at: &dyn Fn(f32) -> Fig, events: Vec<Event>| {
        let poses = (0..=frames).map(|i| at(i as f32 / frames as f32).pose()).collect();
        Clip::keyed(name, frames as f32 / seconds, true, false, poses, events)
    };
    clips.push(looped("idle", 3.2, 48, &|t| idle(t, 0.0), Vec::new()));
    clips.push(looped("crouch_idle", 3.2, 48, &|t| idle(t, 1.0), Vec::new()));
    for (name, dir, effort, crouch) in GAITS {
        clips.push(looped(name, STRIDE_SECONDS, 24, &|t| gait(t, dir, effort, crouch), vec![footstep(0.25, 0), footstep(0.75, 1)]));
    }
    clips.push(looped("air", 1.6, 24, &in_the_air, Vec::new()));
    clips.push(looped("sit", 3.2, 24, &seated, Vec::new()));
    clips.push(looped("swim", 1.4, 28, &swimming, Vec::new()));
    for (i, name) in HOLDS.iter().enumerate() {
        clips.push(Clip::keyed(name, 30.0, true, false, vec![hold(i + 1).pose(), hold(i + 1).pose()], Vec::new()));
    }
    for (name, pitch) in [("look_up_delta", -90.0), ("look_level_delta", 0.0), ("look_down_delta", 90.0)] {
        clips.push(Clip::keyed(name, 30.0, true, true, vec![looking(pitch).delta(), looking(pitch).delta()], Vec::new()));
    }
    let sequences = clips.iter().enumerate().map(|(i, c)| Sequence { name: c.name.clone(), clip: i, looping: c.looping }).collect();
    // what a held thing poses: the chest up, half of the back below it
    let mut upper = vec![None; N];
    for (bone, w) in [(PELVIS, 0.0), (SPINE0, 0.0), (SPINE1, 0.35), (SPINE2, 1.0), (ULEG_L, 0.0), (ULEG_R, 0.0)] { upper[bone] = Some(w); }
    let mut a = Animations::default();
    a.clips = clips;
    a.sequences = sequences;
    a.masks = vec![("upper_body".to_string(), upper)];
    a
}

// ---- the model

/// The stand-in as a model file's contents, if `name` is the default player's.
pub fn model(name: &str) -> Option<ModelFile> {
    if !is_model(name) { return None; }
    let b = built();
    let anim = animations();
    let draws = (0..3).map(|m| DrawCall {
        primitive: 5, base_vertex: 0, vertices: b.count as i32, start_index: b.runs[m].0 as i32, indices: b.runs[m].1 as i32,
        instances: 1, uv_density: 1.0, material: MATERIALS[m].to_string(), material_object: 0,
    }).collect();
    let mesh_groups = vec![cs("default")];
    Some(ModelFile {
        bones: bones(),
        meshes: vec![RenderMesh { name: cs("standin"), objects: vec![SceneObject { min: b.min, max: b.max, draws }], hitbox_sets: vec![hitboxes()], attachments: attachments() }],
        mesh_group_masks: vec![1],
        lod_masks: vec![1],
        body_parts: crate::assets::model::body_parts(&mesh_groups),
        mesh_groups,
        default_mask: 1,
        lod_switch: vec![0.0],
        material_groups: vec![cs("default")],
        group_materials: vec![MATERIALS.iter().map(|m| m.to_string()).collect()],
        sequences: anim.sequences.iter().map(|s| cs(&s.name)).collect(),
        animations: anim.clips.iter().map(|c| cs(&c.name)).collect(),
        anim,
        key_values: Some(Value::Object(vec![("anim_graph_resource".to_string(), Value::String(GRAPH.to_string()))])),
    })
}

/// For looking at it (render/): its bones' skinning matrices in one of its
/// sequences, `cycle` of the way through (none: no such sequence).
pub fn skin_at(sequence: &str, cycle: f32) -> Option<Vec<glam::Mat4>> {
    let m = model("models/citizen/citizen.vmdl")?;
    let seq = m.anim.sequences.iter().find(|s| s.name.eq_ignore_ascii_case(sequence))?;
    let bind: Vec<BonePose> = m.bones.iter().map(|b| BonePose { pos: b.pos, rot: b.rot, scale: 1.0 }).collect();
    let pose = m.anim.sample(seq.clip, cycle, &bind);
    let mut world: Vec<glam::Mat4> = Vec::new();
    for (i, b) in m.bones.iter().enumerate() {
        let p = pose[i];
        let local = glam::Mat4::from_rotation_translation(glam::Quat::from_xyzw(p.rot.x, p.rot.y, p.rot.z, p.rot.w), glam::Vec3::new(p.pos.x, p.pos.y, p.pos.z));
        world.push(if b.parent < 0 { local } else { world[b.parent as usize] * local });
    }
    Some(world.iter().zip(&m.bones).map(|(w, b)| *w * glam::Mat4::from_translation(glam::Vec3::new(b.model.translation.x, b.model.translation.y, b.model.translation.z)).inverse()).collect())
}

/// Its sequences' names.
pub fn sequences() -> Vec<String> {
    animations().sequences.into_iter().map(|s| s.name).collect()
}

// ---- the graph

// What sets each: sbox-public's CitizenAnimationHelper.cs and
// PlayerController (Animation, Input), MIT.
const HOLD_TYPES: [&str; 9] = ["none", "pistol", "rifle", "shotgun", "holditem", "punch", "swing", "rpg", "physgun"];

fn params() -> Vec<Param> {
    let mut out: Vec<Param> = Vec::new();
    let mut add = |name: &str, kind: u8, default: Variant, min: Variant, max: Variant, options: &[&str], auto_reset: bool| {
        let id = out.len() as u32 + 1;
        out.push(Param { name: cs(name), id, kind, default, min, max, options: options.iter().map(|o| cs(o)).collect(), auto_reset });
    };
    let fl = |x: f32| Variant::float(FLOAT, x);
    for name in ["move_x", "move_y", "move_z", "wish_x", "wish_y", "wish_z"] { add(name, FLOAT, fl(0.0), fl(-1000.0), fl(1000.0), &[], false); }
    for name in ["move_groundspeed", "wish_groundspeed"] { add(name, FLOAT, fl(0.0), fl(0.0), fl(1000.0), &[], false); }
    for name in ["move_rotationspeed", "move_shuffle"] { add(name, FLOAT, fl(0.0), fl(-1000.0), fl(1000.0), &[], false); }
    for name in ["duck", "voice", "aim_eyes_weight", "aim_head_weight", "aim_body_weight", "hit_strength"] { add(name, FLOAT, fl(0.0), fl(0.0), fl(1.0), &[], false); }
    add("aim_body_pitch", FLOAT, fl(0.0), fl(-90.0), fl(90.0), &[], false);
    add("sit_offset_height", FLOAT, fl(0.0), fl(-100.0), fl(100.0), &[], false);
    add("scale_height", FLOAT, fl(1.0), fl(0.0), fl(2.0), &[], false);
    add("speed_reload", FLOAT, fl(1.0), fl(0.0), fl(5.0), &[], false);
    let bool_of = |b: bool| Variant::int(BOOL, b as i32);
    add("b_grounded", BOOL, bool_of(true), bool_of(false), bool_of(true), &[], false);
    for name in ["b_swim", "b_sit", "b_noclip", "b_long_idle", "b_weapon_lower", "viewer"] { add(name, BOOL, bool_of(false), bool_of(false), bool_of(true), &[], false); }
    for name in ["b_jump", "b_deploy", "hit"] { add(name, BOOL, bool_of(false), bool_of(false), bool_of(true), &[], true); }
    let en = |i: i32| Variant::int(ENUM, i);
    add("holdtype", ENUM, en(0), en(0), en(8), &HOLD_TYPES, false);
    add("holdtype_handedness", ENUM, en(0), en(0), en(2), &["both", "right", "left"], false);
    add("move_style", ENUM, en(0), en(0), en(2), &["auto", "walk", "run"], false);
    add("special_movement_states", ENUM, en(0), en(0), en(3), &["none", "ledgegrab", "roll", "slide"], false);
    let int = |i: i32| Variant::int(INT, i);
    add("sit", INT, int(0), int(0), int(16), &[], false);
    add("hit_bone", INT, int(0), int(0), int(255), &[], false);
    let vec0 = Variant { raw: [0.0; 4], kind: VECTOR };
    for name in ["aim_eyes", "aim_head", "aim_body", "hit_offset", "hit_direction"] { add(name, VECTOR, vec0, vec0, vec0, &[], false); }
    out
}

/// The stand-in's graph, if `name` is the default player's (or its own).
pub fn graph(name: &str) -> Option<Graph> {
    if !is_graph(name) { return None; }
    let params = params();
    let id = |name: &str| params.iter().find(|p| p.name.to_bytes() == name.as_bytes()).map(|p| p.id).unwrap_or(u32::MAX);
    let mut nodes: HashMap<u32, Node> = HashMap::new();
    let mut next = 100u32;
    let mut put = |nodes: &mut HashMap<u32, Node>, n: Node| { next += 1; nodes.insert(next, n); next };
    let seq = |name: &str| Node::Sequence { name: name.to_string(), speed: 1.0, looping: true, tags: Vec::new() };
    let spring = Some(Damping { tension: 120.0, damping: 20.0, speed: 0.0 });
    // on its feet: where it's going picks among the gaits, standing and crouched
    let walk = 110.0;
    let run = 320.0;
    let standing = put(&mut nodes, Node::Blend2D {
        items: vec![
            ("idle".to_string(), [0.0, 0.0]),
            ("walk_n".to_string(), [walk, 0.0]), ("walk_s".to_string(), [-walk, 0.0]), ("walk_e".to_string(), [0.0, walk]), ("walk_w".to_string(), [0.0, -walk]),
            ("run_n".to_string(), [run, 0.0]), ("run_s".to_string(), [-run, 0.0]), ("run_e".to_string(), [0.0, run]), ("run_w".to_string(), [0.0, -run]),
        ],
        x: id("move_x"), y: id("move_y"), looping: true, speed: 1.0, damping: spring,
    });
    let crouched = put(&mut nodes, Node::Blend2D {
        items: vec![
            ("crouch_idle".to_string(), [0.0, 0.0]),
            ("crouch_walk_n".to_string(), [70.0, 0.0]), ("crouch_walk_s".to_string(), [-70.0, 0.0]), ("crouch_walk_e".to_string(), [0.0, 70.0]), ("crouch_walk_w".to_string(), [0.0, -70.0]),
        ],
        x: id("move_x"), y: id("move_y"), looping: true, speed: 1.0, damping: spring,
    });
    let ground = put(&mut nodes, Node::Blend { children: vec![(standing, 0.0), (crouched, 1.0)], param: id("duck"), damping: Some(Damping { tension: 160.0, damping: 24.0, speed: 0.0 }) });
    let air = put(&mut nodes, seq("air"));
    let sit = put(&mut nodes, seq("sit"));
    let swim = put(&mut nodes, seq("swim"));
    // where it is: on the ground, in the air (or noclipping), seated, in water
    let is = |param: &str, on: bool| Condition::Param { param: id(param), op: Cmp::Eq, value: on as i32 as f32 };
    let to = |dest: u32, conditions: Vec<Condition>| Transition { conditions, dest, blend: 0.18, reset: false };
    let state = |id: u32, input: u32, start: bool, transitions: Vec<Transition>| State { id, input, transitions, start, passthrough: false, tags: Vec::new() };
    let body = put(&mut nodes, Node::StateMachine { states: vec![
        state(1, ground, true, vec![to(3, vec![is("b_sit", true)]), to(4, vec![is("b_swim", true)]), to(2, vec![is("b_noclip", true)]), to(2, vec![is("b_grounded", false)])]),
        state(2, air, false, vec![to(3, vec![is("b_sit", true)]), to(4, vec![is("b_swim", true)]), to(1, vec![is("b_grounded", true), is("b_noclip", false)])]),
        state(3, sit, false, vec![to(1, vec![is("b_sit", false)])]),
        state(4, swim, false, vec![to(1, vec![is("b_swim", false)])]),
    ] });
    // what it holds poses its arms and chest, over whatever its body does
    let mut held = vec![body];
    for name in HOLDS { held.push(put(&mut nodes, seq(name))); }
    let hold = put(&mut nodes, Node::Selector { children: held, by: id("holdtype"), int_min: 0, blend: 0.2, tags: Vec::new(), tag_until_finished: false });
    let armed = put(&mut nodes, Node::Mask { a: body, b: hold, list: "upper_body".to_string(), param: u32::MAX });
    // and it bends to where it's aiming
    let up = put(&mut nodes, seq("look_up_delta"));
    let level = put(&mut nodes, seq("look_level_delta"));
    let down = put(&mut nodes, seq("look_down_delta"));
    let look = put(&mut nodes, Node::Blend { children: vec![(up, -90.0), (level, 0.0), (down, 90.0)], param: id("aim_body_pitch"), damping: Some(Damping { tension: 200.0, damping: 28.0, speed: 0.0 }) });
    let root = put(&mut nodes, Node::Add { base: armed, additive: look });
    Some(Graph { name: cs(GRAPH), params, nodes, root, tag_names: HashMap::new() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animgraph::Instance;

    fn model_space(m: &ModelFile, pose: &[BonePose]) -> Vec<(Vector, Rotation)> {
        let mut out: Vec<(Vector, Rotation)> = Vec::new();
        for (i, b) in m.bones.iter().enumerate() {
            let p = pose[i];
            out.push(if b.parent < 0 { (p.pos, p.rot) } else { let (pp, pr) = out[b.parent as usize]; (pp + pr * p.pos, (pr * p.rot).normalize()) });
        }
        out
    }

    fn set(g: &Graph, i: &mut Instance, name: &str, value: f32) {
        let k = g.params.iter().position(|p| p.name.to_bytes() == name.as_bytes()).unwrap_or_else(|| panic!("no parameter {name}"));
        i.values[k] = match g.params[k].kind { FLOAT => Variant::float(FLOAT, value), kind => Variant::int(kind, value as i32) };
    }

    fn bone(name: &str) -> usize { BONES.iter().position(|b| b.0 == name).unwrap() }

    #[test]
    fn it_answers_the_default_players_names_only() {
        assert!(is_model("models/citizen_human/human.vmdl"));
        assert!(is_model("models/citizen/citizen.vmdl_c"));
        assert!(is_model("Models/Citizen_Human/citizen_human_male.vmdl"));
        assert!(!is_model("models/citizen_clothes/hat/hat.vmdl"));
        assert!(!is_model("models/dev/box.vmdl"));
        assert!(is_graph("models/citizen/citizen.vanmgrph"));
        assert!(is_graph(GRAPH));
        assert!(!is_graph("models/weapons/x.vanmgrph"));
    }

    #[test]
    fn its_model_has_the_bones_and_attachments_games_ask_for() {
        let m = model("models/citizen_human/human.vmdl").unwrap();
        // asked for by name in sbox-public's and Sandbox's C#
        for b in ["pelvis", "hand_L", "hand_R", "head"] { assert!(m.bone(b) >= 0, "{b}"); }
        for a in ["eyes", "hand_R", "foot_L", "foot_R"] { assert!(m.meshes[0].attachments.iter().any(|x| x.name.to_bytes() == a.as_bytes()), "{a}"); }
        // parents come before their children: animation.rs poses in one pass
        for (i, b) in m.bones.iter().enumerate() { assert!((b.parent as i64) < i as i64); }
        assert_eq!(m.meshes[0].hitbox_sets[0].boxes.iter().filter(|h| h.tags[0].to_bytes() == b"head").count(), 1);
        // its eyes where s&box's player looks from
        let eyes = &m.meshes[0].attachments[0].influences[0];
        assert!(((v(BONES[HEAD].2) + eyes.1).z - 64.0).abs() < 0.01);
    }

    #[test]
    fn its_mesh_is_whole_and_every_vertex_has_a_bone() {
        let (meshes, mask, skins) = draw_meshes("models/citizen/citizen.vmdl").unwrap();
        assert_eq!((meshes.len(), mask, skins[0].len()), (1, 1, 3));
        let vb = &meshes[0].buffers.vertices[0];
        assert_eq!(vb.data.len(), vb.count * vb.stride);
        let skin = vb.skin().unwrap();
        assert!(skin.iter().all(|(b, w)| (b[0] as usize) < N && (w[0] - 1.0).abs() < 1e-6));
        let indices = meshes[0].buffers.indices[0].indices();
        assert!(indices.iter().all(|&i| (i as usize) < vb.count));
        assert_eq!(meshes[0].draws.iter().map(|d| d.index_count).sum::<usize>(), indices.len());
        // it stands on the ground, and no taller than a player
        let b = built();
        assert!(b.min.z > -0.5 && b.min.z < 0.5 && b.max.z > 68.0 && b.max.z < 72.0, "{:?} {:?}", b.min, b.max);
        for m in MATERIALS { assert!(material(m).is_some()); }
    }

    #[test]
    fn every_parameter_the_helper_sets_is_there() {
        let g = graph("models/citizen/citizen.vanmgrph").unwrap();
        // CitizenAnimationHelper.cs and PlayerController, sbox-public
        for name in ["aim_body", "aim_body_pitch", "aim_body_weight", "aim_eyes", "aim_eyes_weight", "aim_head", "aim_head_weight", "b_deploy", "b_grounded",
            "b_jump", "b_long_idle", "b_noclip", "b_sit", "b_swim", "b_weapon_lower", "duck", "hit", "hit_bone", "hit_direction", "hit_offset", "hit_strength",
            "holdtype", "holdtype_handedness", "move_groundspeed", "move_rotationspeed", "move_shuffle", "move_style", "move_x", "move_y", "move_z",
            "scale_height", "sit", "sit_offset_height", "special_movement_states", "speed_reload", "viewer", "voice", "wish_groundspeed", "wish_x", "wish_y", "wish_z"] {
            assert!(g.params.iter().any(|p| p.name.to_bytes() == name.as_bytes()), "{name}");
        }
        let hold = g.params.iter().find(|p| p.name.to_bytes() == b"holdtype").unwrap();
        assert_eq!(hold.options.len(), 9);
        // every sequence its nodes name is one its model has
        let m = model("models/citizen/citizen.vmdl").unwrap();
        for n in g.nodes.values() {
            let names: Vec<&String> = match n { Node::Sequence { name, .. } => vec![name], Node::Blend2D { items, .. } => items.iter().map(|i| &i.0).collect(), _ => Vec::new() };
            for name in names { assert!(m.anim.sequences.iter().any(|s| &s.name == name), "{name}"); }
        }
    }

    #[test]
    fn it_stands_walks_crouches_and_leaves_the_ground() {
        let m = model("models/citizen_human/human.vmdl").unwrap();
        let g = graph(GRAPH).unwrap();
        let mut i = Instance::new(&g);
        let run = |i: &mut Instance, seconds: f32| { let mut p = Vec::new(); for _ in 0..(seconds * 60.0) as usize { p = i.run(&g, &m, 1.0 / 60.0); } p };
        // standing: its head over its feet, its feet on the ground
        let stand = model_space(&m, &run(&mut i, 1.0));
        assert!((stand[bone("head")].0.z - 59.5).abs() < 1.0 && stand[bone("ankle_L")].0.z > 2.0 && stand[bone("ankle_L")].0.z < 4.5, "{:?}", stand[bone("ankle_L")].0);
        // walking forward: its feet pass each other, and each step is heard
        set(&g, &mut i, "move_x", 110.0);
        i.events.clear();
        let mut spread = (f32::MAX, f32::MIN);
        for _ in 0..120 {
            let p = model_space(&m, &i.run(&g, &m, 1.0 / 60.0));
            let d = p[bone("ankle_L")].0.x - p[bone("ankle_R")].0.x;
            spread = (spread.0.min(d), spread.1.max(d));
            // a foot is never far under the ground
            assert!(p[bone("ankle_L")].0.z > -1.0 && p[bone("ankle_R")].0.z > -1.0);
        }
        assert!(spread.0 < -8.0 && spread.1 > 8.0, "{spread:?}");
        let steps: Vec<&str> = i.events.iter().map(|&(clip, e, _)| m.anim.clips[clip].events[e].name.to_str().unwrap()).collect();
        assert!(steps.len() >= 4 && steps.iter().all(|s| *s == "AE_FOOTSTEP"), "{steps:?}");
        // crouched: lower, feet still on the ground
        set(&g, &mut i, "move_x", 0.0);
        set(&g, &mut i, "duck", 1.0);
        let duck = model_space(&m, &run(&mut i, 1.5));
        assert!(duck[bone("head")].0.z < stand[bone("head")].0.z - 12.0, "{}", duck[bone("head")].0.z);
        assert!(duck[bone("ankle_L")].0.z > 1.0 && duck[bone("ankle_L")].0.z < 6.0, "{:?}", duck[bone("ankle_L")].0);
        // off the ground: its legs tucked, and back on it, standing again
        set(&g, &mut i, "duck", 0.0);
        set(&g, &mut i, "b_grounded", 0.0);
        let air = model_space(&m, &run(&mut i, 1.0));
        assert!(air[bone("ankle_L")].0.z > stand[bone("ankle_L")].0.z + 2.0, "{:?}", air[bone("ankle_L")].0);
        set(&g, &mut i, "b_grounded", 1.0);
        let back = model_space(&m, &run(&mut i, 1.5));
        assert!((back[bone("head")].0.z - stand[bone("head")].0.z).abs() < 0.5);
    }

    #[test]
    fn what_it_holds_moves_its_arms_and_where_it_aims_bends_it() {
        let m = model("models/citizen_human/human.vmdl").unwrap();
        let g = graph(GRAPH).unwrap();
        let mut i = Instance::new(&g);
        let run = |i: &mut Instance| { let mut p = Vec::new(); for _ in 0..90 { p = i.run(&g, &m, 1.0 / 60.0); } model_space(&m, &p) };
        let rest = run(&mut i);
        // a pistol: its right hand out in front, at its chest's height; its feet where they were
        set(&g, &mut i, "holdtype", 1.0);
        let pistol = run(&mut i);
        let hand = pistol[bone("hand_R")].0;
        assert!(hand.x > 14.0 && hand.z > 46.0, "{hand:?}");
        assert!((pistol[bone("ankle_L")].0 - rest[bone("ankle_L")].0).length() < 0.1);
        // aiming down brings its head forward and down, up takes it back
        set(&g, &mut i, "aim_body_pitch", 60.0);
        let down = run(&mut i);
        set(&g, &mut i, "aim_body_pitch", -60.0);
        let up = run(&mut i);
        assert!(down[bone("head")].0.x > pistol[bone("head")].0.x + 2.0 && up[bone("head")].0.x < pistol[bone("head")].0.x - 2.0);
        // nothing held again: its arms at its sides
        set(&g, &mut i, "aim_body_pitch", 0.0);
        set(&g, &mut i, "holdtype", 0.0);
        let none = run(&mut i);
        assert!((none[bone("hand_R")].0 - rest[bone("hand_R")].0).length() < 0.5, "{:?} {:?}", none[bone("hand_R")].0, rest[bone("hand_R")].0);
    }
}
