# Part 02 and requested gameplay fixes — execution receipt

2026-10-30. Branch `dev/part-02-scenic-route`, based on main merge 475a484 (foundation commit 368da38).
The user's later correction explicitly replaces a proposed loop with a
**point-to-point farmland → city → hills course**. The sketch supplies the concept;
no attachment image or external game media is distributed in the runtime.

## Implementation and automated checks

Valley Run is 6,847.3 m, open, with nine start/ordered/finish gates, eight spawns,
width/bank/surface/sector/prop metadata and elevation from -2.49 to 110.12 m.
The independent handling fixture is 1,979.0 m; its closed geometry is retained for
legacy regression testing, not used as the default race. Authoring contract:
[track guide](../tracks/AUTHORING.md).

[Command output and exit codes](part-02/local-checks.txt):

| Check | Actual result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | Passed |
| `cargo test --workspace --locked` | **50 passed**: 41 client, 6 content, 3 core; zero failures |
| `cargo build --locked` | Passed |
| `cargo build --release --locked` | Passed |
| Minimal offline / debug_physics feature checks | Passed |
| Headless CLI validation of both tracks | Passed, exit 0 |
| Python audit parser/UTF-8 regression and provenance audit | Passed: 510 dependency declarations, 7 runtime source/media records, 23 documentary observations |
| README/tracker synchronization, `git diff --check` | Passed |

[Track process probes](part-02/track-probes.txt) record both valid imports and
exit-2 rejection of negative width, swapped gates, missing point-to-point finish
and overlapping spawns. Typed/unit validation additionally rejects nonfinite
coordinates, self-crossings, duplicate points and unsafe geometry. Original save,
controls, combat, traffic, audio, camera and recovery regressions remain passing.
Earlier schema-1 bike catalogs receive default steering rates without losing tuning.

Key acceptance evidence:

- Both boost-release and fuel-exhaustion scenarios retain >74 m/s on the first
  60 Hz tick from 75 m/s instead of snapping to 55 m/s.
- Four player/AI grass scenarios advance at least 80 m in ten simulated seconds
  without grass-only damage/knockoff. AI moves back into the road corridor.
- Actual crash-generated wreck slides down a solid 30° ramp; support-normal
  sampling retains normal stopping friction on flat ground.
- Anatomical hinge locks/one-sided stops and bounded passive spring motors are
  checked. **100 high-speed drops** run one simulated second each, checking finite
  transforms/velocities, no ground clipping, <0.3 m anchor separation and hinge
  X-axis alignment >0.95. This is not a full anatomical or all-geometry certificate.
- A fixed-controller autopilot drives the entire scenic route without progress
  teleports, crosses every physical gate and awards exactly one result. No traffic
  or scenery obstacles are in that rule/controller harness; separate tests cover
  collision and native gameplay. Pacing/fun remains a playtest requirement.
- Pure gate tests reject reverse, out-of-verge and finish teleports. A skipped
  gate prevents payout and stops at the open finish; retry clears gate state.
- Independent road ribbons have upward winding and fewer collision triangles.
  **240 physics rays** (80 positions × three lanes) must hit a RoadChunk first,
  proving terrain is not covering the sampled banked carriageway. World tests
  verify solid props/clearance and populated farm/city gameplay actors.
- Existing exact 30/60/120 render-rate equality, buffered inputs, twelve fallen
  bike remount approaches and 100 pause/reset cycles continue to pass.

## Native graphics/audio and elevated recovery

Command: `./target/release/road-dash --smoke-test`, actual Fedora/NVIDIA RTX 3080
Vulkan window at 1440×900. Final run: 1,237 render frames / 1,236 fixed ticks
(1,136 active), 6,102 entities / 1,895 colliders; bounded wall average 6.95 ms,
P95 14.50 ms; last gameplay 0.136 ms / physics 0.603 ms. Geometry totals include
terrain: 144,620 render triangles / 133,540 collision triangles. [Native output](part-02/native-smoke.txt) and
[measured diagnostics](part-02/native-metrics.json) are retained. Four audio sinks
initialize and active racing playback is verified. Synthesized PCM is also tested
for non-silent valid audio; no subjective mix/listening approval is claimed.

The script rides/crashes in farmland, follows real get-up/run/remount, then visits
city/hills for visual review, enables track gizmos, triggers a second real crash
at elevated terrain and waits for another real get-up/run/remount before results.
The zone visits and final results setup are explicit presentation teleports;
they do not prove gate validity. The separate full-course fixed-controller test
above provides the non-teleport race/gate evidence. Profile writes are disabled.
The script uses fixed manual time for repeatable captures, so wall-clock numbers
are not real-time pacing, GPU profiling or production performance certification.

Reviewed captures:

- [Farmland](part-02/farms.jpg): open grass, seeded farm/fence dressing, road and HUD.
- [Farm crash](part-02/farm-crash.jpg): articulated rider detached from a riderless wreck.
- [City](part-02/city.jpg): buildings line both sides, sidewalks/pedestrians and road corridor.
- [Hills](part-02/hills.jpg): elevated road and physical roadside guardrails.
- [Hill crash](part-02/hill-crash.jpg): separate rider/wreck at elevation.
- [Hill run back](part-02/hill-running.jpg): capsule/camera follows actual 3D recovery.
- [Track inspector](part-02/inspector.jpg): spline, widths, normals, bounds, gates and spawns.

Raw PNGs for garage, both crash/recovery sequences and results remain locally in
ignored screenshots/. The native JSON includes remaining 3D walk distance and
absolute height difference at the run captures. These are one scenario's samples,
not a general claim that every hill recovery is longer than every flat crash.
Nonfatal Vulkan extension/suboptimal-present warnings do not prevent completion.

## Corrections found and remaining scope

Visual review found grass intruding on banked asphalt: terrain now includes bank
and clearance, verified by first-hit road rays. Always-low wreck friction caused
excessive flat-road sliding: low friction now depends on steep support normals.
An initial full-course test harness had a zero fixed delta; it now explicitly
advances its clock. Open endpoints needed explicit movement/traffic behavior and
missed-gate retry messaging; these are implemented and tested.

Completed Part 02 checklist items: 003, 005–009 and additions 012–015 (001–002
retain their earlier test-fixture evidence). P02-004 is partial only for optional
branch/split/merge authoring. P02-010 has both road lengths but representative
approximately three-minute busy-race pacing remains unmeasured. The overall
P02-011 gate stays open until these requirements are resolved. No later major
part is automatically complete.

Current limitations: original graybox art; static cows, simple bounded pedestrian/
cross-car patrols; kinematic assisted bike, passive tissue approximation rather
than biological muscles, manual walking/foot finish absent, arbitrary enclosed
recovery traps unverified, missing a gate requires retry, aggregate legacy best
record rather than track/class records, Windows hardware unverified. Native smoke
and automated drop/ray tests do not replace external playtests, a busy-race soak,
all possible terrain/traffic traps, clean installs or vulnerability/legal signoff.

## Prior remote CI failure and integration base

Foundation PR #1 was merged into main as 475a484 while this task was active;
its tree equals 368da38. This branch was advanced to that merge base without
changing the workspace or unrelated history. The prior PR workflow run
[41136088754](https://github.com/ImadMashhood/RoadDash/actions/runs/41136088754)
finished: Ubuntu job passed all steps; Windows formatting, Clippy, 33 tests,
build and both feature checks passed, but audit.py failed at Cargo metadata
stdout decoding (`UnicodeDecodeError` in cp1252). This is not a Windows gameplay
or engine compile failure. The audit now explicitly decodes UTF-8, and repository
text readers specify UTF-8. A mocked Windows stdout/code-page regression with a
non-ASCII author verifies the failure mechanism; both Python tests pass locally.
A new remote run must confirm the complete Windows audit; do not substitute the
mocked encoding test for a Windows hardware or full new-matrix acceptance receipt.
