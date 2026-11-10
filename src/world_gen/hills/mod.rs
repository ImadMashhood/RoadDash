//! Original Hills scenery. Render detail never owns/removes physical obstacles.
pub mod atmosphere;
pub mod city_slice;
pub mod static_collision;
pub use countryside::HillsFarm;
pub mod bridge;
mod countryside;
pub mod dressing;
mod geometry;
pub mod mounted;
mod props;
pub mod river;
use crate::{track::Track, world::Art};
use bevy::prelude::*;
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
    let mut census = Census::default();
    geometry::spawn(
        commands,
        meshes,
        materials,
        art,
        track,
        &mut census,
        images.as_deref_mut(),
    );
    bridge::spawn(commands, meshes, materials, art, track);
    river::spawn(commands, meshes, materials, images.as_deref_mut(), track);
    props::spawn(commands, meshes, materials, art, track, &mut census);
    countryside::spawn(commands, meshes, materials, art, track, &mut census);
    dressing::spawn(commands, meshes, materials, track);
    mounted::spawn(commands, meshes, materials, track);
    census.mounted_pairs = mounted::COUNT;
    atmosphere::spawn(commands, meshes, materials, track, images);
    commands.insert_resource(census);
}

pub use props::TownDetailCensus;

pub mod clouds;
