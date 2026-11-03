// Sprites, as s&box draws a batch of them (sprite_ps.shader, public in
// sbox-public's game/core/shaders/sprite): an instanced quad per sprite,
// each sprite read from a structured buffer of SpriteData, here as words
// (37 a sprite, as the structured buffer packs them). Billboarded to the
// camera (always, or turning about z only) or turned as the sprite is;
// coloured by its texture, its tint and overlay (RGB stored around 128 with
// a shared exponent, as Color.ToRgbe packs them); cut off below its alpha
// cutoff; faded near the camera as asked. Drawn in the scene's linear HDR,
// over what's opaque and translucent, blended or added.
//
// Not yet: a texture's sheet of frames (Sheet::Blended), soft edges against
// what's behind (DepthFeather), fog, refraction, text sprites, and lighting
// as s&box's is: a lit sprite here takes the sky's ambient and half the sun.

struct View {
    view_proj: mat4x4<f32>,
    eye: vec4<f32>,
    // the camera's right, up and forward, world space
    right: vec4<f32>,
    up: vec4<f32>,
    forward: vec4<f32>,
    sun_color: vec4<f32>,
    ambient: vec4<f32>,
};

@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<storage, read> sprites: array<u32>;
@group(0) @binding(2) var linear_clamp: sampler;
@group(0) @binding(3) var point_clamp: sampler;
@group(1) @binding(0) var t0: texture_2d<f32>;
@group(1) @binding(1) var t1: texture_2d<f32>;
@group(1) @binding(2) var t2: texture_2d<f32>;
@group(1) @binding(3) var t3: texture_2d<f32>;
@group(1) @binding(4) var t4: texture_2d<f32>;
@group(1) @binding(5) var t5: texture_2d<f32>;
@group(1) @binding(6) var t6: texture_2d<f32>;
@group(1) @binding(7) var t7: texture_2d<f32>;

const WORDS: u32 = 37u;
const POSITION: u32 = 0u;
const ROTATION: u32 = 3u;
const SCALE: u32 = 6u;
const TINT: u32 = 8u;
const OVERLAY: u32 = 9u;
// rewritten for this draw: the texture's slot among the eight bound
const TEXTURE: u32 = 10u;
const FLAGS: u32 = 11u;
const BILLBOARD: u32 = 12u;
const FOG_CUTOUT: u32 = 13u;
const LIGHTING: u32 = 14u;
// rewritten for this draw: 1 to sample point-filtered, 0 bilinear
const SAMPLER: u32 = 16u;
const OFFSET: u32 = 32u;
const CAMERA_FADE: u32 = 34u;

const FLIP_X: u32 = 2u;
const FLIP_Y: u32 = 4u;

fn w(i: u32, k: u32) -> u32 { return sprites[i * WORDS + k]; }
fn f(i: u32, k: u32) -> f32 { return bitcast<f32>(w(i, k)); }
fn v2(i: u32, k: u32) -> vec2<f32> { return vec2<f32>(f(i, k), f(i, k + 1u)); }
fn v3(i: u32, k: u32) -> vec3<f32> { return vec3<f32>(f(i, k), f(i, k + 1u), f(i, k + 2u)); }

fn unpack_color(c: u32) -> vec4<f32> {
    return vec4<f32>(f32(c & 0xffu), f32((c >> 8u) & 0xffu), f32((c >> 16u) & 0xffu), f32(c >> 24u)) / 255.0;
}

// a vector turned about a unit axis by an angle in degrees
fn turn(v: vec3<f32>, axis: vec3<f32>, degrees: f32) -> vec3<f32> {
    let a = radians(degrees);
    let c = cos(a);
    let s = sin(a);
    return v * c + cross(axis, v) * s + axis * dot(axis, v) * (1.0 - c);
}

// Source's angles (pitch, yaw, roll in degrees) as forward, left and up
fn basis(angles: vec3<f32>) -> mat3x3<f32> {
    let p = radians(angles.x);
    let y = radians(angles.y);
    let r = radians(angles.z);
    let forward = vec3<f32>(cos(p) * cos(y), cos(p) * sin(y), -sin(p));
    let left0 = vec3<f32>(-sin(y), cos(y), 0.0);
    let up0 = cross(forward, left0);
    let left = left0 * cos(r) + up0 * sin(r);
    let up = up0 * cos(r) - left0 * sin(r);
    return mat3x3<f32>(forward, left, up);
}

struct Out {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) world: vec3<f32>,
    @location(2) @interpolate(flat) sprite: u32,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, @builtin(instance_index) sprite: u32) -> Out {
    // the batch's quad: corners 10 from its middle, uv 0,0 at -10,-10; two triangles
    var corners = array<vec2<f32>, 4>(vec2<f32>(-10.0, -10.0), vec2<f32>(10.0, -10.0), vec2<f32>(10.0, 10.0), vec2<f32>(-10.0, 10.0));
    var uvs = array<vec2<f32>, 4>(vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 1.0));
    var order = array<u32, 6>(0u, 1u, 2u, 0u, 2u, 3u);
    let corner = order[vi];
    let local = corners[corner];

    let position = v3(sprite, POSITION);
    let rotation = v3(sprite, ROTATION);
    var scale = vec3<f32>(v2(sprite, SCALE), 1.0) / 10.0;
    let offset = v2(sprite, OFFSET);
    let mode = w(sprite, BILLBOARD);

    var x_axis: vec3<f32>;
    var y_axis: vec3<f32>;
    var origin: vec3<f32>;
    if (mode == 2u) {
        // turned as the sprite is
        let m = basis(rotation);
        x_axis = m[1] * scale.x;
        y_axis = m[2] * -scale.y;
        origin = position + (offset.x - 0.5) * m[1] * scale.x * 20.0 + (offset.y - 0.5) * m[2] * scale.y * 20.0;
    } else {
        var forward = view.forward.xyz;
        var up: vec3<f32>;
        var right: vec3<f32>;
        if (mode == 1u) {
            // turning about z only: forward flattened, up the world's
            scale.y = -scale.y;
            forward.z = 0.0;
            let l = length(forward);
            forward = select(vec3<f32>(1.0, 0.0, 0.0), forward / l, l >= 0.001);
            up = vec3<f32>(0.0, 0.0, 1.0);
            right = normalize(cross(up, -forward));
        } else {
            up = -view.up.xyz;
            right = normalize(cross(up, forward));
            up = normalize(cross(forward, right));
        }
        if (rotation.z != 0.0) {
            right = turn(right, forward, rotation.z);
            up = turn(up, forward, rotation.z);
        }
        x_axis = right * scale.x;
        y_axis = up * scale.y;
        origin = position + (offset.x - 0.5) * -right * scale.x * 20.0 + (offset.y - 0.5) * -up * scale.y * 20.0;
    }
    let world = origin + x_axis * local.x + y_axis * local.y;

    var o: Out;
    o.position = view.view_proj * vec4<f32>(world, 1.0);
    o.uv = uvs[corner];
    o.world = world;
    o.sprite = sprite;
    return o;
}

fn sample_slot(slot: u32, nearest: bool, uv: vec2<f32>, dx: vec2<f32>, dy: vec2<f32>) -> vec4<f32> {
    if (nearest) {
        switch slot {
            case 0u: { return textureSampleGrad(t0, point_clamp, uv, dx, dy); }
            case 1u: { return textureSampleGrad(t1, point_clamp, uv, dx, dy); }
            case 2u: { return textureSampleGrad(t2, point_clamp, uv, dx, dy); }
            case 3u: { return textureSampleGrad(t3, point_clamp, uv, dx, dy); }
            case 4u: { return textureSampleGrad(t4, point_clamp, uv, dx, dy); }
            case 5u: { return textureSampleGrad(t5, point_clamp, uv, dx, dy); }
            case 6u: { return textureSampleGrad(t6, point_clamp, uv, dx, dy); }
            default: { return textureSampleGrad(t7, point_clamp, uv, dx, dy); }
        }
    }
    switch slot {
        case 0u: { return textureSampleGrad(t0, linear_clamp, uv, dx, dy); }
        case 1u: { return textureSampleGrad(t1, linear_clamp, uv, dx, dy); }
        case 2u: { return textureSampleGrad(t2, linear_clamp, uv, dx, dy); }
        case 3u: { return textureSampleGrad(t3, linear_clamp, uv, dx, dy); }
        case 4u: { return textureSampleGrad(t4, linear_clamp, uv, dx, dy); }
        case 5u: { return textureSampleGrad(t5, linear_clamp, uv, dx, dy); }
        case 6u: { return textureSampleGrad(t6, linear_clamp, uv, dx, dy); }
        default: { return textureSampleGrad(t7, linear_clamp, uv, dx, dy); }
    }
}

@fragment
fn fs_main(in: Out) -> @location(0) vec4<f32> {
    let i = in.sprite;
    let flags = w(i, FLAGS);
    var uv = in.uv;
    if ((flags & FLIP_X) != 0u) { uv.x = 1.0 - uv.x; }
    if ((flags & FLIP_Y) != 0u) { uv.y = 1.0 - uv.y; }
    let dx = dpdx(uv);
    let dy = dpdy(uv);
    var col = sample_slot(w(i, TEXTURE), w(i, SAMPLER) == 1u, uv, dx, dy);

    // tint and overlay: RGB around 128 times 2^(exponent - 135), alpha as a byte
    let lighting = w(i, LIGHTING);
    let exponent = f32((lighting >> 16u) & 0xffu);
    var tint = unpack_color(w(i, TINT));
    var overlay = unpack_color(w(i, OVERLAY));
    let e = exp2(exponent - 128.0 - 7.0);
    tint = vec4<f32>(select((tint.rgb * 255.0 - 128.0) * e, vec3<f32>(0.0), exponent == 0.0), tint.a);
    overlay = vec4<f32>((overlay.rgb * 255.0 - 128.0) * e, overlay.a);
    if (overlay.a > 0.0) { col = vec4<f32>(mix(col.rgb, overlay.rgb, overlay.a), col.a); }

    // the cutoff on the texture's own alpha, before the tint's
    let cutoff = f32(w(i, FOG_CUTOUT) >> 16u) / 65535.0;
    if (col.a < cutoff) { discard; }
    col = col * tint;

    // faded from near to far of the camera, if it asks
    let fade = unpack2x16float(w(i, CAMERA_FADE));
    if (fade.y > fade.x) {
        col.a = col.a * clamp((distance(in.world, view.eye.xyz) - fade.x) / (fade.y - fade.x), 0.0, 1.0);
    }

    if ((lighting & 0xffu) != 0u) {
        col = vec4<f32>(col.rgb * (view.ambient.rgb + view.sun_color.rgb * 0.5), col.a);
    }
    return col;
}
