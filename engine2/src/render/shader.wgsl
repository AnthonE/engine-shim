// s&box's two stock material shaders, as near as we get yet: complex (91%
// of materials) and blendable (layers, blended by vertex paint), and glass.
//
// complex's textures as s&box compiles them: g_tColor (colour; alpha the
// alpha test's mask, or metalness with F_METALNESS_TEXTURE), g_tNormal
// (the normal hemi-octahedral in red and green, roughness in blue, as
// vr_shared_standard_ps_code.fxc reads it), g_tAmbientOcclusion (red),
// g_tTintMask (red: how much of g_vColorTint a texel takes), g_tMetalness
// (green, with F_METALNESS_TEXTURE; without it, metalness is the colour's
// alpha), g_tSelfIllumMask (F_SELF_ILLUM: what glows, how brightly the
// material says; black where there's none). Translucent (F_TRANSLUCENT):
// the colour's alpha is how opaque, drawn over what's behind.
// blendable's, as blendable.shader reads them: up to three layers here,
// each a colour (its alpha how much of the layer's tint it takes), a
// normal and g_tRma. A material of shader version 1 packs its normal as
// complex's and its g_tRma as metalness, occlusion and the layer's reveal
// mask; one of version 0 (most content yet) a plain normal, and g_tRma as
// roughness, metalness, occlusion and the reveal mask. The vertices' paint:
// TEXCOORD4 how much of layers B and C (red, green), TEXCOORD5 a tint.
//
// glass.shader's (its default, refractive glass): what's behind it, seen
// through it, tinted by its colour and less of it at grazing angles, where
// its reflections take over; F_CHEAP_GLASS is drawn over what's behind,
// its colour's alpha how opaque. Its g_tNormal a plain normal (its alpha a
// tint mask), its g_tRma roughness, metalness, occlusion.
//
// Lit by one sun (GGX) and a sky that metal reflects; the tangent frame
// from the surface's derivatives, as s&box's meshes carry no tangents.
//
// A map's own geometry is lit as s&box lights it from its baked lighting
// (vr_lighting.fxc, ComputeBakedLighting and CalculateIndirectLighting):
// each lightmap texel names up to four lights and how much of each reaches
// it (their shadows baked in), shaded as dynamic lights are; and its
// irradiance, the light that bounced, with the direction most of it came
// from (the "AHD" lightmap: ambient, highlight, direction), so a normal map
// still shades it. No sun but through the lightmaps, there.
//
// Everything else in a map is lit from the light probe volume chosen for
// it, as s&box lights it: each probe the light from each of six ways (an
// "ambient cube", the six stacked in the volume texture's depth), and up
// to four lights it names with how much of each reaches it, as a texel
// of the lightmaps does.

// Whether this pipeline alpha-tests (F_ALPHA_TEST). The opaque pipeline that
// doesn't is specialized without its discard, so the depth test can reject a
// pixel before it's shaded (what's nearer drawn first).
override ALPHA_TEST: bool = true;

struct View {
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    sun: vec4<f32>,       // towards the sun
    sun_color: vec4<f32>,
    sky: vec4<f32>,
    ground: vec4<f32>,
    sky_rotation: mat4x4<f32>,
    // world to the sun's shadow map; x: shadows (1), y: a texel of it in
    // uv, z: in the world
    shadow: mat4x4<f32>,
    shadow_params: vec4<f32>,
};

struct Material {
    tint: vec4<f32>,
    // x: alpha test reference (0: none), y: metalness, z: metalness from
    // the colour's alpha (1), w: normal maps (1)
    params: vec4<f32>,
    // x: blendable's layers (0: complex), y: tint by mask (1), z: tint by
    // vertex paint (1), w: blendable's shader version
    mode: vec4<f32>,
    tint_b: vec4<f32>,
    tint_c: vec4<f32>,
    // x, y: how softly layers B and C blend in; z: roughness's exponent
    blend: vec4<f32>,
    // layer B's texture coordinates' scale (x, y), and C's (z, w)
    scale: vec4<f32>,
    // x: translucent's opacity scale (0: opaque), or glass's blur; y:
    // glass's albedo absorption, z: cheap glass (1), w: self-illuminated (1)
    extra: vec4<f32>,
};

@group(0) @binding(0) var<uniform> view: View;
// what's been drawn so far, for glass to show through it
@group(0) @binding(1) var scene: texture_2d<f32>;
@group(0) @binding(2) var shadow_map: texture_depth_2d;
@group(0) @binding(3) var shadow_compare: sampler_comparison;
// the view's decals (decals.rs): each a box, its colour texture a layer of one array
struct DecalGpu {
    to_decal: mat4x4<f32>,
    // xyz: the axis it projects along; w: how much a surface turned from it fades it
    axis: vec4<f32>,
    tint: vec4<f32>,
    // x: its layer, y: its colour mix, z: its blend mode, w: 1 (0: none)
    params: vec4<f32>,
};
@group(0) @binding(4) var<storage, read> decals: array<DecalGpu>;
@group(0) @binding(5) var decal_array: texture_2d_array<f32>;
@group(0) @binding(6) var decal_sampler: sampler;
@group(1) @binding(0) var<uniform> material: Material;
@group(1) @binding(1) var color_a: texture_2d<f32>;
@group(1) @binding(2) var occlusion_a: texture_2d<f32>;
@group(1) @binding(3) var samp: sampler;
@group(1) @binding(4) var normal_a: texture_2d<f32>;
@group(1) @binding(5) var color_b: texture_2d<f32>;
@group(1) @binding(6) var normal_b: texture_2d<f32>;
@group(1) @binding(7) var occlusion_b: texture_2d<f32>;
@group(1) @binding(9) var color_c: texture_2d<f32>;
@group(1) @binding(10) var normal_c: texture_2d<f32>;
@group(1) @binding(11) var occlusion_c: texture_2d<f32>;

// each object: where it is, and its tint (a scene object's)
struct Object {
    model: mat4x4<f32>,
    tint: vec4<f32>,
    // x: skinned (1)
    flags: vec4<f32>,
};
@group(2) @binding(0) var<uniform> object: Object;
// a skinned object's bones: each one's pose over its bind pose's inverse
@group(2) @binding(1) var<storage, read> bones: array<mat4x4<f32>>;

// the map's baked lighting: the world's lightmap UV scale (x, y) and the
// indices' size in texels (z, w); a probe volume's world to its box (0..1
// across it) and its count of probes each way
struct Lighting {
    lightmap: vec4<f32>,
    to_local: mat4x4<f32>,
    probes: vec4<f32>,
};
@group(3) @binding(0) var<uniform> lighting: Lighting;
// (a probe volume's indices, strengths and ambient cubes in the same
// places: an object reads one or the other; each a layer, or a layer of
// probes)
@group(3) @binding(1) var light_indices: texture_2d_array<f32>;
@group(3) @binding(2) var light_strengths: texture_2d_array<f32>;
@group(3) @binding(3) var irradiance: texture_2d_array<f32>;
@group(3) @binding(4) var directional_irradiance: texture_2d_array<f32>;
struct BakedLight {
    // xyz where it is; w: 0 none, 1 a light, 2 a sun
    position: vec4<f32>,
    // a sun: towards it
    towards: vec4<f32>,
    color: vec4<f32>,
};
@group(3) @binding(5) var<storage, read> baked_lights: array<BakedLight>;
@group(3) @binding(6) var lightmap_sampler: sampler;

struct In {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) blend: vec4<f32>,
    @location(4) paint: vec4<f32>,
    @location(5) bone: vec4<u32>,
    @location(6) weight: vec4<f32>,
    @location(7) lightmap: vec2<f32>,
};

struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) blend: vec4<f32>,
    @location(4) paint: vec4<f32>,
    @location(5) lightmap: vec2<f32>,
};

@vertex
fn vs_main(v: In) -> Out {
    var o: Out;
    var position = vec4<f32>(v.position, 1.0);
    var normal = vec4<f32>(v.normal, 0.0);
    if (object.flags.x > 0.5 && dot(v.weight, vec4<f32>(1.0)) > 0.0) {
        let last = arrayLength(&bones) - 1u;
        let skin = bones[min(v.bone.x, last)] * v.weight.x + bones[min(v.bone.y, last)] * v.weight.y
            + bones[min(v.bone.z, last)] * v.weight.z + bones[min(v.bone.w, last)] * v.weight.w;
        position = skin * position;
        normal = skin * normal;
    }
    let world = object.model * position;
    o.clip = view.view_proj * world;
    o.world = world.xyz;
    o.normal = normalize((object.model * normal).xyz);
    o.uv = v.uv;
    o.blend = v.blend;
    o.paint = v.paint;
    o.lightmap = v.lightmap * lighting.lightmap.xy;
    return o;
}

// The shadow map: only what moves casts into it (a map's own shadows are
// in its lightmaps), seen from the sun.
@vertex
fn vs_shadow(v: In) -> @builtin(position) vec4<f32> {
    var position = vec4<f32>(v.position, 1.0);
    if (object.flags.x > 0.5 && dot(v.weight, vec4<f32>(1.0)) > 0.0) {
        let last = arrayLength(&bones) - 1u;
        let skin = bones[min(v.bone.x, last)] * v.weight.x + bones[min(v.bone.y, last)] * v.weight.y
            + bones[min(v.bone.z, last)] * v.weight.z + bones[min(v.bone.w, last)] * v.weight.w;
        position = skin * position;
    }
    return view.shadow * object.model * position;
}

// How much of the sun reaches a point (1: all): its shadow map, read a
// little off the surface along its normal, nine taps around it.
fn sun_shadow(world: vec3<f32>, n: vec3<f32>) -> f32 {
    if (view.shadow_params.x < 0.5) { return 1.0; }
    let p = view.shadow * vec4<f32>(world + n * view.shadow_params.z * 1.5, 1.0);
    let uv = vec2<f32>(p.x * 0.5 + 0.5, 0.5 - p.y * 0.5);
    if (any(uv < vec2<f32>(0.0)) || any(uv > vec2<f32>(1.0)) || p.z > 1.0 || p.z < 0.0) { return 1.0; }
    var lit = 0.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            lit += textureSampleCompareLevel(shadow_map, shadow_compare, uv + vec2<f32>(f32(x), f32(y)) * view.shadow_params.y, p.z);
        }
    }
    return lit / 9.0;
}

const PI: f32 = 3.14159265;

// the sky a surface sees in a direction, blurred as it gets rougher
fn sky(d: vec3<f32>, rough: f32) -> vec3<f32> {
    let up = clamp(d.z * mix(4.0, 1.0, rough), -1.0, 1.0) * 0.5 + 0.5;
    return mix(view.ground.rgb, view.sky.rgb, up);
}

// what RENDER_DEBUG shows of a light, brought into range (the frame's own
// tonemap passes debug views through as they are)
fn tonemap(x: vec3<f32>) -> vec3<f32> {
    return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0));
}

fn luminance(c: vec3<f32>) -> f32 { return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722)); }

fn ggx(n: vec3<f32>, h: vec3<f32>, rough: f32) -> f32 {
    let a = max(rough * rough, 0.002);
    let nh = max(dot(n, h), 0.0);
    let d = nh * nh * (a * a - 1.0) + 1.0;
    return a * a / (PI * d * d);
}

// A tangent-space normal as s&box packs it: hemi-octahedral (complex,
// blendable v1), y flipped as its tools make them.
fn hemi_oct(rg: vec2<f32>) -> vec3<f32> {
    let t = vec2<f32>(rg.x + rg.y - 1.00392163, rg.x - rg.y);
    let n = normalize(vec3<f32>(t, 1.0 - abs(t.x) - abs(t.y)));
    return vec3<f32>(n.x, -n.y, n.z);
}

// One of blendable's layers: its colour tinted where its mask says, its
// normal (tangent space), roughness, metalness, occlusion, reveal mask.
struct Layer {
    albedo: vec3<f32>,
    ts: vec3<f32>,
    rough: f32,
    metal: f32,
    ao: f32,
    mask: f32,
};

fn layer(color: vec4<f32>, normal: vec4<f32>, rma: vec4<f32>, tint: vec3<f32>) -> Layer {
    var o: Layer;
    o.albedo = mix(color.rgb, color.rgb * tint, color.a);
    if (material.mode.w >= 1.0) {
        o.ts = hemi_oct(normal.rg);
        o.rough = normal.b;
        o.metal = rma.r;
        o.ao = rma.g;
        o.mask = rma.b;
    } else {
        let n = normalize(normal.rgb * 2.0 - 1.0);
        o.ts = vec3<f32>(n.x, -n.y, n.z);
        o.rough = rma.r;
        o.metal = rma.g;
        o.ao = rma.b;
        o.mask = rma.a;
    }
    return o;
}

// How much of a layer shows: its painted amount against its reveal mask,
// as softly as the material says (blendable.shader's ComputeBlendWeight).
fn reveal(amount: f32, softness: f32, mask: f32) -> f32 {
    let s = max(softness, 0.01);
    let scaled = amount * (1.0 + s * 2.0) - s;
    return 1.0 - clamp((mask - (scaled - s)) / (s * 2.0), 0.0, 1.0);
}

fn mix_layer(a: Layer, b: Layer, k: f32) -> Layer {
    var o: Layer;
    o.albedo = mix(a.albedo, b.albedo, k);
    o.ts = normalize(mix(a.ts, b.ts, k));
    o.rough = mix(a.rough, b.rough, k);
    o.metal = mix(a.metal, b.metal, k);
    o.ao = mix(a.ao, b.ao, k);
    o.mask = mix(a.mask, b.mask, k);
    return o;
}

// A surface as a material's textures make it, to be lit.
struct Surface {
    base: vec3<f32>,
    // the normal in the surface's tangent space (flat without a normal map)
    ts: vec3<f32>,
    rough: f32,
    metal: f32,
    ao: f32,
};

// The surface's normal in the world: its tangent-space normal on the
// cotangent frame, from how position and uv change across the pixel.
fn frame(i: Out, front: bool, ts: vec3<f32>, mapped: bool, dp1: vec3<f32>, dp2: vec3<f32>, du1: vec2<f32>, du2: vec2<f32>) -> vec3<f32> {
    var n = normalize(i.normal);
    if (!front) { n = -n; }
    if (mapped) {
        let dp2perp = cross(dp2, n);
        let dp1perp = cross(n, dp1);
        let t = dp2perp * du1.x + dp1perp * du2.x;
        let b = dp2perp * du1.y + dp1perp * du2.y;
        let inv = inverseSqrt(max(max(dot(t, t), dot(b, b)), 1e-20));
        n = normalize(t * inv * ts.x + b * inv * ts.y + n * ts.z);
    }
    return n;
}

// How a surface is lit: the colour it sends back (before the tonemap),
// with the light that bounced and the direct light, for RENDER_DEBUG.
struct Lit {
    color: vec3<f32>,
    ambient: vec3<f32>,
    direct: vec3<f32>,
};

fn light(i: Out, n: vec3<f32>, s: Surface) -> Lit {
    let base = s.base;
    let rough = s.rough;
    let metal = s.metal;
    let ao = s.ao;
    let ts = s.ts;
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
    var ambient = sky(n, 1.0);
    var reflected = sky(reflect(-v, n), rough);
    let shade = sun_shadow(i.world, n);
    var direct = view.sun_color.rgb * ndl * shade;
    var direct_spec = view.sun_color.rgb * spec_sun * shade;
    let lit_by = object.flags.y;
    if (lit_by > 0.5) {
        var indices: vec4<u32>;
        var strengths: vec4<f32>;
        var bounced: vec3<f32>;
        if (lit_by < 1.5) {
            // a texel of the lightmaps
            let uv = i.lightmap;
            let texel = vec2<i32>(clamp(uv, vec2<f32>(0.0), vec2<f32>(0.99999)) * lighting.lightmap.zw);
            indices = vec4<u32>(textureLoad(light_indices, texel, 0, 0) * 255.0 + 0.5);
            strengths = textureSampleLevel(light_strengths, lightmap_sampler, uv, 0, 0.0);
            // the light that bounced: some from everywhere, the rest from one way
            let irr = textureSampleLevel(irradiance, lightmap_sampler, uv, 0, 0.0).rgb;
            let ahd = textureSampleLevel(directional_irradiance, lightmap_sampler, uv, 0, 0.0);
            let hxy = ahd.xy * 2.0 - 1.0;
            let highlight = vec3<f32>(hxy, 1.0 - sqrt(dot(hxy, hxy)));
            let even = clamp(ahd.z, 0.0, 1.0) * irr;
            let along = (irr - even) / max(highlight.z, 0.05);
            bounced = even + max(0.0, dot(highlight, ts)) * along;
        } else {
            // the probes around it: across the box 0..1, kept to their centres
            let dims = lighting.probes.xyz;
            let at = clamp((lighting.to_local * vec4<f32>(i.world, 1.0)).xyz, 0.5 / dims, 1.0 - 0.5 / dims);
            // between two layers of probes
            let z = at.z * dims.z - 0.5;
            let z0 = floor(z);
            let t = z - z0;
            let z1 = min(z0 + 1.0, dims.z - 1.0);
            let nz = i32(dims.z);
            var cube: array<vec3<f32>, 6>;
            for (var f = 0; f < 6; f++) {
                let a = textureSampleLevel(irradiance, lightmap_sampler, at.xy, f * nz + i32(z0), 0.0).rgb;
                let b = textureSampleLevel(irradiance, lightmap_sampler, at.xy, f * nz + i32(z1), 0.0).rgb;
                cube[f] = mix(a, b, t);
            }
            // each way the normal faces, as much as it faces it
            let n2 = n * n;
            bounced = n2.x * select(cube[0], cube[1], n.x < 0.0) + n2.y * select(cube[2], cube[3], n.y < 0.0) + n2.z * select(cube[4], cube[5], n.z < 0.0);
            let near = vec2<i32>(at.xy * dims.xy);
            indices = vec4<u32>(textureLoad(light_indices, near, i32(round(z)), 0) * 255.0 + 0.5);
            strengths = mix(textureSampleLevel(light_strengths, lightmap_sampler, at.xy, i32(z0), 0.0),
                textureSampleLevel(light_strengths, lightmap_sampler, at.xy, i32(z1), 0.0), t);
        }
        // the lights it names, as much of each as reaches it (stored as its root)
        strengths = strengths * strengths;
        direct = vec3<f32>(0.0);
        direct_spec = vec3<f32>(0.0);
        for (var j = 0; j < 4; j++) {
            let light = baked_lights[min(indices[j], 255u)];
            if (strengths[j] <= 0.0001 || light.position.w == 0.0) { continue; }
            var lj = normalize(light.position.xyz - i.world);
            if (light.position.w > 1.5) { lj = normalize(light.towards.xyz); }
            let hj = normalize(lj + v);
            let ndlj = max(dot(n, lj), 0.0);
            var c = light.color.rgb * strengths[j];
            // the sun: what moves shadows it too
            if (light.position.w > 1.5) { c = c * shade; }
            direct += c * ndlj;
            let visj = 1.0 / ((ndlj * (1.0 - k) + k) * (ndv * (1.0 - k) + k) * 4.0);
            direct_spec += c * ggx(n, hj, rough) * visj * ndlj;
        }
        // reflections as bright as the light there (s&box normalizes its
        // cubemaps to the irradiance so)
        reflected = reflected * min(luminance(bounced) / max(luminance(ambient), 0.001), 1.0);
        ambient = bounced;
    }
    var o: Lit;
    o.color = diffuse * (direct + ambient) * ao + fresnel * (direct_spec + reflected * ao);
    o.ambient = ambient;
    o.direct = direct;
    return o;
}

// What RENDER_DEBUG asks to see instead (0: nothing).
fn debug_view(s: Surface, n: vec3<f32>, l: Lit) -> vec4<f32> {
    let debug = view.ground.w;
    if (debug == 1.0) { return vec4<f32>(s.base, 1.0); }
    if (debug == 2.0) { return vec4<f32>(n * 0.5 + 0.5, 1.0); }
    if (debug == 3.0) { return vec4<f32>(vec3<f32>(s.ao), 1.0); }
    if (debug == 4.0) { return vec4<f32>(vec3<f32>(s.rough), 1.0); }
    if (debug == 5.0) { return vec4<f32>(tonemap(l.ambient), 1.0); }
    if (debug == 6.0) { return vec4<f32>(tonemap(l.direct), 1.0); }
    return vec4<f32>(0.0);
}

// Each decal whose box the pixel is in, laid into its surface before it's
// lit, as s&box's Decals::Resolve does: its texture times its tint, faded as
// the surface turns from its axis, then over the albedo, multiplied into it,
// or added as glow, as much as its colour mix and alpha say.
struct Decaled { base: vec3<f32>, glow: vec3<f32> };
fn resolve_decals(world: vec3<f32>, dp1: vec3<f32>, dp2: vec3<f32>, n: vec3<f32>, base: vec3<f32>, glow: vec3<f32>) -> Decaled {
    var o = Decaled(base, glow);
    for (var k = 0u; k < arrayLength(&decals); k = k + 1u) {
        let d = decals[k];
        if (d.params.w < 0.5) { continue; }
        let p = (d.to_decal * vec4<f32>(world, 1.0)).xyz;
        if (any(abs(p) > vec3<f32>(0.5))) { continue; }
        let uv = vec2<f32>(0.5) - p.yz;
        let gx = -(d.to_decal * vec4<f32>(dp1, 0.0)).yz;
        let gy = -(d.to_decal * vec4<f32>(dp2, 0.0)).yz;
        var c = textureSampleGrad(decal_array, decal_sampler, uv, i32(d.params.x), gx, gy) * d.tint;
        let facing = smoothstep(0.55, 1.0, clamp(dot(n, -d.axis.xyz), 0.55, 1.0));
        let att = mix(1.0, facing, d.axis.w);
        let mode = u32(d.params.z + 0.5);
        if (mode == 0u) { c = vec4<f32>(c.rgb * att, c.a); }
        c.a = c.a * att;
        let k_mix = d.params.y * c.a;
        if (mode == 1u) {
            o.base = o.base * mix(vec3<f32>(1.0), c.rgb, k_mix);
        } else if (mode == 2u) {
            o.glow = o.glow + c.rgb * k_mix;
        } else {
            o.base = mix(o.base, c.rgb, k_mix);
        }
    }
    return o;
}

@fragment
fn fs_main(i: Out, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    let layers = material.mode.x;
    let blendable = layers > 0.5;
    // every sample taken before any branch on it, as derivatives need
    var albedo = textureSample(color_a, samp, i.uv);
    let packed = textureSample(normal_a, samp, i.uv);
    let occ = textureSample(occlusion_a, samp, i.uv);
    let uv_b = i.uv * material.scale.xy;
    let uv_c = i.uv * material.scale.zw;
    let albedo_b = textureSample(color_b, samp, uv_b);
    let packed_b = textureSample(normal_b, samp, uv_b);
    let occ_b = textureSample(occlusion_b, samp, uv_b);
    let albedo_c = textureSample(color_c, samp, uv_c);
    let packed_c = textureSample(normal_c, samp, uv_c);
    let occ_c = textureSample(occlusion_c, samp, uv_c);
    // complex's tint mask, self-illumination mask and metalness are where
    // blendable has layer B's colour, normal and g_tRma
    let mask = albedo_b.r;
    let dp1 = dpdx(i.world);
    let dp2 = dpdy(i.world);
    let du1 = dpdx(i.uv);
    let du2 = dpdy(i.uv);

    var s: Surface;
    s.rough = 0.6;
    s.ts = vec3<f32>(0.0, 0.0, 1.0);
    var alpha = 1.0;
    var glow = vec3<f32>(0.0);
    if (blendable) {
        // a model without paint isn't painted black
        var paint = i.paint;
        if (paint.w == 0.0) { paint = vec4<f32>(1.0); }
        var tint = object.tint.rgb;
        if (material.mode.z > 0.5) { tint = tint * paint.rgb; }
        var m = layer(albedo, packed, occ, material.tint.rgb * tint);
        if (layers > 1.5 && i.blend.r > 0.0) {
            let b = layer(albedo_b, packed_b, occ_b, material.tint_b.rgb * tint);
            m = mix_layer(m, b, reveal(i.blend.r, material.blend.x, b.mask));
        }
        if (layers > 2.5 && i.blend.g > 0.0) {
            let c = layer(albedo_c, packed_c, occ_c, material.tint_c.rgb * tint);
            m = mix_layer(m, c, reveal(i.blend.g, material.blend.y, c.mask));
        }
        s.base = m.albedo;
        s.ts = m.ts;
        s.rough = m.rough;
        s.metal = m.metal;
        s.ao = m.ao;
    } else {
        if (ALPHA_TEST && material.params.x > 0.0 && albedo.a < material.params.x) {
            discard;
        }
        var tint = material.tint.rgb;
        if (material.mode.y > 0.5) {
            tint = mix(vec3<f32>(1.0), tint, mask);
        }
        if (material.mode.z > 0.5) {
            tint = tint * i.paint.rgb;
        }
        s.base = albedo.rgb * tint * object.tint.rgb;
        s.ao = occ.r;
        s.metal = material.params.y;
        if (material.params.z > 1.5) { s.metal = occ_b.g; } else if (material.params.z > 0.5) { s.metal = albedo.a; }
        if (material.params.w > 0.5) {
            s.ts = hemi_oct(packed.rg);
            s.rough = packed.b;
        }
        if (material.extra.x > 0.0) { alpha = clamp(albedo.a * material.extra.x, 0.0, 1.0); }
        if (material.extra.w > 0.5) {
            // its glow (tint_c: the tint, as bright as the material says;
            // its w how much of the colour it takes)
            let e = packed_b.rgb * material.tint_c.rgb;
            glow = mix(e, e * s.base, material.tint_c.w);
        }
    }
    s.rough = pow(s.rough, material.blend.z);
    let n = frame(i, front, s.ts, material.params.w > 0.5, dp1, dp2, du1, du2);
    let decaled = resolve_decals(i.world, dp1, dp2, n, s.base, glow);
    s.base = decaled.base;
    glow = decaled.glow;
    let l = light(i, n, s);
    let d = debug_view(s, n, l);
    if (d.a > 0.0) { return d; }
    return vec4<f32>(l.color + glow, alpha);
}

@fragment
fn fs_glass(i: Out, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    let color = textureSample(color_a, samp, i.uv);
    let packed = textureSample(normal_a, samp, i.uv);
    let rma = textureSample(occlusion_a, samp, i.uv);
    let dp1 = dpdx(i.world);
    let dp2 = dpdy(i.world);
    let du1 = dpdx(i.uv);
    let du2 = dpdy(i.uv);
    let nt = normalize(packed.rgb * 2.0 - 1.0);
    var s: Surface;
    s.ts = vec3<f32>(nt.x, -nt.y, nt.z);
    s.rough = pow(rma.r, material.blend.z);
    s.ao = rma.b;
    // glass is never metal
    s.metal = 0.0;
    let albedo = color.rgb * mix(vec3<f32>(1.0), material.tint.rgb * object.tint.rgb, packed.a);
    let n = frame(i, front, s.ts, true, dp1, dp2, du1, du2);
    if (material.extra.z > 0.5) {
        // cheap glass: lit as it is, over what's behind it
        s.base = albedo;
        let l = light(i, n, s);
        return vec4<f32>(l.color, color.a);
    }
    // what's behind, less what it reflects, through its colour; its own
    // colour only as much as it absorbs (glass.shader's AlbedoAbsorption)
    let absorbed = s.rough * material.extra.y;
    s.base = albedo * absorbed;
    let v = normalize(view.eye.xyz - i.world);
    let ndv = clamp(dot(n, v), 0.0, 1.0);
    let reflects = 0.04 + (max(1.0 - s.rough, 0.04) - 0.04) * pow(1.0 - ndv, 5.0);
    // blurred as rough as it is and as glancing the view (glass.shader reads
    // a mip of what's behind, sqrt(amount) * 7 down: here a disc of taps as
    // wide as that mip's texels)
    let amount = material.extra.x * s.rough / max(ndv, 0.05);
    let radius = exp2(sqrt(amount) * 7.0) - 1.0;
    let size = vec2<f32>(textureDimensions(scene));
    var behind = vec3<f32>(0.0);
    for (var t = 0; t < 16; t++) {
        // a golden-angle spiral filling the disc
        let r = radius * sqrt((f32(t) + 0.5) / 16.0);
        let a = f32(t) * 2.39996323;
        let at = clamp(i.clip.xy + r * vec2<f32>(cos(a), sin(a)), vec2<f32>(0.0), size - 1.0);
        behind += textureLoad(scene, vec2<i32>(at), 0).rgb;
    }
    behind = behind / 16.0;
    let through = behind * (1.0 - reflects) * albedo * (1.0 - absorbed);
    let l = light(i, n, s);
    let d = debug_view(s, n, l);
    if (d.a > 0.0) { return d; }
    return vec4<f32>(through + l.color, 1.0);
}
