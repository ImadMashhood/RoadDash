# Parts 00–01 and reported crash/recovery fixes — execution receipt

Date: 2026-10-30. Base commit: a0902c9. Branch:
`dev/parts-00-01-foundation`. Scope includes the user's additional crash/momentum
and failed-remount reports. The bundled production PDF is a specification source;
its future parts are not automatic implementation authority or completion evidence.

## Automated verification

See [complete command output](local-checks.txt) for exit codes. Rust 1.94.0,
Bevy 0.17.3, Rapier 0.32.0, Linux x86_64. Workspace tests: **33 passed**
(27 client, 3 core, 3 content), zero failures. Existing 18 behavioral tests remain.

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | Passed |
| `cargo test --workspace --locked` | Passed, 33 tests |
| `cargo build --locked` | Passed, native debug client |
| `cargo build --release --locked` | Passed, native optimized client |
| `cargo check --locked --no-default-features --features offline` | Passed |
| `cargo check --locked --features debug_physics` | Passed |
| `python3 tools/test_audit.py` | Passed, SPDX expression parser cases |
| `python3 tools/audit.py` | Passed: 510 dependency declarations, 3 runtime media/source records, 23 documentary observations |
| `python3 tools/sync_progress.py --check` | Passed, complete checklist index matches README |
| `git diff --check` | Passed |

Behavior evidence: equal 120 fixed ticks and poses for identical two-second input
at 30/60/120 render FPS; short attack survives a zero-tick render frame and fires
once; 100 crash/pause/resume/reset cycles freeze bodies while paused and clear
transient actors/joints; Boot/Loading/MainMenu/Race/Results lifecycle; versionless
profile preservation and read-only handling of corrupt/future files.

Crash regressions inspect spawn velocities **before the physics solver**: no
synthetic Y boost, forward fallback is not scaled, measured lateral/vertical
velocity is retained, rider separates laterally and the wreck has no seated-rider
visual. Repeat knockdowns use current on-foot movement rather than stale riding
velocity. Twelve approaches (both sides of upright, ±90° roll, upside-down and
±90° pitch wrecks) all remount within eight simulated seconds. The complete
dynamic crash/get-up/run/remount test restores health/immunity and removes debris.
This does not certify arbitrary enclosed geometry, manual walking or foot finish.

[Process probes](process-probes.txt): valid content exits 0, unsupported schema
exits 2; intentional panic exits 101 and creates exactly one local report with
version/OS/backtrace. This deliberate probe is not a game crash observed in play.
No telemetry is sent. The exact root-package fmt/clippy/test commands were also
run locally; workspace commands above include all members and are the CI checks.

## Native release smoke and visual review

Fedora 44 KDE, NVIDIA GeForce RTX 3080, NVIDIA Vulkan adapter. Actual command:
`./target/release/road-dash --smoke-test`. The script opens a 1440×900 window,
initializes four audio playback sinks, verifies active racing playback, triggers
an ordinary crash, observes getting up/running/mounting, then checks results.
Profile writes are disabled by the fixture. It exits successfully.

[Native output](native-smoke.txt) and [diagnostics JSON](native-metrics.json)
are saved with the reviewed captures. Final capture: 732 render frames, 731 fixed
ticks (631 active); bounded wall average 7.00 ms / P95 16.99 ms, last gameplay
0.114 ms / physics 0.604 ms; 4,298 entities and 1,370 colliders. The fixture advances simulation by 1/60 s
per rendered frame; wall timings describe this scripted run, not normal real-time
60 Hz pacing or a certified benchmark. Fixed tick independence is tested separately
at three render rates. Results are induced **after** a real remount by moving the
rider to the finish location and clearing stale pre-teleport contacts; this checks
the results UI, not checkpoint validation or finishing on foot.

- [Racing and F3 overlay](native-racing.jpg): road/scenery, HUD and separated diagnostics are readable.
- [Crash](native-crash.jpg): rider falls independently beside the riderless wreck; other racers remain mounted.
- [Running](native-running.jpg): camera follows the on-foot actor toward the actual bike.
- [Mounting](native-remounting.jpg): bike is raised upright at its physical location.

Visual review covered the captures above plus garage, get-up and results PNGs
in the local ignored `screenshots/` directory. The mathematical Reinhard
replacement renders the procedural scene acceptably; no imported LUT is shipped.
Audio sinks/playback and synthesized non-silent PCM are verified. No subjective
listening, spatial mix approval or controller playtest is claimed. Vulkan emitted
nonfatal unavailable-extension/suboptimal-present warnings; the run completed.

## Corrections found during verification

A results fixture originally retained contacts from before its teleport; those
contacts were explicitly cleared. Fixed state setup and buffered early input were
adjusted to respect Boot/Loading. Crash tests exposed shoulder damage immediately
after remount; existing immunity now covers that damage. A repeat knockdown could
inherit old bike velocity; it now reads the runner controller's movement.
These changes were followed by the full checks and final native smoke.

## Open gates

Part 00's policy/schema/research/backlog/slice artifacts are complete and reviewed.
Part 01 is **partial**: the real core/content split, subsystem plugins, fixed
scheduler, input buffer, schema contracts, diagnostics and local runtime are in
place. Full simulation/physics extraction, authority-event contracts, all proposed
feature/quality modes and Windows hardware launch remain open. CI is configured
for Ubuntu and Windows; a configuration file is not a successful remote run.
Record actual PR checks separately. No Windows GPU/audio/input session or clean
install, ten-minute physics soak, multiplayer, production performance benchmark,
vulnerability clearance or commercial legal signoff was performed here.
