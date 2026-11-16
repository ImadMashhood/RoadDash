//! Per-rider, per-fixed-tick reuse of immutable-world collision forecasts.
use bevy::{
    prelude::{Entity, IVec2, Vec2, Vec3},
    shape::Aabb3d,
};
use bevy_rapier3d::{
    parry::shape::SharedShape,
    rapier::{geometry::Collider, math::Pose},
};
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    time::Instant,
};

#[derive(Clone, Copy, Default, serde::Serialize)]
pub struct Counters {
    pub drive_ms: f64,
    pub static_ms: f64,
    pub static_requests: u64,
    pub static_cache_hits: u64,
    pub static_casts: u64,
    pub static_blocked_cache_hits: u64,
    pub shared_scans: u64,
    pub shared_clear_segments: u64,
    pub shared_scan_ms: f64,
    pub plan_searches: u64,
    pub plan_reuses: u64,
    pub immediate_replans: u64,
}
impl Counters {
    pub fn merge(&mut self, other: Self) {
        self.drive_ms += other.drive_ms;
        self.static_ms += other.static_ms;
        self.static_requests += other.static_requests;
        self.static_cache_hits += other.static_cache_hits;
        self.static_casts += other.static_casts;
        self.static_blocked_cache_hits += other.static_blocked_cache_hits;
        self.shared_scans += other.shared_scans;
        self.shared_clear_segments += other.shared_clear_segments;
        self.shared_scan_ms += other.shared_scan_ms;
        self.plan_searches += other.plan_searches;
        self.plan_reuses += other.plan_reuses;
        self.immediate_replans += other.immediate_replans;
    }
}
const CELL_M: f32 = 2.;
const MAX_CELLS: usize = 1024 * 1024;
const SWEEP_PADDING: f32 = 0.66;
const SCAN_SNAP_M: f32 = 64.;

struct Solid {
    entity: Entity,
    shape: SharedShape,
    pose: Pose,
}
impl Solid {
    fn matches(&self, other: &Self) -> bool {
        self.entity == other.entity
            && self.pose == other.pose
            && std::sync::Arc::ptr_eq(&self.shape.0, &other.shape.0)
    }
}

#[derive(Default)]
pub(crate) struct StaticOccupancy {
    bounds: Option<Aabb3d>,
    base: IVec2,
    width: usize,
    heights: Vec<[f32; 2]>,
    solids: Vec<Solid>,
    scan: Vec<Solid>,
}

impl StaticOccupancy {
    fn cell(p: Vec3) -> IVec2 {
        (Vec2::new(p.x, p.z) / CELL_M).floor().as_ivec2()
    }

    fn layout(bounds: Aabb3d) -> Option<(IVec2, usize, usize)> {
        let base = Self::cell(bounds.min.into());
        let size = Self::cell(bounds.max.into()) - base + IVec2::ONE;
        if size.x <= 0 || size.y <= 0 {
            return None;
        }
        let cells = (size.x as usize).checked_mul(size.y as usize)?;
        (cells <= MAX_CELLS).then_some((base, size.x as usize, cells))
    }

    fn reset(&mut self, bounds: Aabb3d) -> bool {
        let Some((base, width, cells)) = Self::layout(bounds) else {
            return false;
        };
        self.bounds = Some(bounds);
        self.base = base;
        self.width = width;
        let empty = [f32::INFINITY, f32::NEG_INFINITY];
        self.heights.resize(cells, empty);
        self.heights.fill(empty);
        true
    }

    fn covering_bounds(&self, requested: Aabb3d) -> Aabb3d {
        if let Some(bounds) = self.bounds
            && bounds.min.cmple(requested.min).all()
            && bounds.max.cmpge(requested.max).all()
        {
            return bounds;
        }
        Aabb3d {
            min: (requested.min / SCAN_SNAP_M).floor() * SCAN_SNAP_M,
            max: (requested.max / SCAN_SNAP_M).ceil() * SCAN_SNAP_M,
        }
    }

    pub(crate) fn refresh<'a>(
        &mut self,
        bounds: Aabb3d,
        colliders: impl Iterator<Item = (Entity, &'a Collider)>,
    ) -> bool {
        self.scan.clear();
        self.scan.extend(colliders.map(|(entity, collider)| Solid {
            entity,
            shape: collider.shared_shape().clone(),
            pose: *collider.position(),
        }));
        self.scan.sort_unstable_by_key(|solid| solid.entity);
        let same_region = self
            .bounds
            .is_some_and(|previous| bounds.min == previous.min && bounds.max == previous.max);
        if same_region
            && self.solids.len() == self.scan.len()
            && self
                .solids
                .iter()
                .zip(&self.scan)
                .all(|(a, b)| a.matches(b))
        {
            return true;
        }
        if !self.reset(bounds) {
            return false;
        }
        let current = std::mem::take(&mut self.scan);
        for solid in &current {
            self.mark_shape(solid.shape.as_ref(), &solid.pose);
        }
        self.scan = std::mem::replace(&mut self.solids, current);
        true
    }

    fn mark_shape(&mut self, shape: &dyn bevy_rapier3d::parry::shape::Shape, pose: &Pose) {
        use bevy_rapier3d::parry::partitioning::TraversalAction;
        if let Some(mesh) = shape.as_trimesh() {
            let Some(bounds) = self.bounds else { return };
            // Cut the mesh hierarchy at small spatial groups. Its full AABB
            // can cover many streets that have no solid triangles in them.
            mesh.bvh().traverse(|node| {
                let aabb = node.aabb().transform_by(pose);
                if aabb.maxs.cmplt(bounds.min.into()).any()
                    || aabb.mins.cmpgt(bounds.max.into()).any()
                {
                    return TraversalAction::Prune;
                }
                let extent = aabb.extents();
                if node.is_leaf() || extent.x.max(extent.z) <= 6. {
                    self.mark(aabb.mins, aabb.maxs);
                    TraversalAction::Prune
                } else {
                    TraversalAction::Continue
                }
            });
        } else if let Some(compound) = shape.as_compound() {
            for (part_pose, part) in compound.shapes() {
                let aabb = part.compute_aabb(&(*pose * *part_pose));
                self.mark(aabb.mins, aabb.maxs);
            }
        } else {
            let aabb = shape.compute_aabb(pose);
            self.mark(aabb.mins, aabb.maxs);
        }
    }

    pub(crate) fn mark(&mut self, min: Vec3, max: Vec3) {
        let Some(bounds) = self.bounds else { return };
        let min = min.max(bounds.min.into());
        let max = max.min(bounds.max.into());
        if min.cmpgt(max).any() {
            return;
        }
        let lo = Self::cell(min) - self.base;
        let hi = Self::cell(max) - self.base;
        for z in lo.y..=hi.y {
            let row = z as usize * self.width;
            for x in lo.x..=hi.x {
                let range = &mut self.heights[row + x as usize];
                range[0] = range[0].min(min.y);
                range[1] = range[1].max(max.y);
            }
        }
    }

    fn clear(&self, from: Vec3, to: Vec3) -> bool {
        let Some(bounds) = self.bounds else {
            return false;
        };
        let min = from.min(to) - Vec3::splat(SWEEP_PADDING);
        let max = from.max(to) + Vec3::splat(SWEEP_PADDING);
        if min.cmplt(bounds.min.into()).any() || max.cmpgt(bounds.max.into()).any() {
            return false;
        }
        let lo = Self::cell(min) - self.base;
        let hi = Self::cell(max) - self.base;
        for z in lo.y..=hi.y {
            let row = z as usize * self.width;
            for x in lo.x..=hi.x {
                let [bottom, top] = self.heights[row + x as usize];
                if max.y >= bottom && min.y <= top {
                    return false;
                }
            }
        }
        true
    }
}

/// Conservative width envelopes baked once from the authored Hills spline.
#[derive(Default)]
pub struct HillsCorridors {
    first: f32,
    widths: Vec<f32>,
}
impl HillsCorridors {
    const STEP: f32 = 8.;
    pub(crate) fn bake(track: &crate::track::Track) -> Self {
        let first = track.distances[0].min(-300.);
        let count = ((track.length - first) / Self::STEP).ceil() as usize + 1;
        let widths = (0..count)
            .map(|index| {
                let from = first + index as f32 * Self::STEP;
                let to = (from + Self::STEP + 120.).min(track.length - 0.01);
                // The spline width interpolates linearly between the resident stations.
                let range = track.distances.partition_point(|s| *s < from)
                    ..track.distances.partition_point(|s| *s < to);
                let mut width = track.half_width(from).min(track.half_width(to));
                if let Some(stations) = track.distances.get(range) {
                    for &station in stations {
                        width = width.min(track.half_width(station));
                    }
                }
                width
            })
            .collect();
        Self { first, widths }
    }

    pub(crate) fn bounds(&self, progress: f32, center: f32) -> Option<(f32, f32)> {
        let index = (progress - self.first) / Self::STEP;
        if index < 0. {
            return None;
        }
        let width = *self.widths.get(index.floor() as usize)?;
        let verge = if (-300. ..1000.).contains(&progress) {
            2.7
        } else {
            1.5
        };
        let half = width + verge - 0.65;
        Some((center - half, center + half))
    }
}

#[derive(Clone, Copy)]
pub(crate) struct HeldPlan {
    pub tick: u64,
    pub route: usize,
    pub station: f32,
    pub requested: f32,
    pub plan: rr_core::traffic_avoidance::Plan,
}

#[derive(Clone, Copy)]
struct Entry {
    goal: u32,
    speed: u32,
    clear: bool,
}
pub struct Work {
    cache: RefCell<Vec<Entry>>,
    counters: RefCell<Counters>,
    segments: RefCell<HashMap<[u32; 6], bool>>,
    enabled: bool,
    reuse_blocked: bool,
    shared_enabled: bool,
    replan_period: u64,
    shared_attempted: Cell<bool>,
    shared_current: Cell<bool>,
    shared: RefCell<StaticOccupancy>,
}
impl Default for Work {
    fn default() -> Self {
        Self {
            cache: RefCell::new(Vec::with_capacity(128)),
            counters: RefCell::default(),
            segments: RefCell::new(HashMap::with_capacity(256)),
            enabled: std::env::var("ROADDASH_AI_STATIC_CACHE").as_deref() != Ok("0"),
            reuse_blocked: std::env::var("ROADDASH_AI_BLOCKED_CACHE").as_deref() != Ok("0"),
            shared_enabled: std::env::var("ROADDASH_AI_SHARED_SCAN").as_deref() != Ok("0"),
            replan_period: match std::env::var("ROADDASH_AI_REPLAN_HZ").as_deref() {
                Ok("60") => 1,
                Ok("20") => 3,
                _ => 6,
            },
            shared_attempted: Cell::new(false),
            shared_current: Cell::new(false),
            shared: RefCell::new(StaticOccupancy::default()),
        }
    }
}
impl Work {
    pub fn optimized(&self) -> bool {
        self.enabled
    }
    pub fn begin_tick(&self) {
        *self.counters.borrow_mut() = Counters::default();
        self.shared_attempted.set(false);
        self.shared_current.set(false);
    }
    pub(crate) fn prepare_shared(
        &self,
        bounds: impl FnOnce() -> Option<Aabb3d>,
        scan: impl FnOnce(Aabb3d, &mut StaticOccupancy) -> bool,
    ) {
        if !self.shared_enabled || self.shared_attempted.replace(true) {
            return;
        }
        let Some(bounds) = bounds() else { return };
        let start = Instant::now();
        let mut shared = self.shared.borrow_mut();
        let bounds = shared.covering_bounds(bounds);
        if StaticOccupancy::layout(bounds).is_none() {
            return;
        }
        self.shared_current.set(scan(bounds, &mut shared));
        let mut counters = self.counters.borrow_mut();
        counters.shared_scans += 1;
        counters.shared_scan_ms += start.elapsed().as_secs_f64() * 1000.;
    }

    pub(crate) fn shared_clear(&self, from: Vec3, to: Vec3) -> bool {
        let clear = self.shared_current.get() && self.shared.borrow().clear(from, to);
        if clear {
            self.counters.borrow_mut().shared_clear_segments += 1;
        }
        clear
    }

    pub(crate) fn allows_reuse(&self, tick: u64, rider: usize, last_tick: u64) -> bool {
        self.replan_period > 1
            && !(tick + rider as u64).is_multiple_of(self.replan_period)
            && tick.saturating_sub(last_tick) < self.replan_period
    }

    pub(crate) fn record_replan(&self, reused: bool, immediate: bool) {
        let mut counters = self.counters.borrow_mut();
        counters.plan_reuses += u64::from(reused);
        counters.plan_searches += u64::from(!reused);
        counters.immediate_replans += u64::from(immediate);
    }

    /// Flush on every rider: owner, origin, road frame and static world must match.
    pub fn begin_rider(&self) {
        self.cache.borrow_mut().clear();
        self.segments.borrow_mut().clear();
    }
    pub fn static_clear(
        &self,
        goal: f32,
        speed: f32,
        solve: impl FnOnce(&Cell<u64>) -> bool,
    ) -> bool {
        self.counters.borrow_mut().static_requests += 1;
        if self.enabled
            && let Some(e) = self
                .cache
                .borrow()
                .iter()
                .find(|e| e.goal == goal.to_bits() && e.speed == speed.to_bits())
        {
            self.counters.borrow_mut().static_cache_hits += 1;
            return e.clear;
        }
        let start = Instant::now();
        let casts = Cell::new(0);
        let clear = solve(&casts);
        let mut c = self.counters.borrow_mut();
        c.static_ms += start.elapsed().as_secs_f64() * 1000.;
        c.static_casts += casts.get();
        if self.enabled && self.cache.borrow().len() < 256 {
            self.cache.borrow_mut().push(Entry {
                goal: goal.to_bits(),
                speed: speed.to_bits(),
                clear,
            });
        }
        clear
    }
    /// Reuse only an exact sweep within the rider's unchanged static snapshot.
    /// With tracing on, blocked sweeps still run so every requested hit is reported.
    pub fn segment_clear(
        &self,
        from: bevy::prelude::Vec3,
        to: bevy::prelude::Vec3,
        tracing: bool,
        solve: impl FnOnce() -> bool,
    ) -> bool {
        if !self.enabled {
            return solve();
        }
        let key = [from.x, from.y, from.z, to.x, to.y, to.z].map(f32::to_bits);
        let store_blocked = self.reuse_blocked && !tracing;
        if let Some(&clear) = self.segments.borrow().get(&key)
            && (clear || store_blocked)
        {
            if !clear {
                self.counters.borrow_mut().static_blocked_cache_hits += 1;
            }
            return clear;
        }
        let clear = solve();
        if (clear || store_blocked) && self.segments.borrow().len() < 512 {
            self.segments.borrow_mut().insert(key, clear);
        }
        clear
    }

    pub fn finish(&self, start: Instant) -> Counters {
        let mut c = *self.counters.borrow();
        c.drive_ms = start.elapsed().as_secs_f64() * 1000.;
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduled_plan_searches_are_staggered_at_ten_or_twenty_hertz() {
        for period in [3, 6] {
            let work = Work {
                replan_period: period,
                ..Work::default()
            };
            for tick in 60..120 {
                let due = (1..=12)
                    .filter(|&id| !work.allows_reuse(tick, id, tick - 1))
                    .count();
                assert_eq!(due, 12 / period as usize);
            }
            assert!(!work.allows_reuse(100, 0, 100 - period));
        }
    }

    #[test]
    fn hills_corridor_envelope_is_conservative_at_representative_stations() {
        let track = crate::track::Track::hills().unwrap();
        let baked = HillsCorridors::bake(&track);
        for station in (-300..track.length as i32).step_by(7) {
            let station = station as f32;
            let center = track.street_offset(0, station);
            let nominal = crate::game::ai_free_bounds(&track, 0, station);
            let bounds = baked.bounds(station, center).unwrap();
            assert!(bounds.0 >= nominal.0 - 0.001);
            assert!(bounds.1 <= nominal.1 + 0.001);
        }
    }

    #[test]
    fn shared_scan_runs_once_and_only_certifies_covered_empty_space() {
        let work = Work {
            shared_enabled: true,
            ..Work::default()
        };
        let bounds = Aabb3d {
            min: Vec3::splat(-12.).into(),
            max: Vec3::splat(12.).into(),
        };
        work.begin_tick();
        work.prepare_shared(
            || Some(bounds),
            |bounds, scene| {
                assert!(scene.reset(bounds));
                scene.mark(Vec3::new(0., -1., 0.), Vec3::new(2., 4., 2.));
                true
            },
        );
        work.begin_rider();
        work.prepare_shared(|| Some(bounds), |_, _| panic!("second world scan"));
        assert!(work.shared_clear(Vec3::new(-9., 0., -8.), Vec3::new(-5., 0., -8.)));
        assert!(!work.shared_clear(Vec3::new(-2., 0., 1.), Vec3::new(5., 0., 1.)));
        assert!(!work.shared_clear(Vec3::new(63., 0., 0.), Vec3::new(64., 0., 0.)));
        assert_eq!(work.counters.borrow().shared_scans, 1);
        work.begin_tick();
        assert!(!work.shared_clear(Vec3::new(-9., 0., -8.), Vec3::new(-5., 0., -8.)));
    }

    #[test]
    fn refresh_tracks_static_shape_replacement_and_height_separation() {
        use bevy_rapier3d::rapier::geometry::ColliderBuilder;
        let mut world = bevy::prelude::World::new();
        let entity = world.spawn_empty().id();
        let bounds = Aabb3d {
            min: Vec3::splat(-32.).into(),
            max: Vec3::splat(32.).into(),
        };
        let mut occupancy = StaticOccupancy::default();
        let collider = ColliderBuilder::cuboid(1., 1., 1.).build();
        let bridge = ColliderBuilder::cuboid(1., 1., 1.)
            .translation(Vec3::Y * 8.)
            .build();
        let (from, to) = (Vec3::X * -2., Vec3::X * 2.);
        assert!(occupancy.refresh(bounds, [(entity, &collider)].into_iter()));
        assert!(!occupancy.clear(from, to));
        assert!(occupancy.refresh(bounds, [(entity, &bridge)].into_iter()));
        assert!(occupancy.clear(from, to));
        assert!(occupancy.refresh(bounds, std::iter::empty()));
        assert!(occupancy.clear(from + Vec3::Y * 8., to + Vec3::Y * 8.));
    }

    #[test]
    fn batched_mesh_marks_both_solids_and_leaves_the_road_gap_clear() {
        use bevy_rapier3d::rapier::geometry::ColliderBuilder;
        let mut occupancy = StaticOccupancy::default();
        assert!(occupancy.reset(Aabb3d {
            min: Vec3::splat(-40.).into(),
            max: Vec3::splat(40.).into(),
        }));
        let collider = ColliderBuilder::trimesh(
            vec![
                Vec3::new(-25., 0., 0.),
                Vec3::new(-24., 0., 2.),
                Vec3::new(-22., 2., 0.),
                Vec3::new(20., 0., 0.),
                Vec3::new(22., 0., 2.),
                Vec3::new(24., 2., 0.),
            ],
            vec![[0, 1, 2], [3, 4, 5]],
        )
        .unwrap()
        .translation(Vec3::new(0., 0., 8.))
        .build();
        occupancy.mark_shape(collider.shape(), collider.position());
        let clear = |x| occupancy.clear(Vec3::new(x, 1., 6.), Vec3::new(x, 1., 10.));
        assert!(clear(0.));
        assert!(!clear(-24.));
        assert!(!clear(22.));
    }

    #[test]
    fn blocked_sweeps_are_reused_only_for_an_exact_untraced_segment_in_that_snapshot() {
        let work = Work {
            enabled: true,
            reuse_blocked: true,
            ..Work::default()
        };
        assert!(!work.segment_clear(Vec3::ZERO, Vec3::X, false, || false));
        assert!(!work.segment_clear(Vec3::ZERO, Vec3::X, false, || panic!(
            "duplicate blocked cast"
        )));
        assert!(!work.segment_clear(Vec3::ZERO, Vec3::X, true, || false));
        assert!(work.segment_clear(Vec3::X, Vec3::ZERO, false, || true));
        assert_eq!(work.counters.borrow().static_blocked_cache_hits, 1);
        work.begin_rider();
        assert!(work.segment_clear(Vec3::ZERO, Vec3::X, false, || true));
    }

    #[test]
    fn clear_segments_are_reused_only_for_exact_endpoints_and_the_current_rider() {
        let work = Work {
            enabled: true,
            ..Work::default()
        };
        assert!(work.segment_clear(Vec3::ZERO, Vec3::X, true, || true));
        assert!(work.segment_clear(Vec3::ZERO, Vec3::X, true, || panic!(
            "duplicate sphere cast"
        )));
        assert!(!work.segment_clear(Vec3::X, Vec3::ZERO, true, || false));
        assert!(work.segment_clear(Vec3::X, Vec3::ZERO, true, || true));
        work.begin_rider();
        assert!(!work.segment_clear(Vec3::ZERO, Vec3::X, true, || false));
    }

    #[test]
    fn exact_static_results_are_reused_only_within_one_rider_snapshot() {
        let work = Work {
            enabled: true,
            ..Work::default()
        };
        let calls = Cell::new(0);
        let solve = |casts: &Cell<u64>| {
            calls.set(calls.get() + 1);
            casts.set(12);
            false
        };
        assert!(!work.static_clear(2., 30., solve));
        assert!(!work.static_clear(2., 30., |_| panic!("duplicate world query")));
        assert!(!work.static_clear(2., 29., solve));
        work.begin_rider();
        assert!(work.static_clear(2., 30., |_| true));
        assert_eq!(calls.get(), 2);
        assert_eq!(work.counters.borrow().static_casts, 24);
        assert_eq!(work.counters.borrow().static_cache_hits, 1);
        work.begin_tick();
        work.begin_rider();
        assert!(!work.static_clear(2., 30., solve));
        assert_eq!(work.counters.borrow().static_cache_hits, 0);
    }
}
