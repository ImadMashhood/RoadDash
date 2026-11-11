//! Scoped memoization of exact terrain vertices during immutable Hills generation.
//! Never retained by the simulation; cache keys preserve float bits, not rounded positions.
use crate::track::Track;
use bevy::prelude::*;
use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};
const MAX_VERTICES: usize = 131_072;
struct Cache {
    track: usize,
    vertices: HashMap<(u32, u32), Vec3>,
    hits: usize,
    misses: usize,
}
thread_local! {
    static CACHE: RefCell<Option<Cache>> = const { RefCell::new(None) };
}
pub(super) struct Scope<'a> {
    previous: Option<Cache>,
    _track: PhantomData<(&'a Track, Rc<()>)>,
}
impl<'a> Scope<'a> {
    pub fn new(track: &'a Track) -> Self {
        Self::with_enabled(
            track,
            std::env::var("ROADDASH_TERRAIN_CACHE").as_deref() != Ok("0"),
        )
    }
    fn with_enabled(track: &'a Track, enabled: bool) -> Self {
        let current = enabled.then(|| Cache {
            track: std::ptr::from_ref(track) as usize,
            vertices: HashMap::new(),
            hits: 0,
            misses: 0,
        });
        Self {
            previous: CACHE.with(|slot| slot.replace(current)),
            _track: PhantomData,
        }
    }
}
impl Drop for Scope<'_> {
    fn drop(&mut self) {
        let current = CACHE.with(|slot| slot.replace(self.previous.take()));
        if std::env::var("ROADDASH_LOAD_PROFILE").as_deref() == Ok("1")
            && let Some(c) = current
        {
            eprintln!(
                "TERRAIN_CACHE {}",
                serde_json::json!({"vertices":c.vertices.len(),"hits":c.hits,"misses":c.misses,"limit":MAX_VERTICES})
            );
        }
    }
}
pub(super) fn vertex(track: &Track, x: f32, z: f32, compute: impl FnOnce() -> Vec3) -> Vec3 {
    let id = std::ptr::from_ref(track) as usize;
    let key = (x.to_bits(), z.to_bits());
    let cached = CACHE.with(|slot| {
        let mut slot = slot.borrow_mut();
        let c = slot.as_mut().filter(|c| c.track == id)?;
        let value = c.vertices.get(&key).copied();
        if value.is_some() {
            c.hits += 1;
        } else {
            c.misses += 1;
        }
        value
    });
    if let Some(value) = cached {
        return value;
    }
    let value = compute();
    CACHE.with(|slot| {
        if let Some(c) = slot.borrow_mut().as_mut().filter(|c| c.track == id)
            && c.vertices.len() < MAX_VERTICES
        {
            c.vertices.insert(key, value);
        }
    });
    value
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disabled_scope_bypasses_storage_and_vertex_limit_is_bounded() {
        let track = Track::hills().unwrap();
        {
            let _scope = Scope::with_enabled(&track, false);
            assert_eq!(vertex(&track, 1., 2., || Vec3::X), Vec3::X);
            assert_eq!(vertex(&track, 1., 2., || Vec3::Y), Vec3::Y);
            CACHE.with(|slot| assert!(slot.borrow().is_none()));
        }
        let _scope = Scope::with_enabled(&track, true);
        for i in 0..MAX_VERTICES + 16 {
            let expected = Vec3::new(i as f32, 2., 3.);
            assert_eq!(vertex(&track, i as f32, 0., || expected), expected);
        }
        CACHE.with(|slot| assert_eq!(slot.borrow().as_ref().unwrap().vertices.len(), MAX_VERTICES));
        assert_eq!(
            vertex(&track, (MAX_VERTICES + 1) as f32, 0., || Vec3::Z),
            Vec3::Z
        );
    }
    #[test]
    fn cached_resident_heights_are_bit_exact_and_scope_does_not_leak() {
        let track = Track::hills().unwrap();
        let points: Vec<_> = (0..=12500)
            .step_by(97)
            .flat_map(|s| {
                [-260., -30., -8., 0., 8., 30., 260.]
                    .map(|lane| track.sample_route(0, s as f32, lane).0)
            })
            .collect();
        let direct: Vec<_> = points
            .iter()
            .map(|p| super::super::geometry::ground_height(&track, p.x, p.z).to_bits())
            .collect();
        {
            let _scope = Scope::with_enabled(&track, true);
            for _ in 0..2 {
                let cached: Vec<_> = points
                    .iter()
                    .map(|p| super::super::geometry::ground_height(&track, p.x, p.z).to_bits())
                    .collect();
                assert_eq!(direct, cached);
            }
            CACHE.with(|slot| assert!(slot.borrow().as_ref().unwrap().hits > 0));
            let other = Track::hills().unwrap();
            assert_eq!(vertex(&other, 1., 2., || Vec3::splat(7.)), Vec3::splat(7.));
            {
                let _nested = Scope::with_enabled(&other, true);
            }
            CACHE.with(|slot| {
                assert_eq!(
                    slot.borrow().as_ref().unwrap().track,
                    std::ptr::from_ref(&track) as usize
                )
            });
        }
        CACHE.with(|slot| assert!(slot.borrow().is_none()));
    }
}
