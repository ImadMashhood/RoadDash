//! Render-frame input capture -> buffered fixed-tick intentions.
use crate::game::{Phase, Race};
use bevy::{input::InputSystems, prelude::*};
use rr_core::{AttackSide, InputBuffer, VehicleIntent};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
#[derive(Resource, Default)]
pub struct Controls {
    pub buffer: InputBuffer,
    pub start: bool,
    pub pause: bool,
    pub restart: bool,
    pub camera: bool,
    pub mute: bool,
    pub look: Vec2,
    pub foot: Vec2,
    pub context: rr_core::comfort::InputContext,
    pub suppress_frame: bool,
}
#[derive(Resource)]
pub struct Bindings(BTreeMap<String, Vec<KeyCode>>);
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BindingFile {
    pub schema_version: u32,
    pub actions: BTreeMap<String, Vec<String>>,
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
pub fn key_code(name: &str) -> Option<KeyCode> {
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
pub fn pad_button(name: &str) -> Option<GamepadButton> {
    use GamepadButton::*;
    Some(match name {
        "South" => South,
        "East" => East,
        "North" => North,
        "West" => West,
        "LeftTrigger" => LeftTrigger,
        "LeftTrigger2" => LeftTrigger2,
        "RightTrigger" => RightTrigger,
        "RightTrigger2" => RightTrigger2,
        "Select" => Select,
        "Start" => Start,
        "Mode" => Mode,
        "LeftThumb" => LeftThumb,
        "RightThumb" => RightThumb,
        "DPadUp" => DPadUp,
        "DPadDown" => DPadDown,
        "DPadLeft" => DPadLeft,
        "DPadRight" => DPadRight,
        _ => return None,
    })
}
#[allow(clippy::too_many_arguments)]
pub fn capture(
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<Bindings>,
    race: Res<Race>,
    pads: Query<(Entity, &Gamepad)>,
    preferences: Res<crate::settings::Preferences>,
    panel: Res<crate::settings::SettingsPanel>,
    recoveries: Query<(&crate::game::Rider, &crate::recovery::Recovery)>,
    mut controls: ResMut<Controls>,
) {
    let suppressed = std::mem::take(&mut controls.suppress_frame) || panel.open;
    if suppressed {
        controls.buffer.held = default();
        controls.buffer.clear_edges();
        controls.look = Vec2::ZERO;
        controls.start = false;
        controls.pause = false;
        controls.restart = false;
        controls.camera = false;
        controls.mute = false;
        controls.context = rr_core::comfort::InputContext::Menu;
        controls.foot = Vec2::ZERO;
        return;
    }
    // Deterministic single-controller ownership; disconnect immediately falls back to keys.
    let pad = pads.iter().min_by_key(|(e, _)| e.to_bits()).map(|(_, p)| p);
    let deadzone = preferences.file.comfort.deadzone;
    let button = |action: &str| {
        preferences
            .file
            .controller
            .get(action)
            .and_then(|s| pad_button(s))
    };
    let pad_held = |action: &str| pad.zip(button(action)).is_some_and(|(p, b)| p.pressed(b));
    let edge = |action: &str| {
        bindings.edge(&keys, action)
            || pad
                .zip(button(action))
                .is_some_and(|(p, b)| p.just_pressed(b))
    };
    let analog = |action: &str| {
        if bindings.held(&keys, action) {
            1.0
        } else {
            pad.zip(button(action)).map_or(0.0, |(p, b)| {
                rr_core::comfort::axis(
                    p.get(b).unwrap_or(if p.pressed(b) { 1.0 } else { 0.0 }),
                    deadzone,
                )
                .max(0.0)
            })
        }
    };
    let throttle = analog("throttle");
    let brake = analog("brake");
    let keyboard_steer =
        f32::from(bindings.held(&keys, "right")) - f32::from(bindings.held(&keys, "left"));
    let digital = keyboard_steer + f32::from(pad_held("right")) - f32::from(pad_held("left"));
    let steer = if digital != 0.0 {
        digital.clamp(-1.0, 1.0)
    } else {
        pad.map_or(0.0, |p| rr_core::comfort::axis(p.left_stick().x, deadzone))
    };
    controls.look = pad.map_or(Vec2::ZERO, |p| {
        p.right_stick().map(|v| rr_core::comfort::axis(v, deadzone))
    });
    controls.context = if race.phase != Phase::Racing {
        rr_core::comfort::InputContext::Menu
    } else if recoveries
        .iter()
        .any(|(r, s)| r.id == 0 && s.stage != crate::recovery::RecoveryStage::Riding)
    {
        rr_core::comfort::InputContext::OnFoot
    } else {
        rr_core::comfort::InputContext::Riding
    };
    // Allocate an independent on-foot context; recovery remains assisted until Part 06.
    controls.foot = if controls.context == rr_core::comfort::InputContext::OnFoot {
        Vec2::new(steer, throttle - brake)
    } else {
        Vec2::ZERO
    };
    controls.buffer.held = if controls.context == rr_core::comfort::InputContext::OnFoot {
        default()
    } else {
        VehicleIntent {
            throttle: throttle > 0.0,
            brake: brake > 0.0,
            steer,
            nitro: bindings.held(&keys, "nitro") || pad_held("nitro"),
            throttle_amount: throttle,
            brake_amount: brake,
        }
    };
    if race.phase == Phase::Racing && controls.context == rr_core::comfort::InputContext::Riding {
        if edge("punch_left") {
            controls.buffer.push_attack(AttackSide::Left);
        }
        if edge("punch_right") {
            controls.buffer.push_attack(AttackSide::Right);
        }
    } else {
        controls.buffer.clear_edges();
    }
    controls.start |= edge("start");
    controls.pause |= edge("pause");
    controls.restart |= edge("restart");
    controls.camera |= edge("camera");
    controls.mute |= edge("mute");
}
pub struct RoadInputPlugin;
impl Plugin for RoadInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Controls>()
            .init_resource::<crate::settings::Preferences>()
            .init_resource::<crate::settings::SettingsPanel>();
        let json = serde_json::to_string(
            &app.world()
                .resource::<crate::settings::Preferences>()
                .file
                .keyboard,
        )
        .unwrap();
        app.insert_resource(Bindings::parse(&json).expect("validated preference bindings"))
            .add_systems(
                PreUpdate,
                (
                    crate::settings::panel_input,
                    crate::ui::buttons,
                    capture,
                    crate::game::input,
                )
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

#[cfg(test)]
mod device_tests {
    use super::*;
    fn fixture() -> (App, Entity) {
        let mut app = crate::game::tests::simulation();
        app.world_mut().resource_mut::<Race>().phase = Phase::Racing;
        let e = app.world_mut().spawn(Gamepad::default()).id();
        (app, e)
    }
    #[test]
    fn gamepad_has_analog_triggers_steering_freelook_and_disconnect_fallback() {
        let (mut app, pad) = fixture();
        {
            let mut p = app.world_mut().get_mut::<Gamepad>(pad).unwrap();
            p.analog_mut().set(GamepadButton::RightTrigger2, 0.575);
            p.analog_mut().set(GamepadAxis::LeftStickX, 0.575);
            p.analog_mut().set(GamepadAxis::RightStickX, -1.0);
        }
        app.update();
        let c = app.world().resource::<Controls>();
        assert!((c.buffer.held.throttle_strength() - 0.5).abs() < 0.001);
        assert!((c.buffer.held.steer - 0.5).abs() < 0.001);
        assert_eq!(c.look.x, -1.0);
        app.world_mut().despawn(pad);
        app.update();
        let c = app.world().resource::<Controls>();
        assert!(!c.buffer.held.throttle);
        assert_eq!(c.buffer.held.steer, 0.0);
        assert_eq!(c.look, Vec2::ZERO);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.update();
        assert_eq!(app.world().resource::<Controls>().buffer.held.steer, 1.0);
    }
    #[test]
    fn settings_pause_countdown_and_suppress_driving_until_closed() {
        let (mut app, _) = fixture();
        app.world_mut().resource_mut::<Race>().phase = Phase::Countdown;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::F2);
        app.update();
        assert_eq!(app.world().resource::<Race>().phase, Phase::Paused);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::F2);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();
        assert!(!app.world().resource::<Controls>().buffer.held.throttle);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::F2);
        app.update();
        assert_eq!(app.world().resource::<Race>().phase, Phase::Countdown);
        assert!(
            !app.world()
                .resource::<crate::settings::SettingsPanel>()
                .open
        );
    }
    #[test]
    fn on_foot_context_does_not_deliver_mounted_attacks() {
        let (mut app, pad) = fixture();
        let owner = app
            .world_mut()
            .query_filtered::<Entity, With<crate::game::Rider>>()
            .single(app.world())
            .unwrap();
        app.world_mut()
            .get_mut::<crate::recovery::Recovery>(owner)
            .unwrap()
            .stage = crate::recovery::RecoveryStage::Running;
        app.world_mut()
            .get_mut::<Gamepad>(pad)
            .unwrap()
            .digital_mut()
            .press(GamepadButton::West);
        app.update();
        let c = app.world().resource::<Controls>();
        assert_eq!(c.context, rr_core::comfort::InputContext::OnFoot);
        assert!(!c.buffer.held.throttle);
    }
}

#[cfg(test)]
mod rebind_tests {
    use super::*;
    #[test]
    fn panel_rebinding_changes_live_keyboard_and_pad_without_leaking_input() {
        let mut app = crate::game::tests::simulation();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::F2);
        app.update();
        app.world_mut()
            .resource_mut::<crate::settings::SettingsPanel>()
            .selected = 14;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::Enter);
        let prefs = app.world().resource::<crate::settings::Preferences>();
        assert_eq!(prefs.file.keyboard.actions["nitro"], vec!["KeyE"]);
        app.world_mut()
            .resource_mut::<crate::settings::SettingsPanel>()
            .controller = true;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Enter);
        app.update();
        let pad = app.world_mut().spawn(Gamepad::default()).id();
        app.world_mut()
            .get_mut::<Gamepad>(pad)
            .unwrap()
            .digital_mut()
            .press(GamepadButton::RightTrigger);
        app.update();
        assert_eq!(
            app.world()
                .resource::<crate::settings::Preferences>()
                .file
                .controller["nitro"],
            "RightTrigger"
        );
        assert!(!app.world().resource::<Controls>().buffer.held.nitro);
    }
}
