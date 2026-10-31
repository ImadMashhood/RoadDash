# Part 04 verification receipt — 2026-10-30

Scope: start Part 04 (camera, speed feel, input and comfort). The user requested
inclusion in PR #3; GitHub already merged it as dddcf3a before publication, so
Part 04 uses a focused follow-up based on that exact merged tree. No Part 05 start.
Branch: dev/part-04-camera-input. [Contract](../CAMERA_AND_INPUT.md).

## Automated checks

[Full local command log](part-04/local-checks.txt): all commands exit 0.

```sh
cargo fmt --all --check
cargo clippy --workspace --locked --all-targets -- -D warnings
cargo test --workspace --locked
cargo build --locked
cargo build --release --locked
cargo check --locked --no-default-features --features offline
cargo check --locked --features debug_physics
target/release/road-dash --validate-track content/tracks/valley.json
target/release/road-dash --validate-track content/tracks/handling.json
target/release/road-dash --validate-handling content/handling.json
python3 tools/test_audit.py
python3 tools/audit.py --output docs/testing/part-04/declaration-audit.json
python3 tools/sync_progress.py --check
cargo test --locked ten_minute_aggressive -- --ignored --nocapture
git diff --check
```

87 regular Rust tests passed (72 client, seven content, eight core); one explicit
600-second simulated turning soak passed separately; two Python tests passed.
No failures or ignored ordinary requirements are presented as passes. The soak
retains its simulated-time/handling-fixture limits from Part 03. Old regression
coverage includes controls, direct combat, traffic, race/gates/payout, boost/grass,
ragdoll recovery, save compatibility, repeated reset/paused physics and audio.

New checks:

| Source | Scenario / acceptance |
| --- | --- |
| camera_tests.rs | Finite continuous camera at 60/120/144 Hz across acceleration/turn input; final positions within 1.5 m; no ordinary frame displacement >1.5 m |
| camera_tests.rs | Sphere-volume stays in front of a solid wall; configured 95° FOV remains clamped; large retry-style warp snaps near the new rider within four frames |
| controls.rs | Simulated analog trigger/steering at half strength, right-stick look, controller removal clears state and keyboard fallback steers |
| controls.rs | Modal settings pauses/restores countdown; held driving suppressed; dismounted context isolates mounted intentions |
| controls.rs | Live keyboard nitro remap and controller button remap update config without hidden vehicle action |
| settings.rs | Comfort/button roundtrip using isolated temp files, unsupported/corrupt original preserved read-only, duplicate buttons/invalid files rejected |
| rr_core::comfort | Continuous bounded deadzone including NaN rejection; FOV/range validation and anti-nausea removes speed expansion |
| vehicle_tests.rs | Quarter trigger pressure produces less actual chassis acceleration than full pressure |
| feedback.rs | Actual HUD Text reports traction/slip; 100 updates retain twelve speed cues; anti-nausea hides all cues |

Initial development tests caught fixture issues (repeated held-key press without
release; camera read before pose-history warmup). Corrected the input sequences
and initial state, then reran the full suite; final acceptance assertions remain.

## Build environment and provenance

Rust 1.94.0 / Bevy 0.17.3 / Rapier 0.32.0 retained. Enabled compatible Bevy Gilrs;
locked gilrs 0.11.2 / gilrs-core 0.6.8. License/mapping investigation: R14/R15 in
[registry](../third_party/REPOSITORIES.md); MIT/Apache and Zlib notices included.
Controller mapping bytes verified against exact upstream gitlink; no artwork,
examples or unknown-license media copied. Declaration audit: 519 packages,
13 runtime provenance records, 23 historical reference observations.

This Fedora host has libudev.so.1 but lacks development metadata/linker symlink;
passwordless sudo was unavailable. Verification used an explicit temporary
PKG_CONFIG_PATH=/tmp/roaddash-native/lib/pkgconfig with a libudev.pc and symlink to
the installed system library. No elevated install or system-file modification.
Normal builds should install Fedora systemd-devel or Ubuntu libudev-dev; README
and Ubuntu CI now include it. This workaround is not a clean-install certificate.

## Native evidence

Native release verification and reviewed screenshots are recorded below. Smoke
uses original scripted scene changes/hit/result fixtures; it is not an externally
controlled complete race. Preferences are in-memory during smoke to protect the
player's settings. UI/axis/persistence tests use synthetic input and temp files;
no claim of a physical controller being driven by these tests.

Final serial Fedora/RTX 3080/Vulkan release runs passed:

- `target/release/road-dash --smoke-test`: comfort panel, riding, farm crash/get-up/
  running/remount, city/hills, hill recovery, results; all fixed recovery stages
  observed, four active playback sinks. [Log](part-04/native-smoke-log.txt),
  [metrics](part-04/smoke-metrics.json).
- `target/release/road-dash --actor-smoke-test`: distinct human/cow physics and
  camera regression retained. [Log](part-04/actor-smoke-log.txt),
  [body counts](part-04/actor-smoke.json).

Reviewed final release captures: [comfort panel](part-04/15-comfort-settings.jpg),
[riding](part-04/02-racing.jpg), [city](part-04/08-city.jpg),
[hills](part-04/09-hills.jpg), [hill recovery](part-04/11-hill-running.jpg).
The first panel render exposed unsupported degree/bullet glyphs and menu text
behind it; replaced UI glyphs with plain text and hid the underlying menu/HUD,
then rebuilt/reran and reviewed this final capture.

Final main run: 1285 render frames / 1284 fixed ticks,
bounded wall-frame average 6.958 ms / P95
10.600 ms. These are scripted-scene wall samples, not GPU
profiling, busy-race performance or proof of a shipping 60/120 FPS gate.
One OS Gamepad entity was discovered, with the documented default-mapping warning.
No physical buttons/sticks were exercised. Vulkan extension/first-present warnings
did not prevent output; no audio underrun warning in final runs. Audio sinks
verify playback, not subjective mix quality. Native scene changes/results remain
scripted fixtures, not full manual completion or external tester feedback.

## Additional user corrections before publication

Removed cosmetic steering roll in both adapters; wheels/suspension remain.
`human::SPECS`/segment builder now owns all eleven mesh/scale/material slots,
helmet/visor and pose variants for bikers, pedestrians, runners and ragdolls.
Rider palette persists through crash/recovery/reset. Pedestrian support/respawn
proxies cover the shared 2.31 m standing model. Four-state appearance, neutral visual-root and tilted-chassis upright-human
tests pass. Native [held-steering capture](part-04/16-steering-upright.jpg) was
reviewed alongside [standing actors](part-04/13-actors-standing.jpg) and
[ragdoll actors](part-04/14-actors-ragdoll.jpg); the same human parts/helmet/outfit
remain visible through the transition. This remains original procedural art, not a production rig.

Civilian impact now launches the mounted biker using height+margin and actual
bike+rider mass, restoring pre-solver horizontal approach speed; normal crashes
retain momentum-only behavior. [Six measured class/civilian cases](part-04/hazard-clearance.json)
verify finite physics, peak bike origin above standing height+0.25 m, passing the
actor and staying mounted while the civilian ragdolls. Cow peaks: 2.247–2.315 m
against 1.70 m height; pedestrian peaks: 3.993–4.020 m against 2.31 m height.
[Targeted output](part-04/hazard-tests.txt). This is an arcade clearance jump;
arbitrary low-speed crowd piles are outside this scenario matrix.

Crossing cars use explicit hazard classification, all collision types and CCD.
Four physical tests pass: assisted/dynamic bikes hit stopped crossing cars, and
moving crossing cars strike stationary bikes. Tests intentionally omit the generic
Obstacle marker so losing that tag cannot silently remove wipeout behavior.
Existing ordinary-crash momentum and recovery regressions remain in the full suite.

## Open requirements

- P04-004 partial: numeric high-refresh/interpolated-heading coverage exists;
  human 120–144 Hz jitter/readability acceptance through all jumps/impacts remains.
- P04-008/009: physical controller completion and an external tester's responsive,
  fast, comfortable and readable verdict are not available. Keyboard regressions
  and native scripted views do not replace these gates.
- P04-014: native backend discovered an OS device but logged an unmapped-controller
  warning for UUID 03000000-03a1-0000-0500-000011010000 and used its default mapping.
  Exercise real buttons/axes before claiming it is correct or adding a mapping.
- InputContext::OnFoot allocates isolated intentions; manual foot movement,
  optional remount control and foot finish remain Part 06/07.
- Settings do not persist audio controls/garage classes or arbitrary axis remaps.
  No controller rumble, full clean-install/device matrix or Windows GPU run claimed.
- P03 ten human rides/fun acceptance remains open. No Part 05 advancement.

P04-005/006/007 and verified subitems P04-011/012/013 are checked for implemented,
automatically tested capabilities; larger acceptance gates stay unchecked.
