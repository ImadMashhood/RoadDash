# Normal rivals and recorded race audio

Normal is the default for offline races. Choose Hard under F2 → Rider Gear → Rival difficulty, and use Save to keep the selection across launches. The preset is captured on race start or retry. Normal multiplies each rival’s existing drive and pursuit pace budget by 0.88. Hard uses the existing budgets. Online races retain Hard. The game continues to use the same player handling, crash, remount, traffic avoidance and strike rules.

Confirmed crashes emit a one-shot at their world-space contact. Physical collisions use a solver contact when available, with a collider-surface fallback for assisted contacts. Confirmed melee knockoffs use the defender-side impact. The chase camera is the stereo listener. Rider and wreck runovers have distinct samples. Online audio keeps the existing event IDs and uses an optional runover discriminator with a compatible legacy cue.

The default engine uses the existing street motorcycle samples, driven by RoadDash RPM, load and shifts. The prior twin remains available through `ROADDASH_ENGINE_TWIN=1`. Wind, ambience, race cues, and one-shots use source recordings. [Source ledger and edit details](../../third_party/IMPACT_RECORDINGS.md).

Full-course human comeback and headphone mix acceptance remain open.

## Verification

A paired straightaway fixture induced a crash after ten seconds of racing and
used automatic recovery. Both presets remounted after 6.22 seconds.

| Preset | Gap at remount | Gap 30 s later | Gap 60 s later |
| --- | ---: | ---: | ---: |
| Normal | 196.2 m | 249.0 m | 96.4 m |
| Hard | 233.0 m | 430.0 m | 419.8 m |

Normal closed 152.6 m in the second 30-second interval. Hard closed 10.2 m.
The Hard pace fixture reached 49.28 m/s at 30, 60 and 120 FPS.
The settings and frontend fixtures cover saved choice, launch and retry.
Collision tests cover the position and count of confirmed wall, rider and
wreck impacts. Additional tests cover confirmed punch/kick, runover wire
compatibility, loop decoding and differential ragdoll/recovery.

Formatting, strict workspace Clippy, locked debug and release builds, asset
audit, and roadmap checks passed. The full workspace run reported 730 passing,
two ignored and the two existing Rapier failures
`wall_impact_and_high_speed_ejection_remain_bounded` and
`standalone_static_chunks_keep_contact_ccd_and_support_trajectories`.

The Linux release smoke passed on Metropolis and Hills. Both runs opened
seven playback sinks, verified nearby engine playback and wreck idle, and
completed two remounts and the results fixture. Hills used zero exceptional
rescues. An earlier debug run under concurrent test/build load reported
audio buffer underruns. The release smoke checks playback and recovery.
