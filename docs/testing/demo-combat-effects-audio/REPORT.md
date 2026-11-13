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
