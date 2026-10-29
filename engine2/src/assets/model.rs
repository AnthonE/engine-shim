//! What a compiled model (`.vmdl_c`) holds besides its collision: its
//! skeleton, its meshes and the groups that switch them, the hitboxes and
//! attachments its meshes carry, and the names of its sequences and
//! animations. Laid out as ValveResourceFormat (MIT) reads them: the
//! model's DATA block, each mesh a CRenderMesh (an MDAT block the CTRL
//! block's `embedded_meshes` points at, or a `.vmesh_c` DATA names), and
//! the embedded animation blocks (ASEQ, ANIM).
//!
//! Names are kept as C strings: managed reads them by pointer, again and
//! again, and a model lives as long as the process.
use std::ffi::CString;

use rapier3d::math::{Pose, Rotation, Vector};

use super::kv3::Value;
use super::resource::Resource;

pub struct Bone {
    pub name: CString,
    pub parent: i32,
    /// in its parent's space
    pub pos: Vector,
    pub rot: Rotation,
    /// CModelSkeleton::BoneFlags_t
    pub flags: i64,
    /// its bind pose in the model's space
    pub model: Pose,
}

pub struct HitBox {
    pub name: CString,
    pub surface: CString,
    pub bone: CString,
    /// a box's corners, or a capsule's (a cylinder's) two centres, or a sphere's centre
    pub min: Vector,
    pub max: Vector,
    pub radius: f32,
    /// 0 box, 1 sphere, 2 capsule, 3 cylinder
    pub shape: u8,
    pub bone_hash: u32,
    pub color: [u8; 4],
    pub index: u16,
    pub translation_only: bool,
    pub visible: bool,
    pub tags: Vec<CString>,
}

pub struct HitBoxSet {
    pub name: CString,
    pub source: CString,
    pub boxes: Vec<HitBox>,
}

pub struct Attachment {
    pub name: CString,
    /// bone (empty: the model's root), offset, rotation
    pub influences: Vec<(CString, Vector, Rotation)>,
    pub ignore_rotation: bool,
}

pub struct DrawCall {
    pub primitive: i32,
    pub base_vertex: i32,
    pub vertices: i32,
    pub start_index: i32,
    pub indices: i32,
    pub instances: i32,
    pub uv_density: f32,
}

pub struct SceneObject {
    pub min: Vector,
    pub max: Vector,
    pub draws: Vec<DrawCall>,
}

/// A CRenderMesh.
#[derive(Default)]
pub struct RenderMesh {
    pub name: CString,
    pub objects: Vec<SceneObject>,
    pub hitbox_sets: Vec<HitBoxSet>,
    pub attachments: Vec<Attachment>,
}

impl RenderMesh {
    pub fn bounds(&self) -> Option<(Vector, Vector)> {
        self.objects.iter().map(|o| (o.min, o.max)).reduce(|(a, b), (c, d)| (a.min(c), b.max(d)))
    }

    /// vertices, triangles, draw calls
    pub fn counts(&self) -> (i32, i32, i32) {
        let draws = self.objects.iter().flat_map(|o| &o.draws);
        draws.fold((0, 0, 0), |(v, t, d), c| (v + c.vertices, t + c.indices / 3, d + 1))
    }
}

/// A body group: its name and its choices, each a mesh group (name, mask).
pub struct BodyPart {
    pub name: CString,
    pub choices: Vec<(CString, u64)>,
}

#[derive(Default)]
pub struct ModelFile {
    pub bones: Vec<Bone>,
    /// by mesh index; a mesh that couldn't be read is empty
    pub meshes: Vec<RenderMesh>,
    pub mesh_group_masks: Vec<u64>,
    pub lod_masks: Vec<u64>,
    pub mesh_groups: Vec<CString>,
    pub default_mask: u64,
    pub body_parts: Vec<BodyPart>,
    pub lod_switch: Vec<f32>,
    pub material_groups: Vec<CString>,
    pub sequences: Vec<CString>,
    pub animations: Vec<CString>,
    /// the animations decoded, for playing
    pub anim: super::anim::Animations,
}

impl ModelFile {
    pub fn bone(&self, name: &str) -> i32 {
        self.bones.iter().position(|b| b.name.to_bytes().eq_ignore_ascii_case(name.as_bytes())).map(|i| i as i32).unwrap_or(-1)
    }

    /// The hitbox sets of mesh `mesh`, or, if it has none, of the first mesh
    /// that has any (one mesh carries them, as with attachments).
    pub fn hitbox_sets(&self, mesh: i32) -> &[HitBoxSet] {
        if let Some(m) = self.meshes.get(mesh.max(0) as usize).filter(|m| !m.hitbox_sets.is_empty()) { return &m.hitbox_sets; }
        self.meshes.iter().find(|m| !m.hitbox_sets.is_empty()).map(|m| &m.hitbox_sets[..]).unwrap_or(&[])
    }

    /// The attachments of the first mesh that has any.
    pub fn attachments(&self) -> &[Attachment] {
        self.meshes.iter().find(|m| !m.attachments.is_empty()).map(|m| &m.attachments[..]).unwrap_or(&[])
    }

    /// An attachment's place in the model's space, its bones in bind pose.
    pub fn attachment_pose(&self, a: &Attachment) -> Pose {
        let Some((bone, offset, rot)) = a.influences.first() else { return Pose::IDENTITY };
        let local = Pose::from_parts(*offset, *rot);
        match self.bone(&bone.to_string_lossy()) {
            -1 => local,
            i => self.bones[i as usize].model * local,
        }
    }

    /// The meshes of LOD 0 (all of them if none says).
    fn lod0(&self) -> impl Iterator<Item = &RenderMesh> {
        let any = self.lod_masks.iter().any(|&m| m & 1 != 0);
        self.meshes.iter().enumerate().filter(move |(i, _)| !any || self.lod_masks.get(*i).is_some_and(|m| m & 1 != 0)).map(|(_, m)| m)
    }

    /// What LOD 0's meshes cover.
    pub fn render_bounds(&self) -> Option<(Vector, Vector)> {
        self.lod0().filter_map(RenderMesh::bounds).reduce(|(a, b), (c, d)| (a.min(c), b.max(d)))
    }
}

fn cs(s: &str) -> CString {
    CString::new(s.replace('\0', "")).unwrap()
}

fn str_of<'a>(v: &'a Value, name: &str) -> &'a str {
    v.get(name).and_then(Value::as_str).unwrap_or("")
}

fn vec3(v: Option<&Value>) -> Vector {
    let f = v.map(Value::floats).unwrap_or_default();
    Vector::new(f.first().copied().unwrap_or(0.0), f.get(1).copied().unwrap_or(0.0), f.get(2).copied().unwrap_or(0.0))
}

fn quat(v: Option<&Value>) -> Rotation {
    let f = v.map(Value::floats).unwrap_or_default();
    if f.len() < 4 { return Rotation::IDENTITY; }
    let q = Rotation::from_xyzw(f[0], f[1], f[2], f[3]);
    if q.length_squared() < 1e-12 { Rotation::IDENTITY } else { q.normalize() }
}

fn masks(v: Option<&Value>) -> Vec<u64> {
    v.map(Value::as_array).unwrap_or(&[]).iter().map(|m| m.as_i64().unwrap_or(0) as u64).collect()
}

fn truthy(v: &Value, name: &str) -> bool {
    v.get(name).and_then(Value::as_i64).unwrap_or(0) != 0
}

/// A model's skeleton (`m_modelSkeleton`), each bone's bind pose carried
/// down from its parents. A parent always comes before its children.
pub fn skeleton(s: &Value) -> Vec<Bone> {
    let list = |n: &str| s.get(n).map(Value::as_array).unwrap_or(&[]);
    let mut bones: Vec<Bone> = Vec::new();
    for (i, name) in list("m_boneName").iter().enumerate() {
        let parent = list("m_nParent").get(i).and_then(Value::as_i64).unwrap_or(-1) as i32;
        let pos = vec3(list("m_bonePosParent").get(i));
        let rot = quat(list("m_boneRotParent").get(i));
        let local = Pose::from_parts(pos, rot);
        let model = match bones.get(parent.max(0) as usize) {
            Some(p) if parent >= 0 && (parent as usize) < i => p.model * local,
            _ => local,
        };
        let flags = list("m_nFlag").get(i).and_then(Value::as_i64).unwrap_or(0);
        bones.push(Bone { name: cs(name.as_str().unwrap_or("")), parent, pos, rot, flags, model });
    }
    bones
}

/// A CRenderMesh's own data (an MDAT block, or a `.vmesh_c`'s DATA).
pub fn render_mesh(name: &str, m: &Value) -> RenderMesh {
    let objects = m.get("m_sceneObjects").map(Value::as_array).unwrap_or(&[]).iter().map(|o| SceneObject {
        min: vec3(o.get("m_vMinBounds")),
        max: vec3(o.get("m_vMaxBounds")),
        draws: o.get("m_drawCalls").map(Value::as_array).unwrap_or(&[]).iter().map(|d| DrawCall {
            primitive: match d.get("m_nPrimitiveType") {
                Some(Value::String(s)) => PRIMITIVES.iter().position(|p| p == s).map(|i| i as i32).unwrap_or(5),
                Some(v) => v.as_i64().unwrap_or(5) as i32,
                None => 5,
            },
            base_vertex: d.i64("m_nBaseVertex") as i32,
            vertices: d.i64("m_nVertexCount") as i32,
            start_index: d.i64("m_nStartIndex") as i32,
            indices: d.i64("m_nIndexCount") as i32,
            instances: d.i64("m_nInstanceCount") as i32,
            uv_density: d.f32("m_flUvDensity"),
        }).collect(),
    }).collect();
    let hitbox_sets = m.get("m_hitboxsets").map(Value::as_array).unwrap_or(&[]).iter().map(|kv| {
        let set = kv.get("value").unwrap_or(kv);
        let boxes = set.get("m_HitBoxes").or(set.get("m_hitboxes")).map(Value::as_array).unwrap_or(&[]);
        HitBoxSet {
            name: cs(if str_of(set, "m_name").is_empty() { str_of(kv, "key") } else { str_of(set, "m_name") }),
            source: cs(str_of(set, "m_SourceFilename")),
            boxes: boxes.iter().enumerate().map(|(i, b)| {
                let color = b.get("m_cRenderColor").map(Value::bytes).unwrap_or_default();
                HitBox {
                    name: cs(str_of(b, "m_name")),
                    surface: cs(str_of(b, "m_sSurfaceProperty")),
                    bone: cs(str_of(b, "m_sBoneName")),
                    min: vec3(b.get("m_vMinBounds")),
                    max: vec3(b.get("m_vMaxBounds")),
                    radius: b.f32("m_flShapeRadius"),
                    shape: b.i64("m_nShapeType") as u8,
                    bone_hash: b.i64("m_nBoneNameHash") as u32,
                    color: core::array::from_fn(|k| color.get(k).copied().unwrap_or(0)),
                    index: b.get("m_nHitBoxIndex").and_then(Value::as_i64).unwrap_or(i as i64) as u16,
                    translation_only: truthy(b, "m_bTranslationOnly"),
                    visible: truthy(b, "m_bVisible"),
                    tags: b.get("m_HitboxTags").map(Value::as_array).unwrap_or(&[]).iter().filter_map(Value::as_str).filter(|t| !t.is_empty()).map(cs).collect(),
                }
            }).collect(),
        }
    }).collect();
    let attachments = m.get("m_attachments").map(Value::as_array).unwrap_or(&[]).iter().map(|kv| {
        let a = kv.get("value").unwrap_or(kv);
        let n = a.i64("m_nInfluences").clamp(0, 3) as usize;
        let names = a.get("m_influenceNames").map(Value::as_array).unwrap_or(&[]);
        let offsets = a.get("m_vInfluenceOffsets").map(Value::as_array).unwrap_or(&[]);
        let rots = a.get("m_vInfluenceRotations").map(Value::as_array).unwrap_or(&[]);
        Attachment {
            name: cs(if str_of(a, "m_name").is_empty() { str_of(kv, "key") } else { str_of(a, "m_name") }),
            influences: (0..n).map(|i| (cs(names.get(i).and_then(Value::as_str).unwrap_or("")), vec3(offsets.get(i)), quat(rots.get(i)))).collect(),
            ignore_rotation: truthy(a, "m_bIgnoreRotation"),
        }
    }).collect();
    RenderMesh { name: cs(name), objects, hitbox_sets, attachments }
}

const PRIMITIVES: [&str; 10] = [
    "RENDER_PRIM_POINTS", "RENDER_PRIM_LINES", "RENDER_PRIM_LINES_WITH_ADJACENCY", "RENDER_PRIM_LINE_STRIP",
    "RENDER_PRIM_LINE_STRIP_WITH_ADJACENCY", "RENDER_PRIM_TRIANGLES", "RENDER_PRIM_TRIANGLES_WITH_ADJACENCY",
    "RENDER_PRIM_TRIANGLE_STRIP", "RENDER_PRIM_TRIANGLE_STRIP_WITH_ADJACENCY", "RENDER_PRIM_INSTANCED_QUADS",
];

/// Body groups from the mesh groups' names: a choice is `<group>_@<n>`; a
/// mesh group named otherwise is a group of one choice.
fn body_parts(groups: &[CString]) -> Vec<BodyPart> {
    let mut parts: Vec<BodyPart> = Vec::new();
    for (i, g) in groups.iter().enumerate() {
        let g = g.to_string_lossy();
        let (part, choice) = match g.rsplit_once("_@") {
            Some((p, c)) if c.chars().all(|c| c.is_ascii_digit()) => (p.to_string(), g.to_string()),
            _ => (g.to_string(), g.to_string()),
        };
        let mask = 1u64 << (i as u32).min(63);
        match parts.iter_mut().find(|p| p.name.to_bytes() == part.as_bytes()) {
            Some(p) => p.choices.push((cs(&choice), mask)),
            None => parts.push(BodyPart { name: cs(&part), choices: vec![(cs(&choice), mask)] }),
        }
    }
    parts
}

/// A compiled model, read. `read_compiled` finds the meshes it refers to
/// by name.
pub fn read(r: &Resource, read_compiled: impl Fn(&str) -> Option<Vec<u8>>) -> Result<ModelFile, String> {
    let data = r.kv3("DATA").ok_or("no DATA")?.map_err(|e| e.to_string())?;
    let mut m = ModelFile {
        bones: data.get("m_modelSkeleton").map(skeleton).unwrap_or_default(),
        mesh_group_masks: masks(data.get("m_refMeshGroupMasks")),
        lod_masks: masks(data.get("m_refLODGroupMasks")),
        mesh_groups: data.get("m_meshGroups").map(Value::as_array).unwrap_or(&[]).iter().map(|g| cs(g.as_str().unwrap_or(""))).collect(),
        default_mask: data.i64("m_nDefaultMeshGroupMask") as u64,
        lod_switch: data.get("m_lodGroupSwitchDistances").map(Value::floats).unwrap_or_default(),
        material_groups: data.get("m_materialGroups").map(Value::as_array).unwrap_or(&[]).iter().map(|g| cs(str_of(g, "m_name"))).collect(),
        ..Default::default()
    };
    m.body_parts = body_parts(&m.mesh_groups);

    // meshes: by name from files, or embedded (their index says which slot)
    let refs = data.get("m_refMeshes").map(Value::as_array).unwrap_or(&[]);
    m.meshes = (0..refs.len().max(m.mesh_group_masks.len())).map(|_| RenderMesh::default()).collect();
    for (i, name) in refs.iter().enumerate() {
        let Some(name) = name.as_str().filter(|n| !n.is_empty()) else { continue };
        let Some(Ok(mesh)) = read_compiled(name).and_then(|b| Resource::parse(b).ok()).map(|r| r.kv3("DATA").map(|d| d.map_err(|e| e.to_string()))).flatten() else { continue };
        m.meshes[i] = render_mesh(name, &mesh);
    }
    if let Some(Ok(ctrl)) = r.kv3("CTRL") {
        for e in ctrl.get("embedded_meshes").map(Value::as_array).unwrap_or(&[]) {
            let (name, index, block) = match e.get("name") {
                Some(_) => (str_of(e, "name"), e.i64("mesh_index"), e.i64("data_block")),
                None => (str_of(e, "m_Name"), e.i64("m_nMeshIndex"), e.i64("m_nDataBlock")),
            };
            let Some(&(_, at, n)) = r.blocks.get(block.max(0) as usize) else { continue };
            let Ok(mesh) = super::kv3::read(&r.bytes[at..at + n]) else { continue };
            let i = index.max(0) as usize;
            if m.meshes.len() <= i { m.meshes.resize_with(i + 1, RenderMesh::default); }
            m.meshes[i] = render_mesh(name, &mesh);
        }
    }
    // an older model keeps its one mesh in MDAT
    if m.meshes.iter().all(|x| x.objects.is_empty()) {
        if let Some(Ok(mesh)) = r.kv3("MDAT") {
            if m.meshes.is_empty() { m.meshes.push(RenderMesh::default()); }
            m.meshes[0] = render_mesh("", &mesh);
        }
    }

    // sequences and animations embedded in the model
    if let Some(Ok(seq)) = r.kv3("ASEQ") {
        for s in seq.get("m_localS1SeqDescArray").map(Value::as_array).unwrap_or(&[]) {
            let hidden = s.get("m_flags").is_some_and(|f| truthy(f, "m_bHidden"));
            if !hidden { m.sequences.push(cs(str_of(s, "m_sName"))); }
        }
    }
    if let Some(Ok(anim)) = r.kv3("ANIM") {
        m.animations = anim.get("m_animArray").map(Value::as_array).unwrap_or(&[]).iter().map(|a| cs(str_of(a, "m_name"))).collect();
    }
    let names: Vec<String> = m.bones.iter().map(|b| b.name.to_string_lossy().into_owned()).collect();
    m.anim = super::anim::read(r, &names);
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(fields: &[(&str, Value)]) -> Value {
        Value::Object(fields.iter().map(|(k, v)| (k.to_string(), v.clone())).collect())
    }
    fn nums(f: &[f64]) -> Value {
        Value::Array(f.iter().map(|&x| Value::Double(x)).collect())
    }

    #[test]
    fn bones_carry_their_parents_pose() {
        // a root at z 10, a child 5 along the root's x, the root turned 90° about z
        let q = (std::f64::consts::FRAC_PI_4).sin();
        let s = obj(&[
            ("m_boneName", Value::Array(vec![Value::String("root".into()), Value::String("arm".into())])),
            ("m_nParent", Value::Array(vec![Value::Int(-1), Value::Int(0)])),
            ("m_bonePosParent", Value::Array(vec![nums(&[0.0, 0.0, 10.0]), nums(&[5.0, 0.0, 0.0])])),
            ("m_boneRotParent", Value::Array(vec![nums(&[0.0, 0.0, q, q]), nums(&[0.0, 0.0, 0.0, 1.0])])),
            ("m_nFlag", Value::Array(vec![Value::Int(64), Value::Int(64)])),
        ]);
        let b = skeleton(&s);
        assert_eq!(b.len(), 2);
        assert_eq!(b[1].parent, 0);
        // the arm is 5 along the root's turned x: world +y
        assert!((b[1].model.translation - Vector::new(0.0, 5.0, 10.0)).length() < 1e-4, "{:?}", b[1].model.translation);
        assert_eq!(b[1].pos, Vector::new(5.0, 0.0, 0.0), "parent space kept");
    }

    #[test]
    fn mesh_groups_make_body_parts() {
        let groups: Vec<CString> = ["USP_@0", "magazine_@0", "barrel_@0", "barrel_@1", "plain"].iter().map(|s| cs(s)).collect();
        let parts = body_parts(&groups);
        let names: Vec<String> = parts.iter().map(|p| p.name.to_string_lossy().into_owned()).collect();
        assert_eq!(names, ["USP", "magazine", "barrel", "plain"]);
        assert_eq!(parts[2].choices.iter().map(|c| c.1).collect::<Vec<_>>(), [4, 8]);
    }

    #[test]
    fn a_downloaded_model_has_its_skeleton_hitboxes_and_sequences() {
        // s&box's own download cache, if this machine has one
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let dir = std::path::Path::new(&dir).join("models/first_person");
        let Some(file) = std::fs::read_dir(&dir).ok().and_then(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| p.to_string_lossy().contains("v_first_person_arms_human."))) else { return };
        let r = Resource::parse(std::fs::read(file).unwrap()).unwrap();
        let m = read(&r, |_| None).unwrap();
        assert_eq!(m.bones.len(), 70);
        assert_eq!(m.bones[0].name.to_str().unwrap(), "root");
        let hand = m.bone("hand_R");
        assert!(hand > 0);
        // a hand hangs off its arm's end: the chain's lengths add up
        assert!(m.bones[hand as usize].model.translation.length() > 20.0);
        let sets = m.hitbox_sets(0);
        assert_eq!(sets[0].name.to_str().unwrap(), "default");
        let arm = sets[0].boxes.iter().find(|b| b.name.to_str() == Ok("arm_upper_R")).unwrap();
        assert_eq!(arm.shape, 2);
        assert!(m.bone(&arm.bone.to_string_lossy()) >= 0);
        assert!(m.sequences.iter().any(|s| s.to_str() == Ok("Punching_Deploy")));
        assert_eq!(m.animations.first().map(|a| a.to_str().unwrap()), Some("bindPose"));
        assert!(m.render_bounds().is_some());
    }
}
