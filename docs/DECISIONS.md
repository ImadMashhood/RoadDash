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

## ADR-021 — 2026-10-31 — Initial combat/camera correction

Nearby rider bodies obstructed the chase-camera cast. Exclude rider roots while retaining scenery obstruction and existing local-body exclusions. Alternatives include fully disabling obstruction (rejected because buildings remain solid). Mouse punch and Space kick edges use the existing render-to-fixed buffer. Kicks select the closest clear direct-strike target on either side and add lateral velocity without counting a rider punch knockout. No new dependencies or assets. Native camera/contact and kick animation acceptance remains open; revert these small changes independently of world generation if needed.

## ADR-022 — 2026-10-31 — City extensions and controlled riding yaw

The original quota layout did not supply neighborhood depth. Preserve its 247
IDs/quotas and add independently identified city infill and four avenues on
each side, linked by block cross streets and existing junctions. Configurable
bounded city dimensions live in rr_content's construction schema. Every new
lot clears main/secondary streets and existing lots, shares earthworks/pad
support, uses persistent collision and enters recovery routing. Batch street
geometry by material to avoid thousands of draw submissions. Extend terrain
coverage to support the outer neighborhood. Facade windows share cached meshes.
Alternatives: overwrite source quotas or create collisionless skyline scenery;
rejected because they erase history or break reachable-world collision.

Original disc measurements support curb-adjacent 3 m city sidewalks; retain a
2.5 m physical frontage clearance. Tradeoff: a curved corridor creates a
road-relative grid rather than one global Manhattan orientation. Additional
roads are scenery/access, not alternate race routes. No dependencies changed.
Rollback may remove the additions while retaining canonical lots; do not remove
shared collision while leaving reachable visuals.

For riding, bounded angular-velocity steering removes contact-induced yaw
windup while supported. Linear impacts/pushes remain physical; airborne and
ragdoll angular motion is unaffected. This is an arcade stabilization decision,
not original binary parity. Bank is input-only with smooth symmetric visual
poses. Typed kick actions select nearest valid side, push at the center of mass,
and animate the corresponding leg without incrementing the rider punch chain.

## ADR-023 — 2026-10-31 — Shared-edge city infill

The user clarified that city buildings must touch, not merely be more numerous.
Uniformly enlarging rotated boxes leaves wedge gaps on curves and can obstruct
intersections. Instead add solid full-depth building infill joining the exact end
faces of adjacent city buildings. Its roof meets the lower neighboring roof and
its base extends into the shared ground. Preserve all canonical lot IDs and
archetypes. Reject joins crossing main-road/side-street reservations or unrelated
building footprints. Windows are procedural; collision remains resident when
visuals cull. No new dependency, imported art or original-game code.

The union of previously detached buildings changes walking topology. Runtime
navigation includes infill bounds and searches a wider block neighborhood;
interior corners are removed from graph candidates. Audit fixtures must start
at exposed perimeter points, not in former side yards now occupied by buildings.
This is not a runtime teleport or a relaxation of collision checks. Tradeoffs:
infill is additional building volume rather than a new canonical lot; conservative
rectangular navigation bounds may route farther than the rendered facade. Remove
infill and its navigation blockers together to roll back; retain original quotas.

## ADR-024 — 2026-10-31 — Sidewalk shading, street trees and taller city

The user requested improved sidewalks, sidewalk trees and taller buildings.
Use flat mesh normals so curb walls do not round the pavement's lighting.
Batch each main sidewalk into one mesh, sampled every 2 m, with subtle original
procedural expansion joints. Keep existing curb ramps and physical rideable
surfaces. Alternatives were a texture dependency or disconnected slab props;
neither fixes the underlying normal interpolation as directly.

Place original procedural trees every 22 m near the curb, excluding junction
sightlines and structural blockers. Solid trunks and conservative crowns remain
resident while foliage visuals cull. Flush cosmetic beds leave over 2 m beside
the trunk for walking. Shared meshes and materials bound rendering costs.
This adds deliberate physical obstacles, so contact/recovery regressions matter.

Apply an authored 1.6 city floor-count multiplier, rounded up to whole floors,
to urban canonical and supplemental buildings. Derive rendering, facades, roofs,
infill and collision from the same resulting dimensions. Preserve original
source snapshots, canonical quotas, lot footprints and IDs; this latest user
request supersedes the old effective urban height ranges. Validation allows the
scaled archetype range plus one rounding floor. Rural heights remain unchanged.
No dependency or third-party media is added. Tradeoffs: procedural rather than
production art; taller silhouettes may increase visible facade work. Roll back
via height scale 1.0 and removing streetscape spawning together with its tests.

## ADR-025 — 2026-10-31 — Roster, pursuit and physical steering corrections

Use fifteen racers and a stable CITY reference name set. Independently authored
shirt/helmet/bike swatches approximate locally inspected original color schemes;
shared human segments preserve seated/walking/ragdoll consistency. Keep original
RoadDash geometry and bike performance. R01 remains reference-only with unresolved
source reuse permission; no media or source imported.

Raise kick impulse from 3 to 8 m/s; existing decay gives roughly lane-width free
travel. Original combat changes steering words, so a literal SI distance cannot
be asserted. Reduce sidewalk tree frequency from 22 to 44 m. Multiply all authored
turn_response values by 1.5; preserve steering buildup timing and visual bank.

For dynamic players, use body heading for tire force and heading servo instead
of road tangent, and remove automatic road-curvature force compensation. AI keeps
its road-following adapter. Abandoned bikes receive roll angular velocity and a
narrower profile collider; linear momentum and rider separation remain unchanged.
Alternative of a scripted fallen pose was rejected because it bypasses contacts.

Pure rr_core pursuit rules choose nearby targets with start grace, intermittent
engagement and limited downed hunts. Match speed using gap feedback, keep 1.8 m
alongside spacing, and throttle/brake gradually. Downed targets use their actual
runner/torso pose. Traffic avoidance retains priority. AI attack cooldowns exceed
one second. This is reference-inspired behavior, not instruction-level parity.
Tradeoffs: simple deterministic targeting rather than full original relationship
tables; more racers increase simulation load. Revert content, roster validation,
UI counts and pursuit together if rolling back. No dependency migration.

## ADR-026 — 2026-10-31 — Individual relationships and a supported rear grid

The user rejected uniformly aggressive rivals. Replace rider-ID speed/hunting
heuristics with validated level-zero profile observations, normalized net
relationships and the original race/follow/fight threshold structure. Store
numeric profiles in content/rival_profiles.json; pure decisions stay in rr_core,
validation in rr_content. Cache parsed builtins once; avoid per-tick file reads.
Use individual hit responses and decay rather than automatically retaliating.
Shared fallback personalities for CITY spares remain shared; retain distinct
strength/endurance/weight. Keep three-hit knockout rules; apply strength/endurance
to kick force, attack timing and stamina budget instead of changing that contract.
Use weight consistently in attached mass, suspension, forces and upright torque.

Move YOU behind 14 rivals, 12 m apart on both grids. Extend the open approach
road based on the rear-most authored spawn, and project negative approach progress
correctly. Merely moving spawn points would leave unsupported bikes or project
everyone to zero. Bound authored pre-grid extent to 250 m. Legacy tracks remain
loadable and retry falls back safely when fewer slots are authored.

Alternatives rejected: arbitrary cautious/aggressive labels by name, copying
unlicensed C++ or shipping the game executable. No dependencies changed. Tradeoff:
net-score smoothing approximates original three-axis easing, and current gameplay
uses one level rather than original career/random streams. AI-03 preserves these
open parity requirements. Rollback profiles/controller/grid/projection together;
retain physical road support for every authored spawn.

## ADR-027 — 2026-10-31 — Replace presentation while retaining the accepted ragdoll

Problem: the user approved the current physical crashes and explicitly required
that integrating the authored FlushTrim rider and sport motorcycle preserve them.
The supplied art has a 20-bone authoring rig; runtime uses eleven independently
simulated body segments and existing seated/runner visual pose drivers.

Selected approach: a renderer-only eleven-joint skin binding. Bake the authored
standing and seated-0003 surfaces/UVs/weights, folding authoring bone influences
onto the current visual segment entities. Separate inverse-bind arrays allow the
approved seated fit and standing mesh to follow the existing presentation poses.
The physics bodies remain unscaled. The ragdoll merely supplies its interpolated
visual children as skin joints. No animated root motion, colliders from imported
meshes, physics motors, new collision filters, or fixed-step systems are added.
Attach each detached skin below the first body visual so ordinary body teardown
also removes the skin. Pedestrians and recovery runners share the standing skin.
Retain all shirt/helmet/bike swatches and minimap palette semantics.

Bake the original GLBs into a bounded, validated renderer-only buffer; keep source
GLBs and seat/contact metadata separately in art/source. The buffer contains no
physics data. Load meshes/materials once and reuse handles; atlas maps preserve
leather detail, with normal/roughness images loaded as linear data. Existing
chassis/wreck and wheel state drive the replacement motorcycle's presentation.
No dependency changes: Bevy 0.17.3 and Rapier 0.32.0 remain pinned.

Alternatives: adopting the author's whole rig as a new simulated skeleton would
change the accepted physics; replacing physics with animated crash clips would
lose real collisions. Both were rejected. A fully independent 20-bone render
retargeter is possible later, but is unnecessary for this focused substitution.
Tradeoffs: the existing coarse pose drivers remain; fingers, anatomical skin
weight polish, authored transitions and distance LOD are still future art work.
Standing skin and coarse colliders approximate one another; no claim that the
visual surface is a new collision shape is made.

Verification: compare every body collider, mass, damping, friction, restitution,
CCD and joint descriptor between old/new visuals, then compare all eleven body
poses/velocities for 180 fixed ticks. Also compare the complete crash/get-up/run/
remount lifecycle and skin cleanup. Native visual/recovery/performance evidence
belongs in testing/MODELS.md. R08's installed 0.17.3 skinning/asset APIs were
consulted; no new research dependency or reference-game media/code was added.

Rollback: ROADDASH_LEGACY_MODELS=1 selects the old presentation at startup while
retaining identical simulation. Removing the model hooks and assets also restores
the old presentation; no saves, content IDs or physics migrations are involved.
Future animation must consume simulation poses, never overwrite body transforms.

## ADR-028 — 2026-10-31 — Atomic retry, proportionate AI steering and pre-contact velocity

Problems: retry reset logical progress while leaving the authoritative chassis,
velocity and interpolation snapshot at the previous location; rival lane error
was multiplied straight into saturated full-lock commands; crash detection used
pre-solver velocity while detachment could use the later backward solver bounce.

Retry is an explicit PreUpdate control transaction, publishing the authored grid
Transform, zero velocity/force, kinematic/countdown body state, reset BikeState,
recovery and PoseHistory together, including zero-fixed-tick render frames. Reset
Rapier current and next kinematic poses directly as part of that transaction: a
Bevy-only transform reset was overwritten by the existing hierarchical body. Keep
saves, selected camera distance and racer palettes; clear queued attack/look/foot
state. Countdown snaps the chase camera to that new snapshot. This completes the
existing reset boundary rather than moving ongoing simulation out of FixedUpdate.

Only AI lane intent changes: use lane error and measured lateral velocity,
normalize by current tire/turn response, apply a small settled deadband and a
bounded command slew. The player control/force/bank response is unchanged.
Keeping the old saturated controller and merely hiding its motion with cosmetic
lean was rejected: it would retain lateral chatter. Fixed-rate scenarios check
settling and bounded reversal at 30/60/120 Hz.

Impact detachment now uses BikeState.impact_velocity when the victim is mounted;
repeat on-foot impacts still use the runner's actual movement. Never subtract a
car speed and treat the result as ground velocity: 100/60 same-direction is 40
closing, whereas opposing travel is 160 closing. Full contact-normal/mass-aware
vehicle responses, nose-down vault and combat swerve/recenter remain separately
specified open V2-01 tasks. This correction does not retune ragdoll masses,
colliders, joints/motors, tissue damping or ordinary gravity.

Alternatives: forcing all post-impact motion forward or zeroing lateral momentum
would defeat actual contact physics. A broad physics-engine migration is not
justified by these bugs. Current dependencies remain Bevy 0.17.3 / Rapier 0.32.0.
Reference: R01's local race.cpp traffic separation/callback path, studied only;
no C++ copied. Roll back the intent/reset/handoff changes independently using the
new regressions; no save/schema migration. Verification: testing/MOTION_REPAIRS.md.

## ADR-029 — 2026-10-31 — Connected source-anatomy visual skeleton

Problem: ART-03 folded the authored mesh onto legacy box-model segment centers.
Those centers are not the mesh's anatomical joints. Independent legacy limb
translations/rotations could therefore separate knees/elbows or bend them in
unreasonable directions even while the eleven physical joints remained connected.

Export source-model standing/seated bind frames and a validated render-only parent
graph. Spawn eleven nonphysical SkinJoint entities under an avatar render root;
GPU skin uses source inverse binds. Existing visual driver/body rotations feed
forward kinematics: pelvis/chest/shoulder/hip endpoints propagate along source
bone offsets, preserving limb lengths. Elbows/knees are single-plane hinges with
bounded flex (2.35/2.45 rad), rather than unconstrained independent segment spins.
The neutral authored surfaces remain unchanged. Source export/data are original
project art; no external rig/source/media is imported.

The retarget runs before transform propagation in PostUpdate, reading current
hierarchy poses through TransformHelper and writing only SkinJoint transforms.
It spawns nothing in PostUpdate and never mutates a physical body. Scale is one;
rig lifetime follows the original owner/first ragdoll body so cleanup still works.
Changing accepted physics anchors/motors to hide a visual bind error was rejected.
A full twenty-bone animation/finger/terrain-IK system remains V2-02/V2-03 work.

Verification checks neutral bind equivalence, 120 extreme poses per bind, attached
parent endpoints, fixed limb lengths, finite transforms and hinge bounds; passive
body/component/180-tick parity and full crash/run/remount cleanup remain covered.
The full-cycle remount comparison allows 2 mm for f32 projection/entity ordering
(the diagnosed discrepancy was about 0.7 mm), while stage timing, health/speed and
the isolated body trajectory keep their stricter checks. Do not claim universal
bit-identical scene outcomes. Tradeoffs: visual collision proxies and foot plants
remain approximate; gait/foot IK, fingers and authored clips stay open.
Rollback requires restoring both matching skin inverse binds and the old visual
adapter together; never load source inverse binds with legacy segment centers.

## ADR-030 — 2026-10-31 — Reordered execution plan with preserved blueprint history

The user replaces the build order of Parts 01–08: stabilize reported regressions;
reference/label/approve keyframes; interpolate/terrain locomotion/active recovery;
facade art and procedural buildings; props; NPCs then sky/landscape; weapons;
police. Then resume original Part 09 playtest and Part 10 networking gates.

Keep all historical P01–P08/BW entries, statuses and evidence in
roadmap/BLUEPRINT_PARTS_01_08.md; retain the other original IDs in README. Add 54
V2 IDs with reason/dependencies/acceptance/verification in EXECUTION_V2.md. Do not
renumber old tasks, erase unmet gates, call planning implementation, or treat
old graybox counts as production-art acceptance. The sync tool indexes both task
sources and detects duplicate IDs. Migration snapshot/tests protect all 294 old
IDs and completed evidence. Proposed PDF dependencies remain proposals.

Alternative of replacing the old checklist in place would erase history and
break evidence links, so it was rejected. Crosswalks distinguish old milestone
numbers from the current execution order; no runtime/schema change follows just
from moving documentation. Existing public-source/media rights policy remains:
local retail extraction for study, original/licensed runtime art. Revisit the
ordering only through another explicit user decision; preserve history then too.

## ADR-031 — 2026-10-31 — One live checklist, completed work before unfinished work

Problem: the previous README still mixed checked/unchecked items and left older
unfinished requirements in a separate live archive. The user explicitly requests
all completed work earlier and all unfinished work later, following the quoted
crash → animation A/B → buildings → props → NPCs/landscape → weapons → police
sequence, then original Parts 09/10.

Selected approach: README becomes the only live checklist. Part 00 contains all
128 verified tasks in expandable system groups. Parts 01–16 contain all 220 open
requirements, including carried-forward original tasks placed by dependency and
subject. Task IDs retain historical numbering; current section determines order.
The table shows per-part counts and direct navigation. Original Parts 09/10 keep
their purpose and gates. Original production-art criteria in Part 11 are final
integration acceptance, with prerequisite work scheduled in Parts 02–06.

All 348 IDs, states, requirement text and evidence links are preserved exactly
(after normalizing archive-relative URLs). No task was completed or deleted.
Historical BLUEPRINT_PARTS_01_08.md remains a frozen snapshot. sync_progress.py
reads README only; TASK_PLACEMENT.json and README_REORGANIZATION_STATUS.json
record placement and pre-move states. Supporting dependency/environment/audio/
budget/rights prose moves to SUPPORTING_CONTRACTS.md, preserving useful context
without a second editable checklist.

Alternatives: moving only the four recent fixes still leaves older requirements
scattered; renumbering IDs breaks evidence; keeping two live copies risks status
drift. Rejected all three. Tradeoff: README is longer, so completed groups fold
and unfinished sections have a navigation/count table. Broad original tasks may
overlap precise V2 tasks; both stay open until their full criteria pass. Move a
newly verified requirement to Part 00 and update placement/counts/tracker together.

No runtime, dependency, art, save or physics changes and no new repository/asset
research. Rollback is a documentation/tool revert; no migration of game data.
Verification: testing/README_REORGANIZATION.md, preservation/link checks, tracker
unit tests, audit and required Rust checks. This supersedes ADR-030's two-source
live indexing while retaining its original acceptance/history decisions.

## ADR-032 — 2026-10-31 — Input-owned mounted yaw and dissipative arcade crashes

The user requests heading changes from control input, removal of added collision
energy/launches, and 120 km/h rider versus 40 km/h car yielding 80 km/h forward
for both rear and oncoming cases. This explicitly revises earlier clearance-jump
behavior. It does not authorize replacing the accepted ragdoll joints or muscles.

Mounted dynamic chassis lock Rapier's world-Y rotation. The existing steering
response integrates commanded yaw at fixed ticks; readback retains commanded
heading while physical pitch/roll remain available. Contact angular velocity is
never fed back as a steering command. Detached wrecks/ragdolls remain unlocked.
Visual interpolation still cannot overwrite authoritative simulation transforms.

The renderer-independent rr_core::crash rule returns max(rider horizontal speed
minus object horizontal speed magnitude, 0). The native adapter retains incoming
horizontal direction, samples both approach velocities, and uses the same result
for rider and wreck. Opposing 120/40 has 160 km/h closing speed but an explicitly
chosen arcade retained speed of 80; rear 120/40 has 80 closing and 80 retained.
This is not a Newtonian mass/restitution solution. Faster traffic cannot seed a
negative/reversed velocity. Cross traffic uses its actual patrol speed/direction.

Remove scripted height-based hops, collision torque kicks, arbitrary 9 m/s runner
kicks and initial human/wreck spin. Civilian contact shares a stationary-target
inelastic velocity fraction using explicit body masses rather than cloning the
full source velocity into both actors. Wreck release uses a 0.45 rad unstable
lean and zero angular velocity: gravity topples it instead of a synthetic spin.
Its initial potential energy uses that actual tilted COM, not an upright proxy.

Post-solver crash groups use read-back mass, inertia, velocity, rotation and COM
height. Remove negative aggregate approach-direction COM motion, then uniformly
scale linear/angular velocities only downward when kinetic energy exceeds initial
kinetic plus the fall in gravitational potential. f64 accumulation reduces order
noise; individual limbs can still swing backward relative to the torso. The
original body masses, colliders, anatomical limits, motors/damping and gravity
remain unchanged. This is a dissipative game rule for detached crash bodies, not
whole-world momentum conservation: traffic remains kinematic and deliberate melee
shoves remain separate gameplay. Subsequent vault choreography stays V2-01-005.

Alternatives: retaining solver bounce recreates the bug; constant upward impulses
add arbitrary energy; a physically elastic kinematic-car response contradicts the
requested arcade examples; a new engine or rig is unnecessary. Tradeoffs: coarse
vehicle colliders can still stop/pin a wreck. A real over-car vault needs the
separately tracked geometry/contact/animation work, not more launch impulses.

Tests cover player/AI entry cases, real rear/oncoming contacts, energy/gravity/
angular bounds, neutral spin rejection across render rates, all civilian classes,
gravity toppling and slope/recovery. Isolated 11-body/180-tick visual parity stays
exact. Full-scene remount comparison now permits 2 cm rather than 2 mm: observed
contact-order/projection difference after gravity-led wreck toppling was 6–8 mm;
stage timing and health/speed remain strict. This is not bit-identical full-scene
physics. Never silently widen those limits for a later regression.

CI's old soak asserted nonzero yaw during straight throttle and ran off a 1.3 km
fixture floor at ~27 simulated seconds. Correct the neutral-yaw assertion and
extend that isolated floor to cover the possible 600 s path, retaining vertical,
finite/speed and complete-duration stability checks. Add roadmap tests to CI.
No new third-party code/media/dependency. See testing/COLLISION_RESPONSE.md.
Rollback restores heading and entry/envelope changes together; no save migration.

## ADR-033 — 2026-10-31 — Static animation vocabulary before playback

The user authorizes Animation A now and takes responsibility for the previous PR
merge; do not wait for remote CI or merge on their behalf. Preserve the existing
physical rig and collision changes. Part 02 delivers reference study, labels,
original static pose assets and a gallery; Part 03 owns playback/interpolation,
terrain IK, animation events and bounded active-ragdoll control.

Reuse the earlier independently authored 130 numbered mounted blockouts rather
than recreating/renumbering them. Add eighteen original-slot running candidates
and 99 original locomotion/terrain/reaction/recovery/seated/cow layouts: 247 total,
27 semantic use cases. Preserve empty/view/duplicate reference slots in separate
metadata. All poses stay draft; a number is not proof of original sequence timing.
An editable Blender file uses single-frame constant Actions and complete JSON
assembly transforms. No clip timeline, root-motion controller or physical force
is connected to game state. The ten-bone cow is an authoring proxy, not a new
production NPC model. The twenty-bone human authoring rig does not replace the
accepted eleven-body physical rig.

Analytic two-link authoring fits respect fixed bone lengths and elbow/knee flex
limits. It corrected three over-flexed inherited mounting knees and two new
brace/brake hand reaches. The independent rr_content validator rejects nonfinite/
nonrigid/scaled matrices, detached offsets, cycles, bad contacts/references,
wrong skeleton families, unsupported timing and physics/executable fields. The
headless --validate-animation command checks editable files, not asset-count-only
success. Read-only authoring JSON never writes authoritative body transforms.

Source-level research of five pinned candidates informs an evaluation plan only.
No third-party animation code, rig, commercial pack or original media is imported.
Commercial Synty documentation/game leads do not prove gait/terrain algorithms;
actual footage measurement, original descriptor timing and ambiguous labels stay
open. Finger wrapping/fists, skin/cloth clearance, complete get-up/weapon/police
coverage, animal refinement and human approval block full Part 02 signoff.

Alternatives rejected: treating 2D view slots as temporal frames; applying authored
matrices to physics bodies; shipping raw retail media; claiming all 247 drafts are
approved; replacing a working rig with an incompatible plugin. JSON/Blender source
and review renders stay under art/animation, outside runtime assets. The large
editable .blend is a generated local deliverable, reproducible from repository
scripts/source art. Existing gameplay and saves require no migration/rollback.
Remove the authoring module/CLI independently if needed; no runtime system relies
on it. Verification and remaining gates: docs/testing/ANIMATION_A.md.

## ADR-034 — Verify action sequences before reconstructing poses (2026-11-01)

Problem: V1 used inspection-based two-frame helpers as if they established native
punch/kick/weapon semantics. Source playback tables prove that several labels were
wrong; view/mirror variants and repeated phases were flattened into slot names.

Alternatives: keep relabeling visual guesses; replace the runtime animation/rig;
or trace the executable descriptors, CANS view cells and actual action selectors
before authoring. Selected the context-first trace and a new isolated draft set.
The independent reader retains 104 bank/code slots, 194 ordered descriptors,
controls/durations/fallbacks, and all 371 sprite-slot usage entries. Cross-checked
194 descriptor/rear selections against the separately compiled pinned research
library; this is not native executable validation. Unknown callers stay explicit.

Original 3D poses now use semantic action/phase names, with separate authoring
assembly/weapon recipes and static source links. Repeated sprites retain repeated
phases; police/empty art is not invented. V1 artifacts stay as rejected semantic
history and are prominently marked superseded. The shared two-link authoring
math is project code; it never replaces passive physical joints. No gameplay,
physics, dependency or runtime-asset migration. Existing source GLBs are preserved.

Tradeoffs: more provenance/validation files; some source aliases still unresolved,
and draft art still requires silhouette/support/cloth/hand review. No original
pixels or reconstruction C++ imported into production. Reference: arkits/road-rash
cd5504caea48aee59806dbfd11c920f33df21200, reference-only, licensing unresolved for
source incorporation. No code reuse. Rollback: remove context tooling/art without
changing gameplay; never reinstate V1 semantic labels as verified reference facts.

## ADR-035 — Full-body visual interpolation over the accepted physical rig (2026-11-01)

Problem: limb-only keys hid torso/hip motion; static phases snapped instead of
blending from normal riding or a speed tuck. The accepted passive ragdoll must
remain the control fixture. User explicitly requested torso correction followed
by initial interpolation, while general art approval remains unfinished.

Alternatives: replace the physical skeleton with the authoring rig; interpolate
world-space skin frames (breaking bone lengths); or retain a full local hierarchy
solely for rendering. Selected full twenty-bone quaternion FK with immutable
parent offsets, four render-only contact chains and the existing eleven-joint
skin adapter. Physics drivers still own falling/ragdoll presentation. No rigid
body, motor, mass, collision or recovery-stage timing changes. Local clip root
translation is presentation, not authoritative locomotion/root motion.

Fixed simulation stores previous/current pose; render overstep interpolates them.
Attacks capture the actual entry pose and return to the latest normal/tuck base.
Paused/finished state freezes snapshots. Pickup converts between runner and bike
anchor coordinates; bike lift changes only the visual root. Gameplay owns hits,
damage and cooldowns independently of the authoring clock. Phase counters use an
original 0.02 s unit; this is not verified original executable wall-clock timing.

Tuck selection independently implements the pinned reference's boundary rules;
SI conversion derives from the 41/4096 mph HUD coefficient. Ordinary stock flag
mapping is used at runtime; full legacy nitro/flag behavior is not imported.
Source investigation: arkits/road-rash cd5504caea48aee59806dbfd11c920f33df21200,
reference-only; no reference C++ or original media incorporated. No dependencies
added. Tradeoffs: authoring wrists/fists/feet exceed current runtime skin detail;
terrain foot planting, pose-matched landing and final art acceptance remain open.
Rollback: remove MotionPlugin/Bindings and restore legacy presentation; saves,
physical bodies, content IDs and source GLBs require no migration. Evidence:
[Animation B](testing/ANIMATION_B.md).

## ADR-036 — Optional native biomechanical articulation with a preserved control (2026-11-01)

Problem: current ragdoll lacks muscle phases and distal/spine articulation; the
user requests an overhaul for comparison and explicitly forbids merging its PR.
This is explicit physics tuning authorization, superseding presentation-only scope
for this optional mode. The eleven-body baseline remains the default/control.

Alternatives: migrate engines/adopt active-ragdoll-rs; port Unity torque/COM tracking;
or build bounded native constraint motors over existing crash ownership. Selected
the third: installed Rapier 0.31 already provides forces, caps, limits, predictive
CCD, sleeping and island solver iterations. The Rust reference uses Rapier 0.36
and a different glam version; importing it would add a migration and competing
COM control. Unity targetRotation conventions cannot be copied into Rapier.

The new rig adds a spine body, hands and feet without replacing meshes or spawn/
collision/ejection APIs. Rest/active targets, stiffness, damping and torque caps
are validated JSON. Zero activation never disables passive resistance or hard
limits. Full-source bind offsets are removed before using animation rotations;
initial release poses preserve coincident anchors. Later targets act through
native motors only, not transform writes. States/curves belong in rr_core;
validation in rr_content; native adaptation/debug/lab in src/biomechanics.

Source inspection and experiments identified quaternion chart singularities in
wide combined shoulder limits and per-body hard CCD clamping as separate causes
of violent corrections/stretch. Use a safe limited chart and predictive CCD,
60 Hz gameplay with four physical substeps and six additional articulated
island iterations. Keep the original solver path in baseline mode. Damping is
primarily per joint; body damping is a secondary tissue approximation. Activation
fades from bracing/ejection through low control to zero while passive motors remain.
No artificial upright torque or COM animation-following force.

Capture spin before the hidden chassis is stopped; seed coherent point velocities
around the new mass-weighted COM and preserve the established linear response.
Extend the inherited dissipative crash budget with measured initial rotational
energy. Avoid new-mode no-op velocity writes/tiny backward corrections that wake
sleeping bodies. Baseline response and its physical parity fixtures remain intact.

Tradeoffs: sixteen physical proxies feed the current eleven-joint skin; distal
skin articulation needs future art work. Solver quality costs more during many
simultaneous crashes; instrumented headless timings do not certify race p95.
Conservative shoulder bounds trade full reach for stability. Resting/recovery
retain the arcade timing policy. User/native-art/performance acceptance stays open.

References: all five user-nominated projects pinned in
[research/RAGDOLL_BIOMECHANICS.md](research/RAGDOLL_BIOMECHANICS.md), reference-only;
no code/assets incorporated. No dependency or save migration. Rollback: run without
--ragdoll-biomechanical (or press B/reset in the standalone comparison app). Do not
merge or make default without the user's comparison. Verification:
[ragdoll report](testing/RAGDOLL_BIOMECHANICS.md).
