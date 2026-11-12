//! Versioned local preferences. Invalid files are preserved, never silently replaced.
mod view;
pub use view::{display, setup};
fn full_volume() -> f32 {
    1.
}
fn yes() -> bool {
    true
}
use crate::controls::{BindingFile, Bindings, pad_button};
use bevy::{prelude::*, ui::Pressed, ui_widgets::Button};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreferencesFile {
    pub schema_version: u32,
    #[serde(default = "full_volume")]
    pub master_volume: f32,
    #[serde(default)]
    pub audio_muted: bool,
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default = "yes")]
    pub vsync: bool,
    pub comfort: rr_core::comfort::Comfort,
    pub keyboard: BindingFile,
    pub controller: BTreeMap<String, String>,
}
impl Default for PreferencesFile {
    fn default() -> Self {
        Self {
            schema_version: 1,
            master_volume: 1.,
            audio_muted: false,
            fullscreen: false,
            vsync: true,
            comfort: default(),
            keyboard: serde_json::from_str(include_str!("../content/input.json")).unwrap(),
            controller: [
                ("throttle", "RightTrigger2"),
                ("brake", "LeftTrigger2"),
                ("left", "DPadLeft"),
                ("right", "DPadRight"),
                ("nitro", "North"),
                ("punch_left", "West"),
                ("punch_right", "East"),
                ("start", "South"),
                ("pause", "Start"),
                ("restart", "Select"),
                ("camera", "RightThumb"),
                ("mute", "LeftThumb"),
            ]
            .into_iter()
            .map(|(a, b)| (a.into(), b.into()))
            .collect(),
        }
    }
}
impl PreferencesFile {
    pub fn parse(json: &str) -> Result<Self, String> {
        let p: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        p.validate()?;
        Ok(p)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err("unsupported preferences schema".into());
        }
        if !self.master_volume.is_finite() || !(0. ..=1.).contains(&self.master_volume) {
            return Err("invalid master volume".into());
        }
        self.comfort.validate()?;
        Bindings::parse(&serde_json::to_string(&self.keyboard).map_err(|e| e.to_string())?)?;
        let mut used = std::collections::HashSet::new();
        for action in self
            .keyboard
            .actions
            .keys()
            .filter(|a| !matches!(a.as_str(), "pickup" | "drop"))
        {
            if !self.controller.contains_key(action) {
                return Err(format!("missing controller action {action}"));
            }
        }
        for (action, button) in &self.controller {
            if !self.keyboard.actions.contains_key(action)
                && !matches!(action.as_str(), "pickup" | "drop")
            {
                return Err("unknown controller action".into());
            }
            let b = pad_button(button)
                .ok_or_else(|| format!("unknown controller button for {action}"))?;
            if matches!(b, GamepadButton::Mode | GamepadButton::LeftTrigger) {
                return Err("Guide and left bumper are reserved for settings".into());
            }
            if !used.insert(b) {
                return Err("duplicate controller button".into());
            }
        }
        Ok(())
    }
}
#[derive(Resource, Default)]
pub struct Preferences {
    pub file: PreferencesFile,
    pub path: Option<PathBuf>,
    pub read_only: bool,
    pub status: String,
}
impl Preferences {
    pub fn path() -> PathBuf {
        if let Some(path) = std::env::var_os("ROADDASH_SETTINGS") {
            return path.into();
        }
        let root = std::env::var_os(if cfg!(windows) {
            "APPDATA"
        } else {
            "XDG_CONFIG_HOME"
        })
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(std::env::temp_dir)
                .join(".config")
        });
        root.join("roaddash/settings.json")
    }
    pub fn load(path: PathBuf) -> Self {
        let mut p = Self {
            path: Some(path.clone()),
            ..default()
        };
        match std::fs::read_to_string(&path) {
            Ok(s) => match PreferencesFile::parse(&s) {
                Ok(file) => p.file = file,
                Err(e) => {
                    p.read_only = true;
                    p.status = format!("Settings preserved (read only): {e}");
                    warn!("{}", p.status);
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                p.read_only = true;
                p.status = format!("Settings cannot be read: {e}");
                warn!("{}", p.status);
            }
        }
        if let Ok(path) = std::env::var("ROADDASH_BINDINGS") {
            match std::fs::read_to_string(path)
                .map_err(|e| e.to_string())
                .and_then(|s| {
                    Bindings::parse(&s)?;
                    serde_json::from_str::<BindingFile>(&s).map_err(|e| e.to_string())
                }) {
                Ok(keys) => p.file.keyboard = keys,
                Err(e) => warn!("Input override rejected: {e}"),
            }
        }
        p
    }
    pub fn save(&mut self) -> Result<(), String> {
        self.file.validate()?;
        if self.read_only {
            return Err(
                "Existing settings preserved. Move/restore the invalid file before saving.".into(),
            );
        }
        let Some(path) = self.path.as_ref() else {
            return Ok(());
        };
        write_file(path, &self.file)
    }
}
fn write_file(path: &Path, file: &PreferencesFile) -> Result<(), String> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_vec_pretty(file).map_err(|e| e.to_string())?;
    let temp = path.with_extension("json.tmp");
    std::fs::write(&temp, text).map_err(|e| e.to_string())?;
    // Windows cannot rename over an existing destination; retain a recoverable backup.
    let backup = path.with_extension("json.bak");
    if path.exists() {
        std::fs::copy(path, &backup).map_err(|e| e.to_string())?;
        #[cfg(windows)]
        std::fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    if let Err(e) = std::fs::rename(&temp, path) {
        if !path.exists() && backup.exists() {
            let _ = std::fs::copy(&backup, path);
        }
        return Err(e.to_string());
    }
    Ok(())
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Section {
    #[default]
    Camera,
    Controls,
    Audio,
    Display,
}
impl Section {
    pub const ALL: [Self; 4] = [Self::Camera, Self::Controls, Self::Audio, Self::Display];
    pub fn name(self) -> &'static str {
        match self {
            Self::Camera => "Camera",
            Self::Controls => "Controls",
            Self::Audio => "Audio",
            Self::Display => "Display",
        }
    }
    pub fn rows(self) -> &'static [usize] {
        match self {
            Self::Camera => &[0, 1, 2, 3, 4, 5, 8],
            Self::Controls => &[6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 26, 27],
            Self::Audio => &[22, 25],
            Self::Display => &[23, 24],
        }
    }
}
#[derive(Component, Clone, Copy)]
pub enum SettingsAction {
    Tab(Section),
    Adjust(usize, i32),
    Close,
    Dismiss,
}
fn binding_action(row: usize) -> Option<&'static str> {
    match row {
        10..=21 => Some(ACTIONS[row - 10]),
        26 => Some("pickup"),
        27 => Some("drop"),
        _ => None,
    }
}
#[derive(Resource, Default)]
pub struct SettingsPanel {
    pub open: bool,
    pub section: Section,
    pub selected: usize,
    pub rebinding: bool,
    pub controller: bool,
    resume: Option<crate::game::Phase>,
}
pub const ROWS: usize = 28;
pub const ACTIONS: [&str; 12] = [
    "throttle",
    "brake",
    "left",
    "right",
    "nitro",
    "punch_left",
    "punch_right",
    "start",
    "pause",
    "restart",
    "camera",
    "mute",
];
#[derive(Component)]
pub struct SettingsRoot;
#[derive(Component)]
pub struct SettingsText;
#[derive(Component)]
pub struct SettingsRow(pub usize);
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn panel_input(
    keys: Res<ButtonInput<KeyCode>>,
    pads: Query<(Entity, &Gamepad)>,
    clicks: Query<&SettingsRow, (With<Button>, Added<Pressed>)>,
    mut panel: ResMut<SettingsPanel>,
    mut prefs: ResMut<Preferences>,
    mut bindings: ResMut<Bindings>,
    mut race: ResMut<crate::game::Race>,
    mut controls: ResMut<crate::controls::Controls>,
    actions: Query<&SettingsAction, (With<Button>, Added<Pressed>)>,
    mut muted: Option<ResMut<crate::sound::Muted>>,
) {
    let pad = pads.iter().min_by_key(|(e, _)| e.to_bits()).map(|(_, p)| p);
    let edge = |button| pad.is_some_and(|p| p.just_pressed(button));
    let toggle = keys.just_pressed(KeyCode::F2)
        || edge(GamepadButton::Mode)
        || (!panel.rebinding && edge(GamepadButton::LeftTrigger));
    let clicked = if panel.open {
        actions.iter().copied().next()
    } else {
        None
    };
    let dismiss = matches!(clicked, Some(SettingsAction::Dismiss))
        || (!panel.rebinding
            && (keys.just_pressed(KeyCode::Backspace) || edge(GamepadButton::Select)));
    let close = panel.open
        && !panel.rebinding
        && (keys.just_pressed(KeyCode::Escape)
            || edge(GamepadButton::Start)
            || matches!(clicked, Some(SettingsAction::Close))
            || dismiss);
    if toggle || close {
        if panel.open {
            if !dismiss {
                match prefs.save() {
                    Ok(()) => prefs.status = "Settings saved".into(),
                    Err(e) => {
                        warn!("Settings save: {e}");
                        prefs.status = format!("Could not save: {e}");
                        controls.suppress_frame = true;
                        return;
                    }
                }
            } else {
                prefs.status = "Session changes kept; file unchanged".into();
            }
            if let Some(previous) = panel.resume.take() {
                race.phase = previous;
            }
            panel.open = false;
            panel.rebinding = false;
        } else {
            panel.resume = if matches!(
                race.phase,
                crate::game::Phase::Racing | crate::game::Phase::Countdown
            ) {
                Some(race.phase)
            } else {
                None
            };
            if panel.resume.is_some() {
                race.phase = crate::game::Phase::Paused;
            }
            panel.open = true;
        }
        controls.suppress_frame = true;
        return;
    }
    if !panel.open {
        return;
    }
    controls.suppress_frame = true;
    if panel.rebinding {
        if keys.just_pressed(KeyCode::Escape) || edge(GamepadButton::Start) {
            panel.rebinding = false;
            return;
        }
        let Some(action) = binding_action(panel.selected) else {
            panel.rebinding = false;
            return;
        };
        let mut candidate = prefs.file.clone();
        let changed = if panel.controller {
            pad.and_then(|p| p.get_just_pressed().next().copied())
                .map(|b| {
                    candidate.controller.insert(action.into(), format!("{b:?}"));
                })
                .is_some()
        } else {
            keys.get_just_pressed()
                .find(|k| crate::controls::key_code(&format!("{k:?}")).is_some())
                .map(|k| {
                    candidate
                        .keyboard
                        .actions
                        .insert(action.into(), vec![format!("{k:?}")]);
                })
                .is_some()
        };
        if changed {
            match candidate.validate() {
                Ok(()) => {
                    *bindings =
                        Bindings::parse(&serde_json::to_string(&candidate.keyboard).unwrap())
                            .unwrap();
                    prefs.file = candidate;
                    prefs.status = "Binding updated; close to save".into();
                    panel.rebinding = false;
                }
                Err(e) => prefs.status = format!("Binding rejected: {e}"),
            }
        }
        return;
    }
    let next_tab = keys.just_pressed(KeyCode::PageDown) || edge(GamepadButton::RightTrigger);
    let previous_tab = keys.just_pressed(KeyCode::PageUp);
    let tab = match clicked {
        Some(SettingsAction::Tab(section)) => Some(section),
        _ if next_tab || previous_tab => {
            let i = Section::ALL
                .iter()
                .position(|s| *s == panel.section)
                .unwrap();
            Some(Section::ALL[(i + if previous_tab { 3 } else { 1 }) % 4])
        }
        _ => None,
    };
    if let Some(section) = tab {
        panel.section = section;
        panel.selected = section.rows()[0];
        return;
    }
    if keys.just_pressed(KeyCode::Tab) || edge(GamepadButton::North) {
        panel.controller = !panel.controller;
    }
    if keys.just_pressed(KeyCode::ArrowDown) || edge(GamepadButton::DPadDown) {
        let rows = panel.section.rows();
        let i = rows.iter().position(|r| *r == panel.selected).unwrap_or(0);
        panel.selected = rows[(i + 1) % rows.len()];
    }
    if keys.just_pressed(KeyCode::ArrowUp) || edge(GamepadButton::DPadUp) {
        let rows = panel.section.rows();
        let i = rows.iter().position(|r| *r == panel.selected).unwrap_or(0);
        panel.selected = rows[(i + rows.len() - 1) % rows.len()];
    }
    let mut delta = 0;
    if keys.just_pressed(KeyCode::ArrowLeft) || edge(GamepadButton::DPadLeft) {
        delta = -1;
    }
    if keys.just_pressed(KeyCode::ArrowRight) || edge(GamepadButton::DPadRight) {
        delta = 1;
    }
    let mut activate = keys.just_pressed(KeyCode::Enter) || edge(GamepadButton::South);
    for row in &clicks {
        if clicked.is_none() {
            panel.selected = row.0;
            activate = true;
        }
    }
    if let Some(SettingsAction::Adjust(row, d)) = clicked {
        panel.selected = row;
        delta = d;
    }
    if binding_action(panel.selected).is_some() {
        if activate {
            panel.rebinding = true;
            prefs.status =
                "Press a new key/button; Esc/Start cancels. Duplicates are rejected.".into();
        }
    } else if delta != 0 || activate {
        let d = if delta == 0 { 1.0 } else { delta as f32 };
        let c = &mut prefs.file.comfort;
        match panel.selected {
            0 => c.fov_degrees = (c.fov_degrees + d).clamp(60.0, 95.0),
            1 => c.smoothing = (c.smoothing + d).clamp(2.0, 20.0),
            2 => c.shake = (c.shake + d * 0.1).clamp(0.0, 1.0),
            3 => c.anti_nausea = !c.anti_nausea,
            4 => c.speed_fov = !c.speed_fov,
            5 => c.speed_lines = !c.speed_lines,
            6 => c.deadzone = (c.deadzone + d * 0.025).clamp(0.05, 0.4),
            7 => panel.controller = !panel.controller,
            8 => {
                prefs.file.comfort = default();
            }
            22 => prefs.file.master_volume = (prefs.file.master_volume + d * 0.05).clamp(0., 1.),
            23 => prefs.file.vsync = !prefs.file.vsync,
            24 => prefs.file.fullscreen = !prefs.file.fullscreen,
            25 => {
                prefs.file.audio_muted = !prefs.file.audio_muted;
                if let Some(ref mut m) = muted {
                    m.0 = prefs.file.audio_muted;
                }
            }
            9 => {
                prefs.status = match prefs.save() {
                    Ok(()) => "Settings saved".into(),
                    Err(e) => e,
                };
            }
            _ => {}
        }
    }
}
pub(super) fn pretty_binding(value: &str) -> String {
    value
        .split(" / ")
        .map(|v| match v {
            "RightTrigger2" => "RT",
            "LeftTrigger2" => "LT",
            "RightTrigger" => "RB",
            "LeftTrigger" => "LB",
            "South" => "A / Cross",
            "East" => "B / Circle",
            "North" => "Y / Triangle",
            "West" => "X / Square",
            "RightThumb" => "Right stick click",
            "LeftThumb" => "Left stick click",
            "ShiftLeft" => "Left Shift",
            "ShiftRight" => "Right Shift",
            "DPadLeft" => "D-pad left",
            "DPadRight" => "D-pad right",
            _ => v
                .strip_prefix("Key")
                .or_else(|| v.strip_prefix("Arrow"))
                .unwrap_or(v),
        })
        .collect::<Vec<_>>()
        .join(" / ")
}
/// Apply only supported window options; preserve the explicit legacy survey mode.
pub fn apply_preferences(
    mut prefs: ResMut<Preferences>,
    mut windows: Query<&mut Window>,
    mut muted: Option<ResMut<crate::sound::Muted>>,
    mut initialized: Local<bool>,
    mut windowed_size: Local<Option<Vec2>>,
) {
    if let Some(ref mut m) = muted {
        if !*initialized {
            m.0 = prefs.file.audio_muted;
        } else if prefs.file.audio_muted != m.0 {
            prefs.file.audio_muted = m.0;
        }
    }
    *initialized = true;
    if !prefs.is_changed() || std::env::args().any(|a| a == "--world-survey") {
        return;
    }
    for mut w in &mut windows {
        let mode = if prefs.file.fullscreen {
            bevy::window::WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Current)
        } else {
            bevy::window::WindowMode::Windowed
        };
        let present = if prefs.file.vsync {
            bevy::window::PresentMode::AutoVsync
        } else {
            bevy::window::PresentMode::AutoNoVsync
        };
        if w.mode != mode {
            if prefs.file.fullscreen {
                *windowed_size = Some(Vec2::new(w.width(), w.height()));
            } else if let Some(size) = windowed_size.take() {
                w.resolution.set(size.x, size.y);
            }
            w.mode = mode;
        }
        if w.present_mode != present {
            w.present_mode = present;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn settings_roundtrip_and_invalid_files_are_preserved() {
        let root =
            std::env::temp_dir().join(format!("roaddash-settings-test-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let path = root.join("settings.json");
        let mut prefs = Preferences::load(path.clone());
        prefs.file.master_volume = 0.4;
        prefs.file.audio_muted = true;
        prefs.file.fullscreen = true;
        prefs.file.vsync = false;
        prefs.file.comfort.fov_degrees = 88.0;
        prefs.file.comfort.anti_nausea = true;
        prefs
            .file
            .controller
            .insert("nitro".into(), "RightTrigger".into());
        prefs.save().unwrap();
        let loaded = Preferences::load(path.clone());
        assert_eq!(
            (
                loaded.file.master_volume,
                loaded.file.audio_muted,
                loaded.file.fullscreen,
                loaded.file.vsync
            ),
            (0.4, true, true, false)
        );
        assert_eq!(loaded.file.comfort, prefs.file.comfort);
        assert_eq!(loaded.file.controller["nitro"], "RightTrigger");
        let original = "{\"schema_version\":99}";
        std::fs::write(&path, original).unwrap();
        let mut invalid = Preferences::load(path.clone());
        assert!(invalid.read_only);
        assert!(invalid.save().is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), original);
        std::fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn rebind_validation_rejects_conflicts_and_unknown_fields() {
        let mut p = PreferencesFile::default();
        p.validate().unwrap();
        p.controller.insert("nitro".into(), "South".into());
        assert!(p.validate().is_err());
        assert!(PreferencesFile::parse("{}").is_err());
    }
}

#[cfg(test)]
mod menu_tests {
    use super::*;
    fn press(app: &mut App, key: KeyCode) {
        {
            let mut k = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            k.release_all();
            k.press(key);
        }
        app.update();
    }
    fn click(app: &mut App, action: SettingsAction) {
        let e = app.world_mut().spawn((Button, Pressed, action)).id();
        app.update();
        app.world_mut().despawn(e);
        app.update();
    }
    #[test]
    fn old_schema_defaults_new_fields_and_rejects_invalid_volume() {
        let mut v = serde_json::to_value(PreferencesFile::default()).unwrap();
        for k in ["master_volume", "audio_muted", "fullscreen", "vsync"] {
            v.as_object_mut().unwrap().remove(k);
        }
        let p = PreferencesFile::parse(&v.to_string()).unwrap();
        assert_eq!(p.master_volume, 1.);
        assert!(!p.audio_muted && p.vsync && !p.fullscreen);
        let mut p = p;
        p.master_volume = f32::NAN;
        assert!(p.validate().is_err());
        p.master_volume = 1.1;
        assert!(p.validate().is_err());
    }
    #[test]
    fn categories_audio_display_and_close_use_real_panel_input() {
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<crate::game::Race>().phase = crate::game::Phase::Racing;
        let w = app.world_mut().spawn(Window::default()).id();
        press(&mut app, KeyCode::F2);
        assert_eq!(
            app.world().resource::<crate::game::Race>().phase,
            crate::game::Phase::Paused
        );
        click(&mut app, SettingsAction::Tab(Section::Audio));
        assert_eq!(app.world().resource::<SettingsPanel>().selected, 22);
        click(&mut app, SettingsAction::Adjust(22, -1));
        assert!((app.world().resource::<Preferences>().file.master_volume - 0.95).abs() < 0.001);
        click(&mut app, SettingsAction::Adjust(25, 1));
        assert!(app.world().resource::<crate::sound::Muted>().0);
        click(&mut app, SettingsAction::Tab(Section::Display));
        click(&mut app, SettingsAction::Adjust(23, 1));
        click(&mut app, SettingsAction::Adjust(24, 1));
        let window = app.world().get::<Window>(w).unwrap();
        assert_eq!(window.present_mode, bevy::window::PresentMode::AutoNoVsync);
        assert!(matches!(
            window.mode,
            bevy::window::WindowMode::BorderlessFullscreen(_)
        ));
        app.world_mut()
            .get_mut::<Window>(w)
            .unwrap()
            .resolution
            .set(1920., 1080.);
        click(&mut app, SettingsAction::Adjust(24, 1));
        let window = app.world().get::<Window>(w).unwrap();
        assert_eq!(window.mode, bevy::window::WindowMode::Windowed);
        assert_eq!((window.width(), window.height()), (1280., 720.));
        click(&mut app, SettingsAction::Close);
        assert!(!app.world().resource::<SettingsPanel>().open);
        assert_eq!(
            app.world().resource::<crate::game::Race>().phase,
            crate::game::Phase::Racing
        );
    }
    #[test]
    fn failed_save_stays_visible_and_dismiss_preserves_bad_file() {
        let dir =
            std::env::temp_dir().join(format!("roaddash-menu-preserve-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("bad.json");
        let original = "{\"schema_version\":99}";
        std::fs::write(&path, original).unwrap();
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<crate::game::Race>().phase = crate::game::Phase::Countdown;
        app.insert_resource(Preferences::load(path.clone()));
        press(&mut app, KeyCode::F2);
        click(&mut app, SettingsAction::Close);
        assert!(app.world().resource::<SettingsPanel>().open);
        assert!(
            app.world()
                .resource::<Preferences>()
                .status
                .contains("Could not save")
        );
        assert_eq!(
            app.world().resource::<crate::game::Race>().phase,
            crate::game::Phase::Paused
        );
        press(&mut app, KeyCode::Backspace);
        assert!(!app.world().resource::<SettingsPanel>().open);
        assert_eq!(std::fs::read_to_string(path).unwrap(), original);
        std::fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn pickup_binding_and_rebind_cancel_are_supported() {
        let mut app = crate::game::tests::simulation();
        press(&mut app, KeyCode::F2);
        click(&mut app, SettingsAction::Tab(Section::Controls));
        app.world_mut().resource_mut::<SettingsPanel>().selected = 26;
        press(&mut app, KeyCode::Enter);
        assert!(app.world().resource::<SettingsPanel>().rebinding);
        press(&mut app, KeyCode::KeyU);
        assert_eq!(
            app.world().resource::<Preferences>().file.keyboard.actions["pickup"],
            vec!["KeyU"]
        );
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Escape);
        assert!(app.world().resource::<SettingsPanel>().open);
        assert!(!app.world().resource::<SettingsPanel>().rebinding);
        assert_eq!(
            app.world().resource::<Preferences>().file.keyboard.actions["pickup"],
            vec!["KeyU"]
        );
    }
}
