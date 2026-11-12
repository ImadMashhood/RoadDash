# Issue #37 — Bevy 0.20 migration candidate

## 2026-11-11 playtest feedback candidate (ADR163)

The user's persistent Hills log (`~/.local/state/roaddash-bevy20-playtest/performance-1809357.jsonl`)
has 513 racing one-second windows: median raw frame 17.27 ms and p95 27.36 ms,
median gameplay 5.76 ms and physics 3.39 ms. A sampled opening had the player
in seventh around 45 s, 213 m behind first; this is evidence of a fast lead
pack, not a controlled balance comparison. No performance gain is claimed.

The candidate caps race rivals at 5 km/h below the player's unboosted bike,
preserving separate police limits, and boosts AI drive torque only in the
opening five seconds; a trapped-rider
physical escape test passes with the bounded window. Five stable fighters
seek live nonfighters ahead, dropping downed targets immediately. Nonfighters
retaliate against the confirmed attacker; target choice never lowers race
speed. The player can still strike pedestrians. Pedestrian patrols have a
wider swept kinematic capsule and collision events.

Spatial cloud banks increase from eight to fourteen and use the existing
volume material in Valley as well as Hills/Metropolis, with frustum culling.
The imported Hills city keeps source pedestrians and adds sidewalk-supported
small furnishings. Three timed mounted riders cross western streets; two
four-cow groups cross the later desert road with the existing solid cow
contact rig. The original off-road horse/bison paths remain unchanged after
a closer-path attempt failed authoring validation. The bridge water is now
halfway up the canyon with a broad surface, no collider and a vertex-supported
shoreline; its bank strip can differ by under 1 m at the center of triangles
crossing eight-metre resident terrain facets. Existing grass blades are
thicker with a longer Hills draw range, sand gets deterministic vertex colour
variation, and the original bounded dust pool now fades in/out with lower,
sand-coloured opacity and slower drift.

All eleven requirements remain open in the README pending human race/fight,
bridge/crossing, three-map cloud and temporal dust review, plus matched release
frame/GPU measurements. The original Bevy/Rapier high-speed ragdoll and
static-collider trajectory parity tests remain engine merge blockers. This
candidate uses only original procedural source and existing project media;
registered source hashes were refreshed in `docs/third_party/ASSETS.csv`.

Validation on the final source: `cargo fmt --all --check`, strict locked
all-target Clippy, `python3 tools/audit.py`, progress sync and five roadmap
tests pass. `cargo test --workspace --locked --no-fail-fast` records 462 game
library passes, the two established Rapier parity failures and one ignored
test; rr_content has 59 passes and rr_core 151 passes (one ignored), and all
remaining targets pass. Focused buffered strike, physical rider trap escape
and river geometry tests pass. Both locked debug and release builds pass. The
two failing assertions were not relaxed.

The final release executable is 249,163,096 bytes, SHA-256
`9b37f5447613dc3373400244fec8b1b5a0a010cf4dbc08a4eceb3a41a1175327`.
This exact binary passed Metropolis and Hills `--smoke-test`: both initialized
audio, verified the fourteen-rival/five-fighter grid, and completed two
zero-error remounts; Hills reported zero exceptional rescues. The earlier
same-visual-source Hills revision survey completed its 17 camera captures and
bridge fall verification; `screenshots/revision-bridge-open-sides.png` shows
water beside the deck, and the native load reached first race update at
8.55 s. Logs: `/tmp/roaddash-feedback-final-metro-smoke.log`,
`/tmp/roaddash-feedback-final-hills-smoke.log` and
`/tmp/roaddash-feedback-hills-survey.log`.

On the exact final binary, `ROADDASH_TRACK=valley --atmosphere-survey` captured
six moving views with 14 volumetric banks and 12 ray steps. Its CPU wall-frame
median/p95 was 13.39/19.02 ms across 1,080 post-warm frames; this is one
fixture on one GPU, not a matched performance comparison or GPU timestamp.
`screenshots/atmosphere-0.png` visibly shows cloud parallax over Valley.
Report: `screenshots/atmosphere.json`; console:
`/tmp/roaddash-feedback-final-valley-clouds.log`. Human gameplay and matched
cost gates remain open.

## Disappearing world, performance log and rival pace follow-up (2026-11-11)

The user reported that Metropolis geometry disappeared around the city split,
with only nearby facades visible, and Hills failed almost immediately. Installed
Bevy 0.20.1 source shows its render visibility-range registry retains distinct
values for the process lifetime and warns beyond 65,535 slots. RoadDash's
architecture and facade temporal fades generated camera-relative floating
ranges each frame. Both pre-fix native Hills recovery runs warned about this
limit. The isolated candidate now shares a bounded, distance-dependent fade
palette for both systems and assigns a component only on value change; settled
facades keep their exact infinite range. The palette is capped below the GPU
index limit, with static ranges left room. A focused palette test and the
existing fade origin/dither tests pass. This is the identified source-level
failure mode, though a continuous human ride has not yet confirmed that it is
the only cause of the reported view.

Release Metropolis and Hills native smokes passed without the visibility-range
warning, including two remounts each. A 3,500 m Metropolis split survey and a
Hills revision survey also passed without the warning; their visible captures
show road, rider/traffic, terrain and city content. The optional
`ROADDASH_VISIBILITY_LOG=1` diagnostic saw up to 1,078 distinct observed
ranges in the Metropolis survey and 933 in Hills, rather than a renderer slot
exhaustion warning. These values are diagnostic observations, not a precise
GPU registry count. Captures: `visual/metro-split-3500.png`,
`visual/hills-city-driving.png`, `visual/hills-city-to-hills.png`.
Camera surveys do not reproduce every player camera, streaming state or GPU.

`./play-bevy20.sh` now defaults `ROADDASH_LOG_DIR` to
`~/.local/state/roaddash-bevy20-playtest/` and records a console file plus a
`performance-PID.jsonl` file there. It enables raw frames, load spans and the
visibility diagnostic unless explicitly overridden. Each one-second JSON
record has frame/CPU/render timing, visibility churn and a `rider_pace` array
with progress, speed, lane, input and planner state. The optional visibility
history has bounded memory. This instrumentation may itself affect timing;
matched performance comparisons must use the same logging configuration.
No FPS improvement is claimed from the fade change. Recent source still
changed many ranges per window, so that path warrants profiling.

For rival pace, a matched 3,600-frame held-W Metropolis survey compared old
10/12, aggressive 16/18 and selected 14/16 m/s² corner budgets. Around 55 s,
median AI progress was 2,379 / 2,473 / 2,414 m respectively. Rider 13 was
stopped at 734 m in the control but passed 2,600 m in the selected trial;
rider 9 stayed riding and advanced beyond 2,400 m, whereas it crashed in the
aggressive trial. The survey's synthetic player stalled at station 486 in all
three runs while still marked Riding, so this comparison cannot establish
whether a person holding W can still beat every rival. The selected setting
is provisional and does not change physical rig, collision or route policy.
Human race balance and full-course pileup gates remain open.

Final source checks: `cargo fmt --all --check`, strict locked all-target
Clippy, locked debug/release builds, asset/dependency audit, progress index,
roadmap tests and shell syntax pass. `cargo test --workspace --locked
--no-fail-fast` has 461 passing library tests, two pre-existing Rapier parity
failures and one ignored library test; the other workspace targets pass.
The two failures are the high-speed anchor/limit and static-chunk trajectory
cases documented below. The release binary SHA-256 is
`25c891dcc08310abf3c926d7c007e5f0ef794b5b46f4d3f135ab779099a6f67a`.
The manifest hashes for modified original vehicle and facade source were
refreshed; no external media or source was imported.
The exact final binary passed release Metropolis and Hills native smokes with
audio, fourteen role-free rivals and two remounts per map; neither emitted the
visibility-range warning. Their performance logs have 36 and 28 records,
fifteen rider entries per sampled race window, and maxima of 312 and 214
observed distinct ranges respectively. Logs:
`/tmp/roaddash-bevy20-final-metro/` and
`/tmp/roaddash-bevy20-final-hills/`.

## Current-main user playtest (2026-11-11)

The isolated candidate merged current main `6d51d85d`, including free-roam
rival lines, role-free incident combat and the rest of the latest content.
The combined source passes `cargo check --workspace --locked --all-targets`,
`cargo fmt --all --check`, the dependency/asset audit, progress index and five
roadmap checks. Its first `cargo test --workspace --locked` run has 460 passing
library tests, two failures and one ignored test; Cargo stops before other
workspace targets. The remaining failures are the established high-speed
biomechanical anchor/limit gate (0.16624 m / 0.19046 rad) and standalone static
collider trajectory parity (1.611353 m translation divergence). The earlier
predictive traffic clearance test passes on the combined source. Test log:
`/tmp/roaddash-bevy20-playtest-tests.log`. No threshold or rig parameter was
changed to suppress these findings.

`play-bevy20.sh` runs the release binary from the correct asset directory and
uses `~/.local/share/roaddash-bevy20-playtest` for saves by default, keeping
the old-engine profile separate. `cargo build --release --locked` passed; the
249,211,000-byte executable has SHA256
`6f29912886ce2b54a5778aea621023439e2c1961a2f7a2b572a26845232e5f06`.
This exact executable passed `./play-bevy20.sh --smoke-test` on Metropolis and
`./play-bevy20.sh --hills --smoke-test` on Hills, in both the current
biomechanical and `--ragdoll-baseline` eleven-body modes. All four runs
initialized audio, verified fourteen role-free rivals, completed two
zero-pickup-error remounts and reached the results fixture; both Hills runs
reported zero exceptional rescues. Native receipts are
`/tmp/roaddash-bevy20-playtest-{metro,hills}{,-baseline}-smoke.log`.
All four emitted the existing
>65,535 distinct `VisibilityRange`s warning; it may affect distant culling or
crossfades. These scripted runs establish a launchable Linux build, not a full
race, user feel approval, matched performance gain or physics parity. This
branch is a user playtest candidate and is not merged to main.

`cargo clippy --workspace --locked --all-targets -- -D warnings` and
`cargo build --locked` pass on this integrated source. The full library test
failures remain above; no test threshold was relaxed. A subsequent
`cargo test --workspace --locked --no-fail-fast` ran every target: 669 tests
passed, two failed and two were ignored. The only failures were the same two
library physics-parity tests; content (59 tests), core (149 tests) and the
other executable targets passed. Full log:
`/tmp/roaddash-bevy20-playtest-all-target-tests.log`.

Date: 2026-11-11. Branch: `codex/issue-37-bevy-020`. Old-engine control:
`3732f179` on `origin/main` when this worktree was created. ADR149 and
RFU-ENGINE-01..03 govern acceptance. This report is a migration receipt, not a
frame-rate or hardware certificate.

## Compatibility and implementation

- Pinned Rust 1.97.1, Bevy 0.20.1, bevy_rapier3d 0.37.0 (Rapier 0.36.1),
  compatible explicit Cargo features and CI toolchain. The old main build pins
  Rust 1.94.0, Bevy 0.17.3 and bevy_rapier3d 0.32.0 (Rapier 0.31.0).
- Ported the fixed executor, Rapier pose/velocity/query API, world-asset scene
  lifecycle, mesh-owned glove morphs, assets/load builders, UI text/buttons,
  material handles and five original shaders to WESL. Kept fixed 60 Hz,
  buffered input, render-only interpolation, facade ready cache limits and
  accepted velocity-muscle/default plus eleven-body baseline policies.
- Bevy 0.20 resolves Rodio 0.22.2 and CPAL 0.17.3. The Apache-2.0 CPAL source
  is vendored under `vendor/cpal-0.17.3` with the narrow ALSA poll XRUN repair;
  the old 0.15 patch would not apply to this backend. See its patch notice.
- Updated `docs/third_party/ASSETS.csv` for source/shader hash changes and the
  unchanged Fira Mono subset supplied by bevy_text 0.20.1. No proprietary art,
  audio or reference implementation was imported.

## Checks

- `cargo check --locked --lib`: passed on this Linux host.
- `python3 tools/test_roadmap.py`: 5 passed.
- `python3 tools/sync_progress.py --check`: passed.
- `python3 tools/test_audit.py`: 3 passed.
- `python3 tools/audit.py --output /tmp/roaddash-bevy20-audit.json`: passed;
  575 dependency declarations, 205 runtime asset records. Declaration/hash
  audit only; no legal or platform certificate.
- `cargo fmt --all --check`: passed at the latest recorded source snapshot.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: passed.
- First full `cargo test --workspace --locked`: **failed**, 451 lib tests passed,
  6 failed, 1 ignored. Failures: high-speed ejection exceeded the prior
  anchor/limit bound; 100-cycle reset counted allocated entity slots rather than live entities; original
  contact recovery remained Running; Hills width differed by 0.0000005 m;
  standalone static-chunk contact trajectory diverged; the historical
  farm-rim trap stalled without reaching the road but advanced farther. These are being diagnosed;
  none is accepted as a new baseline. The run preceded the final UI picking
  and gizmo feature additions.
- `cargo test --locked --lib models::tests -- --nocapture`: 12/12 passed;
  the 180-tick authored-skin-versus-physical-rig fixture reported zero position
  difference across its default and baseline fixtures.
- Post-ADR160 `cargo test --workspace --locked`: **failed**, 454 lib tests
  passed, 3 failed, 1 ignored; Cargo stopped before other workspace targets.
  The original-contact and Hills recovery tests now pass. Remaining failures
  are high-speed biomechanical anchor/limit parity, standalone static-chunk
  trajectory parity (peak translation 1.611353 m, rotation 2.058868 rad), and
  predictive AI traffic clearance (minimum 1.680863 m versus required >2 m
  with a stopped car on a 6.5 m road). The traffic case also fails alone in a
  focused rerun. None of these thresholds was relaxed. Output:
  `/tmp/roaddash-bevy20-test-final.log` and
  `/tmp/roaddash-bevy20-traffic-targeted.log`.

## Native smoke

The first `target/debug/road-dash --smoke-test` started a native window with
isolated save/log paths, then failed during glTF scene instantiation because
Bevy 0.20's `WorldAsset` required types absent from the explicit city scene
registry. The 0.20 loader adds `GltfSceneName` and
`DynamicSkinnedMeshBounds`; these and the alternate `NoFrustumCulling` marker
are now registered. A temporary `bevy/debug` build named the second missing
type. Its Metropolis route-0 native smoke **passed**, and the production-feature
build and smoke **also passed** (`/tmp/roaddash-bevy20-debug-smoke2.log`,
`/tmp/roaddash-bevy20-production-smoke.log`), including audio output
initialization, two remounts and the farm/city crash-to-results fixture.
Both runs emitted a warning that more
than 65,535 distinct `VisibilityRange`s are in use and additional ranges may
share GPU slot 0, affecting culling or crossfades. Other maps/LOD still need
native checks. This smoke does not override the
failing ragdoll/recovery regression gates.

## Open verification

Workspace tests, debug/release feature matrix,
broader paired native anatomy/recovery and
shader/scene/audio/input/save checks, all map/LOD surveys, Windows CI/hardware and matched cold-load/warm-frame
Hills/Metropolis measurements must pass before integration. In particular,
Rapier's solver and sleep behavior changed across the upgrade. Upstream Bevy
renderer benchmarks cannot establish RoadDash's speedup or 16.7 ms target.

## Physics parity investigation (2026-11-11)

The exact field/pose/query port compiles and the twelve model/skin differential
tests pass, but that does not establish new-solver ragdoll parity. The first
full-suite `wall_impact_and_high_speed_ejection_remain_bounded` failure is a real
trajectory difference. The locked 0.17.3/Rapier 0.31 control's high-speed
biomechanical ejection reported peak anchor error 0.00392 m and peak joint-limit
excess 0.00940 rad; Bevy 0.20.1/Rapier 0.36.1 reported 0.16624 m and
0.19046 rad. Both were finite and all sixteen bodies were asleep at 15 s. The
static-wall case changed from 0.00168 m / 0.02005 rad to 0.00809 m /
0.04516 rad. The old control is recorded in
`docs/testing/guardrails/paired-lab.json` and the candidate in
`/tmp/roaddash_bevy20_test.log`. The first test fails its established 0.05 m
anchor and 0.10 rad limit gates; these gates stay open.

Rapier 0.31→0.36 changed default contact damping 5→10, added separate fixed
contact spring 60 Hz/10 instead of the old shared 30 Hz/5, linear slop
0.001→0.005, corrective speed 10→3 and predictive contact distance
0.002→0.02. An experiment using those exact old default values **only in
the headless lab fixture** did not restore parity: high-speed biomechanical
ejection was 0.13691 m / 0.43906 rad and the eleven-body baseline was
0.22279 m / 0.41503 rad. Static-wall biomechanical limit excess worsened to
0.22780 rad. The full eighteen-case experiment was finite, but it did not
pass the accepted limits. Its output is
`lab-old-defaults.json`; the experiment was reverted.
No production contact/joint/muscle configuration was changed. This remains a
merge blocker pending solver/CCD and visual feel investigation.

The reset test's 256→512 `World::entities().len()` jump measured entity ID
metadata capacity in Bevy 0.20, not live entities. It now uses documented
`count_spawned()` and its focused 100-cycle test passes. The old unconditional
foot-down negative control still never reached the road and stalled 322 ticks;
its traveled distance changed to 14.634 m. Its assertion now checks the actual
trap outcome rather than the former arbitrary 10 m cutoff. Neither adjustment
changes gameplay physics. The static-collider test still confirms contact, CCD
and ground ray support in both paths but fails 1 mm trajectory equality; its
strict assertion now reports peak transform/velocity divergence on the next
run. That divergence has not yet been measured.

The first focused original-contact recovery test passed for the normal condition
but **failed** when stamina was depleted. After 1,800 ticks, the runner
is about 46 m from the stationary wreck. The character controller requested
about 0.1 m of horizontal motion on the last tick but applied almost none;
its sole reported contact was the fixed ground-plane collider, not another
rider or the wreck. The test's human-controlled fixture has no rail or city
navigation data, so `walk_path` and `walk_goal` are empty; the runner still
requests direct motion toward the wreck. This isolates a Rapier 0.36
character-controller/ground-contact regression, rather than a mount-speed gate
or lane restriction. The narrow migration adapter now requests zero grounded
vertical motion and lets Rapier's existing `snap_to_ground` provide support;
airborne descent remains 5 m/s. The original-contact test passes for both
stamina conditions, as do three Hills physical return cases and the model
skin crash/run/remount test. The production capsule, collision, ragdoll and
mount rules remain unchanged. The old-engine core policy remains at 0.5 m/s
grounded adhesion; this adapter is specific to the new KCC behavior. Initial
failure: `/tmp/bevy20-original-contact3.log`; passing focused test:
`/tmp/bevy20-original-contact-ground0.log`.

The rebuilt native binary completed the Hills guardrail survey with two
physical foot crossings, 312 grounded ticks and four captures. Hills west
smoke passed in both the selected sixteen-body and baseline eleven-body rigs:
two 0.0000 m pickup errors, both recovery stages completed, no exceptional
rescue or penalty, audio output initialized and results reached. These runs
use isolated save/log directories and serialized native access. Receipts and
binary hash are in `kcc-native-receipts.json`. Both native smokes emitted the
existing warning about more than 65,535 distinct `VisibilityRange`s; culling
and crossfade correctness remain a separate open gate. This recovery evidence
does not resolve the high-speed ragdoll solver blocker or certify all maps.

The Hills opening width failure came from interpolating equal authored widths:
the newer math returned 6.0000005 m for two 6.0 m endpoints. Constant-width
intervals now return the exact authored value; the focused Hills test passes.

### Rapier CCD/solver isolation

A second lab-only experiment compared four modes on the same Bevy 0.20.1 /
bevy_rapier3d 0.37.0 / Rapier 0.36.1 binary. Values below are peak anchor error
(m) / joint-limit excess (rad) over 900 fixed ticks. The old-engine control
used Bevy 0.17.3 / bevy_rapier3d 0.32.0 / Rapier 0.31.0.

| Mode | High ejection, 16-body | High ejection, 11-body | Static wall, 16-body | Static wall, 11-body |
| --- | ---: | ---: | ---: | ---: |
| Old-engine control | 0.00392 / 0.00940 | 0.04606 / 0.21974 | 0.00168 / 0.02005 | 0.00884 / 0.05120 |
| New default | 0.16624 / 0.19046 | 0.06509 / 0.31238 | 0.00809 / 0.04516 | 0.03738 / 0.24252 |
| Map eight extra solver iterations to eight PGS iterations, lab only | 0.16958 / 0.19043 | 0.06509 / 0.31238 | 0.00116 / 0.08643 | 0.03738 / 0.24252 |
| Disable Rapier CCD globally, lab only | 0.00183 / 0.07552 | 0.06509 / 0.52398 | 0.00128 / 0.15955 | 0.01165 / 0.31297 |
| Remove biomechanical SoftCcd, lab only | 0.11284 / 0.53505 | 0.06509 / 0.31238 | 0.00136 / 0.16042 | 0.03738 / 0.24252 |

The 0.35 Rapier release changed `AdditionalSolverIterations` from extra solver
passes into whole solver substeps for the connected island. Its new
`AdditionalPgsIterations` is the closer API by name, but the measured mapping
did not restore high-speed parity. Rapier 0.35 also made fast dynamic bodies
automatically CCD-sweep fixed colliders even without the `Ccd` component. The
16-body rig already removes `Ccd` after initialization and retains
`SoftCcd { prediction: 0.35 }`; the eleven-body baseline keeps `Ccd`. Turning
CCD off at the context level isolated a large part of the high-speed change,
but it broke the static-wall limit gate and left the eleven-body baseline
without natural sleep at 15 s. Removing SoftCcd also broke limit behavior.
All four-mode lab runs were finite; none gives acceptable paired parity.

Rapier 0.36 exposes `max_ccd_substeps = 0` only for the whole context. Its
`Ccd` flag upgrades fast-body sweeps to dynamic/kinematic targets; setting it
false does not suppress the automatic fixed-collider sweep. The collision
filter hook receives the same collider-pair context for CCD and discrete
contacts, without a phase flag; filtering a pair would remove physical
contacts as well. There is no supported selective per-body fixed-CCD opt-out
in this adapter. Raising an internal body's `ccd_thickness` directly would
mutate backend state that Rapier derives from colliders and is not a safe
project API. No production CCD, joint, muscle, speed or rig setting changed.
The accepted high-speed parity gate remains a hard merge blocker.

Lab receipts: `lab-default.json`,
`lab-pgs.json`,
`lab-ccd-off.json`, and
`lab-soft-off.json`. Source for behavioral changes:
https://github.com/dimforge/rapier/blob/master/CHANGELOG.md (v0.35.0 and
v0.35.0-beta.0); installed crate sources for Rapier 0.31.0 and 0.36.1
were compared locally. These receipts measure headless physics only, not a
native player-feel or frame-rate result.
