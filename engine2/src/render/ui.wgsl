// s&box's batched UI box (game/core/shaders/ui_cssbox_batched.shader and
// ui/text.hlsl, public), ported: each instance a quad over its box, a
// rounded rect with its background and solid borders, or a glyph covered
// analytically from its outline (Lengyel's dual ray method), its blurred
// shadow too. The tables are read as words, as HLSL's structured buffers
// pack them (four-byte alignment). Output is gamma-space, blended as a
// browser blends (UIGammaOutput).
//
// Not yet: background images and gradients, scissors, transforms' paths and
// shapes, shadows and outlines of boxes (modes 1-3), border styles.

struct Params {
    viewport: vec4<f32>,      // x, y, w, h
    layer: mat4x4<f32>,
    instance_offset: i32,
    _p0: i32, _p1: i32, _p2: i32,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> boxes: array<u32>;
@group(0) @binding(2) var<storage, read> texts: array<u32>;
@group(0) @binding(3) var<storage, read> glyph_table: array<u32>;
@group(0) @binding(4) var<storage, read> glyph_curves: array<f32>;
@group(0) @binding(5) var<storage, read> glyph_bands: array<u32>;
@group(0) @binding(6) var<storage, read> transforms: array<f32>;

const BOX: u32 = 68u;      // 272 bytes
const TEXT: u32 = 73u;     // 292 bytes
const GLYPH: u32 = 9u;     // 36 bytes

fn bf(i: u32, at: u32) -> f32 { return bitcast<f32>(boxes[i * BOX + at / 4u]); }
fn bi(i: u32, at: u32) -> i32 { return bitcast<i32>(boxes[i * BOX + at / 4u]); }
fn bv(i: u32, at: u32) -> vec4<f32> { return vec4<f32>(bf(i, at), bf(i, at + 4u), bf(i, at + 8u), bf(i, at + 12u)); }
fn tf(i: u32, at: u32) -> f32 { return bitcast<f32>(texts[i * TEXT + at / 4u]); }
fn ti(i: u32, at: u32) -> i32 { return bitcast<i32>(texts[i * TEXT + at / 4u]); }
fn tv(i: u32, at: u32) -> vec4<f32> { return vec4<f32>(tf(i, at), tf(i, at + 4u), tf(i, at + 8u), tf(i, at + 12u)); }

// A transform from TransformBuffer: HLSL reads its sixteen floats column by column
fn transform(index: i32, v: vec4<f32>) -> vec4<f32> {
    if (index < 0) { return v; }
    let b = u32(index) * 16u;
    if (b + 15u >= arrayLength(&transforms)) { return v; }
    let c0 = vec4<f32>(transforms[b], transforms[b + 1u], transforms[b + 2u], transforms[b + 3u]);
    let c1 = vec4<f32>(transforms[b + 4u], transforms[b + 5u], transforms[b + 6u], transforms[b + 7u]);
    let c2 = vec4<f32>(transforms[b + 8u], transforms[b + 9u], transforms[b + 10u], transforms[b + 11u]);
    let c3 = vec4<f32>(transforms[b + 12u], transforms[b + 13u], transforms[b + 14u], transforms[b + 15u]);
    return c0 * v.x + c1 * v.y + c2 * v.z + c3 * v.w;
}

struct VsOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) instance: u32,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, @builtin(instance_index) ii: u32) -> VsOut {
    // two triangles: corners 0 1 2, 0 2 3 of (0,0) (1,0) (1,1) (0,1)
    let corner_of = array<u32, 6>(0u, 1u, 2u, 0u, 2u, 3u);
    let c = corner_of[vi % 6u];
    let corner = vec2<f32>(select(0.0, 1.0, c == 1u || c == 2u), select(0.0, 1.0, c >= 2u));
    let i = ii + u32(params.instance_offset);
    let rect = bv(i, 0u);
    // the box grown a pixel, for the outer half of its edge's antialiasing
    let local = rect.xy - 1.0 + corner * (rect.zw + 2.0);
    let m = params.layer * transform(bi(i, 236u), vec4<f32>(local, 0.0, 1.0));
    let w = select(m.w, 1e-6, abs(m.w) < 1e-6);
    var o: VsOut;
    let ndc = 2.0 * (m.xy / w - params.viewport.xy) / params.viewport.zw - 1.0;
    o.position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    o.uv = (local - rect.xy) / max(rect.zw, vec2<f32>(0.0001));
    // gamma output: the instance's colour as given; alpha past 1 breaks blending
    let col = bv(i, 16u);
    o.color = vec4<f32>(col.rgb, clamp(col.a, 0.0, 1.0));
    o.instance = i;
    return o;
}

// ---- rounded rects (ui/rounded_rect.hlsl)

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

fn inset_box_sdf(p: vec2<f32>, size: vec2<f32>, rh: vec4<f32>, rv: vec4<f32>, inset: vec4<f32>) -> f32 {
    let inner = max(size - vec2<f32>(inset.x + inset.z, inset.y + inset.w), vec2<f32>(0.0));
    if (inner.x <= 0.0 || inner.y <= 0.0) { return 1e20; }
    let centre = vec2<f32>(inset.x - inset.z, inset.y - inset.w) * 0.5;
    let ih = rh - inset.xzxz;
    let iv = rv - inset.yyww;
    let keep = step(vec4<f32>(0.0001), ih) * step(vec4<f32>(0.0001), iv);
    return rounded_rect_sdf(p - centre, inner * 0.5, ih * keep, iv * keep);
}

// A distance's coverage of a pixel `grad` layout units across (derivatives
// are taken where WGSL allows them, at the top of fs_main)
fn sdf_coverage(d: f32, grad: f32) -> f32 {
    return clamp(0.5 - d / max(grad, 0.0001), 0.0, 1.0);
}

fn alpha_blend(top: vec4<f32>, under: vec4<f32>) -> vec4<f32> {
    let a = top.a + under.a * (1.0 - top.a);
    let rgb = select((top.rgb * top.a + under.rgb * under.a * (1.0 - top.a)) / a, vec3<f32>(0.0), a <= 0.0);
    return vec4<f32>(rgb, a);
}

// Which side's colour a point of the border takes: the side it's nearest, by each side's width
fn border_side_color(p: vec2<f32>, size: vec2<f32>, w: vec4<f32>, l: vec4<f32>, t: vec4<f32>, r: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    let dl = p.x / max(w.x, 0.0001);
    let dt = p.y / max(w.y, 0.0001);
    let dr = (size.x - p.x) / max(w.z, 0.0001);
    let db = (size.y - p.y) / max(w.w, 0.0001);
    var c = l; var d = select(1e20, dl, w.x > 0.0);
    if (w.y > 0.0 && dt < d) { c = t; d = dt; }
    if (w.z > 0.0 && dr < d) { c = r; d = dr; }
    if (w.w > 0.0 && db < d) { c = b; }
    return c;
}

// ---- glyphs (ui/text.hlsl)

struct Glyph { bounds: vec4<f32>, band_offset: i32, band_count: i32, curve_start: i32, curve_count: i32 }

fn glyph(index: i32) -> Glyph {
    let b = u32(max(index, 0)) * GLYPH;
    var g: Glyph;
    if (b + 8u >= arrayLength(&glyph_table)) { g.band_count = 0; return g; }
    g.bounds = vec4<f32>(bitcast<f32>(glyph_table[b]), bitcast<f32>(glyph_table[b + 1u]), bitcast<f32>(glyph_table[b + 2u]), bitcast<f32>(glyph_table[b + 3u]));
    g.band_offset = bitcast<i32>(glyph_table[b + 4u]);
    g.band_count = bitcast<i32>(glyph_table[b + 5u]);
    g.curve_start = bitcast<i32>(glyph_table[b + 6u]);
    g.curve_count = bitcast<i32>(glyph_table[b + 7u]);
    return g;
}

fn curve_point(i: u32) -> vec2<f32> {
    if (i * 2u + 1u >= arrayLength(&glyph_curves)) { return vec2<f32>(0.0); }
    return vec2<f32>(glyph_curves[i * 2u], glyph_curves[i * 2u + 1u]);
}

fn root_code(y1: f32, y2: f32, y3: f32) -> u32 {
    let i1 = bitcast<u32>(y1) >> 31u;
    let i2 = bitcast<u32>(y2) >> 30u;
    let i3 = bitcast<u32>(y3) >> 29u;
    var shift = (i2 & 2u) | (i1 & ~2u);
    shift = (i3 & 4u) | (shift & ~4u);
    return (0x2E74u >> shift) & 0x0101u;
}

fn gauss_erf(x_in: f32) -> f32 {
    let s = sign(x_in);
    let a = abs(x_in);
    var x = 1.0 + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x = x * x;
    return s - s / (x * x);
}

fn gauss_cdf(x: f32) -> f32 { return 0.5 + 0.5 * gauss_erf(x * 0.70710678); }

fn solve_horiz(p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>) -> vec2<f32> {
    let a = p1 - p2 * 2.0 + p3;
    let b = p1 - p2;
    let d = sqrt(max(b.y * b.y - a.y * p1.y, 0.0));
    let ra = 1.0 / a.y;
    var t1 = (b.y - d) * ra;
    var t2 = (b.y + d) * ra;
    if (abs(a.y) < 1.0 / 65536.0) { t1 = p1.y / (2.0 * b.y); t2 = t1; }
    return vec2<f32>((a.x * t1 - b.x * 2.0) * t1 + p1.x, (a.x * t2 - b.x * 2.0) * t2 + p1.x);
}

struct Ray { cov: f32, wgt: f32 }

fn ray(g: Glyph, em: vec2<f32>, ppem: f32, vertical: bool, sigma: f32) -> Ray {
    var out: Ray;
    out.cov = 0.0;
    out.wgt = 0.0;
    if (g.band_count <= 0) { return out; }
    let bmin = g.bounds.xy;
    let bsize = max(g.bounds.zw - bmin, vec2<f32>(1e-6));
    let across = select(em.y, em.x, vertical);
    let band_min = select(bmin.y, bmin.x, vertical);
    let band_size = select(bsize.y, bsize.x, vertical);
    let band = clamp(i32((across - band_min) / band_size * f32(g.band_count)), 0, g.band_count - 1);
    let header = u32(g.band_offset + select(0, g.band_count * 2, vertical) + band * 2);
    if (header + 1u >= arrayLength(&glyph_bands)) { return out; }
    let offset = glyph_bands[header] * 3u;
    let count = min(glyph_bands[header + 1u], 4096u);
    let origin = select(em, em.yx, vertical);
    let xcut = -max(0.5, sigma * 3.0) / ppem;
    for (var k = 0u; k < count; k++) {
        let c = offset + k * 3u;
        var p1 = curve_point(c);
        var p2 = curve_point(c + 1u);
        var p3 = curve_point(c + 2u);
        if (vertical) { p1 = p1.yx; p2 = p2.yx; p3 = p3.yx; }
        p1 -= origin; p2 -= origin; p3 -= origin;
        if (max(p1.x, max(p2.x, p3.x)) < xcut) { break; }
        let code = root_code(p1.y, p2.y, p3.y);
        if (code == 0u) { continue; }
        let r = solve_horiz(p1, p2, p3) * ppem;
        var st: vec2<f32>;
        if (sigma > 0.0) { st = vec2<f32>(gauss_cdf(r.x / sigma), gauss_cdf(r.y / sigma)); } else { st = clamp(r + 0.5, vec2<f32>(0.0), vec2<f32>(1.0)); }
        if ((code & 1u) != 0u) {
            out.cov += st.x;
            out.wgt = max(out.wgt, clamp(1.0 - abs(r.x) * 2.0, 0.0, 1.0));
        }
        if (code > 1u) {
            out.cov -= st.y;
            out.wgt = max(out.wgt, clamp(1.0 - abs(r.y) * 2.0, 0.0, 1.0));
        }
    }
    if (vertical) { out.cov = -out.cov; }
    return out;
}

fn font_coverage(g: Glyph, em: vec2<f32>, ppem: f32) -> f32 {
    let x = ray(g, em, ppem, false, 0.0);
    let y = ray(g, em, ppem, true, 0.0);
    let weighted = abs(x.cov * x.wgt + y.cov * y.wgt) / max(x.wgt + y.wgt, 1.0 / 65536.0);
    let fallback = min(abs(x.cov), abs(y.cov));
    return clamp(max(weighted, fallback), 0.0, 1.0);
}

fn blur_sigma(sigma: f32, ps: f32) -> f32 { return sqrt(sigma * sigma + 0.12 * ps * ps); }

fn glyph_shadow(g: Glyph, em: vec2<f32>, scale: f32, sigma: f32, ps: f32) -> f32 {
    let sx = blur_sigma(sigma, ps);
    let stp = max(ps, sigma * 0.5);
    let n = min(i32(ceil(2.5 * sx / stp)), 16);
    let y = em.y * scale;
    let base = floor(y / stp) * stp;
    let reach = g.bounds.yw * scale;
    var sum = 0.0;
    var wsum = 0.0;
    for (var k = -n; k <= n + 1; k++) {
        let row = base + f32(k) * stp;
        let dy = row - y;
        let w = exp(-(dy * dy) / (2.0 * sx * sx));
        wsum += w;
        if (row < reach.x || row > reach.y) { continue; }
        let r = ray(g, em + vec2<f32>(0.0, dy / scale), scale, false, sx);
        sum += clamp(abs(r.cov), 0.0, 1.0) * w;
    }
    return sum / max(wsum, 1e-6);
}

fn glyph_coverage(g: Glyph, em: vec2<f32>, scale: f32, sigma: f32, ps: f32, aliased: bool) -> f32 {
    if (sigma > 0.05) { return glyph_shadow(g, em, scale, sigma, ps); }
    var cov = 0.0;
    let centre = (g.bounds.xy + g.bounds.zw) * 0.5;
    let half = (g.bounds.zw - g.bounds.xy) * 0.5 + 0.51 * ps / scale;
    let out = abs(em - centre) > half;
    if (out.x || out.y) {
        cov = 0.0;
    } else if (ps > 1.05) {
        let n = min(i32(ceil(ps)), 4);
        let sub = ps / f32(n);
        for (var yy = 0; yy < n; yy++) {
            for (var xx = 0; xx < n; xx++) {
                cov += font_coverage(g, em + (vec2<f32>(f32(xx) + 0.5, f32(yy) + 0.5) * sub - ps * 0.5) / scale, scale / sub);
            }
        }
        cov /= f32(n * n);
    } else {
        cov = font_coverage(g, em, scale / ps);
    }
    if (aliased) { return select(0.0, 1.0, cov >= 0.5); }
    return cov;
}

fn text_coverage(t: u32, p: vec2<f32>, ps: f32) -> f32 {
    let mode = ti(t, 248u);
    let bg = tv(t, 160u);
    if (mode == 5) {
        // an underline and its kin: a box, here
        let x = clamp((min(bg.y, p.x + ps * 0.5) - max(bg.x, p.x - ps * 0.5)) / ps, 0.0, 1.0);
        let y = clamp((min(bg.w, p.y + ps * 0.5) - max(bg.z, p.y - ps * 0.5)) / ps, 0.0, 1.0);
        return x * y;
    }
    let size = bg.z;      // pixels per em
    let sigma = tf(t, 64u); // BorderRadiusV.x
    let aliased = (ti(t, 200u) & 1) != 0;
    return glyph_coverage(glyph(ti(t, 240u)), (p - bg.xy) / size, size, sigma, ps, aliased);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // the pixel's footprint across the box, while control flow is uniform
    let dx = dpdx(in.uv);
    let dy = dpdy(in.uv);
    let i = in.instance;
    // ENGINE2_UI_SOLID: every quad red, to see where they land
    if (params._p0 == 1) { return vec4<f32>(1.0, 0.0, 0.0, 1.0); }
    let flags = u32(bi(i, 228u));
    // a glyph: its text instance's coverage of this pixel
    if ((flags & (1u << 14u)) != 0u) {
        let t = u32(max(bi(i, 144u), 0));
        let trect = tv(t, 0u);
        let p = trect.xy + in.uv * trect.zw;
        let ps = max(0.5 * (length(dx * trect.zw) + length(dy * trect.zw)), 0.0001);
        var color = in.color;
        let cov = select(text_coverage(t, p, ps), 1.0, ti(t, 248u) == 0);
        color.a *= cov;
        return color;
    }
    // mode 0: a box. Shadows and outlines (modes 1-3) aren't drawn yet.
    let mode = flags & 3u;
    if (mode != 0u) { discard; }
    let size = bv(i, 0u).zw;
    let pos = (in.uv - 0.5) * size;
    let rh = bv(i, 32u);
    let rv = bv(i, 48u);
    let d_outer = rounded_rect_sdf(pos, size * 0.5, rh, rv);
    let grad = 0.5 * (length(dx * size) + length(dy * size));
    var col = in.color;
    let border = bv(i, 64u);
    if (border.x != 0.0 || border.y != 0.0 || border.z != 0.0 || border.w != 0.0) {
        let d_inner = inset_box_sdf(pos, size, rh, rv, border);
        var side = border_side_color(in.uv * size, size, border, bv(i, 80u), bv(i, 96u), bv(i, 112u), bv(i, 128u));
        side.a = clamp(side.a, 0.0, 1.0) * (1.0 - sdf_coverage(d_inner, grad));
        col = alpha_blend(side, col);
    }
    col.a *= sdf_coverage(d_outer, grad);
    return col;
}
