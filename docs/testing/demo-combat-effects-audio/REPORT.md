# Demo combat, effects and audio

ADR175 adds an authored fixed-clock arm/leg contact window, independently
tested rider and chassis contact volumes, and bounded AI exchange exits.
Bike contact uses a lateral response and metallic cue. Rider contact retains
the existing strike, recovery and confirmed-audio path. Protocol 4 carries
the new cue types.

Hills and Metropolis share the bounded rear-wheel spray pool. Surface and
wheel support drive spray intensity and loose-surface sound. Existing
windblown sand and city steam provide location effects. Nearby flocks gain
altitude and nearby traffic can trigger a pass-by or air release.

The seven recorded assets are Creative Commons CC0 clips from public
Freesound previews. See the source URLs and hashes in the asset register.
The source for the rev is egomassive's edit of Allen Cote's Harley_01.
Creators of the other clips are lmbubec, loganzsound, Cornwallis89, Sheyvan,
qubodup and PostProdDog. The three running RPM layers are loop edits of the
same rev recording, and the idle is a separate recording. They are demo
assets, with full bike-class recordings and ear-based mix acceptance open.

Kenney Impact Sounds is an alternative CC0 source. Mixkit sport-bike files
and Sonniss GDC tracks were reviewed as candidates. Those libraries are
absent from the repository distribution.

## Verification

- A fixed-rate geometry fixture moves a bike into a kick three ticks after
  the authored key and verifies body-versus-bike classification.
- The native Hills smoke fixture initialized eleven playback sinks, completed
  both remounts, and reached results. This is a device initialization and
  gameplay check, with audible balance and wheel-spray art review still open.
  [Hills smoke metrics](hills-smoke.json).
- Native Metropolis also initialized eleven sinks and passed both recoveries.
  [Metropolis smoke metrics](metropolis-smoke.json).
- The first 38-second two-process run replicated attack, crash and on-foot
  recovery. Both peers remained in the running recovery stage when the
  fixture expired. Longer same-machine verification is needed to establish
  remount for this run.
- Both physical AI fixtures confirmed hits without injected attack events.
  The moving pair exercised chain, kick and punch, and the existing
  normal-engine approach fixture confirmed contacts from 20 m back and
  from a neighboring lane. Confirmed chassis strikes count in these
  fixtures alongside body hits. No race-pace adjustment was made.
- The distinct farm, hill and city beds are synthesized. The two additional
  Freesound ambience candidates returned HTTP 403 during intake. The build
  uses seven recorded clips and three original ambience beds.
- The latest-build Hills polish survey passed all seven captures and its
  grounded dust and fixed-capacity assertions. The sampled plume reads
  subtly against the tan sand. See [effects receipt](hills-effects.json).
- The 70-second protocol 4 native pair passed remote attack, crash,
  on-foot recovery and remount on both peers, with 1,085 matching shared
  snapshots. [Native pair receipt](online-pair.json).
- Formatting, strict workspace Clippy, locked debug build, provenance audit
  and roadmap consistency passed. The workspace recorded 716 passed tests,
  two ignored, and the two known failures for high-speed ragdoll anchor error
  and Metropolis static collider trajectory parity.

## RPM audio audition

ADR176 replaces the default rev-clip player engine with continuous original
inline-four synthesis. The existing gear and RPM drive the firing rate and
shift response. Throttle shapes the exhaust spectrum separately from RPM.
Set `ROADDASH_ENGINE_LEGACY=1` to audition the prior recorded loops. The
recorded pass-by and the other sound buses remain available. Listen at a
steady RPM, through successive upshifts, and during throttle lift before
accepting the new timbre.

The offline 17-second audition renders the exact engine generator over an
idle, five-second 4,000 RPM hold, two shifts and throttle lift. The comparison
render approximates the old sink resampling. Over the constant-RPM section,
200 ms RMS windows varied by 0.05 dB in the new render and 3.96 dB in the
recorded-loop approximation. The generator test checks 1,200, 4,000 and
7,600 RPM firing rates and load contrast.
The locked native game build and Metropolis smoke passed with eight playback
sinks initialized and both remounts completed. Strict Clippy, formatting,
the roadmap index and the asset audit passed.

## Recorded street candidate

ADR177 auditions three loops trimmed from Joseph Sardin's CC0
[75 cc street acceleration recording](https://bigsoundbank.com/75-cm3-motorcycle-acceleration-s0603.html).
The live gearbox controls pitch and blends the two pull bands. Throttle and
shift cut blend toward a quieter tail from the same take. The source contains
no tachometer data. The RPM anchors are playback tuning. Audible acceptance
and the broader AUDIO-008 acceptance checks remain open.

[Engine Simulator](https://github.com/ange-yaghi/engine-sim) is a separate
research candidate. The upstream code uses MIT terms and the application uses
Windows audio. The
[headless exporter fork](https://github.com/Talhasarac/engine-sim-sound-exporter)
documents Windows builds and fixed RPM/throttle WAV capture. No upstream code,
presets or impulse responses were integrated.

The locked build, strict workspace Clippy, formatting and asset audit passed.
The native Metropolis smoke initialized eight playback sinks and reached
the racing and first crash captures. This run did not log completion of the
recovery cases. Listening remains open for the recorded candidate.

## 689 cc parallel-twin engine

ADR179 captures the custom twin from Engine Simulator v0.1.11a under Proton
11.0. The preset is in docs/testing/demo-combat-effects-audio/engine.mr and references the
simulator's bundled minimal_muffling_02 response. The runtime uses six
full-throttle and five throttle-release clips. At 9,000 RPM the closed-throttle
voice reuses the 7,500 RPM sample. Clips use 16 crank cycles at a common
256 frames per cycle, with a two-cycle wrap crossfade. The live RPM selects
and resamples nearby bands. Throttle blends between the two layers and a
gear increase briefly cuts load. The shipped assets are captured output.
The game has no Engine Simulator or Proton runtime dependency.

Formatting, strict workspace Clippy, locked debug build and asset audit passed.
The Metropolis smoke initialized eight audio sinks and passed the race fixture
and both remounts. The workspace test command reproduced the two existing
Rapier parity failures in high-speed ejection and standalone static chunk
trajectories. The runtime mixer audition stayed below clipping through three
scripted upshifts. Full-course listening remains open.

The locked release build was also verified with the comparison overrides
unset. The Metropolis smoke initialized eight playback sinks and completed
both remounts and the results fixture. Run the built game with
`./play-bevy20.sh` from this worktree.

## Spatial bike engines

ADR180 moves each audible bike's engine source with its chassis. Bevy pans
against the chase camera. The engine mixer applies distance falloff and
changes playback speed from radial velocity. A separated bike plays at idle
from the live wreck position through recovery. The emitter returns to the
riding chassis on remount. The obsolete rival pass-by trigger is replaced
by continuous spatial voices. Network rider load is inferred from snapshots.

ADR181 blends a filtered portion of the loaded exhaust into the coast voice
at rolling RPM. Its release envelope is slower than the upshift cut. The
seven-second scripted throttle-tap render peaked at −9.6 dBFS.

The locked release Metropolis smoke verified multiple active spatial bike
sinks, an audible local source at the wreck, both remounts, and the results
fixture. The Doppler and distance unit test passed. The workspace run
reported 718 passes and the two known physics parity failures. Full-race
headphone and multiplayer listening remain open.

## Six-slot map picker

ADR182 presents Metropolis and Hills alongside four blank, non-interactive
slots. The native release walkthrough verified the six cells, launched Hills,
retried it, then loaded Metropolis and returned to the menu. Profile credits,
portrait, preferences and handling survived both world handoffs.

## Hills terrain art

ADR183 uses the supplied images as visual references. The Hills survey ran
on the release binary and captured eleven views. Terrain uses existing mesh
normals for rock exposure and procedural color for grass and strata. The
collision mesh and course grading use the existing terrain vertices.
