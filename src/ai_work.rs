//! Per-rider, per-fixed-tick reuse of immutable-world collision forecasts.
use std::{
    cell::{Cell, RefCell},
    time::Instant,
};

#[derive(Clone, Copy, Default, serde::Serialize)]
pub struct Counters {
    pub drive_ms: f64,
    pub static_ms: f64,
    pub static_requests: u64,
    pub static_cache_hits: u64,
    pub static_casts: u64,
}
impl Counters {
    pub fn merge(&mut self, other: Self) {
        self.drive_ms += other.drive_ms;
        self.static_ms += other.static_ms;
        self.static_requests += other.static_requests;
        self.static_cache_hits += other.static_cache_hits;
        self.static_casts += other.static_casts;
    }
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
    enabled: bool,
}
impl Default for Work {
    fn default() -> Self {
        Self {
            cache: RefCell::new(Vec::with_capacity(128)),
            counters: RefCell::default(),
            enabled: std::env::var("ROADDASH_AI_STATIC_CACHE").as_deref() != Ok("0"),
        }
    }
}
impl Work {
    pub fn optimized(&self) -> bool {
        self.enabled
    }
    pub fn begin_tick(&self) {
        *self.counters.borrow_mut() = Counters::default();
    }
    /// Flush on every rider: owner, origin, road frame and static world must match.
    pub fn begin_rider(&self) {
        self.cache.borrow_mut().clear();
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
