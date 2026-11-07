# Known issues and remaining acceptance

Updated 2026-10-30. This is not a list of fabricated failures; distinguish known
missing features, architectural debt and unverified platforms.

- P01-WIN: Windows GPU/audio/input launch and clean install have not been run on
  this host. Part 02 Ubuntu/Windows CI run 41141131978 passed, including the corrected
  UTF-8 metadata audit. Part 03 remote CI is recorded separately in its PR. No Windows hardware certificate is claimed.
- P01-HW: native scripted rendering/audio is verified on Fedora/RTX 3080/Vulkan.
  Windows hardware/input verification and clean installation remain open. A short
  scripted smoke is not a production benchmark or a real gamepad test.
- P01-SPLIT: workspace has real core/content boundaries and subsystem plugins;
  game.rs still combines vehicle/AI/combat/race. No server/netplay implementation.
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
- P05/P06/P07: kicks are implemented; weapons/stamina, manual foot override and foot finish remain open.
  Ordered bike gates are verified in P02. Recovery auto-seeks the actual bike.
- P06-CONFINED: assisted recovery is verified in representative fallen poses,
  not arbitrary fully enclosed geometry or moving traffic traps. Collision-clear
  crash separation falls back in confined spaces; manual override/stuck recovery
  remain open. Ordinary crashes no longer inject upward momentum.
- P08/P12/P13: police, real garage/economy, career and multiplayer absent.
- P02-END: missing an ordered gate requires R-to-retry; unfinished riders stop at
  the finish. There is no reverse driving/manual return-to-gate feature yet.
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
