use super::*;
#[derive(Component)]
pub(super) struct Visual {
    pub owner: Entity,
    pub original: Option<Transform>,
    pub broken: bool,
}
/// Keep original authored meshes: crush the original model with its collider, rather than
/// replacing it with unrelated cubes. Scene LOD is pinned while damaged.
pub(super) fn update(mut visuals: Query<(&mut Transform, &mut Visual)>, props: Query<&Breakable>) {
    for (mut transform, mut visual) in &mut visuals {
        if visual.original.is_none() {
            visual.original = Some(*transform);
        }
        let Ok(p) = props.get(visual.owner) else {
            continue;
        };
        if p.state.broken == visual.broken {
            continue;
        }
        let original = visual.original.unwrap();
        *transform = original;
        if p.state.broken && p.profile.style == Style::Crush {
            transform.scale.y *= 0.38;
            transform.translation.y *= 0.38;
        }
        visual.broken = p.state.broken;
    }
}
