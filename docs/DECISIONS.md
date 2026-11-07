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

## ADR-037 — Independent original-crash arithmetic comparison (2026-11-01)

Problem: the user requested a separate original-game crash comparison, while the
biomechanical proposal must remain unmerged. Alternatives: replace the passive
rig; translate the entire reconstruction; silently keep RoadDash's absolute
speed subtraction; or isolate recovered arithmetic behind an explicit opt-in.
Selected the last option on codex/original-crash-calculations, based on main
5fac329f. PR12/biomechanics remains separate. ADR036 is reserved for that branch.

Pure native integer contact/separation/damage/entry/drag rules live in rr_core.
Original-crashes opt-in uses the original tall-contact rules in modern bike/bike
and bike/car contacts. The independent 3D bridge uses native weights 256/1800,
elasticity 192/256, explicit condition tuning and separate SI axis scales.
Evidence: pinned arkits aa9cb6d and hashed retail x86 arithmetic checks; native
signed +0x108 wins over the reconstruction's scalar car.speed discrepancy.
Source licensing unresolved: no C++ or proprietary bytes copied. No engine change.

Tradeoffs: original launches deliberately add energy; the energy envelope starts
with those velocities and permits reversal only for opted-in contact crashes.
Rapier owns subsequent tumble/contact, so no full native route-geometry, bounce,
traffic reciprocity or per-racer condition parity is claimed. Steering/controls,
three-hit combat, humanoid specs, masses, joints and recovery timing stay under
existing systems. The reference scalar drag is a calculator, not a per-limb force.
Future consequences: CRASHREF-004 owns remaining full-parity work; compare before
choosing a default or combining with biomechanical motors. Rollback: omit the
flag or remove the bridge; saves, GLBs, locks/dependencies need no migration.

## ADR-038 — Offline facade grammar before runtime replacement (2026-11-01)

Problem: establish how original facade references become variable 3D buildings.
Alternatives: stretch complete facade textures, generate opaque image-to-3D
meshes, or author explicit bay/floor rules. Selected explicit editable JSON rules
with an offline Python OBJ/MTL exporter and inspectable geometric modules.
Fixed window size preserves proportions across widths/heights; corner exposures
and connected-row widths are explicit. Existing local 39-name extraction is
recorded as inherited evidence, not claimed complete disc verification. R01
remains reference-only; no source/pixels copied. No new game dependencies.
Tradeoffs: rectangular draft openings/material colors are not final architecture,
PBR art or optimized runtime meshes. Hidden dimensions are authored choices.
Future: add family details, GLB/LOD/collision and integration after review; current
Valley IDs/lots and physics remain authoritative. Rollback: remove authoring kit;
no saves, runtime assets or physics migration is needed.

## ADR-039 — Isolated kilometre city study before race-map replacement (2026-11-01)

Problem: judge dense streets using the new building concepts, which are PNG
renderings rather than model assets. Alternatives: project angled renders onto
cubes, adopt an unrestricted city-growth library, or use a constrained block/lot
grammar with authored geometry. Selected the existing original facade generator,
six explicit BUSS-family design palettes and an exact seeded 1 km orthogonal grid.
A Python offline exporter writes editable box/chunk geometry and lot metadata; a
separate Bevy0.17.3 binary batches each chunk into a vertex-colored triangle mesh.
No new dependency, physics engine or gameplay world replacement. Dimensions and
hidden sides are authored interpretations; these are not reconstructed meshes.

Justification: route-first constraints, fixed street reservations and exact lot
partitions directly serve the requested dense touching streets. Research: CGA
split-grammar paper and phiresky/procedural-cities; no implementation code copied.
Tradeoffs: rectangular details, six families, solid-color materials, no live
traffic/colliders/LOD/interiors; the 146-image catalog remains design reference.
Consequences: review native street-level shots before higher-detail family models
and integration through world_gen's supported pads, stable IDs, collision and LOD.
Rollback: remove the optional preview and regenerate no files; game/save/physics
configuration is unchanged.

## ADR-040 — Bind the actual facade artwork, not color approximations (2026-11-01)

Problem: user rejected ADR039's palette/window prototype because it did not use
the new facade art. That prototype is not valid visual acceptance evidence.
Alternatives: continue reconstructing generic geometry, paste three-quarter
concepts as walls (bakes perspective), or use the existing straight-on new facade
images directly with UV-mapped relief. Selected unchanged enhanced facade PNGs
as real textures: 100 IDs / 17 sheets, explicit UV bounds and source hashes.
Images are not edited or regenerated. The three-quarter concepts still guide
future detailed geometry. No new source repository or dependency is adopted.

Each exact lot now has a shell/roof plus texture-backed facade modules. Shallow
relief changes physical depth while projecting the same source coordinates;
first-six profiles include windows, canopies and bays. End/rear walls reuse the
front art provisionally. Atlas-shared batches keep native draw costs bounded.
A dedicated native view shows the first six designs for direct comparison.

Tradeoffs: source lighting is baked, so facade material is unlit; this is
textured relief, not a full 3D reconstruction. Source sign mistakes remain;
remaining gable/side/roof geometry and most family-specific relief need authoring.
Metric heights are design choices. Production media clearance remains unverified;
read art locally in an isolated preview, never silently add it to shipped assets.

Consequences: source/image identity and UV orientation become tested invariants;
changed pixels require explicit catalog refresh. Generic-window meshes must not
replace the artwork. No simulation transforms, colliders, physics or saves change.
Rollback: revert the preview importer/generator; raw/new artwork stays unchanged.
ADR039's grid layout survives, but its color-only facade approach is superseded.

## ADR-041 — Real first-sheet mesh kit and native GLB placement (2026-11-01)

Problem: both preceding previews were rejected because neither built the approved
3D building designs as actual meshes. Selected independent Blender4.5.1 authoring
of six specific first-sheet buildings, with meaningful geometry for masonry,
openings, shopfronts, arches, signs, canopies, bays, cornices, sides and roofs.
Alternative image projection/relief is retained only as rejected history; an
opaque image-to-3D service would impede editable dimensions and stable LOD.

Source .blend preserves semantic batches/modifiers. Three filtered geometry LODs
export to one joined mesh per GLB, with embedded independently authored procedural
material tiles. No facade PNG or concept image is a render surface. Independent
GLB validator checks buffers, indices, finite position/normal/UV streams, physical
volume, square material tiles, hashes and LOD reductions. Re-imported clay proof
removes all images to demonstrate actual exported geometry.

Native city_preview uses SceneRoot and shared GLB scene handles, uniform placement
scale, camera-distance LOD with hysteresis and existing reserved street/lot layout.
Bevy0.17.3 minimal features do not auto-register scene reflection: explicitly
register static glTF component types in the preview only. A regression clones
static scene components without enabling global dependency features; this fixes
the initial native unregistered-type failure. No engine/dependency/version change.

Tradeoffs: first six models only, cleaner than concepts, prototype dense mesh/LOD
cost, repeated storefronts, small trim intersections and incomplete wear/side
variation. No production-fidelity or runtime-race approval. Other94 facade designs
and46 structures remain explicit open requirements. Modelled dimensions/hidden
sides are authored interpretations. Blender is an external GPL tool; no software
source incorporated, original geometry/material-generator code is project-authored.

Future: finish remaining families, review fidelity/scale, optimize chunk/collider/
navigation/LOD intake through world_gen without changing canonical IDs/pads.
Rollback: switch/remove isolated viewer and kit; default game simulation, saves
and accepted ragdoll stay unchanged. No animation or physics architecture change.


## ADR042 — Complete typed architectural catalog (2026-11-01)

Problem: the six accepted models cannot represent the remaining 140 concept IDs;
facade imagery on proxy boxes was rejected. Alternatives: image cards, unbounded
monolithic authoring scene, or typed per-design geometry batches.

Selected: explicit 146-ID design plan; retain the first six byte-for-byte; author
94 additional facades, 28 structures and 18 surface modules with reusable mesh
vocabulary. Each sheet gets editable semantic source, individual/group actual
renders and three joined-mesh GLBs with original square material tiles. Separate
manifest reconciliation and binary validation check coverage and hashes. Surface
samples remain modules, not fabricated buildings. Bounded per-sheet Blender
processes limit authoring memory and permit targeted rebuilds.

The isolated Bevy viewer uses all facade families, uniform dimension-aware
placement, lazy detailed scene loading, coarse shared handles and 25 paged gallery
views. No racing physics, accepted ragdoll, dependencies or canonical world_gen
placement is modified. Existing street/lot reservations are preserved.

Tradeoffs: deliberately draft material/fidelity, shared side/back vocabulary,
prototype GPU cost and non-manifold overlapping trim. Full collision/navigation,
artist approval and production profiling remain separate gates. All 146 IDs have
source/render/export evidence, not blanket art approval. Existing Bevy/Blender
references only; no new third-party source or assets incorporated.

Migration/rollback: remove/switch the isolated preview/catalog, preserving the
original six source/manifest. Rebuild affected sheets explicitly after generator
changes; resume presence alone does not certify current generator equivalence.


## ADR043 — Separate authored district roles from original course evidence (2026-11-01)

Problem: grouping every building into one random pool obscures neighbourhood
identity, while naming a district from appearance cannot establish original usage.
Alternatives: filename-only categories, inferred exclusive track assignment,
duplicating model files per location, or separate typed metadata views.

Selected: exactly one primary RoadDash district/support set per146 IDs; separate
many-to-many original-course family eligibility measured from named CEL membership
and MSRR types1/3. Thirteen sets include shared services/utilities/structural trim;
five course views preserve overlap. Group manifests link the unchanged canonical
GLBs using relative paths; Unix folder links avoid duplicate files. Existing
Python art-authoring tools generate indexes and contact sheets offline. These are
authoring views, not new runtime domain rules or an rr_content schema migration.

Evidence: all146 names match the local FAMILIES.RSC. Source course/family and
external local decompression-tool hashes are recorded; only numeric/text findings
are retained. Existing pinned arkits reader is REFERENCE ONLY; no source copied.
The local Python decoder is executed for research only, not vendored or required
for ordinary grouping. No new dependencies or media imports.

Tradeoffs: family load eligibility does not establish selected texture/placement,
route order, frequency or guaranteed all-level visibility. District names/primary
roles and placement guidance are authored interpretations. Current preview and
canonical world placement are unchanged. Art/release/production gates remain open.

Future: apply these pools through reserved block/site grammar with coherent
transitions and existing whole-lot/ground/collision/navigation constraints.
Rollback: remove/regenerate only district indexes/links; mesh hashes, game,
physics and saves are unaffected. Non-symlink platforms can use JSON/CSV paths.


## ADR044 — Independent whole-building assembly and playable 5.3 km City Run (2026-11-01)

Problem: the user clarified that the actual race, not only the1km preview, must
become a5.3km generated map using the modeled facades. Repeating narrow full-depth
catalog models wastes hidden walls/roofs and does not create coherent whole lots.
The reference is a projected facade/roof/end/side-street renderer, not a ready-made
modern procedural3D city generator.

Selected: independently derive300 genuine geometric front modules from our own
100×3LOD GLBs, retaining front triangles/materials and shifting the street face
to localZ0. Original438 catalog GLBs/sources remain unchanged. Use85 measured
City-family-eligible facade IDs in eight district zones. Per-lot front/return runs
join one shared full-depth shell/roof/floor with segmented rear-wall openings,
window frames, downpipes, service door, chimney and rooftop plant. End walls do
not cover the returned facade apertures. Street reservations precede lot filling.

One deterministic offline generator bakes shared CityPlan data and the exact
5,300m open track. rr_content validates dimensions, contiguous districts/frontages,
whole-lot road clearance and finite geometry. Runtime assembly/rendering lives in
world_gen::city; both preview and actual race share it. A named architecture asset
source preserves the existing rider asset root. Detailed facade LODs are lazy;
far scenes are hidden beyond fog distance, with a separate developer overview.

Road/sidewalk support and independent fixed body colliders exist before riding;
all977 roofs receive Rapier ray verification. Starting grid apron extends beyond
the fifteen negative-distance spawns. Existing crossing-car code is factored into
a shared spawn helper without changing actor physics. Existing navigation solver
accepts merged city row footprints; physical recovery remains authoritative.

Alternatives rejected: image cards; copying old source; stretching complete slim
buildings in three axes; one global random facade pool; changing the accepted
ragdoll/engine; silently shortening the old Valley configuration. The new track
becomes default; ROADDASH_TRACK=valley preserves the full earlier world and its
quotas/IDs. Existing player/rider/bike bodies, joints, forces and recovery stages
are not retuned. The user explicitly confirmed the playable-race scope.

Tradeoffs: straight point-to-point city avenue; side roads create reserved grid
blocks but do not define alternate race routes. Exterior assemblies have no
walkable interiors; colliders are collision-independent solid body proxies,
with ornamental trim/stoop/contact fidelity retained as an art/collision gate.
Source-draft art remains local development content; runtime use does not establish
final visual/IP approval or permission to redistribute original retail pixels.
No retail media/code is incorporated and no new dependencies are introduced.
Benchmark/hardware/production acceptance stays open. Rollback: select Valley;
remove city plugin/data without changing original track/world/model files.

## ADR045 — Route downed contacts separately from mounted wipeouts (2026-11-01)

Problem: recovery::impacts treated every foreign OwnedBody as an immediate mounted
wipeout above0.75m/s, including each limb of an already fallen rider and its wreck.
This created pile-ups unrelated to the original contact classes.

Research: arkits/road-rash race.cpp at aa9cb6d3181e20736af71022cd4d6aac0e2788d1,
Race::contact (1818–1950), contact_static and contact_half_static. Original low
objects route before containment/bike/tall/glance handling, avoiding ordinary
knock_off. Detached people have their own callback. Reference only; no code copied.

Selected: independently implement explicit low/bike/tall/glance policy in rr_core.
Downed rider/wreck contacts retain solid Rapier response without scripting another
mounted crash. Actual low geometry can support wheel rays; assisted motion raises
its sweep and re-tests clearance so tall obstacles remain solid. Standing runners
remain distinct and may be knocked down again. Mounted side glances nudge; direct
rider contacts retain the existing50km/h closing-speed gate. Ordinary tall-object
and car responses, including the existing80km/h entry example, stay unchanged.

Alternatives: disabling downed colliders would remove real contact; treating every
limb as a standing rider repeats the bug; enabling original scripted launches would
undo accepted COLLISION-02/03. No engine, body-spec or joint changes are needed.

Tradeoffs: metric low threshold0.8m and projected collider containment are modern
adapters, not original integer/height-unit parity. Dynamic fallen objects remain
solid and can slow/block a bike; airborne debris is not promised to have exact
retail behavior. Preserve separate original-crash comparison mode and its open
parity gates. Passive ragdoll specs, masses, motors and damping are unchanged.
Rollback: remove the new contact routing/support predicate, restoring the prior
scripted wipeout policy without changing assets or saved data. No dependencies
or assets added.

## ADR046 — Exhaustive prop reference banks before new geometry (2026-11-01)

Problem: same-process-as-buildings must cover pedestrians, traffic and wider props,
without missing non-building category paths or presenting sprites/concepts as meshes.
Alternatives: list names heuristically; replace runtime primitives immediately;
scan every family plus all course vehicle/global containers and retain uncertainty.
Selected the last: independent bounded CEL/hierarchy/frame reader, existing external
local decompression only, stable source/pixel-derived IDs and complete source-offset
ledger. Pixel-identical libraries collapse with aliases and frame order retained;
color/view banks stay distinct until reviewed. Source alpha previews are illustrative.

Local3x2 sheets and searchable HTML expose every indexed image and unknown entry;
metadata-only inventory and explicit per-asset stage queue live in the repository.
Known architecture/materials cross-reference the earlier catalog rather than become
new prop meshes. First six bitmap concepts use the existing building art direction.
Metric dimensions, hidden geometry and original placement are not inferred facts.

References: pinned arkits/road-rash family/animation/course_scene/traffic code;
REFERENCE ONLY, root reuse license unresolved. No source copied or dependencies added.
No original pixels enter game/runtime; generated concepts remain unapproved draft art.
Accepted physics/recovery untouched. Future visual meshes must retain independent
collision and physical rig parity. Tradeoff: more explicit records, manual archetype/
unknown review still necessary;3521 non-CEL frame references do not become invented
poses. Rollback: remove authoring tools/metadata/concepts; game and saves unaffected.

ADR046 concept follow-up: batch02 uses six source-subject sheets plus the existing
building concept sheet as explicit separate subject/style references. Output is
raster concept art only. Stable IDs, per-cell mappings, prompts and final hashes
remain independent of review numbering and of later mesh/physics approval.
Two imagegen typography edits remove copied source metadata; original drafts stay
available. DEER2/BIGFNCN2 visual corrections demonstrate why filename heuristics
cannot authorize anatomy, collision or modeling decisions.


### 2026-11-01 — Full prop concept coverage within ADR046

Problem: sample sheets left most source banks without new art. Alternatives:
collapse similar filenames first, or preserve every scoped reference ID through
bitmap generation. Selected: frozen 330-ID remaining manifest plus prior 39 IDs;
59 source-mapped sheets and a full gallery. Filename guesses are overridden by
visual source inspection, with composites and sign backs retained. This costs
more concept review but prevents hidden omissions. No new dependency/repository,
retail source import, mesh generation or gameplay migration. Rollback is removal
of generated-art references; existing physics/render assets are unaffected.
Generated coverage is independent of art approval and actual 3D export.


## ADR047 — Separate draft prop geometry from art acceptance and runtime replacement (2026-11-01)

Problem: the generated concept sheets are bitmaps, not reusable 3D geometry.
Alternatives considered: textured image cards (insufficient volumes), replacing
live entities immediately (would conflate authoring with accepted physics), or
a separate editable GLB catalog. Selected: original Blender4.5.1 semantic recipes,
three geometric detail levels, small independently authored material tiles, source
collections and proof renders from re-imported GLBs. Reuse the existing Bevy0.17.3
static-scene registration plugin in an isolated prop_preview binary. No migration,
new dependencies, reference repository code or original game pixels incorporated.

Tradeoff: complete source-ID draft coverage is possible with shared recipes, but
this does not reproduce the concept artwork faithfully. Actor anatomy/clothes,
vehicle shapes, detailed foliage, pictograms and material finish need substantial
art refinement. Human/quadruped skinning remains absent. Conservative bounds are
authoring metadata, not permission to replace runtime collision shapes or bodies.

Future consequence: production intake must verify art, rigs/animations, semantic
colliders and placement separately. Existing body specs, joints/motors, mass,
momentum, recovery, saves and race content stay untouched. Rollback: remove the
isolated catalog/viewer references; no game state or asset migration to reverse.


## ADR048 — Present draft props through existing physical owners (2026-11-01)

Problem: the exported prop catalog was isolated from racing. Alternatives: replace
physical entities from Blender bounds, or attach presentation to accepted owners.
Selected: a validated 369-record catalog and props:// asset source. Moving traffic
and cross traffic retain their colliders, paths and velocities. City trees/lamps
retain established collisions; new furniture occupies tested semantic sockets
outside the road, crossing and pedestrian lane. Parked cars use parallel streets.
Existing Valley structural IDs/quotas and pad support are unchanged.

Human/cow source GLBs receive separate, generated visual-only skins (11/10 frames).
Existing walking pivots and interpolated physical body frames drive visual bones;
no motor, mass, joint, ejection or recovery parameter is exported or changed.
Skin updates write only non-RigidBody transforms after visual interpolation.
Three static LODs and two actor LODs switch with hysteresis; distant instances hide.
Loading waits on scene dependencies. No new dependencies or copied reference code.

Tradeoffs: rigid per-part weights preserve the draft silhouettes but are not a
finished deformation rig; seams, anatomy and material fidelity remain art work.
Contact trajectories are not bit-identical across ECS layouts because insertion
ordering affects iterative contact solving; body construction and ejection velocity
are tested separately from bounded contact simulations. Performance certification
and final art/release approval remain open. All catalog records are addressable,
not all 369 banks are spawned indiscriminately. Native receipts list selected IDs.

Rollback: ROADDASH_LEGACY_PROPS=1 restores prior presentation and omits added city
furniture. Source static GLBs and accepted rider/bike presentation remain intact.
No save schema or physics-backend migration. Local development source falls back
to art/props/mesh_assets; standalone packaging must supply assets/props beside the
binary and separately satisfy the existing production release gates.

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


### 2026-11-02 — Comparison branch defaults to the requested new rig

Problem: the user expected B to enable new physics during racing, but that key
only belongs to the standalone lab. Alternatives: add live race rig swapping
(unsafe for existing articulations), or select the new rig on startup. Selected:
shared game/lab startup selection, biomechanical by default in this unmerged
branch; explicit --ragdoll-baseline preserves comparison, CLI overrides environment.
The user explicitly authorized this branch default. Main and all physical tuning
stay unchanged. Tradeoff: old bare-command baseline instructions change; updated
docs describe the override. No new repository, dependency, asset or save migration.
Rollback: explicit baseline flag/environment or revert startup selection alone.


## ADR049 — Integrate approved biomechanics with the current race (2026-11-06)

Problem: the reviewed ragdoll branch predates the city, props and original contact
routing. Alternatives: merge without adaptation (loses either newer behavior or
original reverse-response policy), or combine physical articulation with current
owners and visual skins. Selected: the latter, with launch defaults from the
user's2026-11-02 request and merge approval from2026-11-06. Preserve native original
crash comparison, including allow-reverse policy when rebuilding energy envelopes.
Retain props'11-frame human skin drivers over16 physical proxies; cows retain their
existing10-body articulation. Both old/new body paths remain tested and selectable.

No engine/dependency migration or additional source/media import. Historical
comparison-only restrictions are superseded by the user's merge instruction.
Tradeoff: physical wrists/feet/spine still share the existing visual rig; RAG-05/06
art and broad performance acceptance stay open. Rollback: --ragdoll-baseline;
no save migration. Preserve city/prop authoring and quotas. No new repository was
investigated; reuse pinned research in research/RAGDOLL_BIOMECHANICS.md.


### ADR049 addendum — Leg resistance and requested ride-over contacts (2026-11-06)

The user additionally reported leg crossing/weak resistance and requested hops
for rear/downed bike/rider impacts. Mirrored hip medial stops are0.08rad, axial
hip twist±0.22rad; passive hip k/d/cap=12/3.5/28 and knee=18/4/32, with active
hip60/8/65 and knee65/8/65. Knee single-X hinge limits remain0–2.44rad. Four substeps
and eight extra articulation iterations keep the revised high-speed limits bounded.
Do not substitute global damping for joint resistance. Original eleven-body baseline
stays an explicit control; the user's screenshot was labelled baseline.

Rear/downed contacts now use an explicit arcade hop at closing>=3m/s: upward
speed2.8+0.035*closing, capped4m/s (~0.82m ballistic rise). Preserve native horizontal
momentum transfer, charge scripted target pushes to the attacking bike's momentum,
apply a0.35s cooldown, and retain solid colliders. Fast rear contacts still knock
the target off; gentle contacts nudge. Side/head-on/car routing stays separate.
This requested vertical assist is a deliberate gameplay-energy exception, not a
claim of original crash arithmetic parity. No arbitrary crash launch was added
for other classes. Integration tests cover physical contacts and both entity orders.


## ADR050 — Part 03 visual articulation and on-foot authority (2026-11-06)

Problem: authored attack sides opposed gameplay's forward-cross-up right vector;
fixed elbow poles flared during tuck/lean; time-only running ignored blocked
motion; generic get-up discarded the landing pose. The pilot's eleven skin
joints concealed the accepted physical rig's five additional articulations.

Alternatives: replace the physical rig, import an IK/animation dependency, or
extend existing visual FK/two-link constraints and fixed recovery. Selected the
last. No dependency/engine/asset replacement: Bevy0.17.3/Rapier0.32.0 remain pinned.
Correct side selection at the presentation boundary; bounded inward elbow poles
retain held grip endpoints and authored chain lengths. Sixteen skin matrices and
smooth distal weight remapping are generated at startup from original binds;
source model/texture files are untouched. The prop pedestrian rig remains separate.

Animation reads interpolated physical joints, captures the settled pose at get-up,
blends into existing authored recovery, and fits soles to support normals while
locking stance feet. Stride follows measured travel, with a velocity stop blend.
All targets write SkinJoint transforms only, explicitly excluding rigid bodies.
No root motion changes simulation. Production pelvis/stair/contact quality and
confined get-up clearance remain open rather than inferred from numeric tests.

Pure rr_core FootControl owns manual/assist/finished priority and neutral delay.
Validated rr_content recovery tuning owns speed, stride, mount and rescue bounds.
Fixed recovery owns character movement, checkpoints and a shared racer-level
finish guard/payout. Manual movement wins over bike seeking; neutral assist waits
0.35s, or remains disabled via --manual-recovery. Both stick axes are used. Manual
mount requires interaction, proximity, facing and a slow bike. Below-terrain rescue
is explicit, penalized and clamped before the finish; fully enclosed traps remain
open. No save-schema migration; existing progression has one completion path.

Optional player action while supported and at least0.8s into a crash raises only
native capped joint motors to0.12 activation. It applies no direct root impulses,
teleports or limit changes and returns to the ordinary curve on release. This is
a bounded brace prototype, not a balance/recovery simulation or energy-budget
certificate. Existing baseline and no-input trajectories remain regression controls.

Previously pinned research (research/ANIMATION_AND_RAGDOLL.md and
research/RAGDOLL_BIOMECHANICS.md) informed the target/physics separation. No new
repository/source/media imported. Tradeoffs: pilot skin has five extra matrices;
IK probes cost at most two rays per recovering rider per tick, source-key get-up
is still an approximation, and production profiling/user acceptance remain open.
Rollback: revert these visual/recovery changes; --ragdoll-baseline and
brace_activation=0 preserve comparison options without changing saves or content.

## ADR051 — User camera, rear-contact, locomotion and crash-style follow-up (2026-11-06)

Problem: transient actors shortened the camera boom; a ceiling/start penetration
could collapse it onto the focus and make look-at unstable. The earlier requested
rear hop now contradicts the user's latest60km/h push/knockoff rule. The source
run clip and raised elbows were rejected; distal skin neutral frames also differed
from animated parent frames, visibly bending otherwise neutral wrists.

Selected: keep the static camera obstruction sphere, filter dynamic/kinematic,
owned and NPC actors, try horizontal/lower boom candidates under low ceilings,
and retain orientation for degenerate look vectors. Recovery holds its initial
chase heading instead of snapping to a different nearest road tangent while on foot. Static walls still block the
camera. Alternative aggressive near-focus look-at/zoom was the reported defect;
ignoring all scenery would clip buildings. Actor render occlusion can still occur;
this does not turn moving people/cars into camera walls.

Upright same-direction rear contacts always share normal velocity using pair
masses (bike plus70kg rider). This conserves normal momentum and dissipates energy.
Closing>=60km/h knocks the front rider down with that shared response; no scripted
hop. Apply resolved pre-contact velocity to both native bodies instead of retaining
the solver bounce. Downed-body/wreck lift, side/head-on/car and original integer
calculation tests remain separate. This user override supersedes ADR049's upright
rear-hop portion, including the modern optional original-contact bridge; it is not
claimed to be retail arithmetic parity. Native damping/handling otherwise retained.

Replace running clip playback with a continuous distance-phase gait: opposed foot
stance/swing, knee lift, counter-rotating shoulders/pelvis, arm swing and vertical
weight shift. Keep stable forward knee poles during planted two-link fitting.
For wrists/feet/spine, derive each neutral local frame relative to its animated
parent and apply local motion delta there. Carry wrist pivots with forearms; no
extra standing wrist roll. Earlier world-reference calibration mixed authoring and
animation frames and caused the visible kink. Bike grip fitting and physical body
transforms stay independent from this visual correction.

User-authorized crash motion: a one-shot supported sliding roll starts after0.3s
at>=3m/s. Axis is up cross horizontal travel (forward somersault tendency or lateral
roll). Cap angular cue2.2rad/s and pay its full kinetic cost from at most8% of the
current horizontal COM kinetic energy; ground contact supplies momentum reaction.
Preserve vertical momentum and forward direction. No free-air assist, repeated
energy source, teleport, body/joint/solver migration or baseline modification.
Frontal speed loss>3m/s arms a0.38s hip/knee release: hip+.25rad, knee+.65rad,
activation floor<=.35 through existing capped motors. Existing energy envelope
remains active. These are deliberate arcade cues, not an unconscious-body model
or a guarantee of a complete somersault against every obstacle. Alternatives:
free angular kicks rejected for energy gain; pose-only roll rejected because it
would conceal physical collision motion. Both changes retain native solid contacts.

Restore original6.85km Valley as default; explicit city/valley aliases preserve
both courses. No regeneration/quota/asset or save-format changes. No new dependency,
third-party repository investigation or code/media import. Original validated
content/crash_motion.json centralizes style tuning. Set roll_energy_fraction=0 or
release_activation=0 and rebuild to disable those cues; --ragdoll-baseline remains
the physical control. Future consequence: art/contact/energy tuning and platform
acceptance remain explicit gates rather than a claim that Part03 is complete.

## ADR052 — Reusable authored storeys on immutable Valley lots (2026-11-06)

Problem: restoring Valley as the default exposed its old generic building shells,
while the authored 3D facade drafts were only integrated into City Run. Stretching
whole facades to tower heights would distort windows; loading the entire catalog
and every floor at once would create excessive geometry and scene entities.

Selected: derive ground, repeatable upper and crown scenes from our existing
100 facade/28 structure drafts. Preserve original assets and source hashes;
share identical own-authored texture bytes and deduplicate exact vertex tuples.
Use the source generators' semantic floor boundaries. Typed rr_content validation
owns catalog/rule intake; world_gen owns deterministic lot assembly and rendering.
All 19 archetype pools use stable section/block/role and lot selection. Original
canonical and neighborhood layout, rights of way, foundations and Rapier colliders
remain authoritative. Scene geometry never introduces colliders.

Alternatives: whole-model vertical scaling rejected for floor distortion; global
unbounded scene spawning rejected for memory/frame cost; new city/layout package
or engine migration rejected as unnecessary. Reuse existing architecture asset
source and explicit Bevy glTF reflection support. Original clipping, reassembly
and streaming code; no third-party implementation copied or dependencies added.
Previously recorded Bevy and existing building-source investigations are reused.

Tradeoffs: exterior-only solid shells and adapted bay/ground heights remain;
structure fronts and U04 industrial cladding are not bespoke parking/interiors.
Limit near detail to camera-facing faces and reserve budgets for pending loads.
Existing complete near/far shells keep coverage. Coarse ornament shadow draws are
removed while structural shells retain shadows. Hysteresis and a 48-file-LOD cache
bound churn, but production pop-in/memory/performance acceptance remains open.
The initial 1,200-panel survey missed the 16.7 ms wall-frame p95 target (19.36 ms),
so the default is reduced to 750 panels with separate triangle/building ceilings;
final evidence records actual results rather than inferring FPS from budgets.

Consequences: native binary mesh census, per-band envelope tests and full route
survey are required alongside allocation counts. Rendering quality can be tuned
in architecture_rules.json independently of 60 Hz gameplay. Rollback: remove the
visual kit hooks and restore generic facade material; no save migration, collider,
track, player or ragdoll state changes required. Part04 art/exit gates stay open
until accepted; no automatic progression to Part05.

## ADR053 — Surface detail, stable facade swaps and unseen crash recovery (2026-11-06)

User scope: improve plain high-rises/roads, reduce building texture pop-in, and
skip physical crashes behind the player in favor of a five-second delay followed
by a road-centre respawn. Merge the building work, then explicitly start Part05.

Use original batched window reveals/mullions, floor bands and cornices on fallback
shells and party-wall infill. Infill uses its first owner's masonry family; tiled
material sampling avoids stretched brick. Keep the physical lots, joins, ground,
road topology and colliders unchanged. Original CPU-generated asphalt supplies
aggregate, repair/tar detail, subtle wear, normal/roughness maps and complete mip
chains; world-scale UVs apply once to main/junction/grid roads. No downloaded or
AI-edited texture and no additional dependency. These are draft exterior surfaces,
not a claim of artist approval or new navigable interiors.

Keep the used coarse facade LODs and masonry resident (finite catalog bound), with
at most48 extra high-LOD cache entries. Hold the old displayed LOD during loading;
prepare its replacement hidden and swap only after SceneSpawner confirms every
instance is ready. One hidden group per selected lot is allowed, separately bounded
by the panel ceiling. Reserve displayed as well as desired triangle budgets.
Use25m spatial hysteresis to reduce budget churn. Avoid an unbounded global scene
spawn or a transparency/collision rewrite. A discontinuous LOD remains possible;
this fixes missing-scene gaps and texture reloads rather than claiming perfect
morphs/crossfades. Detailed fallback surfaces remain across the skyline.

The expanded eager load reproduced a native IO worker stack overflow. Before
DefaultPlugins, initialize only IoTaskPool with a16MiB explicit worker stack;
preserve Bevy0.17.3's default25% cores/min1/max4 thread policy. Compute/physics pools
are unchanged. Earlier RUST_MIN_STACK workaround affected all threads; configuring
only the named failing asset IO pool isolates the resource requirement. This is
an explicit stack budget, not a claim that upstream loader stack use was reduced.
Record clean native runs without an environment override; the optional city gate
is separate until it too runs. No whole-engine or task-executor dependency change.

An NPC mounted crash is abstracted only when>25m away,>15m behind the player's
forward plane and behind the camera plane. Player, nearby, visible and already
physical/on-foot crashes retain existing accepted physics. At central crash entry,
allocate no human bodies/joints/wreck; disable the existing bike body/collider and
hide it. After exactly300 active60Hz ticks, project the recorded impact origin onto
the road and restore the existing racer at lane0 with zero velocity, full health
and ordinary short immunity. Preserve checkpoints/progression; no forward teleport.
Pause freezes the timer; restart cancels it and removes disabled-body state.
This is an intentional offscreen arcade approximation authorized by the user, not
momentum simulation. Reversing the camera does not retroactively spawn a deferred
ragdoll. Existing already-active ragdolls are not removed when they leave view.

Alternatives: full physical NPC crashes rejected for requested cost saving; mere
sleeping rejected because it still allocates the articulation; timer-only visible
crashes rejected for obvious popping. No force/motor/mass or player recovery tuning.
Rollback: remove the classification system/early branch, keep normal recovery;
revert visual hooks independently. Original mechanics/source code, no new external
repository, copied code/media or dependency. Part04 broad art/performance gates
remain open even though the user authorizes moving to Part05 after integration.

ADR053 domain boundary: rear/camera interest thresholds and the300-tick delay live
in rr_core::performance; the Bevy adapter only captures transforms and performs
entity lifecycle changes. This is an offline NPC approximation. Future server
authority must own interest/respawn policy rather than trust a client camera.

## ADR054 — Soft actors and downed-bike ride-over preserve entry motion

2026-11-07. Problem: solver-stopped bike velocity was reused for ride-over,
so pedestrians/recovering riders could remove all forward speed. Selected:
save entry motion before physics; use mass-weighted normal transfer for soft
actors, knock the target down and retain the attacking rider. Downed-body/wreck
push is bounded; rr_core charges lift against remaining pair linear energy and
prevents reversal. Alternatives: unconditional impulse/speed restoration rejected
because it creates energy; wiping the attacker rejected by the user's policy.
Cows, upright rear-bike contact and car crash rules remain separate. Tradeoff:
arcade soft contact with a bounded clearance hop, not a complete deformable-body
impact model. Existing body/joint/mass/motor settings remain intact. Native adapter
owns lifecycle, pure contact rules own arithmetic. No new dependency/reference
code. Multiplayer will need authoritative entry velocities. Rollback the routing
and response adapter independently of accepted ragdoll articulation.

## ADR055 — Bounded material detail, temporal facades and semantic dressing

2026-11-07. Problem: abrupt architecture changes, flat materials/sky and sparse
world props. Alternatives: duplicate every material with alpha and double all
geometry rejected for memory/draw cost; full custom renderer/weather dependency
rejected for scope. Selected Bevy0.17.3 native visibility-range dither over 0.30s,
retaining ready authored scenes and one displayed group per lot. Keep a detailed
fallback during replacement and enforce actual display budgets during retirement.

Original palette hashes identify34 textures. Shared PBR microdetail and missing
tangents are prepared asynchronously, at most two jobs per queue; tangents change
only tangent attributes. Source colors, physical geometry and skin weights remain.
A typed semantic prop plan protects full lots/access/roads/support and uses existing
licensed draft models with solid proxies. Sky is original seamless procedural
RGBA8 cube data with mipmaps, diffuse environment, bounded sunlight/exposure/fog.
Tradeoffs: additional loading/memory, approximate roughness environment, dither
noise and no volume/cloud-shadow simulation. Full asset-specific texture painting,
production art acceptance, sound/category QA and broad performance remain open.
No new engine/library/repository or copied media. User explicitly authorized this
sky follow-up before the full NPC/landscape exit. Rollback each cosmetic module
without changing saved state, physical building quotas or ragdoll systems.

## ADR056 — Biome-specific roadside clusters and independent seeded variation

2026-11-07. Problem: the15-role cycle placed individual objects and mirrored
left/right furniture, while field-edge objects were too distant to enrich the
riding view. Alternatives: globally increasing tree/prop density rejected for
repetition, clearance and performance; new asset packs unnecessary because the
registered original catalog already contains suitable models. Selected:35 roles
and20 authored small groups, with a separate near-road farm zone, city transit/
post/civic/service groups, rural loading/service/scrub and hill pullouts/timber.
Each member retains full footprint, support, junction, access and existing-object
checks. Small sidewalk offsets/yaw remain confined to the furniture band.

Use the track scenery seed and stable zone/site/member hashes for independent
left/right group, model, spacing, size and heading variation. Shuffle site priority
within each zone and reserve a fair share of the224-prop cap so early farms/city
cannot starve the hills or only fill the beginning of each district. Groups may
lose individual members when clearance rejects them; correctness takes priority
over complete vignettes. Vehicle groups represent solid parked scenery, not new
traffic/AI. No additional trees or gameplay ragdoll changes. Native counts and
whole-route samples are required; authoring counts alone are not placement proof.
Tradeoff: up to64 more static owners than the previous cap, with existing prop
LOD/distance policy; broad performance and finished prop art remain open. No new
repository/dependency/media. Rollback only dressing grammar/placement; map quotas,
collision rules, saves and accepted rider behavior are independent.

## ADR057 — Crash behavior research and deferred implementation

2026-11-07. Problem: requested swerve/release, protective arms and roll-to-back
crashes need an evidence-based approach. User explicitly requires findings before
implementation. Selected action: research/document only on a separate branch after
merging PR17; preserve all runtime/tuning/assets/dependencies unchanged.
Alternatives evaluated: adopting active-ragdoll-rs directly (version/policy mismatch),
copying Godot/Unity controllers (engine and motion-policy mismatch), replacing
Rapier (unnecessary), or adding behavior around the current native motors.
Recommendation, not an approved migration: continuous seated release, real support
sensing and bounded target-pose behavior over current constraints. Tradeoff: a
custom small behavior layer needs fixtures/work accounting, while a wrapper adds
translation tracking/type boundaries and threatens the accepted physical baseline.
Sources/licenses/actual techniques: research/crash-behavior/README.md. No code or
media incorporated. Future consequences: preserve baseline comparisons, explicit
state transitions and authoritative fixed-tick inputs; no lane/body teleport or
unbounded energy. No runtime rollback is needed because nothing was implemented;
any later prototype should remain independently reviewable before default merge.

## ADR058-B — Reciprocal angular-impulse muscle comparison

2026-11-07. User authorized two isolated implementations for A/B review after
research, with no automatic merge. Method B retains Rapier0.31 through existing
bevy_rapier3d0.32/Bevy0.17.3 and all accepted anatomical stops/masses/passive drives.
Its active controller is an original equivalent of reference velocity-muscle
ideas, not copied source or a new incompatible dependency. The alternative is
Method A's native constraint-motor targeting on the same shared crash lifecycle.

Solve desired relative angular-velocity changes through full world inverse
inertia tensors; apply equal/opposite angular impulses to connected child/parent.
Clamp torque*fixed_dt, desired rate, correction rate, positive work per step180W
and total180J per ragdoll. Exact work is J dot relativeOmega plus one-half J dot
combinedInverseInertia J; exhausted/tiny budgets handle linear/quadratic cases.
Never author linear COM velocities or body transforms. Existing CrashEnvelope
remains the final mechanical-energy ceiling and may attenuate motor work.

Genuine fixed-surface upward contact normals on torso/pelvis/spine govern support.
A finite policy chooses brace, yield, attempt, supine slide and quiet rest. Failed
attempts yield after0.85s without measurable progress, reverse once after cooldown
and stop after two attempts. Airborne/high-speed/high-spin motion yields to
physics. Arms/head/leg targets stay within existing anatomical joint limits.
Native active drives are zeroed so the two active controllers do not compete;
passive resistance stays. Settled/resting states stop velocity writes and preserve
sleeping. Standalone16-body labs also exercise this method; baseline11-body does
not acquire the new pilot. Shared swerve/release/hooks/debug controls match A.

Tradeoffs: explicit positive work is bounded but not a production biomechanical
muscle model; the old global energy guard can overpower intended active movement.
Angular impulses are sensitive to contact geometry and may fail to roll a trapped
body; no position pin or forced landing hides that failure. No claim of arbitrary
frame-rate stability or high-speed recovery. Reference licenses/source pins:
research/crash-behavior. No source/media imported; root source is original MIT.
Rollback/select the other comparison branch; neither branch auto-merges to main.

Method B now includes an explicit fixed-support reaction guide (120Nm aggregate torque cap,
3.5rad/s target rate) for the roll attempt. Its positive work shares the same180J/180W
ledger and existing energy ceiling. It never writes linear motion/body poses.
The guide approximates reaction against the fixed supporting world; unlike the
internal joint impulse pairs it does not conserve closed-ragdoll angular momentum.
Upward core support, speed/spin eligibility, next-rotation obstruction checks,
timeouts and failed-progress yield govern it. HUD labels this support guide and
reports guide count/obstruction. This deliberate reference-style assistance is
the meaningful alternative to Method A strict native motors, not a hidden claim
of perfect anatomical actuation.

Final calibration uses response coefficient12/s, torque40Nm (and existing
per-joint caps), desired rate6rad/s, feedback delta5rad/s, exact positive-work
cap180J/180W and aggregate support guide120Nm/3.5rad/s. All active corrections
expire at4.95s by default, so zero-positive-work damping cannot keep limbs awake
indefinitely. Native logical resting accepts genuinely motionless distal ground
support; active roll eligibility remains the stricter core-support proxy. The
prone fixture achieved onlyabout7degrees, so full roll-to-back remains open.
