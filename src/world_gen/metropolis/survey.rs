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
    crossing_walkers: Query<
        'w,
        's,
        (
            &'static super::pedestrians::CrossingWalker,
            &'static Transform,
        ),
        Without<Camera3d>,
    >,
    grass: Query<
        'w,
        's,
        (
            &'static geometry::landscape::GrassBatch,
            &'static ViewVisibility,
        ),
    >,
    cars: Query<
        'w,
        's,
        (
            &'static crate::game::Traffic,
            &'static crate::game::TrafficYield,
            Option<&'static ColliderDisabled>,
        ),
    >,
    walkers: Query<'w, 's, (Entity, &'static MetroWalker, &'static GlobalTransform)>,
    farm_props: Query<
        'w,
        's,
        (
            &'static crate::props::PropVisual,
            Option<&'static bevy::scene::SceneInstance>,
        ),
    >,
    spawner: Res<'w, bevy::scene::SceneSpawner>,
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
    let grass = &actors.grass;
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
    let civic_view = std::env::var_os("ROADDASH_CIVIC_VIEWS").is_some();
    let landscape_view = std::env::var_os("ROADDASH_LANDSCAPE_VIEWS").is_some();
    let scenes = [
        (0, -280., false),
        (0, 900., false),
        (0, 3100., false),
        (0, 3250., false), // Four-lane city entry, before the 3350m split.
        (0, 3500., false),
        (0, 4300., false),
        (1, 4100., false),
        (0, 6800., false),
        (0, 6800., true),
        (0, 8500., false),
        (0, 8500., true),
        (0, 10400., false),
        (0, 10400., true),
        (0, 11900., false),
    ];
    let period = if std::env::var_os("ROADDASH_CROSSING_SURVEY").is_some() {
        300
    } else if kit.is_some() {
        100
    } else {
        55
    };
    let index = ((state.frame / period) as usize).min(scenes.len() - 1);
    let (route, mut s, aerial) = scenes[index];
    if std::env::var_os("ROADDASH_ROAD_EDGE_VIEWS").is_some() {
        if index == 1 {
            s = 2920.;
        }
        if index == 2 {
            s = 2940.;
        }
    }
    let (p, q) = track.sample_route(route, s, 0.);
    for mut camera in &mut cameras {
        let eye = if landscape_view && index < 2 {
            p + q * Vec3::X * 15. - q * Vec3::Z * 10. + Vec3::Y * 1.8
        } else if aerial {
            p + Vec3::new(260., 260., -210.)
        } else {
            p - q * Vec3::Z * 17. + Vec3::Y * 6.
        };
        let target = if aerial {
            p + Vec3::Y * 15.
        } else {
            p + q * Vec3::Z * 30. + Vec3::Y * 6.
        };
        let (eye, target) = if civic_view && index < 2 {
            let site = &track.metropolis.as_ref().unwrap().civic_sites[index];
            let center = Vec3::new(
                site.position[0],
                track.terrain_height(site.position[0], site.position[1]),
                site.position[1],
            );
            (
                center
                    + if index == 0 {
                        Vec3::new(180., 105., -160.)
                    } else {
                        Vec3::new(150., 165., -190.)
                    },
                center + Vec3::Y * if index == 0 { 38. } else { 1. },
            )
        } else {
            (eye, target)
        };
        // Optional repeatable architectural review cameras share the hall's
        // authored heading; they do not move any simulation object.
        let (eye, target) = if std::env::var_os("ROADDASH_HALL_VIEWS").is_some() && index < 5 {
            let site = &track.metropolis.as_ref().unwrap().civic_sites[0];
            let center = Vec3::new(
                site.position[0],
                track.terrain_height(site.position[0], site.position[1]),
                site.position[1],
            );
            let q = Quat::from_rotation_y(site.heading);
            let (offset, height) = [
                (Vec3::new(130., 100., 200.), 40.),
                (Vec3::new(0., 6., 95.), 36.),
                (Vec3::new(-150., 70., -140.), 45.),
                (Vec3::new(60., 100., 100.), 78.),
                (Vec3::new(0., 65., 120.), 35.),
            ][index];
            (center + q * offset, center + Vec3::Y * height)
        } else {
            (eye, target)
        };
        let (eye, target) = if std::env::var_os("ROADDASH_CITY_PEDESTRIAN_VIEWS").is_some()
            && matches!(index, 5 | 6 | 7 | 9)
        {
            (
                p - q * Vec3::Z * 12. + Vec3::Y * 1.8,
                p + q * Vec3::Z * 30. + Vec3::Y * 1.8,
            )
        } else {
            (eye, target)
        };
        *camera = Transform::from_translation(eye).looking_at(target, Vec3::Y);
    }
    if landscape_view && (state.frame == period + 35 || state.frame == period + 85) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/grass-wind-{}.png",
                state.frame
            )));
    }
    if state.frame % period == period - 5 {
        let nearby_walkers = walkers
            .iter()
            .filter(|(_, _, t)| t.translation().xz().distance(p.xz()) <= 80.)
            .count();
        let nearby_moving = walkers
            .iter()
            .filter(|(e, _, t)| {
                t.translation().xz().distance(p.xz()) <= 80.
                    && state
                        .walker_start
                        .get(e)
                        .is_some_and(|start| start.distance(t.translation()) > 0.5)
            })
            .count();
        if let Some(ref kit) = kit {
            state.views.push(serde_json::json!({"view":index,"route":route,"chainage":s,"aerial":aerial,"walkers_within_80m":nearby_walkers,"walkers_moving_within_80m":nearby_moving,"grass_visible_batches":grass.iter().filter(|(_,v)|v.get()).count(),"grass_visible_triangles":grass.iter().filter(|(_,v)|v.get()).map(|(b,_)|b.triangles).sum::<usize>(),"kit":kit.evidence(),"native_draw_audit":if kit.mesh_instances > 0 { kit.audit(authored.iter(), &meshes) } else { serde_json::Value::Null }}));
        }
        if std::env::var("ROADDASH_SURVEY_CAPTURE").as_deref() != Ok("0") {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!("screenshots/metro-{index:02}.png")));
        }
    }
    if state.frame > period * scenes.len() as u32 + 20 {
        commands.insert_resource(crate::diagnostics::FixtureComplete);
        let _verification_span =
            crate::diagnostics::loading::Span::new("metro_survey_verification");
        let farm_ready = actors
            .farm_props
            .iter()
            .filter(|(p, s)| {
                p.family == "farm" && s.is_some_and(|s| actors.spawner.instance_is_ready(**s))
            })
            .count();
        assert_eq!(
            farm_ready, census.farm_props,
            "farm prop scenes must be ready"
        );
        let farm_plan: Vec<_> = super::farms::plan(&track).iter().map(|p| serde_json::json!({"name":p.name,"position":p.position.to_array(),"size":p.size.to_array(),"clearance_radius":p.radius})).collect();
        std::fs::write(
            "screenshots/farm-placements.json",
            serde_json::to_string_pretty(
                &serde_json::json!({"ready_prop_scenes":farm_ready,"placements":farm_plan}),
            )
            .unwrap(),
        )
        .unwrap();
        let m = track.metropolis.as_ref().unwrap();
        assert_eq!(cars.iter().count(), m.street_life.main_traffic);
        let urban: Vec<_> = cars.iter().filter(|(_, y, _)| y.city_only).map(|(c, _, _)| serde_json::json!({"progress":c.progress,"route":c.route,"speed":c.speed})).collect();
        assert_eq!(urban.len(), m.street_life.main_traffic - 64);
        assert!(
            cars.iter()
                .filter(|(_, y, _)| y.city_only)
                .all(|(c, _, _)| c.progress >= 3250.)
        );
        std::fs::write(
            "screenshots/urban-extra-cars.json",
            serde_json::to_vec_pretty(&urban).unwrap(),
        )
        .unwrap();
        assert_eq!(walkers.iter().count(), m.pedestrians.len());
        std::fs::write("screenshots/pedestrian-crossings.json",serde_json::to_vec_pretty(&serde_json::json!({"authored":m.pedestrians.iter().filter(|p|p.crossing_end.is_some()).count(),"started":actors.crossing_walkers.iter().map(|(w,_)|w.started).sum::<u32>(),"completed":actors.crossing_walkers.iter().map(|(w,_)|w.completed).sum::<u32>(),"active":actors.crossing_walkers.iter().filter(|(w,_)|w.active).count(),"max_active":rr_core::pedestrian_crossing::MAX_ACTIVE,"release_interval_s":rr_core::pedestrian_crossing::RELEASE_INTERVAL,"walkers":actors.crossing_walkers.iter().map(|(w,t)|serde_json::json!({"start":w.start.to_array(),"end":w.end.to_array(),"position":t.translation.to_array(),"started":w.started,"completed":w.completed})).collect::<Vec<_>>()})).unwrap()).unwrap();
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
        let out = cars.iter().filter(|(c, _, _)| c.speed > 0.).count();
        let incoming = cars.iter().filter(|(c, _, _)| c.speed < 0.).count();
        assert!(out > 0 && incoming > 0);
        let mut floors = std::collections::BTreeMap::new();
        for b in &m.buildings {
            *floors.entry(b.floors).or_insert(0usize) += 1;
        }
        std::fs::write("screenshots/metro-street-life.json",serde_json::to_string_pretty(&serde_json::json!({"main_street_cars":cars.iter().count(),"collision_active_main_cars":cars.iter().filter(|(_,_,d)|d.is_none()).count(),"outgoing":out,"oncoming":incoming,"parked":census.parked_cars,"walking_pedestrians":census.walking_pedestrians,"street_furniture":census.street_props,"lane_counts":m.street_life.lane_sections.iter().map(|p|p.lanes()).collect::<Vec<_>>(),"height_histogram_floors":floors,"height_min_m":m.buildings.iter().map(|b|b.height).fold(f32::INFINITY,f32::min),"height_max_m":m.buildings.iter().map(|b|b.height).fold(0f32,f32::max)})).unwrap()).unwrap();
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
