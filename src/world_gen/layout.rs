//! Deterministic placement; impossible quotas fail with a section/building report.
use super::{WorldPlan, spatial::Rect};
use crate::track::Track;
use bevy::prelude::*;
use rr_content::world::Row;
use serde::Serialize;

pub const CONSTRUCTION_JSON: &str = include_str!("../../content/world/valley_construction_v1.json");
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)]
pub struct Construction(rr_content::construction::Construction);
impl std::ops::Deref for Construction {
    type Target = rr_content::construction::Construction;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Construction {
    pub fn from_json(json: &str, track: &Track) -> Result<Self, String> {
        rr_content::construction::Construction::from_json(
            json,
            track.definition.id.as_str(),
            track.length,
        )
        .map(Self)
    }
    pub fn row_half(&self, track: &Track, d: f32) -> f32 {
        track.half_width(d)
            + track.definition.verge_width
            + if (self.sidewalk_start_m..=self.sidewalk_end_m).contains(&d) {
                self.sidewalk_width_m
            } else {
                0.0
            }
    }
    pub fn junction_at(&self, d: f32, extra: f32) -> bool {
        self.junctions
            .iter()
            .any(|j| (j.station_m - d).abs() < j.half_width_m + extra)
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct Building {
    pub id: String,
    pub lot_id: String,
    pub section_id: String,
    pub archetype_id: String,
    pub row: Row,
    pub station_m: f32,
    pub lane_m: f32,
    pub position: [f32; 3],
    pub yaw: f32,
    pub size: [f32; 3],
    pub floors: u32,
    pub footprint: Rect,
    pub lot: Rect,
    pub foundation_m: f32,
    pub raw_relief_m: f32,
    pub footing: String,
    pub access: [f32; 3],
    pub section_index: usize,
}
#[derive(Clone, Debug, Serialize)]
pub struct Field {
    pub id: String,
    pub start_m: f32,
    pub end_m: f32,
    pub side: f32,
    pub outer_m: f32,
}
#[derive(Clone, Debug, Serialize)]
pub struct Alley {
    pub section_id: String,
    pub start_m: f32,
    pub end_m: f32,
    pub lane_m: f32,
    pub width_m: f32,
}
#[derive(Resource, Clone, Debug, Serialize)]
pub struct WorldLayout {
    pub settings: Construction,
    pub buildings: Vec<Building>,
    pub fields: Vec<Field>,
    pub alleys: Vec<Alley>,
    pub junction_rects: Vec<Rect>,
    pub road_reservations: Vec<Rect>,
    pub access_reservations: Vec<Rect>,
    pub repairs: usize,
}
pub fn horizontal_frame(track: &Track, d: f32) -> (Vec3, Quat) {
    let (p, q) = track.sample(d, 0.0);
    let f = (q * Vec3::Z).with_y(0.0).normalize();
    (p, Quat::from_rotation_y(f.x.atan2(f.z)))
}
fn noise(id: &str, salt: u32) -> f32 {
    let h = id
        .bytes()
        .fold(u64::from(salt) + 0xcbf29ce484222325, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
        });
    (h % 10000) as f32 / 10000.0
}
impl WorldLayout {
    pub fn generate(
        plan: &WorldPlan,
        track: &Track,
        settings: Construction,
    ) -> Result<Self, String> {
        let mut result = Self {
            settings,
            buildings: Vec::with_capacity(plan.assignments.len()),
            fields: Vec::new(),
            alleys: Vec::new(),
            junction_rects: Vec::new(),
            road_reservations: Vec::new(),
            access_reservations: Vec::new(),
            repairs: 0,
        };
        for j in &result.settings.junctions {
            let (p, q) = horizontal_frame(track, j.station_m);
            result.junction_rects.push(Rect::new(
                p,
                q,
                Vec2::new(j.extent_m, j.half_width_m + 4.0),
            ));
        }
        for d in (0..track.length.ceil() as u32).step_by(8) {
            let d = d as f32;
            let (p, q) = horizontal_frame(track, d + 4.0);
            result.road_reservations.push(Rect::new(
                p,
                q,
                Vec2::new(result.settings.row_half(track, d) + 0.75, 5.5),
            ));
        }
        for i in 0..result.settings.field_count {
            let length = result.settings.field_end_m / (result.settings.field_count / 2) as f32;
            let n = i / 2;
            result.fields.push(Field {
                id: format!("FIELD-{:02}", i + 1),
                start_m: n as f32 * length,
                end_m: ((n + 1) as f32 * length).min(result.settings.field_end_m),
                side: if i % 2 == 0 { -1.0 } else { 1.0 },
                outer_m: 120.0,
            });
        }
        for (section_index, section) in plan.definition.sections.iter().enumerate() {
            let rules = plan
                .definition
                .lot_rules
                .iter()
                .find(|r| r.id == section.lot_rules_id)
                .expect("validated lot rules");
            let entries: Vec<_> = plan
                .assignments
                .iter()
                .filter(|a| a.section_id == section.id)
                .collect();
            let max_depth = entries
                .iter()
                .map(|e| {
                    let a = plan
                        .archetypes
                        .archetypes
                        .iter()
                        .find(|a| a.id == e.archetype_id)
                        .unwrap();
                    (a.footprint_depth_m.min_m + 4.0).min(a.footprint_depth_m.max_m)
                })
                .fold(0.0, f32::max);
            let row_half = result
                .settings
                .row_half(track, (section.start_m + section.end_m) * 0.5);
            let margin = rules
                .earthwork_margin_m
                .max(result.settings.pad_flat_margin_m + result.settings.pad_blend_m);
            let alley = row_half
                + rules.setback_m.max_m.min(25.0)
                + max_depth
                + margin * 2.0
                + rules.secondary_access_width_m;
            if section.secondary > 0 {
                for side in [-1.0, 1.0] {
                    result.alleys.push(Alley {
                        section_id: section.id.clone(),
                        start_m: section.start_m,
                        end_m: section.end_m,
                        lane_m: side * alley,
                        width_m: rules.secondary_access_width_m,
                    });
                }
            }
            let mut row_indices = [0usize; 2];
            for entry in entries {
                let row = if entry.row == Row::Near { 0 } else { 1 };
                let index = row_indices[row];
                row_indices[row] += 1;
                let count = if row == 0 {
                    section.near
                } else {
                    section.secondary
                } as usize;
                let side = if index % 2 == 0 { -1.0 } else { 1.0 };
                let side_count = (count + usize::from(index % 2 == 0)) / 2;
                let target = section.start_m
                    + (section.end_m - section.start_m) * (index / 2 + 1) as f32
                        / (side_count + 1) as f32;
                let a = plan
                    .archetypes
                    .archetypes
                    .iter()
                    .find(|a| a.id == entry.archetype_id)
                    .expect("validated archetype");
                let width = a.footprint_width_m.min_m
                    + (a.footprint_width_m.max_m - a.footprint_width_m.min_m).min(3.0)
                        * noise(&entry.id, 1);
                let depth = a.footprint_depth_m.min_m
                    + (a.footprint_depth_m.max_m - a.footprint_depth_m.min_m).min(4.0)
                        * noise(&entry.id, 2);
                let roof_height = match a.roof {
                    rr_content::world::Roof::Flat => 0.3,
                    rr_content::world::Roof::Pitched => 1.3,
                    rr_content::world::Roof::Sawtooth => 1.0,
                };
                let low = ((a.height_min_m - roof_height) / a.floor_height_m)
                    .ceil()
                    .max(1.0) as u32;
                let high = ((a.height_max_m - roof_height) / a.floor_height_m)
                    .floor()
                    .max(low as f32) as u32;
                let floors = low + ((high - low + 1) as f32 * noise(&entry.id, 3)).floor() as u32;
                let height = floors as f32 * a.floor_height_m + roof_height;
                let frontage = rules.frontage_m.min_m.max(width + margin * 2.0);
                let lot_depth = rules.depth_m.min_m.max(depth + margin * 2.0);
                let lot_front = if row == 0 {
                    row_half
                        + rules.setback_m.min_m.max(1.0)
                        + noise(&entry.id, 4)
                            * (rules.setback_m.max_m - rules.setback_m.min_m.max(1.0))
                                .clamp(0.0, 3.0)
                } else {
                    alley + rules.secondary_access_width_m * 0.5 + 0.5
                };
                let lane = side * (lot_front + lot_depth * 0.5);
                let mut found = None;
                // Bounded oversupply search, nearest target first; never skip a quota.
                for attempt in 0..240 {
                    let shift = if attempt == 0 {
                        0.0
                    } else {
                        ((attempt + 1) / 2) as f32 * 3.0 * if attempt % 2 == 1 { 1.0 } else { -1.0 }
                    };
                    let d = target + shift;
                    if d - frontage * 0.5 < section.start_m + 3.0
                        || d + frontage * 0.5 > section.end_m - 3.0
                    {
                        continue;
                    }
                    let (road, q) = horizontal_frame(track, d);
                    let p = road - q * Vec3::X * lane;
                    let footprint = Rect::new(p, q, Vec2::new(depth * 0.5, width * 0.5));
                    let lot = Rect::new(p, q, Vec2::new(lot_depth * 0.5, frontage * 0.5));
                    if result.road_reservations.iter().any(|r| r.overlaps(&lot))
                        || result.junction_rects.iter().any(|r| r.overlaps(&lot))
                        || result.buildings.iter().any(|b| b.lot.overlaps(&lot))
                    {
                        continue;
                    }
                    let corners = footprint.corners();
                    let c = Vec2::from_array(footprint.center);
                    let samples = [
                        corners[0],
                        corners[1],
                        corners[2],
                        corners[3],
                        c,
                        (corners[0] + corners[1]) * 0.5,
                        (corners[1] + corners[2]) * 0.5,
                        (corners[2] + corners[3]) * 0.5,
                        (corners[3] + corners[0]) * 0.5,
                    ];
                    let heights = samples.map(|v| track.base_terrain_height(v.x, v.y));
                    let min = heights.iter().copied().fold(f32::INFINITY, f32::min);
                    let max = heights.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                    let relief = max - min;
                    if relief > result.settings.max_retained_relief_m {
                        continue;
                    }
                    let datum = track.base_terrain_height(p.x, p.z);
                    let access = road - q * Vec3::X * (lane - side * (depth * 0.5 + margin + 1.5));
                    found = Some(Building {
                        id: entry.id.clone(),
                        lot_id: format!("LOT-{}", entry.id),
                        section_id: entry.section_id.clone(),
                        archetype_id: entry.archetype_id.clone(),
                        row: entry.row.clone(),
                        station_m: d,
                        lane_m: lane,
                        position: [p.x, datum, p.z],
                        yaw: (q * Vec3::Z).x.atan2((q * Vec3::Z).z),
                        size: [depth, height, width],
                        floors,
                        footprint,
                        lot,
                        foundation_m: datum,
                        raw_relief_m: relief,
                        footing: if relief > rules.pad_max_raw_relief_m {
                            "retained_level_pad"
                        } else {
                            "level_pad"
                        }
                        .into(),
                        access: [
                            access.x,
                            track.base_terrain_height(access.x, access.z),
                            access.z,
                        ],
                        section_index,
                    });
                    if attempt > 0 {
                        result.repairs += 1;
                    }
                    break;
                }
                result.buildings.push(found.ok_or_else(|| {
                    format!(
                        "{} / {}: no legal {:?} parcel after 240 candidates (target {target:.1} m)",
                        section.id, entry.id, entry.row
                    )
                })?);
            }
        }
        for alley in &result.alleys {
            let mut d = alley.start_m;
            while d < alley.end_m {
                let next = (d + 8.0).min(alley.end_m);
                let (p, q) = horizontal_frame(track, (d + next) * 0.5);
                result.access_reservations.push(Rect::new(
                    p - q * Vec3::X * alley.lane_m,
                    q,
                    Vec2::new(alley.width_m * 0.5 + 1.0, (next - d) * 0.5 + 1.0),
                ));
                d = next;
            }
        }
        for b in &result.buildings {
            let (p, q) = horizontal_frame(track, b.station_m);
            let side = b.lane_m.signum();
            let start = if b.row == Row::Near {
                result.settings.row_half(track, b.station_m)
            } else {
                result
                    .alleys
                    .iter()
                    .find(|a| a.section_id == b.section_id && a.lane_m.signum() == side)
                    .expect("secondary alley")
                    .lane_m
                    .abs()
            };
            let end = b.lane_m.abs() - b.size[0] * 0.5 - 0.6;
            result.access_reservations.push(Rect::new(
                p - q * Vec3::X * side * (start + end) * 0.5,
                q,
                Vec2::new((end - start).abs() * 0.5, 1.5),
            ));
        }
        result.validate(plan, track)?;
        Ok(result)
    }
    pub fn validate(&self, plan: &WorldPlan, track: &Track) -> Result<(), String> {
        if self.buildings.len() != plan.assignments.len() {
            return Err("layout quota mismatch".into());
        }
        let mut ids = std::collections::HashSet::new();
        for (i, b) in self.buildings.iter().enumerate() {
            let assigned = plan
                .assignments
                .iter()
                .find(|a| a.id == b.id)
                .ok_or_else(|| format!("{}: unassigned building", b.id))?;
            let archetype = plan
                .archetypes
                .archetypes
                .iter()
                .find(|a| a.id == b.archetype_id)
                .ok_or_else(|| format!("{}: unknown archetype", b.id))?;
            if assigned.archetype_id != b.archetype_id
                || assigned.section_id != b.section_id
                || assigned.row != b.row
                || b.size[1] < archetype.height_min_m
                || b.size[1] > archetype.height_max_m
                || b.floors == 0
                || !b.foundation_m.is_finite()
                || !b.raw_relief_m.is_finite()
                || b.raw_relief_m > self.settings.max_retained_relief_m
                || !b
                    .footprint
                    .corners()
                    .iter()
                    .all(|p| b.lot.contains(*p, 0.0))
            {
                return Err(format!(
                    "{}: invalid assignment, dimensions or unsupported foundation",
                    b.id
                ));
            }
            if !ids.insert(&b.id)
                || !b
                    .position
                    .iter()
                    .chain(b.size.iter())
                    .all(|v| v.is_finite())
                || b.size.iter().any(|v| *v <= 0.0)
                || plan.section_at(b.station_m, track) != Some(b.section_index)
                || self
                    .road_reservations
                    .iter()
                    .chain(&self.junction_rects)
                    .any(|r| r.overlaps(&b.lot))
                || self.buildings[..i]
                    .iter()
                    .any(|other| other.lot.overlaps(&b.lot))
            {
                return Err(format!("{}: invalid/overlapping layout footprint", b.id));
            }
        }
        Ok(())
    }
    pub fn scenery_allowed(&self, point: Vec3, radius: f32) -> bool {
        !self
            .buildings
            .iter()
            .any(|b| b.lot.contains(point.xz(), radius + 1.0))
            && !self
                .junction_rects
                .iter()
                .any(|r| r.contains(point.xz(), radius))
            && !self
                .access_reservations
                .iter()
                .any(|r| r.contains(point.xz(), radius))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_spatial_layout_is_complete_and_repeatable() {
        let track = Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap();
        let plan = WorldPlan::from_json(
            super::super::WORLD_JSON,
            super::super::ARCHETYPES_JSON,
            &track,
        )
        .unwrap();
        let load = || {
            WorldLayout::generate(
                &plan,
                &track,
                Construction::from_json(CONSTRUCTION_JSON, &track).unwrap(),
            )
            .unwrap()
        };
        let a = load();
        let b = load();
        assert_eq!(a.buildings.len(), 247);
        assert_eq!(a.fields.len(), 42);
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&b).unwrap()
        );
        println!(
            "LAYOUT {} structures; {} relocated candidates; {} retained pads",
            a.buildings.len(),
            a.repairs,
            a.buildings
                .iter()
                .filter(|b| b.footing == "retained_level_pad")
                .count()
        );
    }
}
