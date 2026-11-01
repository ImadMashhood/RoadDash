//! Additional city blocks. The original 247 named lots remain independently tracked.
use super::{
    WorldPlan,
    layout::{Building, WorldLayout, horizontal_frame},
    spatial::Rect,
};
use crate::track::Track;
use bevy::prelude::*;
use rr_content::world::Row;
use serde::Serialize;
#[derive(Clone, Debug, Serialize)]
pub struct Street {
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub width: f32,
}
impl Street {
    pub fn reservation(&self) -> Rect {
        let a = Vec3::from_array(self.a);
        let b = Vec3::from_array(self.b);
        let delta = b - a;
        Rect::new(
            (a + b) * 0.5,
            Quat::from_rotation_y(delta.x.atan2(delta.z)),
            Vec2::new(self.width * 0.5 + 3.0, delta.xz().length() * 0.5 + 0.5),
        )
    }
}
pub fn generate(layout: &mut WorldLayout, plan: &WorldPlan, track: &Track) {
    // A road-relative grid follows the curved race corridor, with four avenues
    // on each side and cross streets at every block. No new race gates/routes.
    let config = layout.settings.city_grid.clone();
    let start = config.start_m;
    let end = config.end_m;
    let lanes = &config.avenues;
    let point = |d: f32, lane: f32| {
        let (p, q) = horizontal_frame(track, d);
        let mut v = p - q * Vec3::X * lane;
        v.y = track.base_terrain_height(v.x, v.z);
        v
    };
    let mut street = |a: Vec3, b: Vec3| {
        let s = Street {
            a: a.to_array(),
            b: b.to_array(),
            width: 8.0,
        };
        if !layout
            .buildings
            .iter()
            .any(|b| b.lot.overlaps(&s.reservation()))
        {
            layout.grid_streets.push(s);
        }
    };
    for side in [-1.0, 1.0] {
        for &lane in lanes {
            let mut d = start;
            while d < end {
                let next = (d + 8.0).min(end);
                street(point(d, side * lane), point(next, side * lane));
                d = next;
            }
        }
        let mut d = start;
        while d <= end {
            for pair in lanes.windows(2) {
                street(point(d, side * pair[0]), point(d, side * pair[1]));
            }
            d += config.block_length_m;
        }
        // Existing junctions connect the avenue network to the main street.
        for j in &layout.settings.junctions {
            if (start..=end).contains(&j.station_m) {
                street(
                    point(j.station_m, side * j.extent_m),
                    point(j.station_m, side * lanes.last().copied().unwrap()),
                );
            }
        }
    }
    let reservations: Vec<_> = layout
        .grid_streets
        .iter()
        .map(Street::reservation)
        .collect();
    for side in [-1.0, 1.0] {
        for (row, lane) in config.frontage_rows.iter().copied().enumerate() {
            let mut d = start + 14.0;
            while d < end - 12.0 {
                let (p, q) = horizontal_frame(track, d);
                let center = point(d, side * lane);
                let lot = Rect::new(center, q, Vec2::new(9.5, 11.0));
                let legal = !layout
                    .buildings
                    .iter()
                    .chain(&layout.neighborhood)
                    .any(|b| b.lot.overlaps(&lot))
                    && !layout
                        .road_reservations
                        .iter()
                        .chain(&layout.junction_rects)
                        .chain(&layout.access_reservations)
                        .chain(&reservations)
                        .any(|r| r.overlaps(&lot));
                if legal {
                    let section_index = plan.section_at(d, track).unwrap();
                    let section = &plan.definition.sections[section_index];
                    let types: Vec<_> = plan
                        .archetypes
                        .archetypes
                        .iter()
                        .filter(|a| matches!(a.family, rr_content::world::Family::Urban))
                        .collect();
                    let index = layout.neighborhood.len();
                    let a = types[index % types.len()];
                    let floors = 3 + (index % 6) as u32;
                    let height = (floors as f32 * a.floor_height_m + 0.3)
                        .clamp(a.height_min_m, a.height_max_m);
                    let footprint = Rect::new(center, q, Vec2::new(8.0, 9.0));
                    let heights = footprint
                        .corners()
                        .map(|v| track.base_terrain_height(v.x, v.y));
                    let relief = heights.iter().copied().fold(f32::NEG_INFINITY, f32::max)
                        - heights.iter().copied().fold(f32::INFINITY, f32::min);
                    if relief <= layout.settings.max_retained_relief_m {
                        let id = format!(
                            "CITY-{}-{}-{:04}",
                            if side < 0.0 { "L" } else { "R" },
                            row,
                            d as u32
                        );
                        layout.neighborhood.push(Building {
                            id: id.clone(),
                            lot_id: format!("LOT-{id}"),
                            section_id: section.id.clone(),
                            archetype_id: a.id.clone(),
                            row: Row::Secondary,
                            station_m: d,
                            lane_m: side * lane,
                            position: center.to_array(),
                            yaw: (q * Vec3::Z).x.atan2((q * Vec3::Z).z),
                            size: [16.0, height, 18.0],
                            floors,
                            footprint,
                            lot,
                            foundation_m: center.y,
                            raw_relief_m: relief,
                            footing: "city_pad".into(),
                            access: (center + q * Vec3::Z * 11.0).to_array(),
                            section_index,
                        });
                    }
                }
                let _ = p;
                d += config.parcel_pitch_m;
            }
        }
    }
}
