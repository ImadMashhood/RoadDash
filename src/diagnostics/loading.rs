//! Opt-in startup spans. Nested spans overlap; they must not be summed.
use std::{sync::OnceLock, time::Instant};
fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("ROADDASH_LOAD_PROFILE").as_deref() == Ok("1"))
}
fn origin() -> &'static Instant {
    static ORIGIN: OnceLock<Instant> = OnceLock::new();
    ORIGIN.get_or_init(Instant::now)
}
pub fn mark(name: &'static str) {
    if enabled() {
        eprintln!(
            "LOAD_PROFILE {}",
            serde_json::json!({"event":name,"elapsed_ms":origin().elapsed().as_secs_f64()*1000.})
        );
    }
}
pub struct Span {
    name: &'static str,
    start: Option<Instant>,
}
impl Span {
    pub fn new(name: &'static str) -> Self {
        let start = enabled().then(|| {
            origin();
            Instant::now()
        });
        Self { name, start }
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            eprintln!(
                "LOAD_PROFILE {}",
                serde_json::json!({"span":self.name,"duration_ms":start.elapsed().as_secs_f64()*1000.,"elapsed_ms":origin().elapsed().as_secs_f64()*1000.})
            );
        }
    }
}

/// Main-world milestones are explicit; these do not claim GPU presentation.
pub(super) fn milestones(
    screen: bevy::prelude::Res<bevy::prelude::State<crate::runtime::AppScreen>>,
    mut seen: bevy::prelude::Local<[bool; 2]>,
) {
    if !enabled() {
        return;
    }
    for (i, state, name) in [
        (0, crate::runtime::AppScreen::MainMenu, "main_menu_update"),
        (1, crate::runtime::AppScreen::Race, "first_race_update"),
    ] {
        if *screen.get() == state && !seen[i] {
            mark(name);
            seen[i] = true;
        }
    }
}
