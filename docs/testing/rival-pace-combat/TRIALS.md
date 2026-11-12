> Historical experiments, including rejected fixtures and superseded constants.
> Final behavior/results: [REPORT.md](REPORT.md). Intermediate raw native files
> remain in ignored local `output/testing/rival-pace-combat/intermediate-native/`;
> compact command outcomes are retained in `intermediate-receipts.json`.

# Intermediate rival-controller trials — 2026-11-11

Implementation branch: `codex/rival-pace-combat`, clean main baseline `19839421`.
The repository default is `main`; this is the requested update to master.
[Reference investigation](RESEARCH.md); architecture/tuning: ADR168.

## Corrections

Use full drive below the requested pace throughout the race, remove the
obsolete 155 km/h ceiling, and preserve the selected bike class, roster pace,
normal forces and every traffic/corner safety override. Fighters keep a moving
opponent briefly through an overtake and synchronize speed for a bounded
exchange, then resume racing. Attacks anticipate the existing authored contact
frame; actual hit validation, damage, animations, player controls, police,
accepted ragdolls, saves, recovery and world content are retained.

## Evidence in progress

Focused release physical tests passed: seven racer-AI scenarios, including
pair/chain flow, stationary-player passing and a real AI-generated chain hit.
At 20 seconds on the unobstructed physical straight, rival3 reaches49.2804 m/s
(177.41 km/h), versus player49.4822 m/s (178.14 km/h), across30/60/120 Hz.
The encounter produces one AI attack and a confirmed contact while both solid
bikes remain above38 m/s; no attack/contact events are injected by the test.
The pure filtered suite passed11 tests. Full checks/native comparison pending.

Baseline release SHA256 and build receipt are preserved locally under
`output/testing/rival-pace-combat/control/`. The first cold default-target build
was interrupted, then the cached0.20 target build succeeded. Subsequent commands
use `CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadDash-worktrees/issue-37-bevy-020/target`
and `CARGO_BUILD_JOBS=2`; explicit frozen binary paths isolate native comparisons.

## Limits

Short native surveys are sampled functional evidence, not full-race completion,
human difficulty approval or a matched performance benchmark. Baseline traces
did not expose attack serials, so attack-count improvement cannot be inferred
from decision snapshots. Two existing Bevy/Rapier parity failures remain an
engine-level issue; this work does not tune the rigs to hide them.

## First native candidate and user follow-up

The first candidate (SHA256 c5650d9a623a94619da9d064bdb53ea30ac8b0bd2597317b3fe9d06a4d56db20)
still includes police. Its Metropolis sample has no recorded rival attacks;
Hills records one attack by rider1. Neither is a complete race. The user
observed the stopping field in this run and requested disabling police. The
trace shows a fighter selecting officer15; police causation remains a hypothesis.
The Hills harness fails its error gate because the audio stream reports buffer
underruns/overruns during concurrent compilation; retain this failed receipt,
and repeat the final candidate without background builds. Do not call it a
clean native pass or infer GPU errors from its generic render_errors field.

The first synthetic single-file20m launch fixture reported1.63m lateral drift.
That is a compressed, accelerating queue with different rider masses, where
avoidance can be legitimate. The replacement uses both real alternating
Metropolis/Hills authoring grids on the physical straight; it retains a tight
lane/steering bound and checks actual acceleration/health/recovery. This does
not exempt the compressed queue or any physical contact from safety planning.

Latest changes: default police deployment off, opt-in via ROADDASH_POLICE=1
or --police-smoke; retry/HUD regression added. Opening planning holds the initial
lane and forecasts current speed+0.5m/s for5s while full drive remains available.
Real blocker velocities and all existing safety sweeps remain in force.

## Catch-up and fixture corrections

The first controller pass still requested catch-up speed behind the lower
cruise motor ceiling. An active local target now permits the bike's existing
full unboosted class ceiling; normal racing keeps the roster-dependent ceiling.
No torque boost or positional rubber band is added. A physical20m catch-up
fixture now closes the gap and confirms an AI-generated chain hit with both
bikes moving above47m/s. Ten focused racer-AI tests pass.

The initial separate deployment Resource perturbed Bevy entity ordering in the
post-remount visual differential test, and the patrol model factory fixture
assumed default deployment remained on. Move the flag into the already-existing
Police resource and explicitly opt the patrol factory fixture in. Preserve
all existing physics tolerances. Final model regressions are recorded separately.
The failed/interrupted trial logs remain local under output/testing; none count
as final passing evidence. Native confirmed_incidents reads existing combat
memory, which is updated only at confirmed contacts for ordinary race riders.


The corrected launch forecast is actual speed+0.5m/s for5s. The2m/s trial
held the wider Metropolis grid but still requested premature passes on Hills'
24m same-lane spacing; the0.5m/s forecast passes both real-grid fixtures with
lane drift below0.000001m and maximum steering0 over3s, with every rider above
20m/s and no injury/crash. It does not fake leader acceleration or disable
actual-speed clearance. The blocker admission test still requests avoidance.

After removing the separate resource allocation, all12 model tests pass in
release, including the strict full crash/run/remount differential and both
physical rigs'180-tick trajectory comparisons. Patrol factory verification now
explicitly enables deployment. No physical tolerances were changed.


## Native combat follow-up

The police-off/headroom candidate8e48761d passed both7200-frame native map
checks, both recovery smokes (two pickups each, zero pickup error and zero
exceptional rescues), and --police-smoke with ROADDASH_POLICE=0 (explicit smoke
flag correctly enables deployment). It nevertheless records only one Metro
attack and no confirmed rival contacts; Hills records none. These receipts
verify isolation/recovery, not adequate combat. Keep them as an intermediate
control; do not close the fight requirement on this evidence.

The trace shows close longitudinal encounters with5–8m of lateral separation
and old passing commitments. Correct the intent handoff: spend the4s fight
window only once physically aligned in lane and speed; match moving opponents
down to12m/s rather than forcing85percent of cruise; drop targets below8m/s;
release the old pass destination once when within8m, with all real safety
checks rerun. The physical20m catch-up test now also includes a5m lateral
alignment scenario. Final amended-candidate verification follows below.


The amended domain tests also reject initial targets across the whole avenue
and verify one lane handoff per selected moving opponent. A separate dynamic
fixture retains the officer's solid collider while choosing the nearby rival
as the combat target. Eleven focused physical-AI scenarios pass before the
final neighboring-lane acquisition adjustment; the final workspace run covers
that adjustment. The wide-lateral fixture confirms an actual hit after alignment,
with two authored attack attempts. Passing, stationary-human/chain clearance,
20m catch-up, healthy launch and default-police retry/HUD tests remain strict.


Final launch review corrected the preliminary clear-road path to include rider
proxies even while the bounded forecast is active. The previous zero-drift
trial skipped some rider checks and is not final launch acceptance. A new
physical stationary-rider fixture requires braking immediately and preserves
its solid collider. Real grids must stay calm through the first1.5s, retain
bounded steering/lateral travel over3s and accelerate every rider above20m/s
without a crash. Genuine Hills closure can request a pass around2.2s at21.24m/s;
forcing zero steering through that hazard was rejected. This keeps the actual
safety requirement, rather than weakening collision checks to satisfy a visual
zero-steering assertion.
