# Crash behavior investigation — 2026-11-07

**Research only. No runtime, physics, animation, tuning, dependency or asset changes.**
The user requested findings before implementation. The already-verified world/
contact work was published and merged as [PR17](https://github.com/ImadMashhood/RoadDash/pull/17),
main `6aa3f17383c7773ebb81e09a8706128f873bfc08`. This investigation lives on
`codex/crash-behavior-research`, based on that merge. Existing world performance
and art gates remain open; merging did not change their recorded status.

## Findings in RoadDash

1. **The lateral teleport is explicit.** `src/combat.rs` calls `recovery::crash`
   immediately after the third qualifying direct punch. `src/recovery.rs`'s
   `detached_origin` searches clear offsets starting at localX +1.25m, then -1.25m,
   then forward/back2.2m and larger diagonal offsets. The rider skeleton starts at
   that translated origin. It prevents initial penetration but visibly separates
   the rider before any physical loss of balance. Adding a swerve animation alone
   would leave this defect. Re-knocked walking riders use the same entry system,
   so a correction must distinguish mounted ejection from repeat foot knockdowns.
2. **We already have an active articulation.** Sixteen physical bodies, limited
   joints, passive springs/damping and capped native ForceBased joint motors are
   integrated. `muscles.rs` seeds targets from the released pose, then blends to a
   generic brace target while activation decays. Player movement input can briefly
   raise supported activation. There is no directional hand-to-ground protective
   response, posture goal to become supine, or continuing roll/slide controller.
3. **The existing roll is a one-shot cue.** `reactions.rs` waits for support,
   age>=0.3s and horizontal COM speed>=3m/s, then transfers up to8% of horizontal
   kinetic energy into a2.2rad/s roll tendency. There is no high-speed cutoff or
   success test for lying on the back. It follows slide direction, not a supine
   orientation goal. This explains why more resistance alone will not create the
   requested behavior.
4. **Support classification is too broad for the proposed behavior.** In
   `muscles.rs`, any active contact with a non-dynamic body counts as support.
   A wall or kinematic vehicle can qualify; surface normal, support location and
   whether the torso/pelvis is actually ground-supported are not checked there.
   A roll-to-slide controller needs genuine support and a contact normal.
5. **Energy policy needs coordination.** `constrain_crash_energy` limits combined
   owner kinetic energy against initial motion plus gravitational drop. Muscle
   motors can do work; arbitrary root/limb velocity corrections can add energy or
   have the guard scale away their intended result. Use a deliberate capped work
   budget, not unrestricted animation pursuit or extra launch impulses.

Evidence: the source at merged main, especially
[src/recovery.rs](https://github.com/ImadMashhood/RoadDash/blob/6aa3f17383c7773ebb81e09a8706128f873bfc08/src/recovery.rs),
[src/combat.rs](https://github.com/ImadMashhood/RoadDash/blob/6aa3f17383c7773ebb81e09a8706128f873bfc08/src/combat.rs),
[biomechanics/reactions.rs](https://github.com/ImadMashhood/RoadDash/blob/6aa3f17383c7773ebb81e09a8706128f873bfc08/src/biomechanics/reactions.rs)
and [muscles.rs](https://github.com/ImadMashhood/RoadDash/blob/6aa3f17383c7773ebb81e09a8706128f873bfc08/src/biomechanics/muscles.rs).

## What the original-game reference actually confirms

Inspected arkits/road-rash at `40829dbb2d30c5961cfd46c4fcf6373247126125`.
It is a reverse-engineered port; this session did not run a new retail replay.
The source confirms speed/height/reason-driven sprite selection, not a simulated
musculoskeletal attempt to get onto the back.

- Crash entry uses sequence33 normally,29 for reason2,35 for reasons4/5. It copies
  body/bike motion channels and applies caller-specific deltas. A decisive combat
  hit enters the crash path immediately; non-knockdown hits can affect steering.
  No mandatory left-swerve/right-countersteer-before-release sequence was found
  in this inspected path. That would be our new behavior, not an exact port.
  [Entry](https://github.com/arkits/road-rash/blob/40829dbb2d30c5961cfd46c4fcf6373247126125/src/crash.cpp),
  [knock-off decision](https://github.com/arkits/road-rash/blob/40829dbb2d30c5961cfd46c4fcf6373247126125/src/race.cpp).
- Near-ground airborne entry33 selects transition34. Its queued motion depends
  on speed. Moving ground states28/29 share the same seven sprite images with
  different timing; sequences30–32 supply other slower crash variations.
  Under native speed2500,36 is selected; below7001 the slower variants are used;
  faster motion stays in the28/29 family. At speed0 the crash phase changes to
  recovery, selects36 and queues37. Height also gates the first ground transition.
  [Selector source](https://github.com/arkits/road-rash/blob/40829dbb2d30c5961cfd46c4fcf6373247126125/src/actor_animation.cpp).
- Existing executable-backed [context catalog](../../animation/context/CATALOG.md)
  maps these sequences to actual slot series, including33 airborne,34 transition,
  35 combat,36 get-up,37 wait,38 run and41 bike pickup. It does not prove that a
  sprite transition is an intelligent physical recovery attempt.
- Using the previously verified HUD speed coefficient, native2500/7001/8001 are
  approximately40.27/112.78/128.89km/h. These are **animation thresholds**, not safe
  physical roll limits for our3D body. Native speed integration applies surface
  drag/contact friction, reduced airborne drag and clamps at a stop; we should
  reproduce readable phases rather than blindly transplant those raw constants.
  [Motion integration](https://github.com/arkits/road-rash/blob/40829dbb2d30c5961cfd46c4fcf6373247126125/src/bike.cpp).

The remembered visual is a sound target, but literal autonomous supine recovery
is not established by these source selectors. A contact-aware3D interpretation
would be new RoadDash work.

## Library/source comparison

Exact commits, file SHA256s and inspection dates: [sources.json](sources.json).
All are **REFERENCE ONLY**; no source or media incorporated or executed.

| Project | Verified license / compatibility | Useful techniques and limits | Recommendation |
| --- | --- | --- | --- |
| [bhubbard/active-ragdoll-rs](https://github.com/bhubbard/active-ragdoll-rs/tree/f9fb31c4c1d705bbd6b48ed9447ed6563066c2b5) |0.1.1; MIT OR Apache-2.0; Rapier0.36/glam0.33.10. RoadDash uses bevy_rapier3d0.32 wrapping Rapier0.31, with Bevy's glam0.30.10. | Master/follower joint-space targets, quaternion PD and collision strength changes. Adapter directly adds linear tracking velocity and torque impulses; it merely accesses/wakes joints rather than applying all output motor/limit fields. Limp branch zeros spring/damping. These policies conflict with passive resistance and preserving free crash momentum. | Study; do not add as a drop-in dependency. Disabling its default Rapier feature still needs a custom Bevy/type adapter and duplicates much of our existing controller. |
| [DanilKhmyrov/godot-active-ragdoll](https://github.com/DanilKhmyrov/godot-active-ragdoll/tree/bd212867145484aafb9393a0ccc2bfc3ec0befae) | MIT addon; Godot4/GDScript, not Bevy. Last inspected commit2026-11-05. | Strongest behavioral match: directional protective arms/head, partial hit reactions, strength by body group, roll_over/lying_on_back and contact-to-get-up alignment. roll_over keeps a chosen direction, reverses once when stuck and gives up after2s. However it reduces friction and directly blends body velocities/adds angular velocity; pin/length helpers can reposition bodies. These are not verified momentum/energy-safe drop-ins. | Study behavior and pose targeting; implement equivalent bounded native joint control if approved. No asset import; bundled media has separate licenses. |
| [sergioabreu-g/active-ragdolls](https://github.com/sergioabreu-g/active-ragdolls/tree/459cbab9489c7481e65bf16ac93fcfff98e6a594) | Apache-2.0; Unity C#, last commit2021-02-02. | Separates target animation from physics; calibrates joint-space rotations. Optional stabilizer/frozen-rotation modes are unsuitable for uncontrolled motorcycle falls. | Reference target-space conversion/modularity, not engine replacement or upright stabilization during crashes. |
| [ashleve/ActiveRagdoll](https://github.com/ashleve/ActiveRagdoll/tree/749cd80dbccd73dffb7bb1d3827ad0e719b84f1c) | MIT (Lucas Z. copyright); Unity C#, last commit2023-03-05. | Collision-driven loss/gain of strength and animated target skeleton. VelocityChange tracking can pull body COMs toward animation. | Study strength transitions; avoid translation chasing through obstacles or overwriting physical motion. |

The Godot core muscle loop attempts parent counter-reaction using inertia-scaled
velocity adjustments. That is useful intent, not proof of exact full3D angular
momentum conservation or stability at arbitrary timestep. Our native Rapier joint
motors already solve reciprocal constraints; replacing those with direct velocity
writes would give up that advantage. No external demo was built or benchmarked in
this session, so visual/performance claims remain unverified here.

## Proposed next implementation, awaiting user direction

1. **Continuous ejection first.** Keep current seated/world pose at release,
   carry per-bone v_COM + omega×offset, and prevent initial rider/wreck penetration
   with a brief pair-specific release filter/shape-safe handoff, not a metre-scale
   root offset. Preserve collisions with the road, cars, other riders and scenery.
   A temporary wreck exception needs a short clear/end condition; never disable
   the entire body collision group or allow the rider to pass through the car.
2. **Combat loss of balance.** Retain the existing three-direct-hit knockout rule.
   A decisive hit starts an explicit stagger stage while rider/bike stay mounted:
   steer away from the hit, countersteer toward the previous line, then release
   during that countersteer. Actual tire forces/grip/speed determine the path;
   nearby collision interrupts immediately. Do not teleport back to the old lane
   or unlock impact-driven heading generally. A left-first variation could be
   authored, but responding to hit side is the more consistent proposed default.
3. **Directional protective pose.** Ground probes/predicted approach choose a
   reach toward the fall, chin protection and softened elbows. Capped joint motors
   drive targets; hands may miss, buckle or be overwhelmed. At extreme speed,
   use a brief protect/tuck attempt rather than an unbreakable straight-arm brace.
4. **Attempt → yield → back slide.** With real ground support, moderate COM speed,
   manageable spin and clearance, try one limited roll toward a supine posture.
   Observe orientation progress, surface normal and contacts; time out/yield if
   obstructed, airborne or overwhelmed. Once supine, use a low protected pose and
   physical friction to lose speed. At high speed/spin, keep free tumbling until
   conditions permit another bounded attempt. Do not force a landing orientation
   or guarantee a flip; contact geometry and inertia decide whether it happens.
5. **Controlled work and comparison.** Preserve accepted joint stops/masses/solver,
   existing offscreen five-second approximation and baseline flag. Spend existing
   motion energy where appropriate; any active muscle work must be separately
   bounded and compatible with CrashEnvelope. Log added work/COM change and avoid
   free root acceleration. Thresholds/times remain tuning experiments, not claimed
   original constants or completed behavior.

Existing open tasks V2-01-006 and V2-03-005 own these changes. No implementation
or migration decision is approved by this report. Suggested experiments: same
20/60/120/200km/h entries; prone/supine/side poses; airborne and wall-only contacts;
curb/traffic/guardrail interruption during each swerve stage; trapped limbs; slopes;
repeat runner knockdown; player/NPC/offscreen; pause/reset/remount; fifteen riders.
Compare pose continuity, momentum/work, anatomical errors, failed attempts, settle
and recovery time, frame cost and saved-state regressions. A/B review should
precede any default replacement or merge of crash changes.
