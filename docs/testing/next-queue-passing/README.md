# Queued rider passing follow-up — 2026-11-09

Scope: existing CAST-AI-01 / P08-005, no new task ID or requirement closure.
Parent integration owns README, PROGRESS, DECISIONS and provenance indexes.

## Diagnosis and implementation

The original planner predicted every manoeuvre at at least 8m/s, including a
stationary bike behind a stopped car. Close queues therefore had no reachable
passing candidate. Matched-speed followers also considered their current lane
clear until acceleration created closing speed, causing repeated braking.

Original project code now checks current and intended speed, selects a bounded
4/2/1m/s manoeuvre when the bike is slow enough, and commits to its passing side
until the original queue clears. Low-speed lateral predictions scale with speed;
no lateral movement or physical transform is directly applied. All acceleration,
braking and steering still use the existing fixed-tick vehicle adapter.

Each tick rechecks cars, crossing motion and upright riders, including rear and
oncoming threats, swept lane-change clearance and destination occupancy. New
hazards may invalidate the commitment. Rejoining also uses the clearance checks.
The usable outbound road envelope is intersected across the next 120m (the same
look-ahead used by streets::shares_lane_ahead), preventing a pass into a dropping
lane or the inbound/sidewalk space. Curved-road segmented physics sweeps reject
static obstructions. Existing bounded escape/recovery remains available when no
physical passing route exists. Player input/physics, cast, saves, collision and
ragdoll settings are unchanged.

## Acceptance and verification

- Standalone std-only compilation of traffic_avoidance.rs and all 14 module tests
  passed. Covers stopped car + free gap, matched-speed queue, occupied/rear/oncoming
  gaps, stable commitment and safe return, new hazards, narrow/drop constraints,
  solid-corridor rejection and the previous crossing/alongside regressions.
- Physical test added: queued_ai_physically_passes_a_stopped_car_without_escape_or_recovery.
  Drives a real dynamic bike around a fixed collidable car, requires actual forward
  passage and lateral motion inside road bounds, and rejects recovery, teleporting
  and use of exceptional escape planning. Passed through the normal dynamic
  vehicle adapter (1 test, 0.20s). Command:
  `CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo test --locked queued_ai_physically_passes_a_stopped_car_without_escape_or_recovery -- --nocapture`.
  Initial fixture setup incorrectly queried Velocity before vehicle setup attached
  it and failed before simulation; corrected to use the normal at-rest spawn lifecycle.
- `cargo fmt --all --check` and `git diff --check` passed.
- Full workspace clippy/tests/build/audit, paired recovery/NPC/baseline regressions
  and native race review are parent integration gates; not claimed here.

## Research, decisions and provenance

Read project engineering instructions, live checklist/progress/decisions/issues,
frozen Part 08 blueprint, installed Cargo.toml/Cargo.lock, traffic_avoidance,
streets, pursuit, cast, game, vehicle and physical fixtures. Builds on d6547ec2
(city carriageway correction). No external repository, code, media, asset or
package was imported; no dependency or license change. Parent should record this
original-code follow-up against existing CAST-AI-01 and P08-005 and add the evidence
link to its chronological history/provenance entry; keep acceptance open.

The selected model is conservative gap planning with bounded speed staging, not
teleportation, traffic removal, collision exemptions or a new vehicle controller.
Repeated live checks do not guarantee safety against arbitrary unexpected motion;
sealed/too-narrow queues can still wait. Full-course fairness/deadlock freedom,
performance (including additional static sweeps), hardware and user feel review
remain open. Pure domain tests do not establish native performance or legal clearance.

Final source review adds current-speed staging for moving queues: at matching
8m/s a rider now moves into a clear adjacent lane before accelerating, instead
of waiting for the queue to stop. A reduced-speed candidate must lead to a lane
clear at cruise, so merely following the same blocked lane cannot consume the
passing decision. The tightened matched-speed regression and all 14 standalone
module tests pass. Physical fixture passed before this final core refinement;
parent workspace/physical rerun is required on the final combined source.
