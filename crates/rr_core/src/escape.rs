//! Bounded obstacle-aware rejoin planning; callers supply real swept-body clearance.
use std::{cmp::Reverse, collections::BinaryHeap};
pub fn route(
    start: [f32; 2],
    goal: [f32; 2],
    clear: impl Fn([f32; 2], [f32; 2]) -> bool,
) -> Option<Vec<[f32; 2]>> {
    if clear(start, goal) {
        return Some(vec![goal]);
    }
    const N: usize = 49;
    const MID: usize = N / 2;
    const STEP: f32 = 3.;
    let point = |i: usize| {
        [
            start[0] + (i % N) as f32 * STEP - MID as f32 * STEP,
            start[1] + (i / N) as f32 * STEP - MID as f32 * STEP,
        ]
    };
    let distance =
        |a: [f32; 2], b: [f32; 2]| ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt();
    if distance(start, goal) > 68. {
        return None;
    }
    let first = MID * N + MID;
    let mut costs = vec![f32::INFINITY; N * N];
    let mut prev = vec![None; N * N];
    let mut done = vec![false; N * N];
    let mut heap = BinaryHeap::new();
    costs[first] = 0.;
    heap.push(Reverse((0u32, first)));
    let mut expanded = 0;
    while let Some(Reverse((_, u))) = heap.pop() {
        if done[u] {
            continue;
        }
        done[u] = true;
        expanded += 1;
        if expanded > 1024 {
            return None;
        }
        let a = point(u);
        if distance(a, goal) < 6. && clear(a, goal) {
            let mut path = vec![goal];
            let mut i = u;
            while i != first {
                path.push(point(i));
                i = prev[i]?;
            }
            path.reverse();
            let mut compact = vec![];
            let mut at = start;
            let mut index = 0;
            while index < path.len() {
                let mut next = index;
                for j in (index + 1..path.len()).rev() {
                    if clear(at, path[j]) {
                        next = j;
                        break;
                    }
                }
                compact.push(path[next]);
                at = path[next];
                index = next + 1;
            }
            return Some(compact);
        }
        for dz in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dz == 0 {
                    continue;
                }
                let x = (u % N) as i32 + dx;
                let z = (u / N) as i32 + dz;
                if x < 0 || z < 0 || x >= N as i32 || z >= N as i32 {
                    continue;
                }
                let v = z as usize * N + x as usize;
                if done[v] {
                    continue;
                }
                let b = point(v);
                let cost = costs[u] + distance(a, b);
                if cost < costs[v] && clear(a, b) {
                    costs[v] = cost;
                    prev[v] = Some(u);
                    heap.push(Reverse((((cost + distance(b, goal)) * 1000.) as u32, v)));
                }
            }
        }
    }
    None
}
/// Fixed-tick escape state, independent of rendering and physics interfaces.
#[derive(Clone, Debug, Default)]
pub struct Memory {
    best_progress: Option<f32>,
    stalled: f32,
    retry: f32,
    blocked: f32,
    path: Vec<[f32; 2]>,
    pub heading: Option<[f32; 2]>,
    pub reverse: bool,
    backward: bool,
}
impl Memory {
    pub fn active(&self) -> bool {
        self.heading.is_some()
    }
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        dt: f32,
        position: [f32; 2],
        progress: f32,
        heading: [f32; 2],
        goal: [f32; 2],
        wants_progress: bool,
        budget: &mut usize,
        clear: impl Fn([f32; 2], [f32; 2]) -> bool,
    ) -> Option<(bool, bool)> {
        self.retry = (self.retry - dt).max(0.);
        let best = *self.best_progress.get_or_insert(progress);
        if progress > best + 4. {
            self.best_progress = Some(progress);
            self.stalled = 0.;
        } else {
            self.stalled += dt;
        }
        let threshold = if wants_progress { 3. } else { 10. };
        if self.path.is_empty() && self.stalled > threshold && self.retry == 0. && *budget > 0 {
            *budget -= 1;
            self.retry = 2.;
            self.path = route(position, goal, &clear).unwrap_or_default();
        }
        let distance =
            |a: [f32; 2], b: [f32; 2]| ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt();
        while self
            .path
            .first()
            .is_some_and(|p| distance(*p, position) < 1.2)
        {
            self.path.remove(0);
            self.heading = None;
        }
        let Some(next) = self.path.first().copied() else {
            self.heading = None;
            self.reverse = false;
            return (self.stalled > threshold).then_some((false, false));
        };
        let d = distance(next, position).max(0.001);
        let direction = [(next[0] - position[0]) / d, (next[1] - position[1]) / d];
        let dot = heading[0] * direction[0] + heading[1] * direction[1];
        if self.heading.is_none() {
            self.backward = dot < -0.35;
        }
        let sign = if self.backward { -1. } else { 1. };
        self.heading = Some(direction.map(|v| v * sign));
        let safe = clear(
            position,
            [
                position[0] + direction[0] * d.min(3.),
                position[1] + direction[1] * d.min(3.),
            ],
        );
        self.blocked = if safe { 0. } else { self.blocked + dt };
        if self.blocked > 1.5 {
            self.path.clear();
            self.heading = None;
            self.reverse = false;
            self.retry = 0.5;
            self.stalled = 11.;
            return Some((false, false));
        }
        Some((safe && dot * sign > 0.65, self.backward))
    }
}
#[derive(Clone, Debug, Default)]
pub struct WalkWatch {
    anchor: Option<[f32; 2]>,
    elapsed: f32,
}
impl WalkWatch {
    pub fn stalled(&mut self, dt: f32, p: [f32; 2]) -> bool {
        let a = *self.anchor.get_or_insert(p);
        if (a[0] - p[0]).powi(2) + (a[1] - p[1]).powi(2) > 0.25 {
            self.anchor = Some(p);
            self.elapsed = 0.;
        } else {
            self.elapsed += dt;
        }
        self.elapsed > 2.
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn u_clear(a: [f32; 2], b: [f32; 2]) -> bool {
        (0..=100).all(|i| {
            let t = i as f32 / 100.;
            let x = a[0] + (b[0] - a[0]) * t;
            let z = a[1] + (b[1] - a[1]) * t;
            !((x.abs() < 8. && (8. ..11.).contains(&z))
                || ((5. ..8.).contains(&x.abs()) && (-10. ..11.).contains(&z)))
        })
    }
    #[test]
    fn exits_a_concave_dead_end_before_rejoining_ahead() {
        let start = [0., 0.];
        let path = route(start, [0., 20.], u_clear).unwrap();
        assert!(path.iter().any(|p| p[1] < -10.));
        let mut p = start;
        for q in path {
            assert!(u_clear(p, q));
            p = q;
        }
        assert_eq!(p, [0., 20.]);
    }
    #[test]
    fn impossible_routes_are_bounded_and_never_return_wall_crossings() {
        let calls = std::cell::Cell::new(0);
        assert!(
            route([0., 0.], [0., 20.], |_, _| {
                calls.set(calls.get() + 1);
                false
            })
            .is_none()
        );
        assert!(calls.get() < 20);
    }
    #[test]
    fn already_clear_rejoin_has_no_detour() {
        assert_eq!(
            route([0., 0.], [6., 12.], |_, _| true),
            Some(vec![[6., 12.]])
        );
    }
}

#[cfg(test)]
mod memory_tests {
    use super::*;
    #[test]
    fn short_queues_wait_but_oscillating_backups_do_not_hide_deadlock() {
        let mut m = Memory::default();
        for _ in 0..120 {
            assert!(
                m.tick(
                    1. / 60.,
                    [0., 0.],
                    0.,
                    [0., 1.],
                    [0., 20.],
                    false,
                    &mut 1,
                    |_, _| true
                )
                .is_none()
            );
        }
        for i in 0..200 {
            m.tick(
                1. / 60.,
                [0., if i % 2 == 0 { -3. } else { 0. }],
                0.,
                [0., 1.],
                [0., 20.],
                true,
                &mut 1,
                |_, _| true,
            );
        }
        assert!(m.active());
    }
    #[test]
    fn closing_obstacle_stops_motion_and_invalidates_cached_path() {
        let mut m = Memory::default();
        assert_eq!(
            m.tick(
                4.,
                [0., 0.],
                0.,
                [0., 1.],
                [0., 20.],
                true,
                &mut 1,
                |_, _| true
            ),
            Some((true, false))
        );
        assert_eq!(
            m.tick(
                0.1,
                [0., 0.],
                0.,
                [0., 1.],
                [0., 20.],
                true,
                &mut 1,
                |_, _| false
            ),
            Some((false, false))
        );
        m.tick(
            2.,
            [0., 0.],
            0.,
            [0., 1.],
            [0., 20.],
            true,
            &mut 1,
            |_, _| false,
        );
        assert!(!m.active());
    }
}
