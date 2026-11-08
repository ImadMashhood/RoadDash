# Live pickup and parking reverse — 2026-11-08

RECOVERY-01 / REVERSE-01 / ANIM-14; ADR077.

Remount now uses the live wreck location and upright heading. Track projection
updates race progress/lane only. The handoff also resets Rapier's current/next
pose, velocity/forces, BikeState and interpolation history; an old hidden chassis
can no longer restore its crash position on writeback. Uprighting finds solid
support excluding the recovering bodies instead of pulling a bike off an elevated surface to terrain. Only the
legacy assisted adapter needs an authored height offset; dynamic mode uses its
physical chassis. Accepted ragdoll anatomy, joints, muscles and momentum remain.

Hold **S / Down / the mapped brake** to brake forward, then reverse slowly after
stopping. Release to coast; W/throttle returns forward. Reverse targets 9 km/h
(2.5 m/s), with 2 m/s² requested acceleration; constants live in
`crates/rr_core/src/reverse.rs`. Dynamic motion remains tire-force/grip limited,
not a hard velocity assignment; gravity/external forces can exceed that target.
No extra key, automatic backup teleport or AI reversing is introduced.

## Verification

- Full workspace: 311 passed, one existing ten-minute soak ignored. Includes
  unchanged model/physical-body differential checks and 100-crash anatomy tests.
- Displaced elevated-wreck remount on fixed and kinematic supports: both adapters retain the pickup location at
  handoff; later dynamic ticks cannot restore the stale backend/crash position.
- Pure parking rule tested at 30/60/120 Hz; forward braking and throttle conflict
  are covered. Dynamic integration backs away from a solid wall and returns to
  forward drive. Assisted reverse reduces progress without false race finish.
- Native Metropolis routes 0 and 1: real graphics/audio, fifteen-racer grid retry,
  brake/reverse, farm/city crash/getup/run/remount and results fixture passed.
  Both runs measured reverse at −1.827 m/s. Four remounts measured 0.0000 m pickup
  error. This is scripted integration, not a human full-race acceptance or FPS
  improvement claim. See native-route0/1.txt and corresponding JSON receipts.
- fmt, workspace clippy with warnings denied, workspace tests, locked debug and
  release builds, asset audit, roadmap tests/sync, animation coverage --check,
  six source-context tests and git diff --check passed; receipts are alongside.
- reverse.png and pickup.png were inspected from the actual game. Physical
  controller mapping/Windows acceptance is not claimed.

Reproduce native verification:

```bash
DISPLAY=:0 ROADDASH_TRACK=metro ROADDASH_VERIFY_REPAIRS=1 \
  ROADDASH_VERIFY_REVERSE=1 cargo run --release --locked -- --smoke-test
# Add ROADDASH_SMOKE_ROUTE=1 for the other city branch.
```

The smoke deliberately injects crash/results fixtures after driving/reverse;
it does not establish uninterrupted course completion. Existing exceptional trap
rescue and unseen rear NPC approximation remain separate policies. Confined
mount clearance/art approval stays under P06-009; no broad gate is closed.

## Checklist and next part

Reviewed completed/open status against current evidence. Added only verified
RECOVERY-01, REVERSE-01 and ANIM-14 to Part 00; README and generated progress now
contain 204 completed / 242 open tasks (446 total). Broader partly implemented
animation, crash, hardware, art and performance requirements remain unchecked.

Part 02 Animation A resumes with a reproducible hash-bound coverage audit,
[readable table](../../animation/RUNTIME_COVERAGE.md), machine report and
[next review](../../animation/NEXT_REVIEW.md). It maps 41 context sequences / 138
draft human poses to 13 shipped clips. 104 context poses lack a standalone clip;
two runtime lean keys are separately authored. Membership/durations do not prove
correct playback, native timing or approved silhouettes. No animation/character
art changes are smuggled into this pickup fix.

Changed implementation: rr_core reverse rule, game/vehicle adapter integration,
recovery handoff, focused tests and native verification. Documentation: README,
progress/placement, ADR077, issues/history/test matrix, registered vehicle-source
hash, animation audit generator/reports. No files removed, no dependencies or
external source/media added; no new GitHub repository investigation was needed.
