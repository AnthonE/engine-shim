// The frame drawn in linear HDR, exposed and tonemapped as s&box's
// Tonemapping post-process does it (game/core/shaders/tonemapping/
// tonemapping.shader): its curves as it writes them, Hable's filmic its
// default ("the default curve Source 2 uses").

struct Tone {
    // x: exposure, y: the curve (0 linear, 1 Hable, 2 ACES, 3 Reinhard-Jodie,
    // 4 AgX), z: shown as it is (RENDER_DEBUG: 1)
    params: vec4<f32>,
};
@group(0) @binding(0) var<uniform> tone: Tone;
@group(0) @binding(1) var hdr: texture_2d<f32>;

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u)) * 2.0 - 1.0;
    return vec4<f32>(p, 0.0, 1.0);
}

fn luminance(c: vec3<f32>) -> f32 { return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722)); }

fn hable_partial(x: vec3<f32>) -> vec3<f32> {
    let a = 0.15; let b = 0.50; let c = 0.10; let d = 0.20; let e = 0.02; let f = 0.30;
    return ((x * (a * x + c * b) + d * e) / (x * (a * x + b) + d * f)) - e / f;
}

fn hable(x: vec3<f32>) -> vec3<f32> {
    return hable_partial(x) / hable_partial(vec3<f32>(11.2));
}

// WGSL's matrices are column-major: m1's columns are tonemapping.shader's rows,
// and mul(rgb, m1) is m1 * rgb here
fn aces(x: vec3<f32>) -> vec3<f32> {
    let m1 = mat3x3<f32>(vec3<f32>(0.59719, 0.07600, 0.02840), vec3<f32>(0.35458, 0.90834, 0.13383), vec3<f32>(0.04823, 0.01566, 0.83777));
    let m2 = mat3x3<f32>(vec3<f32>(1.60475, -0.10208, -0.00327), vec3<f32>(-0.53108, 1.10813, -0.07276), vec3<f32>(-0.07367, -0.00605, 1.07602));
    let v = transpose(m1) * x;
    let a = v * (v + 0.0245786) - 0.000090537;
    let b = v * (0.983729 * v + 0.4329510) + 0.238081;
    return clamp(transpose(m2) * (a / b), vec3<f32>(0.0), vec3<f32>(1.0));
}

fn reinhard_jodie(x: vec3<f32>) -> vec3<f32> {
    let l = luminance(x);
    let tv = x / (1.0 + x);
    return mix(x / (1.0 + l), tv, tv);
}

fn agx(c: vec3<f32>) -> vec3<f32> {
    let m = mat3x3<f32>(vec3<f32>(0.856627153315983, 0.137318972929847, 0.11189821299995),
        vec3<f32>(0.0951212405381588, 0.761241990602591, 0.0767994186031903),
        vec3<f32>(0.0482516061458583, 0.101439036467562, 0.811302368396859));
    var v = transpose(m) * max(c, vec3<f32>(0.0));
    let lo = -12.47393; let hi = 4.026069;
    v = (clamp(log2(max(v, vec3<f32>(1e-10))), vec3<f32>(lo), vec3<f32>(hi)) - lo) / (hi - lo);
    let v2 = v * v; let v4 = v2 * v2;
    v = -0.20687445 * v + 6.80888933 * v2 - 37.60519607 * v2 * v + 93.32681938 * v4 - 95.2780858 * v4 * v + 33.96372259 * v4 * v2;
    // the Punchy look
    v = pow(max(v, vec3<f32>(0.0)), vec3<f32>(1.35));
    let l = luminance(v);
    v = l + 1.4 * (v - l);
    let inv = mat3x3<f32>(vec3<f32>(1.19687900512017, -0.0528968517574562, -0.0529716355144438),
        vec3<f32>(-0.0980208811401368, 1.15190312990417, -0.0980434501171241),
        vec3<f32>(-0.0990297440797205, -0.0989611768448433, 1.15107367264116));
    v = transpose(inv) * v;
    return max(pow(max(v, vec3<f32>(0.0)), vec3<f32>(2.2)), vec3<f32>(0.0));
}

@fragment
fn fs_main(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    let c = textureLoad(hdr, vec2<i32>(p.xy), 0);
    if (tone.params.z > 0.5) { return vec4<f32>(clamp(c.rgb, vec3<f32>(0.0), vec3<f32>(1.0)), c.a); }
    let rgb = max(c.rgb, vec3<f32>(0.0)) * tone.params.x;
    let curve = tone.params.y;
    var out = rgb;
    if (curve == 1.0) { out = hable(rgb); }
    else if (curve == 2.0) { out = aces(rgb); }
    else if (curve == 3.0) { out = reinhard_jodie(rgb); }
    else if (curve == 4.0) { out = agx(rgb); }
    return vec4<f32>(clamp(out, vec3<f32>(0.0), vec3<f32>(1.0)), c.a);
}
