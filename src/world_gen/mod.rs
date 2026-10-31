//! B00-B02 structural generation foundation. No placement or terrain edits yet.
pub mod report;
use crate::track::Track;
use bevy::prelude::*;
use rr_content::world::{ArchetypeCatalog, BuildingAssignment, District, WorldDef};

pub const WORLD_JSON: &str = include_str!("../../content/world/valley_world_v1.json");
pub const ARCHETYPES_JSON: &str = include_str!("../../content/world/building_archetypes_v1.json");
#[derive(Resource)]
pub struct WorldPlan {
    pub definition: WorldDef,
    pub archetypes: ArchetypeCatalog,
    pub assignments: Vec<BuildingAssignment>,
}
impl WorldPlan {
    pub fn from_json(world: &str, archetypes: &str, track: &Track) -> Result<Self, String> {
        let (definition, archetypes) = WorldDef::from_json(world, archetypes)?;
        if definition.track_id != track.definition.id.as_str() || track.definition.closed {
            return Err("world requires its matching point-to-point track".into());
        }
        if definition.seed != track.definition.scenery_seed {
            return Err("world seed differs from the track scenery seed".into());
        }
        if (definition.road_length_m - track.length).abs() > definition.length_tolerance_m {
            return Err(format!(
                "authored world length {} differs from baked track length {} (tolerance {})",
                definition.road_length_m, track.length, definition.length_tolerance_m
            ));
        }
        let assignments = definition.allocate(&archetypes)?;
        Ok(Self {
            definition,
            archetypes,
            assignments,
        })
    }
    /// Only the final rounding residual is snapped, never scale the route/stations.
    pub fn station(&self, station: f32, track: &Track) -> Option<f32> {
        if !station.is_finite() || station < 0.0 || station > track.length {
            return None;
        }
        Some(station.min(self.definition.road_length_m))
    }
    pub fn section_at(&self, station: f32, track: &Track) -> Option<usize> {
        self.definition.section_at(self.station(station, track)?)
    }
    pub fn color_at(&self, station: f32, track: &Track) -> Color {
        let Some(station) = self.station(station, track) else {
            return Color::WHITE;
        };
        let Some((a, b, t)) = self.definition.district_blend(station) else {
            return Color::WHITE;
        };
        let a = palette(a);
        let b = palette(b);
        Color::srgb(a[0].lerp(b[0], t), a[1].lerp(b[1], t), a[2].lerp(b[2], t))
    }
}
fn palette(district: District) -> [f32; 3] {
    match district {
        District::Farms => [0.35, 1.0, 0.25],
        District::RuralEdge => [0.9, 0.85, 0.2],
        District::Suburban => [1.0, 0.55, 0.15],
        District::InnerCity => [0.9, 0.3, 0.3],
        District::CityCore => [0.85, 0.3, 1.0],
        District::CityToHills => [0.2, 0.8, 1.0],
        District::Hills => [0.35, 0.5, 1.0],
    }
}
pub fn setup_plan(mut commands: Commands, track: Res<Track>) {
    if track.definition.id.as_str() == "valley-run-01" {
        let plan = WorldPlan::from_json(WORLD_JSON, ARCHETYPES_JSON, &track)
            .expect("validate Valley base-world overlay");
        info!(
            "Base world B00-B02: {} sections, {} archetypes, {} allocated buildings (NOT spawned); seed {}",
            plan.definition.sections.len(),
            plan.archetypes.archetypes.len(),
            plan.assignments.len(),
            plan.definition.seed
        );
        commands.insert_resource(plan);
    }
}
/// Developer-only F4 overlay; quotas/physics/sector ownership are unaffected.
pub fn preview(
    track: Res<Track>,
    plan: Option<Res<WorldPlan>>,
    state: Res<crate::environment::TrackInspector>,
    mut gizmos: Gizmos,
) {
    if !state.visible {
        return;
    }
    let Some(plan) = plan else {
        return;
    };
    for d in (0..track.length.ceil() as u32).step_by(8) {
        let d = d as f32;
        let end = (d + 8.0).min(track.length);
        for side in [-1.0, 1.0] {
            let (p, q) = track.sample(d, 0.0);
            let (next, r) = track.sample(end, 0.0);
            gizmos.line(
                p + q * Vec3::X * side * (track.half_width(d) + 1.0) + Vec3::Y * 0.4,
                next + r * Vec3::X * side * (track.half_width(end) + 1.0) + Vec3::Y * 0.4,
                plan.color_at(d, &track),
            );
        }
    }
    for section in &plan.definition.sections {
        let (p, q) = track.sample(section.start_m, 0.0);
        let color = plan.color_at(section.start_m, &track);
        gizmos.line(
            p - q * Vec3::X * 24.0 + Vec3::Y * 0.5,
            p + q * Vec3::X * 24.0 + Vec3::Y * 0.5,
            color,
        );
        gizmos.line(p + Vec3::Y * 0.5, p + Vec3::Y * 8.0, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn world_uses_baked_stations_and_only_snaps_final_rounding_residual() {
        let track = Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap();
        let plan = WorldPlan::from_json(WORLD_JSON, ARCHETYPES_JSON, &track).unwrap();
        assert_eq!(plan.section_at(track.length, &track), Some(14));
        assert!(plan.section_at(track.length + 0.01, &track).is_none());
        for (i, s) in plan.definition.sections.iter().enumerate() {
            assert_eq!(plan.section_at(s.start_m, &track), Some(i));
            let p = track.sample(s.start_m, 0.0).0;
            assert!((track.project(p, 0.0).0 - s.start_m).abs() < 0.05);
        }
        let mut json: serde_json::Value = serde_json::from_str(WORLD_JSON).unwrap();
        json["road_length_m"] = (plan.definition.road_length_m + 10.0).into();
        json["sections"][14]["end_m"] = json["road_length_m"].clone();
        assert!(
            WorldPlan::from_json(&json.to_string(), ARCHETYPES_JSON, &track)
                .err()
                .unwrap()
                .contains("baked track length")
        );
        let mut closed = Track::default().definition;
        closed.id = track.definition.id.clone();
        closed.scenery_seed = track.definition.scenery_seed;
        assert!(
            WorldPlan::from_json(
                WORLD_JSON,
                ARCHETYPES_JSON,
                &Track::from_definition(closed).unwrap()
            )
            .is_err()
        );
        let mut wrong = track.definition.clone();
        wrong.scenery_seed += 1;
        assert!(
            WorldPlan::from_json(
                WORLD_JSON,
                ARCHETYPES_JSON,
                &Track::from_definition(wrong).unwrap()
            )
            .is_err()
        );
        assert!(WorldPlan::from_json(WORLD_JSON, ARCHETYPES_JSON, &Track::default()).is_err());
    }
}
