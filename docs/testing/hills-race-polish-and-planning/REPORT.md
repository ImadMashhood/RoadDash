# Hills race polish and staggered rival planning

## Reproduction and changes

The reported Hills session recorded the player at 11,501.63 m on the east
route, lane 5.50 m, with no finish recorded while three AI had finished.
The built-in point-to-point races now use two directional timing planes.
A forward crossing of the start arms the finish. Both planes check the racer
actually crossed at road height and within the verge. Authored checkpoint
data remains available for route validation and custom courses. The race
world renders checker paint and roadside signs at the start and finish.

A mounted finisher applies full brakes while its bike remains solid.
Riders can run into a finishing or stopped bike. A local bike finisher gets a
short braking run-out before results open. The standings and reward use the
time and place at the line.

The two Hills herds each have 20 solid cows in four staggered rows of five.
They travel at 0.88 m/s through 40 m of lateral range. The minimum tested
gap between adjacent cattle colliders is about 2.9 m. Each cow follows a
terrain and road profile sampled at spawn, keeping terrain projection out of
the per-tick herd path.

The local rider has eight reserved rear-wheel plume slots within the existing
particle pool. Emission samples the actual surface under the rear wheel.
Sand retains the strongest loose-surface penalty with a 24 m/s cap, gentler
drag and more drive force. The bridge has textured shoulder end faces and
tapered deck approach lips using the existing render batches and colliders.

## Planning references

- [ForzaETH](https://f1tenth.github.io/publications/ForzaETH.pdf) describes
  reusing global track geometry while planning locally.
- [DetourCrowd](https://github.com/recastnavigation/recastnavigation/blob/main/DetourCrowd/Source/DetourCrowd.cpp)
  uses distinct path maintenance and local avoidance stages.

The implementation reuses a baked Hills corridor envelope and a per-tick
traffic snapshot. Expensive per-rider plan searches are staggered. Motion,
closing-speed checks, collision and emergency reaction stay on the 60 Hz
fixed clock. The comparison toggle is `ROADDASH_AI_REPLAN_HZ=20`; use
`ROADDASH_AI_REPLAN_HZ=60` for the every-tick planner control.
The research informed design. All geometry and implementation are repository-native.

## Native planner comparison

Same release binary, Hills seed 25, 1440×900 Xvfb, NVIDIA RTX 3080
with Vulkan. Each capture used 620 render frames and reached 7.317 s of
race simulation. Comparison uses the common elapsed window through 12.45 s
including countdown.

| Replan mode | Mean drive ms / fixed tick | Full searches / tick | Exact static casts / tick | Immediate replans / tick |
| --- | ---: | ---: | ---: | ---: |
| 60 Hz | 1.914 | 13.833 | 188.246 | 0 |
| 20 Hz | 1.151 | 5.184 | 96.438 | 0.508 |
| 10 Hz | 0.914 | 3.014 | 60.502 | 0.618 |

The 10 Hz run measured 52% less drive time, 78% fewer searches and 68%
fewer exact casts than the 60 Hz control. At both race snapshots, all
fourteen rivals were riding and above 24 m/s in every run. At 5.98 s of race
time, their station range was −23.73 to 133.47 m at 10 Hz versus −22.64 to
132.48 m at 60 Hz. The timings cover the grid launch and first Western
blocks. App frame wall timings varied under concurrent builds and tests
and cannot establish a display FPS gain. The paired build preceded the
final stationary-yield emergency invalidation refinement. Final core
tests cover a moving hazard entering a parked rider’s retained plan.

The real Hills regression starts two rivals 2.74 m and 7.44 m behind a
stationary player. Both cleared the player by over 30 m in 515, 553,
and 554 fixed ticks at 60, 20, and 10 Hz, respectively. The closest
clearance stayed above 1.2 m and all three riders remained mounted and
at full health.

Reproduce the comparison with an isolated settings/save directory and run
`./play-bevy20.sh --hills --racer-ai-survey` with
`ROADDASH_RACE_SEED=25 ROADDASH_AI_SURVEY_FRAMES=620
ROADDASH_AI_SURVEY_HOLD_W=1 ROADDASH_RAW_FRAMES=1`. Use
`ROADDASH_AI_REPLAN_HZ=60`, `20`, and `10` in separate runs.

## Functional verification

- The fixed-control Valley fixture crosses the start and finish, observes
  the bike moving during run-out, brakes to a stop and awards one result.
  The Hills timing fixture checks both routes at the reported finish lane.
  It rejects a finish without a forward start and a reverse finish.
- The same contact regression rear-ends an active bike and a finished
  bike. Both can eject when contact exceeds the existing threshold. Both
  physics adapters brake and retain the stopped collider.
- The local plume test fills the remote slots and verifies the player can
  emit with readable alpha. Sand driving tests cover both adapters, coast,
  reverse and recovery. Abutment tests check world-space joins, normals
  and render/collision separation.
- Hills wildlife tests cover both 20-cow formations, minimum through gaps,
  center-lane occupancy, and the baked support error. Mounted collision
  tests confirm a fast hit still knocks the rider off in both adapters.
- Asset audit and roadmap synchronization passed. The two existing Rapier
  checks remain tracked in `docs/KNOWN_ISSUES.md`.

The Metropolis native smoke passed both crash recoveries and the results
fixture. The Hills polish survey captured seven frames including the player
dust trail. Its bike traveled 87.2 m, emitted 80 puffs, and used a maximum
of five simultaneously active puffs with the unchanged 96-slot budget.

The Hills native smoke passed two remounts and the results fixture on retry
after the first run timed out. The bridge survey produced both deck-level
and aerial captures before its overall deadline. Neither view provides a
close-up of the far abutment. Full race finishes on all point-to-point maps
and the close-up bridge review remain open.

Final verification passed formatting, strict workspace Clippy, locked debug
and release builds, and the asset and roadmap audits. The locked workspace
suite had 768 passes, two ignored tests and two existing Rapier failures
(wall-impact bounds and Metropolis static chunk contact/CCD parity).
