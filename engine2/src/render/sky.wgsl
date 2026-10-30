// sky.shader, as near as we get yet: its cube texture (HDR, BC6H as s&box
// compiles it) seen along each pixel's view ray, brightened by the
// material's exposure bias, behind everything.

struct View {
    view_proj: mat4x4<f32>,
    inv_view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    sun: vec4<f32>,
    sun_color: vec4<f32>,
    sky: vec4<f32>,
    ground: vec4<f32>,
    // world to the sky's own space (its rotation, undone)
    sky_rotation: mat4x4<f32>,
};

struct Sky {
    // x: brightness
    params: vec4<f32>,
};

@group(0) @binding(0) var<uniform> view: View;
@group(1) @binding(0) var<uniform> sky: Sky;
@group(1) @binding(1) var cube: texture_cube<f32>;
@group(1) @binding(2) var samp: sampler;

struct Out {
    @builtin(position) clip: vec4<f32>,
    @location(0) ndc: vec2<f32>,
};

// one triangle that covers the screen
@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> Out {
    let p = vec2<f32>(f32((i << 1u) & 2u), f32(i & 2u)) * 2.0 - 1.0;
    var o: Out;
    o.clip = vec4<f32>(p, 0.0, 1.0);
    o.ndc = p;
    return o;
}

@fragment
fn fs_main(i: Out) -> @location(0) vec4<f32> {
    let far = view.inv_view_proj * vec4<f32>(i.ndc, 1.0, 1.0);
    let d = normalize(far.xyz / far.w - view.eye.xyz);
    // a cube is kept in Source 2's own axes (z up): looked up as it is, in
    // the sky's space
    let dir = (view.sky_rotation * vec4<f32>(d, 0.0)).xyz;
    let c = textureSample(cube, samp, dir).rgb * sky.params.x * view.eye.w;
    return vec4<f32>(clamp((c * (2.51 * c + 0.03)) / (c * (2.43 * c + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
