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
  simulation, rigged skin or active physical animation.
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

Latest corrections: all human states share original procedural geometry/outfit;
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
