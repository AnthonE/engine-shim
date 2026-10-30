//! A model drawn into a PNG, from content on disk (tools/package.sh):
//!   ENGINE2_CONTENT=../content cargo run --release -p render -- models/sbox_props/tv/tv.vmdl tv.png [yaw] [pitch] [size]
use engine2::render::{Content, Instance, Renderer, View};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (Some(model), Some(out)) = (args.get(1), args.get(2)) else {
        eprintln!("render <models/x.vmdl> <out.png> [yaw] [pitch] [size]   (content from ENGINE2_CONTENT, folders joined by ':')");
        std::process::exit(2);
    };
    let num = |i: usize, d: f32| args.get(i).and_then(|a| a.parse().ok()).unwrap_or(d);
    let content = Content(std::env::var("ENGINE2_CONTENT").unwrap_or_default().split(':').filter(|d| !d.is_empty()).map(Into::into).collect());
    let r = Renderer::new().unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1) });
    let t = std::time::Instant::now();
    let m = r.model(&content, model).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1) });
    let loaded = t.elapsed();
    let size = num(5, 768.0) as u32;
    let image = r.draw(&[Instance { model: &m, transform: Default::default(), tint: [1.0; 4], bones: Vec::new() }], &View::framing(&m, num(3, 35.0), num(4, 20.0)), size, size);
    png(std::path::Path::new(out), size, size, &image).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1) });
    eprintln!("{model} on {}: bounds {:?}..{:?}, loaded in {loaded:.2?}, drawn in {:.2?}", r.adapter, m.min, m.max, t.elapsed() - loaded);
    for n in &m.notes { eprintln!("  {n}"); }
}

/// An RGBA image as a PNG.
fn png(path: &std::path::Path, width: u32, height: u32, rgba: &[u8]) -> Result<(), String> {
    let file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    let mut e = png::Encoder::new(std::io::BufWriter::new(file), width, height);
    e.set_color(png::ColorType::Rgba);
    e.set_depth(png::BitDepth::Eight);
    e.write_header().and_then(|mut w| w.write_image_data(rgba)).map_err(|e| e.to_string())
}
