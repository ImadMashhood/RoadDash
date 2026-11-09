# Part08 scoped police research — 2026-11-08

R01 `arkits/road-rash`, clean local checkout at
`7aecf6044f2dd773b925b325823f30d648d61c89`, authored2026-10-31T14:36:14-07:00,
was inspected directly. Relevant sources: `src/police.cpp` (config/constructor,
pursuit relationships, arrest and periodic spawn predicates), `src/race.cpp`
(police_think, spawn_cop, find_cop and non-racing identity), `src/race.hpp`
(cop/busted flags), `src/career.cpp` (busted outcome and penalty). This is a
reconstruction's claimed behavior, not an independently executed retail police
playthrough or numerical parity certificate.

Observed design: one separate cop outside places/finish order; periodic spawn
opportunities based on tick masking; level/place/personality-dependent chase
relationships and authored bike/stat selection; proximity and relative bike speed
checks for bust; walking target uses the live walking actor rather than the parked
bike. The reconstruction includes snap/ring-window behavior unsuitable for our
fairness policy. RoadDash does not reproduce its original integer thresholds,
spawn teleports, formulas or code. The existing `rr_core::pursuit` is rival AI and
is not misrepresented as an existing police implementation.

The checkout has third-party license notices, but no verified root implementation
license. Reuse permission remains **UNVERIFIED / REFERENCE ONLY**. No source,
retail media, EXE tables, sprites or sound is copied or newly shipped. Existing
`docs/animation/police-slots.json` (45slots,33populated/12empty) and police pose
context supply documentary family hints only; unverified source ordering and
frame meaning are not claimed as completed police animations. No original disc
police execution was performed in this pass.

Independent policy: one resident motorcycle patrol at2.5% of each selected route,
never reinforcement spawning; same physical rider/vehicle controls, top speed,
acceleration/braking/grip/contact/three-hit knockout/recovery as ordinary riders;
10Hz detection plus conditional60Hz safe-contact checks;8s launch grace;30m/s limit;90m LOS;1.5s continuous warning;
3s out of sight plus5s last-known search before escape;2s safe arrest dwell inside
1.8m at <=1.2m/s target/closing speed. Confirmed melee hits (including kicks and
NPC hits) are recent offenses; windup/whiffs are not. Detection is currently
player-targeted; complete discretionary violation/archetype tuning remains open.
A stopped officer can park and enter the existing solid runner/navigation path;
fallen/getting-up officers have no arrest authority. No arrest during falling,
getup, mounting, immunity, unsupported/elevated separation or obscured contact.

Art/audio are original bounded draft presentation: existing project-original
motorcycle/human with palette6, two shared blue/white marker cuboids, current
riding/attack/run clips and a generated dual-frequency sweep cue. These are not
finished bespoke uniforms/vehicle/gesture art. There are no new dependencies,
external assets, proprietary media or implicit visual/legal approval.

A nearby moving on-foot target explicitly requests ordinary stopping/parking within24m, then shared-speed foot pursuit. Moving capture/tackles remain deferred; the1.2m/s safe-arrest limit lets a continuously moving player escape contact. Stationary-foot native evidence does not close full running-capture or human fairness acceptance.
