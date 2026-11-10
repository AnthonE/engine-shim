//! A shader graph's shader, put together: shader.wgsl without its stock
//! materials (what's marked `<stock>` there: complex, blendable and glass's
//! own textures and fragment stages), graph.wgsl (a graph's inputs, the
//! helpers its nodes call, its stages), and what assets/shadergraph.rs
//! wrote for the graph.

/// The WGSL of a graph's shader module: `generated` is a translated
/// graph's (`Translated::wgsl`).
pub fn source(generated: &str) -> String {
    let mut out = String::new();
    let mut stock = false;
    for line in include_str!("shader.wgsl").lines() {
        let l = line.trim_start();
        if l.starts_with("// <stock>") { stock = true; continue; }
        if l.starts_with("// </stock>") { stock = false; continue; }
        if !stock {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.push_str(include_str!("graph.wgsl"));
    out.push_str(generated);
    out
}

/// Is it WGSL that means something: parsed and validated as wgpu will, so a
/// graph that translates into nonsense is refused before a pipeline is made
/// of it (which would panic).
pub fn check(source: &str) -> Result<(), String> {
    use wgpu::naga;
    let module = naga::front::wgsl::parse_str(source).map_err(|e| e.emit_to_string(source))?;
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&module).map(|_| ()).map_err(|e| e.emit_to_string(source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assets::shadergraph::translate;

    #[test]
    fn a_graph_of_every_kind_of_node_is_a_shader_wgpu_takes() {
        // one of each node translated, wired however their widths allow: what
        // must hold is that the WGSL written for each is valid beside the rest
        let unary = ["Cosine", "Abs", "DDX", "DDY", "DDXY", "Exponential", "Frac", "Floor", "Length", "BaseLog", "Saturate", "Sine", "Tan", "Arcsin", "Arccos", "Round", "Ceil", "Sqrt", "Normalize"];
        let binary = ["Add", "Subtract", "Multiply", "Divide", "Mod", "Power", "Min", "Max", "Step", "Arctan2", "DotProduct", "CrossProduct"];
        let modes = ["Mix", "Darken", "Multiply", "ColorBurn", "LinearBurn", "Lighten", "Screen", "ColorDodge", "LinearDodge", "Overlay", "SoftLight", "HardLight", "VividLight", "LinearLight", "HardMix", "Difference", "Exclusion", "Subtract", "Divide", "Add"];
        let mut nodes = vec![
            r#"{"_class": "TextureSampler", "Identifier": "tex", "UI": {"Name": "Color"}, "Coords": {"Identifier": "tile", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "TextureTriplanar", "Identifier": "tri", "UI": {"Name": "Rock"}}"#.to_string(),
            r#"{"_class": "NormapMapTriplanar", "Identifier": "trin", "UI": {"Name": "Rock Normal", "SrgbRead": false}}"#.to_string(),
            r#"{"_class": "TileAndOffset", "Identifier": "tile", "Coords": {"Identifier": "uv", "Output": "Result"}, "Offset": {"Identifier": "time", "Output": "Result"}, "WrapTo01": true}"#.to_string(),
            r#"{"_class": "TextureCoord", "Identifier": "uv", "Tiling": "2,2"}"#.to_string(),
            r#"{"_class": "Time", "Identifier": "time"}"#.to_string(),
            r#"{"_class": "Float", "Identifier": "f", "Name": "Speed", "Value": 2}"#.to_string(),
            r#"{"_class": "Float2", "Identifier": "f2", "Name": "Scroll", "Value": "1,2"}"#.to_string(),
            r#"{"_class": "Float3", "Identifier": "f3", "Value": "1,2,3"}"#.to_string(),
            r#"{"_class": "Float4", "Identifier": "f4", "Name": "Tint", "Value": "1,1,1,1"}"#.to_string(),
            r#"{"_class": "SplitVector", "Identifier": "split", "Input": {"Identifier": "tex", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "CombineVector", "Identifier": "combine", "X": {"Identifier": "split", "Output": "Z"}, "Y": {"Identifier": "f2", "Output": "Y"}, "W": {"Identifier": "f4", "Output": "A"}}"#.to_string(),
            r#"{"_class": "SwizzleVector", "Identifier": "swizzle", "Input": {"Identifier": "f3", "Output": "Result"}, "RedOut": "Blue", "BlueOut": "Red"}"#.to_string(),
            r#"{"_class": "AppendVector", "Identifier": "append", "A": {"Identifier": "f2", "Output": "Result"}, "B": {"Identifier": "f", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "Lerp", "Identifier": "lerp", "A": {"Identifier": "tex", "Output": "Result"}, "B": {"Identifier": "tri", "Output": "Result"}, "C": {"Identifier": "voronoi", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "RemapValue", "Identifier": "remap", "A": {"Identifier": "f3", "Output": "Result"}, "OutMax": 4}"#.to_string(),
            r#"{"_class": "SmoothStep", "Identifier": "smooth", "Input": {"Identifier": "f3", "Output": "Result"}, "DefaultEdge2": 1}"#.to_string(),
            r#"{"_class": "OneMinus", "Identifier": "oneminus", "In": {"Identifier": "smooth", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "Branch", "Identifier": "branch", "Name": "Glow", "True": {"Identifier": "f3", "Output": "Result"}, "False": {"Identifier": "f", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "Branch", "Identifier": "compare", "Operator": "GreaterThan", "A": {"Identifier": "f", "Output": "Result"}, "True": {"Identifier": "f4", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "Fresnel", "Identifier": "fresnel"}"#.to_string(),
            r#"{"_class": "FuzzyNoise", "Identifier": "fuzzy"}"#.to_string(),
            r#"{"_class": "ValueNoise", "Identifier": "valuenoise", "Coords": {"Identifier": "uv", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "SimplexNoise", "Identifier": "simplex"}"#.to_string(),
            r#"{"_class": "VoronoiNoise", "Identifier": "voronoi", "Worley": true, "B": {"Identifier": "f", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "TransformNormal", "Identifier": "tn", "Input": {"Identifier": "tex", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "TransformNormal", "Identifier": "tnobject", "Input": {"Identifier": "osn", "Output": "Result"}, "InputSpace": "Object", "DecodeNormal": false}"#.to_string(),
            r#"{"_class": "TransformNormal", "Identifier": "tnworld", "Input": {"Identifier": "trin", "Output": "Result"}, "InputSpace": "World", "OutputSpace": "World", "DecodeNormal": false}"#.to_string(),
            r#"{"_class": "ObjectSpaceNormal", "Identifier": "osn"}"#.to_string(),
            r#"{"_class": "WorldPosition", "Identifier": "wp"}"#.to_string(),
            r#"{"_class": "ObjectPosition", "Identifier": "op"}"#.to_string(),
            r#"{"_class": "WorldNormal", "Identifier": "wn"}"#.to_string(),
            r#"{"_class": "ViewDirection", "Identifier": "vd"}"#.to_string(),
            r#"{"_class": "VertexColor", "Identifier": "vc"}"#.to_string(),
            r#"{"_class": "Tint", "Identifier": "tint"}"#.to_string(),
            r#"{"_class": "Reroute", "Identifier": "reroute", "Input": {"Identifier": "vc", "Output": "RGB"}}"#.to_string(),
            r#"{"_class": "Distance", "Identifier": "distance", "A": {"Identifier": "wp", "Output": "Result"}, "B": {"Identifier": "camera", "Output": "WorldPosition"}}"#.to_string(),
            r#"{"_class": "Camera", "Identifier": "camera"}"#.to_string(),
            r#"{"_class": "Sun", "Identifier": "sun"}"#.to_string(),
            r#"{"_class": "ScreenPosition", "Identifier": "screen"}"#.to_string(),
            r#"{"_class": "ScreenCoordinate", "Identifier": "screenuv"}"#.to_string(),
            r#"{"_class": "TangentViewVector", "Identifier": "tvv"}"#.to_string(),
            r#"{"_class": "PolarCoordinates", "Identifier": "polar", "RadialScale": {"Identifier": "f", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "NormalBlend", "Identifier": "nb", "A": {"Identifier": "tn", "Output": "Result"}, "B": {"Identifier": "trin", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "NormalBlend", "Identifier": "nbr", "Mode": "Reoriented", "A": {"Identifier": "tn", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "Reflection", "Identifier": "reflect", "A": {"Identifier": "vd", "Output": "Result"}, "B": {"Identifier": "wn", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "RGBtoHSV", "Identifier": "hsv", "In": {"Identifier": "tex", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "HSVtoRGB", "Identifier": "rgb", "In": {"Identifier": "hsv", "Output": "Out"}}"#.to_string(),
            r#"{"_class": "SrgbGammaToLinear", "Identifier": "tolinear", "In": {"Identifier": "f3", "Output": "Result"}}"#.to_string(),
            r#"{"_class": "SrgbLinearToGamma", "Identifier": "togamma"}"#.to_string(),
        ];
        // every result is added into one sum, so each node is reached
        let mut reached: Vec<(String, &str)> = ["tex", "tri", "trin", "combine", "swizzle", "append", "lerp", "remap", "oneminus", "branch", "compare", "fresnel", "fuzzy",
            "valuenoise", "simplex", "tn", "tnobject", "tnworld", "wp", "op", "wn", "vd", "tint", "reroute"].iter().map(|n| (n.to_string(), "Result")).collect();
        reached[3].1 = "XYZW";
        reached[4].1 = "Output";
        reached[5].1 = "Output";
        reached.push(("vc".into(), "Alpha"));
        for n in ["distance", "tvv", "polar", "nb", "nbr", "reflect", "tolinear", "togamma", "screenuv"] { reached.push((n.into(), "Result")); }
        reached.extend([("camera".into(), "Direction"), ("sun".into(), "Direction"), ("sun".into(), "Color"), ("screen".into(), "XYZ"), ("screen".into(), "W"), ("rgb".into(), "Out")]);
        reached.push(("combine".into(), "XY"));
        for u in unary {
            nodes.push(format!(r#"{{"_class": "{u}", "Identifier": "u{u}", "Input": {{"Identifier": "f3", "Output": "Result"}}}}"#));
            reached.push((format!("u{u}"), "Result"));
        }
        for b in binary {
            let (a, c) = match b { "Min" | "Max" | "DotProduct" => ("InputA", "InputB"), "Step" => ("Input", "Edge"), "Arctan2" => ("Y", "X"), _ => ("A", "B") };
            nodes.push(format!(r#"{{"_class": "{b}", "Identifier": "b{b}", "{a}": {{"Identifier": "f3", "Output": "Result"}}, "{c}": {{"Identifier": "f", "Output": "Result"}}}}"#));
            reached.push((format!("b{b}"), "Result"));
        }
        for (k, m) in modes.iter().enumerate() {
            // at each width the blend functions are written for
            let (a, b) = [("f", "f"), ("f2", "f2"), ("f3", "tex"), ("tex", "f4")][k % 4];
            nodes.push(format!(r#"{{"_class": "Blend", "Identifier": "m{m}", "BlendMode": "{m}", "A": {{"Identifier": "{a}", "Output": "Result"}}, "B": {{"Identifier": "{b}", "Output": "Result"}}}}"#));
            reached.push((format!("m{m}"), "Result"));
        }
        let mut last = "f".to_string();
        for (k, (node, output)) in reached.iter().enumerate() {
            nodes.push(format!(r#"{{"_class": "Add", "Identifier": "sum{k}", "A": {{"Identifier": "{last}", "Output": "Result"}}, "B": {{"Identifier": "{node}", "Output": "{output}"}}}}"#));
            last = format!("sum{k}");
        }
        nodes.push(format!(r#"{{"_class": "Result", "Identifier": "0", "Albedo": {{"Identifier": "{last}", "Output": "Result"}}, "Normal": {{"Identifier": "tn", "Output": "Result"}}, "Opacity": {{"Identifier": "voronoi", "Output": "Result"}}, "PositionOffset": {{"Identifier": "lerp", "Output": "Result"}}}}"#));
        let text = format!(r#"{{"BlendMode": "Translucent", "ShadingModel": "Lit", "Domain": "Surface", "nodes": [{}]}}"#, nodes.join(","));
        let t = translate(&text).unwrap();
        assert!(t.offset && t.textures.len() == 3 && t.params.len() == 4, "{:?} {:?}", t.textures, t.params);
        if let Err(e) = check(&source(&t.wgsl)) { panic!("{e}\n{}", t.wgsl); }
        // unlit, opaque and unmoved is one too
        let plain = translate(r#"{"BlendMode": "Opaque", "ShadingModel": "Unlit", "nodes": [{"_class": "Result", "Identifier": "0"}]}"#).unwrap();
        check(&source(&plain.wgsl)).unwrap();
    }

    /// How many of the graphs under `ENGINE2_GRAPH_SURVEY` (other people's,
    /// fetched for the count and never committed) translate into a shader
    /// wgpu takes, and what stops the rest, most common first. It says, and
    /// fails nothing: what to translate next is read off it.
    #[test]
    fn a_survey_of_graphs() {
        let Ok(root) = std::env::var("ENGINE2_GRAPH_SURVEY") else { return };
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
                let p = e.path();
                if p.is_dir() { walk(&p, out); } else if p.extension().is_some_and(|x| x == "shdrgrph") { out.push(p); }
            }
        }
        let mut graphs = Vec::new();
        walk(std::path::Path::new(&root), &mut graphs);
        let (mut good, mut subgraphs) = (0, 0);
        let mut why: std::collections::BTreeMap<String, Vec<String>> = Default::default();
        for g in &graphs {
            let name = g.strip_prefix(&root).unwrap_or(g).to_string_lossy().to_string();
            let Ok(text) = std::fs::read_to_string(g) else { why.entry("unreadable".into()).or_default().push(name); continue };
            match translate(text.trim_start_matches('\u{feff}')) {
                Ok(t) => match check(&source(&t.wgsl)) {
                    Ok(()) => good += 1,
                    Err(e) => why.entry(format!("not valid WGSL: {}", e.lines().next().unwrap_or(""))).or_default().push(name),
                },
                Err(e) if e.contains("a subgraph isn't") => subgraphs += 1,
                Err(e) => why.entry(e).or_default().push(name),
            }
        }
        let mut rows: Vec<(String, Vec<String>)> = why.into_iter().collect();
        rows.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        println!("survey: {} graphs; {subgraphs} are subgraphs (a piece of a shader, not one); of the other {}, {good} translate into valid WGSL", graphs.len(), graphs.len() - subgraphs);
        for (reason, names) in &rows { println!("survey: {:3}  {reason}  (e.g. {})", names.len(), names[0]); }
        // by whose they are (the folder each was put in): how many of each translate
        let mut whose: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
        for g in &graphs {
            let name = g.strip_prefix(&root).unwrap_or(g).to_string_lossy().to_string();
            let owner = name.split('/').next().unwrap_or("").to_string();
            let failed = rows.iter().any(|(_, names)| names.contains(&name));
            let e = whose.entry(owner).or_default();
            e.0 += 1;
            e.1 += !failed as usize;
        }
        let whole = whose.values().filter(|(n, ok)| n == ok).count();
        println!("survey: {} sources; every graph of {whole} of them translates", whose.len());
        for (owner, (n, ok)) in whose.iter().filter(|(_, (n, ok))| n != ok) { println!("survey:   {owner}: {ok} of {n}"); }
    }

    /// Every graph under `ENGINE2_SAMPLE_GRAPHS` (a game's checkout, never
    /// committed here): translated, valid, and its textures and parameters
    /// named as s&box's own compiler named them in the `.shader` it wrote
    /// beside the graph.
    #[test]
    fn a_games_graphs_translate_and_name_what_its_own_shaders_name() {
        let Ok(root) = std::env::var("ENGINE2_SAMPLE_GRAPHS") else { return };
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
                let p = e.path();
                if p.is_dir() { walk(&p, out); } else if p.extension().is_some_and(|x| x == "shdrgrph") { out.push(p); }
            }
        }
        let mut graphs = Vec::new();
        walk(std::path::Path::new(&root), &mut graphs);
        graphs.sort();
        assert!(!graphs.is_empty(), "no graphs under {root}");
        let mut failed = Vec::new();
        for g in &graphs {
            let name = g.file_name().unwrap().to_string_lossy().to_string();
            let t = match translate(&std::fs::read_to_string(g).unwrap()) {
                Ok(t) => t,
                Err(e) => { failed.push(format!("{name}: {e}")); continue; }
            };
            if let Err(e) = check(&source(&t.wgsl)) { failed.push(format!("{name}: {e}")); continue; }
            // the names in the shader s&box's compiler wrote for the same graph
            let Ok(shader) = std::fs::read_to_string(g.with_extension("shader")) else { println!("{name}: ok ({} textures, {} parameters; no .shader beside it to compare)", t.textures.len(), t.params.len()); continue };
            let declared = |prefix: &str| -> Vec<String> {
                let mut names: Vec<String> = shader.lines().filter_map(|l| {
                    let l = l.trim();
                    let rest = l.strip_prefix("Texture2D ").or(l.strip_prefix("float4 ")).or(l.strip_prefix("float3 ")).or(l.strip_prefix("float2 ")).or(l.strip_prefix("float ")).or(l.strip_prefix("bool "))?;
                    let name = rest.split(|c: char| !(c.is_alphanumeric() || c == '_')).next()?;
                    (name.starts_with(prefix) && l.contains('<')).then(|| name.to_string())
                }).collect();
                names.sort();
                names.dedup();
                names
            };
            let mut textures: Vec<String> = t.textures.iter().map(|x| x.name.clone()).collect();
            textures.sort();
            if textures != declared("g_t") { failed.push(format!("{name}: textures {textures:?}, its shader's {:?}", declared("g_t"))); continue; }
            let mut params: Vec<String> = t.params.iter().map(|x| x.name.clone()).collect();
            params.sort();
            let theirs: Vec<String> = ["g_fl", "g_v", "g_b"].iter().flat_map(|p| declared(p)).filter(|n| n != "g_flPreviewTime").collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            if params != theirs { failed.push(format!("{name}: parameters {params:?}, its shader's {theirs:?}")); continue; }
            println!("{name}: ok ({} textures, {} parameters, named as its shader names them)", t.textures.len(), t.params.len());
        }
        assert!(failed.is_empty(), "{} of {} graphs:\n{}", failed.len(), graphs.len(), failed.join("\n"));
    }
}
