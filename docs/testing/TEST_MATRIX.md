# Testing requirements and evidence matrix

Scope: Parts 00-01, 2026-10-30. Detailed receipts: PARTS_00_01.md. Never replace
unrun platform results with assumptions from a successful local build.

| ID | Scenario | Verification |
| --- | --- | --- |
| T-CORE | IDs, edge buffering, award rules | rr_core tests, no renderer/physics dependency |
| T-CONTENT | version, finite tuning, colors, references, duplicates | rr_content tests; --validate-content success/failure |
| T-INPUT | remapped/unknown/duplicate bindings; zero-tick frame press | controls tests and attack_press_survives_a_render_frame_without_a_fixed_tick |
| T-FIXED | identical two-second intent at 30/60/120 render FPS | fixed_simulation_matches_at_30_60_and_120_render_fps; exact 120 ticks and pose tolerances |
| T-RESET | 100 crash/pause/resume/reset cycles | frozen body positions while paused, no debris/joints/disabled colliders/entity leaks |
| T-STATES | Boot -> Loading -> MainMenu -> Race -> Results | lifecycle_visits_loading_menu_race_and_results |
| T-CRASH | momentum/separation and riderless wreck | crash_separates_rider_and_preserves_momentum_without_an_upward_boost; pre-solver spawn snapshot |
| T-REMOUNT | six fallen poses, approached from both sides | remount_reaches_a_fallen_bike_in_sideways_upside_down_and_vertical_poses; 8-second bound each |
| T-REGRESSION | controls/combat/traffic/ragdoll/runner/camera/finish | existing gameplay tests retained in game/track/world/sound |
| T-SAVE | versionless credit/time preservation, schema 1 roundtrip, corrupt/future rejection | legacy_profile_is_upgraded_without_losing_progress |
| T-AUDIT | dependency declarations, media rights/hash, 20+ observations | tools/audit.py plus tools/test_audit.py |
| T-NATIVE | real window, audio sinks, crash -> foot -> remount -> results | --smoke-test screenshots + wall/fixed diagnostics JSON; scripted finish fixture is not checkpoint/foot-finish proof |
| T-PANIC | local backtrace file and expected nonzero process | --verify-panic-report with temporary ROADDASH_LOG_DIR |
| T-FEATURES | minimal offline and debug collider builds | --no-default-features --features offline; --features debug_physics |
| T-WINDOWS | MSVC build/headless tests + native GPU/audio/input | CI matrix for former; separate real Windows session for latter (open) |
| T-NVIDIA | Fedora KDE Wayland/NVIDIA launch | target-specific hardware session required (open unless adapter evidence matches) |

Mandatory local commands:

```sh
cargo fmt --all --check
cargo clippy --workspace --locked --all-targets -- -D warnings
cargo test --workspace --locked
cargo build --locked
python3 tools/test_audit.py
python3 tools/audit.py
cargo run -- --validate-content content/catalog.json
cargo run -- --smoke-test
```

CI repeats the source/headless checks on Ubuntu and Windows. It cannot replace
real input-device, graphics, audio, clean-install, performance or rights review.

## Part 02 additions (2026-10-30)

Detailed receipt: PART_02.md. New checks: headless authored road imports including
open endpoints and invalid geometry/gates/spawns; render/collision ribbon winding;
240 terrain/road support rays across all zones; no-teleport complete fixed-controller
point-to-point race, skipped/reverse/outside gates and missed-finish retry; boost
release/exhaustion, four grass AI/player cases, cow lift, solid slope slide and
100 physical ragdoll drops with finite transforms/velocities, anchor and hinge-axis
checks. Native smoke includes both farm and hill recovery, city, hills and F4
inspector captures, active audio sinks and results. World-tour/result teleports
are presentation fixtures only; race validation is the separate controller test.

## Part 03 additions (2026-10-30)

Receipt: [PART_03.md](PART_03.md). Regular headless checks explicitly cover:
three preset acceleration/braking and wheel support; dynamic acceleration/braking
at 30/60/120 FPS; leaning/inverted self-righting; stationary and moving curb hits;
eight-rider grid; no lane-zero magnet; dry wreck mass and stationary remount;
direct combo timing/duplicates/geometry and matched-speed contact thresholds;
NPC topologies/budget/reset/occupied respawn; shirt-color minimap identities.
Explicit ignored soak runs 600 simulated seconds of alternating steering and
records numeric acceleration/braking/steady-turn data. CI runs it on Linux.
`--validate-handling content/handling.json` validates the pure authoring contract.
Native `--smoke-test` and `--actor-smoke-test` run serially with graphics/audio;
they exercise scripted scene/recovery and distinct ragdoll topologies. They do
not replace manual ten-ride feedback, beginner finish, tactile control testing,
free-form crowd collisions or actual Windows hardware runs.

## Part 04 additions (2026-10-30)

Receipt: PART_04.md. Camera continuity at 60/120/144 Hz, sphere-clearance wall,
warp snap, native comfort panel/city/hill/recovery captures; analog trigger values
reach chassis forces; gamepad steering/freelook/deadzone/disconnect and keyboard
fallback; modal countdown pause/restore; live keyboard/controller rebinding and
invalid/conflicting settings preservation. Real controller completion, tactile
input/comfort feedback and subjective high-refresh readability remain open.

Latest corrections: four human states share mesh/material/scale slots, steering
demand does not roll visual root, six civilian-kind/class contacts clear measured
height/pass obstacle with finite physics, and four cross-car cases wipe out (both
adapters, approaching a stopped car / moving car striking a stopped bike).

## Source-informed steering / camera correction

Receipt: HANDLING_CORRECTION.md. Core checks held-force buildup, return/reversal,
speed dependence/parked zero and timestep agreement; content accepts old profiles
and rejects invalid response. Dynamic integration verifies increasing turn force
and no parked strafe. Actual visual hierarchy banks bike/rider together while
removing a tilted road's roll at zero steering. Camera distance stays bounded in
27 mode/speed/render-rate scenarios with slow smoothing. Existing car/civilian,
recovery, controls, collision and race tests stay in the full suite. Numeric
original-game equivalence is not tested/claimed; P03-020 needs missing data.
