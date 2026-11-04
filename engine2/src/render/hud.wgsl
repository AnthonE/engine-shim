// Shaders over a quad, ported where a game was measured using them. A panel
// layer composited (s&box's ui_filter, ui_dropshadow and ui_borderwrap, MIT:
// a panel with `filter:`, `mask-image` or a drop shadow is drawn into a
// layer of its own, then put on what's under it). And a game's own: Facepunch's Sandbox (MIT) paints its HUD into a float target, blurs
// a glow from what's brighter than white (hud_glow.shader, a full-target
// blit, twice), then composites both over the frame with scanlines
// (hud_composite.shader, a UI quad over each region it painted).

struct Params {
    viewport: vec4<f32>,   // the target's x, y, w, h
    layer: mat4x4<f32>,    // LayerMat
    transform: mat4x4<f32>,// TransformMat
    // glow: GlowStep.xy, GlowReach, GlowExtract
    glow: vec4<f32>,
    // composite: HudInvSize.xy, GlowStrength, HudOpacity
    hud: vec4<f32>,
    // ScanlineIntensity, ScanlinePeriod, ScanlineThickness, ScanlineSoftness
    scan: vec4<f32>,
    // the grab: its texels sRGB-encoded (1) or not, UIGammaOutput, the frame rect's z, w
    flags: vec4<f32>,
    // a panel layer's composite (ui_filter, ui_dropshadow, ui_borderwrap): BoxPosition, BoxSize
    box_: vec4<f32>,
    // FilterBlur, FilterSaturate, FilterSepia, FilterBrightness
    filt: vec4<f32>,
    // FilterContrast, FilterInvert, FilterHueRotate, PainterSourceGamma
    filt2: vec4<f32>,
    // FilterTint; or FilterDropShadowColor, FilterBorderWrapColor
    tint: vec4<f32>,
    // FilterDropShadowOffset.xy, FilterDropShadowBlur (or FilterBorderWrapWidth), D_BLENDMODE
    shadow: vec4<f32>,
    // MaskPos: the mask's offset in the box, and its size
    mask_pos: vec4<f32>,
    // D_MASK_IMAGE, MaskMode (0 match source, 1 alpha, 2 luminance), MaskScope (0 default, 1 filter), MaskAngle
    mask: vec4<f32>,
    // the mask sampler's address modes, u and v (0 wrap, 1 mirror, 2 clamp, 3 border, 4 mirror once)
    mask_address: vec4<f32>,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> vertices: array<vec4<f32>>;  // x y u v, then r g b a, a vertex
@group(0) @binding(2) var t0: texture_2d<f32>;   // glow: GlowSource; composite: HudTexture; a layer's composite: the layer
@group(0) @binding(3) var t1: texture_2d<f32>;   // composite: GlowTexture; ui_filter: MaskTexture
@group(0) @binding(4) var t2: texture_2d<f32>;   // composite: FrameBufferCopyTexture
@group(0) @binding(5) var clamp_s: sampler;      // g_sTrilinearClamp

struct Out {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) clip: vec4<f32>,
}

fn to_linear(c: vec3<f32>) -> vec3<f32> {
    let lo = c / 12.92;
    let hi = pow((max(c, vec3<f32>(0.0)) + 0.055) / 1.055, vec3<f32>(2.4));
    return select(hi, lo, c <= vec3<f32>(0.04045));
}

fn to_gamma(c: vec3<f32>) -> vec3<f32> {
    let lo = c * 12.92;
    let hi = 1.055 * pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(hi, lo, c <= vec3<f32>(0.0031308));
}

// ---- hud_glow: a blit, its positions already the target's

@vertex
fn vs_blit(@builtin(vertex_index) vi: u32) -> Out {
    let a = vertices[vi * 2u];
    var o: Out;
    o.position = vec4<f32>(a.xy, 0.0, 1.0);
    o.clip = o.position;
    o.uv = a.zw;
    return o;
}

const TAPS: i32 = 6;

fn fetch(uv: vec2<f32>) -> vec3<f32> {
    let s = textureSampleLevel(t0, clamp_s, uv, 0.0);
    if params.glow.w == 0.0 { return s.rgb; }
    // what's past white, straight and linear, by how much is covered
    let c = select(vec3<f32>(0.0), to_linear(s.rgb / s.a), s.a > 0.00001);
    return max(c - 1.0, vec3<f32>(0.0)) * s.a;
}

@fragment
fn fs_glow(in: Out) -> @location(0) vec4<f32> {
    let step = params.glow.xy * max(params.glow.z / f32(TAPS), 0.5);
    var sum = fetch(in.uv);
    var weights = 1.0;
    for (var k = 1; k <= TAPS; k++) {
        let w = exp(-f32(k * k) / (2.0 * 2.4 * 2.4));
        sum += (fetch(in.uv + step * f32(k)) + fetch(in.uv - step * f32(k))) * w;
        weights += w * 2.0;
    }
    return vec4<f32>(sum / weights, 1.0);
}

// ---- hud_composite: a UI quad (ui/vertex.hlsl's screen-space path)

@vertex
fn vs_ui(@builtin(vertex_index) vi: u32) -> Out {
    let a = vertices[vi * 2u];
    let m = params.layer * (params.transform * vec4<f32>(a.xy, 0.0, 1.0));
    let w = select(m.w, 1e-6, abs(m.w) < 1e-6);
    let ndc = 2.0 * (m.xy / w - params.viewport.xy) / params.viewport.zw - 1.0;
    var o: Out;
    o.position = vec4<f32>(ndc.x, -ndc.y, 0.0, 1.0);
    o.clip = o.position;
    o.uv = a.zw;
    return o;
}

fn scanline(y: f32) -> f32 {
    let t = fract(y / params.scan.y);
    let half = params.scan.z * 0.5;
    return 1.0 - smoothstep(half - params.scan.w, half + params.scan.w, abs(t - 0.5));
}

@fragment
fn fs_composite(in: Out) -> @location(0) vec4<f32> {
    let pixel = in.position.xy;
    let hud_uv = pixel * params.hud.xy;
    let ndc = in.clip.xy / in.clip.w;
    let frame_uv = (vec2<f32>(ndc.x, -ndc.y) * 0.5 + 0.5) * params.flags.zw;
    // the frame behind, linear (SrgbRead: the grab's bytes are sRGB)
    var backdrop = clamp(textureSampleLevel(t2, clamp_s, frame_uv, 0.0).rgb, vec3<f32>(0.0), vec3<f32>(1.0));
    if params.flags.x != 0.0 { backdrop = to_linear(backdrop); }

    // the glow, added onto the frame in linear space
    let excess = textureSampleLevel(t1, clamp_s, hud_uv, 0.0).rgb;
    let glow = (1.0 - exp(-excess * params.hud.z)) * params.hud.w;
    var lin = clamp(backdrop + glow, vec3<f32>(0.0), vec3<f32>(1.0));

    // the HUD: gamma-encoded and premultiplied, made straight and clamped
    // g_sTrilinearBorder: transparent black past the edges
    let inside = all(hud_uv >= vec2<f32>(0.0)) && all(hud_uv <= vec2<f32>(1.0));
    let s = select(vec4<f32>(0.0), textureSampleLevel(t0, clamp_s, hud_uv, 0.0), inside);
    var hud = select(vec4<f32>(0.0), clamp(vec4<f32>(s.rgb / s.a, s.a), vec4<f32>(0.0), vec4<f32>(1.0)), s.a > 0.00001);
    hud.a *= params.hud.w;
    var hud_linear = vec3<f32>(0.0);
    if hud.a > 0.0 {
        // blended in gamma space, as the rest of the UI
        lin = to_linear(mix(to_gamma(lin), hud.rgb, hud.a));
        hud_linear = to_linear(hud.rgb);
    }

    lin += scanline(pixel.y) * clamp(params.scan.x, 0.0, 1.0) * (hud_linear * hud.a + glow);
    var out = clamp(lin, vec3<f32>(0.0), vec3<f32>(1.0));
    // UIEncodeOutput
    if params.flags.y != 0.0 { out = to_gamma(out); }
    return vec4<f32>(out, 1.0);
}

// ---- a panel layer composited (core/shaders/ui_filter.shader,
// ui_dropshadow.shader, ui_borderwrap.shader). The quad covers the layer
// one to one; the layer holds premultiplied colour, as the UI writes it.

// g_sTrilinearBorder: transparent black past the layer's edges
fn layer_at(uv: vec2<f32>) -> vec4<f32> {
    let inside = all(uv >= vec2<f32>(0.0)) && all(uv <= vec2<f32>(1.0));
    return select(vec4<f32>(0.0), textureSampleLevel(t0, clamp_s, uv, 0.0), inside);
}

// ui/blur.hlsl's gaussian, sigma in texels. s&box reads it from the layer's
// gaussian mip chain; a layer here has one level, so it's summed directly:
// taps 0.6 sigma apart out to 3 sigma, weighted as the gaussian.
const BLUR_TAPS: i32 = 5;

fn blurred(uv: vec2<f32>, sigma: f32) -> vec4<f32> {
    if sigma <= 0.05 { return layer_at(uv); }
    let inv = 1.0 / vec2<f32>(textureDimensions(t0));
    let spacing = max(sigma * 0.6, 0.5);
    var sum = vec4<f32>(0.0);
    var weights = 0.0;
    for (var y = -BLUR_TAPS; y <= BLUR_TAPS; y++) {
        for (var x = -BLUR_TAPS; x <= BLUR_TAPS; x++) {
            let d = vec2<f32>(f32(x), f32(y)) * spacing;
            let w = exp(-dot(d, d) / (2.0 * sigma * sigma));
            sum += layer_at(uv + d * inv) * w;
            weights += w;
        }
    }
    return sum / weights;
}

// SampleLayer: the layer's colour, straight and linear. A painter's layer
// (PainterSourceGamma) or a gamma-blending UI's is sRGB-encoded and decoded
// after it's made straight; otherwise its texels are decoded as they're read.
fn sample_layer(uv: vec2<f32>, sigma: f32) -> vec4<f32> {
    let gamma = params.filt2.w != 0.0 || params.flags.y != 0.0;
    var c = blurred(uv, sigma);
    if !gamma { c = vec4<f32>(to_linear(c.rgb), c.a); }
    var rgb = select(vec3<f32>(0.0), c.rgb / c.a, c.a > 0.00001);
    if gamma { rgb = to_linear(rgb); }
    return vec4<f32>(rgb, c.a);
}

// ApplyColorFilters: CSS's filter functions, on sRGB values, in CSS's order
// as ui_filter.shader applies them
fn color_filters(c: vec4<f32>) -> vec4<f32> {
    var rgb = to_gamma(c.rgb);
    var a = c.a;
    rgb = clamp((rgb - 0.5) * params.filt2.x + 0.5, vec3<f32>(0.0), vec3<f32>(1.0));
    // sepia
    let s = 1.0 - params.filt.z;
    rgb = clamp(vec3<f32>(
        dot(rgb, vec3<f32>(0.393 + 0.607 * s, 0.769 - 0.769 * s, 0.189 - 0.189 * s)),
        dot(rgb, vec3<f32>(0.349 - 0.349 * s, 0.686 + 0.314 * s, 0.168 - 0.168 * s)),
        dot(rgb, vec3<f32>(0.272 - 0.272 * s, 0.534 - 0.534 * s, 0.131 + 0.869 * s))), vec3<f32>(0.0), vec3<f32>(1.0));
    a = clamp(a, 0.0, 1.0);
    // invert
    rgb = mix(rgb, 1.0 - rgb, params.filt2.y);
    // hue-rotate
    let hc = cos(radians(params.filt2.z));
    let hs = sin(radians(params.filt2.z));
    rgb = clamp(vec3<f32>(
        dot(rgb, vec3<f32>(0.213 + hc * 0.787 - hs * 0.213, 0.715 - hc * 0.715 - hs * 0.715, 0.072 - hc * 0.072 + hs * 0.928)),
        dot(rgb, vec3<f32>(0.213 - hc * 0.213 + hs * 0.143, 0.715 + hc * 0.285 + hs * 0.140, 0.072 - hc * 0.072 - hs * 0.283)),
        dot(rgb, vec3<f32>(0.213 - hc * 0.213 - hs * 0.787, 0.715 - hc * 0.715 + hs * 0.715, 0.072 + hc * 0.928 + hs * 0.072))), vec3<f32>(0.0), vec3<f32>(1.0));
    // saturate
    let k = params.filt.y;
    rgb = clamp(vec3<f32>(
        dot(rgb, vec3<f32>(0.213 + 0.787 * k, 0.715 - 0.715 * k, 0.072 - 0.072 * k)),
        dot(rgb, vec3<f32>(0.213 - 0.213 * k, 0.715 + 0.285 * k, 0.072 - 0.072 * k)),
        dot(rgb, vec3<f32>(0.213 - 0.213 * k, 0.715 - 0.715 * k, 0.072 + 0.928 * k))), vec3<f32>(0.0), vec3<f32>(1.0));
    // brightness: a straight multiply
    rgb = clamp(rgb * params.filt.w, vec3<f32>(0.0), vec3<f32>(1.0));
    return vec4<f32>(to_linear(rgb), a) * params.tint;
}

// an address mode applied to a coordinate, as the mask's sampler would;
// the second value is false where a border sampler reads nothing
fn address(x: f32, mode: f32) -> vec2<f32> {
    if mode == 0.0 { return vec2<f32>(fract(x), 1.0); }
    if mode == 1.0 { let f = fract(x * 0.5) * 2.0; return vec2<f32>(select(2.0 - f, f, f <= 1.0), 1.0); }
    if mode == 3.0 { return vec2<f32>(x, select(0.0, 1.0, x >= 0.0 && x <= 1.0)); }
    if mode == 4.0 { return vec2<f32>(clamp(abs(x), 0.0, 1.0), 1.0); }
    return vec2<f32>(clamp(x, 0.0, 1.0), 1.0);
}

// UIEncodeOutput, and premultiplied where D_BLENDMODE 3 blends it
fn layer_out(c: vec4<f32>, premultiply: bool) -> vec4<f32> {
    var o = c;
    if params.flags.y != 0.0 { o = vec4<f32>(to_gamma(clamp(o.rgb, vec3<f32>(0.0), vec3<f32>(1.0))), o.a); }
    if premultiply && params.shadow.w == 3.0 { o = vec4<f32>(o.rgb * o.a, o.a); }
    return o;
}

@fragment
fn fs_filter(in: Out) -> @location(0) vec4<f32> {
    var c = color_filters(sample_layer(in.uv, params.filt.x));
    if params.mask.x != 0.0 {
        let size = max(params.mask_pos.zw, vec2<f32>(1e-3));
        var v = -params.mask_pos.xy / size + in.uv * (params.box_.zw / size);
        // RotateTexCoord, about the middle
        let ca = cos(params.mask.w);
        let sa = sin(params.mask.w);
        v = vec2<f32>(ca * (v.x - 0.5) - sa * (v.y - 0.5), sa * (v.x - 0.5) + ca * (v.y - 0.5)) + 0.5;
        let u = address(v.x, params.mask_address.x);
        let w = address(v.y, params.mask_address.y);
        // the mask's texels are sRGB (SrgbRead), its border transparent black
        let m = textureSampleLevel(t1, clamp_s, vec2<f32>(u.x, w.x), 0.0) * (u.y * w.y);
        let mask = select(dot(to_linear(m.rgb), vec3<f32>(0.2126729, 0.7151522, 0.0721750)), m.a, params.mask.y == 1.0);
        if params.mask.z == 1.0 {
            // mask-scope filter: the filtered layer where the mask is, the layer as it was elsewhere
            c = mix(sample_layer(in.uv, 0.0), c, mask);
        } else {
            c.a *= mask;
        }
    }
    return layer_out(c, true);
}

@fragment
fn fs_dropshadow(in: Out) -> @location(0) vec4<f32> {
    // the layer's alpha, offset and blurred, in the shadow's colour
    let inv = 1.0 / vec2<f32>(textureDimensions(t0));
    let at = in.uv - params.shadow.xy * inv;
    let a = params.tint.a * blurred(at, params.shadow.z).a;
    return layer_out(vec4<f32>(to_linear(params.tint.rgb), a), false);
}

@fragment
fn fs_borderwrap(in: Out) -> @location(0) vec4<f32> {
    // how much of a circle of the border's width around the pixel the layer covers
    let reach = params.shadow.z / vec2<f32>(textureDimensions(t0));
    var covered = 0.0;
    for (var k = 0; k < 32; k++) {
        let angle = f32(k) * 6.2831853 / 32.0;
        covered += layer_at(in.uv + vec2<f32>(cos(angle), sin(angle)) * reach).a;
    }
    covered /= 3.14159265;
    // nothing blended toward the border's colour, then its edges sharpened
    let a = clamp(params.tint.a * covered * 2.0 - 1.0, 0.0, 1.0);
    return layer_out(vec4<f32>(params.tint.rgb, a), false);
}
