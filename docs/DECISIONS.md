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

## ADR-012 - 2026-10-30 - Pinned dynamic arcade chassis adapter

Problem: Part 03 needs suspension, mass and traction without regressing the
working race/recovery loop. Alternatives: upgrade engine/physics for a current
vehicle API, fully articulated motorcycle, retain only kinematic movement.
Selected: `VehiclePhysicsAdapter` with native-default dynamic body/two support
rays, explicit inertia/COM, force suspension, torque/RPM/gears and combined grip.
Shared AI/player intentions preserve existing avoidance. Separate visual lean
and wheel animation; versioned pure handling validation; bounded 60 Hz telemetry.
Three dry masses are 170/210/265 kg, plus 70 kg rider; wrecks keep dry mass.
Remount clears old velocity. R08/R10 installed APIs are compatible; R03 studied
only, no source reused. Tradeoffs: road-tangent force assistance and approximate
tires/upright torque; not full multibody realism. Ten human rides/fun gate open.
Future authority owns BikeState/intent/profile IDs. Rollback via assisted adapter;
keep mass/recovery and numeric tests when migrating, never rewrite profile saves.

## ADR-013 - 2026-10-30 - Direct combos and relative impacts

Problem: broad proximity attacks and unconditional collision knockoffs caused
start pileups. Alternatives: health-only per-hit damage or blanket contact immunity.
Selected pure three-hit chain with consecutive <=3 s gaps and duplicate protection;
resolve one direct side-by-side, visible target. AI has launch grace. Contact
severity uses cached pre-solver relative closing velocity: >=50 km/h longitudinal
or >=6 m/s side wipes out; lower impacts transfer bounded momentum. No universal
rider-to-rider hop. Static/cow hazard responses remain separate. Tradeoff: arcade
thresholds need playtest balancing. No external combat code copied. Future server
must validate geometry/chain/velocity; rollback must retain matched-speed tap and
0/2/4-second regressions rather than restoring broad proximity knockouts.

## ADR-014 - 2026-10-30 - Bounded physical civilian knockdowns

Problem: solid cows/pedestrians did not react like riders. Alternatives: cosmetic
animation, unrestricted ragdolls, copy unknown-license rigs. Selected original
10-body/9-joint cow and reused independent 11-body/10-joint humanoid builder;
physical impacts or the direct combo trigger them. Max eight active; hide/disable
standing root/patrol; restore after 18 s only if original location is unoccupied.
A fresh-spawn guard respects deferred command timing; reset despawns parts/joints.
Tradeoffs: prototype procedural bodies and safe timed respawn, no get-up animation.
No third-party assets used. Future authority owns lifecycle; cosmetic rig changes
must retain physical budgets. Rollback removes root/parts/collider disable state
atomically; budget/reset/occupied-spawn tests protect against leaks and overlaps.

## ADR-015 - 2026-10-30 - Shared shirt colors and readable roads

Problem: map color request meant racers' shirts; overly strong centering/rails and
curb knockoffs reduced road freedom. Selected shared model/HUD shirt palette only,
reduced adjustable assistance with no lane-zero magnet, low sidewalks as support,
American right-hand traffic and separated one-way cross lanes. Hill rails appear
on outer curves only; original trees/windows/doors/vehicle/pedestrian detail adds
readability. Alternatives: actor-category minimap markers, remove curb collision,
uniform rails; rejected against clarified user intent/gameplay solidity.
Tradeoffs: no navigation/traffic-light AI or production art/LOD yet. References:
existing independently authored environment, no new media. Future content/traffic
expansion must preserve support/visibility/actor collision contracts. Rollback
must keep shared palette identity and ground tags; do not leave invisible curbs.

## ADR-016 — 2026-10-30 — Native input and local comfort without engine migration

Problem: Part 04 requires real gamepad discovery/analog control, rebinding and
saved comfort while retaining fixed input edges and recovery. Alternatives:
upgrade to the newer Enhanced Input stack (R06, incompatible with pinned engine),
keyboard-only simulation, hand-write OS HID discovery. Selected Bevy 0.17.3's
Gilrs feature (R08/R14), pure normalization/comfort rules and optional analog
strengths in the existing VehicleIntent, with a modal client preferences panel.
Native connect/disconnect stays in the engine; deterministic single-pad ownership
prevents mixed devices driving together. Binding conflicts/reserved keys rejected.
Corrupt/future files preserved; temp/backup writes occur on explicit save/close.
Camera follows interpolated heading/position with stable horizon, sphere sweep
and large-warp snapping. Optional speed cues reuse twelve UI entities.

Tradeoffs: extra libudev development prerequisite on Linux; fixed stick axes,
limited supported key/button set, no persistent hardware selection; independent
on-foot intent allocation does not implement manual recovery. Physical-pad and
external tester acceptance remain open. Gilrs MIT/Apache and bundled SDL mapping
Zlib notices/hash recorded; only dependency mapping data incorporated, no game
media or example source copied. Future server consumes normalized intentions,
not client comfort preferences. Rollback can disable optional cues/panel, but
must preserve saved files, analog semantics, edge buffering and checked keyboard
behavior; avoid an engine migration merely to replace this adapter.

## ADR-017 — 2026-10-30 — Shared humans and explicit civilian/traffic impact paths

Problem: the user reports steering lean looks odd, ragdolls change human model,
pedestrian/cow impacts fail to clear the biker and cross traffic can miss wipeout
classification. Selected: remove cosmetic turn roll (diagnostic lean demand stays),
one eleven-slot human geometry/material/helmet builder with seated/standing poses,
and preserve palette through recovery/reset. Ragdoll physics keeps its existing
joint/mass topology; only its visuals share the same builder as living actors.
Pedestrian proxy/respawn clearance follows actual shared height. Alternatives:
rescale independent meshes or add production rig/assets now; rejected for drift
and unnecessary art-pipeline scope. No outside model/source copied.

Civilian jumps are a deliberate arcade exception: derive upward speed from
standing height+margin, mass-scale delta-velocity impulse, restore pre-solver
horizontal approach momentum. Ordinary crashes remain momentum-only. Crossing
cars get explicit hazard classification, all-type events and CCD; tests include
stationary cars being struck and moving cars striking stopped bikes in both
adapters. Tradeoffs: stylized jump arc, unchanged approximate passive ragdoll
anatomy, graybox model. Six class/civilian cases verify clearance and finite/pass-
through behavior; four car cases verify knockoff. Future rig/authority changes
must preserve consistent appearance and explicit impact categories. Rollback
must retain shared dimensions/palette and the user's no-lean request, not restore
independent body models or universal rider-to-rider launch behavior.

## ADR-018 — 2026-10-30 — Reference steering memory and input-only bank

Problem: the user now wants bike/rider lean together, no lean from curving roads,
a close camera and the C++ reference's increasing turn strength. This explicitly
supersedes ADR-017's upright-rider correction. Source R01 at 4e052aca confirms a
speed-dependent bounded steering-force accumulator with growth/return rates;
whole handling requires missing original SPEC/EXE data. No source-reuse grant
found and no C++/disc data imported. Alternatives: instant constant steering,
copy unknown-license code, invent values and claim exact parity. Rejected.

Selected independent pure SteeringMemory, validated authored/provisional response
fields and fixed player integration; AI unchanged. Both visible bike and human
share one bank; remove road/chassis roll before applying input bank, retain grade
and actual physics. Camera gets velocity feed-forward plus hard chase-distance
bounds and modest FOV expansion; far view is a chase view, not overhead.
Tradeoffs: source-informed behavior is not bit-exact handling parity; physics,
longitudinal/gear/grip/nitro still use existing RoadDash tuning. P03-020 open until
verified authorized calibration and comparative traces exist; user has no files.
Future authority owns steering memory/reset and tuning. Rollback must retain
shared models, no curve auto-bank and bounded view distances; never restore the
superseded counter-roll pivot or label provisional settings as original values.

Latest turn-strength adjustment (P03-022): profile turn response increases 25%
(light 1.5, balanced 1.25, heavy 1.025). Dynamic steering and the assisted
adapter use the same profile gain. Maximum visual bank and the doubled lean
response are unchanged. These remain authored RoadDash settings.

## ADR-019 — 2026-10-30 — Additive structural world foundation (B00–B02)

Problem: the user's first-map design introduces fifteen construction sections,
19 structural archetypes and 247 buildings, while the working game has three
coarse gameplay sectors and uncounted legacy scenery. Alternatives: replace the
road/physics engine, immediately spawn random masses, or treat PDF instructions
as overriding the user's scope. Selected: retain the road and active systems;
add validated rr_content world/archetype/lot contracts, exact seeded planning
allocation, client baked-station resolver and developer-only district preview.
Separate measured legacy census from target allocations; no geometry placement
or completed-world hash claimed. Canonical ID/seed/open-course checks and a
0.25 m endpoint-only snapping tolerance bind data to the real 3D arc length.

Justification: B03–B10 need explicit quotas, identities and station ownership
before parcel/foundation changes are safe. Original supplied heights/counts
are retained; new dimensional/access ranges are provisional authored tunables.
No new library is needed; current serde/Bevy/Rapier suffice. No new external
repository investigated or source copied. Tradeoffs: embedded built-in content
requires rebuild, gameplay's old sector boundaries stay independent, and the
level's visible geometry does not change in this stage. Future placement must
satisfy whole-footprint reservations and shared support; allocation fingerprints
are not spatial-layout hashes. Rollback can disable the additive overlay without
changing track/save/physics schemas. Version any generation-order/ID changes;
retain source snapshots and historical census evidence.

## ADR-020 — 2026-10-30 — Complete the structural corridor without replacing gameplay

Problem: exact quotas need legal lots, supported masses and safe recovery,
while the existing scenery, height sampler and walking controller are separate.
Alternatives: introduce a city/terrain framework and new physics dependency,
flatten hills, skip impossible buildings, or leave visual-only roads. Selected:
original bounded station-frontage placement with oriented-rectangle reservations,
nine authored junctions, typed structural mass/roof kit and one indexed earthwork
sampler. Exact pad cuts join rendered/collision terrain to foundation planes;
outer sidewalks grade to grass. Preserve the authored main road and non-Valley
fixtures. No new dependency needed; existing serde/Bevy/Rapier cover this scope.

Near/far building meshes and terrain proxies use the installed Bevy 0.17.3
VisibilityRange API (R08); all collision remains authoritative and resident.
The geometry fingerprint covers layout/settings/footprints/foundations; planning
fingerprints remain separate. Stable IDs do not depend on renderer entities.
Recovery adds a cached bounded visibility graph and a visible, penalized rescue
for exceptional >120-second/below-world traps, retaining real normal hill travel.
A general navigation framework was unnecessary for a few local static footprints.

Tradeoffs: low-detail masses and simplified solid roof proxies, resident collision
memory, extra LOD assets, finite search/route bounds and platform floating-point
variation. Full-world generation fails with a building/section error instead of
reducing quotas. Future server authority should distribute validated layout and
recovery events rather than assume floating-point lockstep. Art/streaming swaps
must preserve IDs, reservations and collision. Rollback can disable the canonical
world spawner but must not double-spawn legacy and generated buildings or erase
source snapshots/earlier receipts. Acceptance evidence: world/COMPLETION_TESTS.md.
