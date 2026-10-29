//! Animation graphs (`.vanmgrph`): what a SkinnedModelRenderer poses its
//! model with when it has one. The compiled graph is the editor's own, as
//! KV3: its parameters (bools, enums, ints, floats, vectors, rotations,
//! which managed sets), and its nodes by id, each naming the nodes it takes
//! its pose from. Read here and run per scene object: each step, from the
//! root down, every node the pose depends on advances and gives its pose.
//!
//! Nodes run: sequences and single frames; 1D and 2D blends (their inputs
//! damped as the graph says); adds and subtracts (additive layers); bone
//! masks (a model's weight lists); choices; selectors by bool, enum or int;
//! state machines (states, transitions on parameter, time, finished,
//! cycle and tag conditions, blended over their duration); speed scale,
//! cycle control and direct playback. Others (IK, look-at, foot locks)
//! pass their input through untouched.
use std::collections::{HashMap, HashSet};
use std::ffi::CString;

use rapier3d::math::{Rotation, Vector};

use crate::assets::anim::BonePose;
use crate::assets::kv3::Value;
use crate::assets::model::ModelFile;

const NONE: u32 = u32::MAX;

// NativeEngine.AnimParamType
pub const UNKNOWN: u8 = 0;
pub const BOOL: u8 = 1;
pub const ENUM: u8 = 2;
pub const INT: u8 = 3;
pub const FLOAT: u8 = 4;
pub const VECTOR: u8 = 5;
pub const ROTATION: u8 = 6;

/// A parameter's value, as managed's AnimVariant lays it out: four floats
/// (an int's or bool's bits in the first) and the type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Variant {
    pub raw: [f32; 4],
    pub kind: u8,
}

impl Variant {
    pub fn float(kind: u8, f: f32) -> Variant {
        Variant { raw: [f, 0.0, 0.0, 0.0], kind }
    }

    pub fn int(kind: u8, i: i32) -> Variant {
        Variant { raw: [f32::from_bits(i as u32), 0.0, 0.0, 0.0], kind }
    }

    /// As a number, whatever it is (an enum's option, a bool's 0 or 1).
    pub fn number(&self) -> f32 {
        match self.kind {
            BOOL | ENUM | INT => self.raw[0].to_bits() as i32 as f32,
            _ => self.raw[0],
        }
    }

    pub fn ffi(&self) -> crate::slots::S_NativeEngine_AnimVariant {
        let [x, y, z, w] = self.raw;
        crate::slots::S_NativeEngine_AnimVariant {
            f__rawValue: crate::slots::S_Vector4 { f__vec: crate::slots::S_System_Numerics_Vector4 { f_X: x, f_Y: y, f_Z: z, f_W: w } },
            f_Type: self.kind,
        }
    }

    pub fn from_ffi(v: &crate::slots::S_NativeEngine_AnimVariant) -> Variant {
        let r = v.f__rawValue;
        let q = r.f__vec;
        Variant { raw: [q.f_X, q.f_Y, q.f_Z, q.f_W], kind: v.f_Type }
    }
}

pub struct Param {
    pub name: CString,
    pub id: u32,
    pub kind: u8,
    pub default: Variant,
    pub min: Variant,
    pub max: Variant,
    pub options: Vec<CString>,
    /// a trigger: back to its default once the graph has run with it
    pub auto_reset: bool,
}

/// An input's damping (CAnimInputDamping): a spring toward the value.
#[derive(Clone, Copy, Debug)]
pub struct Damping {
    pub tension: f32,
    pub damping: f32,
    pub speed: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum Cmp { Eq, Ne, Gt, Ge, Lt, Le }

impl Cmp {
    fn of(op: i64) -> Cmp {
        match op { 1 => Cmp::Ne, 2 => Cmp::Gt, 3 => Cmp::Ge, 4 => Cmp::Lt, 5 => Cmp::Le, _ => Cmp::Eq }
    }

    fn test(self, a: f32, b: f32) -> bool {
        match self {
            Cmp::Eq => (a - b).abs() < 1e-4,
            Cmp::Ne => (a - b).abs() >= 1e-4,
            Cmp::Gt => a > b,
            Cmp::Ge => a >= b,
            Cmp::Lt => a < b,
            Cmp::Le => a <= b,
        }
    }
}

pub enum Condition {
    Param { param: u32, op: Cmp, value: f32 },
    /// seconds in the state
    Time { op: Cmp, value: f32 },
    /// the state's pose finished (or didn't)
    Finished { finished: bool },
    Cycle { op: Cmp, value: f32 },
    Tag { tag: u32, on: bool },
}

pub struct Transition {
    pub conditions: Vec<Condition>,
    pub dest: u32,
    pub blend: f32,
    pub reset: bool,
}

pub struct State {
    pub id: u32,
    pub input: u32,
    pub transitions: Vec<Transition>,
    pub start: bool,
    pub passthrough: bool,
}

pub enum Node {
    Pass(u32),
    Sequence { name: String, speed: f32, looping: bool, tags: Vec<(u32, f32, f32)> },
    SingleFrame { name: String, frame: i64 },
    Blend { children: Vec<(u32, f32)>, param: u32, damping: Option<Damping> },
    Blend2D { items: Vec<(String, [f32; 2])>, x: u32, y: u32, looping: bool, speed: f32, damping: Option<Damping> },
    Add { base: u32, additive: u32 },
    Subtract { base: u32, sub: u32 },
    Mask { a: u32, b: u32, list: String, param: u32 },
    Choice { children: Vec<(u32, f32)>, seed: u32 },
    Selector { children: Vec<u32>, by: u32, int_min: i32, blend: f32 },
    StateMachine { states: Vec<State> },
    SpeedScale { input: u32, param: u32 },
    CycleControl { input: u32, param: u32 },
}

pub struct Graph {
    pub name: CString,
    pub params: Vec<Param>,
    pub nodes: HashMap<u32, Node>,
    pub root: u32,
}

fn id(v: Option<&Value>) -> u32 {
    v.and_then(|x| x.get("m_id")).and_then(Value::as_i64).map(|i| i as u32).unwrap_or(NONE)
}

fn input(v: &Value, field: &str) -> u32 {
    id(v.get(field).and_then(|c| c.get("m_nodeID")))
}

fn f(v: &Value, field: &str) -> f32 {
    v.get(field).and_then(Value::as_f64).unwrap_or(0.0) as f32
}

fn b(v: &Value, field: &str) -> bool {
    v.get(field).and_then(Value::as_i64).unwrap_or(0) != 0
}

fn s(v: &Value, field: &str) -> String {
    v.get(field).and_then(Value::as_str).unwrap_or("").to_string()
}

fn cs(t: &str) -> CString {
    CString::new(t.replace('\0', "")).unwrap_or_default()
}

fn damping(v: &Value) -> Option<Damping> {
    let d = v.get("m_damping")?;
    if s(d, "m_speedFunction") == "NoDamping" { return None; }
    Some(Damping { tension: f(d, "m_fTension"), damping: f(d, "m_fDamping"), speed: f(d, "m_fSpeedScale") })
}

fn tag_spans(v: &Value) -> Vec<(u32, f32, f32)> {
    v.get("m_tagSpans").map(Value::as_array).unwrap_or(&[]).iter().map(|t| (id(t.get("m_id")), f(t, "m_fStartCycle"), f(t, "m_fDuration"))).collect()
}

fn condition(c: &Value) -> Option<Condition> {
    let op = Cmp::of(c.get("m_comparisonOp").and_then(Value::as_i64).unwrap_or(0));
    Some(match s(c, "_class").as_str() {
        "CParameterAnimCondition" => {
            let v = c.get("m_comparisonValue")?;
            let value = match v.get("m_data") {
                Some(Value::Bool(b)) => *b as i32 as f32,
                Some(x) => x.as_f64().unwrap_or(0.0) as f32,
                None => 0.0,
            };
            Condition::Param { param: id(c.get("m_paramID")), op, value }
        }
        "CTimeCondition" => Condition::Time { op, value: f(c, "m_comparisonValue") },
        "CFinishedCondition" => Condition::Finished { finished: c.get("m_bIsFinished").map(|v| v.as_i64().unwrap_or(1) != 0).unwrap_or(true) },
        "CCycleCondition" => Condition::Cycle { op, value: f(c, "m_comparisonValue") },
        "CTagCondition" => Condition::Tag { tag: id(c.get("m_tagID")), on: c.get("m_comparisonValue").and_then(Value::as_i64).unwrap_or(1) != 0 },
        _ => return None,
    })
}

fn node(v: &Value) -> Node {
    match s(v, "_class").as_str() {
        "CSequenceAnimNode" => Node::Sequence {
            name: s(v, "m_sequenceName"),
            speed: v.get("m_playbackSpeed").and_then(Value::as_f64).unwrap_or(1.0) as f32,
            looping: b(v, "m_bLoop"),
            tags: tag_spans(v),
        },
        "CSingleFrameAnimNode" => Node::SingleFrame { name: s(v, "m_sequenceName"), frame: v.i64("m_nFrameIndex") },
        "CBlendAnimNode" => Node::Blend {
            children: v.get("m_children").map(Value::as_array).unwrap_or(&[]).iter().map(|c| (input(c, "m_inputConnection"), f(c, "m_blendValue"))).collect(),
            param: id(v.get("m_param")),
            damping: damping(v),
        },
        "CBlend2DAnimNode" => Node::Blend2D {
            items: v.get("m_items").map(Value::as_array).unwrap_or(&[]).iter().map(|i| {
                let p = i.get("m_blendValue").map(Value::floats).unwrap_or_default();
                (s(i, "m_sequenceName"), [p.first().copied().unwrap_or(0.0), p.get(1).copied().unwrap_or(0.0)])
            }).collect(),
            x: id(v.get("m_paramX")),
            y: id(v.get("m_paramY")),
            looping: b(v, "m_bLoop"),
            speed: v.get("m_playbackSpeed").and_then(Value::as_f64).unwrap_or(1.0) as f32,
            damping: damping(v),
        },
        "CAddAnimNode" => Node::Add { base: input(v, "m_baseInput"), additive: input(v, "m_additiveInput") },
        "CSubtractAnimNode" => Node::Subtract { base: input(v, "m_baseInputConnection"), sub: input(v, "m_subtractInputConnection") },
        "CBoneMaskAnimNode" => Node::Mask {
            a: input(v, "m_inputConnection1"),
            b: input(v, "m_inputConnection2"),
            list: s(v, "m_weightListName"),
            param: if s(v, "m_blendValueSource") == "Parameter" { id(v.get("m_blendParameter")) } else { NONE },
        },
        "CChoiceAnimNode" => Node::Choice {
            children: v.get("m_children").map(Value::as_array).unwrap_or(&[]).iter().map(|c| (input(c, "m_inputConnection"), f(c, "m_weight").max(0.0))).collect(),
            seed: v.i64("m_seed") as u32,
        },
        "CSelectorAnimNode" => Node::Selector {
            children: v.get("m_children").map(Value::as_array).unwrap_or(&[]).iter().map(|c| id(c.get("m_nodeID"))).collect(),
            by: match s(v, "m_selectionSource").as_str() {
                "SelectionSource_Bool" => id(v.get("m_boolParamID")),
                "SelectionSource_Int" => id(v.get("m_intParamID")),
                _ => id(v.get("m_enumParamID")),
            },
            int_min: if s(v, "m_selectionSource") == "SelectionSource_Int" { v.i64("m_intParamMinValue") as i32 } else { 0 },
            blend: f(v, "m_blendDuration"),
        },
        "CStateMachineAnimNode" => Node::StateMachine {
            states: v.get("m_states").map(Value::as_array).unwrap_or(&[]).iter().map(|st| State {
                id: id(st.get("m_stateID")),
                input: input(st, "m_inputConnection"),
                start: b(st, "m_bIsStartState"),
                passthrough: b(st, "m_bIsPassthrough"),
                transitions: st.get("m_transitions").map(Value::as_array).unwrap_or(&[]).iter().filter(|t| !b(t, "m_bDisabled")).map(|t| Transition {
                    conditions: t.get("m_conditions").map(Value::as_array).unwrap_or(&[]).iter().filter_map(condition).collect(),
                    dest: id(t.get("m_destState")),
                    blend: f(t, "m_blendDuration"),
                    reset: b(t, "m_bReset"),
                }).collect(),
            }).collect(),
        },
        "CSpeedScaleAnimNode" => Node::SpeedScale { input: input(v, "m_inputConnection"), param: id(v.get("m_param")) },
        "CCycleControlAnimNode" => Node::CycleControl {
            input: input(v, "m_inputConnection"),
            param: if s(v, "m_valueSource") == "Parameter" { id(v.get("m_param")) } else { NONE },
        },
        // the root, direct playback, and every node this doesn't run: its input as it is
        _ => Node::Pass(input(v, "m_inputConnection")),
    }
}

fn param(p: &Value) -> Param {
    let class = s(p, "_class");
    let kind = match class.as_str() {
        "CBoolAnimParameter" => BOOL,
        "CEnumAnimParameter" => ENUM,
        "CIntAnimParameter" => INT,
        "CFloatAnimParameter" => FLOAT,
        "CVectorAnimParameter" => VECTOR,
        "CQuaternionAnimParameter" | "CRotationAnimParameter" => ROTATION,
        _ => UNKNOWN,
    };
    let num = |field: &str| p.get(field).and_then(Value::as_f64).unwrap_or(0.0);
    let (default, min, max) = match kind {
        BOOL => (Variant::int(kind, num("m_bDefaultValue") as i32), Variant::int(kind, 0), Variant::int(kind, 1)),
        ENUM => {
            let n = p.get("m_enumOptions").map(|o| o.as_array().len()).unwrap_or(0) as i32;
            (Variant::int(kind, num("m_defaultValue") as i32), Variant::int(kind, 0), Variant::int(kind, (n - 1).max(0)))
        }
        INT => (Variant::int(kind, num("m_defaultValue") as i32), Variant::int(kind, num("m_minValue") as i32), Variant::int(kind, num("m_maxValue") as i32)),
        FLOAT => (Variant::float(kind, num("m_fDefaultValue") as f32), Variant::float(kind, num("m_fMinValue") as f32), Variant::float(kind, num("m_fMaxValue") as f32)),
        VECTOR => {
            let d = p.get("m_defaultValue").map(Value::floats).unwrap_or_default();
            let at = |i: usize| d.get(i).copied().unwrap_or(0.0);
            (Variant { raw: [at(0), at(1), at(2), 0.0], kind }, Variant::float(kind, 0.0), Variant::float(kind, 0.0))
        }
        ROTATION => {
            let d = p.get("m_defaultValue").map(Value::floats).unwrap_or_default();
            let q = if d.len() == 4 { [d[0], d[1], d[2], d[3]] } else { [0.0, 0.0, 0.0, 1.0] };
            (Variant { raw: q, kind }, Variant::float(kind, 0.0), Variant::float(kind, 0.0))
        }
        _ => (Variant::float(kind, 0.0), Variant::float(kind, 0.0), Variant::float(kind, 0.0)),
    };
    Param {
        name: cs(&s(p, "m_name")),
        id: id(p.get("m_id")),
        kind,
        default,
        min,
        max,
        options: p.get("m_enumOptions").map(Value::as_array).unwrap_or(&[]).iter().filter_map(Value::as_str).map(cs).collect(),
        auto_reset: b(p, "m_bAutoReset"),
    }
}

/// A graph from its DATA block.
pub fn read(name: &str, data: &Value) -> Graph {
    let mut nodes = HashMap::new();
    let mut root = NONE;
    for n in data.get("m_nodeManager").and_then(|m| m.get("m_nodes")).map(Value::as_array).unwrap_or(&[]) {
        let Some(v) = n.get("value") else { continue };
        let key = id(n.get("key"));
        if s(v, "_class") == "CRootAnimNode" { root = input(v, "m_inputConnection"); }
        nodes.insert(key, node(v));
    }
    let params = data.get("m_pParameterList").and_then(|l| l.get("m_Parameters")).map(Value::as_array).unwrap_or(&[]).iter().map(param).collect();
    Graph { name: cs(name), params, nodes, root }
}

// ---- graphs loaded, by name: each at an address of its own for good, as
// are its parameters (managed holds both)

use std::sync::{Mutex, MutexGuard, OnceLock};

#[derive(Default)]
struct Loaded {
    by_name: HashMap<String, usize>,
    graphs: HashSet<usize>,
    params: HashMap<usize, (usize, usize)>,
}

fn loaded() -> MutexGuard<'static, Loaded> {
    static L: OnceLock<Mutex<Loaded>> = OnceLock::new();
    L.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner())
}

/// The graph a name names (`models/x/x.vanmgrph`), read once: its object (0: none).
pub fn load(name: &str) -> usize {
    let key = crate::fs::normal(name.trim_end_matches("_c"));
    if key.is_empty() { return 0; }
    if let Some(&g) = loaded().by_name.get(&key) { return g; }
    let Some(data) = crate::fs::read_compiled(&key).and_then(|b| crate::assets::resource::Resource::parse(b).ok()).and_then(|r| r.kv3("DATA")?.ok()) else {
        crate::trace::note(&format!("animgraph: {name}: no graph to read"));
        return 0;
    };
    let g: &'static Graph = Box::leak(Box::new(read(&key, &data)));
    let ptr = g as *const Graph as usize;
    let mut l = loaded();
    l.by_name.insert(key, ptr);
    l.graphs.insert(ptr);
    for (i, p) in g.params.iter().enumerate() { l.params.insert(p as *const Param as usize, (ptr, i)); }
    ptr
}

/// The graph at `ptr`, if it is one.
pub fn graph(ptr: usize) -> Option<&'static Graph> {
    loaded().graphs.contains(&ptr).then(|| unsafe { &*(ptr as *const Graph) })
}

/// The parameter at `ptr`, if it is one.
pub fn param_at(ptr: usize) -> Option<&'static Param> {
    loaded().params.contains_key(&ptr).then(|| unsafe { &*(ptr as *const Param) })
}

/// A model's own graph: the one its key values name (`anim_graph_resource`).
pub fn of_model(m: &ModelFile) -> usize {
    m.key_values.as_ref().and_then(|kv| kv.get("anim_graph_resource")).and_then(Value::as_str).map(load).unwrap_or(0)
}

// ---- running a graph

type Pose = Vec<BonePose>;

#[derive(Clone)]
/// What a node gives: its pose (a delta, where a graph makes one: an add
/// takes it so), how far through it is, whether it has finished, and the
/// tags on.
struct Out {
    pose: Pose,
    cycle: f32,
    finished: bool,
    tags: HashSet<u32>,
}

/// A state machine's state: where it is, for how long, what it's blending from.
#[derive(Clone, Default)]
struct Machine {
    current: usize,
    time: f32,
    /// the state blended out of, its pose frozen, the blend's time and length
    from: Option<(Pose, f32, f32)>,
}

#[derive(Default)]
pub struct Instance {
    pub values: Vec<Variant>,
    /// each timed node's time in seconds
    times: HashMap<u32, f32>,
    machines: HashMap<u32, Machine>,
    choices: HashMap<u32, usize>,
    /// a selector's choice, and the pose it's blending from
    selectors: HashMap<u32, (usize, Option<(Pose, f32)>)>,
    /// damped inputs: value and speed, per node and axis
    damped: HashMap<(u32, u8), (f32, f32)>,
    random: u32,
    /// this step's outputs, by node and time step: a node many others take
    /// (a graph is a web, not a tree) runs once
    memo: HashMap<(u32, u32), Out>,
}

impl Instance {
    pub fn new(g: &Graph) -> Instance {
        Instance { values: g.params.iter().map(|p| p.default).collect(), random: 0x2545_f491, ..Default::default() }
    }

    pub fn reset_parameters(&mut self, g: &Graph) {
        self.values = g.params.iter().map(|p| p.default).collect();
    }

    fn value(&self, g: &Graph, param: u32) -> Option<Variant> {
        g.params.iter().position(|p| p.id == param).and_then(|i| self.values.get(i).copied())
    }

    fn number(&self, g: &Graph, param: u32) -> f32 {
        self.value(g, param).map(|v| v.number()).unwrap_or(0.0)
    }

    /// A damped input: a spring toward `target`, as the node's damping says.
    fn damp(&mut self, node: u32, axis: u8, target: f32, d: Option<Damping>, dt: f32) -> f32 {
        let Some(d) = d else { return target };
        let e = self.damped.entry((node, axis)).or_insert((target, 0.0));
        if dt > 0.0 {
            // a damped spring, in a few small steps so a stiff one stays steady
            let n = ((dt * 240.0).ceil() as usize).clamp(1, 16);
            let h = dt / n as f32;
            for _ in 0..n {
                let accel = (target - e.0) * d.tension - e.1 * d.damping;
                e.1 += accel * h;
                if d.speed > 0.0 { e.1 = e.1.clamp(-d.speed * 10.0, d.speed * 10.0); }
                e.0 += e.1 * h;
            }
        }
        e.0
    }

    fn next_random(&mut self) -> u32 {
        self.random ^= self.random << 13;
        self.random ^= self.random >> 17;
        self.random ^= self.random << 5;
        self.random
    }

    /// Every timed node under `id` starts again.
    fn reset(&mut self, g: &Graph, id: u32) {
        let mut stack = vec![id];
        let mut seen = HashSet::new();
        while let Some(n) = stack.pop() {
            if !seen.insert(n) { continue; }
            self.times.remove(&n);
            self.machines.remove(&n);
            self.choices.remove(&n);
            match g.nodes.get(&n) {
                Some(Node::Pass(i) | Node::SpeedScale { input: i, .. } | Node::CycleControl { input: i, .. }) => stack.push(*i),
                Some(Node::Blend { children, .. } | Node::Choice { children, .. }) => stack.extend(children.iter().map(|c| c.0)),
                Some(Node::Add { base, additive: o } | Node::Subtract { base, sub: o } | Node::Mask { a: base, b: o, .. }) => stack.extend([*base, *o]),
                Some(Node::Selector { children, .. }) => stack.extend(children),
                Some(Node::StateMachine { states }) => stack.extend(states.iter().map(|s| s.input)),
                _ => {}
            }
        }
    }

    /// The graph run on by `dt`: the model's pose (each bone in its parent's space).
    pub fn run(&mut self, g: &Graph, m: &ModelFile, dt: f32) -> Pose {
        let bind: Pose = m.bones.iter().map(|b| BonePose { pos: b.pos, rot: b.rot, scale: 1.0 }).collect();
        self.memo.clear();
        let out = self.eval(g, m, &bind, g.root, dt, 0);
        self.memo.clear();
        // triggers back to their defaults, now they've been seen
        for (i, p) in g.params.iter().enumerate() {
            if p.auto_reset { self.values[i] = p.default; }
        }
        out.pose
    }

    fn clip_out(&mut self, m: &ModelFile, bind: &Pose, node: u32, name: &str, speed: f32, looping: Option<bool>, dt: f32, tags: &[(u32, f32, f32)]) -> Out {
        let Some(seq) = m.anim.sequences.iter().find(|s| s.name.eq_ignore_ascii_case(name)) else { return still(bind) };
        let clip = &m.anim.clips[seq.clip];
        let looping = looping.unwrap_or(seq.looping);
        let duration = clip.duration();
        let t = self.times.entry(node).or_insert(0.0);
        *t += dt * speed;
        let (cycle, finished) = if duration <= 0.0 {
            (0.0, true)
        } else if looping {
            *t = t.rem_euclid(duration);
            (*t / duration, false)
        } else {
            *t = t.clamp(0.0, duration);
            (*t / duration, *t >= duration)
        };
        let pose = if clip.delta { m.anim.sample(seq.clip, cycle, &identity(bind.len())) } else { m.anim.sample(seq.clip, cycle, bind) };
        let tags = tags.iter().filter(|(_, start, len)| cycle >= *start && cycle <= start + len).map(|t| t.0).collect();
        Out { pose, cycle, finished, tags }
    }

    fn eval(&mut self, g: &Graph, m: &ModelFile, bind: &Pose, id: u32, dt: f32, depth: usize) -> Out {
        if depth > 128 { return still(bind); }
        // a pose looked at without time passing (a selector's, a cycle control's) runs again
        if dt != 0.0 {
            if let Some(o) = self.memo.get(&(id, dt.to_bits())) { return o.clone(); }
        }
        let out = self.node(g, m, bind, id, dt, depth);
        if dt != 0.0 { self.memo.insert((id, dt.to_bits()), out.clone()); }
        out
    }

    fn node(&mut self, g: &Graph, m: &ModelFile, bind: &Pose, id: u32, dt: f32, depth: usize) -> Out {
        let Some(n) = g.nodes.get(&id) else { return still(bind) };
        match n {
            Node::Pass(i) => if *i == NONE { still(bind) } else { self.eval(g, m, bind, *i, dt, depth + 1) },
            Node::Sequence { name, speed, looping, tags } => self.clip_out(m, bind, id, name, *speed, Some(*looping), dt, tags),
            Node::SingleFrame { name, frame } => {
                let Some(seq) = m.anim.sequences.iter().find(|s| s.name.eq_ignore_ascii_case(name)) else { return still(bind) };
                let clip = &m.anim.clips[seq.clip];
                let cycle = if clip.frames > 1 { (*frame as f32 / (clip.frames - 1) as f32).clamp(0.0, 1.0) } else { 0.0 };
                let pose = if clip.delta { m.anim.sample(seq.clip, cycle, &identity(bind.len())) } else { m.anim.sample(seq.clip, cycle, bind) };
                Out { pose, cycle, finished: true, tags: HashSet::new() }
            }
            Node::Blend { children, param, damping, .. } => {
                if children.is_empty() { return still(bind); }
                let target = self.number(g, *param);
                let v = self.damp(id, 0, target, *damping, dt);
                let mut sorted: Vec<(u32, f32)> = children.clone();
                sorted.sort_by(|a, b| a.1.total_cmp(&b.1));
                let (i0, i1, t) = bracket(&sorted.iter().map(|c| c.1).collect::<Vec<_>>(), v);
                // both sides run; the rest keep their time
                let a = self.eval(g, m, bind, sorted[i0].0, dt, depth + 1);
                if i0 == i1 || t <= 0.0 { return a; }
                let b = self.eval(g, m, bind, sorted[i1].0, dt, depth + 1);
                blend(a, b, t)
            }
            Node::Blend2D { items, x, y, speed, damping, looping } => {
                if items.is_empty() { return still(bind); }
                let (tx, ty) = (self.number(g, *x), self.number(g, *y));
                let px = self.damp(id, 0, tx, *damping, dt);
                let py = self.damp(id, 1, ty, *damping, dt);
                let weights = weights_2d(&items.iter().map(|i| i.1).collect::<Vec<_>>(), [px, py]);
                let mut acc: Option<(Out, f32)> = None;
                for (k, w) in weights.into_iter().enumerate().filter(|(_, w)| *w > 1e-4) {
                    // every item at the same cycle: the node's own time
                    let o = self.clip_out(m, bind, id ^ (k as u32).wrapping_mul(0x9e37_79b9), &items[k].0, *speed, Some(*looping), dt, &[]);
                    acc = Some(match acc {
                        None => (o, w),
                        Some((a, wa)) => { let total = wa + w; (blend(a, o, w / total), total) }
                    });
                }
                acc.map(|a| a.0).unwrap_or_else(|| still(bind))
            }
            Node::Add { base, additive } => {
                // the additive input is a delta, whatever made it: a subtract,
                // or a `_delta` sequence, whose clip doesn't say it is one
                let b = self.eval(g, m, bind, *base, dt, depth + 1);
                if !g.nodes.contains_key(additive) { return b; }
                let a = self.eval(g, m, bind, *additive, dt, depth + 1);
                Out { pose: apply(&b.pose, &a.pose), tags: b.tags.union(&a.tags).copied().collect(), ..b }
            }
            Node::Subtract { base, sub } => {
                let b = self.eval(g, m, bind, *base, dt, depth + 1);
                let s = self.eval(g, m, bind, *sub, dt, depth + 1);
                Out { pose: difference(&s.pose, &b.pose), ..b }
            }
            Node::Mask { a, b, list, param } => {
                let x = self.eval(g, m, bind, *a, dt, depth + 1);
                let y = self.eval(g, m, bind, *b, dt, depth + 1);
                let scale = if *param == NONE { 1.0 } else { self.number(g, *param).clamp(0.0, 1.0) };
                let w = mask_weights(m, list);
                let pose = x.pose.iter().zip(&y.pose).enumerate().map(|(i, (p, q))| {
                    p.lerp(q, w.get(i).copied().unwrap_or(0.0) * scale)
                }).collect();
                Out { pose, tags: x.tags.union(&y.tags).copied().collect(), ..x }
            }
            Node::Choice { children, seed } => {
                if children.is_empty() { return still(bind); }
                let chosen = match self.choices.get(&id) {
                    Some(&c) => c,
                    None => {
                        if self.random == 0x2545_f491 { self.random ^= seed | 1; }
                        let total: f32 = children.iter().map(|c| c.1).sum();
                        let mut r = (self.next_random() as f32 / u32::MAX as f32) * total.max(1e-6);
                        let c = children.iter().position(|c| { r -= c.1; r <= 0.0 }).unwrap_or(0);
                        self.choices.insert(id, c);
                        c
                    }
                };
                let out = self.eval(g, m, bind, children[chosen].0, dt, depth + 1);
                // a new choice once this one's done
                if out.finished { self.choices.remove(&id); self.reset(g, children[chosen].0); }
                out
            }
            Node::Selector { children, by, int_min, blend: duration } => {
                if children.is_empty() { return still(bind); }
                let want = ((self.number(g, *by) as i32 - int_min).max(0) as usize).min(children.len() - 1);
                let (current, from) = self.selectors.remove(&id).unwrap_or((want, None));
                let (current, mut from) = if current != want {
                    // the pose being left, to blend from
                    let left = self.eval(g, m, bind, children[current], 0.0, depth + 1).pose;
                    self.reset(g, children[want]);
                    (want, if *duration > 0.0 { Some((left, 0.0)) } else { None })
                } else {
                    (current, from)
                };
                let mut out = self.eval(g, m, bind, children[current], dt, depth + 1);
                if let Some((left, t)) = from.as_mut() {
                    *t += dt;
                    let k = (*t / duration).clamp(0.0, 1.0);
                    out.pose = left.iter().zip(&out.pose).map(|(a, b)| a.lerp(b, k)).collect();
                    if k >= 1.0 { from = None; }
                }
                self.selectors.insert(id, (current, from));
                out
            }
            Node::StateMachine { states } => self.machine(g, m, bind, id, states, dt, depth),
            Node::SpeedScale { input, param } => {
                let k = if *param == NONE { 1.0 } else { self.number(g, *param) };
                self.eval(g, m, bind, *input, dt * k, depth + 1)
            }
            Node::CycleControl { input, param } => {
                let out = self.eval(g, m, bind, *input, 0.0, depth + 1);
                if *param == NONE { return out; }
                // the input's own time set where the parameter says: its cycle
                let cycle = self.number(g, *param).clamp(0.0, 1.0);
                self.set_cycle(g, m, *input, cycle);
                self.eval(g, m, bind, *input, 0.0, depth + 1)
            }
        }
    }

    /// The sequences under `id` put at `cycle` of their length.
    fn set_cycle(&mut self, g: &Graph, m: &ModelFile, id: u32, cycle: f32) {
        match g.nodes.get(&id) {
            Some(Node::Sequence { name, .. }) => {
                if let Some(seq) = m.anim.sequences.iter().find(|s| s.name.eq_ignore_ascii_case(name)) {
                    self.times.insert(id, cycle * m.anim.clips[seq.clip].duration());
                }
            }
            Some(Node::Pass(i) | Node::SpeedScale { input: i, .. }) => self.set_cycle(g, m, *i, cycle),
            Some(Node::Blend { children, .. }) => for c in children.clone() { self.set_cycle(g, m, c.0, cycle) },
            _ => {}
        }
    }

    fn machine(&mut self, g: &Graph, m: &ModelFile, bind: &Pose, id: u32, states: &[State], dt: f32, depth: usize) -> Out {
        if states.is_empty() { return still(bind); }
        let mut sm = self.machines.remove(&id).unwrap_or_else(|| Machine { current: states.iter().position(|s| s.start).unwrap_or(0), ..Default::default() });
        sm.time += dt;
        let mut out = self.eval(g, m, bind, states[sm.current].input, dt, depth + 1);
        // transitions, as many as their conditions allow in one step (passthrough states don't hold)
        for _ in 0..states.len() {
            let st = &states[sm.current];
            let taken = st.transitions.iter().find(|t| {
                states.iter().any(|s| s.id == t.dest) && t.conditions.iter().all(|c| match c {
                    Condition::Param { param, op, value } => op.test(self.number(g, *param), *value),
                    Condition::Time { op, value } => op.test(sm.time, *value),
                    Condition::Finished { finished } => out.finished == *finished,
                    Condition::Cycle { op, value } => op.test(out.cycle, *value),
                    Condition::Tag { tag, on } => out.tags.contains(tag) == *on,
                })
            });
            let Some(t) = taken else { break };
            let dest = states.iter().position(|s| s.id == t.dest).unwrap();
            if t.reset { self.reset(g, states[dest].input); }
            sm.from = if t.blend > 0.0 && !states[sm.current].passthrough { Some((out.pose.clone(), 0.0, t.blend)) } else { None };
            sm.current = dest;
            sm.time = 0.0;
            out = self.eval(g, m, bind, states[dest].input, 0.0, depth + 1);
            if !states[dest].passthrough { break; }
        }
        if let Some((from, t, len)) = sm.from.as_mut() {
            *t += dt;
            let k = smooth((*t / *len).clamp(0.0, 1.0));
            out.pose = from.iter().zip(&out.pose).map(|(a, b)| a.lerp(b, k)).collect();
            if k >= 1.0 { sm.from = None; }
        }
        self.machines.insert(id, sm);
        out
    }
}

fn still(bind: &Pose) -> Out {
    Out { pose: bind.clone(), cycle: 0.0, finished: true, tags: HashSet::new() }
}

fn identity(n: usize) -> Pose {
    vec![BonePose { pos: Vector::ZERO, rot: Rotation::IDENTITY, scale: 1.0 }; n]
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Two outputs mixed, `t` of the way to `b`.
fn blend(a: Out, b: Out, t: f32) -> Out {
    let pose = a.pose.iter().zip(&b.pose).map(|(p, q)| p.lerp(q, t)).collect();
    Out {
        pose,
        cycle: a.cycle + (b.cycle - a.cycle) * t,
        finished: a.finished && b.finished,
        tags: if t < 0.5 { a.tags } else { b.tags },
    }
}

/// A delta laid over `base`.
fn apply(base: &Pose, delta: &Pose) -> Pose {
    base.iter().zip(delta).map(|(b, d)| BonePose { pos: b.pos + d.pos, rot: (b.rot * d.rot).normalize(), scale: b.scale * d.scale }).collect()
}

/// The delta that takes `from` to `to`.
fn difference(from: &Pose, to: &Pose) -> Pose {
    from.iter().zip(to).map(|(f, t)| BonePose { pos: t.pos - f.pos, rot: (f.rot.inverse() * t.rot).normalize(), scale: if f.scale != 0.0 { t.scale / f.scale } else { 1.0 } }).collect()
}

/// The two values `v` is between, and how far: indices into sorted `keys`.
fn bracket(keys: &[f32], v: f32) -> (usize, usize, f32) {
    if v <= keys[0] { return (0, 0, 0.0); }
    for i in 0..keys.len() - 1 {
        if v <= keys[i + 1] {
            let span = keys[i + 1] - keys[i];
            return (i, i + 1, if span > 0.0 { (v - keys[i]) / span } else { 0.0 });
        }
    }
    (keys.len() - 1, keys.len() - 1, 0.0)
}

/// Each item's weight at `p`: gradient band interpolation (Rune Johansen's,
/// the usual for 2D blend spaces), exact on an item, smooth between.
fn weights_2d(points: &[[f32; 2]], p: [f32; 2]) -> Vec<f32> {
    let n = points.len();
    let mut w = vec![0.0; n];
    for i in 0..n {
        let pi = points[i];
        let mut wi = 1.0f32;
        for (j, pj) in points.iter().enumerate() {
            if i == j { continue; }
            let (ex, ey) = (pj[0] - pi[0], pj[1] - pi[1]);
            let len2 = ex * ex + ey * ey;
            if len2 <= 1e-8 { continue; }
            let h = 1.0 - ((p[0] - pi[0]) * ex + (p[1] - pi[1]) * ey) / len2;
            wi = wi.min(h.max(0.0));
        }
        w[i] = wi;
    }
    let total: f32 = w.iter().sum();
    if total > 0.0 { w.iter_mut().for_each(|x| *x /= total); } else if let Some((k, _)) = points.iter().enumerate().min_by(|a, b| dist(a.1, p).total_cmp(&dist(b.1, p))) { w[k] = 1.0; }
    w
}

fn dist(a: &[f32; 2], b: [f32; 2]) -> f32 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)
}

/// A weight list's weight for each bone of the model: as named, or its
/// parent's (a bone under none named: 0).
fn mask_weights(m: &ModelFile, list: &str) -> Vec<f32> {
    let Some((_, named)) = m.anim.masks.iter().find(|(n, _)| n.eq_ignore_ascii_case(list)) else { return vec![0.0; m.bones.len()] };
    let mut w = vec![0.0; m.bones.len()];
    for (i, b) in m.bones.iter().enumerate() {
        w[i] = match named.get(i).copied().flatten() {
            Some(x) => x,
            None if b.parent >= 0 && (b.parent as usize) < i => w[b.parent as usize],
            None => 0.0,
        };
    }
    w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_compare_and_blend_as_graphs_say() {
        assert!(Cmp::of(4).test(0.0, 0.5) && !Cmp::of(4).test(1.0, 0.5), "4: less than");
        assert!(Cmp::of(2).test(1.0, 0.5), "2: greater than");
        assert_eq!(Variant::int(ENUM, 1).number(), 1.0);
        assert_eq!(Variant::int(BOOL, 1).raw[0].to_bits(), 1, "a bool's bits, as managed packs it");
        assert_eq!(bracket(&[-120.0, 0.0, 120.0], 60.0), (1, 2, 0.5));
        assert_eq!(bracket(&[-120.0, 0.0, 120.0], -500.0), (0, 0, 0.0));
        // on an item: all of it; between two: some of each
        let pts = [[0.0, 0.0], [100.0, 0.0], [0.0, 100.0], [100.0, 100.0]];
        let on = weights_2d(&pts, [100.0, 0.0]);
        assert!((on[1] - 1.0).abs() < 1e-5, "{on:?}");
        let mid = weights_2d(&pts, [50.0, 0.0]);
        assert!((mid[0] - 0.5).abs() < 1e-4 && (mid[1] - 0.5).abs() < 1e-4, "{mid:?}");
        assert!((weights_2d(&pts, [37.0, 61.0]).iter().sum::<f32>() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn a_graph_reads_its_parameters_and_nodes() {
        let text = r#"{
            m_nodeManager = { m_nodes = [
                { key = { m_id = 1 } value = { _class = "CRootAnimNode" m_inputConnection = { m_nodeID = { m_id = 2 } } } },
                { key = { m_id = 2 } value = { _class = "CStateMachineAnimNode" m_states = [
                    { _class = "CAnimState" m_stateID = { m_id = 10 } m_bIsStartState = true m_inputConnection = { m_nodeID = { m_id = 3 } }
                      m_transitions = [ { m_destState = { m_id = 11 } m_blendDuration = 0.0 m_bReset = true m_bDisabled = false
                        m_conditions = [ { _class = "CParameterAnimCondition" m_comparisonOp = 0 m_paramID = { m_id = 100 } m_comparisonValue = { m_nType = 1 m_data = true } } ] } ] },
                    { _class = "CAnimState" m_stateID = { m_id = 11 } m_inputConnection = { m_nodeID = { m_id = 4 } } m_transitions = [
                      { m_destState = { m_id = 10 } m_blendDuration = 0.2 m_bDisabled = false m_conditions = [ { _class = "CFinishedCondition" m_comparisonOp = 0 m_bIsFinished = true } ] } ] }
                ] } },
                { key = { m_id = 3 } value = { _class = "CSequenceAnimNode" m_sequenceName = "idle" m_playbackSpeed = 1.0 m_bLoop = true } },
                { key = { m_id = 4 } value = { _class = "CSequenceAnimNode" m_sequenceName = "attack" m_playbackSpeed = 1.0 m_bLoop = false } },
            ] }
            m_pParameterList = { m_Parameters = [
                { _class = "CBoolAnimParameter" m_name = "b_attack" m_id = { m_id = 100 } m_bDefaultValue = false m_bAutoReset = true },
                { _class = "CEnumAnimParameter" m_name = "holdtype" m_id = { m_id = 101 } m_defaultValue = 1 m_enumOptions = [ "none", "pistol", "rifle" ] },
                { _class = "CFloatAnimParameter" m_name = "aim_pitch" m_id = { m_id = 102 } m_fDefaultValue = 0.0 m_fMinValue = -90.0 m_fMaxValue = 90.0 },
            ] }
        }"#;
        let g = read("test.vanmgrph", &crate::assets::kv3text::read(text).unwrap());
        assert_eq!(g.root, 2);
        assert_eq!(g.params.len(), 3);
        assert_eq!(g.params[0].kind, BOOL);
        assert!(g.params[0].auto_reset);
        assert_eq!(g.params[1].default.number(), 1.0);
        assert_eq!(g.params[1].max.number(), 2.0);
        assert_eq!(g.params[1].options[2].to_str().unwrap(), "rifle");
        assert_eq!(g.params[2].min.number(), -90.0);
        let Some(Node::StateMachine { states }) = g.nodes.get(&2) else { panic!("a state machine") };
        assert!(states[0].start);
        assert!(matches!(states[0].transitions[0].conditions[0], Condition::Param { param: 100, value, .. } if value == 1.0));
        assert!(matches!(states[1].transitions[0].conditions[0], Condition::Finished { finished: true }));
        // the machine: idle; b_attack set, attack; once it's done, idle again; the trigger resets itself
        let m = ModelFile::default();
        let mut i = Instance::new(&g);
        i.run(&g, &m, 0.1);
        assert_eq!(i.machines[&2].current, 0);
        i.values[0] = Variant::int(BOOL, 1);
        i.run(&g, &m, 0.1);
        assert_eq!(i.machines[&2].current, 1, "attacking");
        assert_eq!(i.values[0].number(), 0.0, "the trigger is spent");
        // the model has no attack sequence: it's finished at once, back to idle
        i.run(&g, &m, 0.1);
        assert_eq!(i.machines[&2].current, 0);
    }

    #[test]
    fn a_downloaded_graph_punches_when_told() {
        // s&box's download cache, if this machine has one: Facepunch's content, read where it lies
        let Ok(dir) = std::env::var("ENGINE2_SAMPLE_ASSETS") else { return };
        let dir = std::path::Path::new(&dir).join("models/first_person");
        let find = |stem: &str, ext: &str| std::fs::read_dir(&dir).ok()?.filter_map(|e| e.ok()).map(|e| e.path()).find(|p| {
            let n = p.file_name().unwrap().to_string_lossy().to_string();
            n.starts_with(&format!("{stem}.")) && n.ends_with(ext)
        });
        let (Some(model), Some(graph)) = (find("v_first_person_arms_human", ".vmdl_c"), find("v_first_person_arms_punching", ".vanmgrph_c")) else { return };
        crate::fs::add_link("models/first_person/v_first_person_arms_human.vmdl_c", model.to_str().unwrap());
        crate::fs::add_link("models/first_person/v_first_person_arms_punching.vanmgrph_c", graph.to_str().unwrap());
        let m = crate::models::load("models/first_person/v_first_person_arms_human.vmdl");
        let f = crate::models::file(m).unwrap();
        let gp = of_model(f);
        assert_ne!(gp, 0, "the model names its graph");
        let g = super::graph(gp).unwrap();
        assert!(g.params.len() > 50 && g.nodes.len() > 100, "{} parameters, {} nodes", g.params.len(), g.nodes.len());
        // each hand in the model's space, from the bones' parents
        let at = |pose: &Pose, hand: usize| {
            let mut model: Vec<(Vector, Rotation)> = Vec::new();
            for (i, b) in f.bones.iter().enumerate() {
                let local = (pose[i].pos, pose[i].rot);
                model.push(if b.parent >= 0 { let (pp, pr) = model[b.parent as usize]; (pp + pr * local.0, pr * local.1) } else { local });
            }
            model[hand].0
        };
        let (right, left) = (f.bone("hand_R") as usize, f.bone("hand_L") as usize);
        let hands = |pose: &Pose| (at(pose, right), at(pose, left));
        let mut i = Instance::new(g);
        // what a weapon sets: its speeds (0 by default, which holds it still), and on the ground
        let mut set = |name: &str, v: Variant| { let k = g.params.iter().position(|p| p.name.to_str() == Ok(name)).unwrap(); i.values[k] = v; };
        for speed in ["speed_deploy", "speed_reload", "speed_ironsights"] { set(speed, Variant::float(FLOAT, 1.0)); }
        set("b_grounded", Variant::int(BOOL, 1));
        let dt = 1.0 / 60.0;
        let mut rest = (Vector::ZERO, Vector::ZERO);
        for _ in 0..120 { rest = hands(&i.run(g, f, dt)); }
        let attack = g.params.iter().position(|p| p.name.to_str() == Ok("b_attack")).unwrap();
        i.values[attack] = Variant::int(BOOL, 1);
        // the sequences under way: each sequence node's time, by name
        let playing = |i: &Instance| i.times.iter().filter_map(|(id, t)| match g.nodes.get(id) { Some(Node::Sequence { name, .. }) if *t > 0.0 => Some(name.clone()), _ => None }).collect::<Vec<_>>();
        // three punches, a second apart: one hand or the other swings out
        let mut reaches = Vec::new();
        for punch in 0..6 {
            i.values[attack] = Variant::int(BOOL, 1);
            let mut reach = (0.0f32, 0.0f32);
            let mut punches = Vec::new();
            for k in 0..60 {
                let (r, l) = hands(&i.run(g, f, dt));
                if k == 0 { assert_eq!(i.values[attack].number(), 0.0, "the trigger resets itself"); }
                reach = (reach.0.max((r - rest.0).length()), reach.1.max((l - rest.1).length()));
                punches.extend(playing(&i).into_iter().filter(|n| n.starts_with("Punching_") && (n.contains("_Hit_") || n.contains("_Miss_"))));
            }
            punches.sort();
            punches.dedup();
            eprintln!("punch {punch}: {punches:?}, right hand out {:.1}, left {:.1}", reach.0, reach.1);
            assert_eq!(punches.len(), 1, "b_attack plays a punch, one of them: {punches:?}");
            // the hand the punch is for swings out further than the other
            let (out, other) = if punches[0].contains("_Left_") { (reach.1, reach.0) } else { (reach.0, reach.1) };
            assert!(out > 8.0 && out > other, "{}: {reach:?}", punches[0]);
            reaches.push(punches[0].contains("_Left_"));
        }
        assert!(reaches.contains(&true) && reaches.contains(&false), "both hands punch, by the graph's choice");
        // and they come back
        let mut back = rest;
        for _ in 0..240 { back = hands(&i.run(g, f, dt)); }
        assert!((back.0 - rest.0).length() < 3.0 && (back.1 - rest.1).length() < 3.0, "back at rest: {back:?}");
    }
}
