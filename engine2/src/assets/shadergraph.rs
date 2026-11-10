//! Shader graphs (`.shdrgrph`): the material shaders creators make in
//! s&box's graph editor, translated to WGSL.
//!
//! s&box compiles a game's own shaders with the closed `vfx_vulkan`. A
//! graph, though, is plain JSON, and the compiler that turns one into a
//! shader is open (sbox-public, MIT: `game/addons/tools/Code/ShaderGraph`,
//! its nodes in `game/editor/ShaderGraph/Code/Nodes`). What it writes ends
//! by filling in a material (colour, emission, opacity, a tangent-space
//! normal, roughness, metalness, occlusion) and handing it to the engine's
//! standard shading. So a graph needs none of s&box's lighting here: this
//! module writes the function that fills in that material, in WGSL, and
//! `render/graph.wgsl` lights it as `shader.wgsl` lights `complex`.
//!
//! Each node is translated as `GraphCompiler` compiles it: the same
//! inputs in the same order, the same defaults, the same casts between
//! one and four components. The order matters beyond the arithmetic: a
//! texture's name (`g_t<Name>`, numbered when two share one) comes from
//! when its node is reached, and a material names its textures by it.
//! Parameters are named as the compiler names them too (`g_fl`, `g_v`,
//! `g_b` and the name without its spaces), so a material's own values
//! for them are found.
//!
//! Where it departs: WGSL has no implicit casts, so a scalar beside a
//! vector in `min`, `max`, `step`, `smoothstep` and `pow` is widened
//! outright; a `?:` is a `select`, both sides computed, as HLSL computes
//! them; a number is always written as a float.
//!
//! Not translated yet (a graph with one is refused, and says which):
//! subgraphs, cube textures, the scene's colour and depth, screen
//! positions, post-process graphs. Mirrored texture addressing is wrapped.
use std::collections::HashMap;

use serde_json::Value as J;

pub const MAX_TEXTURES: usize = 16;
pub const MAX_PARAMS: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Blend { Opaque, Masked, Translucent }

/// A texture the graph samples: the name a material gives it under
/// (`g_tColor`), how it's read, and what it is with none given.
#[derive(Clone, Debug, PartialEq)]
pub struct Texture {
    pub name: String,
    pub srgb: bool,
    pub clamp: bool,
    pub default: [f32; 4],
    /// set at run time by the game's code, under this name, not by a material
    pub attribute: Option<String>,
}

/// A value the graph reads: the name a material gives it under
/// (`g_flSpeed`, `g_vColor`, `g_bGlow`), how many components (0: a bool),
/// and its value with none given.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub components: u8,
    pub default: [f32; 4],
    pub attribute: Option<String>,
}

/// A graph as WGSL: `graph_material` (and `graph_offset`, if it moves its
/// vertices), with the constants `render/graph.wgsl` reads, and what its
/// texture slots and parameter slots are, in order.
#[derive(Debug)]
pub struct Translated {
    pub wgsl: String,
    pub textures: Vec<Texture>,
    pub params: Vec<Param>,
    pub blend: Blend,
    pub unlit: bool,
    pub offset: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Stage { Pixel, Vertex }

/// A node's result: an expression, how many components it has (0: a
/// bool), and whether it can be written where it's used (a name, a
/// number) rather than kept in a local.
#[derive(Clone)]
struct Val {
    code: String,
    n: u8,
    constant: bool,
}

fn ty(n: u8) -> String {
    match n { 0 => "bool".into(), 1 => "f32".into(), n => format!("vec{n}<f32>") }
}

/// A float as WGSL writes one.
fn lit(f: f32) -> String {
    if !f.is_finite() { return "0.0".into(); }
    let s = format!("{f:?}");
    if s.contains('e') { format!("{:.9}", f) } else { s }
}

fn constructor(v: &[f32]) -> String {
    if v.len() == 1 { return lit(v[0]); }
    format!("vec{}<f32>({})", v.len(), v.iter().map(|x| lit(*x)).collect::<Vec<_>>().join(", "))
}

impl Val {
    // A number a node carries (one typed into it, or an input's default)
    // goes through an identity function (graph.wgsl's g_c1 to g_c4): WGSL
    // works out arithmetic between constants while it compiles and refuses
    // the shader outright where HLSL would shrug (a division by zero, the
    // log of nothing connected). Through a function it's worked out when
    // it runs, as HLSL's is.
    fn float(f: f32) -> Val { Val { code: format!("g_c1({})", lit(f)), n: 1, constant: true } }

    fn vector(v: &[f32]) -> Val { Val { code: format!("g_c{}({})", v.len(), constructor(v)), n: v.len() as u8, constant: true } }

    /// As `n` components, as NodeResult.Cast: the first of more, a scalar
    /// repeated, fewer filled out with zeros.
    fn cast(&self, n: u8) -> String {
        let me = self.n.max(1);
        if me == n { return self.code.clone(); }
        if me > n { return format!("{}.{}", self.code, &"xyzw"[..n as usize]); }
        if me == 1 { return format!("vec{n}<f32>({})", self.code); }
        format!("vec{n}<f32>({}, {})", self.code, vec!["0.0"; (n - me) as usize].join(", "))
    }
}

/// "1,0.5,0" (as the editor saves a vector or a colour), or a number.
fn numbers(v: Option<&J>) -> Vec<f32> {
    match v {
        Some(J::Number(n)) => vec![n.as_f64().unwrap_or(0.0) as f32],
        Some(J::String(s)) => s.split(',').filter_map(|p| p.trim().parse::<f32>().ok()).collect(),
        Some(J::Array(a)) => a.iter().filter_map(|x| x.as_f64().map(|f| f as f32)).collect(),
        _ => Vec::new(),
    }
}

fn number(v: Option<&J>, default: f32) -> f32 {
    numbers(v).first().copied().unwrap_or(default)
}

fn flag(v: Option<&J>, default: bool) -> bool {
    match v { Some(J::Bool(b)) => *b, Some(J::Number(n)) => n.as_f64() != Some(0.0), _ => default }
}

/// An enum, saved by name or by number.
fn choice(v: Option<&J>, names: &[&str]) -> usize {
    match v {
        Some(J::String(s)) => names.iter().position(|n| n.eq_ignore_ascii_case(s)).unwrap_or(0),
        Some(J::Number(n)) => (n.as_u64().unwrap_or(0) as usize).min(names.len().saturating_sub(1)),
        _ => 0,
    }
}

/// A name as GraphCompiler.CleanName leaves it: its letters, digits and underscores.
fn clean(name: &str) -> String {
    name.trim().chars().filter(|c| c.is_alphabetic() || c.is_numeric() || *c == '_').collect()
}

const BLEND_MODES: [&str; 20] = [
    "Mix", "Darken", "Multiply", "ColorBurn", "LinearBurn", "Lighten", "Screen", "ColorDodge", "LinearDodge", "Overlay",
    "SoftLight", "HardLight", "VividLight", "LinearLight", "HardMix", "Difference", "Exclusion", "Subtract", "Divide", "Add",
];

struct Compiler<'a> {
    nodes: HashMap<String, &'a J>,
    stage: Stage,
    memo: HashMap<(Stage, String, String), Option<Val>>,
    locals: HashMap<Stage, Vec<(u8, String, String)>>,
    stack: Vec<(String, String)>,
    textures: Vec<Texture>,
    /// the texture ids taken in each stage, as the compiler's own per-stage table
    stage_textures: HashMap<Stage, Vec<String>>,
    params: Vec<Param>,
    /// helper functions written for this graph (a blend mode at a width), by name
    functions: Vec<(String, String)>,
    error: Option<String>,
}

type Link = Option<(String, String)>;

impl<'a> Compiler<'a> {
    fn fail(&mut self, why: String) {
        if self.error.is_none() { self.error = Some(why); }
    }

    /// What a node's input is connected to: (node, output).
    fn link(node: &J, field: &str) -> Link {
        let v = node.get(field)?;
        let id = v.get("Identifier")?.as_str().filter(|s| !s.is_empty())?;
        Some((id.to_string(), v.get("Output").and_then(J::as_str).unwrap_or("Result").to_string()))
    }

    /// A connection's result, as GraphCompiler.Result: computed once, kept
    /// in a local unless it's a constant.
    fn result(&mut self, link: Link) -> Option<Val> {
        let (id, output) = link?;
        let key = (self.stage, id.clone(), output.clone());
        if let Some(v) = self.memo.get(&key) { return v.clone(); }
        let node = *self.nodes.get(&id)?;
        let class = node.get("_class").and_then(J::as_str).unwrap_or("");
        if class != "Reroute" && self.stack.contains(&(id.clone(), output.clone())) {
            self.fail(format!("node {id} ({class}) takes its own result"));
            return None;
        }
        self.stack.push((id.clone(), output.clone()));
        let made = self.node(node, class, &id, &output);
        self.stack.pop();
        let kept = made.map(|v| {
            if v.constant { return v; }
            let locals = self.locals.entry(self.stage).or_default();
            let name = format!("{}_{}", if self.stage == Stage::Pixel { "l" } else { "v" }, locals.len());
            locals.push((v.n, name.clone(), v.code));
            Val { code: name, n: v.n, constant: true }
        });
        self.memo.insert(key, kept.clone());
        kept
    }

    fn or(&mut self, link: Link, default: f32) -> Val {
        self.result(link).unwrap_or_else(|| Val::float(default))
    }

    /// Two inputs at one width, as GraphCompiler.Result(a, b): the narrower widened.
    fn pair(&mut self, a: Link, b: Link, da: f32, db: f32) -> (Val, Val) {
        let (x, y) = (self.or(a, da), self.or(b, db));
        let n = x.n.max(y.n).max(1);
        (Val { code: x.cast(n), n, constant: x.constant }, Val { code: y.cast(n), n, constant: y.constant })
    }

    /// A value a material can set, as GraphCompiler.ResultParameter: with
    /// no name it's written into the shader.
    fn param(&mut self, name: &str, components: u8, default: [f32; 4], attribute: bool) -> Val {
        if name.trim().is_empty() {
            return match components {
                0 => Val { code: (default[0] != 0.0).to_string(), n: 0, constant: true },
                n => Val::vector(&default[..n as usize]),
            };
        }
        let prefix = match components { 0 => "g_b", 1 => "g_fl", _ => "g_v" };
        let mut full = clean(name);
        if !full.starts_with(prefix) { full = format!("{prefix}{full}"); }
        let slot = match self.params.iter().position(|p| p.name == full) {
            Some(k) => k,
            None => {
                self.params.push(Param { name: full, components, default, attribute: attribute.then(|| name.to_string()) });
                self.params.len() - 1
            }
        };
        if slot >= MAX_PARAMS { self.fail(format!("more than {MAX_PARAMS} parameters")); }
        let code = match self.params[slot].components {
            0 => format!("(graph.v[{slot}].x > 0.5)"),
            1 => format!("graph.v[{slot}].x"),
            2 => format!("graph.v[{slot}].xy"),
            3 => format!("graph.v[{slot}].xyz"),
            _ => format!("graph.v[{slot}]"),
        };
        Val { code, n: self.params[slot].components, constant: true }
    }

    /// A texture node's slot and sampler, as GraphCompiler.ResultTexture
    /// names it: by its name, numbered when the name's taken in this
    /// stage, or by its place in the stage with none.
    fn texture(&mut self, node: &J) -> (usize, &'static str) {
        let ui = node.get("UI");
        let field = |f: &str| ui.and_then(|u| u.get(f));
        let name = clean(field("Name").and_then(J::as_str).unwrap_or(""));
        let taken = self.stage_textures.entry(self.stage).or_default();
        let name = if name.is_empty() { format!("Texture_{}_{}", if self.stage == Stage::Pixel { "ps" } else { "vs" }, taken.len()) } else { name };
        let mut id = name.clone();
        let mut count = 0;
        while taken.contains(&id) {
            id = format!("{name}_{count}");
            count += 1;
        }
        taken.push(id.clone());
        let sampler = node.get("Sampler");
        let address = |f: &str| sampler.and_then(|s| s.get(f)).map(|v| choice(Some(v), &["Wrap", "Mirror", "Clamp", "Border", "Mirror_Once"])).unwrap_or(0);
        let clamp = address("AddressU") >= 2 || address("AddressV") >= 2;
        let default = numbers(field("Default"));
        let attribute = flag(field("IsAttribute"), false);
        let full = format!("g_t{id}");
        let slot = match self.textures.iter().position(|t| t.name == full) {
            Some(k) => k,
            None => {
                self.textures.push(Texture {
                    name: full,
                    srgb: flag(field("SrgbRead"), true),
                    clamp,
                    default: core::array::from_fn(|k| default.get(k).copied().unwrap_or(1.0)),
                    attribute: attribute.then(|| id.clone()),
                });
                self.textures.len() - 1
            }
        };
        if slot >= MAX_TEXTURES { self.fail(format!("more than {MAX_TEXTURES} textures")); }
        (slot, if clamp { "graph_clamp" } else { "graph_wrap" })
    }

    fn sample(&self, slot: usize, sampler: &str, uv: &str) -> String {
        match self.stage {
            Stage::Pixel => format!("textureSample(graph_t{slot}, {sampler}, {uv})"),
            Stage::Vertex => format!("textureSampleLevel(graph_t{slot}, {sampler}, {uv}, 0.0)"),
        }
    }

    /// A blend mode's function at a width, written once: each channel by
    /// the mode's own rule, a fourth the greater alpha, as ShaderTemplate's.
    fn blend_function(&mut self, mode: &str, n: u8) -> String {
        let scalar = format!("g_blend_{}", mode.to_ascii_lowercase());
        if n <= 1 { return scalar; }
        let name = format!("{scalar}_{n}");
        if !self.functions.iter().any(|f| f.0 == name) {
            let channel = |c: char| format!("{scalar}(a.{c}, b.{c})");
            let parts: Vec<String> = match n {
                2 => vec![channel('x'), channel('y')],
                3 => vec![channel('x'), channel('y'), channel('z')],
                _ => vec![channel('x'), channel('y'), channel('z'), "max(a.w, b.w)".to_string()],
            };
            let t = ty(n);
            self.functions.push((name.clone(), format!("fn {name}(a: {t}, b: {t}) -> {t} {{ return {t}({}); }}\n", parts.join(", "))));
        }
        name
    }

    /// One output of one node, as its class compiles it (the classes in
    /// sbox-public's game/editor/ShaderGraph/Code/Nodes, named beside each).
    fn node(&mut self, node: &'a J, class: &str, id: &str, output: &str) -> Option<Val> {
        let link = |f: &str| Self::link(node, f);
        let value = |f: &str| node.get(f);
        let whole = (id.to_string(), "Result".to_string());
        let expr = |code: String, n: u8| Some(Val { code, n, constant: false });
        match class {
            "Reroute" => self.result(link("Input")).or(Some(Val::float(0.0))),

            // Parameter.cs
            "Float" | "Float2" | "Float3" | "Float4" => {
                let n = match class { "Float" => 1, "Float2" => 2, "Float3" => 3, _ => 4 };
                if output == "Result" {
                    let given = numbers(value("Value"));
                    let fill = if class == "Float4" { 1.0 } else { 0.0 };
                    let default: [f32; 4] = core::array::from_fn(|k| given.get(k).copied().unwrap_or(if given.is_empty() { fill } else { 0.0 }));
                    let name = value("Name").and_then(J::as_str).unwrap_or("");
                    return Some(self.param(name, n, default, flag(value("IsAttribute"), false)));
                }
                let component = match output { "X" | "R" => 'x', "Y" | "G" => 'y', "Z" | "B" => 'z', "W" | "A" => 'w', _ => return None };
                let all = self.result(Some(whole))?;
                Some(Val { code: format!("{}.{component}", all.code), n: 1, constant: true })
            }

            // Binary.cs
            "Add" | "Subtract" | "Multiply" | "Divide" | "Mod" => {
                let op = match class { "Add" => "+", "Subtract" => "-", "Multiply" => "*", "Divide" => "/", _ => "%" };
                let (a, b) = self.pair(link("A"), link("B"), number(value("DefaultA"), 0.0), number(value("DefaultB"), 1.0));
                expr(format!("{} {op} {}", a.code, b.code), a.n)
            }
            "Lerp" => {
                let (a, b) = self.pair(link("A"), link("B"), number(value("DefaultA"), 0.0), number(value("DefaultB"), 1.0));
                let fraction = match self.result(link("C")) {
                    Some(f) if f.n > 1 => f.cast(a.n),
                    Some(f) => f.cast(1),
                    None => Val::float(number(value("Fraction"), 0.5)).code,
                };
                expr(format!("mix({}, {}, {fraction})", a.code, b.code), a.n)
            }
            "CrossProduct" => {
                let a = self.result(link("A")).map(|v| v.cast(3)).unwrap_or_else(|| Val::vector(&[0.0; 3]).code);
                let b = self.result(link("B")).map(|v| v.cast(3)).unwrap_or_else(|| Val::vector(&[0.0; 3]).code);
                expr(format!("cross({a}, {b})"), 3)
            }
            "RemapValue" => {
                let inputs = [("A", "In", 0.5), ("B", "InMin", 0.0), ("C", "InMax", 1.0), ("D", "OutMin", 0.0), ("E", "OutMax", 1.0)];
                let got: Vec<Val> = inputs.iter().map(|(input, field, default)| {
                    let default = number(value(field), *default);
                    self.result(link(input)).unwrap_or_else(|| Val::float(default))
                }).collect();
                let n = got.iter().map(|v| v.n.max(1)).max().unwrap_or(1);
                let c: Vec<String> = got.iter().map(|v| v.cast(n)).collect();
                let mut t = format!("({} - {}) / ({} - {})", c[0], c[1], c[2], c[1]);
                if flag(value("Clamp"), true) { t = format!("saturate({t})"); } else { t = format!("({t})"); }
                expr(format!("{t} * ({} - {}) + {}", c[4], c[3], c[3]), n)
            }
            "Arctan2" => {
                let (y, x) = self.pair(link("Y"), link("X"), number(value("DefaultY"), 0.0), number(value("DefaultX"), 1.0));
                expr(format!("atan2({}, {})", y.code, x.code), y.n)
            }
            "Power" => {
                let (a, b) = self.pair(link("A"), link("B"), number(value("DefaultA"), 1.0), number(value("DefaultB"), 1.0));
                expr(format!("pow({}, {})", a.code, b.code), a.n)
            }

            // Unary.cs
            "Cosine" | "Abs" | "DDX" | "DDY" | "DDXY" | "Exponential" | "Frac" | "Floor" | "Length" | "BaseLog" | "Saturate" | "Sine" | "Tan"
            | "Arcsin" | "Arccos" | "Round" | "Ceil" | "Sqrt" | "Normalize" => {
                let input = self.or(link("Input"), 0.0);
                let op = match class {
                    "Cosine" => "cos", "Abs" => "abs", "DDX" => "dpdx", "DDY" => "dpdy", "DDXY" => "fwidth", "Frac" => "fract", "Floor" => "floor",
                    "Length" => "length", "Saturate" => "saturate", "Sine" => "sin", "Tan" => "tan", "Arcsin" => "asin", "Arccos" => "acos",
                    "Round" => "round", "Ceil" => "ceil", "Sqrt" => "sqrt", "Normalize" => "normalize",
                    "Exponential" => if choice(value("Base"), &["BaseE", "Base2"]) == 1 { "exp2" } else { "exp" },
                    _ => match choice(value("Base"), &["BaseE", "Base2", "Base10"]) { 1 => "log2", 2 => "g_log10", _ => "log" },
                };
                if op == "g_log10" { return expr(format!("log({}) * 0.4342944819", input.code), input.n); }
                if matches!(op, "dpdx" | "dpdy" | "fwidth") && self.stage == Stage::Vertex { return Some(Val::float(0.0)); }
                expr(format!("{op}({})", input.code), if class == "Length" { 1 } else { input.n })
            }
            "DotProduct" => {
                let a = self.or(link("InputA"), 0.0);
                let b = self.or(link("InputB"), 0.0).cast(a.n);
                if a.n <= 1 { return expr(format!("{} * {b}", a.code), 1); }
                expr(format!("dot({}, {b})", a.code), 1)
            }
            "Min" | "Max" => {
                let (a, b) = self.pair(link("InputA"), link("InputB"), number(value("DefaultA"), 0.0), number(value("DefaultB"), 0.0));
                expr(format!("{}({}, {})", class.to_ascii_lowercase(), a.code, b.code), a.n)
            }
            "Step" => {
                let (edge, input) = self.pair(link("Edge"), link("Input"), number(value("DefaultEdge"), 0.0), number(value("DefaultInput"), 0.0));
                expr(format!("step({}, {})", edge.code, input.code), edge.n)
            }
            "SmoothStep" => {
                let e1 = self.result(link("Edge1")).unwrap_or_else(|| Val::float(number(value("DefaultEdge1"), 0.0)));
                let e2 = self.result(link("Edge2")).unwrap_or_else(|| Val::float(number(value("DefaultEdge2"), 0.0)));
                let input = self.result(link("Input")).unwrap_or_else(|| Val::float(number(value("DefaultInput"), 0.0)));
                let n = e1.n.max(e2.n).max(input.n).max(1);
                expr(format!("smoothstep({}, {}, {})", e1.cast(n), e2.cast(n), input.cast(n)), n)
            }
            "OneMinus" => {
                let input = self.or(link("In"), 0.0);
                expr(format!("1.0 - {}", input.code), input.n)
            }

            // Channel.cs
            "SplitVector" => {
                let input = self.result(link("Input"));
                let (component, at) = match output { "X" => ('x', 1), "Y" => ('y', 2), "Z" => ('z', 3), "W" => ('w', 4), _ => return None };
                match input {
                    Some(v) if v.n >= at => Some(Val { code: if v.n == 1 { v.code } else { format!("{}.{component}", v.code) }, n: 1, constant: true }),
                    _ => Some(Val::float(0.0)),
                }
            }
            "CombineVector" => {
                let parts: Vec<String> = [("X", "DefaultX"), ("Y", "DefaultY"), ("Z", "DefaultZ"), ("W", "DefaultW")].iter().map(|(input, default)| {
                    let default = number(value(default), 0.0);
                    self.or(link(input), default).cast(1)
                }).collect();
                let n = match output { "XYZW" | "Vector" => 4, "XYZ" => 3, "XY" => 2, _ => return None };
                expr(format!("vec{n}<f32>({})", parts[..n].join(", ")), n as u8)
            }
            "SwizzleVector" => {
                let input = self.result(link("Input"))?;
                let channels = ["Red", "Green", "Blue", "Alpha"];
                let swizzle: String = ["RedOut", "GreenOut", "BlueOut", "AlphaOut"].iter().enumerate().map(|(k, f)| {
                    let c = match value(f) { None => k, v => choice(v, &channels) };
                    ['x', 'y', 'z', 'w'][c]
                }).collect();
                expr(format!("{}.{swizzle}", if input.n == 4 { input.code.clone() } else { format!("({})", input.cast(4)) }), 4)
            }
            "AppendVector" => {
                let (a, b) = (self.or(link("A"), 0.0), self.or(link("B"), 0.0));
                let n = a.n.max(1) + b.n.max(1);
                if n > 4 { self.fail(format!("node {id} appends past four components")); return None; }
                expr(format!("vec{n}<f32>({}, {})", a.code, b.code), n)
            }

            // Time.cs, Geometry.cs
            "Time" => Some(Val { code: "i.time".into(), n: 1, constant: true }),
            "WorldNormal" => Some(Val { code: "i.normal".into(), n: 3, constant: true }),
            "WorldTangent" => Some(Val { code: if output == "V" { "i.tangent_v" } else { "i.tangent_u" }.into(), n: 3, constant: true }),
            "ObjectSpaceNormal" => Some(Val { code: "i.local_normal".into(), n: 3, constant: true }),
            "WorldPosition" => expr("i.world".into(), 3),
            "ObjectPosition" => expr("i.local".into(), 3),
            "ViewDirection" => expr("g_view_dir(i.world)".into(), 3),
            "IsFrontFace" => Some(Val { code: "i.front".into(), n: 1, constant: true }),
            "VertexColor" => if output == "Alpha" { expr("i.color.a".into(), 1) } else { expr("i.color.rgb".into(), 3) },
            "Tint" => expr("i.tint".into(), 4),

            // Texture.cs: a texture is named before its coordinates are looked at
            "TextureSampler" | "TextureTriplanar" | "NormapMapTriplanar" => {
                if output != "Result" {
                    let component = match output { "R" => 'x', "G" => 'y', "B" => 'z', "A" => 'w', _ => return None };
                    return match self.result(Some(whole)) {
                        Some(all) => Some(Val { code: format!("{}.{component}", all.code), n: 1, constant: true }),
                        None => Some(Val::float(0.0)),
                    };
                }
                let (slot, sampler) = self.texture(node);
                let coords = self.result(link("Coords"));
                if class == "TextureSampler" {
                    let uv = coords.map(|c| c.cast(2)).unwrap_or_else(|| "i.uv".into());
                    return expr(self.sample(slot, sampler, &uv), 4);
                }
                let normal = self.result(link("Normal"));
                let at = coords.map(|c| c.cast(3)).unwrap_or_else(|| "(i.world / 39.3701)".into());
                let facing = normal.map(|c| c.cast(3)).unwrap_or_else(|| "normalize(i.normal)".into());
                let lod = if self.stage == Stage::Vertex { "_lod" } else { "" };
                if class == "TextureTriplanar" {
                    expr(format!("g_triplanar_color{lod}(graph_t{slot}, {sampler}, {at}, {facing})"), 4)
                } else {
                    expr(format!("g_triplanar_normal{lod}(graph_t{slot}, {sampler}, {at}, {facing})"), 3)
                }
            }
            "TextureCoord" => {
                // one set of coordinates here: the second set reads as the first
                let tiling = numbers(value("Tiling"));
                let tiling = [tiling.first().copied().unwrap_or(1.0), tiling.get(1).copied().unwrap_or(1.0)];
                if (tiling[0] * tiling[0] + tiling[1] * tiling[1]).sqrt() < 1e-4 { return Some(Val { code: "i.uv".into(), n: 2, constant: true }); }
                expr(format!("i.uv * {}", Val::vector(&tiling).code), 2)
            }

            // Branch.cs
            "Branch" => {
                let (yes, no) = self.pair(link("True"), link("False"), 0.0, 0.0);
                let name = value("Name").and_then(J::as_str).unwrap_or("");
                let condition = if name.trim().is_empty() {
                    let (a, b) = (self.or(link("A"), 0.0).cast(1), self.or(link("B"), 0.0).cast(1));
                    let op = ["==", "!=", ">", "<", ">=", "<="][choice(value("Operator"), &["Equal", "NotEqual", "GreaterThan", "LessThan", "GreaterThanOrEqual", "LessThanOrEqual"])];
                    format!("{a} {op} {b}")
                } else {
                    let on = flag(value("Enabled"), false);
                    self.param(name, 0, [on as i32 as f32, 0.0, 0.0, 0.0], flag(value("IsAttribute"), true)).code
                };
                expr(format!("select({}, {}, {condition})", no.code, yes.code), yes.n)
            }

            // Effects.cs: three components of the one number, as theirs is declared
            "Fresnel" => {
                let default = number(value("DefaultPower"), 10.0);
                let power = self.or(link("Power"), default).cast(1);
                let normal = self.result(link("Normal")).map(|v| v.cast(3)).unwrap_or_else(|| "i.normal".into());
                let direction = self.result(link("Direction")).map(|v| v.cast(3)).unwrap_or_else(|| "g_view_dir(i.world)".into());
                expr(format!("vec3<f32>(pow(1.0 - dot(normalize({normal}), normalize({direction})), {power}))"), 3)
            }

            // Noise.cs
            "FuzzyNoise" | "ValueNoise" | "SimplexNoise" => {
                let f = match class { "FuzzyNoise" => "g_fuzzy_noise", "ValueNoise" => "g_value_noise", _ => "g_simplex_2d" };
                let uv = self.result(link("Coords")).map(|c| c.cast(2)).unwrap_or_else(|| "i.uv".into());
                expr(format!("{f}({uv})"), 1)
            }
            "VoronoiNoise" => {
                let uv = self.result(link("Coords"));
                let angle = self.result(link("A")).map(|v| v.cast(1)).unwrap_or_else(|| Val::float(number(value("AngleOffset"), 3.1415926)).code);
                let density = self.result(link("B")).map(|v| v.cast(1)).unwrap_or_else(|| Val::float(number(value("CellDensity"), 10.0)).code);
                let uv = uv.map(|c| c.cast(2)).unwrap_or_else(|| "i.uv".into());
                expr(format!("{}g_voronoi_noise({uv}, {angle}, {density})", if flag(value("Worley"), false) { "1.0 - " } else { "" }), 1)
            }

            // Transform.cs
            "TransformNormal" => {
                let world_out = choice(value("OutputSpace"), &["Tangent", "World"]) == 1;
                let Some(input) = self.result(link("Input")) else {
                    return Some(Val { code: if world_out { "i.normal".into() } else { "vec3<f32>(0.0, 0.0, 1.0)".into() }, n: 3, constant: true });
                };
                let mut normal = input.cast(3);
                if flag(value("DecodeNormal"), true) { normal = format!("g_decode_normal({normal})"); }
                match choice(value("InputSpace"), &["Tangent", "Object", "World"]) {
                    // an object's tangents aren't kept here: through the world, where they're made
                    1 => normal = format!("g_ws_to_ts(g_object_dir({normal}), i.normal, i.tangent_u, i.tangent_v)"),
                    2 => normal = format!("g_ws_to_ts({normal}, i.normal, i.tangent_u, i.tangent_v)"),
                    _ => {}
                }
                if world_out { normal = format!("g_transform_normal({normal}, i.normal, i.tangent_u, i.tangent_v)"); }
                expr(normal, 3)
            }
            "TileAndOffset" => {
                let coords = self.result(link("Coords"));
                let tile = numbers(value("DefaultTile"));
                let offset = numbers(value("DefaultOffset"));
                let tile = self.result(link("Tile")).map(|v| v.cast(2)).unwrap_or_else(|| Val::vector(&[tile.first().copied().unwrap_or(1.0), tile.get(1).copied().unwrap_or(1.0)]).code);
                let offset = self.result(link("Offset")).map(|v| v.cast(2)).unwrap_or_else(|| Val::vector(&[offset.first().copied().unwrap_or(0.0), offset.get(1).copied().unwrap_or(0.0)]).code);
                let uv = coords.map(|c| c.cast(2)).unwrap_or_else(|| "i.uv".into());
                let mut code = format!("g_tile_and_offset({uv}, {tile}, {offset})");
                if flag(value("WrapTo01"), false) { code = format!("fract({code})"); }
                expr(code, 2)
            }
            "Blend" => {
                let (ra, rb) = (self.result(link("A")), self.result(link("B")));
                let (a, b) = self.pair(link("A"), link("B"), 0.0, 1.0);
                let n = a.n;
                let fraction = match self.result(link("C")) {
                    Some(f) if f.n > 1 => f.cast(n),
                    Some(f) => f.cast(1),
                    None => Val::float(number(value("Fraction"), 0.5)).code,
                };
                let fallback = |field: &str, fill: [f32; 4]| {
                    let given = numbers(value(field));
                    let color: [f32; 4] = core::array::from_fn(|k| given.get(k).copied().unwrap_or(fill[k]));
                    Val::vector(&color).cast(n)
                };
                let a = if ra.is_some() { a.code } else { fallback("DefaultA", [0.0, 0.0, 0.0, 1.0]) };
                let b = if rb.is_some() { b.code } else { fallback("DefaultB", [1.0, 1.0, 1.0, 1.0]) };
                let mode = BLEND_MODES[choice(value("BlendMode"), &BLEND_MODES)];
                let mut code = match mode {
                    "Mix" => format!("mix({a}, {b}, {fraction})"),
                    "Darken" => format!("min({a}, {b})"),
                    "Multiply" => format!("{a} * {b}"),
                    "Lighten" => format!("max({a}, {b})"),
                    "Screen" => format!("({a}) + ({b}) - ({a}) * ({b})"),
                    "Difference" => format!("abs(({a}) - ({b}))"),
                    "Exclusion" => format!("({a}) + ({b}) - 2.0 * ({a}) * ({b})"),
                    "Subtract" => format!("max({}, ({a}) - ({b}))", Val::float(0.0).cast(n)),
                    "Add" => format!("min({}, ({a}) + ({b}))", Val::float(1.0).cast(n)),
                    _ => format!("{}({a}, {b})", self.blend_function(mode, n)),
                };
                if mode != "Mix" { code = format!("mix({a}, {code}, {fraction})"); }
                if flag(value("Clamp"), true) { code = format!("saturate({code})"); }
                expr(code, n)
            }

            "" => None,
            other => {
                self.fail(format!("a {other} node isn't translated yet"));
                None
            }
        }
    }

    /// The locals a stage computed, in the order it reached them.
    fn body(&self, stage: Stage) -> String {
        self.locals.get(&stage).map(|l| l.iter().map(|(n, name, code)| format!("    let {name}: {} = {code};\n", ty(*n))).collect()).unwrap_or_default()
    }
}

/// A graph (a `.shdrgrph`'s text) as WGSL, or why not.
pub fn translate(text: &str) -> Result<Translated, String> {
    let graph: J = serde_json::from_str(text).map_err(|e| format!("not a graph: {e}"))?;
    if flag(graph.get("IsSubgraph"), false) { return Err("a subgraph isn't a material's shader".into()); }
    if choice(graph.get("Domain"), &["Surface", "PostProcess"]) == 1 { return Err("a post-process graph isn't translated yet".into()); }
    let nodes: HashMap<String, &J> = graph.get("nodes").and_then(J::as_array).ok_or("no nodes")?.iter()
        .filter_map(|n| Some((n.get("Identifier")?.as_str()?.to_string(), n))).collect();
    let result = *nodes.values().find(|n| n.get("_class").and_then(J::as_str) == Some("Result")).ok_or("no Result node")?;
    let blend = [Blend::Opaque, Blend::Masked, Blend::Translucent][choice(graph.get("BlendMode"), &["Opaque", "Masked", "Translucent"])];
    let unlit = choice(graph.get("ShadingModel"), &["Lit", "Unlit"]) == 1;
    let mut c = Compiler { nodes, stage: Stage::Pixel, memo: HashMap::new(), locals: HashMap::new(), stack: Vec::new(), textures: Vec::new(),
        stage_textures: HashMap::new(), params: Vec::new(), functions: Vec::new(), error: None };

    // the material, input by input in the Result node's own order
    // (GenerateMaterial); an unlit graph gives a colour and how opaque
    let inputs: &[(&str, &str, u8, Option<(&str, f32)>)] = if unlit {
        &[("Albedo", "albedo", 3, None), ("Opacity", "opacity", 1, Some(("DefaultOpacity", 1.0)))]
    } else {
        &[("Albedo", "albedo", 3, None), ("Emission", "emission", 3, None), ("Opacity", "opacity", 1, Some(("DefaultOpacity", 1.0))),
          ("Normal", "normal", 3, None), ("Roughness", "roughness", 1, Some(("DefaultRoughness", 1.0))),
          ("Metalness", "metalness", 1, Some(("DefaultMetalness", 0.0))), ("AmbientOcclusion", "ao", 1, Some(("DefaultAmbientOcclusion", 1.0)))]
    };
    let mut sets = String::new();
    for (input, field, n, default) in inputs {
        let value = match (Compiler::link(result, input), default) {
            (Some(link), _) => c.result(Some(link)).map(|v| v.cast(*n)),
            (None, Some((name, fallback))) => Some(lit(number(result.get(*name), *fallback))),
            (None, None) => None,
        };
        if let Some(v) = value { sets.push_str(&format!("    m.{field} = {v};\n")); }
    }
    let material = format!("fn graph_material(i: GIn) -> GMat {{\n    var m = g_material();\n{}{sets}    return m;\n}}\n", c.body(Stage::Pixel));

    // where it moves its vertices to (GenerateVertex), if it does
    c.stage = Stage::Vertex;
    let moved = Compiler::link(result, "PositionOffset").and_then(|l| c.result(Some(l))).map(|v| v.cast(3));
    let offset = format!("fn graph_offset(i: GIn) -> vec3<f32> {{\n{}    return {};\n}}\n", c.body(Stage::Vertex), moved.clone().unwrap_or_else(|| "vec3<f32>(0.0)".into()));

    if let Some(e) = c.error { return Err(e); }
    let constants = format!(
        "const GRAPH_BLEND: i32 = {};\nconst GRAPH_UNLIT: bool = {unlit};\nconst GRAPH_OFFSET: bool = {};\n",
        match blend { Blend::Opaque => 0, Blend::Masked => 1, Blend::Translucent => 2 }, moved.is_some());
    let functions: String = c.functions.iter().map(|f| f.1.as_str()).collect();
    Ok(Translated { wgsl: format!("{constants}{functions}{material}{offset}"), textures: c.textures, params: c.params, blend, unlit, offset: moved.is_some() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(blend: &str, nodes: &str) -> String {
        format!(r#"{{"IsSubgraph": false, "BlendMode": "{blend}", "ShadingModel": "Lit", "Domain": "Surface", "nodes": [{nodes}]}}"#)
    }

    #[test]
    fn a_colour_times_a_texture_becomes_a_material() {
        let t = translate(&graph("Opaque", r#"
            {"_class": "Result", "Identifier": "0", "DefaultRoughness": 0.4, "Albedo": {"Identifier": "3", "Output": "Result"}},
            {"_class": "TextureSampler", "Identifier": "1", "UI": {"Name": "Color", "SrgbRead": true, "Default": "1,1,1,1"}, "Sampler": {"AddressU": "Wrap", "AddressV": "Wrap"}},
            {"_class": "Float4", "Identifier": "2", "Name": "Tint Color", "Value": "1,0.5,0.25,1"},
            {"_class": "Multiply", "Identifier": "3", "A": {"Identifier": "1", "Output": "Result"}, "B": {"Identifier": "2", "Output": "Result"}}
        "#)).unwrap();
        assert_eq!(t.textures, vec![Texture { name: "g_tColor".into(), srgb: true, clamp: false, default: [1.0; 4], attribute: None }]);
        assert_eq!(t.params, vec![Param { name: "g_vTintColor".into(), components: 4, default: [1.0, 0.5, 0.25, 1.0], attribute: None }]);
        assert!(t.wgsl.contains("let l_0: vec4<f32> = textureSample(graph_t0, graph_wrap, i.uv);"), "{}", t.wgsl);
        assert!(t.wgsl.contains("let l_1: vec4<f32> = l_0 * graph.v[0];"), "{}", t.wgsl);
        assert!(t.wgsl.contains("m.albedo = l_1.xyz;") && t.wgsl.contains("m.roughness = 0.4;"), "{}", t.wgsl);
        assert!(t.wgsl.contains("const GRAPH_BLEND: i32 = 0;") && !t.offset);
    }

    #[test]
    fn widths_are_matched_as_the_compiler_casts_them() {
        // a number beside a vector is repeated; two components beside three are filled out with a zero
        let t = translate(&graph("Opaque", r#"
            {"_class": "Result", "Identifier": "0", "Albedo": {"Identifier": "3", "Output": "Result"}, "Emission": {"Identifier": "5", "Output": "Result"}},
            {"_class": "Float3", "Identifier": "1", "Value": "1,2,3"},
            {"_class": "Float", "Identifier": "2", "Value": 2},
            {"_class": "Max", "Identifier": "3", "InputA": {"Identifier": "1", "Output": "Result"}, "InputB": {"Identifier": "2", "Output": "Result"}},
            {"_class": "Float2", "Identifier": "4", "Value": "4,5"},
            {"_class": "Add", "Identifier": "5", "A": {"Identifier": "1", "Output": "Result"}, "B": {"Identifier": "4", "Output": "Result"}}
        "#)).unwrap();
        assert!(t.wgsl.contains("max(g_c3(vec3<f32>(1.0, 2.0, 3.0)), vec3<f32>(g_c1(2.0)))"), "{}", t.wgsl);
        assert!(t.wgsl.contains("g_c3(vec3<f32>(1.0, 2.0, 3.0)) + vec3<f32>(g_c2(vec2<f32>(4.0, 5.0)), 0.0)"), "{}", t.wgsl);
        assert!(t.params.is_empty());
    }

    #[test]
    fn textures_are_named_as_the_compiler_names_them() {
        // two of one name: the second numbered; one with none: by its place; each before its own coordinates
        let t = translate(&graph("Masked", r#"
            {"_class": "Result", "Identifier": "0", "Albedo": {"Identifier": "1", "Output": "Result"}, "Opacity": {"Identifier": "3", "Output": "A"}},
            {"_class": "TextureSampler", "Identifier": "1", "UI": {"Name": "Noise One"}, "Coords": {"Identifier": "2", "Output": "Result"}},
            {"_class": "TextureSampler", "Identifier": "2", "UI": {"Name": "Noise One", "SrgbRead": false}, "Sampler": {"AddressU": "Clamp", "AddressV": "Clamp"}},
            {"_class": "TextureSampler", "Identifier": "3", "UI": {"Name": ""}}
        "#)).unwrap();
        let names: Vec<&str> = t.textures.iter().map(|x| x.name.as_str()).collect();
        assert_eq!(names, ["g_tNoiseOne", "g_tNoiseOne_0", "g_tTexture_ps_2"]);
        assert!(t.textures[1].clamp && !t.textures[1].srgb && !t.textures[0].clamp);
        assert!(t.wgsl.contains("textureSample(graph_t1, graph_clamp, i.uv)") && t.wgsl.contains("m.opacity = l_2.w;"), "{}", t.wgsl);
        assert_eq!(t.blend, Blend::Masked);
    }

    #[test]
    fn a_switch_a_material_sets_chooses_between_two_inputs() {
        let t = translate(&graph("Translucent", r#"
            {"_class": "Result", "Identifier": "0", "Albedo": {"Identifier": "3", "Output": "Result"}, "PositionOffset": {"Identifier": "5", "Output": "Result"}},
            {"_class": "Float4", "Identifier": "1", "Name": "Color", "Value": "1,0,0,1"},
            {"_class": "Float4", "Identifier": "2", "Name": "Color B", "Value": "0,0,1,1"},
            {"_class": "Branch", "Identifier": "3", "Name": "Gradient Tint", "Enabled": true, "IsAttribute": true, "True": {"Identifier": "1", "Output": "Result"}, "False": {"Identifier": "2", "Output": "Result"}},
            {"_class": "Time", "Identifier": "4"},
            {"_class": "Sine", "Identifier": "5", "Input": {"Identifier": "4", "Output": "Result"}}
        "#)).unwrap();
        assert_eq!(t.params.iter().map(|p| (p.name.as_str(), p.components)).collect::<Vec<_>>(), [("g_vColor", 4), ("g_vColorB", 4), ("g_bGradientTint", 0)]);
        assert_eq!(t.params[2].default[0], 1.0);
        assert_eq!(t.params[2].attribute.as_deref(), Some("Gradient Tint"));
        assert!(t.wgsl.contains("select(graph.v[1], graph.v[0], (graph.v[2].x > 0.5))"), "{}", t.wgsl);
        // its vertices move by a sine of the time: computed in the vertex stage's own locals
        assert!(t.offset && t.wgsl.contains("let v_0: f32 = sin(i.time);") && t.wgsl.contains("return vec3<f32>(v_0);"), "{}", t.wgsl);
    }

    #[test]
    fn what_isnt_translated_is_said() {
        let e = translate(&graph("Opaque", r#"
            {"_class": "Result", "Identifier": "0", "Albedo": {"Identifier": "1", "Output": "Result"}},
            {"_class": "SceneColor", "Identifier": "1"}
        "#)).unwrap_err();
        assert!(e.contains("SceneColor"), "{e}");
        assert!(translate("{}").is_err());
    }
}
