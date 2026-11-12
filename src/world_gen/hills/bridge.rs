//! Original suspension crossing: supported deck, open edges and square steel hangers.
use super::{HillsSolid, Track, props::Model};
use crate::world::{Art, Ground, Obstacle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const START: f32 = crate::track::hills::BRIDGE_START;
pub const END: f32 = crate::track::hills::BRIDGE_END;
pub const CENTER: f32 = crate::track::hills::BRIDGE_CENTER;
pub const HALF_DECK: f32 = 7.25;
pub const HANGER_WIDTH: f32 = 0.65;
pub const HANGER_SPACING: f32 = 8.;
const HANGER_LANE: f32 = 9.2;
const GIRDER_DROP: f32 = 0.95;
const HANGER_FOOT_DROP: f32 = 0.45;
// Ordinary spaces between the regular hangers, used by the physical fall fixtures.
pub const GAP_STATIONS: [f32; 2] = [4_684., 4_796.];
pub const GAP_HALF_LENGTH: f32 = 3.5;
#[derive(Component)]
pub struct BridgeDeck;
#[derive(Component)]
pub struct BridgeHanger;
#[derive(Component)]
pub struct BridgeGirder;
#[derive(Component)]
pub struct BridgeOutrigger;
#[derive(Component)]
pub struct BridgeTower;
#[derive(Resource, serde::Serialize)]
pub struct BridgeCensus {
    pub span_m: f32,
    pub deck_half_width_m: f32,
    pub gap_centres_m: [f32; 2],
    pub gap_length_m: f32,
    pub tower_pairs: usize,
    pub railing_segments: usize,
    pub square_hangers: usize,
    pub hanger_width_m: f32,
    pub hanger_spacing_m: f32,
    pub edge_girder_segments: usize,
    pub deck_outriggers: usize,
}
/// Road-frame position without the off-road terrain fallback: bridge hardware
/// beyond the asphalt still belongs at deck elevation, not the canyon floor.
pub fn deck_sample(track: &Track, station: f32, lane: f32) -> (Vec3, Quat) {
    let (p, q) = track.sample(station, 0.);
    (p - q * Vec3::X * lane, q)
}
pub fn on_deck(station: f32) -> bool {
    (START..=END).contains(&station)
}
/// Rail only the actual turns, leaving straight approaches open for recovery.
pub fn curve_guardrail(track: &Track, station: f32) -> bool {
    if !(1580. ..5380.).contains(&station) || on_deck(station) {
        return false;
    }
    super::guardrails::side(track, station).is_some()
}
/// Keep farms, trees and distant rocks off the exposed canyon and bridge portals.
pub fn reserved(track: &Track, p: Vec3, radius: f32) -> bool {
    let (center, q) = track.sample(CENTER, 0.);
    let forward = (q * Vec3::Z).with_y(0.).normalize();
    let delta = (p - center).with_y(0.);
    delta.dot(forward).abs() < 195. + radius
        && delta.dot(Vec3::Y.cross(forward)).abs() < 190. + radius
}
pub fn canyon_ground_height(track: &Track, x: f32, z: f32) -> f32 {
    super::geometry::ground_height(track, x, z)
}
fn cable_point(track: &Track, station: f32, side: f32) -> Vec3 {
    let (p, _) = deck_sample(track, station, side * HANGER_LANE);
    let u = ((station - (START + 12.)) / (END - START - 24.)).clamp(0., 1.);
    p + Vec3::Y * (9. + 22. * (2. * u - 1.).powi(2))
}
fn hanger_foot(track: &Track, station: f32, side: f32) -> Vec3 {
    deck_sample(track, station, side * HANGER_LANE).0 - Vec3::Y * HANGER_FOOT_DROP
}
fn beam(
    commands: &mut Commands,
    art: &Art,
    material: Handle<StandardMaterial>,
    a: Vec3,
    b: Vec3,
    width: f32,
    depth: f32,
) -> Entity {
    let edge = b - a;
    let forward = edge.normalize();
    let horizontal_side = Vec3::Y.cross(forward);
    // Minimal Z-to-tangent rotation introduces roll on sloped, turned spans.
    // Keep deck/girder width horizontal, matching the unbanked road frame;
    // otherwise the wide slab misses the transverse attachment underneath it.
    let rotation = if horizontal_side.length_squared() > 0.0001 {
        let side = horizontal_side.normalize();
        Quat::from_mat3(&Mat3::from_cols(side, forward.cross(side), forward))
    } else {
        Quat::from_rotation_arc(Vec3::Z, forward)
    };
    let e = crate::world::part(
        commands,
        art.cube.clone(),
        material,
        (a + b) * 0.5,
        Vec3::new(width, depth, edge.length() + 0.04),
        rotation,
    );
    commands.entity(e).insert((
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5, 0.5),
        HillsSolid,
        Obstacle,
    ));
    e
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    art: &Art,
    track: &Track,
) {
    let steel = materials.add(StandardMaterial {
        base_color: Color::srgb(0.40, 0.13, 0.075),
        metallic: 0.45,
        perceptual_roughness: 0.65,
        ..default()
    });
    let deck = materials.add(Color::srgb(0.28, 0.29, 0.27));
    let hanger_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.33, 0.31, 0.26),
        metallic: 0.45,
        perceptual_roughness: 0.6,
        ..default()
    });
    let (origin, _) = track.sample(CENTER, 0.);
    let mut cable = Model::default();
    // Short solid deck ribs sit below the continuously tessellated asphalt.
    for d in (START as usize..END as usize).step_by(4) {
        let (a, _) = track.sample(d as f32, 0.);
        let (b, _) = track.sample(d as f32 + 4., 0.);
        // Adjacent chord slabs need extra longitudinal overlap at their outer
        // corners, where the road's changing tangent otherwise leaves seams.
        let overlap = (b - a).normalize() * 0.1;
        let e = beam(
            commands,
            art,
            deck.clone(),
            a - overlap - Vec3::Y * 0.43,
            b + overlap - Vec3::Y * 0.43,
            HALF_DECK * 2.,
            0.7,
        );
        commands.entity(e).insert((BridgeDeck, Ground));
    }
    // The user requested completely open deck edges. No rail or low post
    // geometry/collider is placed; square suspension members stand outboard.
    let mut square_hangers = 0;
    let mut edge_girder_segments = 0;
    let mut deck_outriggers = 0;
    for side in [-1., 1.] {
        // Two tall portals with solid towers outside the traffic and drop lanes.
        for station in [START + 12., END - 12.] {
            let (p, _) = deck_sample(track, station, side * 10.2);
            let floor = canyon_ground_height(track, p.x, p.z).min(p.y - 3.);
            let e = beam(
                commands,
                art,
                steel.clone(),
                p.with_y(floor - 1.),
                p + Vec3::Y * 31.,
                2.2,
                2.2,
            );
            commands.entity(e).insert(BridgeTower);
        }
        // The cable and hanger feet use one shared road frame. Continuous
        // edge girders below the road link every hanger to transverse outriggers;
        // none of these structural members adds an above-deck railing.
        for d in ((START + 12.) as usize..(END - 12.) as usize).step_by(4) {
            let a = cable_point(track, d as f32, side);
            let b = cable_point(track, d as f32 + 4., side);
            cable.rod(a - origin, b - origin, 0.23, [0.28, 0.27, 0.24]);
        }
        for d in (START as usize..END as usize).step_by(4) {
            let a = deck_sample(track, d as f32, side * HANGER_LANE).0;
            let b = deck_sample(track, d as f32 + 4., side * HANGER_LANE).0;
            let e = beam(
                commands,
                art,
                hanger_material.clone(),
                a - Vec3::Y * GIRDER_DROP,
                b - Vec3::Y * GIRDER_DROP,
                0.8,
                1.2,
            );
            commands.entity(e).insert(BridgeGirder);
            edge_girder_segments += 1;
        }
        for d in ((START + 12.) as usize..=(END - 12.) as usize).step_by(HANGER_SPACING as usize) {
            let station = d as f32;
            let foot = hanger_foot(track, station, side);
            let e = beam(
                commands,
                art,
                hanger_material.clone(),
                foot,
                cable_point(track, station, side),
                HANGER_WIDTH,
                HANGER_WIDTH,
            );
            commands.entity(e).insert(BridgeHanger);
            square_hangers += 1;
            let inner = deck_sample(track, station, side * (HALF_DECK - 0.4)).0;
            let outer = deck_sample(track, station, side * HANGER_LANE).0;
            let e = beam(
                commands,
                art,
                hanger_material.clone(),
                inner - Vec3::Y * GIRDER_DROP,
                outer - Vec3::Y * GIRDER_DROP,
                0.8,
                1.2,
            );
            commands.entity(e).insert(BridgeOutrigger);
            deck_outriggers += 1;
            // A flush bearing plate makes the load path visible at the joint.
            let e = crate::world::part(
                commands,
                art.cube.clone(),
                steel.clone(),
                foot + Vec3::Y * 0.04,
                Vec3::new(0.95, 0.18, 0.95),
                Quat::IDENTITY,
            );
            commands.entity(e).insert((
                RigidBody::Fixed,
                Collider::cuboid(0.5, 0.5, 0.5),
                HillsSolid,
                Obstacle,
            ));
        }
        // Anchorage backstays terminate beyond the canyon on solid approaches.
        for (tower, anchor) in [(START + 12., START - 18.), (END - 12., END + 18.)] {
            let (p, _) = deck_sample(track, anchor, side * 12.);
            let foot = p.with_y(canyon_ground_height(track, p.x, p.z));
            beam(
                commands,
                art,
                deck.clone(),
                foot - Vec3::Y * 0.2,
                foot + Vec3::Y * 1.6,
                3.,
                3.,
            );
            cable.rod(
                cable_point(track, tower, side) - origin,
                foot - origin + Vec3::Y * 1.6,
                0.23,
                [0.28, 0.27, 0.24],
            );
        }
    }
    for station in [START + 12., END - 12.] {
        let (a, _) = deck_sample(track, station, -10.2);
        let (b, _) = deck_sample(track, station, 10.2);
        beam(
            commands,
            art,
            steel.clone(),
            a + Vec3::Y * 28.,
            b + Vec3::Y * 28.,
            1.5,
            1.8,
        );
        beam(
            commands,
            art,
            steel.clone(),
            a + Vec3::Y * 19.,
            b + Vec3::Y * 28.,
            0.5,
            0.5,
        );
        beam(
            commands,
            art,
            steel.clone(),
            b + Vec3::Y * 19.,
            a + Vec3::Y * 28.,
            0.5,
            0.5,
        );
    }
    commands.spawn((
        Name::new("Original Hills suspension cables"),
        Mesh3d(meshes.add(cable.mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            metallic: 0.35,
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_translation(origin),
    ));
    commands.insert_resource(BridgeCensus {
        span_m: END - START,
        deck_half_width_m: HALF_DECK,
        gap_centres_m: GAP_STATIONS,
        gap_length_m: GAP_HALF_LENGTH * 2.,
        tower_pairs: 2,
        railing_segments: 0,
        square_hangers,
        hanger_width_m: HANGER_WIDTH,
        hanger_spacing_m: HANGER_SPACING,
        edge_girder_segments,
        deck_outriggers,
    });
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    pub(crate) fn scene() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
        ))
        .insert_resource(Time::<Fixed>::from_hz(60.))
        .insert_resource(TimestepMode::Fixed {
            dt: 1. / 60.,
            substeps: 1,
        })
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ))
        .insert_resource(Track::hills().unwrap())
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<StandardMaterial>>()
        .add_systems(
            Startup,
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             track: Res<Track>| {
                let art = Art {
                    props: None,
                    models: None,
                    cube: Handle::default(),
                    sphere: Handle::default(),
                    wheel: Handle::default(),
                    dark: Handle::default(),
                    metal: Handle::default(),
                    white: Handle::default(),
                    colors: vec![],
                    helmets: vec![],
                    bike_colors: vec![],
                };
                super::super::geometry::spawn(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &art,
                    &track,
                    &mut default(),
                    None,
                );
                spawn(&mut commands, &mut meshes, &mut materials, &art, &track);
            },
        );
        for _ in 0..3 {
            app.update();
        }
        app
    }
    fn boxes<T: Component>(app: &mut App) -> Vec<Transform> {
        app.world_mut()
            .query_filtered::<&Transform, (With<T>, With<Collider>)>()
            .iter(app.world())
            .copied()
            .collect()
    }
    fn contains_box(boxes: &[Transform], point: Vec3) -> bool {
        boxes.iter().any(|t| {
            let local = t.compute_affine().inverse().transform_point3(point);
            local.abs().max_element() <= 0.501
        })
    }
    fn assert_regular_connected_hangers(app: &mut App) {
        let hangers = boxes::<BridgeHanger>(app);
        let girders = boxes::<BridgeGirder>(app);
        let outriggers = boxes::<BridgeOutrigger>(app);
        let deck = boxes::<BridgeDeck>(app);
        let track = app.world().resource::<Track>();
        for slab in &deck {
            assert!(
                (slab.rotation * Vec3::X).y.abs() < 0.0001,
                "deck width must remain horizontal across the unbanked crossing"
            );
        }
        let stations: Vec<_> = ((START + 12.) as usize..=(END - 12.) as usize)
            .step_by(HANGER_SPACING as usize)
            .map(|s| s as f32)
            .collect();
        assert_eq!(hangers.len(), stations.len() * 2);
        assert_eq!(outriggers.len(), hangers.len());
        for side in [-1., 1.] {
            for &station in &stations {
                let foot = hanger_foot(track, station, side);
                let top = cable_point(track, station, side);
                let hanger = hangers
                    .iter()
                    .find(|t| t.translation.distance((foot + top) * 0.5) < 0.001)
                    .expect("a square hanger at every regular eight-metre station");
                let axis = hanger.rotation * Vec3::Z;
                let actual_foot = hanger.translation - axis * hanger.scale.z * 0.5;
                let actual_top = hanger.translation + axis * hanger.scale.z * 0.5;
                assert!(actual_foot.distance(foot) < 0.025);
                assert!(actual_top.distance(top) < 0.025);
                assert!(
                    contains_box(&girders, actual_foot),
                    "hanger must end inside its girder"
                );
                // The below-deck outrigger reaches both the girder and the
                // supported deck slab, not an unattached point in the canyon.
                let outer =
                    deck_sample(track, station, side * HANGER_LANE).0 - Vec3::Y * GIRDER_DROP;
                assert!(contains_box(&girders, outer));
                assert!(contains_box(&outriggers, outer));
                let inner =
                    deck_sample(track, station, side * (HALF_DECK - 0.4)).0 - Vec3::Y * 0.55;
                assert!(contains_box(&outriggers, inner));
                assert!(
                    contains_box(&deck, inner),
                    "deck must overlap its outrigger at station {station}, side {side}: inner {inner:?}, nearest {:?}",
                    deck.iter()
                        .min_by(|a, b| a
                            .translation
                            .distance_squared(inner)
                            .total_cmp(&b.translation.distance_squared(inner)))
                        .map(|t| (t, t.compute_affine().inverse().transform_point3(inner)))
                );
            }
        }
        // Fall fixtures use normal spaces between hangers, never artificial
        // twenty-four-metre omissions in the suspension structure.
        for s in GAP_STATIONS {
            assert!(stations.iter().all(|h| (s - h).abs() >= 4.));
        }
    }
    #[test]
    fn suspended_deck_has_no_railings_and_square_solid_hangers() {
        let mut app = scene();
        let samples: Vec<_> = [GAP_STATIONS[0], CENTER, GAP_STATIONS[1]]
            .into_iter()
            .map(|s| {
                let track = app.world().resource::<Track>();
                (s, deck_sample(track, s, 0.).0, deck_sample(track, s, 11.).0)
            })
            .collect();
        app.world_mut()
            .run_system_once(
                move |context: ReadRapierContext,
                      ground: Query<(), With<Ground>>,
                      obstacles: Query<(), (With<Obstacle>, Without<Ground>)>| {
                    let ctx = context.single().unwrap();
                    let supports =
                        |e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
                    let sides =
                        |e, _: &bevy_rapier3d::rapier::geometry::Collider| obstacles.contains(e);
                    for &(station, center, outside) in &samples {
                        let (_, road) = ctx
                            .cast_ray(
                                center + Vec3::Y * 3.,
                                Vec3::NEG_Y,
                                5.,
                                true,
                                QueryFilter::default().predicate(&supports),
                            )
                            .expect("continuous supported deck");
                        assert!((road - 2.975).abs() < 0.05);
                        let (_, canyon) = ctx
                            .cast_ray(
                                outside + Vec3::Y * 3.,
                                Vec3::NEG_Y,
                                120.,
                                true,
                                QueryFilter::default().predicate(&supports),
                            )
                            .expect("physical canyon floor");
                        assert!(
                            canyon > 65.,
                            "no hidden shoulder shelf at {station}: {canyon}"
                        );
                        let across = (outside - center).with_y(0.).normalize();
                        // Every deck edge is open, including the former protected
                        // centre span. Outboard structural hangers are separate.
                        for height in [0.48, 0.65, 1.10] {
                            for side in [-1., 1.] {
                                assert!(
                                    ctx.cast_ray(
                                        center + Vec3::Y * height,
                                        across * side,
                                        HALF_DECK + 0.3,
                                        true,
                                        QueryFilter::default().predicate(&sides),
                                    )
                                    .is_none(),
                                    "no hidden railing at {station}/{side}/{height}"
                                );
                            }
                        }
                    }
                },
            )
            .unwrap();
        let mut probes = Vec::new();
        for station in [GAP_STATIONS[0], CENTER, GAP_STATIONS[1]] {
            for side in [-1., 1.] {
                let track = app.world().resource::<Track>();
                let (p, q) = deck_sample(track, station, side * 5.);
                let height = p.y;
                let entity = app
                    .world_mut()
                    .spawn((
                        Transform::from_translation(p + Vec3::Y * 0.65),
                        RigidBody::Dynamic,
                        Collider::ball(0.4),
                        Ccd::enabled(),
                        Velocity::linear(q * Vec3::NEG_X * side * 16.),
                        Friction::coefficient(0.),
                        Restitution::coefficient(0.),
                    ))
                    .id();
                probes.push((entity, height));
            }
        }
        for _ in 0..180 {
            app.update();
        }
        for (entity, height) in probes {
            let y = app.world().get::<Transform>(entity).unwrap().translation.y;
            assert!(
                y < height - 20.,
                "lateral impact must leave the unrailed deck: {y}/{height}"
            );
        }
        assert_regular_connected_hangers(&mut app);
        let census = app.world().resource::<BridgeCensus>();
        assert_eq!(census.railing_segments, 0);
        assert!(census.square_hangers > 30);
        let expected_hangers = census.square_hangers;
        let mut hangers = app
            .world_mut()
            .query_filtered::<(&Transform, &Collider), With<BridgeHanger>>();
        let hangers: Vec<_> = hangers.iter(app.world()).collect();
        assert_eq!(hangers.len(), expected_hangers);
        for (transform, collider) in hangers {
            assert_eq!(transform.scale.x, HANGER_WIDTH);
            assert_eq!(transform.scale.y, HANGER_WIDTH);
            assert!(transform.scale.z > 9.);
            // Rapier has already baked the render transform's scale into the
            // collider after these fixed steps. Check the actual physical box,
            // not the unit cube supplied before physics synchronization.
            assert!(
                collider
                    .as_cuboid()
                    .unwrap()
                    .half_extents()
                    .abs_diff_eq(transform.scale * 0.5, 0.0001)
            );
        }
    }
}
