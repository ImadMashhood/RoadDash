# Vehicle, contact and combat contract

Updated 2026-10-30. Native client defaults to the dynamic adapter. For comparison,
`ROADDASH_VEHICLE=assisted cargo run --release` selects the retained kinematic
baseline. Both consume the same fixed-tick racer identity, race/gate, damage,
recovery and input contracts. No physics-library migration was performed.

## Part 03 vehicle model

`src/vehicle.rs` owns the adapter, force update, readback, telemetry and visuals.
Human and AI commands share `Rider.drive_intent`; existing traffic avoidance is
preserved instead of duplicating AI decisions in the physics adapter. UI never
sets authoritative position. During racing a dynamic chassis receives forces and
torques; its pose is projected back onto canonical route progress after physics.
Garage/countdown/recovery freeze the hidden chassis, and remount clears stale
velocity. Wreck mass retains the selected bike's dry mass, without rider mass.

Three schema-1 presets in `content/handling.json`: light/agile (170 kg dry),
balanced (210 kg), heavy/powerful (265 kg), plus a 70 kg rider. The pure authoring
validator lives in `rr_content::handling`, rejects unsupported versions, missing/
duplicate presets, invalid IDs and nonfinite/out-of-range physical parameters.
Override with ROADDASH_HANDLING. CLI:

```sh
cargo run --locked -- --validate-handling content/handling.json
```

Two filtered wheel rays apply spring compression/damper forces and weight preload
at contact points, creating pitch/roll moments. Inverted wheel rays cannot apply
lift. A low-speed antiparallel fallback rights an upside-down chassis. Chassis
mass, center of mass and inertia are explicit; CCD is enabled. Upright/heading
assistance uses bounded feedback torques. There is no lane-zero position magnet;
lateral steering is speed-sensitive, with reduced road assistance and F6 adjustment.
It remains an arcade road-tangent-assisted bike, not a free-heading full multibody
motorcycle. Wheel spin, suspension travel and front-wheel steering are separate from the
authoritative collision body. Latest feedback restores coordinated steering-input bank on bike and rider.
Road/chassis roll is removed from the rendered pose before input bank; curves do
not introduce lean. The original aggressive 57-degree visual target is superseded.

A torque/RPM curve, redline cutoff and six automatic ratios drive longitudinal
force, with engine/service braking and quadratic aerodynamic drag. The target
speed tapers engine force; excess momentum is not directly clamped away. The
combined longitudinal/lateral tire request is limited by wheel load × surface
traction × preset grip. `slip` is the saturated-demand fraction; `slip_angle` is
actual lateral velocity angle, not a detailed tire carcass or physical wheel
angular-velocity model. Grass retains positive acceleration and no terrain damage.

F3 shows preset, assist, RPM/gear, slip and wheel contacts alongside timing.
F5 cycles the three presets; F6 switches low/high road assist. Bounded telemetry
retains 3,600 authoritative 60 Hz samples: bike/preset/rider identity, tick/dt,
pose/velocity, inputs, yaw rate/lean, wheel contact/compression/forces, grip/tire
forces, surface coefficient, slip/angle, health and crash state. Render smoke
exports JSONL. No 100 Hz physics or fabricated intermediate samples are claimed.
Production settings persistence, garage class selection, force-curve editors and
multiplayer authority remain later requirements.

## Direct strikes and knockout timing

One target per attack. A direct punch requires the selected side, lateral center
separation 0.65–1.9 m, longitudinal separation <=0.9 m, chest-height difference
<=0.9 m, aligned rival headings and an unobstructed scene-query line. The nearest
legal target receives the strike. Out-of-range, behind/ahead, wrong-side or
occluded targets receive no hit. An attack edge is consumed once, existing punch
cooldown remains, and duplicate damage within 0.25 s cannot advance the chain.

Knockout requires three accepted direct hits, with **each gap <=3 s**. Thus hits
at 0, 2 and 4 s knock out; a gap longer than three seconds starts a fresh chain.
The HUD armor proxy displays the chain's remaining thirds; expiration restores
it. Melee immunity and the three-second AI launch grace prevent immediate spawn
combat. AI waits before weaving and uses a cadence compatible with the hit window.
The same chain/geometry rules apply to pedestrian and cow melee targets.

## Motorcycle contacts

Measure relative velocity closing along the contact normal, not absolute speed.
Rear/longitudinal closing >=50 km/h (13.89 m/s) wipes out the struck rider; a
broadside normal closing >=6 m/s is a substantial side impact. Smaller contacts
transfer a bounded nudge; equal-speed contact is not a knockout. Actual dynamic
contacts transfer mass-dependent momentum through Rapier. The assisted adapter
also applies bounded lateral/longitudinal nudge state. Contact pairs are deduped;
pre-solver velocity is retained for classification so a solver-stopped bike does
not hide a hard collision. These replace the older universal rider-contact hop.
Cow/pedestrian/fallen-object arcade lift and gravity/landing remain separate
hazard behavior. Civilian clearance derives upward speed from standing height
plus margin and scales impulse by bike+rider mass, restoring cached horizontal
approach velocity after the contact solver. Both civilian kinds still ragdoll.
Crossing cars are explicit wipeout hazards with all-type contacts and CCD, even
when a crossing car strikes a stationary bike.
Solid traffic and immovable obstacles retain their existing collision semantics.

Sidewalks are support geometry. The dynamic tires climb the curb and suspension
raises the chassis; the assisted adapter samples a reachable support height.
Support contacts cannot trigger obstacle knockoffs. Larger walls/buildings are
still solid. American right-hand traffic means forward-moving traffic is in the
positive camera-right lane; cross traffic uses separate right-hand lanes with
one-way circulation, not in-lane reversal. Racing remains full-road overtaking.

## NPC physics and presentation

Pedestrians use 11 human bodies / 10 anatomical joints. Cows use a heavier torso,
head/neck and four two-segment legs: 10 bodies / 9 joints, including directional
front/hind leg hinges. These actors never spawn motorcycle wrecks. Solid bike/car
hits can knock them down independently of the three-punch chain. Their standing
collider/patrol is disabled during ragdoll. At most eight NPC ragdolls are active;
cleanup/reset removes their bodies/joints. After 18 s they may stand/respawn at a
clear original location, waiting if occupied. This is an arcade lifecycle, not
production get-up animation or biological simulation. Impact cues use original
synthesized audio. Native actor smoke visually checks both topologies.

Minimap dots are racers only and share their shirt palette, per the user's
correction. No pedestrian/cow/category markers were added. Hill guardrails now
appear on the outer side of curves; straights are open. More trees include
broadleaf/conifer forms in all zones. Buildings have individual windows, doors
and roof caps; crossing vehicles have cabins, wheels and lights, and pedestrians
have animated limbs. This remains original procedural graybox art.

## Shared human appearance

`human::SPECS` and its segment builder define eleven identical body slots, helmet/
visor, proportions and shirt/trouser materials for seated bikers, pedestrians,
recovery runners and ragdolls. Only pose/physical control changes between states.
Rider palette identity survives crash/recovery/reset. Standing pedestrian support
proxies and respawn occupancy checks now cover the shared model's 2.31 m height.
This is original procedural geometry, not a new glTF/production animation rig.

The seated human inherits the same input-only visual bank as the bike, without
its old counter-roll pivot. Actual physics transforms remain unchanged by
presentation. See research/ROAD_RASH_HANDLING.md for verified steering behavior
and the explicitly incomplete whole-handling equivalence gate.
