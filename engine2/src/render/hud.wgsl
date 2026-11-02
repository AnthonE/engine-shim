// A game's own shaders over a quad, ported where a game was measured using
// them: Facepunch's Sandbox (MIT) paints its HUD into a float target, blurs
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
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> vertices: array<vec4<f32>>;  // x y u v, then r g b a, a vertex
@group(0) @binding(2) var t0: texture_2d<f32>;   // glow: GlowSource; composite: HudTexture
@group(0) @binding(3) var t1: texture_2d<f32>;   // composite: GlowTexture
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
