# Hills cattle crossings and Western town encounters

The existing cattle crossings at 9.20 km and 11.05 km now use two rows of
three. New crossings at 8.87 km and 11.80 km use the same formation. Members
share a crossing clock and follow the existing support and cow knockdown
paths. The cow model and collider face local +X, so patrol yaw aligns that
axis with the current direction of travel. The gait swings in the matching
longitudinal plane.

Western town has roadside brawls at 7.525 km and 8.080 km, a center-road
brawl at 7.880 km, and shootouts along the roadside at 7.710 km and 8.310 km.
All five pairs use existing pedestrian rigs and colliders. The actors
alternate gestures near a racing player. Punch sounds, gunshot sounds and
muzzle flashes are presentation. The gun props are original low-poly art.
Ambient audio is generated locally and the online cue format stays unchanged.

The gunshot comes from Joseph SARDIN’s CC0 [Shot Beretta M12 9mm](https://bigsoundbank.com/shot-beretta-m12-9-mm-s0437.html).
The game asset uses the first 1.85 seconds of the [published MP3](https://bigsoundbank.com/UPLOAD/mp3/0437.mp3),
with a low cut, tail fade and gain reduction in 48 kHz mono PCM.
The town brawls reuse the existing punch recording.

## Verification

The focused tests passed for four six-cow formations, both travel headings,
five paired encounters, four attached muzzle flashes, and local punch and
gunshot cues. Formatting, strict Clippy, locked debug/release builds, asset
audit and roadmap synchronization passed. The workspace suite reported
732 passes, two ignored tests and two existing Rapier failures
(`wall_impact_and_high_speed_ejection_remain_bounded` and
`standalone_static_chunks_keep_contact_ccd_and_support_trajectories`).

Native Hills smoke passed in debug and release. The release run initialized
seven playback sinks and completed both remounts with zero exceptional
rescues. The automated checks cover the scene mechanics. Race-speed visual
readability and traffic interaction remain open on the live checklist.
