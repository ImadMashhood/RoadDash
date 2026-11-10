//! Bounded race-local crop memory. Metres, seconds; no renderer/physics dependency.
use std::collections::BTreeMap;
pub const SIDE: i32 = 16;
pub const CELLS: usize = (SIDE * SIDE) as usize;
pub const MAX_TILES: usize = 2048;
pub type Key = (i32, i32);
#[derive(Clone)]
pub struct Tile {
    pub cells: [[f32; 4]; CELLS],
    pub planted: [bool; CELLS],
    pub dirty: bool,
}
impl Default for Tile {
    fn default() -> Self {
        Self {
            cells: [[0.; 4]; CELLS],
            planted: [false; CELLS],
            dirty: false,
        }
    }
}
#[derive(Default)]
pub struct Field {
    pub tiles: BTreeMap<Key, Tile>,
}
pub fn address(p: [f32; 2]) -> (Key, usize) {
    let x = p[0].floor() as i32;
    let z = p[1].floor() as i32;
    (
        (x.div_euclid(SIDE), z.div_euclid(SIDE)),
        (z.rem_euclid(SIDE) * SIDE + x.rem_euclid(SIDE)) as usize,
    )
}
impl Field {
    pub fn plant(&mut self, p: [f32; 2]) {
        let (key, index) = address(p);
        assert!(self.tiles.contains_key(&key) || self.tiles.len() < MAX_TILES);
        self.tiles.entry(key).or_default().planted[index] = true;
    }
    pub fn reset(&mut self) {
        for t in self.tiles.values_mut() {
            if t.cells.iter().any(|c| c[2] > 0.) {
                t.cells.fill([0.; 4]);
                t.dirty = true;
            }
        }
    }
    /// Sweep a capsule; sample the whole segment so even fast bikes leave connected tracks.
    /// Adapter supplies a physical contact gate; airborne/disabled/unplanted cells never change.
    /// Teleports over 12m are deliberately treated as a new contact, not a trail.
    pub fn stamp(
        &mut self,
        from: [f32; 2],
        to: [f32; 2],
        radius: f32,
        time: f32,
        mut contact: impl FnMut([f32; 2], f32) -> bool,
    ) -> usize {
        if !from
            .into_iter()
            .chain(to)
            .chain([radius, time])
            .all(f32::is_finite)
            || radius <= 0.
        {
            return 0;
        }
        let radius = radius.min(2.);
        let mut from = from;
        let mut dx = to[0] - from[0];
        let mut dz = to[1] - from[1];
        if dx * dx + dz * dz > 144. {
            from = to;
            dx = 0.;
            dz = 0.;
        }
        let len2 = dx * dx + dz * dz;
        let length = len2.sqrt();
        let direction = if length > 0.01 {
            [dx / length, dz / length]
        } else {
            [0.707, 0.707]
        };
        let mut changed = 0;
        for z in ((from[1].min(to[1]) - radius).floor() as i32)
            ..=((from[1].max(to[1]) + radius).floor() as i32)
        {
            for x in ((from[0].min(to[0]) - radius).floor() as i32)
                ..=((from[0].max(to[0]) + radius).floor() as i32)
            {
                let p = [x as f32 + 0.5, z as f32 + 0.5];
                let alpha = if len2 > 0.0001 {
                    (((p[0] - from[0]) * dx + (p[1] - from[1]) * dz) / len2).clamp(0., 1.)
                } else {
                    1.
                };
                let d = ((p[0] - from[0] - dx * alpha).powi(2)
                    + (p[1] - from[1] - dz * alpha).powi(2))
                .sqrt();
                if d > radius {
                    continue;
                }
                let (key, index) = address(p);
                let Some(tile) = self.tiles.get_mut(&key) else {
                    continue;
                };
                if !tile.planted[index] || tile.cells[index][2] > 0. || !contact(p, alpha) {
                    continue;
                }
                // First touch is latched, including direction/time. Standing on it never restarts bending.
                tile.cells[index] = [direction[0], direction[1], 0.94, time];
                tile.dirty = true;
                changed += 1;
            }
        }
        changed
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn field() -> Field {
        let mut f = Field::default();
        for x in -30..31 {
            for z in -5..6 {
                f.plant([x as f32, z as f32]);
            }
        }
        f
    }
    #[test]
    fn sweep_persists_without_gaps_and_reset_preserves_planting() {
        let mut f = field();
        assert!(f.stamp([-10., 0.], [1., 0.], 0.8, 2., |_, _| true) > 10);
        for x in -10..1 {
            let (k, i) = address([x as f32, 0.]);
            assert!(f.tiles[&k].cells[i][2] > 0.);
        }
        assert_eq!(f.stamp([-10., 0.], [1., 0.], 0.8, 7., |_, _| true), 0);
        f.reset();
        assert!(f.tiles.values().all(|t| t.cells.iter().all(|v| v[2] == 0.)));
        assert!(f.stamp([0., 0.], [1., 0.], 0.8, 8., |_, _| true) > 0);
    }
    #[test]
    fn excludes_air_unplanted_and_teleport_corridors() {
        let mut f = field();
        assert_eq!(f.stamp([0., 0.], [5., 0.], 1., 0., |_, _| false), 0);
        assert_eq!(f.stamp([500., 0.], [505., 0.], 1., 0., |_, _| true), 0);
        f.stamp([-20., 0.], [20., 0.], 1., 0., |_, _| true);
        let (k, i) = address([0., 0.]);
        assert_eq!(f.tiles[&k].cells[i][2], 0.);
    }
    #[test]
    fn bounded_benchmark() {
        let mut f = field();
        let start = std::time::Instant::now();
        for tick in 0..36000 {
            f.stamp([0., 0.], [0.1, 0.], 1., tick as f32 / 60., |_, _| false);
        }
        let empty = start.elapsed();
        let start = std::time::Instant::now();
        let mut changes = 0;
        for tick in 0..36000 {
            if tick % 600 == 0 {
                f.reset();
            }
            for body in 0..16 {
                let x = (tick % 50) as f32 - 25.;
                let z = (body % 10) as f32 - 5.;
                changes += f.stamp([x, z], [x + 1., z], 0.8, tick as f32 / 60., |_, _| true);
            }
        }
        println!(
            "crop grid: 36000 inactive sweeps {:?}; 576000 body sweeps {:?}; {} newly damaged cells; {} tile bytes",
            empty,
            start.elapsed(),
            changes,
            f.tiles.len() * CELLS * 16
        );
    }
}
