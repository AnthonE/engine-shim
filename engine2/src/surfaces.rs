//! Surfaces: what a shape is made of. Each `.surface` a game loads
//! (`Sandbox.Surface`) is added here by name (`AddProperty`) and handed its
//! friction, elasticity, density and the rest (`UpdatePhysics`); managed
//! keeps the index it was given, and a shape names its surface by that
//! index (its settings, `SetSurfaceIndex`) or by name (`SetMaterialIndex`).
//! Index 0 is the default surface, with the values of the engine's own
//! `surfaces/default.surface` until managed loads that file over them.
//!
//! The object managed holds (a `CPhysSurfaceProperties`) is the surface
//! itself, at an address of its own for good: its index, name hash and
//! base are written where the generated field accessors read them.
use std::ffi::CString;
use std::sync::{Mutex, OnceLock};

use crate::handles::murmur2;
use crate::slots::{
    PROP__CPhysSrfcPrprts_m_baseNameHash, PROP__CPhysSrfcPrprts_m_nBaseIndex, PROP__CPhysSrfcPrprts_m_nIndex,
    PROP__CPhysSrfcPrprts_m_nameHash,
};

/// Cubic inches in a cubic metre: a surface's density is per cubic metre,
/// a shape's volume in inches.
pub const CUBIC_INCHES_PER_CUBIC_METRE: f32 = 61_023.75;

/// What a surface does to the shapes made of it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Props {
    pub friction: f32,
    pub elasticity: f32,
    /// kilograms per cubic metre
    pub density: f32,
    pub rolling_resistance: f32,
    pub bounce_threshold: f32,
}

impl Props {
    /// Kilograms per cubic inch: what a shape's volume weighs.
    pub fn density_per_cubic_inch(&self) -> f32 {
        self.density / CUBIC_INCHES_PER_CUBIC_METRE
    }
}

/// The engine's default surface (`surfaces/default.surface`).
pub const DEFAULT: Props = Props { friction: 0.8, elasticity: 0.05, density: 2000.0, rolling_resistance: 0.0, bounce_threshold: 35.0 };

pub struct Surface {
    pub name: CString,
    pub description: CString,
    pub hash: u32,
    pub index: i32,
    pub base: i32,
    pub props: Props,
}

/// Every surface, by index. Boxed: its address is what managed holds.
fn all() -> std::sync::MutexGuard<'static, Vec<Box<Surface>>> {
    static S: OnceLock<Mutex<Vec<Box<Surface>>>> = OnceLock::new();
    S.get_or_init(|| {
        let s = Box::new(Surface {
            name: CString::new("default").unwrap(),
            description: CString::default(),
            hash: murmur2("default"),
            index: 0,
            base: 0,
            props: DEFAULT,
        });
        remember(&s);
        Mutex::new(vec![s])
    })
    .lock()
    .unwrap()
}

fn ptr(s: &Surface) -> usize {
    s as *const Surface as usize
}

/// Where the generated field accessors look.
fn remember(s: &Surface) {
    let p = ptr(s);
    crate::state::set(p, PROP__CPhysSrfcPrprts_m_nIndex, s.index);
    crate::state::set(p, PROP__CPhysSrfcPrprts_m_nameHash, s.hash);
    crate::state::set(p, PROP__CPhysSrfcPrprts_m_nBaseIndex, s.base);
}

fn find(all: &[Box<Surface>], hash: u32) -> Option<usize> {
    all.iter().position(|s| s.hash == hash)
}

/// The surface named `name` (made, on `base`'s values, if it's new): its
/// object. A name added again is the same surface.
pub fn add(name: &str, base: &str, description: &str) -> usize {
    let mut all = all();
    let hash = murmur2(name);
    let description = CString::new(description.replace('\0', "")).unwrap_or_default();
    if let Some(i) = find(&all, hash) {
        all[i].description = description;
        return ptr(&all[i]);
    }
    let base = find(&all, murmur2(base)).unwrap_or(0);
    let s = Box::new(Surface {
        name: CString::new(name.replace('\0', "")).unwrap_or_default(),
        description,
        hash,
        index: all.len() as i32,
        base: base as i32,
        props: all[base].props,
    });
    remember(&s);
    crate::state::set(ptr(&s), PROP__CPhysSrfcPrprts_m_baseNameHash, all[base].hash);
    let p = ptr(&s);
    all.push(s);
    p
}

pub fn count() -> i32 {
    all().len() as i32
}

/// Surface `index`'s object: null past the end.
pub fn object(index: i32) -> usize {
    let all = all();
    usize::try_from(index).ok().and_then(|i| all.get(i)).map(|s| ptr(s)).unwrap_or(0)
}

/// The index of the surface whose name hashes to `hash` (a StringToken).
pub fn by_hash(hash: u32) -> Option<i32> {
    find(&all(), hash).map(|i| i as i32)
}

/// Every surface's props, by index: for the step, which may not wait on this lock.
pub fn table() -> Vec<Props> {
    all().iter().map(|s| s.props).collect()
}

/// Surface `index`'s name (the default's for one unknown), for as long as the surface lives.
pub fn name(index: i32) -> usize {
    let all = all();
    usize::try_from(index).ok().and_then(|i| all.get(i)).unwrap_or(&all[0]).name.as_ptr() as usize
}

pub fn description(p: usize) -> usize {
    all().iter().find(|s| ptr(s) == p).map(|s| s.description.as_ptr() as usize).unwrap_or(c"".as_ptr() as usize)
}

pub fn name_of(p: usize) -> usize {
    all().iter().find(|s| ptr(s) == p).map(|s| s.name.as_ptr() as usize).unwrap_or(c"".as_ptr() as usize)
}

/// The surface at `p` now does `props`: its index, if it is one.
pub fn update(p: usize, props: Props) -> Option<i32> {
    let mut all = all();
    let s = all.iter_mut().find(|s| ptr(s) == p)?;
    s.props = props;
    Some(s.index)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index_of(p: usize) -> Option<i32> {
        all().iter().find(|s| ptr(s) == p).map(|s| s.index)
    }

    fn props(i: i32) -> Props {
        table()[i as usize]
    }

    #[test]
    fn a_surface_is_added_once_on_its_base_and_found_again() {
        let rubber = add("test_rubber", "default", "bouncy");
        let i = index_of(rubber).unwrap();
        assert!(i > 0);
        assert_eq!(add("Test_Rubber", "default", "bouncier"), rubber, "a name is a surface, any case");
        assert_eq!(crate::state::get::<i32>(rubber, PROP__CPhysSrfcPrprts_m_nIndex), i);
        assert_eq!(crate::state::get::<u32>(rubber, PROP__CPhysSrfcPrprts_m_nameHash), murmur2("test_rubber"));
        assert_eq!(by_hash(murmur2("test_rubber")), Some(i));
        assert_eq!(object(i), rubber);
        assert_eq!(object(count()), 0);
        let read = |p: usize| unsafe { std::ffi::CStr::from_ptr(p as *const core::ffi::c_char) }.to_str().unwrap().to_string();
        assert_eq!(read(name(i)), "test_rubber");
        assert_eq!(read(description(rubber)), "bouncier");
        assert_eq!(read(name(-1)), "default");
        assert_eq!(props(i), DEFAULT, "a new surface starts as its base");

        let bouncy = Props { friction: 0.8, elasticity: 0.4, density: 1200.0, rolling_resistance: 0.0, bounce_threshold: 35.0 };
        assert_eq!(update(rubber, bouncy), Some(i));
        let sheet = add("test_rubber.sheet", "test_rubber", "");
        assert_eq!(props(index_of(sheet).unwrap()), bouncy, "on its base's values");
        assert_eq!(crate::state::get::<i32>(sheet, PROP__CPhysSrfcPrprts_m_nBaseIndex), i);
        assert!((bouncy.density_per_cubic_inch() - 1200.0 / 61_023.75).abs() < 1e-9);
    }
}
