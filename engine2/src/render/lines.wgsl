// Lines, as s&box draws them (line.shader, public in sbox-public's
// game/core/shaders): the triangles line_cs made, coloured by the
// material's colour texture times the vertex colour (stored as gamma), cut
// where that leaves nothing. Lit (D_ENABLE_LIGHTING) as near as we light
// anything without s&box's lighting: the sky's ambient and the sun on the
// side it faces. Drawn in the scene's linear HDR: added, blended, or opaque.
//
// Not yet: soft edges against what's behind (g_DepthFeather), fog, the
// material's normal, roughness and metalness, wireframe.

struct View {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    // towards the sun, and its colour
    sun: vec4<f32>,
    sun_color: vec4<f32>,
    ambient: vec4<f32>,
};

@group(0) @binding(0) var<uniform> view: View;
@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;
// x: lit
@group(1) @binding(2) var<uniform> params: vec4<f32>;

struct In {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec3<f32>,
    @location(3) color: vec4<f32>,
    @location(4) uv: vec2<f32>,
};

struct Out {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vs_main(v: In) -> Out {
    var o: Out;
    o.position = view.view_proj * vec4<f32>(v.position, 1.0);
    o.normal = v.normal;
    o.color = v.color;
    o.uv = v.uv;
    return o;
}

fn gamma_to_linear(c: vec3<f32>) -> vec3<f32> {
    return select(pow((c + 0.055) / 1.055, vec3<f32>(2.4)), c / 12.92, c <= vec3<f32>(0.04045));
}

@fragment
fn fs_main(in: Out, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // the back face's normal turned to face the viewer, as the shader does
    let n = normalize(in.normal) * select(-1.0, 1.0, front);
    var col = textureSample(color_texture, color_sampler, in.uv) * vec4<f32>(gamma_to_linear(in.color.rgb), in.color.a);
    if (col.a - 0.0001 < 0.0) { discard; }
    if (params.x != 0.0) {
        let lit = view.ambient.rgb + view.sun_color.rgb * max(dot(n, view.sun.xyz), 0.0);
        col = vec4<f32>(col.rgb * lit, col.a);
    }
    return col;
}
