//! Measured legacy scene census kept separate from proposed construction quotas.
use super::*;
use crate::{
    environment::{LegacyBuilding, RoadChunk, Terrain, TrackInspector},
    world,
};
use bevy_rapier3d::prelude::Collider;
use serde_json::{Value, json};

/// Specified FNV-1a64 over serialized records. A regression fingerprint, not security.
pub fn fingerprint(bytes: &[u8]) -> String {
    let hash = bytes.iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    });
    format!("fnv1a64:{hash:016x}")
}
#[derive(Resource)]
pub struct BaselineCensus(pub Value);
pub fn census(world: &mut World) -> Value {
    let mut buildings:Vec<_>=world.query::<(&LegacyBuilding,&Transform,Option<&Collider>,Option<&Mesh3d>)>().iter(world).map(|(b,t,c,m)|json!({"id":b.id,"station_m":b.station_m,"position":t.translation.to_array(),"rotation_xyzw":t.rotation.to_array(),"size":t.scale.to_array(),"solid":c.is_some(),"render_mass":m.is_some()})).collect();
    buildings.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    let geometry_hash = fingerprint(&serde_json::to_vec(&buildings).expect("finite scene census"));
    let meshes = world.query::<&Mesh3d>().iter(world).count();
    let colliders = world.query::<&Collider>().iter(world).count();
    let road_chunks = world.query::<&RoadChunk>().iter(world).count();
    let terrain_chunks = world.query::<&Terrain>().iter(world).count();
    let inspector = world.resource::<TrackInspector>();
    json!({"kind":"measured_legacy_scene_not_generated_target", "building_masses":buildings.len(),"render_mesh_entities":meshes,"collider_entities":colliders,"road_chunks":road_chunks,"terrain_chunks":terrain_chunks,"environment_render_triangles":inspector.render_triangles,"environment_collision_triangles":inspector.collision_triangles,"building_geometry_fingerprint":geometry_hash,"buildings":buildings,"limitations":"Environment-only snapshot before racers/UI/audio; triangle counters are existing ribbon/terrain counters, not total building triangles or draw calls. Legacy placement is not validated lot/foundation generation."})
}
pub fn document(plan: &WorldPlan, track: &Track, baseline: Option<&Value>) -> Value {
    let allocation_hash =
        fingerprint(&serde_json::to_vec(&plan.assignments).expect("serialize assignments"));
    let sections:Vec<_>=plan.definition.sections.iter().enumerate().map(|(i,s)| {
        let end=if i+1==plan.definition.sections.len(){track.length}else{s.end_m};
        json!({"id":s.id,"name":s.name,"district":s.district,"start_m":s.start_m,"end_m":end,"authored_end_m":s.end_m,"start_position":track.sample(s.start_m,0.0).0.to_array(),"end_position":track.sample(end,0.0).0.to_array(),"near":s.near,"secondary":s.secondary,"total":s.total,"legacy_masses":baseline.map(|b| b["buildings"].as_array().expect("census records").iter().filter(|b| plan.section_at(b["station_m"].as_f64().unwrap() as f32,track)==Some(i)).count())})
    }).collect();
    json!({"phase":"B00-B02 foundation", "generator_version":plan.definition.generator_version,"track_id":plan.definition.track_id,"seed":plan.definition.seed,"closed":track.definition.closed,"baked_length_m":track.length,"authored_length_m":plan.definition.road_length_m,"end_snap_delta_m":track.length-plan.definition.road_length_m,"elevation_range_m":[track.bounds.0.y,track.bounds.1.y],"gates":track.definition.checkpoints.len(),"grid_spawns":track.definition.spawns.len(),"target":{"sections":sections.len(),"archetypes":plan.archetypes.archetypes.len(),"buildings":plan.definition.building_target_total,"near":plan.definition.near_road,"secondary":plan.definition.secondary_row,"families":plan.definition.archetype_counts},"placement_generated":false,"allocation_fingerprint":allocation_hash,"fingerprint_scope":"quota assignment only; not transforms, lots, foundations or completed-world determinism", "sections":sections,"archetypes":plan.archetypes.archetypes,"planned_assignments":plan.assignments,"baseline":baseline})
}
/// Same setup function as the game; does not initialize a window/GPU/audio.
pub fn headless_baseline(track: Track) -> Value {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(track)
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_systems(Startup, world::setup_world);
    app.update();
    census(app.world_mut())
}
pub fn capture_baseline(world: &mut World) {
    let measured = census(world);
    info!(
        "Existing world census: {} building masses, {} mesh entities, {} collider entities",
        measured["building_masses"],
        measured["render_mesh_entities"],
        measured["collider_entities"]
    );
    if let Some(path) = std::env::var_os("ROADDASH_WORLD_REPORT") {
        let output = if let Some(plan) = world.get_resource::<WorldPlan>() {
            document(plan, world.resource::<Track>(), Some(&measured))
        } else {
            measured.clone()
        };
        std::fs::write(
            path,
            serde_json::to_string_pretty(&output).expect("serialize world report"),
        )
        .expect("write ROADDASH_WORLD_REPORT");
    }
    world.insert_resource(BaselineCensus(measured));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn measured_baseline_and_allocated_target_are_distinct_and_reproducible() {
        let load = || Track::from_json(include_str!("../../content/tracks/valley.json")).unwrap();
        let track = load();
        let plan = WorldPlan::from_json(WORLD_JSON, ARCHETYPES_JSON, &track).unwrap();
        let first = headless_baseline(load());
        let second = headless_baseline(load());
        assert_eq!(first, second);
        assert!(first["building_masses"].as_u64().unwrap() > 10);
        assert!(
            first["buildings"]
                .as_array()
                .unwrap()
                .iter()
                .all(|b| b["solid"] == true && b["render_mass"] == true)
        );
        let report = document(&plan, &track, Some(&first));
        assert_eq!(report["target"]["buildings"], 247);
        assert_eq!(report["placement_generated"], false);
        assert_eq!(
            report["sections"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s["legacy_masses"].as_u64().unwrap())
                .sum::<u64>(),
            first["building_masses"].as_u64().unwrap()
        );
        assert_eq!(report, document(&plan, &track, Some(&second)));
    }
}
