//! engine2's renderer from the command line, from content on disk
//! (tools/package.sh; a map's `.vpk` among the places):
//!   ENGINE2_CONTENT=../content render models/sbox_props/tv/tv.vmdl tv.png [yaw] [pitch] [size]
//!   ENGINE2_CONTENT=../maps/flatgrass.vpk:../content render map maps/flatgrass out.png x y z pitch yaw [width height]
use engine2::assets::{entities, kv3::Value, resource::Resource, worldnode};
use engine2::render::{Content, Files, Instance, Renderer, View};
use glam::{Mat4, Quat, Vec3};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let content = Content::of(std::env::var("ENGINE2_CONTENT").unwrap_or_default().split(':').filter(|d| !d.is_empty()).map(Into::into));
    let num = |i: usize, d: f32| args.get(i).and_then(|a| a.parse().ok()).unwrap_or(d);
    let r = Renderer::new().unwrap_or_else(|e| fail(&e));
    let t = std::time::Instant::now();
    if args.get(1).map(String::as_str) == Some("map") {
        let (Some(folder), Some(out)) = (args.get(2), args.get(3)) else { usage() };
        let (w, h) = (num(9, 960.0) as u32, num(10, 540.0) as u32);
        let mut view = View::camera(Vec3::new(num(4, 0.0), num(5, 0.0), num(6, 64.0)), [num(7, 0.0), num(8, 0.0), 0.0], 75.0, 4.0, 40000.0, w as f32 / h as f32);
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
        let mut models = Vec::new();
        let mut notes = Vec::new();
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
        }).collect();
        let loaded = t.elapsed();
        let image = r.draw(&instances, &view, w, h);
        png(std::path::Path::new(out), w, h, &image).unwrap_or_else(|e| fail(&e));
        eprintln!("{folder} on {}: {} objects, sun {:?}, sky {}, loaded in {loaded:.2?}, drawn in {:.2?}", r.adapter, instances.len(), view.sun, view.sky.is_some(), t.elapsed() - loaded);
        notes.sort();
        notes.dedup();
        for n in notes { eprintln!("  {n}"); }
        return;
    }
    let (Some(model), Some(out)) = (args.get(1), args.get(2)) else { usage() };
    let m = r.model(&content, model).unwrap_or_else(|e| fail(&e));
    let loaded = t.elapsed();
    let size = num(5, 768.0) as u32;
    let image = r.draw(&[Instance { model: &m, transform: Default::default(), tint: [1.0; 4], bones: Vec::new() }], &View::framing(&m, num(3, 35.0), num(4, 20.0)), size, size);
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
