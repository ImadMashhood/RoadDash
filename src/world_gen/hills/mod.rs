//! Original Hills scenery. Render detail never owns/removes physical obstacles.
pub mod atmosphere;
pub mod city_slice;
pub mod static_collision;
pub use countryside::HillsFarm;
pub mod bridge;
mod countryside;
pub mod dressing;
pub(crate) mod encounters;
mod geometry;
pub mod guardrails;
pub mod mounted;
mod props;
pub mod river;
pub(crate) mod terrain_cache;
pub mod wildlife;
pub mod woodland;
use crate::{track::Track, world::Art};
use bevy::prelude::*;

/// Height of the resident Hills terrain mesh at an off-road effect anchor.
pub(crate) fn rendered_ground_height(track: &Track, x: f32, z: f32) -> f32 {
    geometry::ground_height(track, x, z)
}

pub(crate) fn crossing_support(track: &Track, p: Vec3) -> f32 {
    (0..2)
        .map(|route| geometry::crossing_road_support(track, route, p, 0.))
        .fold(geometry::ground_height(track, p.x, p.z), f32::max)
}

#[derive(Component)]
pub struct HillsSolid;
#[derive(Component)]
pub struct HillsBuilding;
#[derive(Component)]
pub struct HillsHorse;
#[derive(Component)]
pub struct HillsCowboy;
#[derive(Resource, Default, serde::Serialize)]
pub struct Census {
    pub road_chunks: usize,
    pub terrain_chunks: usize,
    pub buildings: usize,
    pub frontage_groups: usize,
    pub guardrail_segments: usize,
    pub cows: usize,
    pub horses: usize,
    pub cowboys: usize,
    pub trucks: usize,
    pub mounted_pairs: usize,
    pub trees: usize,
    pub farms: usize,
    pub farm_cows: usize,
    pub grass_tufts: usize,
    pub grass_batches: usize,
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    track: &Track,
    mut images: Option<&mut Assets<Image>>,
) {
    let _load_span = crate::diagnostics::loading::Span::new("hills_generation");
    let _terrain_cache = terrain_cache::Scope::new(track);
    let mut census = Census::default();
    {
        let _span = crate::diagnostics::loading::Span::new("hills_geometry");
        geometry::spawn(
            commands,
            meshes,
            materials,
            art,
            track,
            &mut census,
            images.as_deref_mut(),
        );
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_bridge");
        bridge::spawn(commands, meshes, materials, art, track);
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_river");
        river::spawn(commands, meshes, materials, images.as_deref_mut(), track);
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_props");
        props::spawn(commands, meshes, materials, art, track, &mut census);
        encounters::spawn(commands, meshes, materials, art, track);
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_countryside");
        countryside::spawn(commands, meshes, materials, art, track, &mut census);
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_woodland");
        woodland::spawn(commands, meshes, materials, track);
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_dressing");
        dressing::spawn(commands, meshes, materials, track);
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_mounted");
        mounted::spawn(commands, meshes, materials, track);
    }
    {
        let _span = crate::diagnostics::loading::Span::new("hills_wildlife");
        wildlife::spawn(commands, meshes, materials, track);
        wildlife::spawn_crossing_cattle(commands, art, track);
    }
    census.mounted_pairs = mounted::COUNT;
    {
        let _span = crate::diagnostics::loading::Span::new("hills_atmosphere");
        atmosphere::spawn(commands, meshes, materials, track, images);
    }
    for (index, station) in [
        1260., 1840., 2740., 3910., 5740., 8130., 8770., 10030., 11910.,
    ]
    .into_iter()
    .enumerate()
    {
        let lane = track.half_width(station) + 3.;
        crate::world_gen::metropolis::pedestrians::spawn_road_crossing(
            commands, art, track, station, lane, index,
        );
    }
    commands.insert_resource(census);
}

pub use props::TownDetailCensus;

pub mod clouds;
