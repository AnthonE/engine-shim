//! Joints: s&box's (Box3D's) nine kinds, each one of Rapier's generic
//! joints. A joint keeps what managed told it, in s&box's terms; every
//! change builds the Rapier joint afresh from that (`build`).
//!
//! Box3D's axes: a hinge turns about its frame's z, a slider and a wheel's
//! suspension run along its x, a wheel spins about its z and steers about
//! its x, a ball twists about its z. Rapier frees its frame's x first, so a
//! kind whose main axis is z has its frames turned z onto x (`turn`).
//! Springs are given as frequency and damping ratio, and are acceleration
//! springs (mass doesn't change them), as Box3D's are.
use super::*;

/// A velocity motor's gain. Rapier's motor is a soft constraint, its
/// softness 1/(dt·gain): this high, it holds its speed as a rigid motor
/// does, up to its maximum force, as Box3D's motors do.
const MOTOR_GAIN: f32 = 1.0e6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Weld,
    Spring,
    Revolute,
    Prismatic,
    Spherical,
    Motor,
    Wheel,
    Parallel,
    Filter,
}

impl Kind {
    /// PhysicsJointType, as managed numbers it.
    pub fn native(self) -> i64 {
        match self {
            Kind::Filter => 0,
            Kind::Spherical => 1,
            Kind::Prismatic => 2,
            Kind::Revolute => 3,
            Kind::Motor => 5,
            Kind::Weld => 6,
            Kind::Spring => 7,
            Kind::Parallel => 15,
            Kind::Wheel => 16,
        }
    }

    /// From s&box's frame to Rapier's: z onto x for the kinds that turn about z.
    fn turn(self) -> Rotation {
        match self {
            Kind::Revolute | Kind::Spherical | Kind::Parallel => Rotation::from_rotation_y(-core::f32::consts::FRAC_PI_2),
            _ => Rotation::IDENTITY,
        }
    }
}

/// What drives a hinge's turning or a slider's sliding: a spring to a
/// target, a motor to a speed, or friction (a motor to standing still).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Drive {
    None,
    /// target, frequency, damping ratio
    Spring(f32, f32, f32),
    /// speed, maximum force or torque
    Velocity(f32, f32),
    /// the friction coefficient
    Friction(f32),
}

pub struct Joint {
    pub handle: ImpulseJointHandle,
    pub kind: Kind,
    pub body1: usize,
    pub body2: usize,
    pub frame1: Pose,
    pub frame2: Pose,
    pub collide: bool,
    /// PhysicsSpring as managed passes it: frequency, damping, maximum (a
    /// spring joint's rest length; a weld's most force)
    pub linear_spring: Vector,
    pub angular_spring: Vector,
    /// a spring's length limits; a hinge's angles (radians); a slider's travel
    pub min: f32,
    pub max: f32,
    pub limited: bool,
    pub min_force: f32,
    pub max_force: f32,
    pub drive: Drive,
    pub swing: f32,
    pub swing_limited: bool,
    pub twist_limited: bool,
    /// break thresholds: a force, a torque
    pub max_linear: f32,
    pub max_angular: f32,
    pub broken: bool,
    /// a ball's target rotation (rotation, frequency, damping), or its motor (velocity, torque)
    pub ball_target: Option<(Rotation, f32, f32)>,
    pub ball_motor: Option<(Vector, f32)>,
    pub motor: MotorJoint,
    pub wheel: Wheel,
    /// an upright joint's frequency, damping ratio, most torque
    pub parallel: (f32, f32, f32),
    /// a spring that only pulls or only pushes is on (false: slack)
    pub spring_on: bool,
    /// the spring's rest length when managed never said
    pub initial_length: f32,
}

#[derive(Clone, Copy, Default)]
pub struct MotorJoint {
    pub linear_velocity: Vector,
    pub angular_velocity: Vector,
    pub max_velocity_force: f32,
    pub max_velocity_torque: f32,
    pub linear_hertz: f32,
    pub linear_damping: f32,
    pub angular_hertz: f32,
    pub angular_damping: f32,
    pub max_spring_force: f32,
    pub max_spring_torque: f32,
}

#[derive(Clone, Copy, Default)]
pub struct Wheel {
    pub suspension: bool,
    pub suspension_hertz: f32,
    pub suspension_damping: f32,
    pub suspension_limited: bool,
    pub lower: f32,
    pub upper: f32,
    pub spin: bool,
    pub spin_speed: f32,
    pub spin_torque: f32,
    pub steering: bool,
    pub steering_hertz: f32,
    pub steering_damping: f32,
    pub steering_torque: f32,
    pub steering_limited: bool,
    pub steer_lower: f32,
    pub steer_upper: f32,
    pub target_steer: f32,
}

impl Joint {
    fn new(kind: Kind, body1: usize, body2: usize, frame1: Pose, frame2: Pose) -> Joint {
        Joint {
            handle: ImpulseJointHandle::invalid(),
            kind,
            body1,
            body2,
            frame1,
            frame2,
            collide: false,
            linear_spring: Vector::new(0.0, 0.0, f32::MAX),
            angular_spring: Vector::new(0.0, 0.0, f32::MAX),
            min: 0.0,
            max: 0.0,
            limited: false,
            min_force: f32::MIN,
            max_force: f32::MAX,
            drive: Drive::None,
            swing: 0.0,
            swing_limited: false,
            twist_limited: false,
            max_linear: f32::MAX,
            max_angular: f32::MAX,
            broken: false,
            ball_target: None,
            ball_motor: None,
            motor: MotorJoint::default(),
            wheel: Wheel::default(),
            parallel: (0.0, 0.0, f32::MAX),
            spring_on: true,
            initial_length: 0.0,
        }
    }

    /// A spring joint's rest length: what managed set, else where it began.
    pub fn rest_length(&self) -> f32 {
        let r = self.linear_spring.z;
        if r.is_finite() && r >= 0.0 && r < 1e30 { r } else { self.initial_length }
    }

    /// Whether a spring is one-sided: it only pulls, or only pushes.
    fn one_sided(&self) -> bool {
        self.kind == Kind::Spring && (self.max_force <= 0.0 || self.min_force >= 0.0)
    }
}

/// A spring of `hz` and damping ratio `zeta`, as Rapier's acceleration
/// motor's stiffness and damping.
fn spring(hz: f32, zeta: f32) -> (f32, f32) {
    let w = core::f32::consts::TAU * hz;
    (w * w, 2.0 * zeta * w)
}

fn finite(f: f32) -> f32 {
    if f.is_finite() && f > 0.0 { f.min(1e30) } else { 1e30 }
}

fn drive(g: &mut GenericJoint, axis: JointAxis, d: Drive, friction_limit: f32) {
    match d {
        Drive::None => {}
        Drive::Spring(target, hz, zeta) => {
            let (k, c) = spring(hz, zeta);
            g.set_motor_position(axis, target, k, c);
        }
        Drive::Velocity(speed, most) => {
            g.set_motor(axis, 0.0, speed, 0.0, MOTOR_GAIN);
            g.set_motor_max_force(axis, finite(most));
        }
        Drive::Friction(_) => {
            g.set_motor(axis, 0.0, 0.0, 0.0, MOTOR_GAIN);
            g.set_motor_max_force(axis, friction_limit.max(0.0));
        }
    }
}

/// The Rapier joint `j` describes. `friction` is the most force or torque
/// its friction holds with (`Drive::Friction`, worked out from its bodies).
fn build(j: &Joint, friction: f32) -> GenericJoint {
    use JointAxesMask as M;
    use JointAxis as A;
    let turn = j.kind.turn();
    let mut g = GenericJoint::default();
    g.set_local_frame1(Pose::from_parts(j.frame1.translation, j.frame1.rotation * turn));
    g.set_local_frame2(Pose::from_parts(j.frame2.translation, j.frame2.rotation * turn));
    g.set_contacts_enabled(j.collide);
    for a in [A::LinX, A::LinY, A::LinZ, A::AngX, A::AngY, A::AngZ] {
        g.set_motor_model(a, MotorModel::AccelerationBased);
    }
    match j.kind {
        Kind::Filter => {}
        Kind::Weld => {
            for (axes, mask, s) in [([A::LinX, A::LinY, A::LinZ], M::LIN_AXES, j.linear_spring), ([A::AngX, A::AngY, A::AngZ], M::ANG_AXES, j.angular_spring)] {
                if s.x > 0.0 {
                    let (k, c) = spring(s.x, s.y);
                    for a in axes {
                        g.set_motor_position(a, 0.0, k, c);
                        g.set_motor_max_force(a, finite(s.z));
                    }
                } else {
                    g.lock_axes(mask);
                }
            }
        }
        Kind::Revolute => {
            g.lock_axes(M::LIN_AXES | M::ANG_Y | M::ANG_Z);
            if j.limited && j.min < j.max { g.set_limits(A::AngX, [j.min, j.max]); }
            drive(&mut g, A::AngX, j.drive, friction);
        }
        Kind::Prismatic => {
            g.lock_axes(M::LIN_Y | M::LIN_Z | M::ANG_AXES);
            if j.limited && j.min <= j.max { g.set_limits(A::LinX, [j.min, j.max]); }
            drive(&mut g, A::LinX, j.drive, friction);
        }
        Kind::Spherical => {
            g.lock_axes(M::LIN_AXES);
            if j.twist_limited && j.min < j.max { g.set_limits(A::AngX, [j.min, j.max]); }
            if j.swing_limited {
                let s = j.swing.to_radians().abs();
                g.set_limits(A::AngY, [-s, s]);
                g.set_limits(A::AngZ, [-s, s]);
            }
            if let Some((target, hz, zeta)) = j.ball_target {
                let (k, c) = spring(hz, zeta);
                let angles = (turn.inverse() * target * turn).to_scaled_axis();
                for (a, v) in [(A::AngX, angles.x), (A::AngY, angles.y), (A::AngZ, angles.z)] { g.set_motor_position(a, v, k, c); }
            } else if let Some((velocity, most)) = j.ball_motor {
                let v = turn.inverse() * velocity;
                for (a, s) in [(A::AngX, v.x), (A::AngY, v.y), (A::AngZ, v.z)] {
                    g.set_motor(a, 0.0, s, 0.0, MOTOR_GAIN);
                    g.set_motor_max_force(a, finite(most));
                }
            } else if let Drive::Friction(_) = j.drive {
                for a in [A::AngX, A::AngY, A::AngZ] { drive(&mut g, a, j.drive, friction); }
            }
        }
        Kind::Motor => {
            let m = j.motor;
            for (axes, velocity, hz, zeta, most) in [
                ([A::LinX, A::LinY, A::LinZ], m.linear_velocity, m.linear_hertz, m.linear_damping, m.max_velocity_force.max(m.max_spring_force)),
                ([A::AngX, A::AngY, A::AngZ], m.angular_velocity, m.angular_hertz, m.angular_damping, m.max_velocity_torque.max(m.max_spring_torque)),
            ] {
                if most <= 0.0 { continue; }
                let (k, c) = if hz > 0.0 { spring(hz, zeta) } else { (0.0, 0.0) };
                let gain = if velocity != Vector::ZERO || m.max_velocity_force > 0.0 || m.max_velocity_torque > 0.0 { MOTOR_GAIN } else { 0.0 };
                for (i, a) in axes.into_iter().enumerate() {
                    g.set_motor(a, 0.0, velocity[i], k, c + gain);
                    g.set_motor_max_force(a, finite(most));
                }
            }
        }
        Kind::Wheel => {
            let w = j.wheel;
            g.lock_axes(M::LIN_Y | M::LIN_Z | M::ANG_Y);
            if w.suspension && w.suspension_hertz > 0.0 {
                let (k, c) = spring(w.suspension_hertz, w.suspension_damping);
                g.set_motor_position(A::LinX, 0.0, k, c);
            }
            if w.suspension_limited && w.lower <= w.upper { g.set_limits(A::LinX, [w.lower, w.upper]); }
            if w.steering {
                let (k, c) = spring(w.steering_hertz.max(0.0), w.steering_damping);
                g.set_motor_position(A::AngX, w.target_steer, k, c);
                g.set_motor_max_force(A::AngX, finite(w.steering_torque));
                if w.steering_limited && w.steer_lower <= w.steer_upper { g.set_limits(A::AngX, [w.steer_lower, w.steer_upper]); }
            } else {
                g.lock_axes(M::ANG_X);
            }
            if w.spin {
                g.set_motor(A::AngZ, 0.0, w.spin_speed, 0.0, MOTOR_GAIN);
                g.set_motor_max_force(A::AngZ, finite(w.spin_torque));
            }
        }
        Kind::Parallel => {
            let (hz, zeta, most) = j.parallel;
            if hz > 0.0 {
                let (k, c) = spring(hz, zeta);
                for a in [A::AngY, A::AngZ] {
                    g.set_motor_position(a, 0.0, k, c);
                    g.set_motor_max_force(a, finite(most));
                }
            } else {
                g.lock_axes(M::ANG_Y | M::ANG_Z);
            }
        }
        Kind::Spring => {
            g.coupled_axes |= M::LIN_AXES;
            let rest = j.rest_length();
            let hz = j.linear_spring.x;
            if j.limited && j.min <= j.max { g.set_limits(A::LinX, [j.min.max(0.0), j.max]); }
            if hz > 0.0 {
                if j.spring_on {
                    let (k, c) = spring(hz, j.linear_spring.y);
                    g.set_motor_position(A::LinX, rest, k, c);
                }
            } else if !j.limited {
                // no spring and no range: a rod at its length
                g.set_limits(A::LinX, [rest, rest]);
            }
        }
    }
    g
}

impl World {
    /// The joint at `ptr` made anew in Rapier from what it now says.
    fn rebuild_joint(&mut self, ptr: usize) {
        let Some(j) = self.joints.get(&ptr) else { return };
        let friction = match j.drive {
            Drive::Friction(mu) => self.friction_limit(j, mu),
            _ => 0.0,
        };
        let g = build(j, friction);
        let h = j.handle;
        if let Some(ij) = self.sim.impulse_joints.get_mut(h, true) { ij.data = g; }
    }

    /// Joint friction as 2D s&box works it out: the coefficient times the
    /// second body's weight, and for a turning joint, times its lever.
    fn friction_limit(&self, j: &Joint, mu: f32) -> f32 {
        let Some(rb) = self.bodies.get(&j.body2).and_then(|b| self.sim.bodies.get(b.handle)) else { return 0.0 };
        let force = mu * rb.mass() * self.sim.gravity.length();
        match j.kind {
            Kind::Prismatic => force,
            _ => {
                let m = rb.mass_properties().local_mprops;
                let mass = rb.mass();
                let lever = rb.local_center_of_mass().distance(j.frame2.translation);
                let inertia = m.principal_inertia().max_element();
                let inertia_lever = if mass > 0.0 { (inertia / mass).sqrt() } else { 0.0 };
                force * rb.gravity_scale() * lever.max(inertia_lever)
            }
        }
    }

    /// A spring that only pulls or only pushes, slack or taut as its length
    /// says before a step.
    pub(super) fn tension_springs(&mut self) {
        let ptrs: Vec<usize> = self.joints.iter().filter(|(_, j)| j.one_sided()).map(|(&p, _)| p).collect();
        for p in ptrs {
            let Some(len) = self.anchor_distance(p) else { continue };
            let j = &self.joints[&p];
            let stretched = len > j.rest_length();
            let on = if j.max_force <= 0.0 { stretched } else { !stretched };
            if on != j.spring_on {
                self.joints.get_mut(&p).unwrap().spring_on = on;
                self.rebuild_joint(p);
            }
        }
    }

    fn anchor_distance(&self, ptr: usize) -> Option<f32> {
        let j = self.joints.get(&ptr)?;
        let (a, b) = self.anchors(j)?;
        Some(a.distance(b))
    }

    /// A joint's frames in the world.
    fn frames(&self, j: &Joint) -> Option<(Pose, Pose)> {
        let p1 = self.bodies.get(&j.body1).and_then(|b| self.sim.bodies.get(b.handle)).map(|rb| *rb.position())?;
        let p2 = self.bodies.get(&j.body2).and_then(|b| self.sim.bodies.get(b.handle)).map(|rb| *rb.position())?;
        Some((p1 * j.frame1, p2 * j.frame2))
    }

    fn anchors(&self, j: &Joint) -> Option<(Vector, Vector)> {
        self.frames(j).map(|(a, b)| (a.translation, b.translation))
    }

    /// The joints that broke this step: pulled harder than their force or
    /// torque threshold. Each is told once.
    pub(super) fn broken_joints(&mut self, dt: f32) -> Vec<usize> {
        let mut broken = Vec::new();
        if dt <= 0.0 { return broken; }
        for (&p, j) in self.joints.iter_mut() {
            if j.broken || (j.max_linear >= f32::MAX && j.max_angular >= f32::MAX) { continue; }
            let Some(ij) = self.sim.impulse_joints.get(j.handle) else { continue };
            let (force, torque) = forces(ij, dt);
            if force > j.max_linear || torque > j.max_angular {
                j.broken = true;
                broken.push(p);
            }
        }
        broken
    }
}

/// A joint's force and torque over the last step: what its locked axes, its
/// limits and its motors (a soft weld's springs) pushed with.
fn forces(ij: &ImpulseJoint, dt: f32) -> (f32, f32) {
    let d = &ij.data;
    let i: [f32; 6] = core::array::from_fn(|k| ij.impulses[k] + d.limits[k].impulse + d.motors[k].impulse);
    (Vector::new(i[0], i[1], i[2]).length() / dt, Vector::new(i[3], i[4], i[5]).length() / dt)
}

// ---- what the slots call

/// A new joint at `ptr` of `kind` between two bodies of `world`, at their
/// frames: false if either body is unknown.
pub fn add_joint(world: usize, ptr: usize, kind: Kind, body1: usize, body2: usize, frame1: Pose, frame2: Pose) -> bool {
    let mut p = physics();
    let Some(w) = p.worlds.get_mut(&world) else { return false };
    let (Some(h1), Some(h2)) = (w.bodies.get(&body1).map(|b| b.handle), w.bodies.get(&body2).map(|b| b.handle)) else { return false };
    let mut j = Joint::new(kind, body1, body2, frame1, frame2);
    if let Some((a, b)) = w.anchors(&j) { j.initial_length = a.distance(b); }
    j.min = j.initial_length;
    j.max = j.initial_length;
    j.handle = w.sim.impulse_joints.insert(h1, h2, build(&j, 0.0), true);
    w.joints.insert(ptr, j);
    p.joint_world.insert(ptr, world);
    true
}

/// A joint gone from its world: whether it was there.
pub fn remove_joint(ptr: usize) -> bool {
    let mut p = physics();
    let Some(w) = p.joint_world.remove(&ptr) else { return false };
    let Some(world) = p.worlds.get_mut(&w) else { return false };
    let Some(j) = world.joints.remove(&ptr) else { return false };
    world.sim.impulse_joints.remove(j.handle, true);
    true
}

/// Reads a joint.
pub fn with_joint<R>(ptr: usize, f: impl FnOnce(&Joint) -> R) -> Option<R> {
    let p = physics();
    let w = p.joint_world.get(&ptr)?;
    p.worlds.get(w)?.joints.get(&ptr).map(f)
}

/// Changes what a joint says, and makes it so.
pub fn edit_joint<R>(ptr: usize, f: impl FnOnce(&mut Joint) -> R) -> Option<R> {
    let mut p = physics();
    let w = *p.joint_world.get(&ptr)?;
    let world = p.worlds.get_mut(&w)?;
    let r = f(world.joints.get_mut(&ptr)?);
    world.rebuild_joint(ptr);
    Some(r)
}

/// How hard a joint is holding: its force and its torque.
pub fn joint_forces(ptr: usize) -> (f32, f32) {
    let p = physics();
    let Some(w) = p.joint_world.get(&ptr).and_then(|w| p.worlds.get(w)) else { return (0.0, 0.0) };
    let dt = w.sim.integration_parameters.dt;
    w.joints.get(&ptr).and_then(|j| w.sim.impulse_joints.get(j.handle)).map(|ij| forces(ij, dt)).unwrap_or((0.0, 0.0))
}

/// The second frame's turn from the first about the joint's main axis
/// (a hinge's angle; a wheel's steering), in radians.
pub fn joint_angle(ptr: usize, about: Vector) -> f32 {
    let p = physics();
    let Some(w) = p.joint_world.get(&ptr).and_then(|w| p.worlds.get(w)) else { return 0.0 };
    let Some((a, b)) = w.joints.get(&ptr).and_then(|j| w.frames(j)) else { return 0.0 };
    let q = a.rotation.inverse() * b.rotation;
    let s = Vector::new(q.x, q.y, q.z).dot(about);
    let angle = 2.0 * s.atan2(q.w);
    if angle > core::f32::consts::PI { angle - core::f32::consts::TAU } else if angle < -core::f32::consts::PI { angle + core::f32::consts::TAU } else { angle }
}

/// How fast the second body turns against the first about the joint's
/// `about` axis (in the first frame): a wheel's spin.
pub fn joint_spin(ptr: usize, about: Vector) -> f32 {
    let p = physics();
    let Some(w) = p.joint_world.get(&ptr).and_then(|w| p.worlds.get(w)) else { return 0.0 };
    let Some(j) = w.joints.get(&ptr) else { return 0.0 };
    let Some((a, _)) = w.frames(j) else { return 0.0 };
    let v = |b: usize| w.bodies.get(&b).and_then(|b| w.sim.bodies.get(b.handle)).map(|rb| rb.angvel()).unwrap_or(Vector::ZERO);
    (v(j.body2) - v(j.body1)).dot(a.rotation * about)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ptr() -> usize {
        Box::into_raw(Box::new([0u64; 4])) as usize
    }

    /// A world under s&box's gravity, a static anchor at the origin, and a
    /// 10-unit box at `at`.
    fn rig(at: Vector) -> (usize, usize, usize) {
        let w = ptr();
        create_world(w);
        let anchor = ptr();
        add_body(w, anchor, &BodySettings::default());
        let b = ptr();
        add_body(w, b, &BodySettings { body_type: DYNAMIC as i32, position: v3(at), ..Default::default() });
        assert!(add_shape(b, ptr(), &Geometry::Box(Pose::IDENTITY, Vector::splat(5.0)), &ShapeSettings::default()));
        (w, anchor, b)
    }

    fn at(b: usize) -> Vector {
        with_body(b, |_, rb| rb.translation()).unwrap()
    }

    fn run(w: usize, seconds: f32) -> Vec<usize> {
        let mut broken = Vec::new();
        for _ in 0..(seconds * 60.0) as usize { broken.extend(step(w, 1.0 / 60.0, 2).broken); }
        broken
    }

    /// A frame at `p` whose z (the hinge's axis) points along `axis`.
    fn frame(p: Vector, axis: Vector) -> Pose {
        Pose::from_parts(p, Rotation::from_rotation_arc(Vector::Z, axis))
    }

    #[test]
    fn a_hinge_swings_its_body_round_its_axis() {
        // pivot at the origin, axis along y; the box 100 out along x falls round it
        let (w, anchor, b) = rig(Vector::new(100.0, 0.0, 0.0));
        let j = ptr();
        assert!(add_joint(w, j, Kind::Revolute, anchor, b, frame(Vector::ZERO, Vector::Y), frame(Vector::new(-100.0, 0.0, 0.0), Vector::Y)));
        assert_eq!(with_joint(j, |j| j.kind.native()), Some(3));
        run(w, 0.25);
        let p = at(b);
        assert!((p.length() - 100.0).abs() < 1.0, "keeps its distance: {p:?}");
        assert!(p.y.abs() < 0.5, "stays in the plane the axis is normal to: {p:?}");
        assert!(p.z < -20.0, "and swings down: {p:?}");
        assert!(joint_angle(j, Vector::Z).abs() > 0.2);
        // limited to a quarter turn either way, it hangs at the limit's end at most
        edit_joint(j, |j| {
            j.min = -0.3;
            j.max = 0.3;
            j.limited = true;
        });
        run(w, 2.0);
        assert!(joint_angle(j, Vector::Z).abs() < 0.35, "{}", joint_angle(j, Vector::Z));
    }

    #[test]
    fn a_hinge_motor_turns_at_its_speed() {
        // no gravity to fight: the axis is upright, the motor spins it at 2 rad/s
        // (on a lever this long Rapier's solver takes a few steps to get there)
        let (w, anchor, b) = rig(Vector::new(50.0, 0.0, 0.0));
        let j = ptr();
        assert!(add_joint(w, j, Kind::Revolute, anchor, b, frame(Vector::ZERO, Vector::Z), frame(Vector::new(-50.0, 0.0, 0.0), Vector::Z)));
        edit_joint(j, |j| j.drive = Drive::Velocity(2.0, 1e9));
        run(w, 1.0);
        let spin = with_body(b, |_, rb| rb.angvel()).unwrap();
        assert!((spin.z - 2.0).abs() < 0.05, "{spin:?}");
        assert!((at(b).length() - 50.0).abs() < 0.5);
    }

    #[test]
    fn a_slider_runs_along_its_axis_between_its_limits() {
        // the slider's x is the world's z: the box drops straight down, 40 at most
        let (w, anchor, b) = rig(Vector::new(30.0, 0.0, 0.0));
        let down = Pose::from_parts(Vector::ZERO, Rotation::from_rotation_arc(Vector::X, Vector::Z));
        let j = ptr();
        assert!(add_joint(w, j, Kind::Prismatic, anchor, b, Pose::from_parts(Vector::new(30.0, 0.0, 0.0), down.rotation), down));
        edit_joint(j, |j| {
            j.min = -40.0;
            j.max = 0.0;
            j.limited = true;
        });
        run(w, 2.0);
        let p = at(b);
        assert!((p.x - 30.0).abs() < 0.1 && p.y.abs() < 0.1, "only along its axis: {p:?}");
        assert!((p.z + 40.0).abs() < 1.0, "to its lower limit: {p:?}");
    }

    #[test]
    fn a_weld_holds_and_breaks_past_its_strength() {
        let (w, anchor, b) = rig(Vector::new(0.0, 0.0, 50.0));
        let j = ptr();
        assert!(add_joint(w, j, Kind::Weld, anchor, b, Pose::from_translation(Vector::new(0.0, 0.0, 50.0)), Pose::IDENTITY));
        assert!(run(w, 1.0).is_empty());
        assert!((at(b) - Vector::new(0.0, 0.0, 50.0)).length() < 0.5, "{:?}", at(b));
        // the box weighs ~16 kg: holding it up takes ~14000 (kg in/s²); a threshold under that breaks
        let (force, _) = joint_forces(j);
        assert!(force > 1000.0, "{force}");
        edit_joint(j, |j| j.max_linear = force * 0.5);
        let broken = run(w, 0.1);
        assert_eq!(broken, vec![j], "told once");
        assert!(run(w, 0.1).is_empty());
    }

    #[test]
    fn a_soft_weld_breaks_too() {
        // FixedJoint's own default: a 10 Hz linear spring, its force in its motors
        let (w, anchor, b) = rig(Vector::new(0.0, 0.0, 50.0));
        let j = ptr();
        assert!(add_joint(w, j, Kind::Weld, anchor, b, Pose::from_translation(Vector::new(0.0, 0.0, 50.0)), Pose::IDENTITY));
        edit_joint(j, |j| j.linear_spring = Vector::new(10.0, 0.0, f32::MAX));
        run(w, 1.0);
        let (force, _) = joint_forces(j);
        assert!(force > 1000.0, "{force}");
        edit_joint(j, |j| j.max_linear = 1.0);
        assert_eq!(run(w, 0.1), vec![j]);
    }

    #[test]
    fn a_rope_lets_its_body_fall_to_its_length_and_no_further() {
        // CreateLength: a spring joint with a range and no spring
        let (w, anchor, b) = rig(Vector::new(0.0, 0.0, -20.0));
        let j = ptr();
        assert!(add_joint(w, j, Kind::Spring, anchor, b, Pose::IDENTITY, Pose::IDENTITY));
        assert!((with_joint(j, |j| j.initial_length).unwrap() - 20.0).abs() < 1e-3);
        edit_joint(j, |j| {
            j.max = 100.0;
            j.min = 0.0;
            j.limited = true;
        });
        run(w, 2.0);
        assert!((at(b).length() - 100.0).abs() < 1.0, "{:?}", at(b));
        // a spring at rest 60, stiff enough to hold it near there
        edit_joint(j, |j| {
            j.linear_spring = Vector::new(5.0, 1.0, 60.0);
            j.limited = false;
        });
        run(w, 3.0);
        let l = at(b).length();
        assert!(l > 60.0 && l < 75.0, "sagging a little under its weight: {l}");
    }

    #[test]
    fn a_pull_only_spring_goes_slack_when_compressed() {
        let (w, anchor, b) = rig(Vector::new(0.0, 0.0, 30.0));
        set_gravity(w, Vector::ZERO);
        let j = ptr();
        assert!(add_joint(w, j, Kind::Spring, anchor, b, Pose::IDENTITY, Pose::IDENTITY));
        edit_joint(j, |j| {
            j.linear_spring = Vector::new(5.0, 1.0, 60.0);
            j.min_force = f32::MIN;
            j.max_force = 0.0;
        });
        // shorter than its rest length: no push
        run(w, 0.5);
        assert!((at(b).length() - 30.0).abs() < 0.5, "{:?}", at(b));
        assert!(!with_joint(j, |j| j.spring_on).unwrap());
    }

    #[test]
    fn a_wheel_spins_on_its_axle() {
        let (w, anchor, b) = rig(Vector::new(0.0, 0.0, 0.0));
        set_gravity(w, Vector::ZERO);
        let j = ptr();
        assert!(add_joint(w, j, Kind::Wheel, anchor, b, Pose::IDENTITY, Pose::IDENTITY));
        assert_eq!(with_joint(j, |j| j.kind.native()), Some(16));
        edit_joint(j, |j| {
            j.wheel.spin = true;
            j.wheel.spin_speed = 5.0;
            j.wheel.spin_torque = 1e9;
        });
        run(w, 0.5);
        assert!((joint_spin(j, Vector::Z) - 5.0).abs() < 0.1, "{}", joint_spin(j, Vector::Z));
        assert!(at(b).length() < 0.1);
    }

    #[test]
    fn a_body_removed_takes_its_joints() {
        let (w, anchor, b) = rig(Vector::new(0.0, 0.0, 50.0));
        let j = ptr();
        assert!(add_joint(w, j, Kind::Spherical, anchor, b, Pose::from_translation(Vector::new(0.0, 0.0, 50.0)), Pose::IDENTITY));
        let gone = remove_body(b);
        assert!(gone.contains(&j) && gone.contains(&b));
        assert!(with_joint(j, |_| ()).is_none());
        assert!(!remove_joint(j));
        run(w, 0.1);
    }
}
