//! A model's animations, decoded: the ANIM block's clips (each a run of
//! frames in blocks, each block a list of compressed segments, each segment
//! some bones' positions or rotations over its frames) and the ASEQ block's
//! sequences that play them. As ValveResourceFormat (MIT) decodes them
//! (AnimationSegmentTable, the SegmentDecoders, SequenceAnimation); the
//! AGRP block's decode key says which bone a segment's element is.
//!
//! Only bones move here: morph (flex) and user channels are skipped.
use std::collections::HashMap;

use rapier3d::math::{Rotation, Vector};

use super::kv3::Value;
use super::resource::Resource;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Channel {
    Position,
    Angle,
    Scale,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Codec {
    StaticFullVector3,
    StaticVector3,
    StaticQuaternion,
    FullVector3,
    DeltaVector3,
    AnimVector3,
    AnimQuaternion,
    FullQuaternion,
    StaticFloat,
    FullFloat,
}

/// One segment: some of a channel's elements over a block's frames.
struct Segment {
    codec: Codec,
    channel: Channel,
    /// how many elements a frame holds
    count: usize,
    /// (the element's place in a frame, the bone it moves)
    elements: Vec<(usize, usize)>,
    data: Vec<u8>,
}

/// A bone's pose in its parent's space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BonePose {
    pub pos: Vector,
    pub rot: Rotation,
    pub scale: f32,
}

impl BonePose {
    pub fn lerp(&self, o: &BonePose, t: f32) -> BonePose {
        BonePose { pos: self.pos.lerp(o.pos, t), rot: self.rot.slerp(o.rot, t), scale: self.scale + (o.scale - self.scale) * t }
    }
}

struct Block {
    start: usize,
    end: usize,
    segments: Vec<usize>,
}

pub struct Clip {
    pub name: String,
    pub fps: f32,
    pub frames: usize,
    pub looping: bool,
    /// a delta, laid over the bind pose rather than replacing it
    pub delta: bool,
    blocks: Vec<Block>,
}

impl Clip {
    /// From one frame to the last: the clip's length in seconds.
    pub fn duration(&self) -> f32 {
        if self.frames > 1 && self.fps > 0.0 { (self.frames - 1) as f32 / self.fps } else { 0.0 }
    }
}

/// A sequence: what plays a clip under a name.
pub struct Sequence {
    pub name: String,
    pub clip: usize,
    pub looping: bool,
}

#[derive(Default)]
pub struct Animations {
    segments: Vec<Option<Segment>>,
    pub clips: Vec<Clip>,
    pub sequences: Vec<Sequence>,
    /// bone masks (weight lists): the weight each bone is named with, by
    /// the model's bone index (one not named takes its parent's)
    pub masks: Vec<(String, Vec<Option<f32>>)>,
}

fn f32_at(b: &[u8], i: usize) -> f32 {
    b.get(i..i + 4).map(|x| f32::from_le_bytes(x.try_into().unwrap())).unwrap_or(0.0)
}

fn half_at(b: &[u8], i: usize) -> f32 {
    b.get(i..i + 2).map(|x| half(u16::from_le_bytes(x.try_into().unwrap()))).unwrap_or(0.0)
}

/// An IEEE half as a float.
fn half(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = ((h >> 10) & 0x1f) as i32;
    let frac = (h & 0x3ff) as f32;
    sign * match exp {
        0 => frac * 2f32.powi(-24),
        31 => if frac == 0.0 { f32::INFINITY } else { f32::NAN },
        e => (1.0 + frac / 1024.0) * 2f32.powi(e - 15),
    }
}

fn vec_at(b: &[u8], i: usize) -> Vector {
    Vector::new(f32_at(b, i), f32_at(b, i + 4), f32_at(b, i + 8))
}

fn half3_at(b: &[u8], i: usize) -> Vector {
    Vector::new(half_at(b, i), half_at(b, i + 2), half_at(b, i + 4))
}

/// A rotation packed in six bytes: three components of 15 bits, the
/// largest one left out and its place in the sign bits.
fn quat48(b: &[u8]) -> Rotation {
    if b.len() < 6 { return Rotation::IDENTITY; }
    let i = |lo: u8, hi: u8| lo as i32 + (((hi & 63) as i32) << 8);
    let c = std::f32::consts::FRAC_1_SQRT_2 / 16384.0;
    let comp = |lo: u8, hi: u8| if hi & 64 == 0 { c * (i(lo, hi) - 16384) as f32 } else { c * i(lo, hi) as f32 };
    let (x, y, z) = (comp(b[0], b[1]), comp(b[2], b[3]), comp(b[4], b[5]));
    let mut w = (1.0 - x * x - y * y - z * z).max(0.0).sqrt();
    if b[5] & 128 != 0 { w = -w; }
    let q = match (b[1] & 128 != 0, b[3] & 128 != 0) {
        (true, true) => Rotation::from_xyzw(y, z, w, x),
        (true, false) => Rotation::from_xyzw(z, w, x, y),
        (false, true) => Rotation::from_xyzw(w, x, y, z),
        (false, false) => Rotation::from_xyzw(x, y, z, w),
    };
    q.normalize()
}

impl Segment {
    /// Frame `f` of the block (counted from its start) into `pose`.
    fn read(&self, f: usize, pose: &mut [BonePose]) {
        let d = &self.data;
        for &(e, bone) in &self.elements {
            let Some(p) = pose.get_mut(bone) else { continue };
            let at = f * self.count + e;
            match (self.codec, self.channel) {
                (Codec::StaticFullVector3, Channel::Position) => p.pos = vec_at(d, e * 12),
                (Codec::FullVector3, Channel::Position) => p.pos = vec_at(d, at * 12),
                (Codec::StaticVector3, Channel::Position) => p.pos = half3_at(d, e * 6),
                (Codec::AnimVector3, Channel::Position) => p.pos = half3_at(d, at * 6),
                (Codec::DeltaVector3, Channel::Position) => p.pos = vec_at(d, e * 12) + half3_at(d, self.count * 12 + at * 6),
                (Codec::StaticQuaternion, Channel::Angle) => p.rot = quat48(d.get(e * 6..e * 6 + 6).unwrap_or(&[])),
                (Codec::AnimQuaternion, Channel::Angle) => p.rot = quat48(d.get(at * 6..at * 6 + 6).unwrap_or(&[])),
                (Codec::FullQuaternion, Channel::Angle) => {
                    let q = Rotation::from_xyzw(f32_at(d, at * 16), f32_at(d, at * 16 + 4), f32_at(d, at * 16 + 8), f32_at(d, at * 16 + 12));
                    if q.length_squared() > 1e-12 { p.rot = q.normalize(); }
                }
                (Codec::StaticFloat, Channel::Scale) => p.scale = f32_at(d, e * 4),
                (Codec::FullFloat, Channel::Scale) => p.scale = f32_at(d, at * 4),
                _ => {}
            }
        }
    }
}

fn names(v: Option<&Value>) -> Vec<String> {
    v.map(Value::as_array).unwrap_or(&[]).iter().map(|x| x.as_str().unwrap_or("").to_string()).collect()
}

fn flag(v: &Value, name: &str) -> bool {
    v.get("m_flags").and_then(|f| f.get(name)).and_then(Value::as_i64).unwrap_or(0) != 0
}

/// A model's embedded animations, for its skeleton's bones (by name).
pub fn read(r: &Resource, bones: &[String]) -> Animations {
    let (Some(Ok(anim)), Some(Ok(group))) = (r.kv3("ANIM"), r.kv3("AGRP")) else { return Animations::default() };
    let mut a = Animations::default();
    let mut masks: Vec<(String, Vec<Option<f32>>)> = Vec::new();

    // each data channel: its elements' bones in the model's skeleton
    let by_name: HashMap<String, usize> = bones.iter().enumerate().rev().map(|(i, n)| (n.to_ascii_lowercase(), i)).collect();
    let channels: Vec<Option<(Channel, HashMap<usize, usize>)>> = group.get("m_decodeKey").and_then(|k| k.get("m_dataChannelArray")).map(Value::as_array).unwrap_or(&[]).iter().map(|c| {
        let class = c.get("m_szChannelClass").and_then(Value::as_str).unwrap_or("");
        let channel = match (class, c.get("m_szVariableName").and_then(Value::as_str).unwrap_or("")) {
            ("MorphChannel" | "UserChannel", _) => return None,
            (_, "Position") => Channel::Position,
            (_, "Angle") => Channel::Angle,
            (_, "Scale") => Channel::Scale,
            _ => return None,
        };
        let element_names = names(c.get("m_szElementNameArray"));
        let indices = c.get("m_nElementIndexArray").map(Value::as_array).unwrap_or(&[]);
        // element index -> bone
        let mut map = HashMap::new();
        for (n, i) in element_names.iter().zip(indices) {
            if let (Some(&bone), Some(i)) = (by_name.get(&n.to_ascii_lowercase()), i.as_i64()) { map.entry(i as usize).or_insert(bone); }
        }
        Some((channel, map))
    }).collect();

    let decoders = anim.get("m_decoderArray").map(Value::as_array).unwrap_or(&[]).iter().map(|d| d.get("m_szName").and_then(Value::as_str).unwrap_or("").to_string()).collect::<Vec<_>>();
    for s in anim.get("m_segmentArray").map(Value::as_array).unwrap_or(&[]) {
        let c = s.get("m_container").map(Value::bytes).unwrap_or_default();
        let channel = channels.get(s.i64("m_nLocalChannel").max(0) as usize).and_then(Option::as_ref);
        let segment = (|| {
            let (channel, map) = channel?;
            let u16_at = |i: usize| c.get(i..i + 2).map(|x| i16::from_le_bytes(x.try_into().unwrap()));
            let codec = match decoders.get(u16_at(0)? as usize)?.as_str() {
                "CCompressedStaticFullVector3" => Codec::StaticFullVector3,
                "CCompressedStaticVector3" => Codec::StaticVector3,
                "CCompressedStaticQuaternion" => Codec::StaticQuaternion,
                "CCompressedFullVector3" => Codec::FullVector3,
                "CCompressedDeltaVector3" => Codec::DeltaVector3,
                "CCompressedAnimVector3" => Codec::AnimVector3,
                "CCompressedAnimQuaternion" => Codec::AnimQuaternion,
                "CCompressedFullQuaternion" => Codec::FullQuaternion,
                "CCompressedStaticFloat" => Codec::StaticFloat,
                "CCompressedFullFloat" => Codec::FullFloat,
                _ => return None,
            };
            let count = u16_at(4)?.max(0) as usize;
            let end = 8 + count * 2;
            let elements = (0..count).filter_map(|k| {
                let element = u16_at(8 + k * 2)?;
                map.get(&(element.max(0) as usize)).map(|&bone| (k, bone))
            }).collect();
            Some(Segment { codec, channel: *channel, count, elements, data: c.get(end..)?.to_vec() })
        })();
        a.segments.push(segment);
    }

    for c in anim.get("m_animArray").map(Value::as_array).unwrap_or(&[]) {
        let data = c.get("m_pData").unwrap_or(c);
        let frames = data.get("m_nFrames").or(c.get("numframes")).and_then(Value::as_i64).unwrap_or(0).max(0) as usize;
        let blocks = data.get("m_frameblockArray").map(Value::as_array).unwrap_or(&[]).iter().map(|b| Block {
            start: b.i64("m_nStartFrame").max(0) as usize,
            end: b.i64("m_nEndFrame").max(0) as usize,
            segments: b.get("m_segmentIndexArray").map(Value::as_array).unwrap_or(&[]).iter().filter_map(Value::as_i64).map(|i| i as usize).collect(),
        }).collect();
        a.clips.push(Clip {
            name: c.get("m_name").and_then(Value::as_str).unwrap_or("").to_string(),
            fps: c.f32("fps"),
            frames,
            looping: flag(c, "m_bLooping"),
            delta: flag(c, "m_bDelta"),
            blocks,
        });
    }

    // sequences: each plays the clip its first reference names (a blend's
    // first stands in for it), or the clip another sequence plays
    let clip_of: HashMap<String, usize> = a.clips.iter().enumerate().map(|(i, c)| (c.name.to_ascii_lowercase(), i)).collect();
    if let Some(Ok(seq)) = r.kv3("ASEQ") {
        let bone_names = names(seq.get("m_localBoneNameArray"));
        for m in seq.get("m_localBoneMaskArray").map(Value::as_array).unwrap_or(&[]) {
            let Some(name) = m.get("m_sName").and_then(Value::as_str) else { continue };
            let mut named: Vec<Option<f32>> = vec![None; bones.len()];
            let weights = m.get("m_flBoneWeightArray").map(Value::floats).unwrap_or_default();
            for (k, i) in m.get("m_nLocalBoneArray").map(Value::as_array).unwrap_or(&[]).iter().filter_map(Value::as_i64).enumerate() {
                let bone = bone_names.get(i.max(0) as usize).and_then(|n| by_name.get(&n.to_ascii_lowercase()));
                if let (Some(&b), Some(&w)) = (bone, weights.get(k)) { named[b] = Some(w); }
            }
            masks.push((name.to_string(), named));
        }
        let local = names(seq.get("m_localSequenceNameArray"));
        let descs = seq.get("m_localS1SeqDescArray").map(Value::as_array).unwrap_or(&[]);
        let first = |d: &Value| d.get("m_fetch").and_then(|f| f.get("m_localReferenceArray")).and_then(|r| r.as_array().first()).and_then(Value::as_i64).and_then(|i| local.get(i.max(0) as usize)).map(|n| n.to_ascii_lowercase());
        let direct: HashMap<String, usize> = descs.iter().filter_map(|d| Some((d.get("m_sName")?.as_str()?.to_ascii_lowercase(), *clip_of.get(&first(d)?)?))).collect();
        for d in descs {
            let Some(name) = d.get("m_sName").and_then(Value::as_str) else { continue };
            let Some(r) = first(d) else { continue };
            let Some(&clip) = clip_of.get(&r).or_else(|| direct.get(&r)) else { continue };
            // a sequence loops if it says so, or the clip it plays does
            a.sequences.push(Sequence { name: name.to_string(), clip, looping: flag(d, "m_bLooping") || a.clips[clip].looping });
        }
    }
    a.masks = masks;
    a
}

impl Animations {
    #[cfg(test)]
    pub fn sequence(&self, name: &str) -> Option<&Sequence> {
        self.sequences.iter().find(|s| s.name.eq_ignore_ascii_case(name))
    }

    /// Frame `f` of a clip, over `pose` (the bind pose, in parent space).
    fn frame(&self, clip: &Clip, f: usize, pose: &mut [BonePose]) {
        for b in clip.blocks.iter().filter(|b| f >= b.start && f <= b.end) {
            for &s in &b.segments {
                if let Some(Some(seg)) = self.segments.get(s) { seg.read(f - b.start, pose); }
            }
        }
    }

    /// A clip at `cycle` (0 to 1 over its length), between its two nearest
    /// frames; a delta laid over `bind`. `bind` is each bone in its parent's
    /// space.
    pub fn sample(&self, clip: usize, cycle: f32, bind: &[BonePose]) -> Vec<BonePose> {
        let Some(c) = self.clips.get(clip) else { return bind.to_vec() };
        let base = if c.delta { vec![BonePose { pos: Vector::ZERO, rot: Rotation::IDENTITY, scale: 1.0 }; bind.len()] } else { bind.to_vec() };
        if c.frames == 0 { return bind.to_vec(); }
        let at = cycle.clamp(0.0, 1.0) * (c.frames - 1) as f32;
        let f0 = (at.floor() as usize).min(c.frames - 1);
        let f1 = (f0 + 1).min(c.frames - 1);
        let (mut a, mut b) = (base.clone(), base);
        self.frame(c, f0, &mut a);
        self.frame(c, f1, &mut b);
        let t = at - f0 as f32;
        let mixed = a.iter().zip(&b).map(|(x, y)| x.lerp(y, t));
        if c.delta {
            mixed.zip(bind).map(|(d, b)| BonePose { pos: b.pos + d.pos, rot: (b.rot * d.rot).normalize(), scale: b.scale * d.scale }).collect()
        } else {
            mixed.collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halves_and_packed_rotations_decode() {
        assert_eq!(half(0x3c00), 1.0);
        assert_eq!(half(0xc000), -2.0);
        assert_eq!(half(0x3555), 0.333_251_95);
        // x, y, z all at their middle (16384 with bit 6 clear: zero), w the rest: no rotation
        let q = quat48(&[0, 0x40, 0, 0x40, 0, 0x40]);
        assert!(q.abs_diff_eq(Rotation::IDENTITY, 1e-4), "{q:?}");
    }

    #[test]
    fn a_downloaded_models_sequence_moves_its_bones() {
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let dir = std::path::Path::new(&dir).join("models/first_person");
        let Some(file) = std::fs::read_dir(&dir).ok().and_then(|d| d.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| p.to_string_lossy().contains("v_first_person_arms_human."))) else { return };
        let r = Resource::parse(std::fs::read(file).unwrap()).unwrap();
        let m = super::super::model::read(&r, |_| None).unwrap();
        let bones: Vec<String> = m.bones.iter().map(|b| b.name.to_string_lossy().into_owned()).collect();
        let a = read(&r, &bones);
        assert_eq!(a.clips.len(), m.animations.len());
        assert!(a.segments.iter().filter(|s| s.is_some()).count() > 0);
        let bind: Vec<BonePose> = m.bones.iter().map(|b| BonePose { pos: b.pos, rot: b.rot, scale: 1.0 }).collect();
        // the bind pose sequence is the bind pose
        let bp = a.sequence("bindPose").unwrap();
        let pose = a.sample(bp.clip, 0.0, &bind);
        for (p, b) in pose.iter().zip(&bind) {
            assert!((p.pos - b.pos).length() < 0.05 && p.rot.abs_diff_eq(b.rot, 0.01) || p.rot.abs_diff_eq(-b.rot, 0.01), "{p:?} {b:?}");
        }
        // a punch moves the arm from where it rests, and through the punch
        let punch = a.sequence("Punching_Right_Hit_01").unwrap();
        assert!(a.clips[punch.clip].duration() > 0.1);
        let arm = m.bone("arm_upper_R") as usize;
        let poses: Vec<BonePose> = (0..10).map(|k| a.sample(punch.clip, k as f32 / 9.0, &bind)[arm]).collect();
        assert!(poses.iter().any(|p| !p.rot.abs_diff_eq(bind[arm].rot, 0.05)), "it moves");
        assert!(poses.iter().any(|p| !p.rot.abs_diff_eq(poses[0].rot, 0.05)), "over time");
    }
}
