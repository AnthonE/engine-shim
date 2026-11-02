// s&box's batched UI box (game/core/shaders/ui_cssbox_batched.shader and its
// includes under ui/, public), ported: each instance a quad over its box.
//
// - Boxes (mode 0): a rounded rect, its background colour, image or
//   gradient (sized, placed, repeated and turned as background-size,
//   -position, -repeat and -angle say), cut back to its padding, content or
//   text box (background-clip), then its border: solid, styled (dotted,
//   dashed, double, groove, ridge, inset, outset) or a nine-slice image.
// - Shadows (modes 1, 2): a rounded rect blurred by a gaussian, outside the
//   box or inside it (Evan Wallace's analytic shadow, per-corner radii).
// - Outlines (mode 3): the band past the box.
// - Glyphs (flag 14): covered analytically from their outlines (Lengyel's
//   dual ray method), blurred for text-shadow; a gradient's colour where
//   the text has one.
// - Clip stacks: each instance's own chain of up to four rounded rects a
//   link (overflow: hidden, border-radius clipping), through their
//   transforms, antialiased; and outset shadows' inverse one.
//
// The tables are read as words, as HLSL's structured buffers pack them
// (four-byte alignment). Output is gamma-space, blended as a browser blends
// (UIGammaOutput): colours stay as authored. Bindless textures are a few
// bindings a draw (ui.rs splits a draw where its images outgrow them), found
// by their bindless index; samplers' address modes are applied here.
//
// Not yet: border-shape's shapes and paths (ShapeIndex), drawn as the
// rounded rect; world panels; HDR edge softening (the target is 8-bit).

struct Params {
    viewport: vec4<f32>,      // x, y, w, h
    layer: mat4x4<f32>,
    instance_offset: i32,
    // 1: every quad solid red (ENGINE2_UI_SOLID); 2: premultiplied (D_BLENDMODE 3); 4: panel opacity (D_PANEL_OPACITY)
    flags: u32,
    opacity: f32,
    _p0: u32,
    scissor_at: u32, scissor_count: u32, gradient_at: u32, gradient_count: u32,
    sampler_at: u32, sampler_count: u32, _p1: u32, _p2: u32,
    // the textures bound to this draw: bindless index, width, height
    slots: array<vec4<i32>, 8>,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> boxes: array<u32>;
@group(0) @binding(2) var<storage, read> texts: array<u32>;
@group(0) @binding(3) var<storage, read> glyph_table: array<u32>;
@group(0) @binding(4) var<storage, read> glyph_curves: array<f32>;
@group(0) @binding(5) var<storage, read> glyph_bands: array<u32>;
@group(0) @binding(6) var<storage, read> transforms: array<f32>;
// ScissorBuffer, GradientBuffer and the samplers, one after another (WebGPU allows eight storage buffers a stage)
@group(0) @binding(7) var<storage, read> extra: array<u32>;
@group(0) @binding(8) var tex0: texture_2d<f32>;
@group(0) @binding(9) var tex1: texture_2d<f32>;
@group(0) @binding(10) var tex2: texture_2d<f32>;
@group(0) @binding(11) var tex3: texture_2d<f32>;
@group(0) @binding(12) var tex4: texture_2d<f32>;
@group(0) @binding(13) var tex5: texture_2d<f32>;
@group(0) @binding(14) var tex6: texture_2d<f32>;
@group(0) @binding(15) var tex7: texture_2d<f32>;
@group(0) @binding(16) var linear_clamp: sampler;

const BOX: u32 = 68u;      // 272 bytes
const TEXT: u32 = 73u;     // 292 bytes
const GLYPH: u32 = 9u;     // 36 bytes
const SCISSOR: u32 = 116u; // 464 bytes: count, invert, next, a pad, four clips of 112
const GRADIENT: u32 = 50u; // 200 bytes

const SOLID: u32 = 1u;
const PREMULTIPLIED: u32 = 2u;
const PANEL_OPACITY: u32 = 4u;

fn bf(i: u32, at: u32) -> f32 { return bitcast<f32>(boxes[i * BOX + at / 4u]); }
fn bi(i: u32, at: u32) -> i32 { return bitcast<i32>(boxes[i * BOX + at / 4u]); }
fn bv(i: u32, at: u32) -> vec4<f32> { return vec4<f32>(bf(i, at), bf(i, at + 4u), bf(i, at + 8u), bf(i, at + 12u)); }
fn tf(i: u32, at: u32) -> f32 { return bitcast<f32>(texts[i * TEXT + at / 4u]); }
fn ti(i: u32, at: u32) -> i32 { return bitcast<i32>(texts[i * TEXT + at / 4u]); }
fn tv(i: u32, at: u32) -> vec4<f32> { return vec4<f32>(tf(i, at), tf(i, at + 4u), tf(i, at + 8u), tf(i, at + 12u)); }
fn ef(at: u32) -> f32 { if (at >= arrayLength(&extra)) { return 0.0; } return bitcast<f32>(extra[at]); }
fn ei(at: u32) -> i32 { if (at >= arrayLength(&extra)) { return 0; } return bitcast<i32>(extra[at]); }
fn ev(at: u32) -> vec4<f32> { return vec4<f32>(ef(at), ef(at + 1u), ef(at + 2u), ef(at + 3u)); }
// sixteen floats as HLSL reads a float4x4 from a structured buffer: column by column
fn em(at: u32) -> mat4x4<f32> { return mat4x4<f32>(ev(at), ev(at + 4u), ev(at + 8u), ev(at + 12u)); }

// A transform from TransformBuffer
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
    // where it is in panel space (the instance's transform applied): what clips are measured in
    @location(3) panel: vec4<f32>,
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
    let panel = transform(bi(i, 236u), vec4<f32>(local, 0.0, 1.0));
    let m = params.layer * panel;
    let w = select(m.w, 1e-6, abs(m.w) < 1e-6);
    var o: VsOut;
    let ndc = 2.0 * (m.xy / w - params.viewport.xy) / params.viewport.zw - 1.0;
    o.position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    o.uv = (local - rect.xy) / max(rect.zw, vec2<f32>(0.0001));
    // gamma output: the instance's colour as given; alpha past 1 breaks blending
    let col = bv(i, 16u);
    var alpha = col.a;
    if ((params.flags & PANEL_OPACITY) != 0u) { alpha *= params.opacity; }
    o.color = vec4<f32>(col.rgb, clamp(alpha, 0.0, 1.0));
    o.instance = i;
    o.panel = panel;
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

// A distance's coverage of a pixel `grad` layout units across. HLSL takes the
// distance's own screen derivative; WGSL only allows derivatives where control
// flow is uniform, so the pixel's footprint is taken at the top of fs_main
// and a distance in layout units is assumed to change one a unit.
fn sdf_coverage(d: f32, grad: f32) -> f32 {
    return clamp(0.5 - d / max(grad, 0.0001), 0.0, 1.0);
}

fn alpha_blend(top: vec4<f32>, under: vec4<f32>) -> vec4<f32> {
    let a = top.a + (1.0 - top.a) * under.a;
    return vec4<f32>((top.a * top.rgb + (1.0 - top.a) * under.a * under.rgb) / max(a, 0.0001), a);
}

fn gauss_erf(x_in: f32) -> f32 {
    let s = sign(x_in);
    let a = abs(x_in);
    var x = 1.0 + (0.278393 + (0.230389 + 0.078108 * (a * a)) * a) * a;
    x = x * x;
    return s - s / (x * x);
}

fn gaussian(x: f32, sigma: f32) -> f32 { return exp(-(x * x) / (2.0 * sigma * sigma)) / (2.50662827463 * sigma); }

// How far a corner pulls its side in at a row, dy being the row's distance from that corner's end of the box
fn corner_inset(dy: f32, r: vec2<f32>) -> f32 {
    if (dy >= r.y) { return 0.0; }
    let t = (r.y - dy) / max(r.y, 0.0001);
    return r.x - r.x * sqrt(clamp(1.0 - t * t, 0.0, 1.0));
}

fn row_extent(y: f32, b: vec2<f32>, rh: vec4<f32>, rv: vec4<f32>) -> vec2<f32> {
    let d_top = y + b.y;
    let d_bottom = b.y - y;
    let l = max(corner_inset(d_top, vec2<f32>(rh.x, rv.x)), corner_inset(d_bottom, vec2<f32>(rh.z, rv.z)));
    let r = max(corner_inset(d_top, vec2<f32>(rh.y, rv.y)), corner_inset(d_bottom, vec2<f32>(rh.w, rv.w)));
    return vec2<f32>(-b.x + l, b.x - r);
}

// The box-shadow shape: each row a segment blurred along x (a difference of
// error functions), the rows integrated along y with eight gaussian-weighted samples
fn rounded_rect_shadow(p: vec2<f32>, b: vec2<f32>, rh: vec4<f32>, rv: vec4<f32>, sigma: f32) -> f32 {
    let low = p.y - b.y;
    let high = p.y + b.y;
    let start = clamp(-3.0 * sigma, low, high);
    let end = clamp(3.0 * sigma, low, high);
    let stp = (end - start) / 8.0;
    var y = start + stp * 0.5;
    var value = 0.0;
    for (var k = 0; k < 8; k++) {
        let e = row_extent(p.y - y, b, rh, rv);
        let row = 0.5 * (gauss_erf((p.x - e.x) * (0.70710678 / sigma)) - gauss_erf((p.x - e.y) * (0.70710678 / sigma)));
        value += row * gaussian(y, sigma) * stp;
        y += stp;
    }
    return value;
}

// ---- clip stacks (ui/batched_scissor.hlsl): each link up to four rounded
// rects in their own layout space, the pixel's footprint carried through each
// inverse homography

fn scissor_coverage(first: i32, pos: vec2<f32>, px: vec2<f32>, py: vec2<f32>) -> f32 {
    var coverage = 1.0;
    var index = first;
    for (var guard = 0; guard < 16; guard++) {
        if (index < 0 || u32(index) >= params.scissor_count) { break; }
        let b = params.scissor_at + u32(index) * SCISSOR;
        let count = min(ei(b), 4);
        var local = 1.0;
        for (var k = 0; k < count; k++) {
            let c = b + 4u + u32(k) * 28u;
            let rect = ev(c);
            if (rect.z <= rect.x || rect.w <= rect.y) { local = 0.0; break; }
            let m = em(c + 12u);
            let q = m * vec4<f32>(pos, 0.0, 1.0);
            if (q.w <= 0.0) { local = 0.0; break; }
            let p = q.xy / q.w;
            let d = rounded_rect_sdf(p - (rect.xy + rect.zw) * 0.5, (rect.zw - rect.xy) * 0.5, ev(c + 4u), ev(c + 8u));
            let qdx = m * vec4<f32>(px, 0.0, 0.0);
            let qdy = m * vec4<f32>(py, 0.0, 0.0);
            // outside the projected bounds before the footprint can blow up near the horizon
            let edges = vec4<f32>(q.xy - rect.xy * q.w, rect.zw * q.w - q.xy);
            let edx = vec4<f32>(qdx.xy - rect.xy * qdx.w, rect.zw * qdx.w - qdx.xy);
            let edy = vec4<f32>(qdy.xy - rect.xy * qdy.w, rect.zw * qdy.w - qdy.xy);
            if (any(edges < -0.5 * (abs(edx) + abs(edy)))) { local = 0.0; break; }
            let dpx = (qdx.xy - p * qdx.w) / q.w;
            let dpy = (qdy.xy - p * qdy.w) / q.w;
            let pixel = 0.5 * (length(dpx) + length(dpy));
            local *= clamp(0.5 - d / max(pixel, 0.0001), 0.0, 1.0);
        }
        coverage *= select(local, 1.0 - local, ei(b + 1u) != 0);
        index = ei(b + 2u);
    }
    return coverage;
}

// ---- textures: bound to this draw by bindless index

fn slot_of(index: i32) -> i32 {
    for (var k = 0; k < 8; k++) { if (params.slots[k].x == index) { return k; } }
    return -1;
}

fn slot_size(slot: i32) -> vec2<f32> { return vec2<f32>(params.slots[slot].yz); }

fn sample_grad(slot: i32, uv: vec2<f32>, gx: vec2<f32>, gy: vec2<f32>) -> vec4<f32> {
    switch slot {
        case 0: { return textureSampleGrad(tex0, linear_clamp, uv, gx, gy); }
        case 1: { return textureSampleGrad(tex1, linear_clamp, uv, gx, gy); }
        case 2: { return textureSampleGrad(tex2, linear_clamp, uv, gx, gy); }
        case 3: { return textureSampleGrad(tex3, linear_clamp, uv, gx, gy); }
        case 4: { return textureSampleGrad(tex4, linear_clamp, uv, gx, gy); }
        case 5: { return textureSampleGrad(tex5, linear_clamp, uv, gx, gy); }
        case 6: { return textureSampleGrad(tex6, linear_clamp, uv, gx, gy); }
        case 7: { return textureSampleGrad(tex7, linear_clamp, uv, gx, gy); }
        default: { return vec4<f32>(0.0); }
    }
}

fn load_texel(slot: i32, at: vec2<i32>) -> vec4<f32> {
    switch slot {
        case 0: { return textureLoad(tex0, at, 0); }
        case 1: { return textureLoad(tex1, at, 0); }
        case 2: { return textureLoad(tex2, at, 0); }
        case 3: { return textureLoad(tex3, at, 0); }
        case 4: { return textureLoad(tex4, at, 0); }
        case 5: { return textureLoad(tex5, at, 0); }
        case 6: { return textureLoad(tex6, at, 0); }
        case 7: { return textureLoad(tex7, at, 0); }
        default: { return vec4<f32>(0.0); }
    }
}

// A coordinate past the edge, as an address mode puts it back: 0 wrap, 1
// mirror, 2 clamp, 3 border (left out: the caller gives the border colour), 4 mirror once
fn address(u: f32, mode: u32) -> f32 {
    switch mode {
        case 1u: { let t = fract(u * 0.5) * 2.0; return select(t, 2.0 - t, t > 1.0); }
        case 2u, 3u: { return clamp(u, 0.0, 1.0); }
        case 4u: { return clamp(abs(u), 0.0, 1.0); }
        default: { return fract(u); }
    }
}

// A texture at uv as the bindless sampler `sampler_index` reads it, its
// gradients those of uv before it's wrapped
fn sample_texture(index: i32, sampler_index: i32, uv: vec2<f32>, gx: vec2<f32>, gy: vec2<f32>) -> vec4<f32> {
    let slot = slot_of(index);
    if (slot < 0) { return vec4<f32>(0.0); }
    var filtering = 0x14u;
    var au = 0u;
    var av = 0u;
    var border = 0u;
    if (sampler_index >= 0 && u32(sampler_index) < params.sampler_count) {
        let s = params.sampler_at + u32(sampler_index) * 2u;
        let w = u32(ei(s));
        filtering = w & 0xffu;
        au = (w >> 8u) & 0xffu;
        av = (w >> 16u) & 0xffu;
        border = u32(ei(s + 1u));
    }
    if ((au == 3u && (uv.x < 0.0 || uv.x > 1.0)) || (av == 3u && (uv.y < 0.0 || uv.y > 1.0))) {
        return unpack4x8unorm(border);
    }
    let at = vec2<f32>(address(uv.x, au), address(uv.y, av));
    if (filtering == 0u) {
        let size = slot_size(slot);
        return load_texel(slot, clamp(vec2<i32>(floor(at * size)), vec2<i32>(0), vec2<i32>(size) - 1));
    }
    return sample_grad(slot, at, gx, gy);
}

// ---- gradients (GradientBuffer)

fn g_color(g: u32, k: i32) -> vec4<f32> { return ev(g + u32(clamp(k, 0, 7)) * 4u); }
fn g_offset(g: u32, k: i32) -> f32 { return ef(g + 32u + u32(clamp(k, 0, 7))); }

// Position along a background gradient, 0 at its start and 1 at its end, as the web measures it;
// and how many pixels that spans, for stops written in pixels
struct GradPos { t: f32, len: f32 }

fn gradient_position(g: u32, uv: vec2<f32>, size: vec2<f32>) -> GradPos {
    var out: GradPos;
    out.len = 1.0;
    let kind = ei(g + 42u);
    let angle = ef(g + 41u);
    if (kind == 0) {
        var dir = vec2<f32>(sin(angle), cos(angle)); // 0 down the panel, 90deg right
        let corner = ei(g + 49u);
        if (corner != 0) {
            // a corner keyword leans with the box: perpendicular to the diagonal joining the other two corners
            let towards = vec2<f32>(select(-1.0, 1.0, corner == 2 || corner == 4), select(-1.0, 1.0, corner == 3 || corner == 4));
            dir = normalize(vec2<f32>(towards.x * size.y, towards.y * size.x));
        }
        let rel = (uv - 0.5) * size;
        out.len = abs(size.x * dir.x) + abs(size.y * dir.y);
        out.t = dot(rel, dir) / max(out.len, 0.0001) + 0.5;
        return out;
    }
    let units = ei(g + 46u);
    let centre = vec2<f32>(ef(g + 44u), ef(g + 45u));
    let c = vec2<f32>(select(centre.x, centre.x * size.x, (units & 1) != 0), select(centre.y, centre.y * size.y, (units & 2) != 0));
    let d = uv * size - c;
    if (kind == 2) {
        // conic: zero straight up, clockwise, turned by its angle
        out.t = fract((atan2(d.x, -d.y) - angle) / 6.28318530718);
        return out;
    }
    let near = vec2<f32>(min(c.x, size.x - c.x), min(c.y, size.y - c.y));
    let far = vec2<f32>(max(c.x, size.x - c.x), max(c.y, size.y - c.y));
    let mode = ei(g + 43u);
    var radius: vec2<f32>;
    if (ei(g + 47u) != 0) {
        var r = length(near);
        if (mode == 0) { r = max(far.x, far.y); } else if (mode == 1) { r = length(far); } else if (mode == 2) { r = min(near.x, near.y); }
        radius = vec2<f32>(r);
    } else {
        radius = near * 1.41421356;
        if (mode == 0) { radius = far; } else if (mode == 1) { radius = far * 1.41421356; } else if (mode == 2) { radius = near; }
    }
    out.len = radius.x;
    out.t = length(d / max(radius, vec2<f32>(0.0001)));
    return out;
}

fn gradient_stop(g: u32, k: i32, len: f32) -> f32 {
    if ((ei(g + 48u) & (1 << u32(k))) != 0) { return g_offset(g, k) / max(len, 0.0001); }
    return g_offset(g, k);
}

// Stops interpolate premultiplied with linear alpha, in sRGB space, like the web
fn gradient(index: i32, uv: vec2<f32>, size: vec2<f32>) -> vec4<f32> {
    if (index < 0 || u32(index) >= params.gradient_count) { return vec4<f32>(0.0); }
    let g = params.gradient_at + u32(index) * GRADIENT;
    let at = gradient_position(g, uv, size);
    let last = clamp(ei(g + 40u), 1, 8) - 1;
    if (at.t <= gradient_stop(g, 0, at.len)) { return g_color(g, 0); }
    if (at.t >= gradient_stop(g, last, at.len)) { return g_color(g, last); }
    var col = g_color(g, last);
    for (var s = 0; s < last; s++) {
        let o1 = gradient_stop(g, s + 1, at.len);
        if (at.t <= o1) {
            let o0 = gradient_stop(g, s, at.len);
            let f = clamp((at.t - o0) / max(o1 - o0, 0.0001), 0.0, 1.0);
            let a = g_color(g, s);
            let b = g_color(g, s + 1);
            let alpha = mix(a.a, b.a, f);
            var rgb = mix(a.rgb * a.a, b.rgb * b.a, f);
            if (alpha > 0.0001) { rgb /= alpha; }
            col = vec4<f32>(rgb, alpha);
            break;
        }
    }
    return col;
}

// A text gradient, as RichTextKit built its Skia shader (ui/text.hlsl's
// EvaluateTextGradient): rect is the text's x, y, w, h, p a layout position
fn text_gradient(index: i32, rect: vec4<f32>, p_in: vec2<f32>) -> vec4<f32> {
    if (index < 0 || u32(index) >= params.gradient_count) { return vec4<f32>(1.0); }
    let g = params.gradient_at + u32(index) * GRADIENT;
    let wh = max(rect.zw, vec2<f32>(0.0001));
    let p = p_in - rect.xy;
    var t: f32;
    if (ei(g + 42u) == 0) {
        let c = wh * 0.5;
        let a = radians(180.0 + ef(g + 41u));
        let rot = mat2x2<f32>(cos(a), sin(a), -sin(a), cos(a));
        let s = c + rot * (vec2<f32>(wh.x * 0.5, 0.0) - c);
        let e = c + rot * (vec2<f32>(wh.x * 0.5, wh.y) - c);
        var span = abs(e - s);
        span = vec2<f32>(select(span.x, 1.0, span.x == 0.0), select(span.y, 1.0, span.y == 0.0));
        let q = c + (p - c) / (wh / span);
        t = dot(q - s, e - s) / max(dot(e - s, e - s), 1e-6);
    } else {
        let units = ei(g + 46u);
        let centre = vec2<f32>(ef(g + 44u), ef(g + 45u));
        let c = vec2<f32>(select(centre.x, centre.x * wh.x, (units & 1) != 0), select(centre.y, centre.y * wh.y, (units & 2) != 0));
        let radius = max(wh.x, wh.y);
        let near = vec2<f32>(min(c.x, wh.x - c.x), min(c.y, wh.y - c.y));
        let far = vec2<f32>(max(c.x, wh.x - c.x), max(c.y, wh.y - c.y));
        let mode = ei(g + 43u);
        var sc: vec2<f32>;
        if (mode == 1 || mode == 3) {
            var pick = c;
            let others = array<vec2<f32>, 3>(vec2<f32>(wh.x, 0.0) - c, wh - c, vec2<f32>(0.0, wh.y) - c);
            for (var k = 0; k < 3; k++) {
                let better = select(length(others[k]) < length(pick), length(others[k]) > length(pick), mode == 1);
                if (better) { pick = others[k]; }
            }
            sc = pick / (wh * 0.71);
        } else if (mode == 2) {
            sc = near / wh;
        } else {
            sc = far / wh;
        }
        if (mode != 4) { sc *= wh / radius; }
        let q = c + (p - c) / sc;
        t = length(q - c) / radius;
    }
    t = clamp(t, 0.0, 1.0);
    let last = clamp(ei(g + 40u), 1, 8) - 1;
    if (t <= g_offset(g, 0)) { return g_color(g, 0); }
    if (t >= g_offset(g, last)) { return g_color(g, last); }
    var col = g_color(g, last);
    for (var s = 0; s < last; s++) {
        let o1 = g_offset(g, s + 1);
        if (t <= o1) {
            let o0 = g_offset(g, s);
            col = mix(g_color(g, s), g_color(g, s + 1), clamp((t - o0) / max(o1 - o0, 1e-6), 0.0, 1.0));
            break;
        }
    }
    return col;
}

// ---- borders

// Which side's colour a border pixel takes: the side it's proportionally
// least deep into (CSS splits each corner along the line from the box's
// corner to the padding box's), the join antialiased over a pixel. Sides with
// no border never win. `grad` is the pixel's footprint in layout units.
fn border_side_color(p: vec2<f32>, size: vec2<f32>, w: vec4<f32>, cl: vec4<f32>, ct: vec4<f32>, cr: vec4<f32>, cb: vec4<f32>, grad: f32) -> vec4<f32> {
    let has = step(vec4<f32>(0.0001), w);
    let ww = max(w, vec4<f32>(0.0001));
    var depth = vec4<f32>(p.x, p.y, size.x - p.x, size.y - p.y) / ww;
    depth = depth * has + (1.0 - has) * 1e9;
    var c1 = cl; var c2 = ct;
    var d1 = depth.x; var d2 = depth.y;
    var w1 = ww.x; var w2 = ww.y;
    if (d2 < d1) { c1 = ct; c2 = cl; d1 = depth.y; d2 = depth.x; w1 = ww.y; w2 = ww.x; }
    if (depth.z < d1) { c2 = c1; d2 = d1; w2 = w1; c1 = cr; d1 = depth.z; w1 = ww.z; } else if (depth.z < d2) { c2 = cr; d2 = depth.z; w2 = ww.z; }
    if (depth.w < d1) { c2 = c1; d2 = d1; w2 = w1; c1 = cb; d1 = depth.w; w1 = ww.w; } else if (depth.w < d2) { c2 = cb; d2 = depth.w; w2 = ww.w; }
    // how far this pixel is from the join line, in pixels: fwidth(d2 - d1), as each depth runs 1/width a unit
    let diff = d2 - d1;
    let t = clamp(0.5 - diff / max(grad * (1.0 / w1 + 1.0 / w2), 0.0001), 0.0, 1.0);
    return mix(c1, c2, t);
}

// ui/disc_coverage.hlsl
fn ui_normal(v: vec2<f32>) -> vec2<f32> {
    let s = length(v);
    return select(vec2<f32>(1.0, 0.0), v / s, s > 0.000001);
}

fn pixel_width(n: vec2<f32>, px: vec2<f32>, py: vec2<f32>) -> f32 {
    return max(length(vec2<f32>(dot(n, px), dot(n, py))), 0.000001);
}

fn disc_coverage(p: vec2<f32>, width: f32, px: vec2<f32>, py: vec2<f32>) -> f32 {
    let scale = max(max(max(abs(px.x), abs(px.y)), max(abs(py.x), abs(py.y))), 0.000001);
    let x = px / scale;
    let y = py / scale;
    let xx = x.x * x.x + y.x * y.x;
    let xy = x.x * x.y + y.x * y.y;
    let yy = x.y * x.y + y.y * y.y;
    let largest = 0.5 * (xx + yy + sqrt((xx - yy) * (xx - yy) + 4.0 * xy * xy));
    let major = select(vec2<f32>(0.0, 1.0), ui_normal(vec2<f32>(largest - yy, xy)), abs(xy) > 0.000001 || xx > yy);
    let minor = vec2<f32>(-major.y, major.x);
    let footprint = vec2<f32>(sqrt(largest), abs(x.x * y.y - x.y * y.x) / max(sqrt(largest), 0.000001)) * scale;
    let diameter = max(vec2<f32>(width), max(footprint, vec2<f32>(0.000001)));
    let radii = diameter * 0.5;
    let q = vec2<f32>(dot(p, major), dot(p, minor));
    let k0 = length(q / radii);
    let smallest = min(radii.x, radii.y);
    let gradient = (q / radii) * (smallest / radii);
    let k1 = length(gradient);
    var distance = -smallest;
    var normal = select(minor, major, radii.x < radii.y);
    if (k0 > 0.000001 && k1 > 0.0) {
        distance = k0 * (k0 - 1.0) * smallest / k1;
        normal = (gradient.x * major + gradient.y * minor) / k1;
    }
    let thinness = (width / diameter.x) * (width / diameter.y);
    return clamp(0.5 - distance / pixel_width(normal, px, py), 0.0, 1.0) * thinness;
}

// ui/border_style.hlsl: where a pixel is along the border's midline, its
// four sides and elliptical corners walked clockwise from the top edge
struct Contour { along: f32, len: f32, across: f32, normal: vec2<f32> }

fn ellipse_length(r: vec2<f32>, angle: f32) -> f32 {
    if (r.x == r.y) { return r.x * angle; }
    let nodes = vec4<f32>(0.1834346425, 0.5255324099, 0.7966664774, 0.9602898565);
    let weights = vec4<f32>(0.3626837834, 0.3137066459, 0.2223810345, 0.1012285363);
    var result = 0.0;
    for (var k = 0; k < 4; k++) {
        let t = angle * 0.5 * vec2<f32>(1.0 - nodes[k], 1.0 + nodes[k]);
        result += weights[k] * (length(r * vec2<f32>(sin(t.x), cos(t.x))) + length(r * vec2<f32>(sin(t.y), cos(t.y))));
    }
    return result * angle * 0.5;
}

fn border_contour(p_in: vec2<f32>, size_in: vec2<f32>, border: vec4<f32>, rh: vec4<f32>, rv: vec4<f32>) -> Contour {
    let inset = border * 0.5;
    let size = max(size_in - inset.xy - inset.zw, vec2<f32>(0.0));
    let p = p_in - inset.xy;
    var h = max(rh - inset.xzxz, vec4<f32>(0.0));
    var v = max(rv - inset.yyww, vec4<f32>(0.0));
    let keep = step(vec4<f32>(0.0001), h) * step(vec4<f32>(0.0001), v);
    h *= keep; v *= keep;
    let starts = array<vec2<f32>, 4>(vec2<f32>(h.x, 0.0), vec2<f32>(size.x, v.y), vec2<f32>(size.x - h.w, size.y), vec2<f32>(0.0, size.y - v.z));
    let ends = array<vec2<f32>, 4>(vec2<f32>(size.x - h.y, 0.0), vec2<f32>(size.x, size.y - v.w), vec2<f32>(h.z, size.y), vec2<f32>(0.0, v.x));
    let centres = array<vec2<f32>, 4>(vec2<f32>(size.x - h.y, v.y), vec2<f32>(size.x - h.w, size.y - v.w), vec2<f32>(h.z, size.y - v.z), vec2<f32>(h.x, v.x));
    let axes = array<vec2<f32>, 4>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), vec2<f32>(-1.0, 0.0));
    let radii = array<vec2<f32>, 4>(vec2<f32>(v.y, h.y), vec2<f32>(h.w, v.w), vec2<f32>(v.z, h.z), vec2<f32>(h.x, v.x));
    var out: Contour;
    var nearest = 1e30;
    for (var k = 0; k < 4; k++) {
        let delta = ends[k] - starts[k];
        let extent = length(delta);
        let t = clamp(dot(p - starts[k], delta) / max(dot(delta, delta), 0.000001), 0.0, 1.0);
        var rel = p - mix(starts[k], ends[k], t);
        var dist = dot(rel, rel);
        if (dist < nearest) {
            nearest = dist;
            out.along = out.len + t * extent;
            out.normal = axes[k];
            out.across = dot(rel, out.normal);
        }
        out.len += extent;
        let r = radii[k];
        if (r.x > 0.0 && r.y > 0.0) {
            let next = axes[(k + 1) % 4];
            let q = p - centres[k];
            let local = vec2<f32>(dot(q, axes[k]), dot(q, next));
            var angle = clamp(atan2(local.y / r.y, local.x / r.x), 0.0, 1.57079632679);
            // radial projection isn't the nearest point on an ellipse: refine along its normal
            if (r.x != r.y) {
                for (var it = 0; it < 5; it++) {
                    let sn = sin(angle);
                    let cs = cos(angle);
                    let derivative = (r.y * r.y - r.x * r.x) * sn * cs + r.x * local.x * sn - r.y * local.y * cs;
                    let curvature = (r.y * r.y - r.x * r.x) * (cs * cs - sn * sn) + r.x * local.x * cs + r.y * local.y * sn;
                    if (curvature > 0.000001) { angle = clamp(angle - clamp(derivative / curvature, -0.25, 0.25), 0.0, 1.57079632679); }
                }
            }
            let radial = vec2<f32>(cos(angle), sin(angle));
            let point = centres[k] + axes[k] * radial.x * r.x + next * radial.y * r.y;
            rel = p - point;
            dist = dot(rel, rel);
            if (dist < nearest) {
                nearest = dist;
                out.along = out.len + ellipse_length(r, angle);
                out.normal = ui_normal(axes[k] * radial.x / r.x + next * radial.y / r.y);
                out.across = dot(rel, out.normal);
            }
            out.len += ellipse_length(r, 1.57079632679);
        }
    }
    return out;
}

// the integral of a periodic box, so dash ends antialias even when the period is under a pixel
fn pulse_integral(x: f32, duty: f32) -> f32 { return floor(x) * duty + min(fract(x), duty); }

const DOTTED: i32 = 3;
const DASHED: i32 = 4;
const DOUBLE: i32 = 5;
const GROOVE: i32 = 6;
const RIDGE: i32 = 7;
const OUTSET: i32 = 9;

fn border_pattern(style: i32, width: f32, c: Contour, px: vec2<f32>, py: vec2<f32>) -> f32 {
    let target_len = width * select(6.0, 2.0, style == DOTTED);
    let count = max(1.0, round(c.len / max(target_len, 0.0001)));
    let period = max(c.len / count, 0.0001);
    let tangent = vec2<f32>(-c.normal.y, c.normal.x);
    let phase = fract(c.along / period);
    if (style == DASHED) {
        let footprint = max(pixel_width(tangent, px, py) / period, 0.000001);
        let centre = phase + 0.25;
        return clamp((pulse_integral(centre + footprint * 0.5, 0.5) - pulse_integral(centre - footprint * 0.5, 0.5)) / footprint, 0.0, 1.0);
    }
    let along = (fract(phase + 0.5) - 0.5) * period;
    return disc_coverage(tangent * along + c.normal * c.across, width, px, py);
}

fn styled_border(i: u32, p: vec2<f32>, size: vec2<f32>, border: vec4<f32>, rh: vec4<f32>, rv: vec4<f32>, px: vec2<f32>, py: vec2<f32>, grad: f32) -> vec4<f32> {
    let style = (bi(i, 228u) >> 4u) & 0xf;
    let centred = p - size * 0.5;
    var color = border_side_color(p, size, border, bv(i, 80u), bv(i, 96u), bv(i, 112u), bv(i, 128u), grad);
    if (style == 1 || style == 2) { color.a = 0.0; return color; } // none, hidden
    let width = max(max(border.x, border.y), max(border.z, border.w));
    if (style == DOTTED || style == DASHED) {
        color.a *= border_pattern(style, width, border_contour(p, size, border, rh, rv), px, py);
    }
    if (style == DOUBLE) {
        let first = sdf_coverage(inset_box_sdf(centred, size, rh, rv, border / 3.0), grad);
        let second = sdf_coverage(inset_box_sdf(centred, size, rh, rv, border * (2.0 / 3.0)), grad);
        color.a *= clamp(1.0 - first + second, 0.0, 1.0);
    }
    if (style >= GROOVE) {
        var light = border_side_color(p, size, border, vec4<f32>(0.0), vec4<f32>(0.0), vec4<f32>(1.0), vec4<f32>(1.0), grad).r;
        var middle = 0.0;
        if (style == GROOVE || style == RIDGE) { middle = sdf_coverage(inset_box_sdf(centred, size, rh, rv, border * 0.5), grad); }
        if (style == RIDGE || style == OUTSET) { light = 1.0 - light; }
        if (style == GROOVE || style == RIDGE) { light = mix(light, 1.0 - light, middle); }
        color = vec4<f32>(mix(color.rgb * 0.55, mix(color.rgb, max(color.rgb, vec3<f32>(1.0)), 0.3), light), color.a);
    }
    return color;
}

// border-image: nine slices of a texture, the middle stretched or repeated
fn image_border(i: u32, uv_in: vec2<f32>, size: vec2<f32>, w: vec4<f32>) -> vec4<f32> {
    let index = bi(i, 188u);
    let slot = slot_of(index);
    if (slot < 0) { return vec4<f32>(0.0); }
    let mode = (bi(i, 228u) >> 2u) & 3;
    let fill = (bi(i, 228u) >> 8u) & 1;
    let image = max(slot_size(slot), vec2<f32>(1.0));
    let slice = bv(i, 196u);
    let ratio = slice / vec4<f32>(image, image);
    let b = uv_in * size;
    var uv = vec2<f32>(0.0);
    if (fill == 0 && b.x > w.x && b.x < size.x - w.z && b.y > w.y && b.y < size.y - w.w) { return vec4<f32>(0.0); }
    if (slice.x < image.x * 0.5) {
        if (mode == 1) {
            let middle = 1.0 - (ratio.xy + ratio.zw);
            let repeat = floor((size * middle) / max(w.xy, vec2<f32>(0.0001)));
            uv.x = (b.x - w.x) / (size.x - (w.x + w.z)) * repeat.x;
            uv.x = (uv.x - middle.x * trunc(uv.x / max(middle.x, 0.0001))) + ratio.x;
            uv.y = (b.y - w.y) / (size.y - (w.y + w.w)) * repeat.y;
            uv.y = (uv.y - middle.y * trunc(uv.y / max(middle.y, 0.0001))) + ratio.y;
        } else {
            uv.x = (b.x - w.x) / (size.x - (w.x + w.z));
            uv.x = uv.x * (1.0 - (ratio.x + ratio.z)) + ratio.x;
            uv.y = (b.y - w.y) / (size.y - (w.y + w.w));
            uv.y = uv.y * (1.0 - (ratio.y + ratio.w)) + ratio.y;
        }
    }
    if (b.x < w.x) { uv.x = (b.x / w.x) * ratio.x; } else if (b.x > size.x - w.z) { uv.x = ((b.x - (size.x - w.z)) / w.z) * ratio.z + (1.0 - ratio.z); }
    if (b.y < w.y) { uv.y = (b.y / w.y) * ratio.y; } else if (b.y > size.y - w.w) { uv.y = ((b.y - (size.y - w.w)) / w.w) * ratio.w + (1.0 - ratio.w); }
    return sample_grad(slot, clamp(uv, vec2<f32>(0.0), vec2<f32>(1.0)), vec2<f32>(0.0), vec2<f32>(0.0));
}

// How much of this pixel the background may paint (background-clip): the
// border box everywhere, the padding and content boxes to their edge, text
// only what the panel's glyphs cover (a mask texture)
fn background_clip(i: u32, pos: vec2<f32>, size: vec2<f32>, uv: vec2<f32>, rh: vec4<f32>, rv: vec4<f32>, grad: f32) -> f32 {
    let clip = (bi(i, 228u) >> 12u) & 3;
    if (clip == 0) { return 1.0; }
    let rect = bv(i, 252u);
    if (clip != 3) { return sdf_coverage(inset_box_sdf(pos, size, rh, rv, rect), grad); }
    let mask = bi(i, 244u);
    if (mask == 0) { return 0.0; }
    let at = (uv * size - rect.xy) / max(rect.zw, vec2<f32>(0.0001));
    if (any(at < vec2<f32>(0.0)) || any(at > vec2<f32>(1.0))) { return 0.0; }
    let slot = slot_of(mask);
    if (slot < 0) { return 0.0; }
    return clamp(sample_grad(slot, at, vec2<f32>(0.0), vec2<f32>(0.0)).a, 0.0, 1.0);
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

// ---- an instance's colour and how much of the pixel its shape covers

struct Shaded { color: vec4<f32>, coverage: f32 }

fn shade(in: VsOut, dx: vec2<f32>, dy: vec2<f32>) -> Shaded {
    var out: Shaded;
    let i = in.instance;
    let flags = u32(bi(i, 228u));
    let premultiplied = (params.flags & PREMULTIPLIED) != 0u;
    let opacity = select(1.0, params.opacity, (params.flags & PANEL_OPACITY) != 0u);

    // a glyph: its text instance's coverage of this pixel, in its colour or its gradient's
    if ((flags & (1u << 14u)) != 0u) {
        let t = u32(max(bi(i, 144u), 0));
        let trect = tv(t, 0u);
        let p = trect.xy + in.uv * trect.zw;
        let ps = max(0.5 * (length(dx * trect.zw) + length(dy * trect.zw)), 0.0001);
        var color = in.color;
        let tex = ti(t, 144u);
        if (tex < 0) {
            let g = text_gradient(-tex - 1, tv(t, 208u), p);
            color = vec4<f32>(g.rgb, color.a * clamp(g.a, 0.0, 1.0));
        }
        out.coverage = select(text_coverage(t, p, ps), 1.0, ti(t, 248u) == 0);
        color.a *= out.coverage;
        out.color = color;
        return out;
    }

    let size = bv(i, 0u).zw;
    let grad = 0.5 * (length(dx * size) + length(dy * size));
    let rh = bv(i, 32u);
    let rv = bv(i, 48u);
    let mode = flags & 3u;

    // modes 1 and 2: a shadow. BackgroundRect is its shape in the quad, BackgroundAngle the CSS blur radius
    if (mode == 1u || mode == 2u) {
        let shape = bv(i, 156u);
        let half = shape.zw * 0.5;
        let q = in.uv * size - (shape.xy + half);
        let sigma = bf(i, 152u) * 0.5;
        var a: f32;
        if (sigma > 0.01) { a = rounded_rect_shadow(q, half, rh, rv, sigma); } else { a = sdf_coverage(rounded_rect_sdf(q, half, rh, rv), grad); }
        if (mode == 2u) { a = 1.0 - a; }
        out.coverage = clamp(a, 0.0, 1.0);
        out.color = vec4<f32>(in.color.rgb, in.color.a * out.coverage);
        return out;
    }
    // mode 3: an outline. BackgroundRect is (panel w, panel h, width, offset), BackgroundAngle how far the quad reaches past the panel
    if (mode == 3u) {
        let o = bv(i, 156u);
        let bloat = bf(i, 152u);
        let p = (o.xy + bloat * 2.0) * in.uv - bloat;
        let d = rounded_rect_sdf(p - o.xy * 0.5, o.xy * 0.5, rh, rv);
        out.coverage = clamp(sdf_coverage(d - (o.w + o.z), grad) - sdf_coverage(d - o.w, grad), 0.0, 1.0);
        out.color = vec4<f32>(in.color.rgb, in.color.a * out.coverage);
        return out;
    }

    // mode 0: a box
    let pos = (in.uv - 0.5) * size;
    let d_outer = rounded_rect_sdf(pos, size * 0.5, rh, rv);
    var col = in.color;
    if (premultiplied) { col = vec4<f32>(col.rgb * col.a, col.a); }
    var mask = 1.0;

    // its background image or gradient (a negative TextureIndex is a gradient's), a tile
    // sized and placed by background-size and -position
    let tex = bi(i, 144u);
    if (tex != 0) {
        let bg = bv(i, 156u);
        let bg_size = max(bg.zw, vec2<f32>(0.0001));
        var tint = bv(i, 172u);
        tint.a = clamp(tint.a * opacity, 0.0, 1.0);
        let angle = bf(i, 152u);
        let rot = mat2x2<f32>(cos(angle), sin(angle), -sin(angle), cos(angle));
        let scale = size / bg_size;
        let uv = rot * (-bg.xy / bg_size + in.uv * scale - 0.5) + 0.5;
        let repeat = (flags >> 9u) & 7u;
        var image: vec4<f32>;
        if (tex > 0) {
            // s&box samples with a bias of -1.5: sharper
            let k = exp2(-1.5);
            image = sample_texture(tex, bi(i, 148u), uv, rot * (dx * scale) * k, rot * (dy * scale) * k);
        } else {
            var tile = uv;
            if (repeat == 0u || repeat == 1u) { tile.x = fract(tile.x); }
            if (repeat == 0u || repeat == 2u) { tile.y = fract(tile.y); }
            image = gradient(-tex - 1, tile, bg_size);
        }
        if (repeat != 0u && repeat != 4u) {
            if (repeat != 1u && (uv.x < 0.0 || uv.x > 1.0)) { image = vec4<f32>(0.0); }
            if (repeat != 2u && (uv.y < 0.0 || uv.y > 1.0)) { image = vec4<f32>(0.0); }
        }
        let image_coverage = clamp(image.a, 0.0, 1.0);
        if (premultiplied) {
            image = vec4<f32>(image.rgb * tint.rgb, image.a) * tint.a;
            col = image + col * (1.0 - image.a);
        } else {
            image *= tint;
            let over = image.a + col.a * (1.0 - image.a);
            col = vec4<f32>(mix(col.rgb, image.rgb, select(0.0, image.a / over, over > 0.0)), over);
        }
        // a texture's alpha is a mask (a glyph's edge lives there), so it's coverage; a gradient's isn't
        if (tex > 0) { mask = image_coverage; }
    }

    // the border isn't clipped, so the background is cut back before the border goes on
    let clip = background_clip(i, pos, size, in.uv, rh, rv, grad);
    if (premultiplied) { col *= clip; } else { col.a *= clip; }
    mask *= clip;

    let border = bv(i, 64u);
    if (((flags >> 2u) & 3u) > 0u) {
        var tint = bv(i, 212u);
        tint.a = clamp(tint.a * opacity, 0.0, 1.0);
        let b = image_border(i, in.uv, size, border) * tint;
        col = alpha_blend(b, col);
        mask = max(mask, clamp(b.a, 0.0, 1.0));
    } else if (border.x != 0.0 || border.y != 0.0 || border.z != 0.0 || border.w != 0.0) {
        // the border is everything inside the box that isn't inside the padding box
        let d_inner = inset_box_sdf(pos, size, rh, rv, border);
        let p = in.uv * size;
        var side: vec4<f32>;
        if (((flags >> 4u) & 0xfu) != 0u) {
            side = styled_border(i, p, size, border, rh, rv, dx * size, dy * size, grad);
        } else {
            side = border_side_color(p, size, border, bv(i, 80u), bv(i, 96u), bv(i, 112u), bv(i, 128u), grad);
        }
        side.a = clamp(side.a * opacity, 0.0, 1.0) * (1.0 - sdf_coverage(d_inner, grad));
        col = alpha_blend(side, col);
        mask = max(mask, side.a);
    }

    let edge = sdf_coverage(d_outer, grad);
    out.coverage = edge * mask;
    if (premultiplied) { col *= edge; } else { col.a *= edge; }
    out.color = col;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // the pixel's footprint across the box and in panel space, while control flow is uniform
    let dx = dpdx(in.uv);
    let dy = dpdy(in.uv);
    let panel = in.panel.xy / in.panel.w;
    let px = dpdx(panel);
    let py = dpdy(panel);
    if ((params.flags & SOLID) != 0u) { return vec4<f32>(1.0, 0.0, 0.0, 1.0); }
    let i = in.instance;
    var col = shade(in, dx, dy).color;
    // its clip stack, and an outset shadow's inverse one that keeps it out of its panel
    var clip = 1.0;
    let scissor = bi(i, 232u);
    if (scissor >= 0) { clip *= scissor_coverage(scissor, panel, px, py); }
    let inverse = bi(i, 240u);
    if (inverse >= 0) { clip *= scissor_coverage(inverse, panel, px, py); }
    if (clip <= 0.0) { discard; }
    if ((params.flags & PREMULTIPLIED) != 0u) { return col * clip; }
    return vec4<f32>(col.rgb, col.a * clip);
}
