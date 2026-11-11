# Racer decisions and passing — issue 26

Branch: codex/issue-26-racer-ai. Requirement IDs RFU-AI-01–08. ADR141.

## Investigation

The previous `Mind::choose` minimizes absolute longitudinal distance, so a leading
killer can choose a follower behind despite another rider ahead. Shared
`pursuit::approach` requests `target.speed + gap * 0.3`: a rear gap reduces desired
speed. The native adapter additionally floors upright combat speed at personality
minimum, so this alone is not proof that a pair stops. Traffic avoidance can impose
a further zero speed; its rider-queue trigger used the already combat-reduced pace.
Existing outbound-only bounds also prohibit the requested opposite-lane passes.
These are distinct mechanisms; no blanket diagnosis from a still screenshot.

## Implemented policy

Pure rules live in rr_core. Closest eligible forward mounted rider wins over rear
opportunities. Eligibility excludes self, downed/finished riders and different
route corridors. Exact station ties use racer ID, avoiding reciprocal forward
ordering. Target hysteresis retains a current forward target within 0.75 m of the
nearest. Road-space Euclidean station/lateral distance enters combat at 10 m,
leaves at 12 m; the side is held per target. Beyond that range killers pursue.
A leader may position beside a rear rider within 10 m but retains race pace.
Slow targets enter passing rather than standing combat (5/7 m/s hysteresis).
Exact-attacker retaliation and launch grace remain. Downed riders no longer become
intentional mounted hunt targets, matching the requested racing/moving-combat scope.

Priority: resident collision/curve safety, committed reachable passing, moving
combat, race pace. Rider-queue detection uses race pace independently of combat
speed matching. Outbound passes are preferred. Opposite road is considered only
when outbound cannot progress and both the pass and return forecast are clear.
Return must begin within eight seconds with four seconds reserved to merge; all
opposing/rear/alongside proxies are checked. Bounds cover 720 m of current road,
proxies cover 1500 m; resident static sweeps follow changing street centres.
Memory never reserves a lane or grants collision exemptions. Sealed routes yield.

Native `--racer-ai-survey` runs an unteleported production race with slot zero
using the same AI, normal combat/traffic/recovery, snapshots every 180 frames and
periodic screenshots. It records target/intent, combat speed, final avoidance plan,
controls, passing and recovery. A timeout produces an honest incomplete receipt.
It does not claim a human playtest or guarantee every rider finishes.

## Skills, research and provenance

Read the local game-ai, ai-behavior-trees-utility-ai and performance-optimization
skills. Applied decide/steer separation, shallow state selection, hysteresis,
rechecking safety while committed, and measurement boundaries. A generic behavior
tree runtime is unnecessary for this small policy; no framework or dependency was
added. Inspected existing ADR120/121/126/132/139, prior queue evidence, project
blueprints, game/rr_core source and Cargo.lock (Bevy 0.17.3/Rapier 0.32.0).
No external repository code or media was imported; no new asset/license claims.
Engine migration is separately coordinated by the parent task.

## Verification status

The initial independent core suite passed 122 tests. Production Dynamic
killer/racer, killer/killer, four-killer chain and actual opposite-lane pass/return
regressions passed. The first full game run had 421 passes and two outdated-policy
assertions; no collision assertions failed. The sealed-road control now includes
a real opposite-lane blocker. An independent physical case proves passage and
return when open. The three predictive traffic controls retain 100 health and
more than 2 m clearance (crossing minimum 3.14 m; sealed queue pair minimum
4.57 m). Initial failures remain in initial-tests.txt and avoidance.txt.

The published implementation passed 604 workspace tests. A subsequent harness
correction detects slot-zero's recorded finish without requiring the offline
Results phase; its regression raises the count to **605** (424 game library,
1 binary, 57 content, 123 core), with one existing long soak ignored. The final-*
logs record the clean-workspace-artifact build. Format, strict all-target Clippy,
locked debug/release builds, declaration/hash audit and five roadmap checks pass.

### Native evidence before the crossing refinement

The frozen corrected binary SHA-256 is
`ef59ea667831689674c72ca753a091649db6777fe5e8100ff5c5bedbf37c8a89`.
The unteleported extended production race completed at **26,299 frames / 435.238
simulation seconds**, with slot zero and riders 10, 13, 8, 14 and 2 finished.
It exited successfully without render errors. Both selected velocity-muscle and
`--ragdoll-baseline` native smoke runs passed with two remounts each. Receipts are
in native-race-extended-01, native-recovery-01 and native-baseline-01.
The earlier 21,600-frame run was incomplete for slot zero; that receipt is retained.

The completed race does not prove every queue is fixed: rider 11's longest sampled
avoidance stop was 18 seconds, and some riders did not finish before slot zero.
Slot zero stopped near station 11,152 around 411–414 seconds, then resumed via a
passing maneuver around 423 seconds. Its periodic trace logs raw control amounts without the held flags; a positive
amount does not mean applied throttle (`throttle_strength()` returns zero when
the held flag is false). New diagnostics record effective strengths and escape
state. The original trace also lacks obstacle proxies,
so the cause cannot be reconstructed from that receipt alone. Sample intervals
are about three seconds; stop durations are sampled estimates, not continuous
worst-case bounds. Wall times overlap other branches' bounded builds and establish
functional completion only, not isolated frame performance. Human balance and
broad race/hardware coverage remain separate gates.

### Crossing review and focused follow-up

Review found a definite overbroad rule: any laterally moving proxy within the
longitudinal range vetoed an opposite-road pass, including traffic on parallel
roads or already clear of the rider's arrival. The refinement uses the existing
continuous collision envelope to yield only when the home corridor intersects
crossing traffic at the same time, checking both current-speed arrival and candidate pace. The crossing refinement passed 608 workspace tests, strict Clippy, both builds
and audits. Its focused traffic rules include
unrelated/already-cleared/diverging crossings and an actual crossing conflict.

The opt-in `--segment` runner starts a **synthetic diagnostic** near station 11,090
at race time 408 seconds and enables 0.5-second planner proxy/static-hit traces.
It preserves normal collision, controller and recovery behavior. It is not an
exact replay of the earlier race: obstacle histories were not recorded there.
The first segment run traversed station 11,152 without stopping (29.05 m/s near
11,142; 27.07 m/s near 11,157), with no static-path rejection. Its momentary zero
plan had a nearby outbound car and an oncoming envelope; this does not establish
the original race's cause. The synthetic actor continued past the 11,500 m race
finish because it had skipped earlier gates, then stopped around 11,562 m. That
post-finish behavior is outside the target segment. The harness now ends at the
race finish with a separate `segment_passed` result and requires that result.
Evidence folders are protected against reuse, and `--require-complete` makes an
incomplete full-race receipt fail explicitly. No player physics, rig, save, control
or audio changes are intended.

## Stationary-player and compressed-queue regression

Adding an explicit solid human slot-zero fixture exposed a narrow-road failure
not covered by the earlier crawling-rider case. A racer passed, but its killer
followers stopped about 4.6 m from the human. That stop consumed the room required
by the existing steering delay and collision envelope, leaving no feasible lane
change. The fallback now reserves 7 m of additional launch room behind slow
leaders when no pass is currently available. The approaching three-rider chain
then passed with no reverse pulses and only 63 stopped ticks for its last rider.

An already-compressed queue needed a distinct bounded reposition. General A*
escape left the road in the failing fixture, so compressed queues use the existing
rear-swept reverse control instead. Proxies retain actual signed forward velocity;
a 0.1 m/s perception deadband prevents resting solver drift from becoming oncoming
traffic. Boundary tests distinguish ±0.099 from meaningful ±0.1/2.5 m/s motion.
Escape progress is re-anchored after deliberate backing.

The first 1.25-second pulse implementation required four pulses per rider and was
still incomplete after 38 seconds. At the unchanged 2 m/s² reverse acceleration,
a resting 1.25-second pulse covers only about 1.56 m, before any early rear-sweep
stop. The queue-specific policy instead retains a live launch-space goal for up
to 4.5 seconds, stopping immediately for a restored gap or blocked rear sweep.
Ordinary unsticking retains its 1.25-second pulse; the 6-second cooldown, reverse
speed cap, collision shapes and vehicle physics remain unchanged.

The same three-rider compressed fixture now completes in **1,369 fixed ticks /
22.82 simulation seconds**, using **one reverse pulse per rider**. Longest idle
intervals are 281/223/165 ticks (4.68/3.72/2.75 seconds); minimum player clearance
is 2.92 m. All actors retain 100 health and Riding recovery state and remain on the
road. The regression now requires completion past the player by 30 m within
30 seconds, less than 5 seconds continuously stopped, and at most two reverse
pulses per rider. Ordinary approach fixtures require zero reverse pulses. The
wide-road three-rider case has 1.92 m minimum clearance; the narrow approach
case has 2.97 m. The existing bike clearance assertion is 1.2 m and car cases
retain their larger clearance requirements. Earlier failures remain in
refinement-checks, alongside the superseding compressed-reverse-goal.txt receipt.

All 130 core tests pass after the goal policy. These latest queue changes still
require the final combined workspace/native gates after integrating the civilian
passing branch and its bounded lateral-motion forecast. No blanket elimination
of every possible traffic blockage or human aggression approval is claimed.

## Rule cost (release, 10,000 iterations per case)

The explicit `rr_core` example compares the previous outbound-only primitive to
its new arbitration wrapper on the same96-proxy fixtures. Clear road measured
1.403 versus1.387 microseconds/plan (noise, no speedup claim). Blocked outbound
measured0.479 versus5.583us; an opposing blocker0.506 versus7.597us. The wrapper
adds approximately5–7us in these fallback cases. Static callback counts rose
from1 to2; those callbacks return true in this benchmark, so their native physics
cost is deliberately excluded. See rules-cost.jsonl and reproducible example.

Native integration still scans up to1500m of bounded fleet proxies; only an
opposite-road fallback evaluates the73 road-width samples. Normal static sweeps
remain12 segments, opposite-road candidates24. This is bounded added work, not
proof of a frame budget. Functional native runs overlap another branch's builds;
no FPS result from them is offered as an isolated performance measurement.

Console evidence retains commands/results; trailing whitespace in text receipts is normalized.

### Civilian integration checkpoint

Integrated issue25 main76d4fe and issue27 main7a63ef, then issue28's published
runtime head a4669713 so compressed-queue tests include signed reverse steering.
The racer adapter caches each civilian's previous-step lateral displacement and
committed lane endpoint once per fixed tick. The pure forecast checks the
transition endpoint kink and retains settled occupancy through the return leg;
it does not extrapolate a three-second lane change for twelve seconds. The
132-rule-test receipt predates the added issue27/28 rules and is not the final
combined workspace count.

The first paired interaction fixture used the legacy simulation map, which
intentionally bypasses civilian overtaking. Both attempts recorded zero passing
blend and failed the required actual-maneuver assertion; they are retained as
fixture diagnostics, not production failures or passing acceptance. The corrected
fixture uses Metropolis traffic lanes and a solid sampler-derived road surface.
Final interaction, combined workspace and native checks are pending below.

The corrected Metropolis paired fixture passes: civilian blend reaches1.0,
actual cached lateral motion is observed, and the racer enters avoidance while
preserving health100, Riding recovery state, road bounds and solid car colliders.
Minimum centre clearance is2.632m (required>2m); the racer reaches station1650.26
at40.45m/s. All four racer physical tests pass together on the25+27+published28
runtime. The compressed stationary-human chain still clears in1369ticks/22.82s,
one reverse pulse each, longest stops281/223/165ticks and minimum2.921m. Strict
all-target Clippy and format pass; full workspace/build/native gates follow.

### User lane-choice playtest follow-up (2026-11-10)

The additional report that riders always choose the correct right-hand lane is
supported by two source rules: `src/game.rs` gave free racers a fixed outbound
roaming target; `plan_with_oncoming` only evaluated the full-road corridor when
an outbound plan was stalled. The first rule now varies a racer's requested line
across the paved width. The second admits an intentional opposite-side line only
when the existing return forecast and resident static sweep are clear. Civilian
traffic uses a separate policy. Committed passes still return to their home line.
Seven focused `rr_core` inbound tests pass, including a new clear-road lane choice
and approaching-car veto. The physical Metropolis rider regression passes with
an Assisted bike entering the opposite paved lane. The lane branch and its merge
with the running-hand main each pass **641 locked workspace tests** (439 game
library, 1 preview, 57 content, 144 core; two ignored). The merged tree also
passes `cargo fmt --all --check`, strict workspace Clippy, `cargo build --locked`,
five roadmap tests, progress sync and the provenance audit. Combined native
evidence and quiet per-frame cost remain pending; no visual or performance
acceptance is claimed from the rule tests alone.

The first combined workspace run after that lane change exposed three older
assumptions. A crawling-queue test required the positive half of a two-way paved
road, so its road-clear assertion now accepts either paved side. The first rider's
default roam on a wide legacy road had expanded beyond its old 3.4 m limit and
changed a three-sided trap escape; retain that bounded default while the planner
still tests the full drivable road for passing. A physical opposite-side pass
lost its home return when that same rider's new free line was inbound; keep its
legacy positive-side default and distribute the other racer IDs across both
sides. The focused sealed queue, opposite-side pass/return, crawling queue, trap
escape and clear-road Metropolis lane choice now pass. One dynamic rider's
zero-speed plan left 1.36 m/s of residual chassis creep after 600 ticks; the
sealed-queue assertion allows under 2 m/s while still requiring zero impacts,
over 2.2 m rider spacing and position before both solid blockers. The failed
combined failure is retained in `combined-checks/tests.txt`; the final passing
pre-integration run is in `lane-followup-tests.txt`; the passing merged-tree run
is in `integrated-tests.txt`. The native full race and quiet cost gate remain.

A first new-policy debug native full-race attempt was [interrupted](native-lane-metro-full-01/INTERRUPTED.md)
at about 53.9 simulated seconds because simultaneous branch compilers made the
rendered survey impractically slow. It has no completion receipt and supplies
no performance or full-race acceptance. A fresh release-binary run is required.

### Final combined Metropolis native run (2026-11-11)

After guardrail PR51 and city activity PR53 were merged, the combined branch
passed `cargo fmt --all --check`, strict locked workspace Clippy, locked debug
and release builds, the source/hash audit, progress sync, five roadmap tests and
**647 workspace tests** (443 game library, 1 preview, 57 content, 146 core;
two ignored benchmarks). No source changes followed those checks. The release
binary SHA-256 is `c11a348c77a6f52ca9d4a36c9fdbeab805c96a26bfae17f85745f72f7c9cb38d`.

The unteleported [Metropolis race](native-lane-metro-combined-01/receipt.json)
finished slot zero at 31,689 frames / 525.05 simulated seconds; nine riders finished in the
window, the process exited 0, and no render/panic/shader error was found. The
36,000-frame cap did not end the race prematurely. [Current anatomy](native-lane-metro-recovery-01/receipt.json)
and [eleven-body baseline](native-lane-metro-baseline-01/receipt.json) native
recovery smokes passed on the same binary. These were functional Linux runs,
not isolated frame-time benchmarks or human playtests.

The [decision samples](native-lane-metro-combined-01/screenshots/racer-ai.json)
record rider lane coordinates and a completed slot-zero race. The physical
Metropolis regression above proves entry into the opposite paved lane; raw
native lane coordinates alone do not classify road side on every bend. The
samples also expose remaining stalls: riders 1 and 11 each have
an approximately 33-second longest consecutive sampled stop (three-second
sample interval) near station 3,040–3,055. At those samples combat decision is
`Race`, desired pace is above 50 m/s and final avoidance pace is zero. Thus the
observed stop is imposed by avoidance, not the killer's speed-matching rule at
those instants. The trace does not record the nearby car/rider proxies or static
hits; it cannot distinguish a legitimate blocked road from an avoidable queue.
Some stopped riders fall, and nine eventually finish. RFU-AI-01/02/06/08 and
AI-11 remain open for sustained-queue diagnosis, all-map road-side behavior,
quiet planner cost and human balance. No claim of eliminating every stop follows
from this finish.
