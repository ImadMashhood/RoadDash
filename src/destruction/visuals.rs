use super::*;
#[derive(Component)]
pub(crate) struct Visual {
    pub owner: Entity,
    pub original: Option<Transform>,
    pub damaged: Option<Entity>,
    pub prepared: bool,
}
#[derive(Component)]
pub(super) struct Damaged;
#[derive(Component)]
pub(super) struct Ready;

pub(super) fn ready(event: On<bevy::scene::SceneInstanceReady>, mut commands: Commands) {
    commands.entity(event.entity).insert(Ready);
}

/// Offline buckled variants stay hidden until fully instantiated. While loading,
/// retain the supported original crush presentation; restart never waits on IO.
#[allow(clippy::too_many_arguments)]
pub(super) fn update(
    mut visuals: Query<(&mut Transform, &mut Visibility, &mut Visual), Without<Damaged>>,
    mut damaged: Query<(&mut Visibility, Option<&Ready>), With<Damaged>>,
    props: Query<(&Breakable, Option<&GlobalTransform>)>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    children: Query<&Children>,
    materials: Query<(&Mesh3d, &MeshMaterial3d<StandardMaterial>)>,
    meshes: Option<Res<Assets<Mesh>>>,
    material_assets: Option<Res<Assets<StandardMaterial>>>,
    queue: Option<Res<crate::material_detail::Queue>>,
) {
    for (mut transform, mut visibility, mut visual) in &mut visuals {
        let original = if let Some(original) = visual.original {
            original
        } else {
            visual.original = Some(*transform);
            *transform
        };
        let Ok((p, global)) = props.get(visual.owner) else {
            continue;
        };
        let in_range = cameras
            .iter()
            .next()
            .zip(global)
            .is_none_or(|(camera, global)| {
                camera.translation().distance_squared(global.translation()) <= 500. * 500.
            });
        let mut variant_ready = false;
        if let Some(entity) = visual.damaged
            && let Ok((mut visible, ready)) = damaged.get_mut(entity)
        {
            variant_ready = visual.prepared
                || (ready.is_some()
                    && queue.as_ref().is_none_or(|queue| {
                        let mut count = 0;
                        let ready = children
                            .iter_descendants(entity)
                            .filter_map(|e| materials.get(e).ok())
                            .all(|(mesh, material)| {
                                count += 1;
                                queue.material_ready(material.0.id())
                                    && material_assets
                                        .as_ref()
                                        .and_then(|assets| assets.get(&material.0))
                                        .is_some_and(|m| {
                                            m.normal_map_texture.is_none()
                                                || meshes.as_ref().is_some_and(|meshes| {
                                                    meshes.get(&mesh.0).is_some_and(|mesh| {
                                                        mesh.contains_attribute(
                                                            Mesh::ATTRIBUTE_TANGENT,
                                                        )
                                                    })
                                                })
                                        })
                            });
                        count > 0 && ready
                    }));
            if visual.prepared != variant_ready {
                visual.prepared = variant_ready;
            }
            let wanted = if p.state.broken && variant_ready && in_range {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            visible.set_if_neq(wanted);
        }
        let wanted = if !in_range || (p.state.broken && variant_ready) {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
        visibility.set_if_neq(wanted);
        let mut pose = original;
        if p.state.broken && p.profile.style == Style::Crush {
            pose.scale.y *= 0.38;
            pose.translation.y *= 0.38;
        }
        transform.set_if_neq(pose);
    }
}
