# Known issues and remaining acceptance

Updated 2026-10-30. This is not a list of fabricated failures; distinguish known
missing features, architectural debt and unverified platforms.

- P01-WIN: Windows GPU/audio/input launch and clean install have not been run on
  this host. Prior Windows CI compilation/tests/features passed; its metadata audit failed
  with cp1252 decoding. Explicit UTF-8 is fixed and regression-tested; the new
  remote matrix result is still pending. No Windows hardware certificate is claimed.
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
- P03: riding still uses a kinematic assisted controller, without wheel suspension,
  combined tire forces or three handling presets.
- P05/P06/P07: no kicks/weapons/stamina, manual foot override or foot finish.
  Ordered bike gates are verified in P02. Recovery auto-seeks the actual bike.
- P06-CONFINED: assisted recovery is verified in representative fallen poses,
  not arbitrary fully enclosed geometry or moving traffic traps. Collision-clear
  crash separation falls back in confined spaces; manual override/stuck recovery
  remain open. Ordinary crashes no longer inject upward momentum.
- P08/P12/P13: police, real garage/economy, career and multiplayer absent.
- P02-END: missing an ordered gate requires R-to-retry; unfinished riders stop at
  the finish. There is no reverse driving/manual return-to-gate feature yet.
- P02-ACTORS: cows are static solid graybox animals; pedestrians/cross traffic
  use bounded fixed patrols, not navigation or traffic-light/priority AI.
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
