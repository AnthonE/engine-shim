// s&box's two stock material shaders, as near as we get yet: complex (91%
// of materials) and blendable (two layers, blended by vertex paint).
//
// complex's textures as s&box compiles them: g_tColor (colour; alpha the
// alpha test's mask, or metalness with F_METALNESS_TEXTURE), g_tNormal
// (tangent-space x and y, roughness in blue), g_tAmbientOcclusion (red),
// g_tTintMask (red: how much of g_vColorTint a texel takes).
// blendable's: each layer's colour, normal (as complex's) and g_tRma
// (metalness, occlusion, roughness); the vertices' paint, TEXCOORD4
// (red: how much of layer B) and TEXCOORD5 (a tint).
//
// Lit by one sun (GGX) and a sky that metal reflects; the tangent frame
// from the surface's derivatives, as s&box's meshes carry no tangents.

struct View {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    sun: vec4<f32>,       // towards the sun
    sun_color: vec4<f32>,
    sky: vec4<f32>,
    ground: vec4<f32>,
};

struct Material {
    tint: vec4<f32>,
    // x: alpha test reference (0: none), y: metalness, z: metalness from
    // the colour's alpha (1), w: normal maps (1)
    params: vec4<f32>,
    // x: blendable (1), y: tint by mask (1), z: tint by vertex paint (1)
    mode: vec4<f32>,
    tint_b: vec4<f32>,
};

@group(0) @binding(0) var<uniform> view: View;
@group(1) @binding(0) var<uniform> material: Material;
@group(1) @binding(1) var color_a: texture_2d<f32>;
@group(1) @binding(2) var occlusion_a: texture_2d<f32>;
@group(1) @binding(3) var samp: sampler;
@group(1) @binding(4) var normal_a: texture_2d<f32>;
@group(1) @binding(5) var color_b: texture_2d<f32>;
@group(1) @binding(6) var normal_b: texture_2d<f32>;
@group(1) @binding(7) var occlusion_b: texture_2d<f32>;
@group(1) @binding(8) var tint_mask: texture_2d<f32>;

struct In {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) blend: vec4<f32>,
    @location(4) paint: vec4<f32>,
};

struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) blend: vec4<f32>,
    @location(4) paint: vec4<f32>,
};

@vertex
fn vs_main(v: In) -> Out {
    var o: Out;
    o.clip = view.view_proj * vec4<f32>(v.position, 1.0);
    o.world = v.position;
    o.normal = v.normal;
    o.uv = v.uv;
    o.blend = v.blend;
    o.paint = v.paint;
    return o;
}

const PI: f32 = 3.14159265;

// the sky a surface sees in a direction, blurred as it gets rougher
fn sky(d: vec3<f32>, rough: f32) -> vec3<f32> {
    let up = clamp(d.z * mix(4.0, 1.0, rough), -1.0, 1.0) * 0.5 + 0.5;
    return mix(view.ground.rgb, view.sky.rgb, up);
}

fn ggx(n: vec3<f32>, h: vec3<f32>, rough: f32) -> f32 {
    let a = max(rough * rough, 0.002);
    let nh = max(dot(n, h), 0.0);
    let d = nh * nh * (a * a - 1.0) + 1.0;
    return a * a / (PI * d * d);
}

@fragment
fn fs_main(i: Out, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    let blendable = material.mode.x > 0.5;
    var albedo = textureSample(color_a, samp, i.uv);
    var packed = textureSample(normal_a, samp, i.uv);
    var occ = textureSample(occlusion_a, samp, i.uv);
    // every sample taken before any branch on it, as derivatives need
    let albedo_b = textureSample(color_b, samp, i.uv);
    let packed_b = textureSample(normal_b, samp, i.uv);
    let occ_b = textureSample(occlusion_b, samp, i.uv);
    let mask = textureSample(tint_mask, samp, i.uv).r;
    let dp1 = dpdx(i.world);
    let dp2 = dpdy(i.world);
    let du1 = dpdx(i.uv);
    let du2 = dpdy(i.uv);

    var tint = material.tint.rgb;
    var ao = occ.r;
    var metal = material.params.y;
    if (blendable) {
        let k = clamp(i.blend.r, 0.0, 1.0);
        albedo = vec4<f32>(mix(albedo.rgb * material.tint.rgb, albedo_b.rgb * material.tint_b.rgb, k), 1.0);
        packed = mix(packed, packed_b, k);
        occ = mix(occ, occ_b, k);
        metal = occ.r;
        ao = occ.g;
        tint = vec3<f32>(1.0);
    }
    if (material.params.x > 0.0 && albedo.a < material.params.x) {
        discard;
    }
    if (material.mode.y > 0.5) {
        tint = mix(vec3<f32>(1.0), tint, mask);
    }
    if (material.mode.z > 0.5) {
        tint = tint * i.paint.rgb;
    }

    var n = normalize(i.normal);
    if (!front) { n = -n; }
    var rough = 0.6;
    if (material.params.w > 0.5) {
        // the cotangent frame, from how position and uv change across the pixel
        let dp2perp = cross(dp2, n);
        let dp1perp = cross(n, dp1);
        let t = dp2perp * du1.x + dp1perp * du2.x;
        let b = dp2perp * du1.y + dp1perp * du2.y;
        let inv = inverseSqrt(max(max(dot(t, t), dot(b, b)), 1e-20));
        let xy = packed.rg * 2.0 - 1.0;
        let ts = vec3<f32>(xy.x, -xy.y, sqrt(max(1.0 - dot(xy, xy), 0.0)));
        n = normalize(t * inv * ts.x + b * inv * ts.y + n * ts.z);
        rough = packed.b;
    }
    if (blendable) { rough = occ.b; }
    let base = albedo.rgb * tint;
    if (!blendable && material.params.z > 0.5) { metal = albedo.a; }
    let l = normalize(view.sun.xyz);
    let v = normalize(view.eye.xyz - i.world);
    let h = normalize(l + v);
    let ndl = max(dot(n, l), 0.0);
    let ndv = max(dot(n, v), 1e-3);
    let f0 = mix(vec3<f32>(0.04), base, metal);
    let fresnel = f0 + (max(vec3<f32>(1.0 - rough), f0) - f0) * pow(1.0 - ndv, 5.0);
    let k = (rough + 1.0) * (rough + 1.0) / 8.0;
    let vis = 1.0 / ((ndl * (1.0 - k) + k) * (ndv * (1.0 - k) + k) * 4.0);
    let spec_sun = ggx(n, h, rough) * vis * ndl;
    let diffuse = base * (1.0 - metal);
    let ambient = sky(n, 1.0);
    let reflected = sky(reflect(-v, n), rough);
    let lit = diffuse * (view.sun_color.rgb * ndl + ambient) * ao
        + fresnel * (view.sun_color.rgb * spec_sun + reflected * ao);
    let debug = view.ground.w;
    if (debug == 1.0) { return vec4<f32>(base, 1.0); }
    if (debug == 2.0) { return vec4<f32>(n * 0.5 + 0.5, 1.0); }
    if (debug == 3.0) { return vec4<f32>(vec3<f32>(ao), 1.0); }
    if (debug == 4.0) { return vec4<f32>(vec3<f32>(rough), 1.0); }
    // a gentle curve, so the sun's highlights roll off
    let mapped = lit / (1.0 + lit * 0.25);
    return vec4<f32>(mapped, 1.0);
}
