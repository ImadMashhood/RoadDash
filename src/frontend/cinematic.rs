//! Menu-only city tour. Display motorcycles have no Rider, collider or race state.
use super::*;
#[derive(Component)]
pub(super) struct DisplayBike(usize);

pub(super) fn enabled(front: &Frontend, race: &Race) -> bool {
    front.owns(race.phase)
        && !front.online_pause
        && matches!(
            front.flow.page,
            Page::Splash | Page::Main | Page::Street | Page::Courses
        )
}
fn station(track: &Track, seconds: f32) -> f32 {
    let start = if track.metropolis.is_some() {
        9_250.
    } else {
        120.
    };
    start + 95. * (1. - (seconds * 0.09).cos())
}
#[allow(clippy::too_many_arguments)]
pub(super) fn bikes(
    mut commands: Commands,
    art: Res<crate::world::Art>,
    front: Res<Frontend>,
    race: Res<Race>,
    track: Res<Track>,
    time: Res<Time>,
    prefs: Res<crate::settings::Preferences>,
    mut bikes: Query<(&DisplayBike, &mut Transform, &mut Visibility)>,
    mut initialized: Local<bool>,
) {
    if !*initialized {
        for i in 0..3 {
            let e = crate::world::bike(&mut commands, &art, i + 1, true);
            commands
                .entity(e)
                .insert((DisplayBike(i), Visibility::Hidden));
        }
        *initialized = true;
    }
    let active = enabled(&front, &race);
    let seconds = if prefs.file.static_menu || prefs.file.comfort.anti_nausea {
        0.0
    } else {
        time.elapsed_secs()
    };
    let center = station(&track, seconds);
    for (bike, mut pose, mut visible) in &mut bikes {
        *visible = if active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if !active {
            continue;
        }
        let offset =
            (seconds * (9.0 + bike.0 as f32) + bike.0 as f32 * 38.0).rem_euclid(160.0) - 45.0;
        let (p, q) = track.street_sample(0, center + offset, -2.0 + bike.0 as f32 * 1.8);
        *pose = Transform::from_translation(p).with_rotation(q);
    }
}
pub(super) fn camera(
    front: Res<Frontend>,
    race: Res<Race>,
    track: Res<Track>,
    time: Res<Time>,
    prefs: Res<crate::settings::Preferences>,
    mut cameras: Query<&mut Transform, With<crate::game::ChaseCamera>>,
) {
    if !enabled(&front, &race) {
        return;
    }
    let seconds = if prefs.file.static_menu || prefs.file.comfort.anti_nausea {
        0.0
    } else {
        time.elapsed_secs()
    };
    let d = station(&track, seconds);
    let phase = seconds * 0.09;
    let (p, q) = track.street_sample(0, d, 0.);
    let forward = (q * Vec3::Z).with_y(0.).normalize_or_zero();
    let right = Vec3::Y.cross(forward);
    let (target, _) = track.street_sample(0, d + 28., 0.);
    let eye =
        p - forward * 17. + right * (5. * phase.sin()) + Vec3::Y * (10. + 3. * (1. - phase.cos()));
    for mut t in &mut cameras {
        *t = Transform::from_translation(eye).looking_at(target + Vec3::Y * 2., Vec3::Y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flight_station_moves_and_returns_to_its_start() {
        for track in [Track::metropolis().unwrap(), Track::hills().unwrap()] {
            let start = station(&track, 0.);
            let period = std::f32::consts::TAU / 0.09;
            assert!(station(&track, period * 0.5) > start + 180.);
            assert!((station(&track, period) - start).abs() < 0.001);
        }
    }

    #[test]
    fn menu_tour_is_presentation_only_and_hides_on_race_entry() {
        let mut app = crate::game::tests::simulation();
        app.init_resource::<Frontend>();
        app.world_mut().resource_mut::<Frontend>().flow.page = Page::Main;
        app.world_mut().resource_mut::<Race>().phase = Phase::Garage;
        let count = app
            .world_mut()
            .query::<&crate::game::Rider>()
            .iter(app.world())
            .count();
        let mut schedule = Schedule::default();
        schedule.add_systems(bikes);
        schedule.run(app.world_mut());
        let created = app
            .world_mut()
            .query::<&DisplayBike>()
            .iter(app.world())
            .count();
        assert_eq!(created, 3);
        schedule.run(app.world_mut());
        assert_eq!(
            created,
            app.world_mut()
                .query::<&DisplayBike>()
                .iter(app.world())
                .count()
        );
        assert_eq!(
            count,
            app.world_mut()
                .query::<&crate::game::Rider>()
                .iter(app.world())
                .count()
        );
        assert!(app.world_mut().query_filtered::<Entity, (With<DisplayBike>, With<bevy_rapier3d::prelude::Collider>)>().iter(app.world()).next().is_none());
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        schedule.run(app.world_mut());
        assert!(
            app.world_mut()
                .query_filtered::<&Visibility, With<DisplayBike>>()
                .iter(app.world())
                .all(|v| *v == Visibility::Hidden)
        );
    }
}
