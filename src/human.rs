//! One original human geometry shared by seated, standing and physical states.
use crate::world::{self, Art};
use bevy::prelude::*;
pub type SegmentSpec = (Vec3, Vec3, Option<(usize, Vec3)>);
pub const SPECS: [SegmentSpec; 11] = [
    (Vec3::new(0.0, 1.55, 0.0), Vec3::new(0.5, 0.58, 0.32), None),
    (
        Vec3::new(0.0, 2.10, 0.0),
        Vec3::new(0.40, 0.42, 0.38),
        Some((0, Vec3::new(0.0, 1.87, 0.0))),
    ),
    (
        Vec3::new(0.0, 1.10, 0.0),
        Vec3::new(0.44, 0.28, 0.30),
        Some((0, Vec3::new(0.0, 1.26, 0.0))),
    ),
    (
        Vec3::new(-0.45, 1.55, 0.0),
        Vec3::new(0.24, 0.40, 0.24),
        Some((0, Vec3::new(-0.29, 1.69, 0.0))),
    ),
    (
        Vec3::new(-0.49, 1.14, 0.0),
        Vec3::new(0.21, 0.38, 0.21),
        Some((3, Vec3::new(-0.47, 1.34, 0.0))),
    ),
    (
        Vec3::new(0.45, 1.55, 0.0),
        Vec3::new(0.24, 0.40, 0.24),
        Some((0, Vec3::new(0.29, 1.69, 0.0))),
    ),
    (
        Vec3::new(0.49, 1.14, 0.0),
        Vec3::new(0.21, 0.38, 0.21),
        Some((5, Vec3::new(0.47, 1.34, 0.0))),
    ),
    (
        Vec3::new(-0.17, 0.73, 0.0),
        Vec3::new(0.25, 0.44, 0.25),
        Some((2, Vec3::new(-0.17, 0.96, 0.0))),
    ),
    (
        Vec3::new(-0.17, 0.25, 0.0),
        Vec3::new(0.22, 0.44, 0.22),
        Some((7, Vec3::new(-0.17, 0.49, 0.0))),
    ),
    (
        Vec3::new(0.17, 0.73, 0.0),
        Vec3::new(0.25, 0.44, 0.25),
        Some((2, Vec3::new(0.17, 0.96, 0.0))),
    ),
    (
        Vec3::new(0.17, 0.25, 0.0),
        Vec3::new(0.22, 0.44, 0.22),
        Some((9, Vec3::new(0.17, 0.49, 0.0))),
    ),
];

#[derive(Component, Clone, Copy)]
pub struct HumanSegment(pub usize);
pub const HEIGHT: f32 = 2.31;
pub fn segment(
    commands: &mut Commands,
    art: &Art,
    color: usize,
    index: usize,
    position: Vec3,
    rotation: Quat,
) -> Entity {
    let size = SPECS[index].1;
    let head = index == 1;
    let e = world::part(
        commands,
        if head {
            art.sphere.clone()
        } else {
            art.cube.clone()
        },
        if head {
            art.white.clone()
        } else if index == 2 || index >= 7 {
            art.dark.clone()
        } else {
            art.colors[color % art.colors.len()].clone()
        },
        position,
        if head { size * 0.5 } else { size },
        rotation,
    );
    commands.entity(e).insert(HumanSegment(index));
    if head {
        let visor = world::part(
            commands,
            art.cube.clone(),
            art.dark.clone(),
            Vec3::Z * 0.95,
            Vec3::new(1.7, 0.65, 0.4),
            Quat::IDENTITY,
        );
        commands.entity(e).add_child(visor);
    }
    e
}
pub fn seated(index: usize) -> Transform {
    let (p, q) = match index {
        0 => (Vec3::new(0.0, 1.55, -0.12), Quat::IDENTITY),
        1 => (Vec3::new(0.0, 2.10, -0.12), Quat::IDENTITY),
        2 => (Vec3::new(0.0, 1.10, -0.12), Quat::IDENTITY),
        3 | 5 => (
            Vec3::new(if index == 3 { -0.39 } else { 0.39 }, 1.55, 0.18),
            Quat::from_rotation_x(-1.0),
        ),
        4 | 6 => (
            Vec3::new(if index == 4 { -0.34 } else { 0.34 }, 1.40, 0.50),
            Quat::from_rotation_x(-1.5),
        ),
        7 | 9 => (
            Vec3::new(if index == 7 { -0.28 } else { 0.28 }, 0.98, 0.06),
            Quat::from_rotation_x(-1.05),
        ),
        _ => (
            Vec3::new(if index == 8 { -0.35 } else { 0.35 }, 0.65, 0.25),
            Quat::from_rotation_x(0.1),
        ),
    };
    Transform::from_translation(p).with_rotation(q)
}
pub fn limb(index: usize) -> Option<(f32, bool, Vec3)> {
    match index {
        3 | 4 => Some((-1.0, false, Vec3::new(-0.29, 1.69, 0.0))),
        5 | 6 => Some((1.0, false, Vec3::new(0.29, 1.69, 0.0))),
        7 | 8 => Some((-1.0, true, Vec3::new(-0.17, 0.96, 0.0))),
        9 | 10 => Some((1.0, true, Vec3::new(0.17, 0.96, 0.0))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_human_state_uses_identical_mesh_dimensions_and_outfit() {
        let mut app = crate::game::tests::simulation();
        app.add_systems(Update, spawn_once.run_if(run_once));
        app.update();
        let world = app.world_mut();
        for index in 0..11 {
            let parts: Vec<_> = world
                .query::<(
                    &HumanSegment,
                    &Mesh3d,
                    &MeshMaterial3d<StandardMaterial>,
                    &Transform,
                )>()
                .iter(world)
                .filter(|(p, _, _, _)| p.0 == index)
                .map(|(_, m, color, t)| (m.0.clone(), color.0.clone(), t.scale))
                .collect();
            assert_eq!(
                parts.len(),
                4,
                "segment {index} missing from an actor state"
            );
            assert!(
                parts.iter().all(|p| *p == parts[0]),
                "appearance changed for segment {index}"
            );
        }
    }
    fn spawn_once(mut commands: Commands, art: Res<Art>) {
        world::bike(&mut commands, &art, 4, true);
        world::runner(&mut commands, &art, 4);
        let owner = commands.spawn_empty().id();
        crate::recovery::humanoid(
            &mut commands,
            &art,
            owner,
            4,
            Vec3::ZERO,
            Quat::IDENTITY,
            Vec3::ZERO,
        );
        crate::environment::spawn_pedestrian(&mut commands, &art, Vec3::ZERO, Quat::IDENTITY, 1.0);
    }
    #[test]
    fn turn_demand_does_not_apply_cosmetic_roll_to_the_character() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Update, (crate::vehicle::visuals, world::upright_riders));
        let bike = app
            .world_mut()
            .spawn((
                crate::vehicle::BikeState {
                    lean: 1.0,
                    ..default()
                },
                Transform::from_rotation(Quat::from_rotation_y(0.7) * Quat::from_rotation_z(0.5)),
            ))
            .id();
        let root = app
            .world_mut()
            .spawn((
                crate::runtime::VisualRoot { owner: bike },
                Transform::IDENTITY,
            ))
            .id();
        let human = app
            .world_mut()
            .spawn((world::UprightRider { owner: bike }, Transform::IDENTITY))
            .id();
        app.update();
        app.update();
        let chassis = app.world().get::<Transform>(bike).unwrap().rotation;
        let pose = app.world().get::<Transform>(human).unwrap().rotation;
        assert!(
            (chassis * pose * Vec3::Y).dot(Vec3::Y) > 0.9999,
            "character inherited chassis lean"
        );
        assert_eq!(
            app.world().get::<Transform>(root).unwrap().rotation,
            Quat::IDENTITY
        );
    }
}
