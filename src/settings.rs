//! Versioned local preferences. Invalid files are preserved, never silently replaced.
use crate::controls::{BindingFile, Bindings, pad_button};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreferencesFile {
    pub schema_version: u32,
    pub comfort: rr_core::comfort::Comfort,
    pub keyboard: BindingFile,
    pub controller: BTreeMap<String, String>,
}
impl Default for PreferencesFile {
    fn default() -> Self {
        Self {
            schema_version: 1,
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
        self.comfort.validate()?;
        Bindings::parse(&serde_json::to_string(&self.keyboard).map_err(|e| e.to_string())?)?;
        let mut used = std::collections::HashSet::new();
        if self.controller.len() != self.keyboard.actions.len() {
            return Err("controller action count mismatch".into());
        }
        for action in self.keyboard.actions.keys() {
            let b = self
                .controller
                .get(action)
                .and_then(|s| pad_button(s))
                .ok_or_else(|| format!("missing/unknown controller action {action}"))?;
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
#[derive(Resource, Default)]
pub struct SettingsPanel {
    pub open: bool,
    pub selected: usize,
    pub rebinding: bool,
    pub controller: bool,
    resume: Option<crate::game::Phase>,
}
pub const ROWS: usize = 22;
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
pub fn setup(mut commands: Commands) {
    let root = commands
        .spawn((
            SettingsRoot,
            Node {
                position_type: PositionType::Absolute,
                left: percent(15),
                top: px(20),
                width: percent(70),
                height: percent(95),
                padding: UiRect::all(px(16)),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.015, 0.03, 0.045, 0.98)),
            GlobalZIndex(100),
        ))
        .id();
    let header=commands.spawn((Text::new("COMFORT & CONTROLS   /   F2 or Esc to save & close\nUp/down: select | Left/right: adjust | Enter: rebind | Tab: device"),TextFont{font_size:16.0,..default()},TextColor(Color::WHITE))).id();
    commands.entity(root).add_child(header);
    for row in 0..ROWS {
        let e = commands
            .spawn((
                Button,
                SettingsRow(row),
                Node {
                    min_height: px(25),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
            .id();
        let text = commands
            .spawn((
                SettingsRow(row),
                Text::new(""),
                TextFont {
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ))
            .id();
        commands.entity(e).add_child(text);
        commands.entity(root).add_child(e);
    }
    let footer = commands
        .spawn((
            SettingsText,
            Text::new(""),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(0.2, 0.9, 0.75)),
        ))
        .id();
    commands.entity(root).add_child(footer);
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn panel_input(
    keys: Res<ButtonInput<KeyCode>>,
    pads: Query<&Gamepad>,
    clicks: Query<(&Interaction, &SettingsRow), (With<Button>, Changed<Interaction>)>,
    mut panel: ResMut<SettingsPanel>,
    mut prefs: ResMut<Preferences>,
    mut bindings: ResMut<Bindings>,
    mut race: ResMut<crate::game::Race>,
    mut controls: ResMut<crate::controls::Controls>,
) {
    let pad = pads.iter().next();
    let edge = |button| pad.is_some_and(|p| p.just_pressed(button));
    let toggle = keys.just_pressed(KeyCode::F2)
        || edge(GamepadButton::Mode)
        || (!panel.rebinding && edge(GamepadButton::LeftTrigger));
    let close = panel.open
        && !panel.rebinding
        && (keys.just_pressed(KeyCode::Escape) || edge(GamepadButton::Start));
    if toggle || close {
        if panel.open {
            prefs.status = match prefs.save() {
                Ok(()) => "Settings saved".into(),
                Err(e) => {
                    warn!("Settings save: {e}");
                    e
                }
            };
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
        let action = ACTIONS[panel.selected - 10];
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
    if keys.just_pressed(KeyCode::Tab) || edge(GamepadButton::North) {
        panel.controller = !panel.controller;
    }
    if keys.just_pressed(KeyCode::ArrowDown) || edge(GamepadButton::DPadDown) {
        panel.selected = (panel.selected + 1) % ROWS;
    }
    if keys.just_pressed(KeyCode::ArrowUp) || edge(GamepadButton::DPadUp) {
        panel.selected = (panel.selected + ROWS - 1) % ROWS;
    }
    let mut delta = 0;
    if keys.just_pressed(KeyCode::ArrowLeft) || edge(GamepadButton::DPadLeft) {
        delta = -1;
    }
    if keys.just_pressed(KeyCode::ArrowRight) || edge(GamepadButton::DPadRight) {
        delta = 1;
    }
    let mut activate = keys.just_pressed(KeyCode::Enter) || edge(GamepadButton::South);
    for (i, row) in &clicks {
        if *i == Interaction::Pressed {
            panel.selected = row.0;
            activate = true;
        }
    }
    if panel.selected >= 10 {
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
fn pretty_binding(value: &str) -> String {
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
#[allow(clippy::type_complexity)]
pub fn display(
    panel: Res<SettingsPanel>,
    prefs: Res<Preferences>,
    mut root: Query<&mut Node, With<SettingsRoot>>,
    mut rows: Query<(&SettingsRow, &mut Text)>,
    mut footer: Query<&mut Text, (With<SettingsText>, Without<SettingsRow>)>,
) {
    for mut node in &mut root {
        node.display = if panel.open {
            Display::Flex
        } else {
            Display::None
        };
    }
    if !panel.open {
        return;
    }
    let c = &prefs.file.comfort;
    let labels = [
        format!("Field of view: {:.0} deg", c.fov_degrees),
        format!("Camera response: {:.0}", c.smoothing),
        format!("Shake: {:.0}%", c.shake * 100.0),
        format!("Anti-nausea: {}", if c.anti_nausea { "On" } else { "Off" }),
        format!("Speed FOV: {}", if c.speed_fov { "On" } else { "Off" }),
        format!("Speed lines: {}", if c.speed_lines { "On" } else { "Off" }),
        format!("Stick/trigger deadzone: {:.0}%", c.deadzone * 100.0),
        format!(
            "Rebind device: {}",
            if panel.controller {
                "Controller"
            } else {
                "Keyboard"
            }
        ),
        "Restore comfort defaults".into(),
        "Save settings".into(),
    ];
    for (row, mut text) in &mut rows {
        let label = if row.0 < 10 {
            labels[row.0].clone()
        } else {
            let action = ACTIONS[row.0 - 10];
            let value = if panel.controller {
                prefs.file.controller[action].clone()
            } else {
                prefs.file.keyboard.actions[action].join(" / ")
            };
            format!("{}: {}", action.replace('_', " "), pretty_binding(&value))
        };
        **text = format!(
            "{} {}",
            if panel.selected == row.0 {
                if panel.rebinding { "?" } else { ">" }
            } else {
                " "
            },
            label
        );
    }
    for mut text in &mut footer {
        **text = format!(
            "{}\nLeft stick steers | Right stick looks | F2 / left bumper opens settings",
            prefs.status
        );
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
        prefs.file.comfort.fov_degrees = 88.0;
        prefs.file.comfort.anti_nausea = true;
        prefs
            .file
            .controller
            .insert("nitro".into(), "RightTrigger".into());
        prefs.save().unwrap();
        let loaded = Preferences::load(path.clone());
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
