// A UI backdrop (game/core/shaders/ui_backdropfilter.shader, public),
// ported: what's been drawn behind a panel, grabbed and blurred, filtered
// as CSS's backdrop-filter says (blur, sepia, invert, contrast, hue-rotate,
// saturate, brightness), inside the panel's rounded rect or shape. And the
// grab's blurred mip chain: each level a 2x2 box on the level above, then a
// binomial over the level's own texels (as s&box's downsample_cs builds it),
// so a blur of radius r is read at LOD sqrt(r / 2).

// ---- the mip chain

struct Down {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var above: texture_2d<f32>;
@group(0) @binding(1) var linear_clamp: sampler;

@vertex
fn vs_down(@builtin(vertex_index) vi: u32) -> Down {
    // one triangle over the target
    let p = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u));
    var o: Down;
    o.position = vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
    o.uv = vec2<f32>(p.x, 1.0 - p.y);
    return o;
}

@fragment
fn fs_down(in: Down) -> @location(0) vec4<f32> {
    // a bilinear read halfway between four texels of the level above is their box;
    // nine of them, a texel of this level apart, weighted 1 2 1 each way
    let texel = 2.0 / vec2<f32>(textureDimensions(above, 0));
    var sum = vec4<f32>(0.0);
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let w = f32((2 - abs(x)) * (2 - abs(y)));
            sum += w * textureSampleLevel(above, linear_clamp, in.uv + vec2<f32>(f32(x), f32(y)) * texel, 0.0);
        }
    }
    return sum / 16.0;
}

// ---- the backdrop

struct Params {
    viewport: vec4<f32>,       // x, y, w, h of the target
    layer: mat4x4<f32>,
    transform: mat4x4<f32>,
    rect: vec4<f32>,           // the box's position and size
    radii: vec4<f32>,          // horizontal (top-left, top-right, bottom-left, bottom-right)
    radii_v: vec4<f32>,        // vertical, the same order
    look: vec4<f32>,           // brightness, contrast, saturate, sepia
    look2: vec4<f32>,          // invert, hue-rotate (degrees), blur (px), the grab's mip count
    shape: vec4<f32>,          // kind (0 the rounded rect, 1 polygon, 2 circle), points, bloat, opacity
    points: array<vec4<f32>, 4>,
    circle: vec4<f32>,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> vertices: array<vec4<f32>>;  // x y u v, then r g b a, a vertex
@group(0) @binding(2) var grab: texture_2d<f32>;
@group(0) @binding(3) var grab_sampler: sampler;

struct Out {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_backdrop(@builtin(vertex_index) vi: u32) -> Out {
    let a = vertices[vi * 2u];
    let c = vertices[vi * 2u + 1u];
    let m = params.layer * (params.transform * vec4<f32>(a.xy, 0.0, 1.0));
    let w = select(m.w, 1e-6, abs(m.w) < 1e-6);
    let ndc = 2.0 * (m.xy / w - params.viewport.xy) / params.viewport.zw - 1.0;
    var o: Out;
    o.position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    o.uv = a.zw;
    o.color = c;
    return o;
}

fn corner_distance(q: vec2<f32>, r_in: vec2<f32>) -> f32 {
    if (r_in.x == r_in.y) { return length(q) - r_in.x; }
    let r = max(r_in, vec2<f32>(0.0001));
    let k0 = length(q / r);
    let k1 = length(q / (r * r));
    return k0 * (k0 - 1.0) / k1;
}

fn rounded_rect_sdf(p: vec2<f32>, b: vec2<f32>, rh: vec4<f32>, rv: vec4<f32>) -> f32 {
    let e = abs(p) - b;
    var d = max(e.x, e.y);
    if (p.x < -b.x + rh.x && p.y < -b.y + rv.x) { let r = vec2<f32>(rh.x, rv.x); d = max(d, corner_distance(vec2<f32>(-b.x + r.x - p.x, -b.y + r.y - p.y), r)); }
    if (p.x > b.x - rh.y && p.y < -b.y + rv.y) { let r = vec2<f32>(rh.y, rv.y); d = max(d, corner_distance(vec2<f32>(p.x - b.x + r.x, -b.y + r.y - p.y), r)); }
    if (p.x < -b.x + rh.z && p.y > b.y - rv.z) { let r = vec2<f32>(rh.z, rv.z); d = max(d, corner_distance(vec2<f32>(-b.x + r.x - p.x, p.y - b.y + r.y), r)); }
    if (p.x > b.x - rh.w && p.y > b.y - rv.w) { let r = vec2<f32>(rh.w, rv.w); d = max(d, corner_distance(vec2<f32>(p.x - b.x + r.x, p.y - b.y + r.y), r)); }
    return d;
}

fn shape_point(i: i32) -> vec2<f32> {
    let v = params.points[i / 2];
    return select(v.zw, v.xy, i % 2 == 0);
}

// a polygon of up to eight points (even-odd), or a circle
fn shape_sdf(p: vec2<f32>) -> f32 {
    if (params.shape.x == 2.0) { return length(p - params.circle.xy) - params.circle.z; }
    let count = i32(params.shape.y);
    let first = shape_point(0);
    var d = dot(p - first, p - first);
    var s = 1.0;
    var j = count - 1;
    for (var i = 0; i < 8; i++) {
        if (i >= count) { break; }
        let vi = shape_point(i);
        let vj = shape_point(j);
        let e = vj - vi;
        let w = p - vi;
        let b = w - e * clamp(dot(w, e) / max(dot(e, e), 0.000001), 0.0, 1.0);
        d = min(d, dot(b, b));
        let c = vec3<bool>(p.y >= vi.y, p.y < vj.y, e.x * w.y > e.y * w.x);
        if (all(c) || all(!c)) { s = -s; }
        j = i;
    }
    return s * sqrt(d);
}

fn to_linear(c: vec3<f32>) -> vec3<f32> { return select(pow((c + 0.055) / 1.055, vec3<f32>(2.4)), c / 12.92, c <= vec3<f32>(0.04045)); }
fn to_gamma(c: vec3<f32>) -> vec3<f32> { return select(1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055, c * 12.92, c <= vec3<f32>(0.0031308)); }

fn hue_rotate(c: vec3<f32>, degrees: f32) -> vec3<f32> {
    let k = cos(radians(degrees));
    let s = sin(radians(degrees));
    return vec3<f32>(
        dot(c, vec3<f32>(0.213 + k * 0.787 - s * 0.213, 0.715 - k * 0.715 - s * 0.715, 0.072 - k * 0.072 + s * 0.928)),
        dot(c, vec3<f32>(0.213 - k * 0.213 + s * 0.143, 0.715 + k * 0.285 + s * 0.140, 0.072 - k * 0.072 - s * 0.283)),
        dot(c, vec3<f32>(0.213 - k * 0.213 - s * 0.787, 0.715 - k * 0.715 + s * 0.715, 0.072 + k * 0.928 + s * 0.072)));
}

fn saturation(c: vec3<f32>, s: f32) -> vec3<f32> {
    return vec3<f32>(
        dot(c, vec3<f32>(0.213 + 0.787 * s, 0.715 - 0.715 * s, 0.072 - 0.072 * s)),
        dot(c, vec3<f32>(0.213 - 0.213 * s, 0.715 + 0.285 * s, 0.072 - 0.072 * s)),
        dot(c, vec3<f32>(0.213 - 0.213 * s, 0.715 - 0.715 * s, 0.072 + 0.928 * s)));
}

fn sepia(c: vec3<f32>, amount: f32) -> vec3<f32> {
    let k = 1.0 - amount;
    return vec3<f32>(
        dot(c, vec3<f32>(0.393 + 0.607 * k, 0.769 - 0.769 * k, 0.189 - 0.189 * k)),
        dot(c, vec3<f32>(0.349 - 0.349 * k, 0.686 + 0.314 * k, 0.168 - 0.168 * k)),
        dot(c, vec3<f32>(0.272 - 0.272 * k, 0.534 - 0.534 * k, 0.131 + 0.869 * k)));
}

@fragment
fn fs_backdrop(in: Out) -> @location(0) vec4<f32> {
    // the panel's edge: its rounded rect (or shape) in the box's own pixels, the quad
    // grown by `bloat` all round for the outer half of the antialiasing
    let size = params.rect.zw;
    let bloat = params.shape.z;
    let box_uv = (in.uv - 0.5) * (size + bloat * 2.0) / max(size, vec2<f32>(0.0001)) + 0.5;
    var d: f32;
    if (params.shape.x != 0.0) { d = shape_sdf(box_uv * size); } else { d = rounded_rect_sdf((box_uv - 0.5) * size, size * 0.5, params.radii, params.radii_v); }
    let edge = clamp(0.5 - d / max(length(vec2<f32>(dpdx(d), dpdy(d))), 0.0001), 0.0, 1.0);

    // what's behind, as it was grabbed (gamma-space, as the UI blends), at the level the blur reaches
    let at = (in.position.xy - params.viewport.xy) / params.viewport.zw;
    let lod = clamp(sqrt(max(params.look2.z, 0.0) / 2.0), 0.0, max(params.look2.w - 1.0, 0.0));
    var c = textureSampleLevel(grab, grab_sampler, at, lod).rgb;
    c = sepia(c, params.look.w);
    c = mix(c, 1.0 - c, params.look2.x);
    c = clamp(mix(vec3<f32>(0.5), c, params.look.y), vec3<f32>(0.0), vec3<f32>(1.0));
    c = clamp(hue_rotate(c, params.look2.y), vec3<f32>(0.0), vec3<f32>(1.0));
    c = clamp(saturation(c, params.look.z), vec3<f32>(0.0), vec3<f32>(1.0));
    c *= params.look.x;
    // tinted in linear light, as s&box multiplies its decoded colour, and written back in gamma
    let tinted = to_gamma(to_linear(clamp(c, vec3<f32>(0.0), vec3<f32>(1.0))) * to_linear(in.color.rgb));
    return vec4<f32>(tinted, edge * clamp(in.color.a * params.shape.w, 0.0, 1.0));
}
