# Architecture decisions

## ADR-001 - 2026-10-30 - Retain compatible engine and physics pins

Problem: blueprint proposes Bevy 0.20.1/Avian 0.8/Enhanced Input 0.27, while the
working game is Bevy 0.17.3/Rapier 0.32.0. Exact inspected upstream manifests do
confirm the newer plugins target 0.20, so mixing them would be incompatible.
Alternatives: wholesale migration now; old compatible variants; retain pins.
Selected: retain current locked dependencies, implement Part 01 with their actual
APIs, and record coordinated migration as P01-MIGRATE. Bevy/Rapier official
fixed-schedule examples justify this without reimplementing a physics engine.
Tradeoff: planned newer features are deferred. Consequence: isolate adapter and
input contracts now; migration needs its own regression matrix and Windows gate.
Rollback: original baseline a0902c9; never force-push it over unrelated changes.

## ADR-002 - 2026-10-30 - Fixed simulation and buffered intent

Problem: Update-driven gameplay/physics depends on render cadence and can lose or
repeat button edges. Alternatives: continue variable dt; fixed gameplay with
variable physics; put both in FixedUpdate. Selected: 60 Hz Time<Fixed>, Rapier's
in_fixed_schedule with fixed dt and three substeps, explicit pre-physics gameplay
and post-writeback capture sets. Input captured in PreUpdate; edges buffered until
one tick consumes them. Pause disables physics and simulation progression; reset
removes transient actors. Cameras, UI and audio consume state per render frame.
Tradeoff: interpolation adds one fixed tick of visual latency. Consequence:
new gameplay must join the fixed sets; presentation may never set collision state.
Verification: 30/60/120 FPS comparison, zero-tick input and 100 reset cycles.
Rollback: restore scheduler together with input capture/interpolation, not piecemeal.

## ADR-003 - 2026-10-30 - Incremental workspace and subsystem plugins

Problem: a monolithic binary has no pure domain/content boundary. Alternatives:
create all eleven suggested crates immediately; retain one crate; extract useful
boundaries first. Selected: root offline client library/binary, renderer-free
rr_core (IDs, phases, intents, awards), rr_content (typed validated authoring), and
functional world/simulation/input/presentation/audio/lifecycle/diagnostics plugins.
No empty server/network placeholders. AI/combat/vehicle/race still share existing
systems; their deeper extraction and authoritative server belong to later work.
Tradeoff: P01's full responsibility split remains partial. Rollback: path crates
can be folded back without changing content IDs, profile paths or controls.

## ADR-004 - 2026-10-30 - Interpolate visual children, never bodies

Problem: fixed body transforms look stepped at high refresh rates; mutating them
for interpolation corrupts physics. Selected: previous/current pose history after
fixed writeback; render-only VisualRoot child offsets and camera interpolated
focus. Animation lives below that root. Teleports >20 m reset history. Alternative:
use a third-party interpolation plugin, or overwrite/restore body transforms.
Justification: small explicit adapter works with existing ownership/recovery and
adds no engine-version dependency. Test/risk: captures through crash/get-up/remount;
new model hierarchies must attach visuals through the helper. Rollback: remove
visual offsets/history together, leaving body simulation unchanged.

## ADR-005 - 2026-10-30 - Content and save schema version 1

Selected stable string IDs and serde-validated content in rr_content; built-in
JSON embeds the currently working tuning for a no-download launch. Input bindings
can be replaced via ROADDASH_BINDINGS. Existing versionless profiles decode as
schema 1 without dropping credits/records; unsupported/corrupt profiles are kept
read-only to prevent destructive overwrite. Alternatives: raw hardcoded values,
new persistence crate, or immediate complete career schema. Tradeoff: eight-rider
catalog and prototype aggregate profile are intentionally narrow. P12 owns backups,
checksums, full migration suite and career data. Rollback must preserve v1 decoding.

## ADR-006 - 2026-10-30 - Audit media, use a mathematical tonemapper

Investigation found Bevy embeds font/LUT media beyond Rust crate licenses. The
Fira Mono source license is verified OFL-1.1 and copied. Tony McMapface declares
MIT/Apache, but the cited AgXc revision has no license file/README grant located.
Selected: disable tonemapping_luts and explicitly use Reinhard, preserving PBR
without distributing unverified imported LUTs. No external art/audio is acquired.
Tradeoff: color response differs; visually inspect the native build. Alternatives:
assume package license covers all media, or fork the rendering crate; rejected.
Future: revisit verified color assets in P15. Rollback only after rights are verified.

## ADR-007 - 2026-10-30 - Features and offline authority boundary

Default features offline/devtools; minimal offline build can omit devtools.
debug_physics explicitly enables Rapier's collider renderer. No fake netplay mode
is exposed: transport/server/anti-cheat are P10/P13, tracked as open. Offline race
rules own results; UI derives screen state and never awards rewards. AppScreen
Boot/Loading/MainMenu/Race/Results describes presentation readiness, while core
Phase remains the one gameplay phase source. Linux/Windows CI builds are useful
but do not certify native Windows GPU/input behavior. Rollback keeps public CLI
and save paths stable; future network authority must not trust client transforms.

## ADR-008 - 2026-10-30 - Crash momentum and geometric remount reach

Problem: crash entry scaled forward velocity and added a vertical launch, while
stacked ragdoll/wreck bodies could appear seated. Fallen-bike root pivots could
be unreachable and the runner pushed its target away. Alternatives: more launch
impulse or wider root-radius teleport; rejected because they hide contact errors.
Selected: preserve measured world velocity (including lateral/vertical momentum),
choose a nearby collision-clear rider origin, remove added upward velocity and
use small opposing roll rates. The riderless wreck retains its own body; no
seated visual is copied. Repeat knockdowns read current runner-controller movement, not stale hidden-bike
velocity. Actual incoming impact transfer remains, as does the
previously requested attacker hop when striking another rider/bike.

Remount uses the closest point on the rotated wreck box, low linear speed and
vertical reach; the runner no longer applies impulses to dynamic bodies. Existing
mount animation and one-second protection remain; shoulder damage respects that
protection. No new dependency/repository or asset is used. Tradeoff: separation
uses a short lateral clearance adjustment at crash entry; fully confined geometry
falls back to the original origin and arbitrary maze-like trapping is not solved.
The assisted runner is not manual walking or navigation-mesh pathfinding.
Verification: raw spawn-velocity snapshot, seated-visual absence, twelve approaches
across six fallen poses, full dynamic recovery and native screenshots. Future:
keep crash placement/reach in authority simulation when extracting P06 systems.
Rollback these changes together with regression tests; preserve save/schema APIs.

## ADR-009 - 2026-10-30 - Authoring and point-to-point route authority

Problem: a fixed flat procedural loop cannot express the user's farmland/city/hill
map or valid checkpoints. Alternatives: import a level editor/large terrain plugin,
keep only a longer closed polyline, or author a small validated format. Selected:
pure serde track schema, independently implemented Catmull-Rom baking and real
3D arc length. User correction explicitly chooses point-to-point Valley Run;
extrapolated endpoint handles replace wrapping. A closed handling fixture remains
for existing circuit regressions, not as the new race map. Bevy 0.17.3 mesh APIs
and Rapier 0.32.0 triangle meshes are already compatible; no dependency migration.

Start/finish are distinct ordered physical gates. Racer-level gate state is
updated by fixed movement, not UI. Require forward plane crossing, safe corridor,
height and plausible displacement; progress/finish teleports cannot award results.
Open endpoints stop unfinished riders and report a missed-gate retry. Finished
rivals disable their collider. Civilian traffic recycles outside visible endpoints.
Tradeoff: no branch/split/merge paths or reverse recovery for a missed gate yet.
Future: P07 extends identity to valid foot finishes, P12 stores track/class records.
Rollback retains stable JSON IDs and preserves old saved aggregates; never restore
finish-by-distance without the skipped-gate regression suite.

## ADR-010 - 2026-10-30 - Graybox environment participates in gameplay

Selected original procedural meshes, independent 1.25 m render / 2.5 m road
collision ribbons, shared terrain sampling and 200 m terrain chunks. Terrain must
stay below banked asphalt; route ray tests verify the road is the first surface.
Seeded farmland gaps/fences/cows, solid buildings and reserved city cross streets,
sidewalk pedestrians/cars, actual hill elevation and guardrails use existing
physics. A small fixed patrol system handles crossing cars and walkers; it is not
urban traffic AI or pedestrian navigation. Existing cow contact uses the bounded
attacker lift/landing response. Hazard markers make animal/cliff regions explicit.

Alternatives: decorative-only props, hundreds of unconnected road slabs, external
asset packs. Rejected for collision continuity/provenance/scope. No third-party art
or models incorporated. Tradeoffs: graybox visuals, static cows, simplified patrols,
terrain grid and no streaming/LOD. User sketch is a concept reference, not shipped
media. Origin is fixed in a bounded world; future larger worlds need atomic rebase.
Future consequences: P08 adds decisions/navigation, P11 production assets, P15
performance profiling. Rollback removes environment actors/collision together;
do not leave invisible obstacles or render-only road replacements.

## ADR-011 - 2026-10-30 - Arcade handling and passive ragdoll resistance

Problems: throttle clamped excess boost speed; grass drag exceeded acceleration
and depleted health; uniform ball-joint limits let limbs fold unnaturally; high
wreck friction pinned bikes to slopes. Selected: coast toward the target speed,
positive surface-scaled acceleration/grass speed cap, no terrain-only health loss,
20% higher authored steering rates with backward-compatible schema-1 defaults.

Ragdolls use directional X-axis elbow/knee hinges (locked twist), asymmetric
shoulders and joint-specific hip/spine/neck limits with low-force spring motors
and damping as passive tissue resistance. This is an approximation of an unconscious
body, not a biological muscle/tendon model or active balancing animation. Rapier's
installed generic joint/force-based bounded motor APIs were inspected; no external
ragdoll source was copied. Slope-normal contact sampling selects low bike friction
on steep ground while retaining flat-road stopping friction; gravity drives sliding.
Mount height now derives from terrain rather than sea level.

Alternatives: add stronger launch/spin impulses, universal ball joints, always-low
friction, full muscle simulation. Rejected for behavior/control/cost. Tradeoffs:
arcade kinematic riding remains; limits need future animation/model tuning. Verify
boost release/exhaustion, grass AI scenarios, solid ramp slide, 100 dynamic drops,
existing controls/combat/collisions and native hill recovery. Future networking
keeps these settings server-controlled. Rollback must preserve verified grass
mobility/no-damage behavior and schema defaults.

Part 02 integration follow-up: Cargo emits UTF-8 metadata independently of host
code page. Audit subprocess/text readers now specify UTF-8; Windows cp1252
regression is simulated in the Python unit suite. This corrects an observed CI
failure, without weakening SPDX/hash requirements or claiming a native launch.
