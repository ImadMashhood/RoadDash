//! Bounded visibility routes around actual polygon rings, not intrusive AABB proxies.
use bevy::prelude::*;
use rr_content::metropolis::Metropolis;
use std::collections::{BTreeSet, HashMap};
#[derive(Resource)]
pub struct Navigation {
    rings: Vec<Vec<Vec2>>,
    prop_rings: Vec<usize>,
    original_props: Vec<Vec<Vec2>>,
    owners: Vec<usize>,
    cells: HashMap<(i32, i32), Vec<usize>>,
}
fn cell(p: Vec2) -> (i32, i32) {
    ((p.x / 32.).floor() as i32, (p.y / 32.).floor() as i32)
}
fn distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let v = b - a;
    p.distance(a + v * ((p - a).dot(v) / v.length_squared().max(1e-9)).clamp(0., 1.))
}
fn intersects(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
    let ab = b - a;
    let cd = d - c;
    ab.perp_dot(c - a) * ab.perp_dot(d - a) <= 0. && cd.perp_dot(a - c) * cd.perp_dot(b - c) <= 0.
}
impl Navigation {
    pub fn new(m: &Metropolis) -> Self {
        let mut result = Self {
            rings: vec![],
            prop_rings: vec![],
            original_props: vec![],
            owners: vec![],
            cells: HashMap::new(),
        };
        for (owner, b) in m.buildings.iter().enumerate() {
            for ring in &b.shape.rings {
                let ring: Vec<_> = ring.iter().map(|p| Vec2::from_array(*p)).collect();
                result.add_ring(owner, ring);
            }
        }
        for (i, p) in m.street_props.iter().enumerate() {
            let [w, d] = if p.kind == "broadleaf" {
                [0.7, 0.7]
            } else {
                [p.size[0], p.size[2]]
            };
            let (s, c) = p.heading.sin_cos();
            let ring: Vec<_> = [
                [-w / 2., -d / 2.],
                [w / 2., -d / 2.],
                [w / 2., d / 2.],
                [-w / 2., d / 2.],
            ]
            .into_iter()
            .map(|[x, z]| Vec2::new(p.position[0] + c * x + s * z, p.position[1] - s * x + c * z))
            .collect();
            result.prop_rings.push(result.rings.len());
            result.original_props.push(ring.clone());
            result.add_ring(m.buildings.len() + i, ring);
        }
        for (i, p) in m.parked_cars.iter().enumerate() {
            let (s, c) = p.heading.sin_cos();
            let ring = [[-0.9, -1.9], [0.9, -1.9], [0.9, 1.9], [-0.9, 1.9]]
                .into_iter()
                .map(|[x, z]| {
                    Vec2::new(p.position[0] + c * x + s * z, p.position[1] - s * x + c * z)
                })
                .collect();
            result.add_ring(m.buildings.len() + m.street_props.len() + i, ring);
        }
        // Civic footprints remain navigational obstacles independent of detail.
        // The hall foundation and fountain basin are solid; cafe patios reserve
        // their occupied furniture area while the authored public walk stays open.
        let first = m.buildings.len() + m.street_props.len() + m.parked_cars.len();
        for (i, site) in m.civic_sites.iter().enumerate() {
            let (sin, cos) = site.heading.sin_cos();
            let world = |p: Vec2| {
                Vec2::new(
                    site.position[0] + cos * p.x + sin * p.y,
                    site.position[1] - sin * p.x + cos * p.y,
                )
            };
            let ring = if site.kind == "park" {
                (0..32)
                    .map(|j| {
                        let a = j as f32 / 32. * std::f32::consts::TAU;
                        world(Vec2::new(a.cos(), a.sin()) * 8.5)
                    })
                    .collect()
            } else {
                let size = if site.kind == "city_hall" {
                    Vec2::new(62., 46.) * super::civic::HALL_SCALE
                } else {
                    Vec2::from_array(site.size)
                };
                [
                    Vec2::new(-1., -1.),
                    Vec2::new(1., -1.),
                    Vec2::new(1., 1.),
                    Vec2::new(-1., 1.),
                ]
                .map(|p| world(p * size * 0.5))
                .to_vec()
            };
            result.add_ring(first + i, ring);
            for [x, z] in &site.trees {
                result.add_ring(
                    first + i,
                    [
                        Vec2::new(-0.5, -0.5),
                        Vec2::new(0.5, -0.5),
                        Vec2::new(0.5, 0.5),
                        Vec2::new(-0.5, 0.5),
                    ]
                    .map(|v| v + Vec2::new(*x, *z))
                    .to_vec(),
                );
            }

            if site.kind == "park" {
                let mut furniture = vec![(Vec2::new(24., 130.), Vec2::splat(14.))];
                for side in [-1., 1.] {
                    for z in [-100., -50., 50., 100.] {
                        furniture.extend([
                            (Vec2::new(side * 23., z), Vec2::new(8., 12.)),
                            (Vec2::new(side * 9., z), Vec2::new(4., 1.2)),
                            (Vec2::new(side * 5., z + 12.), Vec2::splat(0.2)),
                        ]);
                    }
                }
                for z in [-35., 35., 105.] {
                    furniture.extend([
                        (Vec2::new(90., z), Vec2::new(8., 12.)),
                        (Vec2::new(75., z), Vec2::new(4., 1.3)),
                    ]);
                }
                for (p, size) in furniture {
                    result.add_ring(
                        first + i,
                        [
                            Vec2::new(-1., -1.),
                            Vec2::new(1., -1.),
                            Vec2::new(1., 1.),
                            Vec2::new(-1., 1.),
                        ]
                        .map(|v| world(p + v * size * 0.5))
                        .to_vec(),
                    );
                }
                for p in super::civic::park_tree_positions(site) {
                    result.add_ring(
                        first + i,
                        [
                            Vec2::new(-0.5, -0.5),
                            Vec2::new(0.5, -0.5),
                            Vec2::new(0.5, 0.5),
                            Vec2::new(-0.5, 0.5),
                        ]
                        .map(|v| world(p + v))
                        .to_vec(),
                    );
                }

                for side in [-1., 1.] {
                    for z in [-20., 20.] {
                        {
                            let (center, size) = (Vec2::new(side * 16., z), Vec2::new(4., 1.3));
                            result.add_ring(
                                first + i,
                                [
                                    Vec2::new(-1., -1.),
                                    Vec2::new(1., -1.),
                                    Vec2::new(1., 1.),
                                    Vec2::new(-1., 1.),
                                ]
                                .map(|p| world(center + p * size * 0.5))
                                .to_vec(),
                            );
                        }
                    }
                }
            }
        }
        result
    }
    fn add_ring(&mut self, owner: usize, ring: Vec<Vec2>) {
        let lo = ring
            .iter()
            .fold(Vec2::splat(f32::INFINITY), |a, p| a.min(*p))
            - Vec2::ONE;
        let hi = ring
            .iter()
            .fold(Vec2::splat(f32::NEG_INFINITY), |a, p| a.max(*p))
            + Vec2::ONE;
        let i = self.rings.len();
        let a = cell(lo);
        let b = cell(hi);
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                self.cells.entry((x, y)).or_default().push(i);
            }
        }
        self.rings.push(ring);
        self.owners.push(owner);
    }
    pub fn move_prop(&mut self, slot: usize, lo: Vec2, hi: Vec2) {
        self.replace_prop(
            slot,
            vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)],
        );
    }
    pub fn reset_props(&mut self) {
        for slot in 0..self.original_props.len() {
            self.replace_prop(slot, self.original_props[slot].clone());
        }
    }
    fn replace_prop(&mut self, slot: usize, ring: Vec<Vec2>) {
        let Some(&id) = self.prop_rings.get(slot) else {
            return;
        };
        if self.rings[id] == ring {
            return;
        }
        let old = &self.rings[id];
        let lo = cell(
            old.iter()
                .copied()
                .fold(Vec2::splat(f32::INFINITY), Vec2::min)
                - Vec2::ONE,
        );
        let hi = cell(
            old.iter()
                .copied()
                .fold(Vec2::splat(f32::NEG_INFINITY), Vec2::max)
                + Vec2::ONE,
        );
        for x in lo.0..=hi.0 {
            for y in lo.1..=hi.1 {
                if let Some(v) = self.cells.get_mut(&(x, y)) {
                    v.retain(|i| *i != id);
                }
            }
        }
        self.rings[id] = ring;
        let lo = cell(
            self.rings[id]
                .iter()
                .copied()
                .fold(Vec2::splat(f32::INFINITY), Vec2::min)
                - Vec2::ONE,
        );
        let hi = cell(
            self.rings[id]
                .iter()
                .copied()
                .fold(Vec2::splat(f32::NEG_INFINITY), Vec2::max)
                + Vec2::ONE,
        );
        for x in lo.0..=hi.0 {
            for y in lo.1..=hi.1 {
                self.cells.entry((x, y)).or_default().push(id);
            }
        }
    }

    fn nearby(&self, a: Vec2, b: Vec2) -> BTreeSet<usize> {
        let lo = cell(a.min(b) - Vec2::ONE);
        let hi = cell(a.max(b) + Vec2::ONE);
        let mut ids = BTreeSet::new();
        for x in lo.0..=hi.0 {
            for y in lo.1..=hi.1 {
                if let Some(v) = self.cells.get(&(x, y)) {
                    ids.extend(v);
                }
            }
        }
        ids
    }
    pub fn clear(&self, a: Vec2, b: Vec2) -> bool {
        let mut inside_owners = BTreeSet::new();
        for id in self.nearby(a, b) {
            let ring = &self.rings[id];
            let mut inside = false;
            for (&c, &d) in ring
                .iter()
                .zip(ring.iter().cycle().skip(1))
                .take(ring.len())
            {
                if (c.y > a.y) != (d.y > a.y) && a.x < (d.x - c.x) * (a.y - c.y) / (d.y - c.y) + c.x
                {
                    inside = !inside;
                }
                let cross = intersects(a, b, c, d)
                    && a.min(b).cmple(c.max(d)).all()
                    && c.min(d).cmple(a.max(b)).all();
                if cross
                    || distance(a, c, d) < 0.4
                    || distance(b, c, d) < 0.4
                    || distance(c, a, b) < 0.4
                    || distance(d, a, b) < 0.4
                {
                    return false;
                }
            }
            if inside && !inside_owners.remove(&self.owners[id]) {
                inside_owners.insert(self.owners[id]);
            }
        }
        inside_owners.is_empty()
    }
    pub fn route(&self, track: &crate::track::Track, from: Vec3, to: Vec3) -> Option<Vec<Vec3>> {
        let a = from.xz();
        let mut goal = to.xz();
        if a.distance(goal) > 350. {
            return None;
        }
        if !self.clear(goal, goal) {
            goal = (0..16)
                .map(|i| {
                    let r = i as f32 * std::f32::consts::TAU / 16.;
                    goal + Vec2::new(r.cos(), r.sin()) * 1.5
                })
                .find(|p| self.clear(*p, *p))?;
        }
        let point = |p: Vec2| Vec3::new(p.x, track.terrain_height(p.x, p.y) + 0.95, p.y);
        if self.clear(a, goal) {
            return Some(vec![point(goal)]);
        }
        let ids = self.nearby(
            a.min(goal) - Vec2::splat(80.),
            a.max(goal) + Vec2::splat(80.),
        );
        let mut candidates = vec![];
        for id in ids {
            for p in &self.rings[id] {
                if distance(*p, a, goal) > 100. {
                    continue;
                }
                for i in 0..8 {
                    let angle = i as f32 * std::f32::consts::TAU / 8.;
                    let p = *p + Vec2::new(angle.cos(), angle.sin());
                    if self.clear(p, p) {
                        candidates.push(p);
                    }
                }
            }
        }
        candidates.sort_by(|x, y| {
            (x.distance(a) + x.distance(goal)).total_cmp(&(y.distance(a) + y.distance(goal)))
        });
        candidates.dedup_by(|x, y| x.distance(*y) < 0.25);
        candidates.truncate(160);
        let mut nodes = vec![a, goal];
        nodes.extend(candidates);
        let n = nodes.len();
        let mut costs = vec![f32::INFINITY; n];
        let mut previous = vec![None; n];
        let mut done = vec![false; n];
        costs[0] = 0.;
        for _ in 0..n {
            let u = (0..n)
                .filter(|i| !done[*i])
                .min_by(|x, y| costs[*x].total_cmp(&costs[*y]))?;
            if !costs[u].is_finite() {
                return None;
            }
            if u == 1 {
                let mut path = vec![point(goal)];
                let mut i = 1;
                while let Some(j) = previous[i] {
                    if j == 0 {
                        break;
                    }
                    path.push(point(nodes[j]));
                    i = j;
                }
                path.reverse();
                return Some(path);
            }
            done[u] = true;
            for v in 0..n {
                if done[v] {
                    continue;
                }
                let cost = costs[u] + nodes[u].distance(nodes[v]);
                if cost < costs[v] && self.clear(nodes[u], nodes[v]) {
                    costs[v] = cost;
                    previous[v] = Some(u);
                }
            }
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moved_wreckage_updates_only_its_spatial_cells_and_reset_restores_footprints() {
        let track = crate::track::Track::metropolis().unwrap();
        let m = track.metropolis.as_ref().unwrap();
        let mut nav = Navigation::new(m);
        let id = nav.prop_rings[0];
        let original = nav.rings[id].clone();
        let distant = Vec2::splat(-5000.);
        assert!(nav.clear(distant, distant));
        nav.move_prop(0, distant - Vec2::ONE, distant + Vec2::ONE);
        assert!(!nav.clear(distant, distant));
        assert_eq!(nav.rings.len(), nav.owners.len());
        nav.reset_props();
        assert_eq!(nav.rings[id], original);
        assert!(nav.clear(distant, distant));
    }
    #[test]
    fn recovery_navigation_accounts_for_every_furniture_footprint() {
        let track = crate::track::Track::metropolis().unwrap();
        let m = track.metropolis.as_ref().unwrap();
        let nav = Navigation::new(m);
        for site in &m.civic_sites {
            let p = Vec2::from_array(site.position);
            assert!(!nav.clear(p, p), "missing civic obstacle {}", site.id);
        }
        for car in &m.parked_cars {
            let q = Vec2::from_array(car.position);
            assert!(!nav.clear(q, q));
        }
        for p in &m.street_props {
            let q = Vec2::from_array(p.position);
            assert!(!nav.clear(q, q), "{}", p.id);
        }
    }
    #[test]
    fn exact_diagonal_boundaries_leave_the_race_corridor_open() {
        let track = crate::track::Track::metropolis().unwrap();
        let nav = Navigation::new(track.metropolis.as_ref().unwrap());
        for route in 0..2 {
            for s in (3200..11900).step_by(80) {
                let a = track.sample_route(route, s as f32, 0.).0;
                let b = track.sample_route(route, s as f32 + 3., 0.).0;
                assert!(nav.clear(a.xz(), b.xz()), "{route} {s}");
            }
        }
    }
}
