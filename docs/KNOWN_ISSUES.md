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
- P01-SETTINGS: configurable keyboard JSON exists, but gamepad support, in-game
  rebinding and persisted comfort/audio settings remain P04/P15 work.
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
- P05/P06/P07: no kicks/weapons/stamina, manual foot override or foot finish.
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
