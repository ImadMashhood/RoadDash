# Civilian traffic stall follow-up — 2026-11-10

Scope: existing AI-QUEUE-02/P08-005. Source branch codex/traffic-stall-fix,
starting at 8891300b, isolated from concurrent Hills/heavy-traffic work.

## Reproduction

The previous selector treated cars within 5 cm as tied, but used strict progress
outside that band. Those two rules were not transitive. Cars A/B/C at 0/0.04/0.08 m
selected leaders C/A/B, producing a circular wait with no free leader.
[Core failure](before-core.log) records the exact dependency cycle.
[Production fixed-update failure](before-integrated.log) records three inbound
cars remaining at 2900/2899.96/2899.92 m with zero speeds for 20 simulated seconds.

The screenshots establish a reported stopped queue, not its complete runtime
state. This reproduction identifies a real indefinite-stall defect; legitimate
traffic waiting for an actual person/obstacle must still stop safely.

## Correction

Strict signed longitudinal ordering; stable identity only resolves exactly equal
stations. Every leader chain has a free front car. Preserve merge lookahead,
following gap, braking/acceleration, car population, downed-person stops and
physical collision/recovery. No teleport or forced motion through blockers.

Pure rule: crates/rr_core/src/streets.rs. Integration: src/game.rs.
Regression fixtures: src/traffic_stall_tests.rs and src/traffic_stall_survey.rs.
Native --traffic-stall-survey uses three production car models in a deliberately
compressed queue, both travel directions, with other traffic/racers excluded from
the scenario. It is functional/rendered evidence, not a benchmark or full race.

Verification: 551 workspace tests passed, one existing long soak ignored; strict
Clippy, fmt, locked debug/release builds and asset audit passed on the isolated
branch. Both new production regressions also pass with the concurrent Hills
changes in the shared checkout. Shared-wide fmt/audit currently flag unrelated
Hills work in progress (see shared logs), not the isolated fix.

The rendered survey passed both directions over 3,000 frames. All six observed
cars resumed their 18 m/s cruising speed and moved at least 177 m. See
[recorded positions/speeds](traffic-stall.json) and [inbound capture](traffic-1-moving.png).
The outbound camera was obscured by a building; it is not offered as visual proof.
Native active and baseline-rig recovery checks both passed, including two
crash/remount sequences each. [Receipts](native-receipt.json) identify the frozen
release binary; no performance claim is made from these functional runs.

 No new task IDs or checkbox closures. No external
research/code/assets, dependency changes, physical-rig tuning or measured FPS claim.

Git: focused fix retained on codex/traffic-stall-fix; source and report also
applied to the shared codex/hills-map checkout without replacing its pending
Hills work. No remote push/merge for this follow-up. Rebuild/restart the game to
use the fix. Broader full-race traffic acceptance remains open.
