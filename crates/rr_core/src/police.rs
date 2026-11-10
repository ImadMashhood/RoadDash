//! Authored police policy, independent of presentation and physics.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rules {
    pub speed_limit: f32,
    pub detection_radius: f32,
    pub launch_grace: f32,
    pub warning_seconds: f32,
    pub lose_seconds: f32,
    pub search_seconds: f32,
    pub arrest_seconds: f32,
    pub arrest_radius: f32,
    pub arrest_speed: f32,
    pub patrol_speed: f32,
    pub patrol_fraction: f32,
}
/// A nearby on-foot target causes ordinary braking, then deliberate foot pursuit.
pub const FOOT_APPROACH_RANGE: f32 = 24.0;
pub fn approach_speed(target_speed: f32, gap: f32, on_foot: bool, top: f32, rules: Rules) -> f32 {
    if on_foot && gap.abs() <= FOOT_APPROACH_RANGE {
        return 0.;
    }
    let stand_off = if target_speed <= rules.arrest_speed {
        6.0
    } else {
        2.8
    };
    (target_speed + (gap - stand_off) * 0.75).clamp(0., top)
}
/// Search approaches a stationary last-seen point, without retaining the unseen
/// rider's velocity feedforward or driving indefinitely beyond that point.
pub fn search_speed(gap: f32, top: f32) -> f32 {
    ((gap - 2.0) * 0.75).clamp(0.0, top)
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Patrol,
    Warning,
    Pursue,
    Intercept,
    Search,
    Return,
    Arrested,
}
impl State {
    /// Direction changes inside an existing chase do not restart its siren/HUD.
    pub fn announces(self, previous: Self) -> bool {
        self != previous
            && !(matches!(self, Self::Pursue | Self::Intercept)
                && matches!(previous, Self::Pursue | Self::Intercept))
    }
    pub fn wanted(self) -> bool {
        matches!(self, Self::Pursue | Self::Intercept | Self::Search)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Observation {
    pub elapsed: f32,
    pub visible: bool,
    pub violation: bool,
    pub target_ahead: f32,
    pub officer_ready: bool,
    pub target_ready: bool,
    pub distance: f32,
    pub target_speed: f32,
    pub relative_speed: f32,
    pub at_patrol: bool,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Mind {
    pub state: State,
    pub warning: f32,
    pub unseen: f32,
    pub search: f32,
    pub arrest: f32,
}
impl Mind {
    /// Called only at the budgeted perception cadence. Unsafe frames must also
    /// clear arrest dwell through `interrupt_arrest`, so stale LOS cannot arrest.
    pub fn step(&mut self, o: Observation, dt: f32, rules: Rules) {
        if !dt.is_finite() || dt <= 0.0 || o.elapsed < rules.launch_grace {
            self.interrupt_arrest();
            return;
        }
        match self.state {
            State::Patrol | State::Return => {
                if o.visible && o.violation && o.officer_ready {
                    self.state = State::Warning;
                    self.warning = 0.0;
                } else if o.at_patrol {
                    self.state = State::Patrol;
                }
            }
            State::Warning => {
                if !o.visible || !o.violation || !o.officer_ready {
                    self.state = State::Patrol;
                    self.warning = 0.0;
                } else {
                    self.warning += dt;
                    if self.warning >= rules.warning_seconds {
                        self.state = State::Pursue;
                        self.unseen = 0.0;
                    }
                }
            }
            State::Pursue | State::Intercept => {
                self.unseen = if o.visible { 0.0 } else { self.unseen + dt };
                if self.unseen >= rules.lose_seconds {
                    self.state = State::Search;
                    self.search = 0.0;
                } else if o.visible {
                    self.state = if o.target_ahead < 0.0 {
                        State::Intercept
                    } else {
                        State::Pursue
                    };
                }
            }
            State::Search => {
                if o.visible && o.officer_ready {
                    self.state = State::Pursue;
                    self.unseen = 0.0;
                } else {
                    self.search += dt;
                    if self.search >= rules.search_seconds {
                        self.state = State::Return;
                    }
                }
            }
            State::Arrested => (),
        }
    }
    /// Validate every fixed tick while in arrest range, independently of perception.
    pub fn contact(&mut self, o: Observation, dt: f32, rules: Rules) {
        if self.state.wanted()
            && o.elapsed >= rules.launch_grace
            && dt.is_finite()
            && dt > 0.
            && self.state != State::Search
            && o.visible
            && o.officer_ready
            && o.target_ready
            && o.distance <= rules.arrest_radius
            && o.target_speed <= rules.arrest_speed
            && o.relative_speed <= rules.arrest_speed
        {
            self.arrest += dt;
            if self.arrest >= rules.arrest_seconds {
                self.state = State::Arrested;
            }
        } else {
            self.interrupt_arrest();
        }
    }
    pub fn interrupt_arrest(&mut self) {
        self.arrest = 0.0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn rules() -> Rules {
        Rules {
            speed_limit: 30.,
            detection_radius: 90.,
            launch_grace: 8.,
            warning_seconds: 1.,
            lose_seconds: 2.,
            search_seconds: 3.,
            arrest_seconds: 1.,
            arrest_radius: 1.8,
            arrest_speed: 1.2,
            patrol_speed: 14.,
            patrol_fraction: 0.05,
        }
    }
    fn seen() -> Observation {
        Observation {
            elapsed: 10.,
            visible: true,
            violation: true,
            officer_ready: true,
            target_ready: true,
            distance: 10.,
            target_speed: 40.,
            relative_speed: 10.,
            ..Default::default()
        }
    }
    #[test]
    fn unseen_target_cannot_change_interception_direction() {
        let mut m = Mind {
            state: State::Pursue,
            ..Default::default()
        };
        let mut o = seen();
        o.visible = false;
        o.target_ahead = -50.;
        m.step(o, 0.1, rules());
        assert_eq!(m.state, State::Pursue);
        o.visible = true;
        m.step(o, 0.1, rules());
        assert_eq!(m.state, State::Intercept);
        o.visible = false;
        o.target_ahead = 50.;
        m.step(o, 0.1, rules());
        assert_eq!(m.state, State::Intercept);
    }
    #[test]
    fn search_brakes_at_last_seen_point_and_direction_flips_do_not_reannounce() {
        assert_eq!(search_speed(1000., 55.), 55.);
        assert!(search_speed(10., 55.) < search_speed(20., 55.));
        for gap in [-100., 0., 2.] {
            assert_eq!(search_speed(gap, 55.), 0.);
        }
        assert!(!State::Intercept.announces(State::Pursue));
        assert!(!State::Pursue.announces(State::Intercept));
        assert!(State::Pursue.announces(State::Search));
        assert!(State::Search.announces(State::Pursue));
        assert!(State::Return.announces(State::Search));
    }
    #[test]
    fn nearby_moving_foot_target_requests_ordinary_stop_not_endless_bike_tail() {
        for gap in [-20., 0., 20.] {
            assert_eq!(approach_speed(4., gap, true, 60., rules()), 0.);
        }
        assert!(approach_speed(4., 20., false, 60., rules()) > 4.);
        assert_eq!(approach_speed(4., 200., true, 60., rules()), 60.);
    }
    #[test]
    fn warning_is_continuous_and_compliance_cancels() {
        let mut m = Mind::default();
        let mut o = seen();
        for _ in 0..5 {
            m.step(o, 0.1, rules());
        }
        assert_eq!(m.state, State::Warning);
        o.violation = false;
        m.step(o, 0.1, rules());
        assert_eq!(m.state, State::Patrol);
        o.violation = true;
        for _ in 0..13 {
            m.step(o, 0.1, rules());
        }
        assert!(m.state.wanted());
    }
    #[test]
    fn grace_loss_search_and_return_are_explicit() {
        let mut m = Mind::default();
        let mut o = seen();
        o.elapsed = 1.;
        m.step(o, 1., rules());
        assert_eq!(m.state, State::Patrol);
        o.elapsed = 10.;
        for _ in 0..3 {
            m.step(o, 1., rules());
        }
        assert!(m.state.wanted());
        o.visible = false;
        for _ in 0..6 {
            m.step(o, 1., rules());
        }
        assert_eq!(m.state, State::Return);
        o.at_patrol = true;
        m.step(o, 0.1, rules());
        assert_eq!(m.state, State::Patrol);
    }
    #[test]
    fn arrest_needs_slow_settled_visible_continuous_contact() {
        let mut m = Mind {
            state: State::Pursue,
            ..Default::default()
        };
        let mut o = seen();
        o.distance = 1.;
        m.contact(o, 2., rules());
        assert_ne!(m.state, State::Arrested);
        o.target_speed = 0.;
        o.relative_speed = 0.;
        o.target_ready = false;
        m.contact(o, 2., rules());
        assert_ne!(m.state, State::Arrested);
        o.target_ready = true;
        m.contact(o, 0.5, rules());
        assert_eq!(m.arrest, 0.5);
        o.visible = false;
        m.contact(o, 0.1, rules());
        assert_eq!(m.arrest, 0.);
        o.visible = true;
        m.contact(o, 1., rules());
        assert_eq!(m.state, State::Arrested);
    }
}
