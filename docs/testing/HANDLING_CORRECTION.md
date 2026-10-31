# Handling / lean / camera correction — 2026-10-30

Scope: repair turn bank and excessive chase distance; investigate the user's
requested Road Rash handling target and implement verified steering buildup.
Branch: fix/reference-steering-camera, based on merged Part 04 (cbf25dd).

## Implemented and verified software

- Persistent normalized player steering force, speed-scaled angle target,
  distinct buildup/return rates and countersteering through existing force.
  Both adapters consume it at fixed ticks; stopped input cannot strafe the bike.
  Latest user adjustment halves rise/release timing and doubles visual response
  to 16 Hz for 2x faster full lean, keeping the same force/angle limits.
  Crash/reset clears memory. Telemetry separates raw input and resolved force.
- Bike and seated rider bank together. Zero input force means zero rendered bank
  even on a chassis tilted by road bank; grade/physics remain. No road-curvature
  term feeds rider lean. Repeated presentation updates do not accumulate roll.
- Near/default/far remain close. Velocity lead compensates follow lag; camera
  distance from its focus is bounded to 3.93/5.20/6.68 m respectively. Optional
  speed FOV expansion is at most 3 degrees, not 8. Saved comfort values remain.

[Source investigation](../research/ROAD_RASH_HANDLING.md) distinguishes inspected
reference behavior from implementation and missing exact calibration. The user
confirmed no original files. **The entire original handling has not been copied**:
engine/brake, gear, nitro, traction, original table values and fixed-point numeric
parity remain unverified/unimplemented as original-equivalent. P03-020 stays open.
Provisional RoadDash steering timings are explicitly identified in the source
investigation/content contract; no original data or C++ code imported.

## Automated verification

[Exact command log](handling-correction/local-checks.txt). Commands:

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
python3 tools/audit.py --output docs/testing/handling-correction/declaration-audit.json
python3 tools/sync_progress.py --check
cargo test --locked ten_minute_aggressive -- --ignored --nocapture
git diff --check
```

Final results recorded in the log. Regular suite: 95 tests (76 client, eight
content, eleven core); explicit ignored soak runs separately. New/updated scenarios:

| Source | Acceptance |
| --- | --- |
| rr_core::steering | Held force increases, release returns to neutral, reversal unwinds before changing sign; low speed reduces target; 30/60/120 step sizes agree |
| rr_content::handling | Older schema-1 profile defaults and invalid response rejection |
| vehicle_tests.rs | A parked bike cannot strafe, while a sustained riding turn produces substantially greater force than a short tap |
| human.rs | Actual shared visual hierarchy, left/neutral/right bank over a yawed chassis with 0.35 rad road roll; human follows bike and neutral removes road roll across 60 updates |
| camera_tests.rs | 27 combinations: all three views, 0/30/80 m/s, 60/120/144 render Hz, response set to slowest 2; every camera sample remains within distance bound |
| Existing regressions | Camera continuity/wall clearance/warps, car wipeouts, civilian clearance, ragdoll momentum/recovery/reset, analog input/settings, grass/boost, race gates/payout, shirt map and audio |

The old parked instantaneous-steer assumptions were corrected to the reference's
no-steering-at-zero-speed and persistent-countersteering behavior. Solid slow
side contact still must trigger an impact while moving; its assertion is retained.
The 600-second simulated soak remains an isolated handling fixture, not ten human
rides or a native 10-minute endurance race. No reference game was executed and
no original-game golden numeric parity is asserted by these tests.

## Environment / provenance

Pinned Rust 1.94.0 / Bevy 0.17.3 / Rapier 0.32.0 unchanged; no new dependency.
System pkg-config now resolves libudev version 259. Commands use plain Cargo,
without the earlier temporary PKG_CONFIG_PATH workaround. Previous Part 04
receipts remain historical; no evidence was overwritten.

R01 source follow-up records exact revision, functions, data dependencies and
unresolved source-reuse permission. Four new source observations append to the
23 historical records. Modified original model/config source hashes refreshed.
No reference source/media, disc tables, game models or assets incorporated.

## Native verification

Both serial release checks passed on NVIDIA RTX 3080 / Vulkan:

- [Native log](handling-correction/native-smoke.txt): farm crash, get up, run,
  remount, city/hills, hill crash, remount, finish; four audio playback sinks.
- [Actor log](handling-correction/actor-smoke.txt): independent human/quadruped
  ragdolls, solid contacts and bounded bodies. This fixture sends direct hit
  messages; it does not replace a manually driven collision check.
- [Scene metrics](handling-correction/smoke-metrics.json): 1,209 fixed ticks,
  1,109 active ticks; 16.83 ms average / 18.41 ms p95 sampled wall frame time.
  These are bounded samples, not GPU timing or hardware certification.
- Reviewed [held input bank](handling-correction/17-held-steering.jpg),
  [buildup](handling-correction/16-steering-buildup.jpg),
  [default chase](handling-correction/02-racing.jpg),
  [city near view](handling-correction/08-city.jpg),
  [hills far view](handling-correction/09-hills.jpg) and
  [hill recovery](handling-correction/11-hill-running.jpg). Bike and rider bank
  together under steering, and the neutral hill view stays upright.

The driver reported missing optional Vulkan extensions and a controller UUID
without a mapping (default mapping used); neither prevented the checks. These
scripted scene changes are not a complete manual race, a verified physical
controller test, or a reference-game comparison.

## Remaining acceptance

P03-020 whole-handling equivalence is blocked on missing verified/authorized
original calibration and comparative traces. Current response times are authored
provisional values. The modern 3D force/contact adapter is retained; it is not
fixed-point C++ simulation parity. Original braking, gearing, nitro-charge and
surface behavior are not misrepresented as complete.

Physical-controller mapping (P04-014), external high-refresh/readability/fun
acceptance (P04-004/008/009), ten human rides and broader crowd/low-speed tuning
remain open. This correction supersedes the historical upright-character request;
latest input-only coordinated bank controls the current implementation. No Part 05.

Latest turn-strength adjustment (P03-022): profile turn response increases 25%
(light 1.5, balanced 1.25, heavy 1.025). Dynamic steering and the assisted
adapter use the same profile gain. Maximum visual bank and the doubled lean
response are unchanged. These remain authored RoadDash settings.
