# Known issues and remaining acceptance

Updated 2026-11-10. This is not a list of fabricated failures; distinguish known
missing features, architectural debt and unverified platforms.

- PERF-08/P16-003: release crossing-admission optimization lowers gameplay work, but matched survey total frame times remain about25ms and differences overlap run variation. Physics remains about11ms/fixed step. No60FPS/GPU/full-race/hardware acceptance; evidence in testing/performance-pass.

- ONLINE-PLAY-01: listen-host/UI/discovery is implemented and locally verified on
  codex/multiplayer-ui-walking. Public DHT discovery removes a user-operated
  directory but depends on existing bootstrap peers and is not exhaustive.
  Temporary UPnP mappings depend on router support; CGNAT/blocked inbound traffic
  are not solved by finding an address. Separate-network connectivity, network
  impairment/prediction/bandwidth and production acceptance remain unverified.
  Do not confuse injected UI fixtures or loopback tests with Internet play.
- V2-03-003: visual walking cadence/support correction has targeted test evidence;
  native152-frame after-review and both-rig recovery have passed; broader terrain/art acceptance remains open. Civilian NPC gait and exact footstep-audio
  synchronization are outside this rider recovery animation correction.

- CITY-ACTIVITY-02:144main cars/384urban walkers and two completed crossings pass the native survey. The debug functional run measured wall p9532.73ms, above16.7ms; this is not directly comparable to earlier release benchmarks. Rural/shared fleet and structural content are unchanged.
- AI-QUEUE-02: four-rider crawling-bike/slow-car physical queues now drain without damage/recovery or leaving outbound road. Dedicated rendered queue and broader human/full-course deadlock/balance review remain open.

- UI-02: source UI/media catalog and artist skeleton are available in docs/ui, with original references in the separate local kit. A first art-led offline native entry flow is integrated (ADR091); full production UI/art acceptance remains open. Dynamic retail handlers/fonts/disabled states, inherited backgrounds and runtime Windows-dialog labels are not reproduced; unused clouds are context-only/inferred. Source-file coverage is not a retail playthrough or full UI parity. See ui/README.md and testing/ui-kit/README.md.

- RACE-SETUP: user-requested four/three/four lanes, wider single-file staggered grid and disabled player nitrous supersede historical lane/nitro descriptions. Rider palettes are reference-informed original swatches; shared armored costume geometry and PBR appearance still need user art review. Primary sidewalks return to2.4m with1.5m authored clear strip; larger furniture and30 parked cars move to side streets (formerly65 curb-parked cars). Human race balance and full-course/performance/hardware approval remain open. See testing/race-setup/README.md.

- P01-WIN: Windows GPU/audio/input launch and clean install have not been run on
  this host. Part 02 Ubuntu/Windows CI run 41141131978 passed, including the corrected
  UTF-8 metadata audit. Part 03 remote CI is recorded separately in its PR. No Windows hardware certificate is claimed.
- P01-HW: native scripted rendering/audio is verified on Fedora/RTX 3080/Vulkan.
  Windows hardware/input verification and clean installation remain open. A short
  scripted smoke is not a production benchmark or a real gamepad test.
- P01-SPLIT: workspace has real core/content boundaries and subsystem plugins;
  game.rs still combines vehicle/AI/combat/race; listen-host networking is implemented, with production gates open.
- P01-MIGRATE: newer Bevy/Avian/Enhanced Input manifests are compatible with each
  other, not this baseline. Coordinated migration with its regressions is pending.
- P04: native gamepad/analog input, key/button rebinding and persisted comfort are
  implemented. Physical-controller finish/reconnect matrix, subjective 120–144 Hz
  jitter/readability and external tester acceptance remain open. Stick axes are
  fixed; lowest entity ID owns controller input; some special keys/buttons are
  unsupported. LB/Guide/F2 are reserved. Audio settings persistence remains P15.
- P04-MAPPING: native Gilrs discovered one OS Gamepad but warned that UUID
  03000000-03a1-0000-0500-000011010000 has no database mapping and uses defaults.
  Discovery is verified; physical button/axis correctness is not (P04-014).
- P04-BUILD: Linux native controller backend needs libudev development files
  (Ubuntu libudev-dev; Fedora systemd-devel). This host lacked development metadata;
  verification used an explicit local pkg-config shim to its installed system
  runtime library. System packages were not installed or elevated.
- P02: Valley Run is authored/elevated and point-to-point. Optional branches,
  split/merge authoring, a visual editor and representative three-minute race
  pacing/playtest acceptance remain open.
- P03: dynamic chassis/two-ray suspension/combined grip/three presets now run by
  default. Forces remain road-assisted arcade handling, not a fully free-heading
  multibody motorcycle. Ten human rides and beginner/fun acceptance are open;
  simulated stability and scripted scenes do not establish subjective feel.
- P03-TRACE: telemetry is authoritative 60 Hz, bounded to 3,600 samples; no
  fabricated 100 Hz trace or full-session persistence. Slip reports tire-demand
  saturation and lateral slip angle, not a detailed tire deformation model.
- P05/P06/P07: kicks and one first-club candidate are implemented; complete weapon families/ability schemas and balance remain open. Part03 now implements manual foot override and checkpoint-valid foot finishes; production trap/clearance acceptance remains open.
  Ordered bike gates are verified in P02. Recovery auto-seeks the actual bike.
- P06-CONFINED: assisted recovery is verified in representative fallen poses,
  not arbitrary fully enclosed geometry or moving traffic traps. Collision-clear
  crash separation falls back in confined spaces; manual override is implemented; arbitrary confined/stuck recovery
  remains open. Ordinary crashes no longer inject upward momentum.
- P08: one resident player-targeting police motorcycle is integrated; full uniforms/vehicle/dismount/gesture art, moving-foot capture/tackles (safe arrest requires slowing to<=1.2m/s), rival violation selection, road-block/stuck traffic policies, human fairness, repeated full-race soak and broad performance/hardware acceptance remain open. No reinforcement spawns or hidden acceleration/grip boosts. The return state resumes a forward patrol along the open course, rather than teleporting to a home station. See testing/part08-police/README.md.
- P12/P13: production garage/economy and career remain absent. Listen-host multiplayer now exists; separate-network reachability, prediction, bandwidth, loss and scale acceptance remain open.
- P02-END: missing an ordered gate requires R-to-retry; unfinished riders stop at
  the finish. Parking reverse is now available; manual return-to-gate and missed-gate race recovery acceptance remain open.
- P02-ACTORS: cows stand until hit; pedestrians/cross traffic use bounded patrols,
  not grazing/navigation/traffic-light AI. Both actor kinds ragdoll on hard impact
  or three direct strikes, with an eight-active budget and safe 18-second respawn.
  Production get-up animations/animal behavior remain open.
- P06-TUNING: motorcycle collision cutoffs (50 km/h longitudinal, 6 m/s side)
  are original arcade tuning; broader human balance feedback remains pending.
- P06-TISSUE: passive spring/damping approximates tissue; no anatomical muscle
  simulation or active physical animation. ART-03 adds renderer-only skinning to the existing bodies.
- P12-RECORDS: existing best_seconds is an aggregate legacy record; class/track
  records and migrations remain P12-005/P07-007.
- P15: placeholder procedural art/audio; no production animal assets,
  spatial engine bands/music, full accessibility or performance presets.
- P16: declaration/hash audits are not vulnerability analysis, legal counsel,
  complete clean-install tests or shipping signoff. Do not infer release readiness.
- Graphics: optional imported LUTs are disabled for provenance; Reinhard color
  response has been visually reviewed in the local native smoke. Renderer/shader first-run compilation is
  not representative steady-state GPU performance.
- Save: unsupported/corrupt profiles are now kept read-only; the user must move or
  restore the original profile before saving new progress. Rotating backups and
  arbitrary career migrations are later P12 requirements.

See PROGRESS.md for task IDs, acceptance, dependencies and verification.

Historical Part 04 correction: all human states shared procedural geometry/outfit; ART-03 now replaces that presentation with the authored skin.
production skin/rig remains future work. Latest user feedback supersedes the upright-pivot correction: bike and human now
bank together only from steering force, with road bank removed from visual roll. Civilian jumps are
explicit arcade clearance responses, not ordinary crash impulses; broader crowd/
low-speed pile tuning remains unverified beyond six finite clearance scenarios.
Cross-car wipeouts are covered in both adapters, including moving-side impacts.

- P03-REFERENCE: the entire Road Rash handling is not reproduced. Investigated
  exact C++ snapshot 4e052aca; original BIKESPEC.RSC/EXE tables are now available from the verified local disc, but complete calibration is not implemented. Progressive steering uses authored provisional
  timings; engine/brake/gear/grip/nitro/3D integration remain existing RoadDash
  behavior. No original-bike numerical parity or code-reuse permission claimed.
- Camera correction: follow-lag distance is bounded in all views; manual comfort/
  high-refresh and physical-controller acceptance remain open as before.

- BASE-WORLD: the first-map structural generator now places the complete quota
  with lots, pads, streets, field masks and persistent-collision visual LOD.
  Detailed production art/interiors and additional courses remain future scope.
  Recovery uses structural routes plus existing local contact avoidance; unusual
  moving/prop traps have an explicit >120-second or below-world rescue with a
  five-second penalty. See world/COMPLETION_TESTS.md for the measured performance,
  native survey and platform evidence; no universal manual-path or cross-platform
  floating-point lockstep guarantee is claimed.

2026-10-31 follow-up: verified local disc matches all 341 upstream import hashes; original-data availability is no longer the handling blocker. Numerical calibration remains undone. City blocks/frontage quality remains incomplete despite historical allocation checks. Superseded by CITY_COMBAT.md: kicks now animate the selected leg; supported contact yaw and input-only lean are corrected, with automated camera/contact checks and native pose captures. Original handling parity, human subjective handling acceptance and Windows runtime remain open.

CITY-02 supersedes the detached-city spacing limitation: shared-edge infill now
joins neighboring city volumes. Road/junction clearances remain open deliberately.
Navigation uses conservative bounds around infill, so a runner may take a wider
route than the rendered facade requires. See testing/FRONTAGES.md for evidence.

CITY-03 corrects rounded sidewalk shading and introduces curbside tree beds and
taller urban masses. Trees are physical obstacles; junctions remain clear and a
walking strip remains beside the trunks. Facades, foliage and paving are original
procedural art; bespoke production assets and interiors remain future work.
See testing/STREETSCAPE.md for local native evidence and platform limitations.

Rider/combat follow-up selects a stable CITY roster; original other-course and rare
alternate names remain research, not runtime variants. Swatches approximate original
color schemes using procedural models. Original instruction-level handling/kick
distance and relationship-table AI parity remain open. Deliberate downed-rider
hunts are limited to some opponents and short approach distances; human difficulty
tuning remains a playtest gate. See testing/RIDERS.md.

AI-02 supersedes uniform rival aggression and ID-based pace: current-level profiles
and target relationships now drive racing/following/fighting, and the player starts
last. Net relationship easing, display-speed conversion and modern stamina/kick
mapping are adaptations. Full career levels, event-axis/random-stream/rubber-band
parity remain AI-03. See testing/PERSONALITIES.md. No human difficulty acceptance
or Windows certification is claimed.

ART-01 was research only; ART-03 now integrates the independently authored pilot. Production-quality skinned avatar,
animation-to-ragdoll bone mapping and pilot asset acquisition/verification remain
ART-02. Quaternius's current general license and old CC0 pack labels conflict;
public raw-file redistribution must be established per acquired version.

ART-03 introduces an eleven-joint visual binding for the authored rider and detailed
motorcycle while retaining existing physics. Full 20-bone authored animation,
closed-finger grips/fists, skin-weight refinement and distance LOD remain
ART-04/ART-05. Detailed fairings are visual and can overhang the unchanged coarse
bike collider. The source GLB/pose files remain in art/source; no original game
media are redistributed. Existing RIGHTS-002 review and Windows acceptance stay
open. Native receipts and any discovered limitations: testing/MODELS.md.

## V2 immediate repairs and remaining work — 2026-10-31

README is the live checklist: all verified tasks are in Part 00 and all remaining
requirements are in Parts 01–16. V2 Parts 01–08 precede original 09/10; historical
P01–P08/BW requirements are carried forward into those sections and also retained
in a frozen docs/roadmap snapshot. Do not infer that planned clips,
active ragdoll, new facades/props/NPC/landscape production art, weapons or police
have been implemented. The full over-car vault and combat swerve/countersteer
behavior remains V2-01-005/006, pending measured contact geometry/normal/velocity
and animation design. The crash handoff correction only fixes the source of
spawned momentum; a later solid collision can still transfer or reverse momentum.

The authored skin now uses connected source-anatomy pivots and bounded visual
elbow/knee hinges, retaining the physical eleven-body rig. Full finger grips,
foot planting, terrain gait, the twenty-bone animation rig and authored clips are
still V2-02/V2-03 / ART-04 requirements. Coarse vehicle/body proxies are unchanged;
visible car hood shape versus full-height contact boxes needs evaluation in the
vehicle-response task before assuming an arbitrary launch impulse is the fix.
See testing/MOTION_REPAIRS.md for actual verification rather than checklist intent.

## Collision-response correction boundary — 2026-10-31

COLLISION-01/02/03 revise mounted yaw and crash response. Scripted civilian/body
clearance jumps and spawn spin are removed; P06-022 is historical, superseded by
the latest no-launch request. A 120/40 -> 80 forward result is the chosen arcade
entry rule for both directions, not a physical closing-speed identity. Crash
energy/COM guards drain solver-added rebound while retaining gravity/tumbling.
They do not make the whole world momentum-conserving: traffic is still kinematic.

Coarse car collision shapes may stop or pin a wreck. Full nose-down over-car vault
and combat swerve choreography remain V2-01-005/006. Ragdolls still rotate and limbs
may swing backward relative to their torso; only mounted steering yaw is input-
owned. This is intentional. Full-scene render comparison permits 2 cm remount
projection/contact-order tolerance after the new gravity-led wreck release;
isolated eleven-body parity remains exact. New Windows runtime/human acceptance
and a broader car-shape/vehicle-height matrix remain open. PR #8's Windows CI
passed; its Linux soak failure was traced to incorrect fixture expectations, now
corrected and rerun locally. New remote CI evidence is recorded separately.

## Animation A review candidates — 2026-10-31

The 247 static poses are authoring candidates, not runtime animation or an approved
production library. The physical rig and its playback remain unchanged. Fingers
are rigid: closed punch fists, wrapped grips and weapon sockets still need source
mesh/rig work. Matrix attachment/contact/flex tests do not establish that clothing
and bodies never intersect or that a static stance makes a natural moving gait.
The current cow is a separate review proxy only.

Old broad mounted labels can be ambiguous; original CANS/RRAN/EXE sequence timing,
view aliases and some native reaction/mount associations remain unresolved. Raw
reference media stays local. Controlled Synty-game footage/gait measurements and
the screenshot game's identity remain open. Complete support/get-up, weapon
handling and police restraint poses need art review/additions. V2-02 approval and
all Part 03 interpolation/terrain/active-control gates remain unchecked.

## Context correction — 2026-11-01

V1 semantic labels are superseded. Its “documented-helper-anchor” strings describe
an inspection-based approximation, not native action proof. In particular44/45
are a post-hit gesture; punch forehand is176/177/178 and kick225/226 under decoded
rear selection. Use docs/animation/context/catalog.json; do not approve V1 motion.

The new catalog records source order/mirroring/fallbacks and logical timings, but
code7 and some alias callers remain unresolved. Comparison against the compiled
research reconstruction is not native original execution. Police poses are metadata
only. Code43 has empty rider art. Slot64 is populated but not referenced by recovered
CANS groups. Draft human/bike layouts need art approval, supported get-up/remount,
hand/weapon/cloth clearance and multi-angle silhouette refinement. Fist shape keys
are authoring proxies. No runtime interpolation or ragdoll change is included.

- ANIM-B (2026-11-01): first 13 visual clips and torso corrections are integrated;
  full Part 02/03 approval remains open. Twenty-bone FK feeds eleven existing skin
  joints: runtime wrist/finger/foot deformation can differ from the Blender GIFs.
  Grip/peg numeric targets do not certify glove/cloth/bike clearance. Current run
  loop is not terrain-foot-planted or velocity-stride-matched; get-up does not
  start from an exact captured physical landing pose. Native smoke covers recovery,
  not all normal/tuck/attack combinations. Source logical timing conversion and
  stock legacy selector inputs are RoadDash mappings; Windows and original-native
  playback not verified. ANIM-09 tracks native/user acceptance; ANIM-02/04/06 and
  full V2-03 gates remain unchecked. Accepted passive ragdoll physics unchanged.

- CRASHREF-004 (2026-11-01): --original-crashes is an opt-in tall-contact/entry
  comparison, not full original-game trajectory parity. Modern 3D geometry,
  Rapier post-entry friction/bounce, approximate steering-rate/SI mapping, current
  traffic controller, and comparison condition defaults differ from native.
  Static/low-object/civilian/downed-body and combat caller routing retain current
  behavior. Complete native geometry/order, traffic reciprocity, per-racer stats,
  attack callers and trajectory oracle coverage remain open. Signed native
  oncoming motion was verified; entire original Windows game was not run.

- V2-04-001–007 facade prototype: catalog inherits 39 named city records; not
  independently verified all-course extraction. Three families/12 OBJ variants
  are offline geometric drafts with rectangular openings/flat colors. Arches,
  distinctive roofs, production textures, optimized topology, GLB/LOD/collision,
  native placement/navigation/performance and art review remain open. Details:
  research/FACADE_PROTOTYPE.md. No map or physical gameplay changes.

- V2-04-003/004/006 city study (2026-11-01): optional city_preview binary
  builds 414 geometric draft buildings along a separate kilometre avenue from six
  BUSS-family concept references. This is not all-146 asset conversion or Valley
  integration. Shared rectangular windows, untextured solid-color materials,
  generic signs/trees, projecting party-wall trim and no interiors/traffic/LOD/
  colliders/navigation remain. Native frame statistics include vsync/capture IO
  and are not a production performance pass. See testing/city-preview/README.md.

- Facade correction supersedes the preceding color-only city study: actual new
  PNGs now texture 416 buildings (100 facade IDs / 17 unchanged sheets), with
  relief on cornices and first-six details. This remains a local textured-relief
  preview, not final all-family model conversion. Baked lighting/unlit fronts,
  existing AI sign lettering, gable background corners, shared height/width rules,
  provisional side/rear reuse and generic roof/vegetation remain. Production
  clearance, detailed meshes, LOD/colliders/navigation and Valley integration are
  still open. Previous generic captures are retained under rejected-geometric.

- Actual mesh milestone supersedes both rejected city representations. Six first-
  sheet buildings have real geometry / embedded GLBs and editable Blender source;
  other94 facade and46 structure/surface assets are still unmodeled (MESH-01/02).
  Materials/paint wear are cleaner/simpler than approved renders; small trim
  intersections and repeated storefront/side variations need art review. Prototype
  LOD swapping causes frame spikes in the kilometre viewer; no production 60fps
  approval. No colliders/navigation/canonical Valley integration yet. Scene glTF
  reflection types are explicitly registered without changing engine dependencies.


### Full building catalog — draft art and isolated preview

All 146 reference IDs have geometric authoring coverage; this supersedes earlier
first-six-only scope notes. Additional models are drafts: exact concept proportions,
wear, joints/trim cleanup, side/rear diversity and user art acceptance remain open.
Surface modules are not standalone buildings. Gallery scales specimens to fit;
source and GLB manifests retain actual dimensions. The 1 km map uses uniform
frontage scaling, shared model instances and lazy LODs; runtime performance is a
preview observation, not production racing acceptance. No building physics,
traffic/navigation, world_gen/canonical Valley integration or release-rights
approval is supplied by this art pass.


### District sets — eligibility versus actual original placement

All146 catalog IDs are grouped, but course indexes measure named assets inside
loaded families, not the exact original scenery selections/coordinates, route
order or spawn frequencies. District labels are RoadDash interpretations. Applying
pools/transitions to preview/canonical world generation remains V2-04-003 work;
this catalog-only change does not alter either map. Windows symlink restoration
is unverified; portable JSON/CSV canonical mesh paths are supplied.


### City Run5300 — playable development-world limitations

The default race is now the generated5.3km city, not the separate earlier Valley.
Select ROADDASH_TRACK=valley to play the unchanged farmland/city/hills route.
New city is a straight open avenue with reserved grid side streets; it does not
reconstruct the original game's route coordinates or level scenery selectors.
Building fronts/returns are actual3D geometry, roofs/rears are shared complete
exteriors. Interiors are solid collision volumes, not enterable rooms. Ornamental
stoops/awnings/trim do not have exact mesh collision; core building, street,
sidewalk, trunk and lamp collisions are independent of rendered LOD. Fine collision
fidelity remains V2-04-005 work. Large scene uses shared/lazy LOD assets but broad
hardware, cold-load, long-session and simultaneous-crash profiling remains open.
Default runtime resolves local development art through architecture://; production
packaging must include the front modules/materials and preserve draft-art review
gates. Existing rider/bike/ragdoll dynamics and Valley quotas/IDs remain intact.

- COLLISION-04: original-style contact routing uses modern0.8m low geometry and
  collider-projected containment, not exact native dimensions/damage units.
  Downed bodies stay physically solid and can slow or block vehicles; removing
  the forced wipeout is not noclip. No change to accepted passive ragdoll tuning.
  Full retail parity, airborne-debris behavior and subjective pile-up feel remain
  within the existing crash-reference/playtest gates.

- PROP-01 / V2-05: full scoped indexed-image coverage is extracted, not a finished
  model set. Eleven prop banks need identification;369 rebuild entries include
  alternate view/color/size banks requiring archetype consolidation.3521 non-CEL
  frame references remain explicit. Family residency does not prove placement.
  The first six concept images have no editable mesh/rig/GLB; vehicle badge and
  character style consistency need review. No new props integrated into runtime.


- Full prop bitmap concepts preserve source-bank coverage, not final 3D geometry
  or exact hidden topology. Art direction, lettering, small generated symbols,
  animal/character anatomy and archetype consolidation need production review.
  Source category names remain provisional even where generation prompts correct
  a visibly misclassified object. Reference-derived designs remain local review
  assets pending release review. No runtime props or accepted ragdoll changes.


### Draft 3D prop catalog (PROP-02; V2-05-002/V2-06-001 remain open)

- Geometry exists for 369 IDs, but shared procedural recipes do not faithfully
  match all 66 concept sheets. Several source views share the same silhouette.
  Human forms are simple disconnected static authoring parts, not a validated
  skin or replacement ragdoll; cows/sheep and their coat patterns are approximate.
- Vehicles need distinct bodies, wheel arches, interior and mechanical detail;
  foliage uses simplified volumes; signs/billboards need proper original icons
  and complete graphics. Current warning emblems are generic on several IDs.
- Some narrow geometry intersects, and most category source collections are hidden
  on initial opening to prevent all IDs overlapping. Unhide one collection in Blender.
- Bounding proxies are metadata only, not production colliders. No placement,
  performance budget, rigging/animation, release approval or native race replacement.
- Early native prop smoke found missing scene-type registration; fixed by reusing
  the existing GLB registration plugin. Early export colors were invalid; replaced
  with independently authored embedded material tiles and re-exported before delivery.


### 2026-11-01 — Draft prop gameplay integration (supersedes isolation notes above)

Default gameplay now presents selected draft props: traffic/cross traffic, city
and Valley trees, city lamps/furniture/parked cars, pedestrian and cow skins.
The 369-ID intake is not 369 distinct placed/approved objects. Unused categories
remain catalogued; native receipts identify actual selected variants. New actor
skins retain draft segmented anatomy and rigid weights; joints can show gaps and
clothing deformation is not production quality. Existing rider/bike art stays intact.
These are the rough volumetric drafts, not faithful finished versions of the AI
concept sheets. Pictograms, foliage, vehicles, topology and materials need refinement.
Original game media is not embedded. Existing design-release review remains pending;
local development use is not a legal-clearance claim or distributable release gate.
No new prop-specific audio or animal species behavior is claimed. Broad performance,
Windows/macOS and human gameplay acceptance remain open. Static furniture adds solid
objects only in reserved sockets; collision shapes for replaced objects are preserved.

- RAG-05/06 (2026-11-01): biomechanical physics is an opt-in comparison; the
  eleven-body baseline remains default. Sixteen physical segments still feed an
  eleven-joint skin: wrist/foot/intermediate-spine deformation needs authoring.
  Conservative shoulders avoid a quaternion-chart singularity but do not model
  scapular motion or every extreme human reach. Distal angular-velocity peaks
  during high-speed impacts require subjective native review; this is not a
  clinical injury/ligament simulation. Existing automatic recovery may transition
  before the standalone body reaches Resting. Full race p95/multi-crash profiling,
  Windows hardware and user feel approval remain open. Do not merge this
  comparison PR or replace defaults without the user's comparison. See
  testing/RAGDOLL_BIOMECHANICS.md and ADR-036.


2026-11-02 launch usability follow-up: B only operates in the standalone lab.
The comparison branch now selects biomechanical physics automatically for both
game and lab, with --ragdoll-baseline for the old rig. This supersedes its earlier
opt-in/default instructions. Main remains unchanged; RAG-05/06 remain open.


2026-11-06 integration supersedes comparison-only/default restrictions above:
user approved merging the new rig, which now starts automatically. --ragdoll-baseline
remains available. Current prop humans still use11 visual frames over16 physical
segments; independent distal deformation and broad performance/platform checks
remain RAG-05/06. Cow anatomy is unchanged. Original-crash direction policy is
preserved across physical upgrades; no new physics backend or asset import.


2026-11-06 follow-up: new-mode hip medial/twist stops and stronger passive leg
motors address knee crossing. Four substeps/eight extra iterations pass the current
stress/settling scenes; fifteen-racer production performance remains RAG-06.
Rear/downed contacts now intentionally receive a bounded arcade hop (COLLISION-05),
superseding the older no-scripted-hop statement for those classes. Baseline-labelled
comparison screenshots still show the preserved original physical control. Broad
art and independent visual distal joints remain unfinished under RAG-05.


## Part 03 implementation (2026-11-06, ADR050)

- Corrected mirrored attacks and inward tuck/lean elbow poles. Pilot skin now has
  sixteen independently driven joints; prop pedestrian skins still use eleven.
  Fingers, mesh/cloth/bike intersection, all viewpoints, Windows and user art
  acceptance remain ANIM-09/RAG-05 gates.
- Distance-driven gait, support-normal soles, stance locks and physical-pose
  get-up are working prototypes. Pelvis adjustment, stair probes, directional
  turn-in-place, measured slip/penetration budgets and confined clearance remain
  V2-03-003/006. One generic get-up clip cannot represent every support posture.
- Manual input/0.35s delayed assist, slow-bike interaction and once-only foot
  finishes are tested. Manual cliff rescue handles below-terrain falls with a
  penalty; arbitrary fence/car traps, user handoff feel and100 mixed-world
  crashes remain open. BustedOrRecover waits for police implementation.
- Supported player bracing caps activation at0.12 using existing motor torque
  caps; zero input preserves normal passive behavior. No free-space COM control,
  conscious/unconscious gameplay or measured active-work budget is claimed.
- ANIM-12: one default-stack debug city launch overflowed IO Task Pool stack
  during asset loading. RUST_MIN_STACK=16777216 passed city default/baseline
  smoke. Cause/default-stack fix and release-build reproduction remain open;
  do not infer it is an animation regression or claim it is resolved.
- Actor smoke expects a cow and pedestrian: use ROADDASH_TRACK=valley. City Run
  intentionally lacks a cow, so running that fixture on the default city fails
  its two-actor precondition. No city cow was added to satisfy a test.
- Local paired15-crash lab settles; it is not native race frame/physics p95,
  allocation,15-racer-plus-NPC or Windows production certification (RAG-06).

## User motion/camera follow-up (2026-11-06, ADR051)

- Neutral wrist kink was a parent/rest-frame mismatch in visual retargeting, not
  an anatomical joint-limit failure. Parent-relative distal calibration now keeps
  neutral wrists straight; no extra twist is applied to standing. Gait and tuck
  elbows were revised and reviewed natively. Final motion feel/mesh clearance and
  platform acceptance remain ANIM-09; prop NPC skins are still separate.
- Upright rear collisions now share momentum without hops and use60km/h closing
  speed for the front-rider knockoff. This supersedes earlier rear-hop behavior;
  downed-body/wreck hops remain. Side/head-on tuning is unchanged.
- Supported slide rolls are one-shot, energy-paid cues, not guaranteed complete
  rolls/somersaults. Terrain, obstacles and other bodies may interrupt them. The
  frontal leg release uses capped joint motors; production subjective crash/energy
  and worst-case race profiling still need review (RAG-05/06).
- Valley Run is the launch default again; ROADDASH_TRACK=city selects the retained
  city course. The Valley debug smoke passes without the thread-stack workaround.
  This does not diagnose or close ANIM-12 for the optional large city load.
- Camera avoids static scenery with lower-boom candidates; transient actors no
  longer collapse its distance. Extremely enclosed geometry and broad user comfort
  review remain outside the small tested ceiling/wall cases.

## Part04 Valley architecture integration (2026-11-06, ADR052)

- All657 existing lots are bound to authored exterior designs. Detailed geometry
  is deliberately bounded and camera-facing; far and streaming fallback remain
  simpler shells/windows. Review pop-in, seams, over-scaled bay/ground dimensions
  and texel density before production art acceptance (MESH/V2-04).
- KIT-03: party-wall infill remains plain; parking/service roles can use industrial
  facade drafts without their intended bespoke full exterior. Rural structures
  are frontage adaptations around unchanged solid collision masses. Navigable
  interior barns/buildings are not implemented by this kit (ENV-005 remains open).
- The warmed1080p15-racer survey still misses16.7ms wall-frame p95 after bounded
  detail/shadow tuning. Native logs/JSON record measurements; no60FPS production
  certification is claimed. Frame time, cache memory and streaming stress across
  hardware remain V2-04-006. The original1,200-panel19.36ms result is retained too.
- Existing art covers facade/structure drafts, not final material/layout/style
  acceptance for every design. Coast/cliff/woodland bible remains P11-002 work.
- Optional City Run's ANIM-12 loading-stack issue remains separate; Valley's new
  kit loads/runs on the default worker stack here. No Windows certification.

## User-requested world/performance follow-up — ADR053

- ANIM-12 resolved locally: explicit16MiB asset IO stacks permit both native Valley
  and City Run smoke without RUST_MIN_STACK. The old failures/workarounds above are
  historical; default compute/physics pools and worker-count policy remain intact.
- Plain high-rise/infill surfaces and road material improved; resident coarse
  textures and ready-instance swaps remove empty-scene gaps. LOD silhouette changes,
  layout/material fidelity, bespoke service/parking geometry and final user/art
  approval remain open. This does not close KIT-03/MESH/Part04 production gates.
- Offscreen mounted NPC crash shortcuts intentionally discard physical crash
  simulation: no ragdoll/wreck, five active seconds, then road-centre return at the
  original impact station with zero speed. Turning back does not materialize a
  physical crash mid-delay. Existing physical ragdolls remain simulated; nearby,
  camera-visible and player crashes stay on the accepted physical path.
- Coarse residency trades a finite catalog memory cost for less texture reload;
  high-LOD cache and hidden preparation are separately bounded. Long streaming/
  memory stress, frame-time acceptance and Windows certification remain open.
- The user explicitly authorizes Part05 after merging this work even though broad
  Part04 art/performance acceptance remains unfinished. Keep those tasks open.

- Latest ADR053 survey meets the local1080p wall-frame target at15.83ms p95 after
  ornament-shadow trimming. The earlier failed snapshots remain historical evidence;
  long-race/streaming-memory and multi-hardware performance gates are still open.

## Part05 world/contact follow-up — 2026-11-07

- Pedestrians/recovering riders no longer stop the mounted attacker; downed-bike
  lift uses saved entry motion and a bounded pair-linear-energy budget. Cows,
  cars and upright rear-bike contacts retain their separate policies. This is
  an arcade contact adapter, not deformable-body impact reconstruction.
- New shared microdetail covers34 original palette source profiles. It does not
  repaint every vehicle/prop/rider/building albedo. MAT-02 explicitly tracks that
  remaining per-family artwork and native approval; broad V2 art gates stay open.
- Cloud sky is 256px RGBA8 cube data, not HDR photography/volumetric weather.
  Approximate environment mips and slow rotation do not model cloud shadows or
  physically convolved specular irradiance. Final art/landscape approval remains.
- Ready-scene transitions use shader dither over 0.30s; silhouette shifts and
  temporary dither grain can still be visible. Long streaming/memory/hardware
  certification remains open. First new native sample missed16.7ms at19.02ms p95;
  retain the receipt rather than claiming a performance gate passed.
-131 new Valley props use existing draft geometry and solid proxies. Full model
  fidelity, category sound/breakables, encounters and all-category placement are
  still V2-05-002..005 work. No new trees or building lots were added.
- Hidden prepared-scene fade command trimming improved the second local sample
  to17.93ms p95, still above16.7ms. An unrelated repository's CPU-heavy test was
  running during that sample; isolated idle-machine performance remains unverified.

## Roadside variety follow-up — 2026-11-07

- Roadside dressing now offers35 roles/20 small groups from the existing draft
  catalog, within224 static owners and weighted biome shares. Larger props/groups
  can lose members to clearance or support checks. Parked vehicles do not drive.
- No extra trees were added. Final prop fidelity, broader encounter/audio behavior
  and V2-05 production acceptance remain open; more variety does not finish those.
- The new pattern is for the default Valley structural layout; explicit City Run
  retains its separate existing city socket pass. Structural building quotas and
  accepted rider/ragdoll behavior are unchanged.
- Roadside-variety native survey retained all224 owners/680 rendered mesh
  descendants; the final local sample measured17.11ms p95, still above16.7ms.
  Placement acceptance is separate from the still-open performance gate.

## Crash research findings — 2026-11-07; fixes NOT implemented

- Decisive punches call immediate crash; detached_origin searches metre-scale
  lateral offsets (first1.25m), causing visible separation. No mounted swerve/
  countersteer release stage exists. Changing only animation would not fix this.
- Current one-shot grounded roll follows slide direction, not a supine goal;
  support accepts any non-dynamic contact, including walls/kinematic vehicles.
  Directional hand protection, high-speed yield and progress-aware roll attempts
  remain proposed V2-01-006/V2-03-005 work pending user direction.
- Original reference confirms animation speed/height gating, not autonomous3D
  biomechanical recovery. Rust candidate uses incompatible Rapier0.36/glam0.33;
  Godot/Unity examples cannot be imported as native Bevy plugins. Their direct
  velocity/pin techniques would need momentum/work safeguards. No dependency added.

## Method B comparison branch — 2026-11-07

- Experimental reciprocal angular-impulse muscles preserve native anatomy/passive
  drives and avoid linear velocity/position pursuit.180J total/180W positive-work
  cap may lose against contacts or the existing energy guard; roll success is not
  guaranteed. Native/user feel/performance/hardware gates remain open.
- RAG-B-01..03 remain unchecked until their full acceptance, including native and
  user comparison. This is not a default/main merge. Existing world/art gates stay.

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

Active corrections expire after the configured brace plus both attempt/retry
windows and1s (4.95s with defaults), including cheap slide damping. This returns
control to native passive tissue/sleep rather than repeatedly waking limbs.
Native recovery/bracing consumes the previous fixed tick's verified ground
support proxy; support clears on the next sample after contact loss.

The final worst-case fully-prone8m/s fixture starts attempts but reaches only
supine score-0.9932 (about7degrees torso roll), not a completed roll-to-back.
This remains an explicitly partial comparison prototype; no full active
recovery/animation gate has been checked.

## Velocity selection / facade handoff — 2026-11-07

Velocity is user-selected; the native-motor comparison candidate is excluded.
The existing worst prone fixture still does not complete a rollover. RAG-B gates
remain open beyond selecting the controller. Anatomical constraints are retained.
Facade replacements now crossfade directly; old fallback detours and same-sign
dither are corrected, but genuine geometry/silhouette differences at initial
activation, optional City Run's independent LOD renderer and long streaming/
production performance require further review. No universal zero-pop-in claim.

Final survey sample: p95=16.94174ms (16.7ms target missed), average14.05263ms.
Actual displayed731panels/731523triangles and48cachedLODs fit unchanged bounds.
LOD-03 tracks distant identity and optional City Run; full zero-pop-in and broad
production/hardware/performance acceptance are not claimed.

## Facade identity and City Run lifecycle — 2026-11-07

LOD-04 supplies model-derived far identity and direct ready scene transitions on
both maps. LOD-03 now awaits user visual/extended-streaming acceptance. Projected
far surfaces can flatten depth/parallax; close real geometry can still change a
silhouette, and dither grain is visible during transitions.128MiB texture and
250kstatic-proxy triangle caps are distinct from near detail budgets. Native
City cache/transition maxima are48/8. The initial isolated full-world sample
p95=18.45385ms misses16.7ms; a later49.09ms sample overlapped test compilation
and is not a controlled performance result. Keep broad performance/art gates open.

Final isolated facade LOD survey: p95=16.80066 ms against16.7 ms, so performance gate remains open (exit1); geometry/source/coverage assertions and all15sections pass. Final native Valley sweep54direct crossfades, City389; both recovery smokes and preview pass. Longer streaming/user approval remain open.

## 2026-11-07 world repair follow-up

- LOD-05: near/far opaque cores are now inset using actual scaled cross-LOD depth;
  all 657lot depth checks and the large-facade native fixture cover this defect.
  LOD-03 remains open: these captures do not prove every user-reported transition
  is gone. The isolated visibility-range probe did NOT reproduce the initial
  zero-range hypothesis on this GPU.
- WORLDART-01/MAT-02: complete city-wide texture rebuilding, coherent property-led
  city rearrangement, new terrain/sky art and final 3D Come Bye-inspired approval
  are not delivered by this repair batch. Plans are reviewable; proposed fences,
  owners and 3/4-lane hierarchy are not placed gameplay content.
- NPC-SEAMS-01: civilian skins still use rigid segment bindings; floating/separated
  pieces under demanding poses need a source-bind/weight/seam correction and
  close-up native approval. Actor lifecycle smoke is not visual seam acceptance.
- COMBAT-FEEL-01: fixed-clock strikes now resolve once at the authored extended
  runtime contact keys; kick side is selected at windup and cannot follow a target
  across the rider. Range/LOS are evaluated at contact, so moving out of reach can
  cause an intentional miss. Subjective impact/audio quality and full combat
  playtest acceptance remain open; see testing/part03-runtime/README.md.
- V2-03-003: renderer foot plants now use physics-local support frames, finite
  swing release and fresh stance capture. Synthetic support/phase tests are not
  full production foot-slide/pop acceptance across stairs/curbs/turns. Visual
  pelvis reach, terrain coverage and final motion approval remain open.
- Part03 provenance: changing src/animation.rs initially invalidated the old
  hand-review source hashes. A fresh current-release33-view native hand run and
  package/check restored current provenance. Those held-pose images validate
  glove/socket integration, not live strike timing; separate keyboard/contact
  screenshots and headless pose/hit tests cover that path. Final hand art and
  source/runtime normal/tessellation parity remain open.
- TRAFFIC-04: basic downed-rider lane yielding/horns do not implement general
  intersection priorities, signal phases or complete traffic-to-traffic queues.
- Performance: the isolated world repair run measured 22.38 ms p95 at 1080p versus
  the 16.7 ms target. It failed that gate. More traffic is now simulated; no claim
  is made that this explains the whole difference or that performance improved.
- Animation knee correction concerns visual chains. It does not certify every
  extreme physical hip/knee collision posture or change accepted ragdoll anatomy.

- LAYOUT-01 (2026-11-07): the dense replacement is a 2D proposal only. The previous
  dispersed plan was rejected; runtime placement has not been altered. Exact
  corner geometry, secondary street widths on inside curves, overlap/access,
  terrain support and canonical lot migration need validation before later 3D work.
  User layout acceptance remains open; no new performance or visual-art approval.

- LAYOUT-02 / ROUTE-SPLIT-01 (2026-11-07): the wider city-center/two-fork design is
  a 2D local-coordinate proposal only. Four paths/nominal lengths and plot clearance
  are verified, but the current game does not yet support these route choices.
  Diagonal corner frontage, merge radii/traffic/visibility, closure/signage/access,
  terrain/collision, checkpoint progress, AI and branch-aware recovery need future
  implementation. No 3D changes or city-layout approval implied.

- LAYOUT-03 (2026-11-07): latest proposal is farms → long city → urban finish,
  with one split/rejoin and proposed urban elevation. It supersedes the earlier
  two-fork layout. The first new drawing incorrectly bent the building grid;
  the corrected export uses only straight rectangular plots and rejects bent ones.
  Current playable content still includes its existing hills until a future
  approved map bake; the new plan is not live gameplay. Exact length, full corner/
  access/terrain checks, migration, branch balance, traffic and performance remain
  open under WORLD-EXTEND-01/ROUTE-SPLIT-01. No 3D changes in this layout session.

- LAYOUT-03 correction (2026-11-07, ADR066): the prior four-right-angle building
  constraint and attempted forced-right-angle street junctions were incorrect.
  Current 2D proposal derives actual land blocks from streets and sidewalks,
  including fitted triangular corner sites and polygonal frontage parcels.
  Geometry coverage/overlap checks are not full architectural/transport planning:
  intersection controls/radii, courtyard servicing/access, final elevation pads,
  exact extension length, user approval and later playable integration remain open.
  No 3D/runtime changes in this revision.

- LAYOUT-04 (2026-11-07): offline height/detail/access plan now exists, with
  downtown height hierarchy, scattered frontage accents/interior towers and
  route-distance authored tiers. These are not placed game meshes or runtime LOD.
  User review, final architecture/property assembly, service turning/vertical
  clearance, pads/support, native silhouette transitions and performance remain
  open. Six-metre access and endpoint grades are planning checks, not building
  code/fire-access certification. No game/physics change in this session.

- LAYOUT-05 (2026-11-07): race-centered city and primary-avenue junction plan is
  offline only. Whole-cell edges step around the development envelope; some
  blocks merge where competing secondary segments are omitted. No bent buildings
  or imposed90degree doglegs. Proposed priority/traffic control is not implemented
  in the game. Junction turn radii/markings/signals, exact architecture/access/pads,
  visual approval and native LOD/performance remain open. Regenerated proposal
  IDs/counts do not migrate canonical runtime lots or change the race/farm trace.

## 2026-11-07 — Metropolis runtime gates

Approved12km farm-to-city plan is now integrated as the default separate map;
legacy selectors/quotas remain. See testing/metropolis/README.md. Native wall-frame
p9520.38ms misses16.7ms in the bounded dev-build sample. Full-course traffic/merge,
all-lot slope/support, extended streaming/multi-crash performance, final bespoke
architecture/street art and platform acceptance remain open. Facade repetition,
paved rear courts and distant fog remain visible. Survey proves resident full
envelopes, not final user art approval. No interiors or additional finish runout.

## SIDEWALK-01 — 2026-11-07

Metropolis walks are now typically3.4m with730 checked curb-furniture owners;
primary city roads10m / typical secondaries6m, farms/building records unchanged.
Native survey retains all envelopes; measured dev p9518.36ms still misses16.7ms.
Walking-strip/junction authoring checks are per placement, not a complete pedestrian
AI/accessibility certificate. Full traffic/art/platform/performance acceptance
remains open. Original generation report is retained as historical evidence;
current pavement/owner counts live in testing/metropolis-sidewalks.

## AUTHORED-01 — 2026-11-07

Actual earlier GLB facade meshes now supply nearby Metropolis detail, with stable
opaque backups and bounded resident source handles. Twelve currently mapped
families are integrated; PALETTE-01 keeps broader district/catalog selection open.
Detailed relief is cosmetic/fitted rather than a new physical collision surface.
Final draft fidelity, source signage/palette/infill, graded frontage art and corner
review remain open. Native bounded dev p9519.81ms misses16.7ms; no production or
full-course certification. Existing sidewalks/furniture/accepted physics remain.

## STREET-LIFE-01 — 2026-11-07

Metropolis now has96 main-street cars,65 parked vehicles,995 curb objects and
2/3/4 marked travel lanes with per-building2–32-floor contrast. Existing10m asphalt
and3.4m typical sidewalks stay; four-lane2.4m widths are an arcade allocation,
not a new real-world street widening. Sampled vehicle/pavement checks pass.
Main-car collision uses all-racer170/220m gameplay interest, independent of camera;
unobserved remote car/pedestrian contacts are omitted. Spectator/network observer
interest must be extended later. Full merge/intersection/long-race/art/platform
acceptance remains open. Release bounded p9524.93ms misses16.7ms; increased activity
still needs performance profiling/instancing/streaming work. Existing gates remain
unchecked. Earlier source height maps are historical; current roof heights live
in the runtime manifest/native census. Footprints/race paths remain unchanged.

- DESTRUCTION-02 (2026-11-07): all995 Metropolis sidewalk props now persistently
  crush/topple and reset. These are coarse original-model damage states, not
  authored snapped/fractured variants, glass/splinters or tree-crown physics.
  No cars/buildings/roads are damage targets per the user's explicit scope.
  Legacy-map destructibility is not included. The48-body cap and supported settling
  are tested; a worst-case fully damaged-course performance benchmark and Windows
  hardware test are still outstanding. See testing/prop-destruction/README.md.

- AI-11 (2026-11-07): seven racers/seven killers and exact-ID retaliation are
  implemented. Human full-race fairness/aggression review remains open; roles
  alternate by stable ID and grudges persist until restart. Direct combat hits
  provoke racers; incidental vehicle contact does not. No exact original-game
  AI parity or subjective difficulty approval is claimed. See testing/racer-killer-ai.

- PERF-08 (2026-11-07): new performance logs measure wall/main/fixed/render CPU-wait
  spans and memory/paging; they do not measure actual GPU timestamps. Low aggregate
  CPU/GPU utilization and a full swap device do not identify the bottleneck. Native
  worst-case profiling/optimization and user-hardware60FPS approval remain open.
  Farm/city work adds a skyline-height gradient; full suburb/property art and
  aesthetic acceptance remain WORLDART-01. See testing/startup-world-diagnostics.

- PERF-09 (2026-11-07): parallel_physics activates existing independent-island
  solving, not within-island parallelism, automatic ECS synchronization/writeback
  parallelization or bitwise deterministic replay. Physics tuning is unchanged.
  Controlled serial/parallel results live in testing/parallel-physics. Actual slow
  race reproduction, broad60FPS and Windows hardware acceptance remain PERF-08.
  The paired15-rider active lab improves8.971s→5.830s/900 ticks, while small isolated
  cases slow from dispatch overhead. Native warm-window gains are modest and
  physics-span time remains roughly9–10ms/frame; PERF-08 remains open. All18
  non-timing lab observations matched on this host; this is not network determinism.


- RECOVERY-01 / REVERSE-01: live pickup placement and parking reverse are verified
  on this Linux host. P06-009 retains confined remount clearance/art approval;
  explicit exceptional rescue and unseen rear NPC approximation remain intentional
  separate policies. No claim that every possible trap is eliminated. Reverse
  uses the ordinary tire force/grip limits, not an absolute downhill speed clamp.
- ANIM-14: source-to-runtime audit starts the next Part02 review;104 context poses
  have no standalone clip and two runtime lean keys are authored extras. Missing
  playback is not evidence of missing source frames. Finger/support/silhouette,
  unresolved caller/native timing and human approval gates remain open.


- ANIM-15/16: runtime glove shapes/sockets are a visual prototype on the existing
  source geometry. Finger/thumb silhouettes and grip clearance need user art
  review; individual finger bones/tendons are not simulated. The static Blender recipe now reads the runtime profile and has a real
  GLB export/reload (ADR080); exact native/source tessellation and hard-normal
  parity, angular thumb/finger contours and human clearance/art approval remain
  ANIM-16 gates. See testing/part02-authoring/README.md. Weapon sockets are attachment metadata only;
  Part07 weapon gameplay is not implemented here. Morphs add a draw surface and
  bounded shared textures; no broad performance or Windows certificate is claimed.


2026-11-08 Part01 runtime review (ADR079): default velocity-muscle combat already
has swerve/countersteer and continuous release. Tall-solid/vehicle normal contacts
at1m/s now interrupt that sequence; ordinary riding/baseline gates are unchanged.
Vehicle-contact measurements are bounded diagnostic observations, not over-car
trajectory/vault acceptance. All29 Part01 boxes remain open; see
[the audit](testing/part01-runtime/README.md). Verified local-disc calibration
files exist; missing complete numerical calibration and unresolved reconstruction
source reuse permission remain distinct. No Windows/controller/human-feel claims.

- PART04-PALETTE: Metropolis now instantiates24 curated facade designs, with
  district/tower rules and model-derived crowns; it still does not integrate all
  eligible128storey-kit designs. BUSS4 upper intentionally uses neutral CLSTGA11
  aside from one detailed sign; other unique frontage/signage and final corner/
  foundation art require user review. Resident source inputs roughly double;
  hot streaming budgets remain unchanged. Measured native evidence and limits:
  testing/part04-world/README.md. Native release p95 is21.08ms (16.7ms target unmet);
  no controlled prepass comparison. Plain courts/mid-block ground, atmospheric
  wash and simple tower sides remain unfinished art. Production frame budget, long reverse/high-speed
  streaming, broad hardware and final visual approval remain open.

- PART05-DAMAGE: six crush roles now have twelve buckled original-source draft
  variants; seven topple roles still use intact whole-object art. These folds
  are draft deformation, not approved final severed/shattered art. There are no
  extra fragments, leaks or per-piece colliders. Ready hidden siblings add
  resident hierarchy/mesh/material cost, recorded in testing/part05-props. The
  existing48-body cap and solid crush envelope are retained. Full worst-case
  damaged-route60FPS, legacy-map destructibility, Windows/hardware and user art
  approval remain open under DESTRUCTION-02/V2-05.

2026-11-08 Part06 NPC-SEAMS-01: continuous civilian weights reduce measured mean
posed attachments but do not finish the disconnected draft volume topology. Some
coarse torso/shoulder/pelvis samples worsen; per-case worst/regression receipts
are in testing/part06-npcs/. One-level neutral-surface subdivision increases actor
triangles/vertex buffers and resident asset bytes. No full-crowd60FPS, production
art, all-motion or hardware certificate. Cow activity/navigation and landscape
remain deferred; accepted human/quadruped physics and cow skin bytes unchanged.

- Part07 club: one original three-piece training baton only. E/Q are remappable
  keyboard actions; pickup/drop settings-panel/controller workflow is incomplete.
  Pending-contact pickup edges are rejected; press again after contact. Q queues
  one drop after contact; crash/reset clears that queue. Side-to-side club handoff is immediate; holster/equip/drop art,
  finished grips/silhouette/clearance, bespoke impact audio, steal/chain/backhand,
  swept contact/result schema, exhaustive balance and human race enjoyment remain
  open. Static nonphysical pickups use bounded downward support with clamped
  terrain fallback when fixed support is unavailable. Thirty prepared held roots
  and up to12 pickups add resident hierarchy/render cost despite shared assets.
  Native evidence is scoped, not Windows/controller/60FPS or broad acceptance.

- Part07 raw hand captures show attached shafts and active grip profiles, but
  angular/protruding glove fingers do not establish convincing anatomical wrap
  or clearance. One right-side closeup is body-occluded. Preserve these images
  as draft evidence; do not infer approved grip anatomy or modeled-club fidelity.


- PERF-08 / ADR089: the latest user race capture confirms substantial CPU physics/catch-up cost as the player approaches the city. The current pass retains 60Hz, substeps, accepted rig and every static triangle while removing fixed-body bookkeeping and material-based obstacle collider duplication. This is a scoped optimization, not a60FPS/user-hardware certificate. New phase/window/backend counters distinguish inactive contact-graph pairs from active contacts. GPU timing, full route/crash stress and human confirmation remain open. UI prompts are briefs only; all original art / native integration gates remain open. See testing/performance-ui-prompts/README.md.


- UI-04/05: all28 screen-level generated PNG drafts are previewable, but they are not approved142source-resource replacements or layered production art. Actual outputs are1672×941 (main1671×941), below the requested4K master brief. Model fidelity, anatomy, alpha edge halos, component/variant mapping and typography need artist review. UI-05 tracks those masters; UI-02 retains native Bevy integration, settings/controller flows and viewport acceptance. No native UI/gameplay/performance changes in this pass. See testing/generated-ui/README.md.

- UI-05: all 28 initial screen compositions have rejected art direction. Only one corrected grotesque painted main-menu candidate exists; remaining screens, layered masters, exact retro coarseness and user approval are outstanding. UI-02 native integration remains deferred.

- UI-05 update: user approved the shared grotesque painted direction; four new scene variations now accompany the main sample. Individual composition approval, layered masters and native placement remain open. Earlier single-candidate limitation is historical.

- Native frontend first pass (ADR091): current-course entry, street visits and real settings/results are integrated. UI-02 remains partial: no runtime course swapping, campaign, purchases, selectable player profiles, rival conversations or network mode. Final art/typography, human controller/accessibility review and prolonged hardware/performance testing remain open. New frontend rider art is illustrative, not a new gameplay mesh.

- Random grid weapons: only the existing Club is distributed, by default to seven of fourteen rivals. Human weapon balance/grip art and complete arsenal acceptance remain open; player/police are excluded. Part09 now has an isolated entry receipt runner, not ten external playtests or a passed slice gate. Full current-course recovery/police suite and session metrics remain P09-AUTO-01/P09-001..007. The old on-foot-finish-not-implemented sentence in P07-004 was corrected against existing FOOT tests, without closing that broader gate.

- Approved portrait browser (ADR094): sixteen concept portraits are available in the Biker Bar, not selectable player profiles or simulation identity replacements. Names/bios describe concepts and do not promise implemented character-specific abilities. Decoded portrait residency adds 27 MiB RGBA plus existing seven screen images; GPU/backend and user hardware costs remain unmeasured. Human layout/accessibility and broad UI-02/UI-05 acceptance remain open.

- ADR095 rider dossiers preserve 4:3 with letterboxing on wide displays; portrait UI derivatives remain 768×576. Detailed bios are editorial adaptations under the user's later non-racist direction, not promises of unique simulated abilities. The Open Sans 1.10 substitute is not the original game's font. Full UI/art/profile/hardware acceptance remains open.

- AI-12: predictive traffic braking/passing, crossing-velocity sweeps and hazard-only upright-rider occupancy are integrated. The forecast assumes bounded near-term motion and cannot guarantee collision-free races after sudden impacts/cut-ins. Physical crash rules remain active; AI-11 human race balance/performance review remains open.
- WEAPON-MODELS-01: four original3D weapon families are reviewable via weapon_preview and optional ROADDASH_WEAPON_MODEL skins. All skins still use the existing club ability/contact/animation; chain/whip flexibility, separate family attacks, automatic character loadouts/runtime distance LOD and final art/performance acceptance remain open.
- ROAD-WALK-01: grounded adhesion fixes the reproduced support-edge cycling with unchanged step/capsule/solids. RACE-SETUP data was already4/3/4; new continuous paint and actual3250m entry evidence clarify the layout. Broader arbitrary-world traversal and hardware/art acceptance remain open.
- FOOT-SPAWN-01 remains open: GettingUp centre-ray placement lacks capsule/interior clearance. The demonstrated fully enclosed start was an invalid TEST setup, not a reproduced ordinary gameplay crash. Guarded stand-up placement, legitimate rooftop/car supports and explicit deep-invalid-state recovery need their own scoped fixtures; no new placement or rescue behavior is included here.

- DOWNTOWN-01: source-authored full-city trees/parking/patrol coverage retains open pavement/capsule corridors, but existing walker animation and sharp patrol turnarounds remain. Added resident props/vehicles/walkers have unverified user-hardware cost; final density/art and prolonged damaged-course performance remain open. Narrow side-street curb bays do not add parking to live race lanes.
- Part07 arsenal follow-up (ADR100): family identity, reach/stamina/cooldown and default original model skins are integrated; all five retain the existing shared side-swing contact keys and three-direct-hit rule. Flexible chain/whip dynamics, new damage rules/attack silhouettes, holster/equip, human fairness/grip art, automatic model LOD and final full Part07 exit remain open. Earlier Club-only limitations describe the prior implementation.
- Part08 follow-up (ADR101): last-seen search and HUD repetition defects corrected; broader NPC target discretion, moving foot capture, bespoke presentation, human four-racer/offline loop acceptance and hardware/performance gates stay open.
- Results/Busted R retry follow-up: native police integration exposed the frontend clearing restart before authority consumed it. ADR101 now routes one captured edge through normal Ride/Starting; full combined/native evidence owns verification. This does not implement moving police capture or finish the broader UI/art gates.
- Original-format frontend (ADR102): main/street use stored retail-sized control proportions and all entry pages use4:3; supported course/results/shop pages are adaptations, not reconstructed dynamic retail screens. Sixteen-card grid/dossiers retain user overrides. Jost is a licensed substitute, not exact FUT/BADLOC glyph parity. Centre crop can remove art edges. HUD/pause/settings formatting and final human/controller/font/art acceptance remain UI-02/UI-05; no complete all-screen visual parity claimed.
- ADR103 supersedes the user-rejected literal-retro ADR102 entry style. Menus fill the viewport with proportional cover crop; portrait grid/dossiers retain4:3. Clean type/native gradients and restrained focus are a presentation pass, not user approval or completed UI-02/UI-05. HUD/pause/settings and physical controller/Windows/accessibility/performance remain open. No original art bytes/gameplay/physics changed.
- Settings menu ADR104: grouped camera/controls/master audio/mute/fullscreen/VSync settings integrate with protected preferences and live adapters. Kick remains Space and mouse punches remain fixed alternatives; pickup/drop can now be rebound. Controller,Windows and real monitor/fullscreen restore/audible transitions require human hardware review. Native setter validation is not proof a driver/compositor honors VSync. Per-bus engine/SFX/ambience/music sliders,quality presets,subtitles and smaller-than720p/accessibility acceptance remain P15/AUDIO-013/UI-02 gates. Closing without saving intentionally keeps live session changes; it does not revert them. Old builds reject extended preference fields as read-only; restore backup for rollback.

- PERF-08 / ADR105: pass01 corrects facade retained/retiring selection hysteresis, but full-course driven racing, raw GPU timestamps, target frame budget and human visual acceptance remain open. Camera survey views retain farm physics actors and cannot certify downtown simulation load. See testing/polish-12/pass01/README.md.


Pass02 contact audit (ADR106): reproduced and corrected mixed-time downed-body/
wreck lift eligibility. Pre-contact matched-speed touches cannot gain a scripted
hop from a later target bounce. Current car/wall/rear thresholds and reverse/
pickup policy remain. Ordinary incidence was not measured; arbitrary enclosed
mount clearance, human feel/full-race and hardware acceptance remain open under
existing P06-009/P06-CONFINED and V2 exit gates. See testing/polish-12/pass02/.

## 2026-11-09 — Pass03 elevation integration limits

One authored profile now supplies Metropolis streets/terrain, branch navigation,
props and property foundations. Apron and field tessellation address interpolation
mismatch; both route/grid support is measured against actual pavement triangles.
This does not certify every frontage/pad, all moving-traffic merge cases or an
uninterrupted human full race. Human slope/crest readability, confined recovery,
production terrain art, Windows/controller and broad hardware/performance gates
remain open. The preserved flat building floors retain their existing stepped
foundation treatment; elevation work is not new architectural approval.

Pass03 isolated native costs: Metropolis bounded survey19.978ms average /
38.934ms p95; Valley world survey16.280ms average /19.208ms p95. Both exceed the
16.7ms p95 target; Valley exits1 for that gate despite complete15-section/657-root
geometry assertions. Before Metropolis visual capture overlapped authoring CPU
work and is not a valid performance baseline. No causal speedup/regression or
new broad performance acceptance is claimed.

- Polish04 / SKY-01: coordinated late-afternoon baseline has matched native
  evidence in testing/polish-12/pass04. Human art/readability and hardware gates
  remain open. Existing cubemap rotation moves its sun disc while directional
  light stays fixed; pass10 sky work must reconcile this. Cloud appearance,
  atmospheric depth and shadow grounding remain later sequential scopes.

- Polish05 / P15-004: reduced normal bias and wider existing cascade blend have
  paired native contact evidence (testing/polish-12/pass05), but static views do
  not certify elimination of temporal shimmer/cascade transitions at racing speed.
  Existing texel snapping retained. Four2048px maps/150m reach and caster policy
  unchanged; overlap increases work near boundaries. Full GPU cost, scalable
  presets, art and hardware acceptance remain open; no blanket shadow enabling.

- Polish06 / P11-004: original mipmapped road PBR now reaches default Metropolis;
  road/paving/paint scale and restrained wear have native evidence. Repeated
  tiles remain procedural approximations. Static screenshots cannot certify
  racing-speed shimmer or human art acceptance. Paving dirt follows slab edges;
  this is not curb-specific runoff, real drain grates or hydraulic simulation.
  Authored drainage, broad modular road/curb art and GPU/hardware gates remain
  open; no geometry/collision or facade-atlas change. See testing/polish-12/pass06.

- Polish07 / MAT-03: coordinated source finishes and runtime-equivalent roughness
  are baked into all facade distance assets. Original geometry/textures retained;
  distant normals still approximate subpixel procedural relief with projected
  geometric normals. Flat proxies cannot reproduce close parallax. Entrances
  gain restrained contrast without new geometry/signage; production ground-floor
  authoring and human architectural/art/temporal/long-streaming acceptance remain
  PALETTE-01/LOD-03 gates. See testing/polish-12/pass07 for bounded native evidence;
  no claim of target GPU cost, Windows coverage or user art approval.

- Polish08 / PALETTE-01 / V2-04-003: low-rise approach facades now progress through
  roadside businesses, workshops and residential fronts on existing envelopes.
  This retains the approved continuous street walls and fixed setbacks; it does
  not add detached farmstead models, new property layouts or extra vegetation.
  Complete site art, human transition/readability review, long streaming and
  target GPU/hardware acceptance remain open. Authored index groups share use
  selection but are not guaranteed spatial neighborhoods. Evidence: polish-12/pass08.

- Polish09 / SKY-01: exponential-squared haze preserves >99% contrast at100m and
  shares exposed horizon radiance with the sky. Non-HDR sky output bypasses
  Reinhard while PBR fog uses it, so a visible horizon mismatch remains under
  V2-06-007. An HDR trial produced all-black native frames with clean logs and
  was rejected; exact cause remains unresolved for pass10 investigation. This
  is analytic distance haze, not
  volumetric/height-dependent weather. Existing flat terrain silhouette,
  cloud shapes and drifting sun disc remain pass10 presentation issues; no fog
  concealment or geometry/LOD closure. Human racing-speed/readability, target GPU
  cost, Windows/controller and broad hardware gates remain open. Evidence:
  testing/polish-12/pass09.


- Polish10 / V2-06-007: the supported non-HDR procedural display cube now shares
  the fog's exposed analytic tonemap and correct world sun direction (ADR114).
  Clouds are intentionally stationary: the previous wind rotated the sun and
  environment incorrectly. Independent cloud-only motion is still unimplemented;
  the legacy wind setting is retained but inactive. Static authored exposure
  requires a display-cube/fog rebuild if a later runtime exposure system is added.
  Full landscape/art, temporal filtering, Windows and broad GPU/frame-budget
  acceptance remain open. ADR113's rejected black HDR trial remains historical.

- Polish11 / ART-04 / ANIM-09 / NPC-SEAMS-01: source ankle pivots now replace
  the shortened skin approximation, preventing boot flex from orbiting a point
  above the anatomical ankle. Landmark accuracy is not proof of complete boot
  surface contact, cloth/leg clearance or final gait quality. Existing civilian
  intersecting-volume seams/individual deformation regressions and hovering
  appearance in the close native walking fixture, fingers and full
  multi-angle art/hardware/performance acceptance remain open. Standing wrist
  inheritance is retained; no twist or accepted physics change. See ADR115 and
  testing/polish-12/pass11 for exact measured scope.

- Polish12 / COMBAT-FEEL-01 / P05-006: confirmed mounted hits now have bounded
  directional upper-body recoil. A rider's active attack owns its contact pose,
  so the ordinary flinch ages underneath and can be visually suppressed during
  a simultaneous strike; input is never delayed. Third-punch knockoff retains its
  existing mounted stagger/physical release. There is no block mechanic; LOS
  rejection is a miss. Native scripted reactions and contact landmarks do not
  certify full attacker/victim mesh overlap, subjective sound/feel, combat balance,
  all weapon/foot states or broad art/hardware/performance acceptance. No new
  launch, hit-stop or physical rig tuning. See ADR116/testing/polish-12/pass12.

- PERF-08 final polish regression: first Metropolis survey farm screenshot omitted ground despite a passing entity census/nonblank check; same-binary repeat rendered it, as did actual gameplay smoke. Fixed-frame survey capture lacks pipeline-readiness gating; root cause and cold-start visibility acceptance remain unresolved. Both images/receipts retained in testing/polish-12/final. Final p9539.747ms and repeat33.901ms exceed16.7ms; no60FPS/GPU claim.

- Landscape / V2-06-007 / WORLDART-01: original bounded grass covers the farm
  corridor and conservative unplanted verges only. Distinct crop palettes and
  soil ribbons improve near/mid ground; distant fields retain vertex-color
  variation. No wind deformation, new trees/stone walls or complete terrain
  silhouette rebuild. Existing field boundaries/fences remain. Grass receives
  lighting but does not cast expensive cascade shadows. Native dither range is
  not a guarantee of temporal/art acceptance at every speed. Full-route GPU,
  Windows/hardware and user visual acceptance stay open. PERF-08 cold-start
  survey readiness and the existing unmet16.7ms p95 target remain separate.
  Evidence/provenance: testing/landscape-graphics/README.md and ADR117.
  Final landscape verification: Metropolis p9534.765ms and legacy Valley
  world-survey p9518.597ms exceed16.7ms. Valley exits1 solely for that benchmark;
  geometry/657roots/15sections/224dressing and overlap assertions pass. Original
  visual candidates and58nominal-width shoulder overlaps were rejected; final
  full-polygon clipping matches6,606independent zero-overlap quads. Final121native
  captures and18-case lab are retained; no blanket visual/hardware acceptance.

- Farm follow-up (ADR118): user rejected the abrupt ADR117 road/grass edge and
  reported cows on fences. Gravel-to-short-grass verges and whole-field placement
  now address those specific defects. New props reuse existing draft art; animal
  deformation, full landscape art, GPU/performance/hardware approval remain open.
  The16 added props have fixed bounds and do not remove legacy collision. See
  testing/farm-verges for native captures, tests and any outstanding findings.

- ADR119 clothing/grass: user selected helmets with stylized clothing, not final
  visual approval. Original body silhouette and glove morph path are retained;
  vertex cloth shading is broad variation, not detailed plaid tailoring. Human
  NPC faceting does not resolve existing anatomical seam/art gates. Six curved
  blades increase geometry/memory; measured performance and hardware coverage
  remain separate gates. Grass covers the bounded farm corridor, not every field.
  Shader wind is visual only and does not react to riders. See testing/stylized-humans-grass.

  Final ADR119 survey p9535.341ms still misses16.7ms. Grass resident attribute/index
  payload estimate115.46MB (previous49.62MB), excluding engine/CPU/GPU overhead;
 333batches and65–110m visibility do not eliminate this resident memory cost.

- ADR120 living-city AI/cast:14active opponents use the approved portrait names
  and descriptor-based loadouts; George/Elif remain bar cast rather than expanding
  the15rider race. New pace/curve planning, lane memory and bounded backing need
  human fairness review. Rear sweeps reduce unsafe reverse attempts but do not
  guarantee avoidance of a fast vehicle entering the gap after a check.
- ADR120 city:20%sidewalk area expansion approximates width on irregular blocks;
  individual widths differ. Existing building IDs/envelopes remain with landward
  setbacks. Static procedural fountain jets are not fluid simulation. Final
  civic/cafe art, full-course streaming, GPU timing and target hardware acceptance
  remain open.192urban pedestrians,32extra urban main cars, more crossing cars
  and1521street props increase work; no performance improvement is claimed.

- ADR121 bounded escape routing covers mounted and walking AI with real concave
  trap fixtures; it does not prove every procedural enclosure escapable. The local
  planner has a72m radius and1024-expansion cap; sealed spaces cannot be escaped
  physically. Live obstructions can still enter after a sweep. Existing foot
  rescue remains the exceptional fallback. Full-race deadlock and performance
  acceptance remain open. Civic/park art remains subject to user review.

- ADR122: added civic foliage costs geometry/instances; no optimization gain is
  claimed. CITY-WIDTH-01 is only a map proposal; runtime street trimming, edge
  dressing, navigation and performance validation remain pending selection.
  Sixteen named portraits remain distinct art, not sixteen unique rider meshes.

- ADR123 supersedes ADR119's rejected rider clothing presentation: original biker
  outfit is restored. This does not close broader human art/skin seam gates.
  Detailed civic stonework/PBR is resident geometry with no new interior or snag
  colliders. Static sky IBL is not local occlusion, reflections or dynamic GI;
  SSAO/temporal-AA and Windows/target-GPU approval remain open.
- CROP-TRAIL-01/ADR124: wheat covers grain parcels within72m of the farm route,
  with1m contact cells and current-race persistence only. Existing grass remains
  wind-only. Damage state is bounded, but crop mesh memory and per-tile draws cost
  resources. Scripted shape-contact native evidence is not a free-running crash
  or full-course/GPU benchmark; active crop-view p95 remains above16.7ms.
- ELEVATION-01/ADR125:25% stronger relief remains below4% analytic longitudinal
  grade. Sampled support error is close to the12mm gate; more relief would require
  tessellation review. Existing full-map planar/identity/lane content is retained;
  both-branch native recovery now passes; user feel remains an acceptance gate.
- ADR126 queue passing uses bounded live forecasts and static sweeps, not lane
  reservations or guaranteed general deadlock freedom. Narrow/sealed gaps can
  still wait, and unexpected vehicle motion can invalidate a forecast. Additional
  sweeps need combined performance review; player handling is unchanged.
- POLICE-ART-01/ADR127: officer texture and touring equipment are integrated with
  the original rig. Beacons are static emissive lenses; no flashing lights or
  new siren behavior was added. This remains one resident officer, with existing
  pursuit/arrest limitations. Native gallery/behavior proof does not close human
  art/fairness, full-course performance or Windows/hardware acceptance.

- Final fresh-agent integration:481tests and12release native cases plus18paired
  lab cases pass. Route wall p9522.376ms and crop-contact p9520.527ms exceed16.7ms.
  One existing600-second turning soak remains intentionally ignored. No broad
  GPU/full-course/Windows certification or user art/feel approval is claimed.
  See testing/next-pass for accepted receipts and excluded interrupted/harness runs.

## Hills follow-up (2026-11-10 / ADR135)

HILLS-01/02/03 add a separate course with strong road relief, western branch
streets rejoining inside town, dunes and requested countryside dressing.
Final art, horse gait/physical animal rig, full driven-race balance/performance
and broad hardware coverage remain open. Horses are static solid scenery.
The initial native camera survey paused before Rapier's first active step,
leaving its query broadphase uninitialized; its road-support failure cannot
serve as passing evidence. A bounded countdown warmup precedes the corrected
survey. Initial images also exposed nearest-road remote terrain height jumps;
continuous remote terrain replaces them without flattening the authored road.

Hills corrected final survey passes1506road-support probes (max25.898mm), with
11completed native captures. Both town branches, steep-hill and baseline-rig
recovery pass, as does the unchanged Metropolis recovery control. This resolves
the initial survey initialization failure; prior rejected logs are retained.
Visual authoring remains procedural/draft, and these scripted cases are not a
full manual race or performance certification.

## Hills polish (ADR136 / HILLS-04/05/06)

Mounted cowboys are bounded scenic kinematic actors with fixed-step animated
colliders; they do not eject riders, dynamically avoid traffic or use new animal
ragdolls. Guest poses are derived from replicated race time; broader live-network
latency/visual review remains open. Dust is a bounded sprite approximation, not
volumetric simulation; it requires wheel contact on dynamic bikes. Bird flocks
are presentation-only and have no collisions. Clouds remain stationary to keep
the visible sun and lighting coherent. Native art and full-course performance
remain separate from automated geometry/state checks.

Initial polish test failures traced to an ungated render system requiring render
assets in headless fixtures; gating on the Hills atmosphere resource fixes that
without physics changes. Steep-shelf rock coverage now uses physically smaller
solitary stones with matching clearance, alongside original large clusters.
Initial close-up captures used the player camera for bird culling/billboards
before the survey camera override; corrected ordering uses the final view.

The extra native pass exposed a Hills recovery trap after a high-speed hill
crash: the stationary wreck lies outside the continuous rail while automatic
foot recovery oscillates inside it. Cumulative walking distance is not proof of
progress. The correction uses bounded displacement to detect the exceptional
trap and retains the existing configured rescue delay and penalty; it does not
remove rail collision or alter the physical rigs. Rescue receipts must be
reported separately from ordinary remounts.

Final native validation resolves the indefinite Hills trap via the explicit
120-second/+5-second fallback; west/east receipts distinguish it from unassisted
recovery. Baseline and Metropolis controls need no rescue. This does not add
rail vaulting or make the continuous barrier traversable; full-course recovery
balance and human playtesting remain open.


## Hills revision follow-up (2026-11-10, ADR137)

The continuous-rail composition above is historical: the revised hills now
protect bends and leave straights open. A physical open-straight recovery test
and native west/east fixtures remount without exceptional rescue. The original
delayed fallback remains available for exceptional traps.

The new city opening reuses the source street/facades and its actual pavement;
full-course art/feel and hardware acceptance remain open. The canyon deliberately
has open deck edges without low railings, with square solid suspension hangers.
Physical tests verify support and the deep floor; production steering and combat
kicks on both sides trigger real falls through the Assisted controller. The
active native fixture checks a lateral departure and physical recovery. This
does not prove every combat approach or canyon recovery scenario. The native
Dynamic trial remains mounted during its physical descent; it does not establish
automatic return from the canyon floor to the bridge road.
Heavy traffic has supported authored trajectories and native motion evidence;
full multiplayer session review and broad vehicle/animal behavior remain open.

Performance comparison and remaining limits are recorded in
`testing/hills-revision/REPORT.md`. Historical prior-build timing is not the
paired baseline. CPU/wall measurements must not be described as GPU timestamps
or universal hardware/60 FPS certification.

- AI-QUEUE-02/P08-005: the reproduced civilian circular wait is corrected (testing/traffic-stall). A still image cannot establish the cause of every stopped car. Legitimate obstacle yielding and broader full-race traffic acceptance remain distinct from this regression.

## Hills course and landscape delivery (2026-11-10, ADR138)

UI-COURSE-01/HILLS-11/SKY-SUN-01/SHADOW-RANGE-01 and existing HILLS-07/08/10
are implemented with 590 passing workspace tests (one existing ignored soak).
The native course selector verifies Metropolis → Hills → Metropolis, profile
credits/preferences, retry and menu return. All seven native cases pass across
recorded pre-polish/final binary hashes; the final active run replaces the first
active receipt after river texture/camera polish. The interrupted first east
attempt is excluded. Assisted falls/recovery and all route/control recoveries
use no exceptional rescue; Dynamic remains mounted during its 82.677 m chassis
descent. Six farm parcels and final river/sun/cloud/bridge captures passed visual
review. Four clean ABBA shadow-distance runs with clouds constant measured
150 → 500 m median window means of 21.848 → 21.773 ms and P95 window means of
23.544 → 23.814 ms. Differences are within run variation, not a speedup. This
Hills-view fixture is not a full Metropolis/GPU benchmark or 60 FPS certificate.
See `testing/hills-delivery/REPORT.md` and `testing/hills-delivery/PERFORMANCE.md`.

All six farm parcels now choose gentler natural land and check whole-field
slopes/relief plus road-cut clearance. This does not flatten the course's strong
elevation changes. Regular bridge hangers connect through girders/outriggers;
open deck edges still permit falls. Water is presentation over resident terrain,
not swimming/fluid physics or an added collision surface.

Volumetric cumulus and a directional-light-aligned sun are bounded presentation,
not weather simulation. The 500 m Metropolis/Hills shadow policy preserves the
two near splits but trades outer resolution and rendering work for distance.
No speedup, GPU timestamp, sustained 60 FPS, full-course human art/feel or Windows
hardware acceptance is inferred. Legacy maps retain their 150 m default shadow
range; explicit cloud-off and 150 m shadow controls remain available.


- ADR139/HILLS-01/AI-QUEUE-02/CITY-CIVIC-01: reported rear-grid building
  obstruction, false AI stops through Metropolis bends and thin green asphalt
  seam have targeted/runtime/native corrections; see testing/hills-grid/REPORT.md.
  594tests and both-rig native recovery pass; complete human race/passing balance,
  other hardware/network conditions and60FPS remain open. No blanket guarantee
  that every traffic blockage is eliminated.

- RFU-AI-01–08/ADR141: targeted racer/killer decision and oncoming-pass corrections
  have604-test/code/build verification for RFU-AI-03/04/05/07. Other gameplay/native gates remain open. Forecasts do not reserve lanes; an opening can close
  after entry. Sealed roads legitimately wait. Full native race, human aggression
  balance and broad performance/hardware acceptance are separate gates. See
  testing/issue26-racer-ai/REPORT.md; no physical rig tuning is part of issue26.
