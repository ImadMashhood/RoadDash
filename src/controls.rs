//! Render-frame input capture -> buffered fixed-tick intentions.
use crate::game::{Phase, Race};
use bevy::{input::InputSystems, prelude::*};
use rr_core::{AttackSide, InputBuffer, VehicleIntent};
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
#[derive(Resource, Default)]
pub struct Controls {
    pub buffer: InputBuffer,
    pub start: bool,
    pub pause: bool,
    pub restart: bool,
    pub camera: bool,
    pub mute: bool,
}
#[derive(Resource)]
pub struct Bindings(BTreeMap<String, Vec<KeyCode>>);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BindingFile {
    schema_version: u32,
    actions: BTreeMap<String, Vec<String>>,
}
const ACTIONS: [&str; 11] = [
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
];
impl Bindings {
    pub fn parse(json: &str) -> Result<Self, String> {
        let file: BindingFile = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if file.schema_version != 1 {
            return Err("unsupported input schema".into());
        }
        let mut bindings = BTreeMap::new();
        let mut used = HashSet::new();
        for action in ACTIONS.into_iter().chain(["mute"]) {
            let keys = file
                .actions
                .get(action)
                .ok_or_else(|| format!("missing action {action}"))?;
            if keys.is_empty() {
                return Err(format!("empty binding for {action}"));
            }
            let mut codes = Vec::new();
            for key in keys {
                let code = key_code(key).ok_or_else(|| format!("unknown key {key}"))?;
                if !used.insert(code) {
                    return Err(format!("duplicate key binding {key}"));
                }
                codes.push(code);
            }
            bindings.insert(action.into(), codes);
        }
        if file.actions.len() != bindings.len() {
            return Err("unknown input action".into());
        }
        Ok(Self(bindings))
    }
    fn held(&self, keys: &ButtonInput<KeyCode>, action: &str) -> bool {
        self.0[action].iter().any(|k| keys.pressed(*k))
    }
    fn edge(&self, keys: &ButtonInput<KeyCode>, action: &str) -> bool {
        self.0[action].iter().any(|k| keys.just_pressed(*k))
    }
}
impl Default for Bindings {
    fn default() -> Self {
        let default =
            || Self::parse(include_str!("../content/input.json")).expect("default input bindings");
        if let Ok(path) = std::env::var("ROADDASH_BINDINGS") {
            match std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| Self::parse(&s))
            {
                Ok(bindings) => return bindings,
                Err(error) => warn!("Input configuration {path}: {error}; using defaults"),
            }
        }
        default()
    }
}
fn key_code(name: &str) -> Option<KeyCode> {
    use KeyCode::*;
    Some(match name {
        "KeyA" => KeyA,
        "KeyB" => KeyB,
        "KeyC" => KeyC,
        "KeyD" => KeyD,
        "KeyE" => KeyE,
        "KeyF" => KeyF,
        "KeyG" => KeyG,
        "KeyH" => KeyH,
        "KeyI" => KeyI,
        "KeyJ" => KeyJ,
        "KeyK" => KeyK,
        "KeyL" => KeyL,
        "KeyM" => KeyM,
        "KeyN" => KeyN,
        "KeyO" => KeyO,
        "KeyP" => KeyP,
        "KeyQ" => KeyQ,
        "KeyR" => KeyR,
        "KeyS" => KeyS,
        "KeyT" => KeyT,
        "KeyU" => KeyU,
        "KeyV" => KeyV,
        "KeyW" => KeyW,
        "KeyX" => KeyX,
        "KeyY" => KeyY,
        "KeyZ" => KeyZ,
        "ArrowUp" => ArrowUp,
        "ArrowDown" => ArrowDown,
        "ArrowLeft" => ArrowLeft,
        "ArrowRight" => ArrowRight,
        "ShiftLeft" => ShiftLeft,
        "ShiftRight" => ShiftRight,
        "Enter" => Enter,
        "Escape" => Escape,
        "Space" => Space,
        "ControlLeft" => ControlLeft,
        "ControlRight" => ControlRight,
        _ => return None,
    })
}
pub fn capture(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<Bindings>,
    race: Res<Race>,
    mut controls: ResMut<Controls>,
) {
    controls.buffer.held = VehicleIntent {
        throttle: bindings.held(&keys, "throttle"),
        brake: bindings.held(&keys, "brake"),
        steer: f32::from(bindings.held(&keys, "right")) - f32::from(bindings.held(&keys, "left")),
        nitro: bindings.held(&keys, "nitro"),
    };
    if race.phase == Phase::Racing {
        if bindings.edge(&keys, "punch_left") {
            controls.buffer.push_attack(AttackSide::Left);
        }
        if bindings.edge(&keys, "punch_right") {
            controls.buffer.push_attack(AttackSide::Right);
        }
    } else {
        controls.buffer.clear_edges();
    }
    controls.start |= bindings.edge(&keys, "start");
    controls.pause |= bindings.edge(&keys, "pause");
    controls.restart |= bindings.edge(&keys, "restart");
    controls.camera |= bindings.edge(&keys, "camera");
    controls.mute |= bindings.edge(&keys, "mute");
}
pub struct RoadInputPlugin;
impl Plugin for RoadInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Controls>()
            .init_resource::<Bindings>()
            .add_systems(
                PreUpdate,
                (crate::ui::buttons, capture, crate::game::input)
                    .chain()
                    .after(InputSystems)
                    .after(bevy::ui::UiSystems::Focus),
            );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bindings_are_validated_and_remappable() {
        let original = include_str!("../content/input.json");
        Bindings::parse(original).unwrap();
        Bindings::parse(&original.replace("KeyW", "KeyE")).unwrap();
        assert!(Bindings::parse(&original.replace("KeyW", "KeyS")).is_err());
        assert!(Bindings::parse(&original.replace("KeyW", "bad-key")).is_err());
    }
}
