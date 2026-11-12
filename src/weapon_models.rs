//! Family-aware original weapon presentation. Never writes simulation transforms.
use bevy::prelude::*;
use rr_core::weapons::Weapon;
#[derive(Resource, Default)]
pub(crate) struct Selection {
    scenes: Vec<(Weapon, Handle<WorldAsset>)>,
    override_scene: Option<Handle<WorldAsset>>,
}
pub(crate) fn load(mut commands: Commands, assets: Option<Res<AssetServer>>) {
    let Some(assets) = assets else { return };
    let catalog = rr_content::weapon_models::Catalog::builtin();
    let scenes: Vec<_> = Weapon::ALL
        .into_iter()
        .filter_map(|weapon| {
            catalog
                .records
                .iter()
                .find(|m| m.id == weapon.id())
                .map(|m| {
                    (
                        weapon,
                        assets.load(GltfAssetLabel::Scene(0).from_asset(m.lods[0].file.clone())),
                    )
                })
        })
        .collect();
    let override_scene = std::env::var("ROADDASH_WEAPON_MODEL").ok().and_then(|id| {
        let scene = catalog
            .records
            .iter()
            .find(|m| m.id == id)
            .map(|m| assets.load(GltfAssetLabel::Scene(0).from_asset(m.lods[0].file.clone())));
        if scene.is_none() {
            warn!("Unknown ROADDASH_WEAPON_MODEL={id}; retaining family model");
        }
        scene
    });
    commands.insert_resource(Selection {
        scenes,
        override_scene,
    });
}
#[derive(Component)]
pub(crate) struct ModelRoot;
#[derive(Component)]
pub(crate) struct Skin {
    weapon: Weapon,
    entity: Entity,
}
/// Replace only the presentation child when inventory changes. Retain resident
/// fallback until the new scene is ready; a stale callback cannot hide it.
pub(crate) fn apply(
    mut commands: Commands,
    selection: Res<Selection>,
    roots: Query<(Entity, &Children, &crate::weapons::Kind, Option<&Skin>), With<ModelRoot>>,
    parts: Query<(), With<crate::weapons::ClubPart>>,
    assets: Option<Res<AssetServer>>,
) {
    let Some(assets) = assets else { return };
    for (root, children, kind, old) in &roots {
        if old.is_some_and(|s| s.weapon == kind.0) {
            continue;
        }
        let scene = selection.override_scene.as_ref().or_else(|| {
            selection
                .scenes
                .iter()
                .find(|(w, _)| *w == kind.0)
                .map(|(_, h)| h)
        });
        if scene.is_none() && old.is_none() {
            continue;
        }
        if scene.is_some_and(|s| !assets.is_loaded_with_dependencies(s.id())) {
            continue;
        }
        let fallback: Vec<Entity> = children.iter().filter(|e| parts.contains(*e)).collect();
        if let Some(old) = old {
            commands.entity(old.entity).despawn();
        }
        for entity in &fallback {
            commands.entity(*entity).insert(Visibility::Inherited);
        }
        commands.entity(root).remove::<Skin>();
        let Some(scene) = scene else { continue };
        let skin = commands
            .spawn((
                WorldAssetRoot(scene.clone()),
                Transform::default(),
                Visibility::Inherited,
            ))
            .observe(
                move |ready: On<bevy::world_serialization::WorldInstanceReady>,
                      current: Query<&Skin>,
                      mut commands: Commands| {
                    if current.get(root).is_ok_and(|s| s.entity == ready.entity) {
                        for entity in &fallback {
                            commands.entity(*entity).insert(Visibility::Hidden);
                        }
                    }
                },
            )
            .id();
        commands
            .entity(root)
            .insert(Skin {
                weapon: kind.0,
                entity: skin,
            })
            .add_child(skin);
    }
}
