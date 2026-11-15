# Race performance, difficulty and world follow-up

The latest Hills playtest recorded 329 racing windows with a median window p95
of 27.54 ms. The median main-world cost was 17.00 ms/frame, including 5.97 ms of
fixed gameplay and 4.76 ms of physics. The captured game and its content use a
16.67 ms frame budget at 60 FPS. Scene complexity is only one part of the
comparison with another game. This trace already spends most of that budget
on the game thread before accounting for presentation.

## Performance work

The static rival planner now builds one filtered Rapier query for each lane
forecast and reuses a blocked sweep only when its endpoints and rider snapshot
match exactly. Collision handling checks active vehicle contacts adjacent to
riders rather than scanning the whole contact graph. The first prototype
tested a whole-route broad phase. A 900-frame Hills replay was deterministic
between its two runs, but none of the 23,622 forecast bounds were empty.
That shortcut was removed.

The final same-binary Hills A/B switched only blocked-sweep caching. The 900
scripted frames yielded identical racer state in all 75 survey records and
avoided 280 duplicate blocked casts in logger windows from race time 3.0–11.5
seconds. Native frame timings overlapped with compilation and a separate game
instance. These runs verify work removed and behavior parity. Whole-frame
speed, the separate LOD contribution and a clean full-course target stay open.

## Gameplay and presentation

- Medium retains the shipped offline Normal pace and loads the old settings
  value. Hard uses full bike-class pace and a five-percent higher offline
  corner acceleration budget through the existing surface and traffic limits.
- Verified weapon contacts now use separate authored bike and rider condition
  damage. The established hit count, recoverable crash behavior and pre-contact
  momentum remain in effect.
- Two Hills herds hold staggered six-cow formations and walk at 0.88 m/s.
  Shorter paths keep more of each crossing around the road. A fast mounted collision can topple
  the racer and leave the crossing cow on its patrol. Individual cows retain
  the soft contact response.
- Ambient sand travels horizontally and fades through shared opacity levels.
  Stronger rear-wheel dust uses the contacted surface and stays close to the
  ground. The in-tree textures use generated masks. Kenney's Smoke Particles
  pack is a CC0 option considered for future sprite variation. [Kenney Smoke Particles](https://kenney.nl/assets/smoke-particles)
- Grass coverage and terrain color blend into the meadow from 1.00 through
  1.64 km. New landscape mesh tiers include the 12-triangle middle woodland
  tree, lower-detail rocks, boulders, cacti and deadwood, and simpler Valley
  street-tree crowns. Shared authored trees and rocks switch to cheaper models
  sooner across courses. Colliders stay resident through the visual tiers.

## Verification

Formatting, strict workspace lint, locked debug and release builds, asset
audit and roadmap checks passed. The final workspace suite passed 755 tests
with two existing Rapier failures and two ignored tests. The known failures
cover the high-speed ejection anchor limit and the Metropolis standalone
static collision trajectory.

Focused coverage includes Medium settings migration, weapon hit and miss
condition changes, both riding adapters against a crossing cow, ordinary
cow ride-over, formation spacing, wheel ground contact and plume fade, the
city-edge grass ramp, mesh budgets, and retained Valley tree colliders.
Native release smoke passed on both Hills and Metropolis, with two remounts
and race standings on each course.
Hard balance and high-quality real-time presentation remain human review gates.
