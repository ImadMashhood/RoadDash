# Part08 resident police development pass

2026-11-08 · `codex/part08-police-loop` · parent `5dcc791` · ADR086.
This is a bounded offline implementation; P08-006 verifies the narrow state-transition requirement; the other9 Part08 requirements retain their
remaining production/art/human/performance/exit acceptance. No Part09 work,
networking, new dependency, new proprietary media or new task IDs.

One resident motorcycle patrol starts at2.5% of the selected course, alongside
the unchanged15-racer roster. It uses the actual Rider vehicle, handling, control
intent, combat clock, contact/three-hit knockout and accepted physical recovery
path. It has no reinforcement spawning, catch-up teleport, hidden grip/speed /
acceleration multiplier or special launch energy. Police is explicitly excluded
from standings, gates/finish awards, profile records and the seven/seven rival
role census; retry preserves that identity. Legacy maps, Metropolis geometry /
facade pools, cars,48-prop-body cap and original rigs are retained.

`rr_core::police` owns Patrol/Warning/Pursue/Intercept/Search/Return/Arrested;
`rr_content::police` validates original `content/police.json`. Detection runs at
10Hz, with8s launch grace, a30m/s limit and90m solid LOS. Confirmed melee hits
must be visible at the actual hit point/time; windup, whiff and unseen earlier
contact do not become retroactive crimes. A1.5s continuous warning allows
compliance. Chase follows the last visible target, using projected live runner
positions on foot.3s without sight enters a5s last-known search; escape resumes
a forward patrol on the open course, without returning magically to a station.
The initial discretion targets the player; full rival violation selection is open.

The officer brakes through ordinary drive intent. A nearby on-foot target within24m explicitly requests a stop and deliberate foot pursuit, including a moving runner. A6m desired standoff for a
stopped target gives room to park; a moving target uses2.8m, with bounded.75/s
proportional closure and the ordinary vehicle top speed. These are prototype
RoadDash tuning, not original-game parity or human balance approval. Dismount
requires a slow, settled, immune-free and unstaggered bike; both sides are tried
with a capsule path sweep, destination overlap and support within.35m of the
bike's floor. Overhead roofs cannot attract the parked dismount. The ordinary
solid runner/navigation follows the last visible target, ignoring that target
only for local steering avoidance while preserving its actual KCC collision.
The officer stops within the authored arrest radius and returns to its actual
parked motorcycle after escape or a target remounting and riding away. Moving-foot capture/tackles are deferred: the officer walks at the same configured speed, and arrest still requires the target to slow to<=1.2m/s. A stationary-foot native fixture is not full running capture/fairness acceptance. No foot rescue or
offscreen crash teleport is granted to police.

Arrest requires2s continuous near-level supported LOS contact within1.8m,
<=1.2m/s target speed and **vector** relative speed. Conditional contact candidates
get fresh LOS/support validation on **every60Hz fixed tick**, independently of
10Hz detection. Falling/getup/mounting, stagger, unsupported separation, immunity,
wall obstruction, officer crash, pause/finish and retry interrupt dwell. A running
player gets.5s stage grace; stale mounted immunity cannot protect them forever
following another crash. Intentional police foot chase has a separate flag;
post-crash Running cannot gain arrest authority before normal recovery/remount.

Intentional-foot officers remain valid player melee/club targets at their actual
runner, with target-owned runner collider exclusion for contact LOS. Landed hits
reset arrest and add a bounded.25s police contact interruption. Third spaced
punch/club contact uses the existing repeat-runner crash path and actual walking
momentum; kicks interrupt arrest while retaining their existing separate hit
policy. No new punch impulse, joint/motor tuning or general foot-combat system.
A player can move away or remount to escape; ordinary controls, E/Q, timed club
contacts, frozen kick side, recovery, reverse and save format are unchanged.

Drive/gate finish and confirmed combat/knockoff resolve before police arrest.
PreUpdate already captures pause/restart input before the fixed loop. A committed
finish wins without duplicate payout; a committed arrest ends in BUSTED without
race award, profile race/win/best-time or credit writes. Enter/R retries through
the existing reset path, including stored dormant-chassis restoration. HUD shows
warning, chase, search countdown, safe arrest progress and explicit busted retry;
original generated sweep/bust audio respects mute, without a victory tune.

Draft presentation reuses existing project-original motorcycle/human palette6,
riding/attack/run clips and two blue/white shared cuboids. This is not finished
police uniform/vehicle/dismount/gesture art. No new mesh/material GPU resources;
there is one additional vehicle/visual rider and, during foot/crash states, the
normal runner/wreck/ragdoll resources. Those entities and conditional60Hz contact
rays have real traversal/physics/render cost. Detection is bounded to one officer,
10Hz; safe contact is at most one candidate, up to three rays per tick. Broader
performance and hardware acceptance remain open.

## Verification

See [RESULTS.md](RESULTS.md) for the final commands, native outcomes, measurements
and precise unrun gates. [RESEARCH.md](RESEARCH.md) pins the inspected source and
reference-only boundary. Source/body parity and recovery are regression tested;
scripted fixtures do not establish finished art, fun, legal clearance or60FPS.

The opt-in `--police-smoke` places actors through the normal authority reset helper
and uses actual W/Shift input. After warning/pursuit it creates a temporary solid
LOS barrier behind the player to deterministically test collision, loss, search
and escape. A real R retry cleans that scenario before a stopped crash fixture;
the accepted ragdoll/getup, deliberate officer dismount, live foot chase and safe
arrest run through normal schedules. Recent-remount immunity remains set into
Running, proving its bounded foot grace. Busted is captured, followed by actual
R input and a clean Countdown roster. Scenario placements/barrier/manual-foot
preference are fixture-only; this is not an unscripted human race or balance test.

## Per-task disposition

| ID | Disposition / remaining gate |
| --- | --- |
| V2-08-001 | Partial: pinned source study, validated original tuning, shared original draft vehicle/rider/marker/audio; bespoke uniform, vehicle, dismount and gesture art/approval remain open. |
| V2-08-002 | Partial: one resident noncompetitor, shared bounded driving and explicit pursuit/interception/search/return; broader discretion, traffic/obstacle handling, encounter balance and long-course acceptance remain open. |
| V2-08-003 | Partial: normal knockoffs, live foot chase/counterattack, safe arrest/no-payout/retry and escape; broader remount/trap/animation and human lifecycle acceptance remain open. |
| V2-08-004 | Partial: warning/grace, witnessed LOS offense, chase/search/arrest HUD, synthesized cue and safety tests; human fairness, physical-controller/accessibility and final audio/art review remain open. |
| V2-08-005 | Partial: deterministic integration and native scripted regression receipts; complete offline human race/soak,16.7ms budget and multiple-hardware gates remain open. |
| V2-08-006 | Open: user exit acceptance and original Part09/10 gates have not been granted or started. |
| RIGHTS-014 | Partial: police source/provenance/cue integration; full sessions23–24 and town/farm/cliff soundscape remain open. |
| P08-004 | Partial: police pursuit/arrest states coexist with existing weighted rival AI; a complete shared racer blackboard/state contract remains open. |
| P08-006 | Verified in Part00: resident patrol, witnessed violation, warning, pursuit/interception, safe bust, loss/search and return transitions with core/integration/native evidence. This closes the state requirement only; full art/discretion/moving-capture/human/performance gates remain under the other9 IDs. |
| P08-009 | Open: no fair/repeatable human four-racer traffic/police/melee exit acceptance claimed;15-racer roster is preserved. |
