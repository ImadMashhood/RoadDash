//! Compact woodland meshes baked from the authored roadside tree families.
use bevy::prelude::*;
use serde::Deserialize;
use std::sync::OnceLock;

pub(crate) const CONIFERS: [&str; 5] = ["TREE4A2", "TREE4B2", "TREE5A2", "TREEF2", "TREEU12"];
pub(crate) const BROADLEAVES: [&str; 5] = ["TREEH2", "TREOAKA2", "TREOAKB2", "TREEK2", "TREEL2"];

#[derive(Deserialize)]
pub(super) struct Template {
    pub positions: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
}

#[derive(Deserialize)]
struct Family {
    name: String,
    tiers: [Template; 3],
}

fn families() -> &'static [Family] {
    static FAMILIES: OnceLock<Vec<Family>> = OnceLock::new();
    FAMILIES.get_or_init(|| {
        let families: Vec<Family> =
            serde_json::from_str(include_str!("../../../../content/hills_tree_lods.json"))
                .expect("baked Hills tree families");
        assert!(
            families
                .iter()
                .map(|family| family.name.as_str())
                .eq(CONIFERS.into_iter().chain(BROADLEAVES))
        );
        families
    })
}

pub(super) fn named(name: &str) -> &'static [Template; 3] {
    &families()
        .iter()
        .find(|family| family.name == name)
        .expect("authored Hills tree")
        .tiers
}

pub(super) fn fitted(
    seed: u32,
    height: f32,
    position: Vec3,
) -> (&'static [Template; 3], Transform) {
    // Match the countryside's one conifer row and two broadleaf rows.
    let conifer = seed.is_multiple_of(3);
    let variant = (seed as usize / 7) % CONIFERS.len();
    let width = height * if conifer { 0.5 } else { 0.75 };
    let angle = super::noise(seed.wrapping_add(13)) * std::f32::consts::TAU;
    let family = if conifer {
        variant
    } else {
        CONIFERS.len() + variant
    };
    (
        &families()[family].tiers,
        Transform::from_translation(position)
            .with_rotation(Quat::from_rotation_y(angle))
            .with_scale(Vec3::new(width, height, width)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_roadside_families_have_grounded_simplified_meshes_and_distinct_palettes() {
        let families = families();
        assert_eq!(families.len(), 10);
        for family in families {
            assert!(
                family
                    .tiers
                    .windows(2)
                    .all(|pair| pair[0].triangles.len() > pair[1].triangles.len())
            );
            for (tier, limit) in family.tiers.iter().zip([272, 188, 56]) {
                assert!(tier.triangles.len() <= limit);
                assert_eq!(tier.positions.len(), tier.colors.len());
                assert!(tier.positions.iter().any(|p| p[1].abs() < 0.02));
                assert!(tier.positions.iter().any(|p| p[1] > 0.95));
                assert!(
                    tier.triangles
                        .iter()
                        .flatten()
                        .all(|&i| (i as usize) < tier.positions.len())
                );
                assert!(tier.colors.iter().flatten().all(|v| (0. ..=1.).contains(v)));
            }
        }
        let green = families[0].tiers[2].colors.last().unwrap();
        let oak = families[6].tiers[2].colors.last().unwrap();
        assert!(green[1] > green[0] && oak[0] > oak[1]);
    }
}
