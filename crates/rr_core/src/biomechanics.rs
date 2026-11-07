//! Backend-independent crash activation. No transforms or forces are authored here.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum State {
    Riding,
    Bracing,
    Ejection,
    Tumbling,
    Resting,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Activation {
    pub riding: f32,
    pub bracing: f32,
    pub ejection: f32,
    pub tumbling: f32,
    pub impact_seconds: f32,
    pub ejection_seconds: f32,
    pub tumbling_fade_seconds: f32,
    pub settle_seconds: f32,
    pub resting_linear: f32,
    pub resting_angular: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct CrashControl {
    pub state: State,
    pub age: f32,
    pub activation: f32,
    pub severity: f32,
    quiet: f32,
}
impl CrashControl {
    pub fn impact(severity: f32, config: &Activation) -> Self {
        Self {
            state: State::Bracing,
            age: 0.0,
            activation: config.riding,
            severity: severity.clamp(0.0, 1.0),
            quiet: 0.0,
        }
    }
    pub fn step(
        &mut self,
        dt: f32,
        speed: f32,
        angular: f32,
        supported: bool,
        config: &Activation,
    ) {
        let dt = dt.clamp(0.0, 0.05);
        self.age += dt;
        let brace = config.impact_seconds * (1.0 - 0.7 * self.severity);
        let smooth = |t: f32| {
            let t = t.clamp(0.0, 1.0);
            t * t * (3.0 - 2.0 * t)
        };
        if self.age < brace {
            self.state = State::Bracing;
            let phase = self.age / brace;
            self.activation = if phase < 0.2 {
                config.riding + (config.bracing - config.riding) * smooth(phase / 0.2)
            } else {
                config.bracing + (config.ejection - config.bracing) * smooth((phase - 0.2) / 0.8)
            };
        } else if self.age < brace + config.ejection_seconds {
            self.state = State::Ejection;
            self.activation = config.ejection
                + (config.tumbling - config.ejection)
                    * smooth((self.age - brace) / config.ejection_seconds);
        } else {
            let quiet =
                supported && speed < config.resting_linear && angular < config.resting_angular;
            self.quiet = if quiet { self.quiet + dt } else { 0.0 };
            self.state = if self.quiet >= config.settle_seconds {
                State::Resting
            } else {
                State::Tumbling
            };
            // No active motion while settling; passive springs/damping are a separate layer.
            self.activation = if self.state == State::Resting {
                0.0
            } else {
                config.tumbling
                    * (1.0
                        - smooth(
                            (self.age - brace - config.ejection_seconds)
                                / config.tumbling_fade_seconds,
                        ))
            };
        }
    }
    pub fn secondary_impact(&mut self, severity: f32) {
        if severity > 0.08 {
            self.severity = self.severity.max(severity.clamp(0.0, 1.0));
            self.quiet = 0.0;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> Activation {
        Activation {
            riding: 1.0,
            bracing: 0.7,
            ejection: 0.15,
            tumbling: 0.025,
            impact_seconds: 0.24,
            ejection_seconds: 0.3,
            tumbling_fade_seconds: 2.0,
            settle_seconds: 0.6,
            resting_linear: 0.25,
            resting_angular: 0.5,
        }
    }
    #[test]
    fn severe_impacts_decay_faster_and_limits_are_not_part_of_activation() {
        let c = config();
        let mut mild = CrashControl::impact(0.0, &c);
        let mut severe = CrashControl::impact(1.0, &c);
        for _ in 0..6 {
            mild.step(1.0 / 60.0, 5.0, 3.0, false, &c);
            severe.step(1.0 / 60.0, 5.0, 3.0, false, &c);
        }
        assert!(severe.activation < mild.activation);
        assert_eq!(severe.state, State::Ejection);
    }
    #[test]
    fn airborne_body_never_rests_and_external_impacts_wake_state() {
        let c = config();
        let mut s = CrashControl::impact(0.5, &c);
        for _ in 0..180 {
            s.step(1.0 / 60.0, 0.0, 0.0, false, &c);
        }
        assert_eq!(s.state, State::Tumbling);
        for _ in 0..60 {
            s.step(1.0 / 60.0, 0.0, 0.0, true, &c);
        }
        assert_eq!(s.state, State::Resting);
        assert_eq!(s.activation, 0.0);
        s.step(1.0 / 60.0, 2.0, 1.0, true, &c);
        assert_eq!(s.state, State::Tumbling);
    }
    #[test]
    fn activation_curve_is_equivalent_at_render_rates() {
        let c = config();
        let mut values = vec![];
        for hz in [30, 60, 120] {
            let mut s = CrashControl::impact(0.3, &c);
            for _ in 0..hz {
                s.step(1.0 / hz as f32, 1.0, 1.0, true, &c);
            }
            values.push(s.activation);
        }
        assert!((values[0] - values[2]).abs() < 0.0001);
    }
}
