//! A model drawn into a PNG, from content on disk (tools/package.sh):
//!   ENGINE2_CONTENT=../content cargo run --release -p render -- models/sbox_props/tv/tv.vmdl tv.png [yaw] [pitch] [size]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (Some(model), Some(out)) = (args.get(1), args.get(2)) else {
        eprintln!("render <models/x.vmdl> <out.png> [yaw] [pitch] [size]   (content from ENGINE2_CONTENT, folders joined by ':')");
        std::process::exit(2);
    };
    let num = |i: usize, d: f32| args.get(i).and_then(|a| a.parse().ok()).unwrap_or(d);
    let content = render::Content(std::env::var("ENGINE2_CONTENT").unwrap_or_default().split(':').filter(|d| !d.is_empty()).map(Into::into).collect());
    let r = render::Renderer::new().unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1) });
    let t = std::time::Instant::now();
    let m = r.model(&content, model).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1) });
    let loaded = t.elapsed();
    let size = num(5, 768.0) as u32;
    let image = r.draw(&m, size, num(3, 35.0), num(4, 20.0));
    render::png(std::path::Path::new(out), size, &image).unwrap_or_else(|e| { eprintln!("{e}"); std::process::exit(1) });
    eprintln!("{model} on {}: bounds {:?}..{:?}, loaded in {loaded:.2?}, drawn in {:.2?}", r.adapter, m.min, m.max, t.elapsed() - loaded);
    for n in &m.notes { eprintln!("  {n}"); }
}
