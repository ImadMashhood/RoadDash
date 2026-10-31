//! A single earthwork height reference for terrain, buildings and side streets.
use super::{
    layout::{WorldLayout, horizontal_frame},
    spatial::Rect,
};
use crate::track::Track;
use bevy::prelude::*;
use std::collections::HashMap;
#[derive(Clone, Debug)]
struct Pad {
    rect: Rect,
    datum: f32,
    flat: f32,
    blend: f32,
}
#[derive(Clone, Debug)]
struct Street {
    rect: Rect,
    datum: f32,
    bank_slope: f32,
    grade_slope: f32,
    protected_half: f32,
    road_lane: f32,
    inner_blend: f32,
    blend: f32,
}
#[derive(Clone, Debug)]
pub struct Earthworks {
    pads: Vec<Pad>,
    streets: Vec<Street>,
    pad_cells: HashMap<(i32, i32), Vec<usize>>,
    street_cells: HashMap<(i32, i32), Vec<usize>>,
}
fn cell(x: f32, z: f32) -> (i32, i32) {
    ((x / 32.0).floor() as i32, (z / 32.0).floor() as i32)
}
fn index_rect(rect: &Rect, margin: f32, id: usize, cells: &mut HashMap<(i32, i32), Vec<usize>>) {
    let corners = rect.expanded(margin).corners();
    let min = corners
        .iter()
        .copied()
        .fold(Vec2::splat(f32::INFINITY), Vec2::min);
    let max = corners
        .iter()
        .copied()
        .fold(Vec2::splat(f32::NEG_INFINITY), Vec2::max);
    let a = cell(min.x, min.y);
    let b = cell(max.x, max.y);
    for x in a.0..=b.0 {
        for z in a.1..=b.1 {
            cells.entry((x, z)).or_default().push(id);
        }
    }
}
impl Earthworks {
    pub fn new(layout: &WorldLayout, track: &Track) -> Self {
        let mut e = Self {
            pads: Vec::new(),
            streets: Vec::new(),
            pad_cells: HashMap::new(),
            street_cells: HashMap::new(),
        };
        for b in &layout.buildings {
            e.pads.push(Pad {
                rect: b.footprint.clone(),
                datum: b.foundation_m,
                flat: layout.settings.pad_flat_margin_m,
                blend: layout.settings.pad_blend_m,
            });
        }
        for j in &layout.settings.junctions {
            let (p, q) = horizontal_frame(track, j.station_m);
            let (_, bank) = track.sample(j.station_m, 0.0);
            let slope = (bank * Vec3::X).y;
            e.streets.push(Street {
                rect: Rect::new(p, q, Vec2::new(j.extent_m, j.half_width_m)),
                datum: p.y - 0.01,
                bank_slope: slope,
                grade_slope: (bank * Vec3::Z).y,
                protected_half: track.half_width(j.station_m) + 0.5,
                road_lane: 0.0,
                inner_blend: 2.0,
                blend: 3.0,
            });
        }
        let mut d = layout.settings.sidewalk_start_m;
        while d <= layout.settings.sidewalk_end_m {
            if !layout.settings.junction_at(d, 1.0) {
                let (p, q) = horizontal_frame(track, d);
                let (_, bank) = track.sample(d, 0.0);
                for side in [-1.0, 1.0] {
                    let inner = track.half_width(d) + track.definition.verge_width;
                    let lane = side * (inner + layout.settings.sidewalk_width_m * 0.5);
                    let center = p - q * Vec3::X * lane;
                    e.streets.push(Street {
                        rect: Rect::new(
                            center,
                            q,
                            Vec2::new(layout.settings.sidewalk_width_m * 0.5, 5.0),
                        ),
                        datum: p.y - (bank * Vec3::X).y * lane
                            + layout.settings.curb_height_m
                            + 0.005,
                        bank_slope: (bank * Vec3::X).y,
                        grade_slope: (bank * Vec3::Z).y,
                        protected_half: inner,
                        road_lane: lane,
                        inner_blend: 1.0,
                        blend: 3.0,
                    });
                }
            }
            d += 8.0;
        }
        for (i, p) in e.pads.iter().enumerate() {
            index_rect(&p.rect, p.flat + p.blend + 4.0, i, &mut e.pad_cells);
        }
        for (i, s) in e.streets.iter().enumerate() {
            index_rect(&s.rect, s.blend, i, &mut e.street_cells);
        }
        e
    }
    pub fn height(&self, x: f32, z: f32, base: f32) -> f32 {
        let point = Vec2::new(x, z);
        let key = cell(x, z);
        let mut height = base;
        if let Some(ids) = self.street_cells.get(&key) {
            for &i in ids {
                let s = &self.streets[i];
                let local = s.rect.local(point);
                let distance = (local.abs() - Vec2::from_array(s.rect.half))
                    .max_element()
                    .max(0.0);
                let weight = smooth(1.0 - distance / s.blend)
                    * smooth(((local.x - s.road_lane).abs() - s.protected_half) / s.inner_blend);
                if weight > 0.0 {
                    height = height.lerp(
                        s.datum + local.x * s.bank_slope + local.y * s.grade_slope,
                        weight,
                    );
                }
            }
        }
        if let Some(ids) = self.pad_cells.get(&key) {
            for &i in ids {
                let p = &self.pads[i];
                let local = p.rect.local(point).abs();
                let distance = (local - Vec2::from_array(p.rect.half) - Vec2::splat(p.flat))
                    .max_element()
                    .max(0.0);
                let weight = smooth(1.0 - distance / p.blend);
                if weight > 0.0 {
                    height = height.lerp(p.datum, weight);
                }
            }
        }
        height
    }
}
fn smooth(v: f32) -> f32 {
    let t = v.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
impl Earthworks {
    /// Cut the low-resolution terrain at exact pad edges. Both collision and
    /// render use this same topology; flat planes then fill the holes.
    pub fn cut_pads(&self, track: &Track, vertices: &mut Vec<Vec3>, indices: &mut Vec<[u32; 3]>) {
        let mut out = Vec::with_capacity(indices.len());
        for tri in indices.iter().copied() {
            let original = tri.map(|i| vertices[i as usize]);
            let center = (original[0] + original[1] + original[2]) / 3.0;
            let Some(ids) = self.pad_cells.get(&cell(center.x, center.z)) else {
                out.push(tri);
                continue;
            };
            let mut pieces = vec![original.iter().map(|p| p.xz()).collect::<Vec<_>>()];
            let mut changed = false;
            for &id in ids {
                let rect = self.pads[id].rect.expanded(self.pads[id].flat);
                let mut next = Vec::new();
                for poly in pieces {
                    if let Some(cut) = subtract(&poly, &rect) {
                        changed = true;
                        next.extend(cut);
                    } else {
                        next.push(poly);
                    }
                }
                pieces = next;
            }
            if !changed {
                out.push(tri);
                continue;
            }
            for poly in pieces {
                if poly.len() < 3 {
                    continue;
                }
                let base = vertices.len() as u32;
                for p in &poly {
                    vertices.push(Vec3::new(p.x, track.terrain_height(p.x, p.y), p.y));
                }
                for i in 1..poly.len() - 1 {
                    out.push([base, base + i as u32, base + i as u32 + 1]);
                }
            }
        }
        *indices = out;
    }
}
fn split(poly: &[Vec2], rect: &Rect, axis: usize, sign: f32) -> (Vec<Vec2>, Vec<Vec2>) {
    let mut inside = Vec::new();
    let mut outside = Vec::new();
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        let da = rect.local(a)[axis] * sign - rect.half[axis];
        let db = rect.local(b)[axis] * sign - rect.half[axis];
        if da <= 0.0 {
            inside.push(a);
        } else {
            outside.push(a);
        }
        if (da > 0.0) != (db > 0.0) {
            let hit = a.lerp(b, da / (da - db));
            inside.push(hit);
            outside.push(hit);
        }
    }
    (inside, outside)
}
fn subtract(poly: &[Vec2], rect: &Rect) -> Option<Vec<Vec<Vec2>>> {
    let mut remaining = poly.to_vec();
    let mut outside = Vec::new();
    for (axis, sign) in [(0, 1.0), (0, -1.0), (1, 1.0), (1, -1.0)] {
        let (inside, part) = split(&remaining, rect, axis, sign);
        if part.len() >= 3 {
            outside.push(part);
        }
        remaining = inside;
        if remaining.len() < 3 {
            return None;
        }
    }
    let area = (1..remaining.len() - 1)
        .map(|i| {
            (remaining[i] - remaining[0])
                .perp_dot(remaining[i + 1] - remaining[0])
                .abs()
        })
        .sum::<f32>();
    if area < 0.00001 { None } else { Some(outside) }
}

pub fn terrain_grid(
    track: &Track,
    origin: Vec2,
    tile: f32,
    step: f32,
) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let count = (tile / step).round() as usize;
    let mut vertices = Vec::with_capacity((count + 1) * (count + 1));
    let mut indices = Vec::with_capacity(count * count * 2);
    for z in 0..=count {
        for x in 0..=count {
            let p = origin + Vec2::new(x as f32 * step, z as f32 * step);
            vertices.push(Vec3::new(p.x, track.terrain_height(p.x, p.y), p.y));
        }
    }
    for z in 0..count {
        for x in 0..count {
            let a = (z * (count + 1) + x) as u32;
            let b = a + (count + 1) as u32;
            indices.extend([[a, b, a + 1], [a + 1, b, b + 1]]);
        }
    }
    (vertices, indices)
}
