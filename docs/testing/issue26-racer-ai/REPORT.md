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

Initial independent core suite: 122 tests passed. New production Dynamic killer/racer, killer/killer, four-killer chain and actual opposite-lane pass/return tests passed. The first full game run had421passes/two outdated-policy assertions; no collision failures. The sealed-road control now includes a real opposite-lane blocker, while the independent opposite-pass case proves progress when open. Opposite borrowing yields to nearby lateral junction traffic. All three predictive traffic physical controls pass, retaining100health and >2m clearance assertions (crossing minimum3.14m, sealed-queue pair minimum4.57m). See initial-tests.txt and avoidance.txt. One initial test-format compile
error was corrected (Target has no Debug implementation). Final workspace suite:604passed (423game library,1binary,57content,123core), one pre-existing long soak ignored. Format, declaration/hash audit and five roadmap checks pass. Strict Clippy (all targets), locked debug/release builds also pass. Native race/recovery runs are in progress. No performance claim
or full-race/human acceptance yet. Player controls, physics rigs, saves, audio,
collision and recovery have no intentional changes.


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
