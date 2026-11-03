//! Local lights drawn as they move: point and spot lights that aren't in a
//! map's lightmaps (a `PointLight` or `SpotLight` component, a lamp spawned
//! at runtime, the physgun's glow), shaded per pixel as s&box shades its
//! dynamic lights (vr_lighting.fxc: a falloff `1 / (linear d + quadratic
//! d²) - bias`, truncated to nothing at its radius, and a spot's cone),
//! and the nearest few that cast shadows given shadow maps: six faces of a
//! cube for a point light, one for a spot, in one depth array that
//! everything opaque within reach is drawn into.
use glam::{Mat4, Vec3};

/// A light as a frame lights with it.
#[derive(Clone, Debug)]
pub struct LocalLight {
    pub position: Vec3,
    /// the way a spot light shines
    pub forward: Vec3,
    /// its colour, as bright as it is (linear)
    pub color: Vec3,
    /// how far it reaches
    pub radius: f32,
    /// constant, linear and quadratic attenuation, as native is given them
    pub attenuation: [f32; 3],
    /// a spot light's inner and outer cone, half-angles in degrees
    pub cone: Option<(f32, f32)>,
    pub shadows: bool,
}

/// How many lights a frame shades with, and how many of their shadow faces.
pub const MAX_LIGHTS: usize = 32;
pub const SHADOW_LAYERS: u32 = 24;
pub const SHADOW_SIZE: u32 = 512;

impl LocalLight {
    /// How much of it reaches `d` away: 1 / (c + l d + q d²) (at least a
    /// unit away), less what's left of that at its radius, so it ends there.
    pub fn falloff(&self, d: f32) -> f32 {
        let [c, l, q] = self.attenuation();
        let f = |d: f32| { let d = d.max(1.0); 1.0 / (c + l * d + q * d * d).max(1e-6) };
        (f(d) - f(self.radius)).clamp(0.0, 1.0)
    }

    /// Its attenuation, a quadratic one if it was given none at all.
    fn attenuation(&self) -> [f32; 3] {
        let [c, l, q] = self.attenuation;
        if c <= 0.0 && l <= 0.0 && q <= 0.0 { [0.0, 0.0, 1e-4] } else { [c.max(0.0), l.max(0.0), q.max(0.0)] }
    }

    /// The views its shadow is drawn from: a cube's six faces (+x -x +y -y
    /// +z -z, as the shader picks them) for a point light, one for a spot.
    pub fn faces(&self) -> Vec<Mat4> {
        let near = (self.radius * 0.005).clamp(0.5, 4.0);
        let far = self.radius.max(near + 1.0);
        let look = |dir: Vec3| {
            let up = if dir.z.abs() > 0.99 { Vec3::Y } else { Vec3::Z };
            Mat4::look_at_rh(self.position, self.position + dir, up)
        };
        match self.cone {
            Some((_, outer)) => {
                let fov = (outer * 2.0 + 4.0).clamp(4.0, 170.0).to_radians();
                vec![Mat4::perspective_rh(fov, 1.0, near, far) * look(self.forward.normalize_or(Vec3::X))]
            }
            None => [Vec3::X, -Vec3::X, Vec3::Y, -Vec3::Y, Vec3::Z, -Vec3::Z].iter()
                .map(|&d| Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, near, far) * look(d)).collect(),
        }
    }
}

/// One light as the shader reads it (shader.wgsl's LocalLight).
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightGpu {
    /// xyz where it is; w its radius (0: none)
    position: [f32; 4],
    /// xyz the way it shines; w: 1 a spot light
    forward: [f32; 4],
    /// rgb its colour; w its first shadow face, or -1 for none
    color: [f32; 4],
    /// constant, linear, quadratic, and what's left at its radius
    falloff: [f32; 4],
    /// a spot's cosines: inner, outer, 1 / (inner - outer)
    cone: [f32; 4],
}

impl LightGpu {
    pub fn position_xyz(&self) -> [f32; 3] { [self.position[0], self.position[1], self.position[2]] }
    pub fn radius(&self) -> f32 { self.position[3] }
}

/// What a frame lights with: the lights nearest it (by how near their reach
/// comes), as the shader reads them; and the shadow faces, each the light it
/// belongs to and the view it's drawn from, in the order of their layers.
pub fn pack(lights: &[LocalLight], eye: Vec3) -> (Vec<LightGpu>, Vec<(usize, Mat4)>) {
    let mut order: Vec<usize> = (0..lights.len()).filter(|&i| lights[i].radius > 0.0 && lights[i].color.max_element() > 0.0).collect();
    order.sort_by(|&a, &b| {
        let near = |l: &LocalLight| l.position.distance(eye) - l.radius;
        near(&lights[a]).total_cmp(&near(&lights[b]))
    });
    order.truncate(MAX_LIGHTS);
    let mut faces = Vec::new();
    let mut gpu = Vec::new();
    for &i in &order {
        let l = &lights[i];
        let mut first = -1.0;
        if l.shadows {
            let f = l.faces();
            if faces.len() + f.len() <= SHADOW_LAYERS as usize {
                first = faces.len() as f32;
                faces.extend(f.into_iter().map(|m| (gpu.len(), m)));
            }
        }
        let [c, li, q] = l.attenuation();
        let d = l.radius.max(1.0);
        let left = 1.0 / (c + li * d + q * d * d).max(1e-6);
        let (inner, outer) = l.cone.map(|(i, o)| (i.min(o).to_radians().cos(), o.max(i).to_radians().cos())).unwrap_or((0.0, 0.0));
        gpu.push(LightGpu {
            position: l.position.extend(l.radius).into(),
            forward: l.forward.normalize_or(Vec3::X).extend(l.cone.is_some() as i32 as f32).into(),
            color: l.color.extend(first).into(),
            falloff: [c, li, q, left],
            cone: [inner, outer, 1.0 / (inner - outer).max(1e-4), 0.0],
        });
    }
    (gpu, faces)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn light(cone: Option<(f32, f32)>) -> LocalLight {
        LocalLight { position: Vec3::ZERO, forward: Vec3::X, color: Vec3::ONE, radius: 400.0, attenuation: [0.0, 0.0, 1e-4], cone, shadows: true }
    }

    #[test]
    fn a_light_falls_off_to_nothing_at_its_radius() {
        // a point light's defaults: quadratic 1 (1e-4 to native), 400 across
        let l = light(None);
        assert_eq!(l.falloff(50.0), 1.0, "full near it");
        assert!((l.falloff(200.0) - (0.25 - 0.0625)).abs() < 1e-5, "a quarter at twice the distance it's full to, less what's left at its edge");
        assert_eq!(l.falloff(400.0), 0.0);
        assert_eq!(l.falloff(600.0), 0.0);
    }

    #[test]
    fn shadows_go_to_the_nearest_while_there_are_faces() {
        // five point lights 1000 apart (six faces each) and a spot (one) at 500: by
        // how near their reach comes, the first point, the spot, then the other points,
        // two of which still fit in the 24 layers
        let mut lights: Vec<LocalLight> = (0..5).map(|k| LocalLight { position: Vec3::new(k as f32 * 1000.0, 0.0, 0.0), ..light(None) }).collect();
        lights.push(LocalLight { position: Vec3::new(500.0, 0.0, 0.0), ..light(Some((20.0, 30.0))) });
        let (gpu, faces) = pack(&lights, Vec3::ZERO);
        assert_eq!(gpu.len(), 6);
        assert_eq!(faces.len(), 6 + 1 + 6 + 6);
        let firsts: Vec<f32> = gpu.iter().map(|g| g.color[3]).collect();
        assert_eq!(firsts, vec![0.0, 6.0, 7.0, 13.0, -1.0, -1.0], "the last two don't fit");
        assert_eq!(gpu[1].forward[3], 1.0, "the second is the spot");
        // a cube face looks along its axis: what's 100 along +x is in the middle of face 0
        let p = faces[0].1 * Vec3::new(100.0, 0.0, 0.0).extend(1.0);
        assert!((p.x / p.w).abs() < 1e-5 && (p.y / p.w).abs() < 1e-5 && p.z / p.w > 0.0 && p.z / p.w < 1.0);
    }

    /// A square of floor, built at runtime as an editor mesh is.
    fn square(r: &crate::render::Renderer, name: &str, half: f32, z: f32) -> crate::render::Model {
        use crate::rendermesh as rm;
        let l = rm::new_layout(32);
        for (s, f, o) in [("position", 6, 0), ("normal", 6, 12), ("texcoord", 16, 24)] { rm::add_field(l, s, 0, f, o); }
        let v: Vec<f32> = [[-half, -half], [half, -half], [half, half], [-half, half]].iter().flat_map(|p| [p[0], p[1], z, 0.0, 0.0, 1.0, 0.0, 0.0]).collect();
        let vb = unsafe { rm::new_vertices(4, &rm::layout_of(l).unwrap(), v.as_ptr() as usize) };
        let idx: [i32; 6] = [0, 1, 2, 0, 2, 3];
        let ib = unsafe { rm::new_indices(6, true, idx.as_ptr() as usize, 6) };
        let mesh = rm::new_mesh(0, rm::TRIANGLES, name);
        unsafe { rm::give(mesh, vb, 0, 0); rm::give(mesh, ib, 0, 0); }
        let meshes = rm::for_drawing(&[mesh], &[], |_| String::new());
        r.model_of(&crate::render::Content(Vec::new(), Vec::new()), name, &meshes, u64::MAX, &[]).unwrap()
    }

    /// A floor 400 square seen from 300 above, no sun, and a point light 100
    /// over its middle: brightest under the light, falling off away from it.
    /// A plate 24 square at half the light's height, 20 to one side, throws a
    /// shadow twice its size beside it, darker than the floor was there; with
    /// the light's shadows off, nothing is darker.
    #[test]
    fn a_point_light_lights_the_floor_and_what_is_in_the_way_shadows_it() {
        let Ok(r) = crate::render::Renderer::new() else { eprintln!("no adapter: skipped"); return };
        let floor = square(&r, "light floor", 200.0, 0.0);
        let plate = square(&r, "light plate", 12.0, 0.0);
        fn at(m: &crate::render::Model, t: Mat4) -> crate::render::Instance<'_> {
            crate::render::Instance { model: m, transform: t, tint: [1.0; 4], bones: Vec::new(), lightmapped: false, origin: None, mask: None, skin: 0 }
        }
        let draw = |with_plate: bool, shadows: bool| {
            let mut view = crate::render::View::camera(Vec3::new(0.0, 0.0, 300.0), [90.0, 0.0, 0.0], 60.0, 1.0, 1000.0, 1.0);
            view.sun = Some((Vec3::Z, Vec3::ZERO));
            view.exposure = crate::render::Exposure::Fixed(1.0);
            view.lights = vec![LocalLight { position: Vec3::new(0.0, 0.0, 100.0), forward: Vec3::X, color: Vec3::splat(4.0), radius: 400.0, attenuation: [0.0, 0.0, 1e-4], cone: None, shadows }];
            let mut objects = vec![at(&floor, Mat4::IDENTITY)];
            if with_plate { objects.push(at(&plate, Mat4::from_translation(Vec3::new(20.0, 0.0, 50.0)))); }
            r.draw(&objects, &view, 64, 64)
        };
        let lum = |img: &[u8], i: usize| img[i * 4] as i32 + img[i * 4 + 1] as i32 + img[i * 4 + 2] as i32;
        let bare = draw(false, true);
        let (middle, corner) = (lum(&bare, 32 * 64 + 32), lum(&bare, 2 * 64 + 2));
        eprintln!("middle {middle}, corner {corner}");
        assert!(middle > corner + 60, "brightest under the light: {middle} vs {corner}");
        let shadowed = draw(true, true);
        let darker: Vec<usize> = (0..64 * 64).filter(|&i| lum(&shadowed, i) < lum(&bare, i) - 30).collect();
        eprintln!("{} pixels in its shadow", darker.len());
        // the shadow, 48 square on the floor, is about 9 pixels across (5.4 units each), part of it under the plate itself
        assert!(darker.len() > 20 && darker.len() < 120, "{}", darker.len());
        let unshadowed = draw(true, false);
        assert_eq!((0..64 * 64).filter(|&i| lum(&unshadowed, i) < lum(&bare, i) - 30).count(), 0, "no shadow when it casts none");
    }
}
