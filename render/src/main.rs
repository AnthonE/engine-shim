//! engine2's renderer from the command line, from content on disk
//! (tools/package.sh; a map's `.vpk` among the places):
//!   ENGINE2_CONTENT=../content render models/sbox_props/tv/tv.vmdl tv.png [yaw] [pitch] [size]
//!   ENGINE2_CONTENT=../maps/flatgrass.vpk:../content render map maps/flatgrass out.png x y z pitch yaw [width height]
use engine2::assets::{entities, kv3::Value, resource::Resource, worldnode};
use engine2::render::{BakedLight, Content, Files, Instance, ProbeVolume, Renderer, View};
use glam::{Mat4, Quat, Vec3};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content = Content::of(std::env::var("ENGINE2_CONTENT").unwrap_or_default().split(':').filter(|d| !d.is_empty()).map(Into::into));
    let num = |i: usize, d: f32| args.get(i).and_then(|a| a.parse().ok()).unwrap_or(d);
    let r = Renderer::new().unwrap_or_else(|e| fail(&e));
    // RENDER_TONEMAP: hable (s&box's default), aces, agx, reinhard, linear;
    // RENDER_EXPOSURE: a scale, or auto (s&box's default: 1 to 3)
    let tone = |v: &mut View| {
        if let Some(t) = std::env::var("RENDER_TONEMAP").ok().and_then(|t| engine2::render::Tonemap::named(&t)) { v.tonemap = t; }
        if let Some(e) = std::env::var("RENDER_EXPOSURE").ok().and_then(|e| e.parse::<f32>().ok()) { v.exposure = engine2::render::Exposure::Fixed(e); }
    };
    let t = std::time::Instant::now();
    if args.get(1).map(String::as_str) == Some("map") {
        let (Some(folder), Some(out)) = (args.get(2), args.get(3)) else { usage() };
        let (w, h) = (num(9, 960.0) as u32, num(10, 540.0) as u32);
        let mut view = View::camera(Vec3::new(num(4, 0.0), num(5, 0.0), num(6, 64.0)), [num(7, 0.0), num(8, 0.0), 0.0], 75.0, 4.0, 40000.0, w as f32 / h as f32);
        tone(&mut view);
        // the map's sun and sky, as its entities name them
        for e in lump(&content, folder) {
            let get = |k: &str| e.get(engine2::string_token(k)).map(str::to_string);
            let floats = |k: &str| get(k).map(|v| v.split_whitespace().filter_map(|x| x.parse::<f32>().ok()).collect::<Vec<_>>()).unwrap_or_default();
            match get("classname").as_deref() {
                Some("light_environment") => {
                    let a = floats("angles");
                    let rot = Quat::from_rotation_z(a.get(1).copied().unwrap_or(0.0).to_radians()) * Quat::from_rotation_y(a.first().copied().unwrap_or(0.0).to_radians());
                    let c = floats("color");
                    let b = floats("brightness").first().copied().unwrap_or(1.0);
                    view.sun = Some((rot * -Vec3::X, Vec3::new(c.first().copied().unwrap_or(255.0), c.get(1).copied().unwrap_or(255.0), c.get(2).copied().unwrap_or(255.0)) / 255.0 * b));
                }
                Some("env_sky") => match get("skyname").map(|s| r.sky(&content, &s)) {
                    Some(Ok(s)) => {
                        view.sky = Some(s);
                        let a = floats("angles");
                        view.sky_rotation = Quat::from_rotation_z(a.get(1).copied().unwrap_or(0.0).to_radians()) * Quat::from_rotation_y(a.first().copied().unwrap_or(0.0).to_radians());
                    }
                    Some(Err(e)) => eprintln!("  {e}"),
                    None => {}
                },
                _ => {}
            }
        }
        let mut notes = Vec::new();
        // its baked lighting, lit by its lights (RENDER_LIGHTMAPS=0: without)
        let entities = lump(&content, folder);
        let lights = BakedLight::from_entities(&entities);
        if std::env::var("RENDER_LIGHTMAPS").as_deref() != Ok("0") {
            match r.lightmaps(&content, folder, &lights) {
                Ok(l) => view.lightmaps = l.map(std::sync::Arc::new),
                Err(e) => notes.push(e),
            }
            // and its light probe volumes, for what isn't the map's own
            for (place, [t, i, s]) in ProbeVolume::from_entities(&entities) {
                match r.probe_volume(&content, &t, &i, &s, place, &lights) {
                    Ok(v) => view.probes.push(std::sync::Arc::new(v)),
                    Err(e) => notes.push(e),
                }
            }
        }
        // models to place in it (RENDER_PROPS: name@x,y,z;...), lit by its probes
        let props: Vec<(engine2::render::Model, Vec3)> = std::env::var("RENDER_PROPS").unwrap_or_default().split(';').filter(|p| !p.is_empty()).filter_map(|p| {
            let (name, at) = p.split_once('@')?;
            let v: Vec<f32> = at.split(',').filter_map(|x| x.parse().ok()).collect();
            match r.model(&content, name) {
                Ok(m) => Some((m, Vec3::new(v.first().copied().unwrap_or(0.0), v.get(1).copied().unwrap_or(0.0), v.get(2).copied().unwrap_or(0.0)))),
                Err(e) => { notes.push(e); None }
            }
        }).collect();
        let mut models = Vec::new();
        for (name, rows, tint) in worldnode::scene_objects(folder, |p| content.read(p)) {
            match r.model(&content, &name) {
                Ok(m) => { notes.extend(m.notes.clone()); models.push((m, rows, tint)); }
                Err(e) => notes.push(e),
            }
        }
        let instances: Vec<Instance> = models.iter().map(|(m, rows, tint)| Instance {
            model: m,
            transform: Mat4::from_cols_array_2d(&[
                [rows[0][0], rows[1][0], rows[2][0], 0.0],
                [rows[0][1], rows[1][1], rows[2][1], 0.0],
                [rows[0][2], rows[1][2], rows[2][2], 0.0],
                [rows[0][3], rows[1][3], rows[2][3], 1.0],
            ]),
            tint: *tint,
            bones: Vec::new(),
            lightmapped: true,
            origin: None,
            mask: None,
            skin: 0,
        }).chain(props.iter().map(|(m, at)| Instance { model: m, transform: Mat4::from_translation(*at), tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 })).collect();
        let loaded = t.elapsed();
        let image = r.draw(&instances, &view, w, h);
        png(std::path::Path::new(out), w, h, &image).unwrap_or_else(|e| fail(&e));
        eprintln!("{folder} on {}: {} objects, sun {:?}, sky {}, lightmaps {} ({} baked lights, {} probe volumes), loaded in {loaded:.2?}, drawn in {:.2?}", r.adapter, instances.len(), view.sun, view.sky.is_some(), view.lightmaps.is_some(), lights.len(), view.probes.len(), t.elapsed() - loaded);
        notes.sort();
        notes.dedup();
        for n in notes { eprintln!("  {n}"); }
        return;
    }
    // what a material says (its shader, textures and values), for looking into one:
    //   render material materials/x.vmat [more...]
    if args.get(1).map(String::as_str) == Some("material") {
        for name in &args[2..] {
            match content.compiled(name).ok_or_else(|| format!("{name}: not found")).and_then(|r| engine2::assets::material::read(&r)) {
                Ok(m) => {
                    println!("{name}: shader {} (version {})", m.shader, m.shader_version);
                    let mut rows: Vec<String> = m.textures.iter().map(|(k, v)| format!("  texture {k} = {v}")).collect();
                    rows.extend(m.ints.iter().map(|(k, v)| format!("  int {k} = {v}")));
                    rows.extend(m.floats.iter().map(|(k, v)| format!("  float {k} = {v}")));
                    rows.extend(m.vectors.iter().map(|(k, v)| format!("  vector {k} = {v:?}")));
                    rows.sort();
                    for r in rows { println!("{r}"); }
                }
                Err(e) => println!("{e}"),
            }
        }
        return;
    }
    // a material on a ball, for looking at one (a shader graph's, say):
    //   render ball materials/x.vmat out.png [yaw pitch size]
    if args.get(1).map(String::as_str) == Some("ball") {
        let (Some(material), Some(out)) = (args.get(2), args.get(3)) else { usage() };
        let m = r.model_of(&content, "a ball", &[ball(material, 32.0)], 1, &[]).unwrap_or_else(|e| fail(&e));
        for n in &m.notes { eprintln!("{n}"); }
        let instance = [Instance { model: &m, transform: Default::default(), tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 }];
        let view = { let mut v = View::framing(&m, num(4, 35.0), num(5, 20.0)); tone(&mut v); v };
        let size = num(6, 512.0) as u32;
        let pixels = r.draw(&instance, &view, size, size);
        png(std::path::Path::new(out), size, size, &pixels).unwrap_or_else(|e| fail(&e));
        return;
    }
    // the default player's stand-in (engine2/src/standin.rs), in one of its sequences:
    //   render standin out.png [yaw pitch size] [sequence] [cycle]
    if args.get(1).map(String::as_str) == Some("standin") {
        let Some(out) = args.get(2) else { usage() };
        let name = "models/citizen/citizen.vmdl";
        let (meshes, default, skins) = engine2::standin::draw_meshes(name).unwrap_or_else(|| fail("the stand-in is off"));
        let m = r.model_of(&content, name, &meshes, default, &skins).unwrap_or_else(|e| fail(&e));
        let sequence = args.get(6).map(String::as_str).unwrap_or("idle");
        let bones = engine2::standin::skin_at(sequence, num(7, 0.0)).unwrap_or_else(|| fail(&format!("no sequence {sequence}: {}", engine2::standin::sequences().join(" "))));
        let instance = [Instance { model: &m, transform: Default::default(), tint: [1.0; 4], bones, lightmapped: false, origin: None, mask: None, skin: 0 }];
        let view = { let mut v = View::framing(&m, num(3, 35.0), num(4, 20.0)); tone(&mut v); v };
        let size = num(5, 512.0) as u32;
        let pixels = r.draw(&instance, &view, size, size);
        png(std::path::Path::new(out), size, size, &pixels).unwrap_or_else(|e| fail(&e));
        return;
    }
    let (Some(model), Some(out)) = (args.get(1), args.get(2)) else { usage() };
    let m = r.model(&content, model).unwrap_or_else(|e| fail(&e));
    let loaded = t.elapsed();
    let size = num(5, 768.0) as u32;
    let instance = [Instance { model: &m, transform: Default::default(), tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 }];
    let view = { let mut v = View::framing(&m, num(3, 35.0), num(4, 20.0)); tone(&mut v); v };
    // RENDER_REPEAT: drawn this many times more first (with RENDER_PROFILE, to time a warm frame)
    for _ in 0..std::env::var("RENDER_REPEAT").ok().and_then(|n| n.parse::<u32>().ok()).unwrap_or(0) { r.draw(&instance, &view, size, size); }
    let image = r.draw(&instance, &view, size, size);
    png(std::path::Path::new(out), size, size, &image).unwrap_or_else(|e| fail(&e));
    eprintln!("{model} on {}: bounds {:?}..{:?}, loaded in {loaded:.2?}, drawn in {:.2?}", r.adapter, m.min, m.max, t.elapsed() - loaded);
    for n in &m.notes { eprintln!("  {n}"); }
}

/// A map's entities (its default lump).
fn lump(content: &Content, folder: &str) -> Vec<entities::Entity> {
    content.read(&format!("{folder}/entities/default_ents.vents_c"))
        .and_then(|b| Resource::parse(b).ok())
        .and_then(|r| r.kv3("DATA")?.ok())
        .map(|d: Value| entities::read_lump(&d))
        .unwrap_or_default()
}

/// A ball of one material: positions, normals and coordinates that go
/// twice around it.
fn ball(material: &str, radius: f32) -> engine2::assets::mesh::Mesh {
    use engine2::assets::mesh::{Attribute, Buffers, Draw, IndexBuffer, Mesh, VertexBuffer};
    let (rings, segments) = (32usize, 64usize);
    let mut data = Vec::new();
    for ring in 0..=rings {
        let lat = std::f32::consts::PI * ring as f32 / rings as f32;
        for segment in 0..=segments {
            let lon = std::f32::consts::TAU * segment as f32 / segments as f32;
            let n = [lat.sin() * lon.cos(), lat.sin() * lon.sin(), lat.cos()];
            for f in n.map(|c| c * radius).into_iter().chain(n).chain([2.0 * segment as f32 / segments as f32, ring as f32 / rings as f32]) { data.extend_from_slice(&f.to_le_bytes()); }
        }
    }
    let row = (segments + 1) as u32;
    let mut indices = Vec::new();
    for ring in 0..rings as u32 {
        for segment in 0..segments as u32 {
            let (a, b) = (ring * row + segment, (ring + 1) * row + segment);
            indices.extend_from_slice(&[a, b, a + 1, a + 1, b, b + 1]);
        }
    }
    let attribute = |name: &str, format: u32, offset: usize| Attribute { name: name.to_string(), index: 0, format, offset };
    Mesh {
        draws: vec![Draw { material: material.to_string(), vertex_buffer: 0, index_buffer: 0, base_vertex: 0, start_index: 0, index_count: indices.len(), tint: [1.0; 3] }],
        buffers: Buffers {
            // DXGI's R32G32B32_FLOAT (6) and R32G32_FLOAT (16)
            vertices: vec![VertexBuffer { count: (rings + 1) * (segments + 1), stride: 32, attributes: vec![attribute("POSITION", 6, 0), attribute("NORMAL", 6, 12), attribute("TEXCOORD", 16, 24)], data }],
            indices: vec![IndexBuffer { count: indices.len(), size: 4, data: indices.iter().flat_map(|i| i.to_le_bytes()).collect() }],
        },
        bones: Vec::new(),
        group_mask: 1,
    }
}

fn usage() -> ! {
    eprintln!("render <models/x.vmdl> <out.png> [yaw] [pitch] [size]\nrender map <maps/name> <out.png> x y z pitch yaw [width height]\n  (content from ENGINE2_CONTENT: folders and paks joined by ':')");
    std::process::exit(2);
}

fn fail(e: &str) -> ! {
    eprintln!("{e}");
    std::process::exit(1);
}

/// An RGBA image as a PNG.
fn png(path: &std::path::Path, width: u32, height: u32, rgba: &[u8]) -> Result<(), String> {
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut e = png::Encoder::new(std::io::BufWriter::new(file), width, height);
    e.set_color(png::ColorType::Rgba);
    e.set_depth(png::BitDepth::Eight);
    e.write_header().and_then(|mut w| w.write_image_data(rgba)).map_err(|e| e.to_string())
}
