// A shader graph's shader: what assets/shadergraph.rs writes for a
// `.shdrgrph` (graph_material, graph_offset and the GRAPH_ constants) goes
// after this, and this goes after shader.wgsl without its stock materials
// (what's marked <stock> there). The graph fills in a material; it's lit
// as complex is, by shader.wgsl's own light().
//
// The helpers are s&box's, ported: procedural.hlsl's noise, the graph
// editor's ShaderTemplate.cs (triplanar sampling, the blend modes), and
// common/utils/normal.hlsl (sbox-public, MIT).

// its parameters, a slot each (x, xy, xyz or all four; a bool's x), and
// its textures, a slot each, with a sampler that wraps and one that doesn't
struct GraphParams {
    v: array<vec4<f32>, 32>,
};
@group(1) @binding(0) var<uniform> graph: GraphParams;
@group(1) @binding(1) var graph_wrap: sampler;
@group(1) @binding(2) var graph_clamp: sampler;
@group(1) @binding(3) var graph_t0: texture_2d<f32>;
@group(1) @binding(4) var graph_t1: texture_2d<f32>;
@group(1) @binding(5) var graph_t2: texture_2d<f32>;
@group(1) @binding(6) var graph_t3: texture_2d<f32>;
@group(1) @binding(7) var graph_t4: texture_2d<f32>;
@group(1) @binding(8) var graph_t5: texture_2d<f32>;
@group(1) @binding(9) var graph_t6: texture_2d<f32>;
@group(1) @binding(10) var graph_t7: texture_2d<f32>;
@group(1) @binding(11) var graph_t8: texture_2d<f32>;
@group(1) @binding(12) var graph_t9: texture_2d<f32>;
@group(1) @binding(13) var graph_t10: texture_2d<f32>;
@group(1) @binding(14) var graph_t11: texture_2d<f32>;
@group(1) @binding(15) var graph_t12: texture_2d<f32>;
@group(1) @binding(16) var graph_t13: texture_2d<f32>;
@group(1) @binding(17) var graph_t14: texture_2d<f32>;
@group(1) @binding(18) var graph_t15: texture_2d<f32>;

// What a graph's nodes read of where they are (s&box's PixelInput, as far
// as the nodes use it).
struct GIn {
    uv: vec2<f32>,
    world: vec3<f32>,
    normal: vec3<f32>,
    tangent_u: vec3<f32>,
    tangent_v: vec3<f32>,
    local: vec3<f32>,
    local_normal: vec3<f32>,
    color: vec4<f32>,
    tint: vec4<f32>,
    time: f32,
    front: f32,
};

// What a graph fills in (s&box's Material, as far as the Result node sets
// it): its normal in tangent space.
struct GMat {
    albedo: vec3<f32>,
    emission: vec3<f32>,
    opacity: f32,
    normal: vec3<f32>,
    roughness: f32,
    metalness: f32,
    ao: f32,
};

// ShaderTemplate.Material_init
fn g_material() -> GMat {
    var m: GMat;
    m.albedo = vec3<f32>(1.0);
    m.emission = vec3<f32>(0.0);
    m.opacity = 1.0;
    m.normal = vec3<f32>(0.0, 0.0, 1.0);
    m.roughness = 1.0;
    m.metalness = 0.0;
    m.ao = 1.0;
    return m;
}

// A number a node carries, worked out when the shader runs and not while
// it compiles (shadergraph.rs, Val::float).
fn g_c1(x: f32) -> f32 { return x; }
fn g_c2(x: vec2<f32>) -> vec2<f32> { return x; }
fn g_c3(x: vec3<f32>) -> vec3<f32> { return x; }
fn g_c4(x: vec4<f32>) -> vec4<f32> { return x; }

fn g_tile_and_offset(uv: vec2<f32>, tile: vec2<f32>, offset: vec2<f32>) -> vec2<f32> {
    return uv * tile + offset;
}

fn g_decode_normal(v: vec3<f32>) -> vec3<f32> {
    return normalize(2.0 * v - 1.0);
}

fn g_ws_to_ts(v: vec3<f32>, n: vec3<f32>, tu: vec3<f32>, tv: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(dot(v, tu), dot(v, tv), dot(v, n));
}

// TransformNormal: tangent space to the world, its y turned over as
// s&box's is
fn g_transform_normal(ts: vec3<f32>, n: vec3<f32>, tu: vec3<f32>, tv: vec3<f32>) -> vec3<f32> {
    return normalize(ts.x * normalize(tu) - ts.y * normalize(tv) + ts.z * n);
}

fn g_object_dir(v: vec3<f32>) -> vec3<f32> {
    return normalize((object.model * vec4<f32>(v, 0.0)).xyz);
}

fn g_view_dir(world: vec3<f32>) -> vec3<f32> {
    return normalize(view.eye.xyz - world);
}

// ---- procedural.hlsl

fn g_fuzzy(uv: vec2<f32>, d: vec2<f32>) -> f32 {
    return fract(sin(dot(uv, d)) * 43758.5453);
}

fn g_fuzzy_noise(uv: vec2<f32>) -> f32 {
    return g_fuzzy(uv, vec2<f32>(12.9898, 78.233));
}

fn g_fuzzy_offset(uv: vec2<f32>, offset: f32) -> vec2<f32> {
    let n = vec2<f32>(g_fuzzy(uv, vec2<f32>(12.9898, 78.233)), g_fuzzy(uv, vec2<f32>(39.346, 11.135)));
    return vec2<f32>(sin(n.y * offset) * 0.5 + 0.5, cos(n.x * offset) * 0.5 + 0.5);
}

fn g_value_noise(uv: vec2<f32>) -> f32 {
    let cell = floor(uv);
    var f = fract(uv);
    f = f * f * (3.0 - 2.0 * f);
    let top = mix(g_fuzzy_noise(cell), g_fuzzy_noise(cell + vec2<f32>(1.0, 0.0)), f.x);
    let bottom = mix(g_fuzzy_noise(cell + vec2<f32>(0.0, 1.0)), g_fuzzy_noise(cell + vec2<f32>(1.0, 1.0)), f.x);
    return mix(top, bottom, f.y);
}

fn g_simplex_mod(x: vec3<f32>, m: f32) -> vec3<f32> {
    return x - floor(x * 0.0034602076124567) * m;
}

fn g_simplex_permute(x: vec3<f32>) -> vec3<f32> {
    return g_simplex_mod(x * x * 34.0 + x, 289.0);
}

fn g_simplex_2d(uv: vec2<f32>) -> f32 {
    let c = vec4<f32>(0.2113248654, 0.36602540378, -0.5773502692, 0.0243902439);
    var corner = floor(uv + dot(uv, c.yy));
    let x0 = uv - corner + dot(corner, c.xx);
    let i1 = select(vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 0.0), x0.x > x0.y);
    var x12 = x0.xyxy + c.xxzz;
    x12 = vec4<f32>(x12.xy - i1, x12.zw);
    corner = corner - floor(corner * 0.0034602076124567) * 289.0;
    let permutation = g_simplex_permute(g_simplex_permute(corner.y + vec3<f32>(0.0, i1.y, 1.0)) + corner.x + vec3<f32>(0.0, i1.x, 1.0));
    var contribution = max(vec3<f32>(0.5) - vec3<f32>(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3<f32>(0.0));
    contribution = contribution * contribution;
    contribution = contribution * contribution;
    let x = 2.0 * fract(permutation * c.www) - 1.0;
    let h = abs(x) - 0.5;
    let a0 = x - floor(x + 0.5);
    contribution = contribution * inverseSqrt(a0 * a0 + h * h);
    let gradient = vec3<f32>(a0.x * x0.x + h.x * x0.y, a0.yz * x12.xz + h.yz * x12.yw);
    return 130.0 * dot(contribution, gradient);
}

fn g_voronoi_noise(uv: vec2<f32>, angle: f32, density: f32) -> f32 {
    let g = floor(uv * density);
    let v = fract(uv * density);
    var nearest = 4.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let cell = vec2<f32>(f32(x), f32(y));
            nearest = min(nearest, distance(cell + g_fuzzy_offset(cell + g, angle), v));
        }
    }
    return nearest;
}

// ---- ShaderTemplate.cs: a texture laid on from three sides

fn g_triplanar_weights(n: vec3<f32>) -> vec3<f32> {
    let w = saturate(pow(abs(n), vec3<f32>(4.0)));
    return w / max(dot(w, vec3<f32>(1.0)), 0.0001);
}

fn g_triplanar_color(t: texture_2d<f32>, s: sampler, p: vec3<f32>, n: vec3<f32>) -> vec4<f32> {
    let w = g_triplanar_weights(n);
    let sign = select(vec3<f32>(1.0), vec3<f32>(-1.0), n < vec3<f32>(0.0));
    let x = textureSample(t, s, vec2<f32>(p.y * sign.x, p.z));
    let y = textureSample(t, s, vec2<f32>(p.x * sign.y, p.z));
    let z = textureSample(t, s, vec2<f32>(p.x * -sign.z, p.y));
    return x * w.x + y * w.y + z * w.z;
}

fn g_triplanar_color_lod(t: texture_2d<f32>, s: sampler, p: vec3<f32>, n: vec3<f32>) -> vec4<f32> {
    let w = g_triplanar_weights(n);
    let sign = select(vec3<f32>(1.0), vec3<f32>(-1.0), n < vec3<f32>(0.0));
    let x = textureSampleLevel(t, s, vec2<f32>(p.y * sign.x, p.z), 0.0);
    let y = textureSampleLevel(t, s, vec2<f32>(p.x * sign.y, p.z), 0.0);
    let z = textureSampleLevel(t, s, vec2<f32>(p.x * -sign.z, p.y), 0.0);
    return x * w.x + y * w.y + z * w.z;
}

fn g_triplanar_mix(tx: vec3<f32>, ty: vec3<f32>, tz: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    let w = g_triplanar_weights(n);
    let sign = select(vec3<f32>(1.0), vec3<f32>(-1.0), n < vec3<f32>(0.0));
    let nx = g_decode_normal(tx) * vec3<f32>(sign.x, 1.0, 1.0);
    let ny = g_decode_normal(ty) * vec3<f32>(sign.y, 1.0, 1.0);
    let nz = g_decode_normal(tz) * vec3<f32>(-sign.z, 1.0, 1.0);
    let bx = vec3<f32>(nx.xy + n.yz, n.x);
    let by = vec3<f32>(ny.xy + n.xz, n.y);
    let bz = vec3<f32>(nz.xy + n.xy, n.z);
    return normalize(bx.zxy * w.x + by.xzy * w.y + bz.xyz * w.z + n);
}

fn g_triplanar_normal(t: texture_2d<f32>, s: sampler, p: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    let sign = select(vec3<f32>(1.0), vec3<f32>(-1.0), n < vec3<f32>(0.0));
    let x = textureSample(t, s, vec2<f32>(p.y * sign.x, p.z)).xyz;
    let y = textureSample(t, s, vec2<f32>(p.x * sign.y, p.z)).xyz;
    let z = textureSample(t, s, vec2<f32>(p.x * -sign.z, p.y)).xyz;
    return g_triplanar_mix(x, y, z, n);
}

fn g_triplanar_normal_lod(t: texture_2d<f32>, s: sampler, p: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    let sign = select(vec3<f32>(1.0), vec3<f32>(-1.0), n < vec3<f32>(0.0));
    let x = textureSampleLevel(t, s, vec2<f32>(p.y * sign.x, p.z), 0.0).xyz;
    let y = textureSampleLevel(t, s, vec2<f32>(p.x * sign.y, p.z), 0.0).xyz;
    let z = textureSampleLevel(t, s, vec2<f32>(p.x * -sign.z, p.y), 0.0).xyz;
    return g_triplanar_mix(x, y, z, n);
}

// ---- ShaderTemplate.cs: the blend modes, a channel at a time

fn g_blend_colorburn(a: f32, b: f32) -> f32 {
    if (a >= 1.0) { return 1.0; }
    if (b <= 0.0) { return 0.0; }
    return 1.0 - saturate((1.0 - a) / b);
}

fn g_blend_linearburn(a: f32, b: f32) -> f32 {
    return max(0.0, a + b - 1.0);
}

fn g_blend_colordodge(a: f32, b: f32) -> f32 {
    if (a <= 0.0) { return 0.0; }
    if (b >= 1.0) { return 1.0; }
    return saturate(a / (1.0 - b));
}

fn g_blend_lineardodge(a: f32, b: f32) -> f32 {
    return min(1.0, a + b);
}

fn g_blend_overlay(a: f32, b: f32) -> f32 {
    if (a <= 0.5) { return 2.0 * a * b; }
    return 1.0 - 2.0 * (1.0 - a) * (1.0 - b);
}

// as written there (its first branch's `1.0f * 2.0f * b` and all)
fn g_blend_softlight(a: f32, b: f32) -> f32 {
    if (b <= 0.5) { return 2.0 * a * b + a * a * (1.0 * 2.0 * b); }
    return sqrt(a) * (2.0 * b - 1.0) + 2.0 * a * (1.0 - b);
}

fn g_blend_hardlight(a: f32, b: f32) -> f32 {
    if (b <= 0.5) { return 2.0 * a * b; }
    return 1.0 - 2.0 * (1.0 - a) * (1.0 - b);
}

fn g_blend_vividlight(a: f32, b: f32) -> f32 {
    if (b <= 0.5) { return g_blend_colorburn(a, b * 2.0); }
    return g_blend_colordodge(a, 2.0 * (b - 0.5));
}

fn g_blend_linearlight(a: f32, b: f32) -> f32 {
    if (b <= 0.5) { return max(0.0, a + b * 2.0 - 1.0); }
    return min(1.0, a + 2.0 * (b - 0.5));
}

fn g_blend_hardmix(a: f32, b: f32) -> f32 {
    if (a + b >= 1.0) { return 1.0; }
    return 0.0;
}

fn g_blend_divide(a: f32, b: f32) -> f32 {
    if (b > 0.0) { return saturate(a / b); }
    return 0.0;
}

// ---- the stages

// A vertex, moved by the graph if it moves its vertices (its Result node's
// PositionOffset, in the world).
@vertex
fn vs_graph(v: In) -> Out {
    var o = vertex(v);
    if (GRAPH_OFFSET) {
        var g: GIn;
        g.uv = o.uv;
        g.world = o.world;
        g.normal = o.normal;
        // no tangents on a vertex here: any two across its normal
        let across = select(vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(1.0, 0.0, 0.0), abs(o.normal.z) > 0.9);
        g.tangent_u = normalize(cross(across, o.normal));
        g.tangent_v = cross(o.normal, g.tangent_u);
        g.local = o.local;
        g.local_normal = o.local_normal;
        g.color = v.paint;
        g.tint = object.tint;
        g.time = view.shadow_params.w;
        g.front = 1.0;
        o.world = o.world + graph_offset(g);
        o.clip = view.view_proj * vec4<f32>(o.world, 1.0);
    }
    return o;
}

@fragment
fn fs_graph(i: Out, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    let dp1 = dpdx(i.world);
    let dp2 = dpdy(i.world);
    let du1 = dpdx(i.uv);
    let du2 = dpdy(i.uv);
    var n = normalize(i.normal);
    if (!front) { n = -n; }
    // the tangents frame() lays a normal map on, for the nodes that ask
    let dp2perp = cross(dp2, n);
    let dp1perp = cross(n, dp1);
    let t = dp2perp * du1.x + dp1perp * du2.x;
    let b = dp2perp * du1.y + dp1perp * du2.y;
    let inv = inverseSqrt(max(max(dot(t, t), dot(b, b)), 1e-20));

    var g: GIn;
    g.uv = i.uv;
    g.world = i.world;
    g.normal = n;
    g.tangent_u = t * inv;
    g.tangent_v = b * inv;
    g.local = i.local;
    g.local_normal = normalize(i.local_normal);
    // a model without paint isn't painted black
    g.color = select(i.paint, vec4<f32>(1.0), i.paint.w == 0.0);
    g.tint = object.tint;
    g.time = view.shadow_params.w;
    g.front = select(0.0, 1.0, front);
    let m = graph_material(g);

    // ShaderTemplate.Material_output, then lit as complex is
    if (GRAPH_BLEND == 1 && m.opacity < 0.5) { discard; }
    var alpha = 1.0;
    if (GRAPH_BLEND == 2) { alpha = saturate(m.opacity); }
    if (GRAPH_UNLIT) { return vec4<f32>(m.albedo, alpha); }
    var s: Surface;
    s.base = m.albedo;
    s.ts = m.normal;
    s.rough = saturate(m.roughness);
    s.metal = saturate(m.metalness);
    s.ao = saturate(m.ao);
    let lit_normal = frame(i, front, s.ts, true, dp1, dp2, du1, du2);
    let l = light(i, lit_normal, s);
    let d = debug_view(s, lit_normal, l);
    if (d.a > 0.0) { return d; }
    return vec4<f32>(l.color + m.emission, alpha);
}
