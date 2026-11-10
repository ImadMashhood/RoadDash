# Part08 pursuit memory and feedback follow-up — 2026-11-09

Focused local branch `codex/part08-police-fresh`, based on `d5db26f`.
This pass continues V2-08-002/004/005 without closing their broader gates or
starting Part09. No new requirement IDs, dependency/lockfile migration, asset,
physics, vehicle-control, recovery, save or roster changes.

## Implementation and decision

Review against ADR086 exposed three gaps in the last-visible-target contract:

* While its drive target was frozen, the pure mind still changed between
  Pursue and Intercept from an **unseen live** target's relative progress.
  Direction changes now require visible perception. The 3s sight-loss and 5s
  search/escape windows retain their authored behavior.
* Search retained the remembered rider's velocity feedforward, so a moving
  last-seen target could keep the officer driving beyond its search point.
  Search now approaches the stationary remembered progress using a bounded
  0.75/s closure and 2m stop margin. Passed points request zero speed rather
  than reverse/teleport. The common physical controller performs braking;
  authoritative transforms/speed are never overwritten. Downed/foot memory
  retains the existing 24m ordinary stop/park policy.
* Pursue/Intercept flips replayed the transition siren and reset the race
  message. Those internal direction changes no longer announce a new chase.
  Actual warning/pursuit/search/reacquisition/escape/arrest transitions retain
  their cues. Warning HUD now states remaining compliance time; unseen chase
  states explain time to search. Arrest progress retains feedback priority.

Policy stays in `rr_core::police`; native HUD/controller adaptation stays in
`src/police.rs`. No tuning/rig migration is needed. Existing speed cap, one
resident officer,10Hz perception, conditional60Hz arrest validation and safe
contact requirements remain. Search stopping is a requested control speed,
not proof of a physically instantaneous stop.

## Research and rights

Inspected the installed manifest/lock, native/core/content police modules,
README Part08, PROGRESS, DECISIONS ADR086, KNOWN_ISSUES, frozen blueprint police
section and prior `docs/testing/part08-police` implementation/research/results.
No external repository/media/code imported. Existing pinned reference study and
its unresolved permissions remain historical evidence; this follow-up's
implementation is project-original policy code. No new license claim.

## Verification and remaining gates

See RESULTS.md for commands actually run. New regression cases cover unseen
interception direction, bounded stationary search, chase announcement semantics,
native drive goal/physical-authority separation, preserved foot stopping and
HUD countdown/arrest priority. Existing integrated police tests cover controller
approach/parking/walking/arrest/no-payout/retry, LOS loss/escape, unsafe contact,
finish/pause/restart priority, witnessed assault and officer knockoff.

Broader rival target discretion, obstacle/intersection encounter coverage,
moving-foot capture/tackles, bespoke police art/gestures/audio, accessibility,
human fairness, complete human offline race/soak and production performance /
hardware acceptance remain open. This policy follow-up does not establish those
or original-game parity. Root owns live checklist/progress synchronization and
combined final regression/native verification.

## Native-discovered Results/Busted retry follow-up

The integrated native police fixture reached Arrested/Finished but its real R
input was swallowed by frontend menu ownership. Earlier police-only headless
fixtures lacked the frontend and therefore missed this integration boundary.
`frontend::input` now consumes restart once and maps it to `Action::Ride` only
in Finished. This uses the existing Starting page/loading boundary and menu
handoff; it cannot start a Garage screen. Settings/readiness suppression still
consume the edge. Retry is independent of Results row selection. Existing Enter,
menu selection, profile/reset and single input owner semantics are retained.

Two new frontend-installed regressions exercise ordinary Results and police
Busted retry via actual captured R input, Starting then Countdown, cleared arrest
and preserved officer identity/profile, held-key nonrepeat, Garage rejection and
settings suppression. The standalone police survey now creates its own evidence
directory before requesting the Busted screenshot.

Worker ran direct rustfmt and `git diff --check` for this follow-up. Per root's
instruction, no Cargo worktree rebuild was performed: integrated root owns the
focused/full tests, clippy/build and repeat native fixture. The earlier400-test
receipt predates these retry changes and must not be cited as their verification.
