use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    ready: bool,
    views: Vec<serde_json::Value>,
    walker_start: std::collections::HashMap<Entity, Vec3>,
}
#[derive(bevy::ecs::system::SystemParam)]
pub struct StreetActors<'w, 's> {
    cars: Query<
        'w,
        's,
        (
            &'static crate::game::Traffic,
            Option<&'static ColliderDisabled>,
        ),
    >,
    walkers: Query<'w, 's, (Entity, &'static MetroWalker, &'static GlobalTransform)>,
}
#[allow(clippy::too_many_arguments)]
pub fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    census: Res<Census>,
    mut race: ResMut<crate::game::Race>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    mut exit: MessageWriter<AppExit>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    metrics: Res<crate::diagnostics::Metrics>,
    clock: Res<crate::runtime::SimulationClock>,
    chunks: Query<&Mesh3d, With<MetroChunk>>,
    kit: Option<Res<super::facades::Kit>>,
    authored: Query<(&Mesh3d, Option<&Collider>), With<super::facades::AuthoredMesh>>,
    meshes: Res<Assets<Mesh>>,
    actors: StreetActors,
) {
    let cars = &actors.cars;
    let walkers = &actors.walkers;
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    for (entity, parent) in &ui {
        if parent.is_none() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    state.ready = true;
    state.frame += 1;
    race.phase = if state.frame > 435 {
        crate::game::Phase::Racing
    } else {
        crate::game::Phase::Paused
    };
    assert_eq!(
        chunks.iter().count(),
        census.chunks,
        "resident building/ground geometry changed with camera"
    );
    if state.frame == 500 {
        state.walker_start = walkers
            .iter()
            .map(|(e, _, t)| (e, t.translation()))
            .collect();
    }
    let scenes = [
        (0, -280., false),
        (0, 900., false),
        (0, 3100., false),
        (0, 3250., false), // Four-lane city entry, before the 3350m split.
        (0, 3500., false),
        (0, 4300., false),
        (1, 4300., false),
        (0, 6800., false),
        (0, 6800., true),
        (0, 8500., false),
        (0, 8500., true),
        (0, 10400., false),
        (0, 10400., true),
        (0, 11900., false),
    ];
    let period = if kit.is_some() { 100 } else { 55 };
    let index = ((state.frame / period) as usize).min(scenes.len() - 1);
    let (route, s, aerial) = scenes[index];
    let (p, q) = track.sample_route(route, s, 0.);
    for mut camera in &mut cameras {
        let eye = if aerial {
            p + Vec3::new(260., 260., -210.)
        } else {
            p - q * Vec3::Z * 17. + Vec3::Y * 6.
        };
        let target = if aerial {
            p + Vec3::Y * 15.
        } else {
            p + q * Vec3::Z * 30. + Vec3::Y * 6.
        };
        *camera = Transform::from_translation(eye).looking_at(target, Vec3::Y);
    }
    if state.frame % period == period - 5 {
        if let Some(ref kit) = kit {
            state.views.push(serde_json::json!({"view":index,"route":route,"chainage":s,"aerial":aerial,"kit":kit.evidence(),"native_draw_audit":if kit.mesh_instances > 0 { kit.audit(authored.iter(), &meshes) } else { serde_json::Value::Null }}));
        }
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/metro-{index:02}.png")));
    }
    if state.frame > period * scenes.len() as u32 + 20 {
        let m = track.metropolis.as_ref().unwrap();
        assert_eq!(cars.iter().count(), m.street_life.main_traffic);
        assert_eq!(walkers.iter().count(), m.pedestrians.len());
        let moved = walkers
            .iter()
            .filter(|(e, _, t)| {
                state
                    .walker_start
                    .get(e)
                    .is_some_and(|p| p.distance(t.translation()) > 0.5)
            })
            .count();
        assert!(
            moved > 80,
            "downtown pedestrians must actually patrol, not just spawn"
        );
        let mut bands = std::collections::BTreeMap::new();
        for (_, walker, _) in walkers {
            *bands
                .entry((walker.station / 1000.) as usize)
                .or_insert(0usize) += 1;
        }
        std::fs::write("screenshots/metro-pedestrians.json", serde_json::to_string_pretty(&serde_json::json!({"spawned":walkers.iter().count(),"moved_over_half_metre":moved,"per_kilometre":bands})).unwrap()).unwrap();
        let out = cars.iter().filter(|(c, _)| c.speed > 0.).count();
        let incoming = cars.iter().filter(|(c, _)| c.speed < 0.).count();
        assert!(out > 0 && incoming > 0);
        let mut floors = std::collections::BTreeMap::new();
        for b in &m.buildings {
            *floors.entry(b.floors).or_insert(0usize) += 1;
        }
        std::fs::write("screenshots/metro-street-life.json",serde_json::to_string_pretty(&serde_json::json!({"main_street_cars":cars.iter().count(),"collision_active_main_cars":cars.iter().filter(|(_,d)|d.is_none()).count(),"outgoing":out,"oncoming":incoming,"parked":census.parked_cars,"walking_pedestrians":census.walking_pedestrians,"street_furniture":census.street_props,"lane_counts":m.street_life.lane_sections.iter().map(|p|p.lanes()).collect::<Vec<_>>(),"height_histogram_floors":floors,"height_min_m":m.buildings.iter().map(|b|b.height).fold(f32::INFINITY,f32::min),"height_max_m":m.buildings.iter().map(|b|b.height).fold(0f32,f32::max)})).unwrap()).unwrap();
        if let Some(kit) = kit {
            assert!(kit.ready() && kit.mesh_instances > 0 && kit.triangles > 0);
            std::fs::write(
                "screenshots/metro-authored-facades.json",
                serde_json::to_string_pretty(&serde_json::json!({"kit":kit.evidence(),"views":state.views,"native_draw_audit":kit.audit(authored.iter(), &meshes)})).unwrap(),
            )
            .unwrap();
        }

        assert_eq!(
            census.buildings,
            track.metropolis.as_ref().unwrap().buildings.len()
        );
        assert!(
            census.chunks > 100 && census.collision_triangles > 100000 && census.crossing_cars > 0
        );
        std::fs::write(
            "screenshots/metro-survey.json",
            serde_json::to_string_pretty(&*census).unwrap(),
        )
        .unwrap();
        std::fs::write("screenshots/metro-performance.json",serde_json::to_string_pretty(&serde_json::json!({"metrics":metrics.evidence(&clock),"census":&*census,"p95_target_ms":16.7,"p95_target_met":metrics.p95_ms<=16.7,"limits":"Bounded native wall-frame sample; fifteen racers simulated in farm section, render camera at urban finish; not GPU timing or full-course/hardware certification."})).unwrap()).unwrap();
        exit.write(AppExit::Success);
    }
}
