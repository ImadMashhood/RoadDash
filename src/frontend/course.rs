//! Offline course cards and the existing process-isolated world-load boundary.
use super::*;
use std::{ffi::OsString, process::Command};

#[derive(Component)]
pub(super) struct CourseCard(pub Course);

pub(super) const SELECTABLE: [Course; 2] = [Course::Metropolis, Course::Hills];
const SLOT_COUNT: usize = 6;

#[derive(Component)]
pub(super) struct ReservedCourseSlot;

fn slot_area(index: usize) -> Node {
    area([
        5. + (index % 3) as f32 * 30.5,
        22. + (index / 3) as f32 * 29.,
        29.,
        26.,
    ])
}

pub(super) fn loaded(track: &Track) -> Option<Course> {
    if track.is_hills() {
        Some(Course::Hills)
    } else if track.metropolis.is_some() {
        Some(Course::Metropolis)
    } else if track.city_surface.is_some() {
        Some(Course::City)
    } else if track.definition.id.as_str() == "valley-run-01" {
        Some(Course::Valley)
    } else {
        None
    }
}

/// A previous explicit Hills launch must not override the newly selected map.
/// Preserve handling, recovery, diagnostics and user configuration arguments.
pub(crate) fn map_launch_arguments(args: impl IntoIterator<Item = OsString>) -> Vec<OsString> {
    args.into_iter()
        .filter(|arg| {
            !matches!(
                arg.to_str(),
                Some(
                    "--hills"
                        | "--hills-survey"
                        | "--wildlife-survey"
                        | "--hills-polish-survey"
                        | "--hills-revision-survey"
                        | "--metro-survey"
                )
            )
        })
        .collect()
}
fn command(course: Course) -> Result<Command, String> {
    let mut cmd = Command::new(std::env::current_exe().map_err(|e| e.to_string())?);
    cmd.args(map_launch_arguments(std::env::args_os().skip(1)))
        .env("ROADDASH_TRACK", course.key())
        .env("ROADDASH_OFFLINE_BOOT", course.key());
    for key in [
        "ROADDASH_ONLINE_BOOT",
        "ROADDASH_EXPECTED_GUESTS",
        "ROADDASH_HOST_SECRET",
        "ROADDASH_NAT_REUSE_PORT",
        "ROADDASH_NAT_TOKEN",
    ] {
        cmd.env_remove(key);
    }
    Ok(cmd)
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Session {
    portrait: usize,
    street_mode: bool,
    preferences: crate::settings::PreferencesFile,
    preset: usize,
    assist: f32,
    races: u32,
    wins: u32,
    credits: u32,
    best_seconds: Option<f32>,
}
impl Session {
    fn capture(
        front: &Frontend,
        preferences: &crate::settings::Preferences,
        tuning: &crate::vehicle::Tuning,
        profile: &Profile,
    ) -> Self {
        Self {
            portrait: front.flow.portrait,
            street_mode: front.flow.mode == rr_core::frontend::Mode::Street,
            preferences: preferences.file.clone(),
            preset: tuning.selected,
            assist: tuning.assist,
            races: profile.races,
            wins: profile.wins,
            credits: profile.credits,
            best_seconds: profile.best_seconds,
        }
    }
    fn apply(
        self,
        front: &mut Frontend,
        preferences: &mut crate::settings::Preferences,
        tuning: &mut crate::vehicle::Tuning,
        profile: &mut Profile,
        bindings: &mut crate::controls::Bindings,
    ) -> Result<(), String> {
        self.preferences.validate()?;
        if self.portrait >= ROSTER.len()
            || self.preset >= tuning.presets.len()
            || !self.assist.is_finite()
            || !(0. ..=1.).contains(&self.assist)
            || self.best_seconds.is_some_and(|s| !s.is_finite() || s < 0.)
        {
            return Err("invalid course session state".into());
        }
        let keys = crate::controls::Bindings::parse(
            &serde_json::to_string(&self.preferences.keyboard).map_err(|e| e.to_string())?,
        )?;
        front.flow.portrait = self.portrait;
        front.flow.mode = if self.street_mode {
            rr_core::frontend::Mode::Street
        } else {
            rr_core::frontend::Mode::Thrash
        };
        preferences.file = self.preferences;
        *bindings = keys;
        tuning.selected = self.preset;
        tuning.assist = self.assist;
        // Retain loaded file paths/read-only guards; only carry live progress values.
        profile.races = self.races;
        profile.wins = self.wins;
        profile.credits = self.credits;
        profile.best_seconds = self.best_seconds;
        Ok(())
    }
}
pub(super) fn restore_session(
    mut front: ResMut<Frontend>,
    mut preferences: ResMut<crate::settings::Preferences>,
    mut tuning: ResMut<crate::vehicle::Tuning>,
    mut profile: ResMut<Profile>,
    mut bindings: ResMut<crate::controls::Bindings>,
) {
    if std::env::var_os("ROADDASH_ONLINE_BOOT").is_some()
        || std::env::var_os("ROADDASH_OFFLINE_BOOT").is_none()
    {
        return;
    }
    let Ok(json) = std::env::var("ROADDASH_OFFLINE_STATE") else {
        return;
    };
    let result = serde_json::from_str::<Session>(&json)
        .map_err(|e| e.to_string())
        .and_then(|s| {
            s.apply(
                &mut front,
                &mut preferences,
                &mut tuning,
                &mut profile,
                &mut bindings,
            )
        });
    if let Err(error) = result {
        warn!("Course session state rejected: {error}");
    }
}
pub(super) fn handoff(
    mut front: ResMut<Frontend>,
    mut exit: MessageWriter<AppExit>,
    preferences: Res<crate::settings::Preferences>,
    tuning: Res<crate::vehicle::Tuning>,
    profile: Res<Profile>,
) {
    let Some(course) = front.course_load.take() else {
        return;
    };
    let session = Session::capture(&front, &preferences, &tuning, &profile);
    let result = serde_json::to_string(&session)
        .map_err(|e| e.to_string())
        .and_then(|json| {
            command(course).and_then(|mut cmd| {
                cmd.env("ROADDASH_OFFLINE_STATE", json)
                    .spawn()
                    .map_err(|e| e.to_string())
            })
        });
    match result {
        Ok(_) => {
            front.handoff_started = true;
            exit.write(AppExit::Success);
        }
        Err(error) => {
            front.course_error = Some(format!("Could not open this road: {error}"));
            front.flow.page = Page::Courses;
            front.released = false;
        }
    }
}

fn area([x, y, w, h]: [f32; 4]) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: percent(x),
        top: percent(y),
        width: percent(w),
        height: percent(h),
        ..default()
    }
}
fn label(
    commands: &mut Commands,
    parent: Entity,
    value: &str,
    size: f32,
    rect: [f32; 4],
    color: Color,
) {
    let e = text(commands, value, size, area(rect), color);
    commands.entity(parent).add_child(e);
}
pub(super) fn render(
    commands: &mut Commands,
    canvas: Entity,
    front: &Frontend,
    track: &Track,
    art: &Art,
) {
    let paper = layout::PAPER;
    let selected = front
        .flow
        .course
        .or_else(|| loaded(track))
        .filter(|course| SELECTABLE.contains(course))
        .unwrap_or(Course::Metropolis);
    let panel = commands
        .spawn((
            area([0., 0., 100., 100.]),
            BackgroundColor(Color::srgba(0.012, 0.018, 0.021, 0.92)),
        ))
        .id();
    commands.entity(canvas).add_child(panel);
    label(
        commands,
        canvas,
        "ROAD / DASH   /   OFFLINE RACING",
        16.,
        [5., 4., 90., 4.],
        layout::ACCENT,
    );
    label(
        commands,
        canvas,
        "PICK YOUR ROAD",
        48.,
        [5., 9., 90., 9.],
        paper,
    );
    for (index, (course, distance, route)) in [
        (Course::Metropolis, "11 KM", "FARMLAND  /  CITY STREETS"),
        (Course::Hills, "12.5 KM", "CITY  /  HILLS  /  DUNES"),
    ]
    .into_iter()
    .enumerate()
    {
        let mut node = slot_area(index);
        node.border = UiRect::all(px(3.));
        node.overflow = Overflow::clip();
        let card = commands
            .spawn((
                Button,
                CourseCard(course),
                MenuButton {
                    index,
                    action: Action::SelectCourse(course),
                },
                node,
                BackgroundColor(Color::BLACK),
                BorderColor::all(Color::NONE),
            ))
            .id();
        let photo = commands
            .spawn((
                ImageNode::new(art.courses[index].clone()),
                area([0., 0., 100., 100.]),
            ))
            .id();
        commands.entity(card).add_child(photo);
        let shade = commands
            .spawn((
                area([0., 61., 100., 39.]),
                BackgroundColor(Color::srgba(0.01, 0.015, 0.02, 0.9)),
            ))
            .id();
        commands.entity(card).add_child(shade);
        label(
            commands,
            card,
            course.label(),
            26.,
            [4., 65., 74., 13.],
            paper,
        );
        label(
            commands,
            card,
            distance,
            16.,
            [76., 68., 23., 10.],
            layout::ACCENT,
        );
        label(
            commands,
            card,
            route,
            14.,
            [4., 83., 94., 10.],
            paper.with_alpha(0.8),
        );
        if selected == course {
            let tag = commands
                .spawn((area([4., 5., 30., 10.]), BackgroundColor(layout::ACCENT)))
                .id();
            commands.entity(card).add_child(tag);
            label(
                commands,
                tag,
                "SELECTED",
                14.,
                [7., 8., 90., 90.],
                Color::BLACK,
            );
        }
        commands.entity(canvas).add_child(card);
    }
    for index in SELECTABLE.len()..SLOT_COUNT {
        let mut node = slot_area(index);
        node.border = UiRect::all(px(1.));
        let slot = commands
            .spawn((
                ReservedCourseSlot,
                node,
                BackgroundColor(Color::srgba(0.045, 0.055, 0.062, 0.85)),
                BorderColor::all(Color::srgba(0.36, 0.42, 0.46, 0.34)),
            ))
            .id();
        commands.entity(canvas).add_child(slot);
    }
    let ride = entry(format!("RIDE {}", selected.label()), Action::Ride);
    layout::button(commands, canvas, 2, &ride, [5., 81., 65., 10.], false);
    layout::button(
        commands,
        canvas,
        3,
        &entry("BACK", Action::Back),
        [74., 81., 21., 10.],
        false,
    );
    label(
        commands,
        canvas,
        front
            .course_error
            .as_deref()
            .unwrap_or("ARROWS / D-PAD  Browse     ENTER / A  Select     ESC / B  Back"),
        16.,
        [5., 94., 90., 5.],
        if front.course_error.is_some() {
            layout::ACCENT
        } else {
            paper.with_alpha(0.65)
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_handoff_preserves_live_selection_preferences_progress_and_read_only_paths() {
        let mut front = Frontend::default();
        front.flow.portrait = 7;
        front.flow.mode = rr_core::frontend::Mode::Street;
        let mut prefs = crate::settings::Preferences::default();
        prefs.file.master_volume = 0.37;
        let tuning = crate::vehicle::Tuning {
            selected: 0,
            assist: 0.45,
            ..default()
        };
        let mut profile = Profile::default();
        profile.credits = 987;
        let json =
            serde_json::to_string(&Session::capture(&front, &prefs, &tuning, &profile)).unwrap();
        let mut restored = Frontend::default();
        let mut restored_prefs = crate::settings::Preferences {
            read_only: true,
            path: Some("preserved.json".into()),
            ..default()
        };
        let mut restored_tuning = crate::vehicle::Tuning::default();
        let mut restored_profile = Profile::default();
        serde_json::from_str::<Session>(&json)
            .unwrap()
            .apply(
                &mut restored,
                &mut restored_prefs,
                &mut restored_tuning,
                &mut restored_profile,
                &mut crate::controls::Bindings::default(),
            )
            .unwrap();
        assert_eq!(restored.flow.portrait, 7);
        assert_eq!(restored.flow.mode, rr_core::frontend::Mode::Street);
        assert_eq!(restored_prefs.file.master_volume, 0.37);
        assert!(restored_prefs.read_only);
        assert_eq!(restored_prefs.path, Some("preserved.json".into()));
        assert_eq!(restored_tuning.selected, 0);
        assert_eq!(restored_tuning.assist, 0.45);
        assert_eq!(restored_profile.credits, 987);
    }
    #[test]
    fn map_handoff_drops_previous_map_forcing_flags_but_preserves_player_options() {
        let args = [
            "--hills",
            "--ragdoll-baseline",
            "--manual-recovery",
            "--frontend-smoke",
            "--metro-survey",
        ]
        .map(OsString::from);
        assert_eq!(
            map_launch_arguments(args),
            [
                "--ragdoll-baseline",
                "--manual-recovery",
                "--frontend-smoke"
            ]
            .map(OsString::from)
        );
    }
    #[test]
    fn offline_command_selects_map_without_rejoining_a_previous_online_session() {
        for course in Course::ALL {
            let cmd = command(course).unwrap();
            let env: std::collections::BTreeMap<_, _> = cmd.get_envs().collect();
            assert_eq!(
                env[std::ffi::OsStr::new("ROADDASH_TRACK")],
                Some(std::ffi::OsStr::new(course.key()))
            );
            assert_eq!(
                env[std::ffi::OsStr::new("ROADDASH_OFFLINE_BOOT")],
                Some(std::ffi::OsStr::new(course.key()))
            );
            assert_eq!(env[std::ffi::OsStr::new("ROADDASH_ONLINE_BOOT")], None);
            assert_eq!(env[std::ffi::OsStr::new("ROADDASH_NAT_TOKEN")], None);
            assert!(!env.contains_key(std::ffi::OsStr::new("ROADDASH_SETTINGS")));
            assert!(!env.contains_key(std::ffi::OsStr::new("ROADDASH_SAVE_DIR")));
        }
    }
}
