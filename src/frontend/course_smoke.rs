//! Native three-process menu selection, retry and return-to-menu proof.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};

#[derive(Resource, Default)]
struct Check {
    leg: u8,
    stage: u8,
    wait: u32,
    frames: u32,
}
pub(super) fn install(app: &mut App) {
    std::fs::create_dir_all("screenshots").expect("course evidence directory");
    let leg = match std::env::var("ROADDASH_OFFLINE_BOOT").as_deref() {
        Ok("hills") => 1,
        Ok("metro") => 2,
        _ => 0,
    };
    app.insert_resource(crate::game::Persistence(false))
        .insert_resource(Check { leg, ..default() })
        .add_systems(Update, run.after(super::resize));
}
fn capture(commands: &mut Commands, name: &str) {
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(format!("screenshots/{name}.png")));
}
fn advance(check: &mut Check) {
    check.stage += 1;
    check.wait = 0;
}
fn receipt(path: &str, profile: &Profile, track: &Track) {
    std::fs::write(path,serde_json::to_vec_pretty(&serde_json::json!({
        "map":track.definition.id.as_str(),"credits":profile.credits,"process":std::process::id()
    })).unwrap()).unwrap();
}
fn unchanged(profile: &Profile) {
    let prior: serde_json::Value =
        serde_json::from_slice(&std::fs::read("screenshots/course-handoff-hills.json").unwrap())
            .unwrap();
    assert_eq!(prior["credits"].as_u64(), Some(profile.credits as u64));
}
#[allow(clippy::too_many_arguments)]
fn run(
    mut check: ResMut<Check>,
    mut front: ResMut<Frontend>,
    mut race: ResMut<Race>,
    track: Res<Track>,
    mut profile: ResMut<Profile>,
    mut preferences: ResMut<crate::settings::Preferences>,
    mut tuning: ResMut<crate::vehicle::Tuning>,
    state: Res<State<AppScreen>>,
    server: Res<AssetServer>,
    art: Res<Art>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
    buttons: Query<(&MenuButton, &ComputedNode)>,
    reserved: Query<&ComputedNode, With<course::ReservedCourseSlot>>,
) {
    check.frames += 1;
    assert!(
        check.frames < 16000,
        "course selection native proof timed out"
    );
    if !ready(Some(&state), Some(&server), Some(&art)) {
        return;
    }
    check.wait += 1;
    if check.wait < 10 {
        return;
    }
    if check.leg > 0 {
        assert_eq!(front.flow.portrait, 7);
        assert_eq!(preferences.file.master_volume, 0.37);
        assert_eq!(tuning.selected, 0);
        assert_eq!(tuning.assist, 0.45);
    }
    match (check.leg, check.stage) {
        (0, 0) => {
            assert_eq!(course::loaded(&track), Some(Course::Metropolis));
            assert_eq!(front.flow.page, Page::Splash);
            front.flow.portrait = 7;
            preferences.file.master_volume = 0.37;
            tuning.selected = 0;
            tuning.assist = 0.45;
            profile.credits = 987;
            front.pending = Some(Action::Continue);
            advance(&mut check);
        }
        (0, 1) => {
            assert_eq!(front.flow.page, Page::Main);
            front.pending = Some(Action::Thrash);
            advance(&mut check);
        }
        (0, 2) => {
            assert_eq!(front.flow.page, Page::Courses);
            assert_eq!(buttons.iter().count(), 4);
            assert_eq!(reserved.iter().count(), 4);
            assert!(reserved.iter().all(|node| node.size().min_element() > 0.));
            assert!(buttons.iter().all(|(button, _)| match button.action {
                Action::SelectCourse(course) => course::SELECTABLE.contains(&course),
                _ => true,
            }));
            assert!(
                buttons
                    .iter()
                    .all(|(_, n)| n.size().x > 0. && n.size().y > 0.)
            );
            if check.wait == 10 {
                capture(&mut commands, "course-menu-metropolis");
            }
            if check.wait > 25 {
                front.pending = Some(Action::SelectCourse(Course::Hills));
                advance(&mut check);
            }
        }
        (0, 3) => {
            assert_eq!(front.flow.course, Some(Course::Hills));
            assert_eq!(course::loaded(&track), Some(Course::Metropolis));
            assert_eq!(race.phase, Phase::Garage);
            if check.wait == 10 {
                capture(&mut commands, "course-menu-hills");
            }
            if check.wait > 25 {
                receipt("screenshots/course-handoff-hills.json", &profile, &track);
                front.pending = Some(Action::Ride);
                advance(&mut check);
            }
        }
        (1, 0) => {
            assert_eq!(course::loaded(&track), Some(Course::Hills));
            unchanged(&profile);
            if race.phase != Phase::Racing || check.wait < 240 {
                return;
            }
            capture(&mut commands, "course-hills-riding");
            advance(&mut check);
        }
        (1, 1) => {
            race.phase = Phase::Finished;
            advance(&mut check);
        }
        (1, 2) => {
            assert_eq!(front.flow.page, Page::Results);
            front.pending = Some(Action::Ride);
            advance(&mut check);
        }
        (1, 3) => {
            if !matches!(race.phase, Phase::Countdown | Phase::Racing) {
                return;
            }
            assert!(
                front.course_load.is_none() && !front.handoff_started,
                "same-course retry must not reload"
            );
            unchanged(&profile);
            advance(&mut check);
        }
        (1, 4) => {
            race.phase = Phase::Finished;
            advance(&mut check);
        }
        (1, 5) => {
            assert_eq!(front.flow.page, Page::Results);
            front.pending = Some(Action::Courses);
            advance(&mut check);
        }
        (1, 6) => {
            assert_eq!(race.phase, Phase::Garage);
            assert_eq!(front.flow.page, Page::Courses);
            front.pending = Some(Action::SelectCourse(Course::Metropolis));
            advance(&mut check);
        }
        (1, 7) => {
            assert_eq!(front.flow.course, Some(Course::Metropolis));
            if check.wait == 10 {
                capture(&mut commands, "course-return-to-metropolis");
            }
            if check.wait > 25 {
                receipt("screenshots/course-handoff-metro.json", &profile, &track);
                front.pending = Some(Action::Ride);
                advance(&mut check);
            }
        }
        (2, 0) => {
            assert_eq!(course::loaded(&track), Some(Course::Metropolis));
            unchanged(&profile);
            if race.phase != Phase::Racing || check.wait < 240 {
                return;
            }
            capture(&mut commands, "course-metropolis-riding");
            advance(&mut check);
        }
        (2, 1) => {
            race.phase = Phase::Finished;
            advance(&mut check);
        }
        (2, 2) => {
            assert_eq!(front.flow.page, Page::Results);
            front.pending = Some(Action::Back);
            advance(&mut check);
        }
        (2, 3) => {
            assert_eq!(race.phase, Phase::Garage);
            assert_eq!(front.flow.page, Page::Main);
            unchanged(&profile);
            if check.wait == 10 {
                capture(&mut commands, "course-return-main-menu");
            }
            if check.wait > 30 {
                std::fs::write("screenshots/course-selector-report.json",serde_json::to_vec_pretty(&serde_json::json!({
                    "verified":true,"featured_courses":["Metropolis","Hills"],"reserved_slots":4,
                    "loaded_sequence":["Metropolis","Hills","Metropolis"],"same_course_retry_without_reload":true,
                    "return_to_menu":true,"profile_credits_preserved":true,"live_portrait_preferences_handling_preserved":true,"screenshots":6,
                    "limits":"Native menu actions and world handoff; legacy map handoffs covered by command/identity tests, not a full driven race."
                })).unwrap()).unwrap();
                exit.write(AppExit::Success);
            }
        }
        _ => {
            assert!(
                check.wait < 180,
                "world handoff failed to exit originating process"
            );
        }
    }
}
