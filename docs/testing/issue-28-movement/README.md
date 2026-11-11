# Issue #28 — recovery running and reverse steering

ADR143; RFU-MOVE-01..04. Base754f9ddb, branch codex/issue-28-movement.

The existing controller requests6m/s; the old run blend already reached full
weight at4.5m/s. Keep that policy. The new full run has a2.5m cycle,1m contact
sweep and32cm swing clearance, with opposing arm drive/trunk lean. Phase still
advances only from measured horizontal displacement. Terrain swing probes
complement existing support-local stance locks and visual pelvis reach fitting.
The animation path changes no authoritative body transform, collider, mass, muscle/joint, recovery policy,
input buffer, sound schedule, model bind or asset.

Dynamic heading reverses with signed longitudinal speed; tire-side demand and
forward response stay intact. Local+X is rider-left (forward is+Z, camera-right
is-X): D while backing yields positive local heading X. Assisted remains
road-constrained, using a reverse-only heading offset bounded by0.28rad and
fading to zero with speed. AI escape's explicit target heading is preserved.

Swing terrain queries run inside the existing60Hz rider animator, never per
render frame. Zero for mounted riders, getting-up or civilian pedestrians; at
most2extra rays per running rider/tick (32if all15racers+officer run at once).
No extra skeleton evaluation. Runtime query cost remains unmeasured pending
a coordinated performance window; allocation/query bounds are not timing.

Read installed chrisgliddon/bevy-skills (MIT) bevy, bevy-animation and
bevy-input-actions; gamedev-skills/awesome-gamedev-agent-skills (Apache-2.0)
game-feel/input-systems. Use engine-neutral presentation/input separation.
Pinned local Cargo.lock Bevy0.17.3/Rapier0.32.0 APIs remain unchanged;0.20 migration
is a separate authorized branch. No reference code/assets imported.

RFU-MOVE-03/04 are verified. RFU-MOVE-01/02 retain human motion/terrain approval; broad performance and
Windows/hardware acceptance are not inferred from scripted fixtures.

Native terrain review: set ROADDASH_GAIT_REVIEW=1 and ROADDASH_GAIT_TERRAIN to
flat, up, down, cross, uneven; run road-dash --animation-lab-view
--animation-lab-smoke in separate output directories. The fixture uses the
real skin/animator and Rapier supports but scripted translation, not manual input.
Real recovery controls require separate --smoke-test cases, both rigs, with
ROADDASH_VERIFY_WALK_CROSSINGS=1 (see tools/verify_road_walkability.py).

Native reverse receipt: ROADDASH_REVERSE_REVIEW=1 road-dash --smoke-test;
repeat with ROADDASH_VEHICLE=assisted. Captures real keyboard events through
the normal fixed input path, both A/D and reverse-to-forward transition.
Add ROADDASH_VERIFY_MANUAL_RETURN=1 to each actual walk-crossing smoke to
exercise manual interruption followed by automatic return and remount.

Initial animation-only suite:22passed. Audit passed after updating the changed
original vehicle source hash. See final integrated checks below.

## Headless checkpoint

Latest workspace run:596passed,0failed,1existing ignored600-second turning soak.
Strict all-target Clippy, format, audit and5roadmap tests pass. All12model tests
include exact physical trajectory and full crash/run/remount parity. Native
fixtures compile under Clippy; required executable build passed.

The eight translated terrain fixtures measure at most0.688mm established
planted-sole displacement per fixed tick (see terrain-contact.txt; tuple columns
are speed, longitudinal grade, cross grade, uneven flag, maximum displacement,
samples). They assert unchanged physical roots and less than35mm sole-point
penetration; they do not certify whole-boot mesh clearance or user visual feel.

Use tools/verify_movement.py --binary <absolute executable> --output <fresh dir>
with optional --only gait/reverse/recovery/lab for coordinated native windows.
The earlier headless checkpoint preceded the final native matrix below. Runtime query timing remains unmeasured.

Historical first frozen executable (ignored local artifact):
output/testing/issue-28-frozen/road-dash. SHA256 is recorded in headless.json.
The final native matrix below supersedes this earlier checkpoint. Runtime
grounding-query cost and broader motion acceptance remain open.

## First native checkpoint and harness correction

The first frozen build completed the flat sequence with149frames. Uphill exited0
but the strict log check rejected an ERROR caused by screenshot sending after
shutdown. Preserve that failed receipt under output/testing/issue-28-native.
The corrected harness stops captures at750, drains through at least800frames
and waits for Bevy0.17.3 Screenshot entities to finish before normal exit.
This changes only the opt-in review lifecycle; production movement is unchanged.

## Final native matrix and integration

All ten cases pass (native-receipt.json): five terrains with150captures each,
both steering adapters with96samples and actual signed-heading assertions,
both recovery rigs with manual interruption/automatic return/remount and two
walking zones, and18finite paired lab scenarios. Exit codes and logs contain no
ERROR/panic/shader failure. Frozen binary SHA256:
`bd918694615ec10390238780d2123926f2d5dfbcb33f53313f613793a5c038c4`.

Reviewed flat/up/down/cross/uneven native captures, reverse adapter views and
manual/remount captures. Bent opposing arms and lifted swing knees read as a run;
no obvious sole penetration in selected terrain views. Existing articulated
knee seams and full human motion/terrain approval remain open. Screenshots alone
do not prove heading sign; the native fixed-step assertions check A/D and the
reverse-to-forward transition. Raw sequences and initial failure are retained
in ignored output/testing/issue-28-native[-final]. No timing/hardware claim.

Integrated performance main76d4fe15, preserving its measured defaults and bounded
natural-sleep tests. Native evidence above predates that integration; production
movement code is unchanged. Latest integrated strict checks recorded separately.

Compact native previews (source hashes in capture-provenance.json):

- [gait-flat-gait-0456.jpg](gait-flat-gait-0456.jpg)
- [gait-up-gait-0456.jpg](gait-up-gait-0456.jpg)
- [gait-down-gait-0456.jpg](gait-down-gait-0456.jpg)
- [gait-cross-gait-0456.jpg](gait-cross-gait-0456.jpg)
- [gait-uneven-gait-0456.jpg](gait-uneven-gait-0456.jpg)
- [reverse-dynamic-reverse-0294.jpg](reverse-dynamic-reverse-0294.jpg)
- [reverse-assisted-reverse-0294.jpg](reverse-assisted-reverse-0294.jpg)
- [recovery-anatomy-manual-return-interrupt.jpg](recovery-anatomy-manual-return-interrupt.jpg)
- [recovery-anatomy-06-remounting.jpg](recovery-anatomy-06-remounting.jpg)

Actual native samples (reverse-samples.json): Dynamic D/A heading X is
+0.3085/-0.3086 at about-2.505m/s. Assisted D/A is+0.0812/-0.0354
at-2.5m/s; it is road-constrained and returns to near-zero local heading
when moving forward. Input demand is gradual; fixture amplitudes are not a
claim of identical trajectories. Game-side checks enforce direction and bounded
angular transitions; headless tests cover symmetric rules/unchanged forward.

The capture-drain query tuple became a named SystemParam to satisfy strict
Clippy; query types, filters and scheduling are identical to the native fixture.

## Final strict checks on integrated source

600workspace tests pass (426game,1preview,57content,116core); one existing
explicit600-second turning soak stays ignored. Format, strict all-target Clippy,
locked executable build, provenance audit, five roadmap tests and synchronized
progress check pass. All12models tests retain trajectory/recovery parity.
Checks ran at nice15/jobs2/testthreads2; no elapsed-time performance conclusions.

## Civilian integration (main7a63ef18)

Merged civilian overtaking after its615-test and selected/native fleet checks.
Production movement and the civilian planner share game::drive; both changes
are retained. Explicit smoke dispatch preserves reverse-review and civilian-survey
entry points. Removed inherited merge markers in PROGRESS while retaining both
scope summaries. The combined suite covers movement signs/zero crossings, exact
model parity, terrain contacts and all civilian pass/obstruction/endpoint tests.
Paired native recovery is repeated against the combined executable because the
planner and running controls share a fixed update; the unchanged isolated gait
fixtures are retained from the ten-case native matrix.

Combined source passes617tests (433game,1preview,57content,126core), with the
existing600-second vehicle soak and isolated civilian CPU benchmark ignored.
Strict all-target Clippy, fmt, locked build, audit and five roadmap tests pass.
No tracked conflict markers remain. Native paired recovery receipt follows.

Combined native recovery passes2/2 (combined-native-receipt.json), both rigs,
manual interruption/automatic return and remount, with two walking zones and
no logged ERROR/panic/shader failure. Distances match the earlier fixtures.
Executable SHA256:68fd87eb326895e3b4f691d5d3196d3c6fda9db475b4878a93fb99bcb3b081a3.
Reviewed the combined anatomy remount capture; no new pose discontinuity visible.
Earlier five-terrain/two-reverse/18-case lab receipts remain valid because their
production paths were preserved. User motion/terrain acceptance remains open.
