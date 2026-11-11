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

## ADR060 — Selected velocity muscles and direct, render-aligned facade handoff

2026-11-07. Problem: user selected velocity after two isolated candidates and
reports severe texture pop-in. Selected: build on Method B, with bounded reciprocal
angular corrections and supported guide; exclude Method A. Keep native anatomical
constraints/passive springs because removing these disconnects the body. No engine,
dependency, mass, anatomy, contact or save migration. Historical eleven-body
control and comparison triggers remain diagnostic-only options.

Facade investigation found authored LOD changes faded out to the generic fallback
then faded in the new facade, both groups used the same positive dither branch,
and Update-time ranges lagged the final PostUpdate camera. Selected: keep old
ready geometry while hidden new scenes/materials/tangents finish, crossfade them
directly using opposing start/end dither branches, recenter ranges after transform
propagation before visibility, and prefetch detail inside130m instead of18/55m.
Native Bevy0.17.3 installed range/render shader sources confirm branch quantization
and origin semantics. Source is studied, not copied; existing Bevy MIT/Apache
dependency/license applies. No new repository or media imported.

Reserve25% of the existing display panel/triangle budget for overlapping groups;
count both and defer upgrades if they do not fit. Keep the48-entry high-LOD cache
strictly bounded with referenced groups pinned. Tradeoffs: modestly fewer steady
detail panels, earlier asynchronous IO, finite hidden preparation, and dither grain.
This is not silhouette morphing or a new distant facade art pass. Physical lots,
colliders, support, navigation and structural IDs are unchanged. Multiplayer remains
independent of cosmetic LOD; rollback renderer independently of velocity physics.

## ADR061 — Model-derived distant identity and bounded City Run scene lifecycle

2026-11-07. Problem: generic far windows/trim did not match authored facades;
City Run still replaced live SceneRoot directly. Alternatives: raising detail
budgets/global scene residency creates excessive entities; freezing coarse LOD
loses close geometry; longer fades alone cannot repair facade identity. Selected:
project the existing source triangles/UV/PBR materials into383storey-band tiles
for128designs, batch four face proxies per657lots, keep authored close geometry.
Projection uses original source data and a depth buffer, not retail media.
Albedo filtering is alpha-weighted in linear light; normals renormalize; packed
PBR stays linear.16pixel guards and a mip4clamp prevent neighboring atlas bleed.
The composed map budget is128MiB; static proxies are separately capped250ktriangles.
Keep source and output hashes and verify every source band.

Fade proxies per face according to old/new authored masks, so an uncovered face
remains visible until its replacement is actually ready. Use physical building
extent only to select visual detail; authoritative transforms/colliders do not
change. Cache settled fallback coverage. City separates logical instance roots
from active/pending/retiring scene groups, waits for actual scene/material/tangent
readiness, then reuses final-camera complementary dither. Cap transitions at8
and high-detail caches at48; pin referenced groups and keep old geometry if full.
The city design viewer shares the lifecycle. No engine/library/physics migration.

Tradeoffs: about128MiB additional resident PBR images and flat projected distant
surfaces; grazing-angle parallax/silhouette differences and dither grain remain.
Technical coverage is not user art approval or a60FPS certificate. Existing
production performance and longer streaming gates stay open. Bevy0.17.3 is the
existing dependency; Blender4.5.9 is an external build tool only. No third-party
code/media incorporated. Roll back proxy assembly and city stream independently;
all physical layout/ID/navigation and selected velocity-ragdoll contracts remain.

## ADR062 — Scaled facade cores and measured world-repair planning (2026-11-07)

Problem: the solid fallback body could hide authored facade geometry. The old
near-body inset assumed a 1.3 scale; the far body stayed full width. Wide bays
and deep facade bands violated that assumption. A GPU probe did not reproduce
zero-width visibility ranges as the cause; do not present that hypothesis as fact.

Selected: calculate the deepest cross-LOD band envelope after actual bay scaling,
add an 8 cm clearance and inset BOTH static backup cores at spawn. Hide a visual
core if no positive interior remains. Keep detailed geometry, proxy atlas and
colliders unchanged. Alternatives: deeper arbitrary fixed inset, removing physical
buildings, replacing the renderer. Those either miss large cases or damage gameplay.
Tradeoff: the decorative core may become very small on shallow lots; exterior
surfaces still use the existing authored faces. Rollback is isolated to visual
core assembly. All-lot depth tests and large-facade native sweep cover this defect;
long-streaming/art approval remains separate.

The user's 3D Come Bye reference guides original art, not asset reuse. Generate
measured street/property/height plans before city rearrangement; ownership/fences
are proposed and require whole-lot/access validation. Narrow rural road profiles
are intentional geometry changes; preserve all lot quotas/IDs and the route.

Static-obstacle wipeout now uses normal closing speed, preserving parking touches
and glances. Existing car crash policy stays intact. Animated leg chains use one
stable forward bend pole and same-side ankle targets; no physical rig replacement.
Rival race pace scales the roster ranking to the current player's unboosted bike
class (94–106% for the shipped roster), without positional rubber-banding. Original
reference-unit helpers remain available. Traffic retains signed actual speed for
contacts, stores cruise separately, yields to downed riders and limits horn rate.
No physics backend, library or dependency migration; no new external code/media.

## ADR063 — Dense city layout study without runtime changes (2026-11-07)

Problem: the user rejected the dispersed property plan and wants a very close,
dense city along the race road. The latest instruction explicitly limits work
to layout and excludes 3D changes.

Selected: a separate 2D planning generator and concept parcel set, following the
existing city centerline. Continuous party-wall frontages sit 6 m from the center,
with an 8 m road, 1.8 m sidewalks and 0.2 m setbacks; courtyards/service wings sit behind.
A second block band and parallel streets add depth. Cross streets interrupt the
frontage, with 92.53 percent station coverage overall and no within-block gaps.
Do not change game content, structural generation, models, heights, textures,
rendering, collision or physics. Proposal IDs are independent of canonical lots.

Alternatives: move existing game lots immediately; add more distant scattered
buildings; modify art to mask layout. The first violates scope, and the others
do not resolve the requested close street enclosure. Tradeoff: this concept is
not a runnable new city. Corner/overlap/access/terrain/quota/performance validation
and user layout acceptance precede future runtime implementation. No library,
upstream code or external asset adoption. Rollback is planning documents/tool
only; retain the earlier measured survey as historical evidence, not approval.

## ADR064 — City-centered graph with two choice sections, layout only (2026-11-07)

Problem: the close-frontage draft still read as a road lined with buildings, not
an actual city. User requested a center and confirmed two split-and-rejoin sections.
The preceding no 3D change restriction remains active.

Selected: a broad connected street/block footprint around a civic square, city
hall, market and neighborhood districts. The race graph has two two-edge choice
sections: Market/Station streets meet at the center, followed by a 300 m common avenue,
then Old Town/Civic alternatives meet before exit. Four complete paths, balanced
nominal lengths and shared split-to-merge progress intervals are documented in
proposal data. This is local-coordinate 2D planning, not a terrain/runtime bake.

Alternatives: add more parallel strips; a single fork; immediately implement a
branching TrackDef. The first misses the city's center, the second conflicts with
the confirmed answer, and the third exceeds the layout-only scope. Keep current
models, content, simulation, assets and lot IDs untouched. No new library or
upstream source/media reuse. Tradeoffs: some diagonal corner parcels are held out;
merge radii, access/traffic closures, terrain, checkpoint/AI/recovery integration
and actual racing-time balance remain future gates. Preserve the prior proposal
as history. Rollback is planning tool/artifacts/documents only.

## ADR065 — Farm start, extended urban finish, one fork, rigid footprints (2026-11-07)

The user superseded the two-split city-only proposal: preserve the farm approach,
remove the separate hills district, make the city substantially longer, finish
inside it, and rejoin a single split near the first city quarter. Curves and street
elevation are part of the drawing; the layout-only restriction remains active.

Selected: a ~12 km spine proposal, with 3.15 km farm/transition approach and 8.85 km city.
Retain the current farm trace shape via a rigid projection. Use one smoothly
separating pair of roads near city hall, common quarter rejoin, then continuous
city through the finish. Report actual branch distances separately from chainage.
A continuous height study supplies urban crests/dips and 5 m contours.

The first draft wrongly warped the block/footprint grid when curving the roads.
User rejected that. Corrected: use rigid rectangular individual building plots
and independent curved road traces; hold out corner sites rather than deforming
buildings. Explicit right-angle and negative bent-building checks enforce this.
Alternatives: retain two forks/hills; bend the entire city grid; apply runtime
changes now. These contradict the revised brief or the no 3D scope.

Tradeoffs:~12 km is a draft scale, not approved exact race length. The two drawn
paths differ in distance/time potential. Corner geometry, full access/terrain
support, ID/quota migration, route schema/progress/AI/recovery, merge controls and
performance remain future gates. No engine/dependency migration or third-party
source/media adoption. Rollback affects this planning tool/artifacts/docs only;
keep previous proposals as historical records, not active instructions.

## ADR066 — Streets define polygonal blocks and parcels (2026-11-07)

Problem: the user rejected the rectangular-only interpretation of ADR065 and the
uncommitted forced-right-angle junction draft. NYC/Flatiron is the explicit
reference: roads retain their course; intervening land defines shaped blocks and
straight-edged buildings. A triangle is not a bent building.

Selected: original offline Python street-first geometry using pinned Shapely2.1.2
(GEOS3.13.1 in the installed wheel). Union carriageways/2.4m sidewalks, subtract
these reservations from the city envelope, and partition the residual blocks.
Regular centerline grid pitch is180m east–west by360m north–south. Curved race roads
are approximated by straight road segments with2m centerline simplification
 tolerance; there are no dogleg approaches to impose90degree intersections.
Small usable corner sites become whole polygonal buildings; unbuildable slivers
become explicit landscape islands. Boundary-seeded Voronoi parcels cover larger
blocks; their23m street-side bands form touching frontage building footprints.
Courts and clipped civic reservations remain separate from building cover.
All centerline crossings are noded into an offline street graph. Race eligibility
is separately the existing one-split/two-path proposal, not every background street.

Alternatives: force rectangles and discard corner land; warp building meshes;
handwrite unreliable polygon subtraction; change game generation now. These
miss the corrected geometry or exceed layout-only scope. Shapely's robust GEOS
operations are justified for authoring; no Cargo/physics dependency changes.
Source/license records are in third_party/REPOSITORIES.md. Library APIs are used;
no upstream implementation or NYC source map/media copied into the game/repo.

Tradeoffs: frontage seeds are a planning partition, not an architectural or legal
cadastral survey. Junction signals/turn radii, fire/service access, final property
heights/terraces, exact race length and runtime integration still require review.
GEOS/Python version pins are needed for reproducible exports. No user approval or
runtime performance claim. Supersedes ADR065's rectangle-only/90degree tests,
not its farm/one-fork/long-city/elevation constraints. Rollback is authoring tool,
requirements and planning artifacts/docs only; original game remains untouched.

## ADR067 — Height zoning and route-distance detail before 3D (2026-11-07)

Problem: the accepted street-first proposal still has no building massing or
quality hierarchy. User requests taller central buildings, scattered taller
neighbors/interior buildings, and less detail farther from the playable road,
finalized as a plan before rendering3D.

Selected: add an offline, seed-stable height/detail layer to the existing block
and parcel plan. Use the existing central Market Quarter as the downtown height
core; taper both along the city and laterally to its outskirts. Normal street
walls share neighborhood height bands; occasional frontage accents and dispersed
interior tower sites create taller exceptions. Reserve a six-metre service access
easement for each interior site, cut its opening in frontage footprints and record
all affected proposed parcels. Keep parks, public land and existing streets clear.

Distance means minimum footprint distance to either legal race alternative/common
route, not distance to any background street. Assign authored hero/standard/
skyline/massing tiers, while tall skyline anchors retain their silhouette tier.
All tiers require complete walls/roofs and consistent height/material identity;
this does not implement or change camera LOD/fades, physics or runtime geometry.

Alternatives: uniform random heights; towers in every courtyard; downgrade all
distant buildings to roof-only shells; build3D before reviewing massing. These
lose district structure, access/open-space or silhouette continuity. Keep the
existing Shapely authoring backend; no new dependency or source/media adoption.
Tradeoffs: ranges/distances are proposed production budgets, not measured GPU
performance. Property assembly/easements, service/vertical clearance and final
architecture need later implementation validation. Preserve the one-fork farm-to-
urban-finish route and180×360m regular grid. Rollback is this offline layer and its
maps/configuration/docs only. Runtime3D integration remains separately scoped.

## ADR068 — Center city fabric on a primary race avenue (2026-11-07)

Problem: the latest user reference shows a city built around its race avenue,
with track-first intersections. The existing fixed-origin envelope leaves the
route offset within the city; treating all background crossings equally can
create competing parallel streets and crowded junction throats.

Selected: keep the full race graph/curve/elevation/farm trace fixed. Select whole
180×360m grid cells around the midpoint of the actual race paths in each city
row, including both alternatives. Build the primary avenue before secondary
streets. Omit complete secondary street segments which compete closely in
parallel, crowd split/rejoin throats or add redundant nearby race intersections.
Retain straight grid segments and natural angled crossings; never warp buildings,
move the race to fit the grid, or add doglegs to force right-angle intersections.
Derive all block/parcel polygons from the final reserved road network afterward.

Record centering sections, primary junction locations/angles, omitted secondary
segments/reasons and street connectivity. Reapply seeded height/detail/access
rules to the regenerated proposal sites. This changes planning IDs/sites, not
canonical runtime IDs/quotas or game assets. New reference is user-supplied visual
guidance only; no media imported into the game or new dependency adopted.

Alternatives: shift only the drawing/camera; bend the grid around the road; retain
all secondary streets and hide conflicts with art; rewrite the race. These miss
the design or contradict prior instructions. Tradeoff: whole-cell boundaries
step along the city edge, and some avenue-adjacent blocks merge where secondary
segments are omitted. Junction traffic controls/radii, final plot support,
architecture/LOD/performance and runtime route integration remain later gates.
Rollback is offline authoring/maps/docs only. Layout-only scope remains active.

ADR068 follow-up, latest user steering: the screenshot circles secondary streets
still close beside the main avenue. The initial35m/60%-of-segment check was too
permissive. Replace it with local direction/distance samples (at most8m spacing):
remove a whole secondary segment if a run of48m stays within110m of either race
path and within40degrees of its tangent. Preserve perpendicular cross streets,
well-spaced avenues and both legal race branches. Audit all kept secondaries and
add four focused regression fixtures, including a partial run missed by the old
fraction rule. On concave merged blocks use native1e-9m fixed-precision frontage
overlay and preserve footprint holes; keep strict overlap/coverage validation.

## ADR069 — Build the approved farm-to-city course (2026-11-07)

User approval now authorizes3D/runtime implementation, superseding the preceding
layout-only restriction. Preserve legacy Valley/City assets, quotas and IDs as
separate maps. Bake the approved LAYOUT.json into a validated new metropolis
content manifest with its exact polygon footprints, heights, primary/secondary
roads and two complete route alternatives. No engine/physics migration.

Use existing Shapely/GEOS constrained triangulation offline, not a new runtime
triangulation dependency. Batch closed building volumes and pavement by spatial
chunk; reuse original model-derived facade atlas/mips and authored model library.
Keep stable full envelopes resident; additive close details never replace bodies
with roof-only shells. Collision remains independent of rendering.

Normalize both routes to common0–12000m progress with branch-local physical
sampling/projection, explicit rider route choice, ordered physical gates and
branch-aware recovery/camera/AI. The two actual path lengths remain distinct.
Validate content in rr_content and pure route-choice rules in rr_core. Shared
analytic terrain height underlies pavement, scenery and off-road support.

Alternatives: alter the immutable Valley lots; force one branch; rebuild the
physics engine; triangulate malformed polygons at runtime. These lose accepted
content/gameplay or make authoring failures unpredictable. Tradeoffs: authored
facade detail/materials and terrain/site support need actual native review;
performance must be measured rather than inferred from batch counts. Retain
legacy map selectors and rollback by selecting them; current new course becomes
the default after verification. No upstream source/media copying or new dependency.

ADR069 verification follow-up: retain closed full building envelopes permanently
and fade only additive cornices. Use a single ExtendedMaterial atlas with tile ID
in vertex color, unwrapped UV derivatives, wrapped gutters and five authored mip
levels. Original source bands/pixels are pinned; no per-building material loads.
Native chunks fell11,020→2,991 with unchanged geometry. The measured dev-build p95
20.38ms still misses16.7ms; no production performance gate is closed. Recovery
navigation indexes actual polygon boundaries rather than rectangular lot proxies.
Both physical routes share logical progress/gates, trading equal metre ranking
for fair common completion stations; preserve distinct physical lengths.

## ADR070 — Wider Metropolis sidewalks and curated curb furniture (2026-11-07)

User requests a small sidewalk expansion and street population. Existing2.4m
walks already meet the approved buildings, so preserve every building/lot and
widen walks to3.4m by reallocating1m from each city carriageway edge. Typical
primary road12→10m; secondary8→6m. Farm geometry below the city boundary remains
unchanged. Both race paths, source elevations, split, gates and physical rider
configuration stay intact. Match city half-width metadata and cross-traffic lane
centres to the resulting carriageway. Alternative: move/trim thousands of lots;
rejected as unnecessary for this change.

Bake deterministic curb furniture from existing original licensed prop roles.
Require the whole buffered footprint on sidewalk,20m priority-junction setbacks,
separation from other furniture and a2m paved building-side walking strip. Sparse
tree trunks use conservative well footprints; crowns can overhang the street.
Reuse existing visible GLBs, independent static colliders and original deterministic
concrete paving with metre-scale slab joints. No added dynamic lights or per-frame
placement queries. Replace old regular tree/lamp placement rather than layering
both. Fallback legacy-prop comparison may omit the new model furniture.

Tradeoffs: streets are narrower, not buildings further apart; corner widths vary
with the existing network. Width/clearance checks are authoring constraints, not
accessibility/building-code certification. Existing navigation does not become a
full city pedestrian AI. Native visual, recovery and performance checks remain
required; broader art/performance acceptance stays open. Rollback: previous bake
and streetscape spawn; no save/dependency/physics migration or new external code.

ADR070 recovery follow-up: register furniture footprints in the existing bounded
visibility-navigation index, with distinct ownership and the same authored yaw/
dimensions (conservative tree wells). Avoid introducing a separate pathfinder or
changing physical recovery. Verify every furniture centre blocks a direct path
and both race corridors remain clear. General pedestrian behavior remains deferred.

## ADR071 — Restore authored3D facade models on Metropolis (2026-11-07)

User clarified that the new city should use the earlier detailed3D facade models,
not just their flattened albedo proxies. Keep the accepted polygonal layout,
sidewalks, furniture and independent physical envelopes. Reuse the twelve existing
source-matched style families as actual ground/upper/crown GLB meshes, repeated
per polygon edge and floor. No new models/textures or dependency migration.

Preload fixed authored LOD1/LOD2 prototypes and prepare materials/tangents while
hidden. Share the actual mesh/material handles; no SceneRoot replacement while
racing. Use finer street-level rows and authored coarse upper rows, with one
consistent composition per face rather than distance-driven file swaps. Bound
nearby faces, primitive instances and actual triangle counts, including retiring
fade groups. Preserve full opaque envelopes behind additive relief. Smooth whole
face entry/exit using the final propagated render camera and shared fade origin.

Alternatives: instantiate all19,068 buildings' detailed scenes; reuse rectangular
Valley assemblies on irregular footprints; hide the base wall before readiness.
These respectively exceed budgets, misfit the accepted street plan or reproduce
roof-shell pop-in. Tradeoff: distant/over-budget faces retain source-matched
proxies, and relief depth is fitted to the narrow sidewalk clearance. Native mesh
source/census, visual/recovery and performance checks are required; no60FPS or
final art approval is assumed. Rollback is cosmetic only; save/physics unchanged.

ADR071 native-review follow-up: the BUSS4 upper source includes a unique RADIO
sign. Repeat its same-masonry CLSTGA11 authored window module on ordinary upper
bays; retain the original sign at one first-upper-floor central bay per frontage.
Use the same neutral source in Metropolis's distant upper tile and record its
actual source ID/hash. Canonical source GLBs/materials and the global facade atlas
remain unchanged. Bound outward relief to0.22m plus0.025m cosmetic offset so it
fits the recently widened street margin. These are visual fitting transforms,
not new physical surfaces. Survey dwell is longer to capture completed fades,
not partially prepared views immediately after a camera teleport.

## ADR072 — Main-street life, lane variety and varied rooflines (2026-11-07)

User requests more street activity, main-road traffic,2/3/4 lanes and visible
building-height variety. Retain existing race geometry, street reservations,
sidewalks, whole building footprints and the earlier authored facade assets.

Use marked lane layouts inside the existing10m city carriageway: two3m lanes
plus parking edges; three2.8m lanes with an extra direction-specific lane; four
2.4m arcade lanes. This changes active lane allocation/markings, not asphalt/curb
width. Wider real-world lane dimensions would require a separate parcel/road
redesign; do not silently narrow sidewalks or cut building lots. Publish editable
section/direction/width settings. Interpolate lane positions/dividers over100m.
Use double-yellow opposing-flow dividers and dashed white same-direction lines.

Increase main fleet24→96, initialized on its actual road positions before Racing.
Right-hand American direction/centres apply to Metropolis. Keep legacy behavior.
Use native existing car models/physics contact policy, measured following speed,
occupied-lane slowing/honks and seeded fleet restart. Riders normally target the
marked carriageway, not the parking edges. Car traffic and paint follow the actual
reserved street centre; preserve exact race paths/progress/gates. Simplify each
original edge independently so fork/rejoin/common streets agree. Validate whole
vehicle footprints, including tapers, against real paved polygons.

Bake more original catalog furniture and fully road-supported curb parking,
keeping junction/walking-strip setbacks. Add static parked cars to recovery
navigation. Mix2–4-floor shops, medium and taller frontages per building seed,
with downtown-dependent upper ranges and preserved interior towers. Height edits
update both render and collision roofs; all footprint/lot IDs remain fixed.
The old±1-floor block variation was too uniform from behind the racer.

Alternatives: more uniformly spaced24-car traffic; paint four unrealistic lines
without lane-aware traffic; rebuild city rights-of-way; retain±1-floor variation.
These miss visible density/flow or expand scope. No engine/dependency, handling,
ragdoll muscle/mass or save migration. Tradeoffs: narrow arcade four-lane roads,
more scene owners/contact work and new roof heights require measured native
performance/traffic/recovery checks. Broad art/full-course/performance gates remain
open. Rollback is the prior manifest/config and traffic adapters; legacy maps stay.

ADR072 performance follow-up: the first denser dev survey missed the frame budget
(p9532.58ms). Keep all96 cars' logical motion, queues and rendering, but admit car
colliders by nearest-racer gameplay interest (170m enter/220m retain), independent
of the render camera. Include all riders' projected physical recovery targets.
At highway speeds this leaves many fixed ticks before contact. Do not delete the
car, change its trajectory, or cull solid terrain/building/furniture collision.
Unobserved car/pedestrian interactions outside every racer radius are not simulated;
future spectator/network observers must extend this interest contract. Existing
cross cars yield to main-road traffic before committing at intersections.

Avoid per-car full nearest-route scans when authoritative progress is already
known: compute its lateral coordinate in that progress frame. Preserve general
nearest-route projection for bikes/recovery. Record both native development and
release samples; proximity bounds and allocation counts alone do not certify60FPS.

## ADR073 — Persistent sidewalk-prop destruction only (2026-11-07)

User clarified scope: props only; no car damage, building collapse or road craters.
Register all13 placed sidewalk types as stable damage owners linking original
models and physical shapes. Collision/melee energy accumulates without healing;
low-speed nudges do not break props. Damage persists through camera/LOD changes
and through the race, resetting only on explicit start/retry.

Reuse original models: crush fragile objects, topple larger ones; the damaged original model remains solid. Physics supplies motion/gravity; do not inject launch velocities or
retune rider/accepted ragdoll masses, joints or recovery. Predict genuine imminent
prop contacts before the physics step so fragile objects can yield to the impact.
Hard poles/trees retain normal crash risk. Deduplicate swept/contact damage with
pair cooldowns. Bound dynamic debris; settle supported low-energy bodies and use
supported static ruins when the budget is full. Persistent ruins never disappear
on a lifetime timer. Reset restores poses, shapes, models and navigation.

Alternatives: generic disappearance/replacement cubes, indefinite dynamic debris,
full Voronoi fracture/engine migration, or damaging vehicles/buildings. These lose
original art/persistence, destabilize performance or contradict clarified scope.
No new dependency/assets. Analytic strength profiles and compressed models are an arcade
approximation, not material fracture or fluid simulation. Record native visual,
collision/recovery/reset and budget evidence; full art/performance gates stay open.

## ADR074 — Racers and killers with identity-specific retaliation (2026-11-07)

User requests half the fourteen rivals to race/pass peacefully unless attacked,
and half to actively hunt/fight. Alternate stable rival IDs: odd IDs are racers,
even IDs are killers (seven each); player controls remain unrestricted. Retain
names/colors, authored per-rider pace/strength/stamina and shared60Hz vehicle,
combat geometry, hit-chain and crash/recovery simulation.

Old pursuit interpreted ten personality relationship columns; several actual
riders shared column0. This cannot safely represent retaliation against one
specific opponent. Use a race-local exact-ID aggressor set in rr_core::rival_ai,
recorded only after a landed direct punch/kick. Racers never start fights, cannot
auto-kick an innocent bystander, do not hunt downed opponents and remember their
actual aggressors until restart. Multiple real attackers can each earn retaliation.
Incidental wheel/traffic contact does not create a combat grudge. Reset through
Rider::new clears memory. Existing relationship observations remain historical
roster data, not authority for the new combat permission.

Killers may attack any rival/player, seek nearby opponents regardless of historical
hostility, use at least64m forward pursuit and60% of the old attack cooldown with
a0.75s minimum. Keep rear range10m, lane separation9m, initial8s grace, stamina,
actual strike reach/occlusion and same-corridor checks. Existing forward28m downed
hunting remains for killers only. Both use shared passing-space selection when
not pursuing a fight; retain traffic avoidance, curvature speed limits and smooth
steering. UI shows R/K on standings.

Alternatives: max hostility for everyone, shared-personality revenge or new physics/
engine controllers. These violate the requested roles/identity or change accepted
handling. No new dependency/media or external code copied. This is the user's new
RoadDash behavior specification, not a claim of exact original-game AI parity.
Tradeoff: stable assignment and race-long grudges are predictable; subjective
full-race balance remains AI-11. Rollback restores prior pursuit/attack policy;
no save migration is required because AI memory is race-local.

## ADR075 — Opening-road repairs and local performance evidence (2026-11-07)

User reports inward seated knees, uncapped fence bays, a crowded narrow race start,
abrupt farm/city skyline, parking-speed vehicle ejections and low FPS despite low
aggregate CPU/GPU utilization. Preserve accepted racer/killer roles, passive anatomy,
velocity muscles, momentum/recovery and source race paths/parcels/IDs.

Presentation: bias only held seated leg IK poles slightly outward (0.28 lateral
relative to forward), retain peg targets/anatomical lengths and unheld knee planes.
Fence rails gain both end posts with shared-coordinate deduplication; intentional
field entrances remain. No ragdoll joint or physical body tuning.

Authoring: farm asphalt and starting apron become10m, matching5m handling half-width,
with two3.6m lanes and1.4m paved shoulders each side; verge2m. Correct the former
4/6m handling half-width versus4m rendered pavement mismatch. Reduce moving main
traffic96→64 and seed it from700m, keeping city lanes/cross traffic/parked cars,
seeded restart, race gates/branches/lot footprints. City-edge building heights,
including towers, receive a smooth2→32-storey cap over1.8km north from the approved
boundary. Rebuild from the unchanged layout; physical and rendered roofs agree.
This is a height transition, not completed suburb/property-art approval.

Contacts: vehicles must pass a finite closing-speed gate (5m/s /18km/h); parked
obstacles retain the wall gate. Normalize/orient horizontal contact normals, use
mid-body shape projection for swept contacts, retain pre-solver velocity and
re-evaluate persistent rider/vehicle contacts so a gentle touch can become a hard
impact without another Started event. Known cars are not low ground geometry.
Normal collision response stays solid; no added crash impulse or ragdoll retune.
Historical test fixtures that expected a low-speed scrape to eject are corrected.

Performance: add default-on1Hz local JSONL via bounded nonblocking queue/background
writer,16MiB rotation. Record wall average/P95, all fixed-step gameplay/physics
spans per frame, main-world duration, renderer extraction and renderer CPU/wait
spans, visible meshes/assets, active bodies, route/camera and Linux process/system
memory/paging. Renderer counters are atomic and may overlap main; never label them
GPU timestamps or sum parallel spans as serial GPU cost. Full swap alone does not
prove active paging. Disable with ROADDASH_PERF_LOG=0; directory override uses
ROADDASH_LOG_DIR. No network telemetry or new dependency.

Alternatives: hide low FPS behind lower detail, guess from aggregate utilization,
remove car solidity, widen city parcels or retune ragdolls. These lose evidence,
change gameplay or violate scope. Existing Bevy0.17.3 SubApp extraction wrapper and
RenderSystems ordering are used without replacing extraction. No imported source
or art. Native views, contact/recovery regressions and recorded timing establish
technical integration; user art/balance, true GPU profiling and60FPS remain open.

## ADR076 — Opt-in dependency feature, default parallel island solving (2026-11-07)

User authorizes enabling multicore physics after measured race slowdown. Current
Bevy0.17.3 already uses its multi_threaded scheduler; bevy_rapier3d0.32.0 wraps
rapier3d0.31.0, with SIMD enabled but parallel disabled. Enable the existing
bevy_rapier3d/parallel feature through a RoadDash parallel_physics feature, on by
default. Retain an explicit serial build for regression/benchmark comparison.
Keep exact engine pins, fixed60Hz/substeps, anatomy/muscle/solver parameters,
collision policy, momentum and recovery. This is no physics-engine migration.

Inspected installed rapier3d0.31.0 physics_pipeline.rs: supported parallel solver
uses Rayon par_iter_mut across independent islands. Intra-island parallelism is
explicitly not ported to this solver version; one connected ragdoll/contact pile
cannot be assumed to scale across every core. Broad/narrow phase, synchronization,
writeback, game systems and render waits are not automatically parallelized by
this feature. Do not claim linear core scaling or a guaranteed60FPS fix.

Use existing pool behavior initially and measure; RAYON_NUM_THREADS permits a
controlled worker-count comparison without changing physical tuning. Lock newly
activated Rayon transitives, inspect declared licenses and keep dependency audits.
Record compiled parallel flag/CPU availability/requested thread environment in
local logs. Compare the preserved serial release binary and parallel release
using paired headless crash labs and identical native route fixtures; retain
broad performance/user-hardware acceptance separately from feature integration.

Alternatives: hand-thread the Bevy world or connected body chains, replace the
physics engine, lower substeps, or disable accepted constraints. These are larger
and riskier changes than the backend's native scheduling feature. Parallel order
can affect floating-point trajectories; regression checks use established physical
bounds rather than promise bitwise network determinism. Future server authority
must explicitly revisit reproducibility. Rollback builds omit parallel_physics;
no save/content/asset migration is required. If defaults regress, retain an
explicit feature and choose worker policy only from measured comparisons.

## ADR077 — Live pickup ownership and signed parking reverse (2026-11-08)

Problem: remount projected the wreck onto the track, discarding its actual
support height, while the old hidden riding chassis could retain a stale backend
pose/history. The handoff now takes the live wreck position and upright heading;
projection updates race progress/lane only. Reset Rapier current/next pose,
linear/angular velocity, forces, BikeState and interpolation history together
before forces/writeback. Lift queries solid support excluding the recovering bodies instead of pulling elevated
bikes to terrain. The assisted adapter receives the corresponding height offset.

Alternatives: teleport the wreck/rider back to the crash origin, resample the
centerline, or overwrite visual transforms after simulation. Rejected because
these move the pickup, lose elevation or desynchronize authority. No ragdoll
mass, anatomy, motor, damping, ejection or solver parameters change. Explicit
exceptional rescue and the existing five-second unseen NPC approximation remain
separate policies, not ordinary remount behavior.

Brake input below0.15m/s forward speed enters parking reverse only without
throttle. Pure rr_core rules target2.5m/s backwards with2m/s² acceleration;
dynamic mode requests bounded tire force, assisted mode integrates signed speed.
Normal forward braking remains unchanged; release coasts and throttle returns
forward. These are centralized tuning constants, not a gearbox/engine migration.
World forces can exceed the parking target; no teleport or hard velocity clamp
is used. Existing collision, grip, checkpoints, boost and input buffering remain.

New input action/key or automatic escape teleport were considered unnecessary.
Reverse is player-controlled with existing S/Down/brake input; ordinary AI does
not reverse. Rollback is isolated to the helper/adapters and pickup handoff; no
content/save/dependency/asset migration. No external code/assets incorporated.

## ADR078 — Visual glove morphs and explicit attachment frames (2026-11-08)

Problem: the accepted visual skin has rigid digits. Authoring-only fist shapes
were not shipped to the runtime. A stiff open glove survived punches and riding;
future held objects had no typed, side-specific attachment points.

Use the installed Bevy0.17.3 native morph/skinning pipeline. Split only the
original glove surfaces from the body; add at most three rounds of long-edge
refinement to give the original rigid digits phalanx samples. Keep source UVs,
material regions, skin weights and silhouettes. Four shared targets represent
left/right grip/fist. Hinge curls change position, normal and tangent together;
wrist vertices stay fixed. Meshes/images are generated once, shared by all
actors; each frame updates four weights only. Do not deform every body vertex
or allocate unique meshes for racers. The extra glove surface adds one draw
surface per human; native evidence is integration, not a performance certificate.

HandContacts authoring lives in validated typed rr_content metadata: curl angles,
hinge locations, bind-space basis, punch keys and grip/weapon socket frames.
Pose weights follow existing fixed-clock keys and interpolate with the body.
Holding grips and punching the other side therefore use independent shapes;
relaxed walking/crash fallbacks do not inherit fists. Sockets are children of
visual skin joints, never physical bones, hitboxes or gameplay weapon authority.
Seated socket offsets compensate for the existing reference contact/retarget
basis and are checked against universal grip targets.

Alternatives: replace the physical skeleton, CPU-deform per-actor vertex buffers,
or import a new character/animation dependency. Rejected: they risk accepted
physics, increase hot-path work, or require unrelated licensing/retarget work.
No physical body, joint, mass, impulse, muscle, recovery, control or save changes.
No external implementation/assets copied. Bevy's existing MIT/Apache-2.0 API is
used without a dependency/version migration (R08).

The hand review holds exact extended-punch keys and shows native close-ups; the
ordinary animation lab still plays full animations. These are original gloved
shape prototypes, not individual tendon simulation or approved final finger art.
Context/Blender round-trip and user silhouette approval remain open. Rollback
removes visual morphs/sockets and extra surface partitioning; original .rdmesh,
GLB sources, authored clips and physics assets are unchanged.


## ADR079 — Measured contact observations and bounded stagger interruption (2026-11-08)

Part01 audit found that the accepted velocity-muscle path already integrates
three-hit swerve/countersteer and continuous seated release. A solid wall contact
below the ordinary50km/h gate, however, could leave a rider continuing a mounted
loss-of-balance sequence against an obstacle. Interrupt an existing stagger at
1m/s normal closing speed against the tall-solid/vehicle path. Resting touches,
low suspension support, downed-body ride-over, ordinary riding thresholds and
the explicit baseline retain their existing paths. This threshold is new bounded
arcade policy, not recovered original calibration. No extra impulse, translation,
solver, anatomy/mass/joint/muscle or retained-speed tuning is introduced.

Pure measurement/classification and the interruption predicate live in rr_core.
The adapter retains128 vehicle-contact observations with simulation tick, both
entities/world velocities, surface normal/closing speed, approach, solid height
and crash decision; ROADDASH_TRACE_CONTACTS prints each observation. The ring is
read-only diagnostic evidence, not authority for hits, animations or outcomes.
Do not confuse100/60 same-direction40km/h closing or oncoming160km/h closing with
the existing entry ground-speed rule. No vault impulse is added to manufacture
clearance; full contact-driven trajectories over car heights remain V2-01-005.

Alternatives: lower every wall crash threshold, add launch/spin, replace passive
physics or implement a server/network boundary now. Rejected as outside the
narrow Part01 correction and accepted feel. No dependencies/assets/source imports
or save migrations. Rollback removes the stagger-specific predicate and telemetry.
Tests/evidence and all29 open requirements are in testing/part01-runtime/README.md.

## ADR080 — Static hand authoring shares runtime profiles (2026-11-08)

ANIM-16 replaces the historical Blender Context_Fist compression with the
validated content/hand-contacts.json hinge/curl/socket profile used by native
rendering. tools/authoring_hands.py converts source units through the existing1.2
visual scale and uses the forearm-inherited .27m distal origin. Blender hand
bones already carry the seated basis; do not multiply seated_basis again.
Independent grip/fist keys per side and complete assembly hand states prevent
loading a pose from retaining the previous pose's attack. Grip and Weapon empties
parent to visual authoring wrists; original club/chain proxies originate at the
weapon socket and orient its +Y handle axis to the authored silhouette direction.
This is an original authoring interpretation, not recovered native joint data.

Use the locally available Blender4.5.9 LTS (8bf95cbd38d1), verified by actual
export/import; no Cargo dependency/version or runtime asset migration. Keep source
GLBs and physical rig untouched. Blender conforming long-edge subdivision samples
only distal hand-weighted edges; native triangle refinement remains independent.
Export rederives morph normals. Exact source/runtime tessellation, hard-normal
identity, thumb contours, grip/weapon clearance and user approval remain ANIM-16 /
ANIM-02 gates. Matching numeric curl recipes is necessary, not art approval.

Alternatives: import another character/rig, replace physical hands, overwrite
motion.json or add Part03 interpolation. Rejected as outside this Part02 pass.
The external editable .blend / round-trip .glb are reproducible original outputs;
repo review captures, geometry-error measurements and source hashes record this
technical subset. No retail media, external code or new repository adoption.
Rollback restores authoring scripts/assembly/gallery; runtime physics, gameplay,
controls, saves and timing are unchanged. See testing/part02-authoring/README.md.

ADR080 contact follow-up: measured source neutral socket-to-bike error was
10.467mm despite sub-micrometre recipe/export errors. Replace the legacy raw
wrist-offset fitting with the installed socket's bone-local offset and desired
hand rotation. Use the same fixed-length bounded IK; validate declared held
contacts at5mm and reject unreachable layouts. All138 poses fit within4.82e-7
authoring metres in this pass; neutral/tuck/opposite-hand checks additionally
measure renderer-scaled world errors. Source forearm/hand poses and83 images
change, while torso/seat/peg/action semantics, runtime clips and physics remain
untouched. Source fitting is not proof of final native silhouette/clearance.

## ADR081 — Fixed contact clocks and support-local visual stance locks (2026-11-08)

Context: drive emitted immediate melee attempts while the visible attack entered
for55ms and played at1.8x. Punch runtime keys2220/2230 draw back,2221/2231
chamber,2222/2232 extend/contact; kicks2261/2271 extend/contact. Generic
"second-key" timing would hit at the punch chamber. Grounding also retained
world-space plants until exponentially smoothed weight reached exact zero,
which never occurs, and could reuse a prior-stride lock.

Decision: drive emits AttackIntent after existing cooldown/stamina admission.
A Rider-owned rr_core StrikeClock advances only at fixed60Hz; rr_content checks
explicit contact IDs and matching mirrored duration sums. It derives source
contact at.26s for punches/.24s for kicks, plus the same.055s entry and1.8x
playback used by the visual sampler (.199444/.188333s). Both cross at tick12
(.200000s). The visual pose samples this clock without physical writes.
Pause freezes clocks; other inactive phases, rider reset, stagger, recovery and
finish discard pending contacts. An unfired clock rejects replacement/duplicate
intents. One StrikeContact is emitted even when a tick crosses the threshold.
The existing AttackAttempt remains an explicitly already-resolved contact fixture
for geometry/three-hit/retaliation tests; live drive never writes it.

Kick side is chosen once at windup from the nearest eligible rival/NPC within12m
(with deterministic entity tie-break); it cannot flip at contact. Existing player,
exact-attacker racer retaliation, killer/NPC and immunity rules remain. Contact
still evaluates current range/LOS and eligible targets; a target moving to the
other side or out of reach misses. No new targeting magnetism or weapon tuning.

Visual plants retain collider-local point/normal and follow the current Rapier
collider frame, including parented/kinematic supports. Each new stance captures
its current sole. Swing weight clears below.02 (not unreachable exact zero).
Removed, steep, lost or unreachable supports clear immediately; support must
still ray-test as the same solid collider. Physics frames avoid stale render
GlobalTransform propagation. No runner/pelvis/body teleport, root-motion write,
physical rig/contact/mass/joint/motor/energy/recovery tuning or source clip export.
Full pelvis/stair/turn/clearance and subjective motion gates remain open.

Alternatives rejected: resolving damage at input, moving visual timing separately,
choosing the kick leg again at impact, retaining world-space locks or replacing
accepted anatomy with the source authoring skeleton. No dependency migration,
external repository/code/media import or new requirement IDs. Existing animation
source studies remain research only. Native and automated evidence and precise
remaining limitations are in testing/part03-runtime/README.md. Rollback restores
the prior input-tick contact and world-space foot-lock adapter; existing assets,
saves, controls, legacy maps and accepted physical paths are untouched.

## ADR082 — District facade pools and shared full/coarse fitting (2026-11-08)

PALETTE-01 replaces global modulo12 building assignment with the approved source
block's district and existing computed floor/role information. The original24
selected storey-kit designs are facades, never structures/surface samples. Five
frontage pools retain local identity; >=13floors and interior towers use the
office pool. Stable BLAKE2b over existing IDs selects within each pool; no platform
process hash or new placement randomness. Unknown pools/types fail authoring.

Keep the original first12 atlas indices stable; append12 additional draft models.
Validate the content palette ordering against rr_content's runtime IDs. Rebuild
with tools/bake_metropolis.py from the unchanged approved source. No topology,
parcel, height, ground, route, props, collider, rig, input or dependency migration.
The source/detail renderer and coarse renderer share bay/ground/upper/crown
fitting; coarse identity uses all three actual source-derived model bands and
five mip levels. BUSS4's upper intentionally aliases CLSTGA11 except its once-only
detailed RADIO sign; provenance records this exception. It is not24unique upper
source identities. No source model/material or global far atlas is edited.

The expanded coarse atlas is1024x1152 with128pixel cells,16pixel wrapped gutters
and a1pixel final-mip guard. Source GLBs atLOD1/2 and materials remain ready
before Loading exits. Keep28selected faces/8retiring slots,6000instances and1.5M
actual triangles including retiring groups. Expanded resident source cost is
explicitly measured; unchanged hot caps do not imply unchanged cold-load/memory.

Alternatives: load every128design, randomize global facades, make non-office
towers, replace source geometry/materials, alter world topology or tune physical
rigs. Rejected as outside the bounded Part04 pass. No external code/media or
repository investigation was needed; local original source catalogs suffice.
Rollback restores palette/compiler and matching derivative atlas/runtime source
indices together. Final art/signage, broad transition/performance/platform and
user acceptance remain open under PALETTE-01/LOD-03/V2-04. See evidence in
testing/part04-world/README.md.

## ADR083 — Prepared original buckled furniture and shared melee contact (2026-11-08)

Part05 bounded DESTRUCTION-02 pass bakes twelve GLB variants for six existing
crush roles from their exact original low-LOD source meshes. The offline recipe
folds side walls, dents inset panels, tilts lids and sags roof/foliage components;
indices, UVs, material definitions and embedded image bytes remain unchanged.
Positions, unit normals and accessor bounds are regenerated. The existing asynchronous
runtime tangent queue derives tangent frames from the changed positions/UVs. No facade/material
source changes, atlas rebake, imported media, repository investigation or new
dependency. Existing source generators/catalog remain editable and unchanged;
the small deterministic derivative recipe is the damage authoring source.

Prepare a hidden fixed-LOD damaged scene sibling for every affected furniture
owner. Switch visibility only after hierarchy instantiation and every descendant
material is marked ready by the existing detail queue and every normal-mapped mesh has tangents. Before readiness, retain
the old root-crush fallback; reset immediately shows the original source. The
baked Y height and fitted base offset are each compressed once; damage scene
scale is unchanged. Separate solid collision retains its existing38% envelope.
PropLods runs before damage visibility, which preserves the500m visibility cap.
Unchanged transforms are not rewritten; readiness checks latch after completion.
The native survey observes the PropDamageVisuals set after PropLods; it waits
for fixture variants with a30s real-wall timeout and explicit missing source IDs.
No per-frame vertex generation, extra physical debris, lifetime expiry or
collision/navigation/racer tuning. Added resident hierarchies are explicitly
counted; this is a cost and not free merely because they are hidden.

The earlier Part03 move from AttackAttempt to scheduled StrikeContact revealed
a prop-contact integration gap. Destruction now reads the same resolved message
with the chosen leg, after combat::schedule and before melee. Legacy already-
resolved fixture messages remain supported. Pause clears messages; recovered,
staggered and finished attackers are rejected. No input-time or duplicate hit.

Rejected: runtime CPU deformations, unrelated generic cubes, extra fragment
bodies, cars as damage targets, loading replacement scenes only at impact, or
changing accepted physical rigs. Seven topple roles, artist-approved severed
variants and broad damaged-street/performance/hardware acceptance remain open.
Rollback removes prepared derivatives/visibility adapters and restores the old
crush presentation; keep the contact-event regression correction independently.
Evidence: testing/part05-props/README.md. No new requirement IDs.

## ADR084 — Continuous original civilian weights and bounded neutral refinement (2026-11-08)

Part06 NPC-SEAMS-01 remains partial. Retain the original55 human draft sources,
physical11-frame driver interface,16-body default anatomy,11-body control and10-body
cow rig. Rebuild110 human runtime derivatives with a source-space continuous
four-influence field, normalized for the source author's female-height scaling.
Body/clothing/hand vertices at the same location sample the same field. Keep
held equipment rigid with its hand. Central pelvic tissue has zero leg influence
near x=0 throughout the actual source pelvic extent; distal thigh/arm transitions
avoid splitting complete joint caps. Cows retain byte-identical derivatives.

Skin interpolation across large original cylinder/low-LOD faces exposes finite
mesh sampling error. Apply exactly one conforming shared-edge midpoint subdivision
to each human primitive containing blended weights. This preserves the complete
neutral triangle surface, winding, original vertices, UV interpolation, materials
and embedded source bytes; interpolate unit normals and recalculate weights at
new vertices. Every primitive is bounded to4x triangles and <= original vertices
plus original index count. No runtime mesh generation or added entities, meshes,
materials, bodies or simulation work. Total actor geometry/residency increases;
source-and-cost.json records actual file/vertex/triangle totals. It is not free.

The broad mean improves, but individual coarse torso/shoulder/pelvis attachment
samples can worsen. These are explicit regressions/remaining art work, not hidden
by an average or a completed checkbox. Preserve per-case receipts, native source-
family captures and actual propagated driver-matrix measurements. Worst synthetic
and native measurements are distinct from full production motion acceptance.
The original disconnected volume topology, finished silhouette/materials and full
pose/hardware/performance acceptance remain open. Stop this bounded pass rather
than increasing refinement depth or replacing physical rigs.

Rejected: replacing the accepted rig with the authoring skeleton, physics tuning,
adding cosmetic standing wrist twist, hiding components, rigid fallback blocks,
scene/entity spawning during PostUpdate, unbounded subdivision and unrelated sky
work before NPC acceptance. No external source/media/repository investigation or
dependency migration; local project-original generators/catalogs suffice. Source
and output hashes bind the recipe and derivatives; refresh only changed human
asset intake records. Rollback restores converter and110 matching derivatives
with their hashes together. No new requirement IDs.

## ADR085 — First held club on the shared fixed combat clock (2026-11-08)

Implement a bounded Part07 development pass without changing accepted handling,
physical riders, velocity muscles, motors/joints/masses or crash/recovery policy.
One original training club uses three resident shared cuboid parts, a typed
ability validated in rr_content, and pure side/reach/stamina rules in rr_core.
Six pickups seed per race; at most12 ground clubs survive drops. Clubs have no
colliders/rigid bodies and no async asset readiness advantage. Players use
buffered remappable E pickup/Q drop; rivals use the same distance/occlusion
availability and same ability/tell/contact clock. Restrained racers still only
retaliate against their exact landed-hit attacker. Kicks remain independent.

Capture the complete ability in StrikeClock at windup. Pickup during pending contact is rejected without changing loadout. Drop is
queued once in the buffered control until contact completes; crash/reset cancels
the queue without duplicating drops. Crash/stagger/finish cancels the clock and drops equipment. The existing
three-direct-hits/three-second gap rule owns knockoffs; no weapon launch impulse
or arbitrary damage multiplier. Prop and rider/NPC consumers read the same
scheduled contact and captured reach. Default club source contact .32s maps
through the existing .055s entry/1.8 playback to .232778s; 60Hz contact is the
first crossing, normally .233333s. Cooldown .85s, stamina300, lateral reach2.55m,
longitudinal .9m, pickup2.5m are new authored tuning, not original-game numbers.

Append only six existing original authored club keys and two clips; preserve all
36 released keys/13 clips and hierarchy/offsets/IK calibration deeply unchanged.
Never run the wholesale exporter after hand calibration. Layer only the selected
weapon-ready arm over the current riding/tuck/turn base; play full authored club
attack then return to the current layered base. Grip morphs replace fists. A
separate weapon hand prevents ordinary kicks transferring the club; side changes
between club attacks remain a prototype instant handoff, pending art review.
Attachments spawn in Update before transform propagation, inherit owner visibility
and never write body transforms. Drops preserve live X/Z, raycast below the live
point and use clamped terrain height if no fixed support is available; overhead
roofs cannot attract them. Pickups require unobstructed contact-side access.

R01 pinned follow-up is reference-only with unknown root license. No imported
source/media, engine migration, new dependency, police or new map work. Reject
replacing the accepted physical rig, animation-time damage, source regeneration,
unbounded dropped bodies and invisible unloaded combat advantages. Roll back
club inventory/rules/append-only keys/attachments together; preserve inherited
Parts01–06 and the fixed contact correction. Broad arsenal, balance, transient
swept sensors/results schema, bespoke audio/holster/steal and human fun/art /
performance/hardware acceptance remain open. Evidence: testing/part07-weapons/.

## ADR086 — One resident police rider and safe pursuit/arrest policy (2026-11-08)

Part08 gets a bounded offline implementation using the existing Rider path rather
than an invulnerable kinematic traffic car. One explicit police noncompetitor
(id15, independently named; palette6) lives beside the unchanged15-racer roster.
The officer shares handling, top speed, forces, collision/knockoff, physical rig,
combat clocks and recovery. Domain state/policy lives in rr_core::police;
rr_content validates authored content/police.json; native adapters own LOS,
world support, input-independent decisions and presentation. No dependency or
physical-rig migration, no arbitrary launch energy, and no car/prop-cap changes.

States: Patrol, Warning, Pursue, Intercept, Search, Return and Arrested. One
resident patrol begins at authored2.5% route distance; no reinforcements, live
spawning/despawning or catch-up teleports. Detection is fixed10Hz (every6ticks); conditional near-arrest LOS/support checks run every60Hz fixed tick for at most one candidate,
not frame-rate dependent; last-seen snapshots are retained through lost sight,
without tracking unseen transforms. Normal controls drive bounded pursuit speed
and lateral interception; patrol resumes forward along the open route after loss
rather than magically returning to an original station. Cop offscreen/rescue
teleports are excluded. The officer remains solid and can be knocked down by existing
combat/contact rules; it must recover before renewed arrest authority.

Continuous warning and authored8s grace give the player a chance to comply.
Speeding and confirmed melee contact are current offenses; whiffs/windups are not.
One player target is deliberate first-pass scope, not complete police discretion.
LOS includes solid geometry; actual live runner/torso supplies the target point.
Safe arrest requires continuous2s dwell, target/closing speeds<=1.2m/s,
range<=1.8m, near-level support and unobscured perception. Falling/getup/mounting,
stagger/immunity and crashed officers cannot arrest. Unsafe fixed frames clear
dwell, and pause/finish/reset interrupt it. Mounted officers can deliberately
stop and park before approaching a stopped or running target; an intentional-foot flag keeps
post-crash Running distinct from police foot chase. Foot pursuit uses the existing
navigation/KCC and walk speed, returns to the officer's actual parked bike after
loss, and never writes physical driver poses from animation. Retry restores a
stored dormant chassis through the existing restoration helper before rebuilding
race authority/presentation, including intentional police dismounts.

Busted ends the race without finish award, profile races/wins/best-time writes or
credits. Drive/gate finish and combat/knockoff resolve before police arrest; a committed finish has priority because police only think in
Racing; a committed arrest cannot later call ordinary finish. Retry retains
noncompetitor identity, clears pursuit and restores the original resident start.
UI explicitly excludes police from all15 standings/content indexes and preserves
the seven/seven rival role census. Weapons retain15-racer prepared attachments;
police uses current unarmed attacks, not unapproved arsenal advantages.

Original draft marker cuboids and generated sweep cue use resident resources.
Full police uniforms/vehicles/dismount/gesture animation, discretionary rival
targeting, human fairness/art, broad performance/hardware and exit gates remain
open. No Part09/networking begins. R01 pinned research remains reference-only
with root permission UNVERIFIED; no source/media reuse. Rollback removes police
state/content/adapter/HUD/cue together; keep the safe dormant-chassis retry fix
independently. Verification and exact limits: testing/part08-police/README.md.

ADR086 verification disposition: P08-006 alone verifies the requested police states/transitions. The other9 Part08 requirements retain broad art, discretionary targeting, moving-foot capture/tackles, human fairness, offline soak and frame/hardware gates. Nearby moving-foot targets request normal braking/parking and shared-speed foot pursuit; safe capture still requires slowing below1.2m/s. Confirmed hits on the intentional-foot officer interrupt contact for.25s and retain the three-hit repeat-runner crash path. This is not full general foot combat or a tackle system.

## ADR087 — Reference-informed rider palette and wider race setup (2026-11-08)

User requests original rider color schemes, temporarily disabled player nitrous,
more grid spacing and four lanes before/after the city fork with three on each
alternative. Keep the actual 12km race paths, grade, gates, building/lot geometry,
accepted rigs and contact policy. Alternatives: squeeze more stripes into the
old10m road; regenerate city parcels; or restore the full existing12m primary
carriageway inside its original right of way. Select the last: four3m lanes
(2outbound/2oncoming), fork3350–5362.5m has2outbound/1oncoming;100m lane-centre
tapers remain. Farm asphalt expands to13.6m with0.8m paved shoulders; handling
half-width6m matches marked12m. The starting apron extends340m to support the
20m staggered single-rider intervals, player last at−288m, using both outbound
lanes at−4.5/−1.5m. Legacy track definitions remain untouched.

Restore primary asphalt by protecting it from the older1m sidewalk curb overlay;
secondary streets retain that overlay. Primary sidewalks return to the original
2.4m reservation; fine curb placement reserves a1.5m walking strip. Move parking
and larger original furniture to nearby secondary streets; no lane-blocking curb
parking is retained on the live avenue. The original road/sidewalk union and
all building footprints remain intact. Physics/collision and visuals are rebaked
together, avoiding a render-only width change. Full human traffic/art/performance
acceptance remains open.

Keep boost formulas/inputs for later use, but author player_nitro_enabled=false
in handling.json. Both authoritative adapters enforce it, including injected
drive intents; HUD/help clearly mark disabled. This avoids altering save format
or bindings. Independently adjust existing project-original shirt/bike swatches
and neutral leather gain so albedo does not obscure their hues. Shared material
identity continues across seated/walk/ragdoll and shirt-colored minimap dots.
R01 at7aecf6044f2dd773b925b325823f30d648d61c89 plus local fifteen-scheme visual
inspection are reference-only. No retail pixels, tables or source are shipped;
root license/reuse remains unverified. These are visual approximations, not exact
retail RGB/PBR or all character-selection remap parity.

Rollback world config/compiler/runtime width together; retain original authored
mesh, rig, shaders and normal/roughness maps. Restore player boost only through
the explicit flag and matching help text. Evidence: testing/race-setup/.

## ADR088 — Separate UI research media from the artist skeleton (2026-11-08)

User wants to create UI art themselves and requests all original UI/menu/image
references plus a skeleton first. Alternatives: immediately replace Bevy UI;
provide only a filename list; or build an isolated, traceable authoring kit.
Choose the isolated kit. No game frontend, economy, saves, input or physics changes.

Independent Python readers catalog all supplied IMAGES files, embedded icons,
original frontend tables/popups and native Windows dialog templates. Freeze IDs
against source paths/resource IDs and bind EXE/English strings/palette hashes.
Keep original PNG/video-thumbnail/caption references outside Git/runtime; commit
only sanitized metadata, source paths/hashes, original SVG geometry and a static
HTML/CSS/JS artist desk. No source copied from unverified-license R01. Font/AVI /
EXE/DLL binaries are not copied into the reference kit. Standard-library/Pillow /
existing FFmpeg authoring tools avoid a game dependency migration.

Original29populated frontend entries differ from36raw slots,13popup tables and
7unique native Windows dialogs repeated in two EXEs. Do not count transitions or
reconstruction-native display/pause/network pages as original menus. The new
28page RoadDash flow is a proposal with working preview navigation, source IDs,
replacement art-box previews and export/import of status/notes only. Keep live
text/metrics/widget states separate from backgrounds; source4:3 geometry is not
a mandated new16:9 layout. Original images remain reference-only in their own tab.

Keep all142resource/15component art boxes open, grouped into66master families;
code/placeholder success is not artwork or native UI approval. UI-02/P15-006/007
retain artist approval, real lifecycle/settings/controller/accessibility hooks,
resolution testing and native rendering. Cloud context, inherited backgrounds,
runtime-populated labels and unexecuted retail states remain explicitly limited.
Rollback removes the authoring kit/tracker entries without touching runtime;
retain stable asset IDs if the artwork workflow advances. Evidence:testing/ui-kit/.

## ADR089 — Remove redundant static-world and render synchronization work (2026-11-08)

Problem: the latest local race capture (performance-2247031.jsonl) degrades from
farmland into the city while multiple fixed steps accumulate per rendered frame.
Physics dominates its CPU/wall samples; GPU utilization is not attribution.
Alternatives: reduce solver quality, hide hazards, broadly rewrite the rig, or
remove repeated synchronization work while retaining authored geometry and rules.
Choose the last. Keep 60 Hz, substeps, masses, constraints, CCD, contacts and saves.

Use Rapier's native standalone static colliders for immutable Metropolis chunk
geometry instead of explicit Fixed rigid bodies. Both represent immovable solids;
retain every collider triangle, transform, friction, semantic tag and render mesh.
Leave breakable props, parked cars and every dynamic/kinematic actor unchanged.
This removes unnecessary body bookkeeping/writeback for immutable chunks, not
collision. Independently combine immutable obstacle triangles per256m chunk by
gameplay role, rather than by visual material. Preserve exact triangle order /
winding/positions, render batching, road/sidewalk/ground tags, separate farm-fence bounds and friction; never
merge breakables or actors. Record source/combined collider counts in the native
census and require exact triangle-soup equality. This reduces redundant overlapping
static pairs without deleting geometry. Installed bevy_rapier3d 0.32.0 / Rapier 0.31.0 APIs/source are the primary
reference; no new dependency, engine migration or copied implementation.
Verify paired drop/contact/ray trajectories, full native census and recovery.
Rollback is restoring Fixed on chunk entities, independent of art/source atlases.

Only update prop Visibility, HUD Text and settled facade range data on actual
value changes. Fading detail retains camera-relative margins after final camera
propagation; fully visible detail uses a stable unbounded range. Resident closed
shells and independent collision remain intact. Measure matched native runs and
keep broad PERF-08 hardware/frame-budget acceptance open.

Split diagnostics into backend sync, simulation step and writeback; include an
instantaneous backend contact/body census and independent logging-window mean.
Report per-fixed-step work separately from work per render frame so catch-up is
visible. These spans include CPU scheduling/wait and are not GPU timestamps or
an internal solver-only profile. No timers/counters alter authoritative state.

The separate UI handoff adds complete prompts for 28 proposed pages and 142 UI
image IDs / 66 master families. Original source sizes, variants and filenames
remain traceable; prompts do not imply final art, new features or approval.


## ADR090 — Persistent generated screen drafts in the artist preview (2026-11-08)

Problem: user requests ChatGPT image generation from the screen prompts and
placement into the existing UI handoff. Alternatives: generate only examples;
replace the native game UI before art review; reuse one reference-resource ID
across multiple distinct pages; or save one selected draft per proposed page.
Choose the last. Use the imagegen skill's built-in tool, one call per asset;
no API-key/CLI fallback or new library. No game, physics, save or performance
settings change.

Twenty-eight screen IDs map to selected full-canvas PNGs:25 background scenes
and3 real-alpha overlays. Retain native tool pixels/dimensions rather than fake
4K upscaling. Existing project rider/bike views plus the selected generated main
scene provide identity/style references. An initial open-helmet main variant was
rejected; no original-game UI pixels were passed to the generator. Record exact
prompts, tool output names, dimensions, alpha and hashes; keep all drafts outside
native runtime assets with production review pending.

Keep generated screen identities separate from142 retail-source replacement IDs
and15 component IDs. Embed a small generated manifest for offline/file access,
load the active page's full image, lazy-load gallery images and keep HTML text /
buttons separate. Generated overlays preview over a hash-matched project native
city screenshot. Wireframes/source-resource uploads remain accessible. Extend
progress validation for generated asset IDs, preserving existing notes/statuses.

Repackaging preserves reviewed checklist rows/notes and refuses to overwrite an
externally edited generated PNG. Originals remain in Codex's generated-images
directory; selected outputs are also saved in the repository and local kit.
UI-04 verifies generation/placement only. UI-05 owns human-approved layered /
high-resolution masters; UI-02 retains native integration. Undo by removing the
optional manifest/art layer and returning to wireframe mode; retain stable IDs
and human review history. Evidence: testing/generated-ui/README.md.

## 2026-11-08 — UI direction correction (ADR090 follow-up)

Problem: using the gameplay world/model direction produced polished frontend art the user rejected. Alternatives: continue refining those renders, or inspect original frontend screens and establish a distinct UI language. Selected: reference grotesque painted splash/shop/bar/street art, acknowledge the photographic main-menu exception, and generate one separate original direction sample. Preserve rejected images and exact prompts, disable their default display, and keep art approval/native integration open. No dependency, physics or runtime migration. Rollback: candidate preview override can be removed without touching gameplay or historical files. Reference observations and tradeoffs: docs/ui/UI_ART_DIRECTION.md.

## ADR091 — Native art-led offline frontend (2026-11-08)

Problem: approved gritty screen artwork existed only in an artist preview; the game still had a one-button garage. User requests the original game's entry structure using the new images. Alternatives: continue HTML-only mockups, port the unlicensed C++ frontend, or independently implement the screen relationships over the current game lifecycle. Choose independent implementation. Pure navigation lives in rr_core::frontend; Bevy input, art and widgets live in src/frontend.rs. Original R01 source is behavioral reference only, with no C++ or retail media copied.

Thrash opens the current course; Big Game opens a rider introduction and street hub with course, shop, bar and existing settings. Only the already-loaded course and current player/bike are active; campaign progression, purchases, other playable profiles and multiplayer remain deferred and visibly identified. Existing Profile credits/payouts, SettingsPanel, countdown/reset and physical simulation are retained. Results can retry or return to the main menu without another award.

Input is captured first, then the frontend consumes menu edges before game::input. A short Starting page hands one explicit MenuAction to the existing reset path; held confirmation cannot skip pages. Asset/world readiness prevents early launch; failed menu art produces an explicit error. Seven retained image handles avoid per-frame disk work. A letterboxed 16:9 stage preserves art proportions; live text scales separately. A dedicated default UI camera keeps the HUD/settings usable while the 3D camera is inactive in garage/results. Restoring the old entry requires removing FrontendPlugin plus the optional game/input overlay guards; saves and physics have no migration.

Art: five existing original generated scenes plus two new imagegen outputs (course postcard and rider introduction). Project MIT declarations apply to independently generated compositions; they do not assert exclusive copyright, legal review or rights to original Road Rash media. Preserve exact prompts, tool origin and source-reference IDs. Final art/layers/accessibility/hardware acceptance remains UI-02/UI-05/P15-006/007. Evidence: testing/native-frontend/README.md.

ADR091 resize follow-up: native programmatic window changes exposed Bevy 0.17's UI camera target cache retaining an earlier size until a native resize message/projection change. Refresh the dedicated UI camera Projection only when its physical target differs from the Window. Retain automatic viewport sizing; an evaluated explicit viewport alternative caused a transient oversized scissor validation error and was removed. The native fixture records computed canvas/button bounds and actual window sizes, not requested sizes. The first 2560×1440 request was constrained by the desktop compositor; this cannot count as that exact viewport acceptance.

## ADR092 — Seeded random rival starting clubs (2026-11-08)

Problem: user requests weapons on random riders; existing rivals only armed themselves by finding route pickups. Alternatives: randomize every frame, assign fixed identities, add a new RNG dependency, or select a seeded subset at the authoritative grid reset. Choose the last. GridWeapons owns a session seed and race counter; rr_core ranks distinct rival IDs by a stable seeded integer mixer. Default is half of eligible non-player/non-police rivals, controlled by validated content/weapons.json starting_armed_fraction (0..1; older files default to 0.5). A process-random std RandomState seed requires no dependency; ROADDASH_RACE_SEED allows reproduction of the sequence. The race counter changes only on start/retry.

Apply loadouts after Rider reset using existing Club inventory, sockets, visuals, windup/contact, stamina, direct-hit, pickup/drop and interruption paths. Never re-equip on pause/resume, recovery or per-tick selection. Players start unchanged, police are excluded, racer retaliation/killer policy and launch grace remain intact. No geometry, rigid bodies, hit damage or crash-energy tuning. Tradeoff: exactly half is a random subset, not an independent chance for each racer; retries reroll, so the same identity is not always armed. Existing route/drop caps remain. Rollback: set fraction to zero to disable starting clubs; remove reset hook to restore pickup-only behavior. Evidence: testing/random-rival-weapons/README.md.

## ADR093 — Begin Part09 with isolated scripted slice receipts (2026-11-08)

The user requested the next part. A clarification was offered while weapons were implemented; absent a response after useful independent work, interpret the next numbered milestone as Part09, following the previous Part08 pass. Earlier art/physics/hardware gates are not closed by advancing a development pass.

Build a standard-library CLI that runs existing native fixtures in new per-run/per-scenario directories, isolates profile/preferences, pins the race seed, strips unrelated RoadDash overrides, captures native output/exit/timeout/wall duration, and fingerprints the binary. Entry receipt validation requires real native navigation, seed/loadout, camera/root/window and return-to-menu evidence. Full mode additionally exposes existing police/recovery fixtures, whose current combined results remain an open gate until actually run. Do not implement transport, replace the approved 12km route, or present scripted checks as external play sessions/fun targets/FPS benchmarks. Alternatives: ad-hoc commands or prematurely claim a full slice; a reusable failure-reporting runner is safer and useful for later external playtest preparation. No new dependencies/assets. Rollback is removal of the independent CLI/fixture observations.

## ADR094 — Approved portrait browser (2026-11-08)

Problem: the user approved ten exact attachments plus six remakes, but runtime still displayed a generic bar background. Choose a named sixteen-card 4×4 Biker Bar picker with mouse/keyboard/controller focus, portrait details and return-to-card controls over the existing input/flow contract. Alternatives: immediately replace simulation identities or make portraits playable profiles; defer those because approved artwork is not a physics/AI/profile specification. Pure catalog/selection and wrap rules live in rr_core::frontend; Bevy presents fixed handles and live text. Existing racer identities, save formats and physics remain intact.

Store original approved files and a source/hash manifest; derive contained 768×576 UI cards without cropping or stretching square sources. Sixteen retained RGBA cards cost 27 MiB decoded (in addition to existing screen art; backend overhead unmeasured). Preload through existing readiness/failure handling; never reload disk assets every frame. New paintings were generated with the built-in image tool from user-selected style references; no original retail media or new library/source imported. Project artwork declarations do not assert exclusive rights or legal clearance. Rollback by restoring the old Bar entries/view, retaining original approved artwork and manifest. Tradeoffs: concept names/bios are UI only, not selectable profiles or conversations. Prompt source: output/portraits/selected-style-remakes/PROMPTS.md.

## ADR095 — Full-page character biographies and retro menu proportions (2026-11-08)

The user requests full-screen portraits with their earlier descriptions and a frontend closer to the original menus. Inspect the private RR-0E/RR-1A source-composition previews and catalog entries: both center portrait art; RR-1A declares a lower 640×70 interaction rectangle at y=410 on a 640×480 page. These are extracted table/default-composition observations, not captured retail dynamic text/fonts. Independently implement a 4:3 portrait/picker canvas, painting across the complete detail page, a lower live biography strip and compact Back control. Preserve the approved 4×4 choice and return-to-card focus. Other frontend pages keep their existing 16:9 geometry.

Alternatives: stretch the paintings across widescreen, crop faces, or regenerate the approved art. Choose aspect preservation and letterboxing with retained contained derivatives. User-approved originals remain unchanged; 768×576 UI derivatives still limit texel density. User character descriptions are expanded into personal quirks/race behavior following their later instruction to remove racist material; these are editorial paraphrases, not verbatim source descriptions or implemented ability specifications.

Replace the generic monospaced frontend text with Open Sans Regular/ExtraBold 1.10, unmodified from the installed Fedora open-sans-fonts-1.10-25.fc44.noarch package. Font metadata and supplied LICENSE.txt identify Apache 2.0; preserve the license/copyright metadata and register hashes. This is an era-appropriate independently licensed substitute, not a claim to the original proprietary typeface. Fonts use explicit load/failure gates and retained handles. No new crate, original game media or reference code imported.

Rollback: restore prior gallery/detail rendering, 16:9 selection canvas and default fonts; retain approved art and source approval manifests. No gameplay/save/physics migration. UI-02/UI-05 retain human art/layout, full production fonts/widgets/profile integration and hardware gates. Native testing includes all sixteen full bios at 720p and the existing entry/race/results flow.

## ADR096 — Predictive traffic intent with physical consequences retained (2026-11-09)

The user reports the field repeatedly riding into cars. The old35m point trigger reflected the lane and pursuit could overwrite cruise/braking; that ignored actual closing time, car lateral motion and an occupied escape lane. Choose a pure road-frame constant-velocity/swept-interval planner plus normal steering/service brakes in both adapters. Alternatives: lower crash sensitivity, immunity, teleporting dodges or a new navigation dependency; none addresses unsafe decisions while preserving physical consequences.

Read actual main-street car placements, crossing car world velocity and orientation-aware extents; project into the rider route/corridor. Speed-dependent3–7s forecasts include lateral crossing extrema and conservative lane-change delay. Actual hazards constrain evasion to the map-correct outbound carriageway and apply upright-rider occupancy/following to prevent queue rear ends. Hazard-free police/combat goals keep their previous width/throttle behavior; downed-body/wreck policy stays separate. Collision rules, player handling and rig tuning remain intact. No new dependency, third-party source or actor spawning. Forecasts are bounded approximations of motion, not a guarantee against unavoidable contacts; human race balance remains AI-11. Rollback is the planner/adapter hook; retain deterministic control/scenario receipts. Evidence: testing/ai-traffic-repair/README.md.

## ADR097 — Original weapon meshes as optional presentation skins (2026-11-09)

The user requests real3D models for chain, whip, pizza rolling pin and bat. Choose independent Blender lathe/tube geometry, editable sources, meter-scale grip-zero/+Y exports and three decreasing LODs per model. Existing catalog/socket/club behavior is reused instead of introducing new ability/save IDs or pretending flexible attacks exist. No stock meshes or textures were imported; source/GLB hashes and13 runtime intake rows declare original project work.

Native weapon_preview displays every GLB/LOD; optional ROADDASH_WEAPON_MODEL reuses held/pickup roots. Keep primitive fallback until SceneInstanceReady, attach before transform propagation, inherit visibility/drop/reset, and tolerate renderer-free tests. Default club remains available; unknown IDs fall back. No colliders or new damage/timing/momentum rules. Tradeoffs: chain/whip are rigid authored silhouettes, LOD selection is manual in the viewer, and per-character loadouts/family animations/automatic distance LOD/final art/performance remain open. Revert optional skin hook to roll back; no schema migration. Source and native proof: testing/new-weapon-models/README.md.

## ADR098 — Grounded foot adhesion and road-paint legibility (2026-11-09)

Actual authored KCC support crossings reproduce multi-second sticking with unconditional5m/s downward input. Choose prior authoritative grounded output to use0.5m/s supported adhesion and retain5m/s unsupported descent. Step-height/landing increases alone still stuck; the final adhesion-only control passes with the original capsule,0.30m step,0.15m landing probe, slopes/snap/nudge and all solids unchanged. This corrects on-foot intent without changing accepted rider/bike/ragdoll constraints.

The authored lane profile already is4/3/4. Rendering emitted2m fragments every4m even for solid boundaries. Emit continuous divider/edge strips and6m dashes/6m gaps; drape corners above actual pavement triangles using a startup-only spatial index. Preserve all pavement/sidewalk collision triangles, source geometry/rights-of-way and Valley quotas. Add3250m actual-entry survey view:3500m is already on the3-lane fork. Current-map audit found0buried separator centres; do not present a synthetic crossfall robustness defect as demonstrated current-map burial.

A native test placement inside a building was correctly blocked. Correct the fixture to footprint-clear public terrain/sidewalk support; do not weaken solids or expand normal recovery policy to disguise it. Production get-up volume/interior validation remains explicitly unchecked under FOOT-SPAWN-01, with ordinary incidence unverified. Rollback is the grounded adhesion helper and paint construction; no dependency/content-format/map rebake migration. Actual physical/native proof: testing/roads-walkability/README.md.

## ADR099 — Route-distributed downtown street life and reserved curb bays (2026-11-09)

Problem: the approved city had no trees near its main avenue; parking was limited to a few wide secondary streets, and walker placement used unchecked12m patrols. Earlier station-independent allocations and overconservative0.7m tree footprints wasted the narrow curb band (native destruction uses0.36m trunk diameter).

Alternatives: scatter more unchecked actors, put cars in live race lanes, widen/move city lots, or use the existing sidewalk/road reservations. Select deterministic500m district round-robin budgets, actual trunk clearance with a retained1.5m walk, and bounded96 source-authored capsule-clear pedestrian patrols. Side-street parking may reclaim only local original-road curb-overlay bays, retaining a checked1.5m outer walk. Narrow secondary bays sit beyond the live crossing sweep; primary4/3/4 lanes, all building footprints/heights/styles, paths, gates, farms and terrain remain unchanged. This is not a facade/LOD migration; atlases are byte-identical.

Tradeoffs:1494 resident props,122 parked cars and96 walkers add hierarchy/collider/skinning cost; budgets are explicit, not a60FPS guarantee. Walkers use existing NPC/ragdoll/recovery/animation; no new crowd/physics engine. No third-party code/assets imported. Rollback: compiler population helpers, baked cars/props/patrols/local bays and their runtime adapter; original planning source and legacy maps remain available. Evidence: testing/downtown-street-life/.

## ADR100 — Typed original arsenal on existing timed contact authority (2026-11-09)

Fresh Part07 pass extends Club with Chain, Whip, RollingPin and BaseballBat. Original authored schema2 data owns per-family reach/stamina/cooldown; schema1 stays compatible. Inventory/drop/pickups preserve identity; seeded grid assignment is stable, and prepared optional GLBs retain primitive fallback until ready. Shared timed side attacks preserve existing three-direct-hit knockoff rules and accepted physics. Separate damage, flexible chains/whips, specialist silhouettes and holster/equip motion remain future acceptance work rather than invented completed mechanics. No dependencies/source imports. Roll back family adapters/schema2 assets together; see testing/part07-arsenal-fresh/.

## ADR101 — Last-seen police pursuit memory and readable loss windows (2026-11-09)

Fresh Part08 inspection found unseen live progress could flip intercept direction, stale velocity drove Search past the remembered position and direction flips repeated siren/HUD announcements. Preserve memory-only unseen intent and request a stationary bounded search speed; distinguish internal Pursue/Intercept flips from new chase announcements. Show authored warning compliance and sight-loss countdowns, preserving arrest priority. Alternatives were changing police tuning/engine or weakening contact; neither fixes memory leakage. Existing one-officer rules/60Hz physics/10Hz perception/LOS/arrest/retry remain. Human fairness, rival discretion, moving capture and bespoke police art remain open. No new library/assets. Rollback is local core/native policy; evidence: testing/part08-police-fresh/.

ADR101 integration follow-up: the current results frontend swallowed captured R restart edges before game::input. Preserve the real input contract by dispatching Action::Ride from Finished/Busted through the existing Starting readiness handoff, rather than bypassing menu lifecycle or changing police reset rules. Garage/settings/not-ready contexts remain guarded and held R cannot repeatedly rearm. The native fixture now creates its own screenshot directory. The first missing-directory and second genuine retry failures are retained in combined evidence; final rerun owns acceptance.

## ADR102 — Source-table4:3 frontend formatting (2026-11-09)

User requests original-game UI formatting. Existing reference-only catalog pins R01 commit7aecf6044f2dd773b925b325823f30d648d61c89; stored RR09 controls are four160×60 cells at y420 on640×480, and RR0F uses213/213/214×60 cells. Tables are evidence of stored geometry, not captured dynamic retail rendering. Replace the generic16:9 floating title/body cards and inset button bar with an all-menu4:3 letterboxed canvas, art-first page, compact lower copy, full-width bottom controls and gold text/underline focus. Main gets four stored-proportion choices; street gets bar/shop/race. Esc/back/controllerEast and F2 retain quit/options access. Other supported pages adapt the band to their real available actions; do not pretend unimplemented courses/campaign/purchases exist. User's16-card4×4 picker/full-page dossiers remain intentional overrides.

Use unmodified Jost3.7 Book400/Bold700 from pinned35f141c970538f1ed0f235789c19156b3ce2a762 under verified OFL1.1 as an original-font substitute. Existing retail FUTR/FUTB/BADLOC binaries remain reference-only/unshipped; exact glyph/color/dynamic handler parity is not claimed. Centre-crop existing16:9 art into4:3, preserving aspect and original asset bytes. No generated/replaced painting, physics/input authority/profile/roster migration. Alternative was retaining wide cards or stretching art; neither preserves original proportions. Tradeoff: some approved edge composition is cropped; final art/readability/human/controller review remains UI-02/UI-05, and HUD/pause/options full original-format parity remains open. Roll back layout module/font handles together; retained OpenSans and original art unchanged. Evidence: testing/original-ui-format/.

## ADR103 — Modern presentation after rejected literal-retro UI (2026-11-09)

User rejected ADR102's dated appearance. Supersede literal4:3/source-table entry presentation with full-window artwork, clean warm-white typography, a right-side main action stack, inset secondary actions and restrained rust-orange focus. Keep original-inspired approved paintings/navigation and the explicitly requested4×4 portraits/dossiers; those keep4:3. Remove hard title shadows, red block headers, black strips and yellow full-width underline. Use native Bevy0.17.3 BackgroundGradient/LinearGradient for continuous readability scrims; no new artwork/dependency/shader. Fit actual loaded image dimensions proportionally to viewport (cover), and scale text from the smaller width/height reference so ultrawide does not inflate it. Avoid rewriting unchanged Node/font values each frame. Actual Bevy installed examples/ui/gradients.rs and UI gradient declarations inspected; existing R08 dependency/license retained, no upstream implementation copied.

Alternative was modifying the approved painting or applying more retro decoration; user criticizes UI presentation, so preserve image bytes and interaction authority. Tradeoffs: some background edges crop on unusual aspects, final density/contrast/crop/art approval is human; no measured whole-game FPS claim. Rollback: frontend layout/focus/resize presentation only. Current gameplay,retry,profile,physical rigs and font licensing remain unchanged. Evidence: testing/modern-ui-polish/README.md. ADR102 remains a historical rejected presentation, not current design.

## ADR104 — Grouped functional settings with protected persistence (2026-11-09)

User requests a settings menu after the modern UI pass. Reuse the existing Preferences/Bindings/pause contract and render a separate Camera/Controls/Audio/Display editor with the same licensed Jost/warm-white/orange palette. Keep camera comfort/validation and add optional version1 fields master_volume/audio_muted/fullscreen/vsync. Old version1 files default to1/false/false/true; no load-time rewrite. Earlier builds may reject added fields as read-only; retain backups on rollback. Invalid/future/unreadable files remain protected.

Add real linear master gain and persisted mute; use an unscaled CueGain on one-shots and the existing loop base gains so already playing/new audio both respond without repeated attenuation. Installed Bevy0.17.3 GlobalVolume documentation states it only affects newly started playback, so changing that resource alone was rejected. Default master1 preserves the previous mix. Fullscreen uses borderless current-monitor mode with cached windowed bounds; VSync uses native AutoVsync/AutoNoVsync. Preserve explicit --world-survey display override. Driver/fullscreen/hardware behavior needs separate acceptance; no FPS claim or quality-slider placebo.

Existing input mappings stay stable; add optional pickup/drop editor rows and retain reserved keys/duplicate rejection. Tabs navigate only visible rows; keyboard,mouse actions and one deterministic controller owner share the handler. Category navigation uses PageUp/Down/RB; Tab/Y switches binding editor device. Modal input remains suppressed before fixed capture, races/countdowns pause/restore exactly, and the opaque settings screen suspends3D rendering without changing simulation/rig tuning. Save failure stays visible/open/paused; explicit close without saving retains session values but leaves the file unchanged. No ambiguous discard/reset behavior. Camera defaults reset only comfort.

Alternatives were decorating the old22-row list or adding unsupported graphics/audio buses; neither provides a truthful usable editor. No backend/dependency/art migration or external code/media import. Tradeoffs: two-column bindings validated at720p+,per-bus audio/quality presets/mouse or kick rebinding and human/controller/Windows/monitor acceptance remain open. Rollback is optional preference extensions,render/input adapters and master-gain multiplication; restore prior settings backup if using an older build. Evidence: testing/settings-menu/.


## ADR105 — Retained-only facade selection hysteresis (2026-11-09)

Metropolis additive authored detail granted radius/ranking bonuses to every active
group, including groups already retiring. Near a face plane, sign changes also
reversed selection without a deadband. Keep the existing 25m radius/20m ranking
bonuses only while a group is wanted; require retiring groups to qualify as new.
Retain wanted faces until the camera is 2m behind their plane. New faces still
require a front-side camera. This suppresses tangent-camera jitter without
changing source geometry, materials/atlas hashes, collision, fade duration or
mesh/triangle budgets. Resident envelopes remain independent. Regression tests
cover tangent jitter, exit threshold and retiring radius/rank behavior. No solver,
rig, engine, dependency or asset change. PERF-08 remains open; native CPU/window
measurements are not GPU timings. See testing/polish-12/pass01/README.md.


## ADR106 — Paired pre-solver motion for downed contact eligibility (2026-11-09)

The contact adapter used mounted-bike pre-solver motion but downed wreck/ragdoll
post-solver motion to decide the existing3m/s ride-over threshold. A focused ECS
regression reproduces a false lift with2m/s matched pre-contact speeds and a
solver-changed target at−5m/s. Conversely a target accelerated to the bike's
10m/s could suppress a genuine approach. Use the existing PreviousMotion sample
for both eligibility and the existing energy response; retain current-velocity
fallback only if that component is absent. No new domain rule or constants.

Keep car18km/h and wall50km/h gates, rear60km/h front-rider knockoff/shared
normal momentum, downed lift cap/energy accounting/cooldown, reverse and remount
policy unchanged. No body/joint/motor/velocity-muscle tuning, dependencies or
assets. Rollback is this adapter sample selection. Broader confined clearance,
human feel and hardware approval remain open; synthetic solver fixtures are
not a measured ordinary-game incidence rate. Evidence: testing/polish-12/pass02/.

## ADR107 — One authored smooth Metropolis elevation profile (2026-11-09)

Pass03 of the sequential polish replaces duplicated Python/Rust sine formulas
with one serialized knot profile (`metropolis_elevation.json`, embedded in the
approved planning layer and baked runtime manifest). Quintic interpolation has
zero first/second derivatives at crests; broad farm rises lead into the climbing
urban approach and long city slopes, with no separate hills district. rr_core
owns interpolation; rr_content rejects malformed/steep profiles and stale path
heights. Both route adapters sample the shared land for lanes and the extended
grid; traffic navigation heights are recalculated after street projection.

Preserve exact approved 2D source geometry, nodes' XZ, edges' XZ, blocks, IDs,
rights of way and building envelopes; reauthor only the elevation layer, then
run the normal Metropolis bake. Subdivide the formerly single starting apron
and field surface triangles at32m. Field boundaries/fences remain their original
rings; only visible/contact surface patches change. Check field union/overlap
coverage offline and area at intake. Existing8m ground and32m streets remain.
No rig, dynamics, controls, material/facade source or dependency migration.
Native and geometric evidence: docs/testing/polish-12/pass03/README.md. Broader
terrain/full-route/art/hardware acceptance stays open under existing requirements.

## ADR108 — Unified late-afternoon lighting profile (2026-11-09)

The sky adapter already replaced world lighting and exposure, but sun direction,
colors and the camera's tonemapping lived in separate defaults. Author them in
one bounded rr_content configuration and apply the complete profile in existing
PostStartup installation. Share sun direction with the existing cubemap disc.
Use 42klux warm sun at34.7 degrees,3400 cool ambient,1500 environment intensity,
EV12.7 and explicit Reinhard. Retain Reinhard's gradual highlight rolloff; avoid
contrast-heavy ACES and LUT-dependent operators/new embedded media. Supported
analytic operators are an enum, unsupported LUT choices rejected at intake.

No time-of-day system, autoexposure, shadow/cascade tuning, clouds/fog overhaul,
new lights or passes. Existing drifting sky still rotates the disc independently
of the fixed light over time; pass10 owns that known coherence issue. No facade
materials or geometry changed: model-derived atlas rebake is not needed. No
physics, save, dependency or asset migration. Tests assert actual installed
camera/light settings and unchanged shadow policy. Native evidence and cost
limits: testing/polish-12/pass04. Human art approval remains open.

## ADR109 — Bounded shadow contact bias and cascade blending (2026-11-09)

Pass05 preserves the pass04 lighting/exposure and Bevy0.17.3 four2048px
cascades,10m first split and150m maximum reach. Reduce texel-scaled normal bias
from1.8 to1.5 to tighten wheel/foot/foundation/vegetation receiver contact;
retain0.02 depth bias as acne protection. Increase overlap from0.20 to0.25
for a wider transition between existing maps. This can increase overlap work;
it does not add maps or widen maximum shadow reach. Nearby geometry retains
the finest map. Existing ornament/coarse-facade NotShadowCaster exclusions
remain; resident envelopes, collision, trees and rider/bike casters unchanged.

Installed Bevy cascade implementation already stabilizes projections by rounding
cascade diameter and snapping to texels. Do not add a competing camera/light
snapper. No PCSS, SSAO, new lights, runtime quality toggles, dependencies or
material/atlas changes. Local dependency/API research only; no external code or
media imported. Native paired evidence/cost limits: testing/polish-12/pass05.
The initial1.0 normal-bias trial exposed faint terrain self-shadow patterning
in native recovery and was rejected;1.5 retains a more conservative margin.
Rollback changes only world light bias/cascade config and its integration
assertions. P15-004 retains presets, temporal motion/hardware and art acceptance.


## ADR110 — Shared measured-scale road materials on existing surfaces (2026-11-09)

The legacy road installer queries RoadChunk render meshes, while Metropolis
separates render chunks from RoadChunk collision owners. Consequently default
roads remained flat-color despite the original asphalt PBR implementation.
Create the shared asphalt material directly when Metropolis builds its road
material slot. Use world-coordinate UVs before256m origin rebasing and generate
valid tangents for its normal map. All triangle positions/indices, pavement
height projection, collision roles/friction and marks remain unchanged.

Use8m original aggregate tiles, quiet1.28–1.84m repair patches and short tar
seals. Remove repeated world-axis tire bands. Preserve complete linear-filtered
mip chains. Paint adds bounded opaque wear at0.5m world scale; no alpha holes,
new blended passes or overlays. Existing1m sidewalk slab seams receive restrained
slab tones and edge-dirt gradients; they are visual joints, not drainage physics
or new grates. No facade material/atlas, dependencies, rig/simulation or licensed
media changed. Model-derived facade source hashes/mips remain byte-identical.

No new road meshes/entities/draw groups; default roads now sample three PBR
maps and paint two small albedo maps. Native wall-frame measurements cannot
isolate GPU texture cost or establish shimmer-free motion. Physical drainage
hardware/full curb authoring and human/hardware approval remain P11-004 gates.
Evidence: testing/polish-12/pass06. Rollback is the material/UV/paving change.

## ADR111 — Source-compiled facade finishes and effective roughness parity (2026-11-09)

Coordinate the24 placed Metropolis facade families at their original-model front
compilation boundary. Keep original full models, editable blends, texture tiles,
geometry streams, shade variants and architectural openings. A shared offline
policy makes painted metal/masonry dielectric, retains restrained gold trim,
quietens masonry by6% in linear color, slightly neutralizes concrete, and lifts
opaque glazing/interior colors for readable existing entrances. No transmission,
new overlays, geometry or runtime recoloring. Original hues/signage remain.

Refresh72front and72storey GLB material headers directly from source materials;
assert identical BIN bytes and every nonmaterial JSON field. The regular front
compiler consumes the same policy, so a later normal rebuild reproduces it.
Only current24families change source materials; legacy unplaced designs retain
source finishes. Rebake all383far bands/five mip levels and Metropolis crops.
Add a crop-only mode to the existing Metropolis baker so this material-only work
cannot alter world geometry/elevation. Hashes include the effective profile input.

The runtime already replaces textured roughness factors with provenance-linked
micro-roughness maps. The old far baker ignored that operation. Bake the identical
profile lookup/quantized micro-roughness instead; a shared fixture checks the
actual Rust maps and Python baker values. Distant geometric normals still omit
subpixel procedural micro-normal relief; this is an intentional LOD approximation.
Metropolis previously cropped only albedo and forced roughness0.84/metallic0.
Crop packed factors at the identical24family/three-band/five-mip mapping and
sample them in its existing facade shader with matching gradients/gutters.
Use linear RGBA8 (G=roughness/B=metallic), one extra texture sample and6,285,312
resident pixel bytes; no new draw groups, meshes or lights. This removes the
fixed-finish mismatch without changing silhouette/collision/ready lifecycle.
No physical rule, dependency or imported media change.
Human architectural/art, temporal/long-streaming and hardware/performance gates
remain open under MAT-03/PALETTE-01/LOD-03. Evidence: testing/polish-12/pass07.

## ADR112 — Low-rise use transition within approved property envelopes (2026-11-09)

The existing height cap grows the first1.8km skyline gradually, but source-role
selection gave even capped two-storey interior towers office facades. Keep the
height profile and all approved geometry. Blend existing24-family palette use
from roadside businesses to workshops, residential fronts and original city
pools across authored northward stations−180/250/650/1100/1800m. Only buildings
of at most five storeys before1800m participate. All taller buildings and all
assignments beyond1800m retain the previous source-role rule exactly.

Three consecutive authored footprint-index groups share a deterministic use
choice; individual building IDs choose family variation. These groups are an
index coherence device, not a claim that arbitrary polygon indices guarantee
spatial adjacency. Finite normalized station weights and eligible non-office
pools are checked in rr_content and the offline compiler. Existing setbacks,
fields/fences, vegetation, closed collision envelopes and access gaps remain
unchanged. No new clutter, mesh, texture, material or light allocation.

Regenerate Metropolis through tools/bake_metropolis.py from the unchanged approved
LAYOUT.json. The unchanged source families/GLBs/far atlas and both pixel crops
retain identity; palette provenance changes. This is district-use presentation,
not a layout redesign or newly authored farm buildings. PALETTE-01/V2-04-003
retain complete site/transition art, human review, streaming and performance
acceptance. Evidence: testing/polish-12/pass08. No external media/code imported.

## ADR113 — Near-readable haze and exposure-matched horizon (2026-11-09)

Pinned Bevy0.17.3 applies fog after exposed PBR lighting and before tonemapping;
its skybox multiplies sampled linear colour by sky brightness and exposure. The
previous fixed fog colour ignored that scale and became completely opaque at
1050m. Use built-in exponential-squared distance haze at density0.00075/m:
99.44% scene contrast at100m,98.19% at180m,53.79% at1050m and16.16% at1800m.
This gradually separates distance layers while showing more distant geometry.
One shared authored horizon sRGB colour (unchanged0.68/0.76/0.82) supplies both
cube generation and fog. Convert it to linear and scale by actual sky_nits and
Exposure::exposure for the fog target. Do not independently retune lighting.

Native evaluation exposed a second mismatch: the existing non-HDR camera
applies Reinhard inside the PBR shader, but the skybox writes exposed colour
directly and clips. Trialing Hdr on the camera produced black frames throughout
the native survey despite exit0 and no shader/pipeline errors. Reject that
candidate; do not broaden this pass into a renderer migration. Keep the supported
non-HDR path and document that shared radiance improves the seam but does not
make sky and fog display colours identical. Pass10 must evaluate a coherent
sky-output tonemap using the same profile; do not compensate with inverse-fog
colour hacks. Preserve rejected-hdr images/logs and intermediate non-HDR receipts.

Validate finite density0.0004–0.001/m and colour channels in rr_content. No added
volumetric passes, textures, meshes, lights, directional-scattering loop or
visibility/collision changes. The shader substitutes an exponential evaluation
for a clamp; measure bounded native wall frames without claiming GPU cost.
No facade source/material change, so no atlas rebake is required. Existing
city_preview diagnostic lighting remains separate. Content config is authored
and embedded, not a player save; replace obsolete linear endpoints in-place.

Sky/cloud design and sun-disc drift stay with pass10; share the horizon parameter
and preserve its radiometric relation if that palette or brightness changes.
No external source or media intake. SKY-01 evidence is extended, no new IDs,
closures or task moves. Human racing-speed readability and broad art/performance/
hardware gates remain open. Evidence: testing/polish-12/pass09.


## ADR114 — Display-referred procedural sky with stable solar direction (2026-11-09)

Preserve the supported non-HDR renderer after ADR113's rejected black HDR trial.
Bake the exposed analytic tone operator into a separate256px sRGB display cube;
set Skybox brightness to reciprocal camera exposure so Bevy's built-in multiplier
leaves that display-linear value unchanged. PBR fog uses the same horizon radiance
and Reinhard operator. Keep original radiance/specular and16px diffuse cubes,
intensities and all lighting settings unchanged. This is authored static exposure;
a future dynamic exposure change must rebuild the display cube together with fog.

Author the visible cube in world directions, compensating for Bevy's Z-reversed
cube sampling. Previously the sun disc was mirrored north/south and the entire
sky/environment rotated as wind. Remove that update system: sun, horizon and
lighting now remain world-fixed. Hold clouds stationary until independent cloud
motion is implemented; retained legacy wind config is explicitly inactive.
Avoid per-frame texture uploads or new custom render passes for this bounded fix.

Original continuous direction-domain banks with fine lobes, sunward shading and
a weak high veil replace the visible single-scale mottling. Directional warmth
and clouds fade out at the horizon to its exact shared fog target. Separate sky
presentation avoids changing accepted environment illumination. One additional
2,097,144byte cubemap with nine mips is generated once; no added draw call or
per-frame sky sampling, meshes, lights, collision or visibility changes. Angular
cubemap filtering remains subject to native/hardware review at extreme minification.
SBDT fallback is adapted from the installed licensed Bevy analytic implementation;
attribute Tomasz Stachowiak and retain upstream notices. No external media intake.
Full landscape, independent cloud motion, human art/temporal and broad performance/
hardware gates remain open; SKY-01 evidence extends without checklist closure.

## ADR115 — Authored ankle pivots for render-only boot articulation (2026-11-09)

The distal skin adapter approximated both shin-to-ankle offsets as0.50m,
although the immutable source animation chain places them at0.558046m. This
rotated boots around points above the ankle: a0.6rad distal flex displaced the
anatomical ankle by34.3mm. Use the authored endpoint distance in runtime skin
binds and their paired inverse binds. Keep the original weights, mesh geometry,
materials, eleven base frames and physical16/11body rigs exactly as before.
No wrist twist, motion-library rewrite, physics/muscle/recovery tuning or dependency.

The source-space sole/grip landmarks are independently retargeted across normal,
tuck, steering blend, run, get-up and both pickup clips; check inverse-bind neutral
identity and physical/visual differential fixtures. Native current walking/running,
standing, mounted and recovery views supplement these numeric clips. This is a
bounded articulation correction, not certification of whole mesh clearance,
civilian volume seams, full gait quality or final art/performance/hardware gates.
Evidence: testing/polish-12/pass11. Rollback changes only the two distal skin pivot
distances and their regression; no authored asset/schema migration.

## ADR116 — Confirmed-contact presentation recoil without physical authority (2026-11-09)

Successful mounted melee contacts record one replaceable race-clock pulse on the
victim. rr_core owns finite duration/envelope and signed local impact direction;
the visual adapter layers a small spine/chest/neck/head rotation onto the current
riding pose. Punch peaks emphasize head recoil; kick peaks emphasize trunk fold.
Root/pelvis/feet remain unchanged, shoulders inherit the connected chest, and
existing two-link fitting retains held grips/pegs. The pulse never writes physical
transforms, velocities, drive intent, cooldown, immunity or recovery.

Attack clips take priority to preserve released strike contact landmarks; the
reaction ages underneath rather than delaying/queuing input. Decisive knockoff
clears the ordinary pulse and retains existing stagger/crash presentation.
Recovery/finish/inactive race phases discard pulses, pause freezes the race clock,
and retry reconstructs Rider. Replacement prevents additive stacking. Misses and
LOS/immunity rejection do not trigger a pulse, impact audio or camera cue. There
is no defensive block mechanic to invent; obstruction remains a miss. Existing
confirmed-hit punch/kick sound and bounded camera shake remain on the same
contact event. Kick push and three-direct-punch knockoff rules are unchanged.

No engine/dependency, audio/media, physical rig, model/bind or authoring asset
change. Opt-in native reaction smoke uses temporary logical attackers and the real
LOS/melee resolver to isolate confirmed victim reactions; it bypasses windup and
is separate from the existing real keyboard contact-clock fixtures. Broad
contact geometry, human impact/audio/balance/art and hardware/performance gates
stay open under COMBAT-FEEL-01/P05-006/ANIM-09; no new IDs or closures. Evidence:
testing/polish-12/pass12. Stop after the twelfth polish pass.

## ADR117 — Bounded original farm grass and continuous shoulder color (2026-11-09)

The user explicitly requested a new landscape pass after the twelve polish passes,
using pastoral grass/road-edge references. Keep Bevy0.17.3 and the accepted world,
collision and physical rigs. Investigated warbler_grass (Bevy0.13),
bevy_procedural_grass (0.12.1), and GEMI/bevy-grass (0.20); pinned source/license
receipts are in testing/landscape-graphics/research. None is imported. New grass,
soil geometry and spatial color functions are original project code. Supplied
screenshots inform density, irregular edges and palette only; no reference media,
models, texture pixels, terrain or protected UI is copied.

Use opaque two-sided five-blade tufts with independent deterministic placement,
density and size seeds, coherent pasture/grain/fallow colors,32m batches and native
65–110m dither visibility ranges. Avoid grass shadow cascades, transparent cards,
per-frame uploads, new shaders and collider changes. Retain continuous ground
color and soil-edge ribbons beyond the tuft range. Shared original64px flecks
with seven mips add21,844bytes and no new render pass; shoulder outer vertices
use the same field material, UVs and color function. Dense0.6m verge candidates
replace sparse1.2m candidates within12m; farther fields stop at34m lateral extent
to concentrate the geometry budget near the racing camera. Soil ramps meet the
actual6.8m farm pavement, above the retained physical ground without adding a
collider. Exclude field holes, roads,
access surfaces, building footprints, barns and fence margins. Unplanted verges
are eligible only outside field bounding boxes; this conservative rule may leave
some extra bare space. Caps fail explicitly during generation instead of silently
truncating the end of the course. Census records actual tuft/batch/triangle counts.

Terrain surface vertices/indices, field polygons, building envelopes, source
layout, Valley quotas and both legacy maps remain unchanged. No facade changes
require an atlas rebake. This is scoped V2-06-005/006/007 and WORLDART-01 evidence,
not completion of the entire landscape/NPC/art/performance gates. Wind, new trees,
stone walls, terrain silhouette remodeling and full-map vegetation remain future
work. Verification and measured limitations: testing/landscape-graphics/README.md.

## ADR118 — Rural gravel verges and whole-field farm placement (2026-11-09)

User review found the asphalt/grass transition abrupt and cows overlapping fences,
and requested reuse of existing farmland props. Extend ADR117 with a light gravel
shoulder about1.5m wide, a graded short-grass verge and an irregular outer edge.
Set tall tuft candidates back beyond the gravel. Keep actual polygon clipping,
shared ground texture/mips, source road/terrain geometry and physical support.
This is a rural shoulder, not an urban raised sidewalk or a new collision curb.

Replace the fixed16m cow offset with deterministic whole-body field-interior
clearance. Preserve population, physical rig and reset/recovery ownership. Root
positions now rest at field surface height; fences require body radius plus1m.
Reuse the existing BRNGRN2, WINDVN2, HAYDOUB2 and HAYTRIP2 original catalog models
in four small field groups, with16 independent fixed obstacle owners using fitted
bounds. Exclude authored roads/access/building polygons, old barns, parcel holes,
fences and other farm occupants; new grass excludes these same placements.
Existing world layout/source buildings/colliders remain; the16 prop colliders are
explicit additions, never hidden by LOD. No external asset/code intake, dependencies,
facade edits or source-layout migration. Full-map art/performance acceptance stays
open under V2-06-003/006/007, ENV-008 and WORLDART-01. Evidence: testing/farm-verges.

## ADR119 — Helmeted matte clothing and curved wind grass (2026-11-09)

User requested humans and grass closer to the supplied pastoral low-poly reference,
then explicitly selected helmets with stylized clothing. Keep original authored
surfaces, bind frames, weights, glove morphs and both physical rigs. Render-only
cloth partitions use canonical standing height, retaining every source triangle
and skin influence. Matte rust-red player jacket, independent rival palettes,
charcoal trousers, dark boots/gloves and faceted normals replace shiny armor-like
cloth cues. Vertex tint gives broad cloth variation, not a new sculpted garment or
high-resolution plaid texture. Helmets and visor remain. Human prop skins receive
cached flat-normal/matte variants; cows retain their existing presentation.

Extend ADR117/118 with narrow curved six-blade tufts, variable green/dry tips,
root-darkening and bounded GPU wind. Use pinned Bevy0.17.3 ExtendedMaterial and
standard PBR shading; no dependency migration. Roots remain fixed; maximum tip
sway is below0.078m and conservative bounds include0.08m. Opaque two-sided blades
avoid alpha overdraw and shadow casting. Keep32m batches,65–110m visibility ranges,
road/fence/building/whole-farm-occupant exclusions and independent collision.
Near candidates use0.5m spacing. Geometry cost increases and must be reported;
this is visual work, not a claimed performance optimization. No facade/source
layout/material atlas changes are involved.

Research-only: Foxtrot, warbler_grass and Saddle Rendering Grass pinned receipts
are in testing/stylized-humans-grass/research. Their Bevy versions differ from the
installed engine. No repository implementation or proprietary screenshot pixels
are imported. New code/shader is original, using installed Bevy interface APIs.
Existing ART-04, V2-06-007 and WORLDART-01 remain partial; art acceptance, character
silhouette remodeling, full-map vegetation and hardware/performance gates stay
open. No new IDs or task closures. Evidence: testing/stylized-humans-grass.

## ADR120 — Portrait cast and living city integration (2026-11-09)

User explicitly requests major AI/cast/loadout changes, safe reversing, urban-only
population growth, crossing-car speed correction, progressive roadside buildings,
City Hall with arches, a fountain park, covered cafes,20%wider sidewalks and clear
lane/crosswalk geometry. Extend the current polish branch without merging main.
No engine/dependency migration or physical ragdoll tuning. Keep stable content IDs,
save format and the15rider race contract. Fourteen active slots use approved bar
portrait names; George and Elif stay in the16portrait bar cast. Weapon/behavior
choices follow described riding/fighting/occupation, never demographic traits.
Use existing weapon families; no invented new combat abilities. Preserve seven
aggressive and seven race-focused opponents with exact-identity retaliation.

Pure fixed-tick driving memory lives in rr_core: slower committed lane movement,
description-based pace/patience/kicks and timed low-speed backing only when the
rider wants progress and a full rear shape sweep is clear. Use the existing reverse
force adapter, no teleport/recovery shortcuts. Traffic queues must not trigger
backing. Reset history with the existing Rider restart lifecycle.

Crossing paths wrap140m. Reset Rapier current and next kinematic poses during
recycling instead of representing that displacement as a physics velocity.
Apply the same reset to main traffic recycling. Add32urban-only main cars and
increase city patrol budget96->192; retain original64-car baseline distribution
through farms. Populate additional city crossings without increasing farm actors.

Bake source-derived civic sites and approach buildings through bake_metropolis.py
using content/world/living_city.json alongside the approved planning source.
User corrected the first small-public-lot interpretation and explicitly selected
park 3 (LC-S0152) for City Hall and park 5 (LC-S0246) for the detailed park.
City Hall faces the actual adjacent driving street to the east/southeast, with a 174m frontage, 129m deep foundation,
116m clock tower, pediment and grand arched colonnade. Other parks remain.
Civic site validation now permits bounded sites up to400m to fit these real park blocks. Original resident civic geometry
has persistent compound collision and actual open archways. Existing model-derived
facade atlas/materials remain; geometry uses new source footprints with the same
near/far family contract. Sidewalk expansion grows landward, with bounded building
setbacks retaining all old IDs/envelopes and lane widths; precise geometry checks
own acceptance.20%area is an approximation to width on irregular junctions, not
an assertion that every cross-section is exactly20%wider. Cafe placement retains
an open1.5m walking strip. New pavement-aligned zebra crossings and stronger lane
paint share actual carriageway triangles. No proprietary media or external code
intake; all additions are original procedural models/code and existing project
assets. Art/AI fairness, long-streaming and hardware/performance remain gates.

ADR120 final user clarification: city-facing means facing the actual driving
street, not the westward city centre. Derive the entrance heading from the nearest
authored traffic path instead of a cardinal-direction guess. Author full-block
civic ground patches from each selected source polygon minus road/sidewalk,
triangulate in32m tiles, validate Shape data in rr_content, and render those exact
patches at terrain height. Do not substitute an inscribed rectangular lawn.
Keep other parks unchanged; extend park5 planting/paths throughout its footprint.

## ADR121 — Connected civic grounds and live AI escape routing (2026-11-09)

User rejected the isolated City Hall plaza and disconnected park paths, and
confirmed stuck AI occurs both mounted and walking. Keep the approved park3/5
locations, road-facing architecture, previous cast/density work and all old IDs.
Move the hall24m toward the race street while retaining its whole foundation and
stairs inside the block. Frame it with31original low-rise closed-envelope buildings
using existing facade families and ten trees; replace most open paving with lawn.
Bake one park path network joining six actual sidewalk points and an annular
fountain path, using source road/sidewalk polygons. Retain full-block grass.

Pure bounded search, progress memory and walking stall detection live in rr_core.
Search49×49cells at3m spacing, at most1024expansions and one plan per fixed tick
per mounted/walking subsystem. Physics adapters supply live swept-body/capsule
clearance; route segments are rechecked while moving and blocked routes expire.
Mounted recovery detects lack of forward progress so repeated backing oscillations
do not hide deadlock. Follow world-space waypoints through the existing vehicle
forces and bounded heading assistance; reverse is latched per segment. Ordinary
short traffic queues wait; prolonged gridlock may seek an alternative after10s.
Walking NPCs replan when blocked or stationary; manual/player recovery is unchanged.
No rescue teleport was added. Existing exceptional on-foot rescue remains unchanged.
No ragdoll masses, joints, muscles, accepted player controls or saves were tuned.

No dependency migration, external code or media intake. Existing CAST-AI-01 and
CITY-CIVIC-01 acceptance is clarified, not closed; art, general deadlock freedom,
long races and hardware/performance remain open. Search may fail in a sealed or
larger-than-local enclosure; it must stop/retry rather than return a wall-crossing
route. Model changes reuse existing baked facade families/atlas source contracts.

## ADR122 — Civic planting and reviewable city-width reduction (2026-11-09)

Increase hall planting to 120 trees using actual lawn/paving clearances; retain
all 31 context buildings and public access. Add park interior trees with furnishing
exclusions. Civic blade patches reuse the existing 32m grass batches, 65–110m
visibility range, shared opaque wind material and existing budgets; turf remains
continuous. Trunk colliders/navigation use the same positions; no physics tuning.

The user wants a map of a narrower city, then specified about half the blocks
which do not touch main race roads. CITY-WIDTH-01 is a review proposal, not a live
map deletion. Protect whole blocks within 45m of either race centreline (conservative
road/walk frontage), plus all six park sites. Rank other whole blocks by distance
from the race corridor; propose the farthest 136 of 273 non-frontage blocks.
This preserves 194/330 blocks and 11,275/19,108 buildings. Keep both branches,
source planning and the live manifest's buildings untouched. Edge streets and
terrain transitions need design after selection. Counts do not imply FPS gains.

Character gallery uses existing approved portrait art and actual exported GLB
review sheets. Do not imply the 16 portrait identities already have distinct 3D
rider meshes. No external media/code import or runtime dependency migration.

## ADR123 — Fresh-agent civic, lighting and race follow-up (2026-11-09)

The user requested separate contextless GPT-6 Astra/medium workers for each
follow-up. Isolated branches are reviewed and integrated by the parent, with
serialized native measurements and final combined regressions. This is the
active follow-up, not advancement to the next major roadmap part. The earlier
city-width reduction remains a proposal; no blocks are removed.

Restore original biker presentation from6039be4e, retaining helmets and the
accepted physical rigs. The rejected render-only clothing style is removed;
later world/grass work remains. Give civic ornament five resident opaque batches
rather than individual entities: limestone mouldings, window surrounds, doors,
clocks and copper roof articulation preserve the existing hall pose, open arches,
31 context buildings and120trees. Original periodic surface maps and metre-scale
UVs apply to those resident batches. No facade family/atlas geometry changes;
MAT-03 still governs any future facade changes. Art acceptance remains open.

Use the existing pinned Bevy0.17.3 environment-light API with original, once-built
cosine diffuse and GGX specular prefilters of the same procedural sky. Correct
cube handedness; preserve direct sun, MSAA, exposure, haze and shadow policy.
ROADDASH_LIGHTING=legacy retains the earlier profile. This is static global IBL,
not scene-aware GI. SSAO/temporal-AA migration is deferred; research and costs
are in testing/next-lighting. No dependency or external media intake.

Metropolis keeps the full12km physical map and both routes, but its playable
interval is500..11500m. Shift the existing grid by500m; retain relative spacing.
Remove visible checkpoint structures and their dedicated colliders/HUD, retaining
internal ordered route validation. Legacy race intervals/presentation remain.
RACE-TRIM-01 tracks integrated start, retry, endpoint and branch acceptance.

The entry-road defect came from a late asphalt overlay under raised sidewalks,
not merely incorrect lane paint. Use final road/walk partitioning and a smooth
4→2 lane taper at2800..2900m ahead of the2970m city envelope; retain subsequent
city widening. Traffic centers, full car footprints and paint use the same road
profile. Preserve all building envelopes and landward walks. Equal-progress cars
use stable merge priority and look ahead for shared downstream lanes. Geometry
sampling alone does not close native traffic/AI or performance acceptance.

Parent integrated tests, provenance and native review remain required; worker
source commits and historical captures are not final combined acceptance.

## ADR124 — Persistent, sparsely updated field deformation (2026-11-09)

Use original static wheat meshes in grain parcels within72m of the farm course,
with16m batches and1m damage cells. Each cell stores bend direction, amount and
first-contact time; the shader animates a0.32s bend. Fixed-step swept physical
bike/rider/wreck/limb shapes stamp planted cells. Vertical separation, disabled
bodies, teleport-sized displacements and pause are excluded. Marks survive actor
removal and culling, lasting until race reset; no save schema change. Grain-only
undergrass is shortened so it cannot hide the flattened canopy.

This avoids per-stalk entities, dynamic mesh rebuilds, crop colliders and ongoing
simulation per stem. State is capped at2048tiles/8MiB per damage copy; current
geometry uses635tiles/2.60MB and885,312allocated triangles, with distance culling.
Only dirty4KiBtiles upload; the shared clock is16bytes. Stable COPY_DST GPU buffers
are necessary: native review rejected an earlier asset-replacement path whose
unchanged material bindings still read stale allocations. Startup-resident buffer
handles live across retries; future runtime map teardown must also clear queued
render writes. Research and rejected/accepted evidence: testing/next-crops.

This is a bounded selected method, not a proof of globally fastest vegetation.
One material/draw per visible tile remains a cost; a custom instanced atlas could
reduce it after profiling. The native fixture verifies physical-shape contact,
visible marks, persistence, pause/reset and stationary zero tile uploads; it is
not a free-running crash or GPU timestamp benchmark. User art approval, broad
hardware/full-course performance and the16.7ms target remain open under
CROP-TRAIL-01. No external code/media or dependency migration.

## ADR125 — Height-only moderate relief enhancement (2026-11-09)

Scale longitudinal height displacement above the existing10m datum by1.25;
retain knot stations, quintic C2 blends and0.8m lateral crossfall. Relief rises
52→65m; maximum analytic longitudinal grade3.00→3.75%, below the existing4% bound.
Preserve full12km paths/progress, all x/z/IDs/counts/footprints and final lane repair.
A guarded elevation-only baker requires the exact previous layout hash and rejects
all source changes except heights and explicitly derived metadata; it updates
shared profile, source hash and route/control-point y without planar regeneration.

Terrain/render/collision/pads already use the shared profile. Existing actual
triangle support sampling remains below12mm (11.921mm pavement,11.828mm fields);
that narrow margin is not permission to raise tolerances or further increase
relief without tessellation review. Both-branch native recovery and user feel
remain acceptance gates under ELEVATION-01. No physics, dependencies, legacy map
or external media changes. Evidence: testing/next-elevation.

## ADR126 — Reachable queue passing through existing vehicle forces (2026-11-09)

The prior traffic planner assumed at least8m/s during passing, excluding close
stopped queues; matched-speed followers could repeatedly accelerate then brake.
Evaluate current and intended speeds, including a current-speed stage for moving
queues and4/2/1m/s stages when physically reachable. A reduced-speed choice must
lead to a clear passing lane rather than merely follow forever. Remember a passing
side until the original queue clears, while rechecking live rear/oncoming/alongside
traffic, destination occupancy and safe return every fixed tick.

Intersect outbound road bounds120m ahead to reject dropping lanes and inbound or
sidewalk space. Segmented physics sweeps follow actual curved-road candidates
and reject static obstructions. Pure planning remains in rr_core; steering,
braking and acceleration use the existing vehicle adapter. No transform writes,
teleports, traffic removal, collision exemptions or player-controller tuning.
The physical stopped-car fixture must pass without exceptional escape/recovery.
Existing bounded escape remains the fallback when no safe passing route exists.

These bounded forecasts do not reserve lanes or guarantee deadlock freedom against
arbitrary traffic motion. Additional sweeps need combined native performance
review; narrow/sealed queues may still wait. CAST-AI-01/P08-005 remain open for
full-course fairness/feel/performance. No new dependency or external intake.
Evidence: testing/next-queue-passing.

## ADR127 — Officer-only uniform and patrol equipment (2026-11-09)

Use an original UV-baked navy/silver uniform, white helmet and generic gold shield
on the existing human mesh/binds. A reserved usize::MAX presentation token is
assigned only to the resident officer; all16 ordinary roster identities retain
the restored original appearance. The token follows the existing runner/ragdoll/
wreck/remount lifecycle. Generic project-original art uses no real department or
motorcycle brand. Original rider texture and source geometry remain untouched.

Add20 visual equipment children from three shared meshes/seven materials: touring
screen/nose, chamfered panniers, radio box and compact red/blue emissive lamps.
Officer fairings are navy and paint white. Replace only the officer stock screen
with the touring screen; preserve all ordinary bikes. Lamps are static emissive
surfaces, not new lights or siren logic. No collider, mass, joint, motor, pursuit,
recovery or player physics changes. The primitive fallback safely wraps palettes.

A first native review rejected doubled glass and toy-like luggage; the refined
actual-model gallery and both-rig police behavior fixtures passed. Differential
model trajectories remain exact. Root combined verification remains required.
POLICE-ART-01 keeps user art/fairness/full-course/hardware gates separate from
presentation integration. Evidence/provenance: testing/next-police-art.

## ADR128 — Player-hosted sessions and distributed public discovery (2026-11-09)

The user requests native listen-host races and Quick Play joining an available
host at measured RTT <=250ms or creating a host, with no separately operated
lobby directory. The current request authorizes this focused networking work;
it does not close the earlier production latency, prediction or platform gates.
The host owns simulation; guests submit validated input and consume snapshots.

For public discovery, evaluate and integrate pinned MIT mainline8.0.1 against
the existing Mainline DHT bootstrap network, using a game/protocol-specific
namespace. No torrent content is downloaded or shared. Public endpoints alone
are announced; private codes and names are never published. Bounded off-thread
lookups supply untrusted candidate endpoints to the game-specific measured-ping
and availability handshake. Reject nonpublic/reserved destinations from DHT
results, cap candidates and never block the fixed simulation. Short-lived nodes
avoid the library's automatic long-lived routing-server mode.

This removes a user-operated directory, not external bootstrap dependencies or
NAT restrictions. DHT discovery is eventually consistent and may miss hosts;
results must not be called an exhaustive global search. An unreachable host
still needs forwarding or a future relay/NAT-traversal solution. Keep direct
codes and local discovery; expose discovery failure rather than asserting an
empty internet. Private lobbies use endpoint plus a random128-bit capability.
No public deployment or guaranteed different-network connectivity is claimed.

Sources inspected: https://github.com/pubky/mainline (crate8.0.1 source and MIT
license), https://docs.rs/mainline/8.0.1/mainline/ and BEP5 at
https://www.bittorrent.org/beps/bep_0005.html. Alternatives: owned registry
(rejected by user), LAN/known-peer only (insufficient WAN discovery), platform
matchmaking (requires platform integration). Pending: locked dependency checks,
local DHT/handshake tests and actual different-network verification.

## ADR129 — Temporary router mappings for explicit listen hosting (2026-11-09)

An explicit Host or Quick Play host can request a temporary TCP mapping from a
local UPnP IGD, using pinned MIT igd-next0.17.1 on a worker. Use a finite120-second
lease, refresh while hosting and remove the owned mapping on cancellation; never
request a permanent lease or map unrelated services. ROADDASH_UPNP=0 disables
this behavior and an explicit advertised endpoint takes precedence. Native
automation disables it; tests use mocks, not the developer's router.

Expose successful mapped endpoints to code/discovery updates. A mapping response
is not proof of external reachability; CGNAT, firewall restrictions, disabled
UPnP and routers refusing finite leases remain explicit limitations. No relay
service or separate user-operated directory is introduced. Relevant upstream:
https://github.com/dariusc93/rust-igd and
https://docs.rs/igd-next/0.17.1/igd_next/struct.Gateway.html . Inspected published
source, MIT license, lease/error behavior and Rust1.75 minimum before intake.

## ADR130 — Travel-matched presentation gait (2026-11-09)

The user corrected the walking complaint to animation. Keep the original6m/s
controller, turning, physical rigs, recovery and gameplay audio ownership. A
phase-continuous renderer-only gait uses measured travel and smoothed speed,
walking double support and running flight, opposing bent arms, velocity-relative
swing targets and bounded visual pelvis lowering when a support chain exceeds
reach. Physical body transforms are unchanged. This corrects the previous
3.2m cycle versus0.84m support-sweep mismatch rather than tuning player physics.

Targeted animation/model/recovery tests passed before integration. Established
contact slip is at most0.411mm/tick in translated flat and12%up/down fixtures;
this is not full terrain/stair/contact acceptance. Native after-review and
both-rig recovery remain required. Existing footstep sound timing is retained;
exact sound/contact synchronization and civilian NPC gait remain separate.
Evidence: testing/walking-animation. V2-03-003 remains open for broader gates.

ADR129 intake follow-up: igd-next pulls unmodified attohttpc0.30.1 (MPL-2.0).
Read its complete license and Mozilla FAQ Q8/Q11; retain a source-access notice
and license copy. The declaration audit accepts only this exact registry
package/version/license with validated intake files, not MPL globally. Binary
packaging must carry the notice/license; release packaging remains open.

## ADR131 — Bounded world replication neighborhoods (2026-11-09)

Actual rendered-map verification measured roughly285KB JSON per20Hz snapshot,
about45.6Mbps per guest, making an empty-world bandwidth estimate misleading.
Retain every racer/chassis/runner/wreck/rig and all persistent damaged-prop state;
limit NPC/traffic snapshots to the union of750m neighborhoods around each human's
actual runner, torso or rider position. The boundary is beyond the current fog
range. The host still simulates/collides with every actor; this is replication
selection, not removal of authoritative hazards. Guests hide excluded actor
parents and ragdoll parts, restore them on reentry, and restore normal visibility
when returning offline. No interpolation writes into host bodies.

The protocol worker separately adds bounded lossless compression once per
broadcast. Tests cover human-neighborhood boundaries, on-foot centers, stale
parts/reentry/offline visibility and preserved host entities. Native populated
payload measured6.32Mbps per guest (789,422encoded bytes/second at20Hz); this remains above the100kbps target. These changes do not establish production
bandwidth or hardware acceptance. New user city-density work follows this pass.

## ADR132 — Slow rider queues activate predictive passing (2026-11-09)

The prior traffic planner added rider occupancy only after a car hazard, leaving
rider-only crawling queues outside its safe passing path. Promote slow forward
upright riders into hazard detection, then include every nearby upright rider
in existing lateral/rear clearance and committed passing. Preserve normal-speed
combat spacing, existing steering/physics, outbound bounds and the requirement
for a reachable gap. A sealed road still waits. AI-QUEUE-02 records this follow-up.

Four-rider physical cases now clear both a crawling rider and a slow-car queue,
with sustained passing, no damage/recovery and no inbound/shoulder excursions.
Existing predictive passing and blocked/reopened corridor tests remain controls.
Full-route human balance and a dedicated rendered queue fixture remain separate.

## ADR133 — Denser urban activity with paced pedestrian crossings (2026-11-09)

CITY-ACTIVITY-02 raises Metropolis main traffic96→144 and pedestrians192→384,
without rural changes. Rebuild from the approved planning source; only generated
street_life/pedestrians fields change. Ten crossing routes pass full-capsule
support and prop/cafe clearance checks. At most two walkers cross concurrently,
with12seconds between admissions and45seconds rest. Forecast relative traffic
motion for the full crossing; derive main-car signed velocity from its traffic
state because those movers do not have Velocity components.

Fixed host simulation owns admission and movement. Actual support rays place
walkers on road/sidewalk surfaces; waiting actors have zero velocity. Preserve
NPC collision/ragdolls, authored-origin replication identities, every world lot
and all Valley quotas. No new dependencies, external assets or physical tuning.
Native crossing/census and final integrated checks remain required; higher counts
are not a performance certificate.

## ADR134 — Measure crossing forecasts before broader physics changes (2026-11-09)

PERF-08/P16-003 continue on a focused performance branch after local main was
fast-forwarded to d8824bf. Compare frozen release binaries in the same isolated
Metropolis survey; no compiler or second game runs during measurements. Window
statistics are not GPU timestamps or full-race/frame-budget certification.

Retain the cheap pedestrian crossing admission/cooldown/capacity check before
its traffic forecast. Preserve the original admission rules and all world actors.
Add pair-type census to distinguish graph edges from actual active collisions.
Correct the native destruction fixture's obsolete 995-world-prop count: record
the loaded production population and assert it plus the 13 fixture props remains
present. Keep all actual impact, presentation, reset and dynamic-budget checks.

Rejected experiments: reducing Rayon workers did not improve total frame time;
a standalone-collider prop representation failed the existing debris settling
regression; a conservative damage-sweep prefilter passed trajectory/occlusion
checks but showed no consistent improvement in the same-binary control. None ships. Preserve
fixed bodies, existing damage queries, physical rigs, solver and worker defaults.
Pinned Rapier source inspection showed body-type rejection already precedes
collision groups after graph construction, so a group filter would not remove
static graph edges. No dependency migration, external code or asset import.

Retained optimization reduces measured gameplay work. Overall frame timing
varies across runs and remains above target; do not claim 60 FPS or GPU attribution.

## ADR135 — Independent Hills course with an in-town rejoin (2026-11-10)

Implement the user's supplied12.5km concept as a separate Hills map; preserve
Metropolis as default and retain Valley/City. Sequence: city0–1.5km, winding
hills1.5–5.5km, western town5.5–8.5km, dunes8.5–12.5km. The latest user
correction overrides the diagram's town-exit merge: join branches around7.4km
and continue through a shared town street before entering dunes.

Reuse existing route/control/recovery interfaces and original asset catalog.
Author missing western silhouettes as original procedural geometry; the supplied
diagram and local original-game research are references, not imported retail
media. No dependency migration, existing-map regeneration, physical-rig tuning
or facade-source material override is authorized by this map addition.

HILLS-01/02/03 retain user art/feel and broad performance acceptance separately
from technical implementation/tests. Native geometry and branch recovery evidence
are required; entity allocation alone is insufficient.

ADR135 follow-up: user explicitly reinforces strong elevation/turns and asks
for trees, grass, farms and a livelier map. Preserve the strong authored road
relief; correct only discontinuous remote terrain. Add original countryside
dressing and bounded grass/tree/farm populations with lane and footprint
clearance, tapering vegetation before dunes. HILLS-03 acceptance includes this.

## ADR136 — Hills atmosphere, mounted life and scenery polish (2026-11-10)

The user explicitly authorizes extra polish and agent discretion for dust, birds,
clouds, lighting, cacti, richer trees/props and cowboys riding horses. Keep the
authored strong road relief/turns and town split/rejoin, all other maps, controls,
saves and accepted physical rigs. New mounted pairs are separate world actors,
not a change to motorcycle/ragdoll anatomy. Authoritative moving collision stays
in the fixed simulation; bird/dust/gait presentation cannot overwrite bodies.

Use bounded reusable effect pools and original procedural/reused assets. No new
dependency or retail media intake. Hills-specific lighting must keep the visible
sun, illumination, fog and environment maps coherent. Shared-map defaults stay
unchanged. New HILLS-04/05/06 remain open until their technical and human gates
are met; native active views are required, not just paused allocation counts.

ADR136 recovery follow-up: native west/east hill crashes exposed an unreachable
foot-recovery case behind the continuous mountain guardrail. Hills must support
the existing exceptional rescue duration and +5-second penalty without changing
rigs, step height or solid rails. Only an automatic, non-police rider genuinely
stationary for the configured duration qualifies; advancing or manual movement
resets the watch. Native rescue evidence uses an explicit opt-in extended
deadline and reports rescues separately from ordinary remounts.

## ADR137 — Hills composition, canyon crossing and heavy traffic (2026-11-10)

The user rejects the sparse polish and explicitly requests three fresh agent
scopes. Keep the city → rolling hills → western town → dunes sequence; move
farms out of the opening and develop hills using the existing farm concept.
Build closer, denser western frontage around both branches and the shared
in-town rejoin. Preserve the strong route profile and accepted physical rigs.

Place an original suspension bridge over an actual lowered canyon in the hills,
with deliberate visible and physical gaps in side barriers. Falling and combat
knockoff through those gaps are intended hazards, not collision defects. Retain
a supported road deck, protected sections and integrated recovery. No invisible
terrain shelf may prevent the requested fall.

Add Hills-only eighteen-wheel semitrailer and agricultural tractor traffic with
correct full-size collision and turn behavior. Shared avoidance/replication must
respect those dimensions; existing maps and defaults are retained. Use original
procedural work and existing catalog/farm sources, with no dependency migration
or proprietary media. HILLS-07/08/09 remain open until technical and human
acceptance; allocation counts alone do not demonstrate the visual improvement.

ADR137 follow-up: user selects the existing city race's last kilometre rather
than a new generic city. Use Metropolis physical street chainage 10,500–11,500 m
as the Hills opening, translated with its whole buildings, source facade
families and clipped pavement. Match street centre/elevation, then blend into
hills. Tighten hill bends, retain barriers on curves rather than continuous
straights, and increase batched greenery.

Performance work remains PERF-08/P16-003: record a measured baseline, then
compare the same new scene/binary with and without redundant fixed rigid bodies
on immutable Hills solids. Colliders, identities, transforms, semantic roles
and moving/breakable bodies must be preserved. Retain only verified gains;
never report GPU timing or target-frame-rate success from entity counts.

ADR137 collision verification: standalone immutable colliders preserve settling,
scaled-wall CCD, ground rays, dynamic-bike knockoff and assisted blocking in
physical equivalence fixtures. Source city breakables retain their existing
lifecycle. Native west/east recovery now succeeds without exceptional rescue
in both recorded fixtures; the existing delayed rescue policy stays available.
The first bridge survey incorrectly treated the assisted chassis as a dynamic
body. The corrected test must invoke real knockoff/recovery physics, without
changing production handling to satisfy a fixture.

ADR137 second visual follow-up: user explicitly removes all bridge low rails,
not just their earlier gaps. Retain physical square-section thick hangers and
towers, with unguarded intervals and no hidden side walls. Reuse actual
Metropolis farm props and ground texture/material helpers, and populate western
backlots/interior spaces with coherent rural yards rather than only road rows.
New HILLS-10 covers bounded volumetric clouds; their frame cost must be measured
and they are not assumed to improve speed. Three fresh agents handle bridge,
farms and town, followed by a fresh cloud agent when a slot is available.

## ADR132 follow-up — Civilian queue ordering (2026-11-10)

The user reports stopped city cars and attached queue screenshots. Investigation
reproduced a circular leader dependency: three cars at stations0,0.04,0.08 can
select C,A,B as leaders when the five-centimetre fuzzy-tie rule mixes position
with entity priority. The production fixed-update regression also freezes all
three inbound cars at2900/2899.96/2899.92 for20simulated seconds.

Use strict longitudinal order, resolving exactly coincident stations by stable
entity priority. This is a total order, so car-following dependencies cannot
cycle. Preserve lane-merge lookahead, desired cruising speeds, braking, safe
following gaps, downed-person yielding, population, collision and physical rigs.
A legitimate occupied lane can still stop traffic. No timeout-through-obstacles,
teleport, or new lane-changing behavior is part of this correction.

Continue existing AI-QUEUE-02/P08-005; no new IDs or acceptance closures. Add
headless compressed-queue/both-direction/full-fleet regressions and an opt-in
rendered --traffic-stall-survey fixture. This proves the reproduced defect,
not every possible stop shown in a still image. No new assets/dependencies or
external source intake. Work is isolated from concurrent Hills revisions.

ADR137 final integration: original bounded volume-cloud material avoids the
built-in fog renderer's distant-light/shadow limitations without adding shadow
maps or a depth prepass. Eight fixed high banks share one 48³ density texture
and at most twelve march steps. Cloud-only NoFrustumCulling avoids the ordinary
1 km scenery culling boundary; global camera/scenery distance stays unchanged.
The baked fallback remains ROADDASH_HILLS_CLOUDS=0.

Dust alpha material events now filter out materials without normal maps before
tangent preparation scans mesh instances. Dynamic normal-map intake remains
regression-tested; no mesh positions or collisions change. The survey itself
filters its object query to actual actors/cameras/farm markers; this removes
fixture overhead and is not claimed as a production speedup. Paired body-state
measurements hold this common code constant.

ADR137 assisted-edge correction: a direct knockoff fixture proves the canyon
geometry, not accidental deck departure. The optional assisted controller kept
authored road height beyond the 7.25 m deck until the wider road/verge sampler
snapped to the canyon floor. Missing real Ground support at deck height must
hand off to existing physical crash/recovery after solid sweeps, retaining
forward/lateral momentum. Do not alter accepted rigs or globally change handling.
The native fixture must apply ordinary lateral shove state and observe actual
physical Falling/OwnedBody descent instead of calling crash itself. Preliminary
timing runs before this correction remain labeled separately.


ADR137 final composition review: a high view at station 6800 still exposed the
four frontage rows after large ranch parcels were added. Retain their reserved
footprints, then use smaller mixed courtyard footprints for narrow branch
medians, including an explicit 6650–6950 placement check. Extend outer farmyards
with broad irregular textured crop/orchard parcels clipped cell-by-cell around
roads, existing lots and mounted paths. Batch field ground, crops and trunks;
keep physical tree trunks resident, and do not use extra building rows as a
substitute for shared town space. Native defaults to Dynamic; the Assisted
bridge handoff is an optional-adapter correction. Native evidence must identify
its adapter and separately measure authoritative Dynamic chassis descent.


## ADR138 — Selectable courses and Hills structure/landscape follow-up (2026-11-10)

The user requests merge/commit/push plus a native Metropolis/Hills selector,
regular connected bridge hangers, a canyon river, flatter farm sites, broader
city shadows, spatial clouds and a distinct sun. The repository default branch
is main. Integrate the published traffic fix without dropping Hills or changing
accepted physical rigs; preserve profiles, controls, legacy maps and online flow.
Existing HILLS-07/08/10 cover farm/bridge/cloud refinements. New UI-COURSE-01,
HILLS-11, SKY-SUN-01 and SHADOW-RANGE-01 record the additional acceptance gates.

Use original procedural/reused assets and pinned Bevy0.17.3; no dependency or
retail-media intake. Prefer the existing safe course relaunch lifecycle over a
new partial world teardown. Structural suspension hangers must connect to actual
support members without reinstating low railings. Farm placement must evaluate
whole parcels on gentler land rather than manufacture cliff-sided barn mesas.
Water remains bounded presentation over the resident terrain; preserve physical
canyon falls. Visible sun/cloud direction and illumination must agree. Expand
shadow distance with a bounded cascade policy and measure cost, not an assumed
speedup. Validate before pushing; no force push or history rewrite.


## ADR139 — Preserve the curved Hills starting apron (2026-11-10)

The reported last-place spawn used straight backward extrapolation from station
zero, while the reused city street bends behind that point. Preserve source
street samples from -300 m through the grid, sample those negative stations
directly, and let the imported surface own the apron. Extend the source slice
to include it. Preserve grid offsets, race timing, spacing and physical rigs.
HILLS-01 covers this correction; no new requirement or dependency migration.
Validate every spawn and launch corridor against placed street/building geometry.

The follow-up screenshot also exposes stalled Metropolis rivals and a road-edge
grass seam. Normalize future outbound bounds and obstacle lanes into a shared
carriageway frame before planning; preserve real lane-drop limits, collision
queries and accepted physics. AI-QUEUE-02 covers the runtime regression. Close
the 8 cm exposed terrain seam with a soil render-edge underlap, retaining road,
sidewalk/building exclusions and independent collision (CITY-CIVIC-01). No
facade material changes or third-party assets.

## ADR144 — Explicit sand resistance with shared authored terrain semantics (2026-11-10)

Issue29 requires sand to impose the strongest driveable-surface speed penalty.
Move the existing serde Surface contract to rr_core and re-export it from
rr_content, appending Sand without changing old names or catalog/save versions.
Use the shared limit/forward-drive/rolling-resistance rules in both adapters;
preserve reverse control, braking and all non-sand physical coefficients.
Sand power cutoff must not clamp speed on entry, and coasting must dissipate
energy without introducing reverse motion. Share Hills' authored biome blend
between terrain presentation and surface classification; asphalt wins at branch
junctions. Metropolis soil is not sand. Preserve accepted rigs, controls, saves,
collision and geometry. RFU-SURF-01/02/03 remain open for user feel acceptance and broader
coverage; controlled adapter and four native functional cases pass. No dependency migration or performance claim.

## ADR140 — Measure course loading and frame distributions before optimization (2026-11-10)

Issue #25 prioritizes both Hills and Metropolis loading/runtime performance.
Keep the pinned Bevy 0.17.3/Rapier 0.32.0 control while recording opt-in nested
CPU startup spans, bounded raw-frame samples and supported render diagnostics.
Readiness is the existing required-asset gate; it does not prove GPU pipelines
have completed or that every distant detail is resident. Nested spans overlap.
Do not sum them or label CPU render scheduling as GPU timestamps. Preserve all
collision, physics, recovery, saves and content while measuring changes.

Primary crates.io registry queries confirm Bevy 0.20.1 (Rust 1.97.1) and
bevy_rapier3d 0.37.0 with Bevy ^0.20. The requested migration is coordinated
separately after these branches; this work must not silently mix three releases
of breaking APIs. Installed Bevy skills are MIT guidance, the game-development
performance skill package is Apache-2.0 guidance; no source/media is imported.
Installed 0.17.3 renderer diagnostics source is the API authority for this pass.

ADR140 measured loading follow-up: instrumented Hills construction costs
29.2 seconds on Ryzen 5 3600 (countryside14.48s, props9.44s, terrain4.63s).
Memoize exact-bit support vertices only within an immutable borrowed-track
generation scope; cap131072entries, release at scope exit and retain a same-binary
ROADDASH_TERRAIN_CACHE=0 control. A different track bypasses the scope; nested
scopes restore previous state. Skip preliminary terrain heights immediately
overwritten by the final resident terrain sampler, preserving topology/order.
No disk cache/version invalidation, approximate heights, moved props, collision
removal or physical-rig tuning. The GPU sample supports investigating CPU work
first; it is not a full-course GPU conclusion.

ADR140 runtime experiment: delayed flat CPU sampling reports executor work as a
major user-cycle consumer, but awaited work/spin can obscure cause. Evaluate
ROADDASH_FIXED_EXECUTOR=single against the default multi-threaded FixedUpdate
dispatcher. This changes scheduler dispatch only: retain ordered systems, deferred
command barriers,60Hz, substeps, collider/rig parameters and Rapier internal
parallel solving. The default remains unchanged until repeatable timing and
physical/recovery parity pass; revert the experiment on regression.

ADR140 load-admission experiment: Metropolis source construction takes3.24s
while required readiness takes10.96s, with thousands of materials. Instrument
pending/jobs/done queues; evaluate an opt-in eight-job admission ceiling during
Boot/Loading (ROADDASH_LOAD_JOBS=8), retaining two jobs during gameplay and
unchanged worker-pool size, derived bytes/fingerprints and readiness gates.
Do not adopt a higher default without measured queue/load improvement.

ADR140 first isolated matched samples: exact reuse reduces Hills world setup
29.910→18.595s and Metro3.221→2.366s with unchanged native census/support fields.
Single fixed dispatch improves raw p95 Hills25.56→20.45ms and Metro28.66→24.76ms;
eight loading jobs reduce Metro readiness10.132→6.950s. These are one sample per
configuration, not repeated acceptance. Both configurations pass598 workspace
tests (one existing ignored soak). Dedicated recovery/cold/reverse-order repeats
remain before changing dispatch/admission defaults; see issue25 evidence.

ADR140 controlled follow-up: replacement indexing invalidated the first pair's
full-isolation claim; retain it as exploratory only. Eight new cold-asset/reverse
warm runs with verified PID2060 paused/restored confirm readiness gains37–39%
and raw p95 gains13–23%, with unchanged census/support structures. One61.423ms
Metro event is the exact terminal audit frame; retain it and add explicit fixture
completion/span markers, plus same-length cohort comparisons across all controls.
Four both-map/both-rig native recovery cases pass. Select single FixedUpdate
dispatch and eight loading-only material jobs, preserving explicit multi/two-job
controls, internal parallel Rapier,60Hz/substeps and two-job gameplay admission.
Rebuilt18-case explicit paired lab checks pass with exact non-timing equality;
598workspace tests pass on selected defaults. Final source-frozen native default-only recovery and terminal-log surveys
pass on both maps; no dropped evidence and no timing claim under build load.

CI on unchanged main and the diagnostic PR shows15resting riders at900updates,
240finite bodies, anchor error0.00217m, linear0.00639m/s, angular0.13730rad/s, but
only208sleeping bodies. An independent1800-update diagnostic sleeps all240 with
zero velocity. Correct the test's hardware-dependent deadline using one fixture:
retain900-update finite/body/resting/anchor checks and low motion<0.1m/s,<0.5rad/s;
require all240naturally asleep with zero motion by1800updates, record first sleep
update, and run both explicit executors. No forces, damping, motors, collision,
sleep thresholds or accepted physical behavior change. Original failedCI retained.
