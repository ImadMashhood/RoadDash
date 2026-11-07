//! Full-depth authored-model city in the actual racing world.
pub mod geometry;
pub mod props;
pub mod survey;
#[cfg(test)]
mod tests;
pub mod visuals;
use super::spatial::Rect;
use crate::{
    track::Track,
    world::{Art, Ground, Obstacle},
};
use bevy::{asset::io::AssetSourceBuilder, prelude::*};
use bevy_rapier3d::prelude::*;
use rr_content::city::{CityPlan, CitySettings};
pub const SETTINGS_JSON: &str = include_str!("../../../content/world/city_settings.json");
pub const CITY_JSON: &str = include_str!("../../../content/world/city5300.json");
#[derive(Resource)]
pub struct GeneratedCity {
    pub plan: CityPlan,
    pub blockers: Vec<Rect>,
}
#[derive(Component)]
pub struct CityBuilding {
    pub id: String,
}

pub fn register_assets(app: &mut App) {
    let packaged = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("assets/architecture")));
    let source = packaged.filter(|p| p.is_dir()).unwrap_or_else(|| {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("art/buildings/mesh_assets")
    });
    app.register_asset_source(
        "architecture",
        AssetSourceBuilder::platform_default(&source.to_string_lossy(), None),
    );
}

pub fn surface_height(c: &CitySettings, x: f32, z: f32) -> f32 {
    let cross = z
        .rem_euclid(c.block_pitch_m)
        .min(c.block_pitch_m - z.rem_euclid(c.block_pitch_m));
    if (cross <= c.cross_street_width_m / 2.0 && x.abs() <= c.parallel_road_offset_m + 20.0)
        || x.abs() <= c.road_width_m / 2.0
        || (x.abs() - c.parallel_road_offset_m).abs() <= c.parallel_road_width_m / 2.0
    {
        return 0.025;
    }
    if (cross <= c.cross_street_width_m / 2.0 + c.sidewalk_width_m
        && x.abs() <= c.parallel_road_offset_m - c.parallel_road_width_m / 2.0)
        || x.abs() <= c.road_width_m / 2.0 + c.sidewalk_width_m
        || (x.abs() - c.parallel_road_offset_m).abs()
            <= c.parallel_road_width_m / 2.0 + c.sidewalk_width_m
    {
        0.16
    } else {
        -0.03
    }
}

pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    server: Option<&AssetServer>,
    art: &Art,
    track: &Track,
) {
    let plan = CityPlan::from_json(CITY_JSON).expect("validated generated City Run");
    assert_eq!(
        plan.seed,
        u64::from(track.definition.scenery_seed),
        "city/track seed mismatch"
    );
    assert!(
        (plan.length_m - track.length).abs() < 0.05,
        "city/track length mismatch"
    );
    if server.is_some() {
        geometry::spawn_dressed(
            commands,
            meshes,
            materials,
            server,
            &plan.chunks,
            art.props.is_some(),
        );
        if let Some(props) = &art.props {
            crate::props::dress_city(commands, props, &plan);
        }
    }
    if let Some(server) = server {
        visuals::spawn(commands, server, &plan.lots, false);
    }
    let mut physical = 0;
    // Supported roads, the full starting grid and the post-finish apron.
    for chunk in &plan.chunks {
        for (name, centre, size, _) in &chunk.boxes {
            if !matches!(
                name.as_str(),
                "supported_city_ground"
                    | "continuous_road"
                    | "crossroad"
                    | "sidewalk"
                    | "cross_street_sidewalk"
                    | "curb"
                    | "trunk"
                    | "lamp"
            ) {
                continue;
            }
            let mut entity = commands.spawn((
                Name::new(format!("city solid {name}")),
                RigidBody::Fixed,
                Collider::cuboid(size[0] / 2.0, size[1] / 2.0, size[2] / 2.0),
                Transform::from_translation(Vec3::from_array(*centre)),
                Friction::coefficient(0.85),
            ));
            if matches!(name.as_str(), "trunk" | "lamp") {
                entity.insert(Obstacle);
            } else {
                entity.insert((Ground, crate::vehicle::RideableStep));
            }
            physical += 1;
        }
    }
    for lot in &plan.lots {
        let width = lot.z_end - lot.z_start;
        commands.spawn((
            Name::new(format!("city building {}", lot.id)),
            CityBuilding { id: lot.id.clone() },
            RigidBody::Fixed,
            Obstacle,
            Collider::cuboid(
                lot.depth / 2.0 + 0.18,
                lot.height_m / 2.0 + 0.1,
                width / 2.0 + 0.1,
            ),
            Transform::from_xyz(
                lot.center_x,
                lot.height_m / 2.0 + 0.16,
                (lot.z_start + lot.z_end) / 2.0,
            ),
            Friction::coefficient(0.85),
        ));
        physical += 1;
    }
    let mut rows = std::collections::BTreeMap::<(usize, i32, String), Vec<_>>::new();
    for lot in &plan.lots {
        rows.entry((lot.block, lot.side, lot.row.clone()))
            .or_default()
            .push(lot);
    }
    let blockers = rows
        .values()
        .map(|lots| {
            let start = lots.iter().map(|l| l.z_start).fold(f32::INFINITY, f32::min);
            let end = lots
                .iter()
                .map(|l| l.z_end)
                .fold(f32::NEG_INFINITY, f32::max);
            let lot = lots[0];
            Rect::new(
                Vec3::new(lot.center_x, 0.0, (start + end) / 2.0),
                Quat::IDENTITY,
                Vec2::new(lot.depth / 2.0 + 0.2, (end - start) / 2.0 + 0.12),
            )
        })
        .collect();
    let glass = materials.add(StandardMaterial {
        base_color: Color::srgb(0.16, 0.28, 0.32),
        perceptual_roughness: 0.35,
        ..default()
    });
    for prop in &track.definition.props {
        if prop.kind == rr_content::track::PropKind::Crossing {
            let (p, q) = track.sample(prop.fraction * track.length, 0.0);
            crate::environment::spawn_crossing(commands, art, &glass, p, q);
        }
    }
    for station in (250..5300).step_by(200) {
        for side in [-1.0, 1.0] {
            let (p, q) = track.sample(station as f32, side * 8.9);
            crate::environment::spawn_pedestrian(
                commands,
                art,
                p.with_y(0.32),
                q,
                station as f32 + side,
            );
        }
    }
    info!(
        "CITY5300: {} complete buildings; {} blocks; {} solid bodies; {} facade IDs",
        plan.lots.len(),
        (plan.length_m / plan.config.block_pitch_m) as usize,
        physical,
        plan.model_design_ids.len()
    );
    commands.insert_resource(GeneratedCity { plan, blockers });
}

/// Validated authoring report. Native and Rapier evidence are recorded separately.
pub fn document() -> Result<serde_json::Value, String> {
    let plan = CityPlan::from_json(CITY_JSON)?;
    let track = Track::from_json(include_str!("../../../content/tracks/city5300.json"))?;
    let parts = |name: &str| {
        plan.chunks
            .iter()
            .flat_map(|c| &c.boxes)
            .filter(|b| b.0 == name)
            .count()
    };
    Ok(
        serde_json::json!({"track_id":track.definition.id.as_str(),"length_m":track.length,"closed":track.definition.closed,"seed":plan.seed,"blocks":(plan.length_m/plan.config.block_pitch_m)as usize,"buildings":plan.lots.len(),"facade_designs":plan.model_design_ids.len(),"shared_roofs":parts("shared_closed_roof"),"shared_floor_slabs":parts("shared_floor_slab"),"rear_windows":parts("recessed_rear_glass"),"district_zones":plan.district_zones,"starting_grid":track.definition.spawns,"checkpoints":track.definition.checkpoints,"scope":"validated full geometry/lot plan; not a physics or performance receipt","runtime_asset_source":"architecture://front_modules and architecture://materials","legacy_world_preserved":true}),
    )
}
