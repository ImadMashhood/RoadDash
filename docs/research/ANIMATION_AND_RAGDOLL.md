# Animation, locomotion and active-ragdoll research — 2026-10-31

This is an initial comparison for V2-02/V2-03, not an adopted animation/physics
rewrite. Current engine: Bevy 0.17.3 / Rapier 0.32.0. The user-approved passive
ragdoll remains the control fixture. No new library, C#/C++ code or media was
incorporated from these repositories.

## Comparison and decision

| Candidate | Observed design / useful idea | Fit, risks and current decision |
| --- | --- | --- |
| Sergio Abreu active-ragdolls | Unity 2020.1.9f1 sample separates an animated target and a physical body; modules organize physical pose following and control. | Useful small reference for bounded pose-follow torque and independent behavior modules. Historical Unity APIs, old maintenance, no Bevy integration; study only, never import its rig/media blindly. |
| Hairibar.Ragdoll | Separate target and physical bone hierarchies, animation matching, distinct position/rotation matching, and rendering the physical pose. | Strong reference for preserving a clear physics/render boundary. Unlimited world-position matching could defeat contacts or inject energy; compare capped rotation matching first. Unity package, not a Rust dependency; reference only. |
| JoltPhysics | Ragdoll/skeleton support and motor-driven animated poses, including powered-rig examples. | Useful mature constraint/motor and stability comparison. C++ engine substitution is not needed for this problem. Keep Rapier; study algorithms and benchmark design, not a backend migration. Sample assets need separate checks. |
| bevy_mod_inverse_kinematics | Positional and pole targets; README maps Bevy 0.17 to plugin 0.10. HEAD is plugin 0.12 / Bevy 0.19. | Relevant to foot, knee, elbow and bike contacts. HEAD is incompatible with this build; evaluate a pinned compatible release before considering integration. Licensing declaration exists but root license-file/API lookup is unresolved; no incorporation. |
| ozz-animation | Animation sampling/blending, offline conversion/toolchain and IK-oriented examples in a C++ animation library. | Useful contact/locomotion and blend/compression reference. FFI/build/format costs make an immediate dependency unjustified; use existing Bevy tools first. Core MIT grant was read, separate third-party/sample rights still apply. |

These are architectural comparisons based on pinned READMEs/licenses and exposed
features, not compiled cross-engine experiments. Source-level motor/IK prototypes,
energy/time-to-recover comparisons and multi-racer profiling are explicit future
acceptance tasks. No candidate is claimed to reproduce Road Rash or solve gait
quality automatically.

## Proposed evaluation scenarios

- Flat, uphill/downhill, uneven verge, steps/sidewalks and moving-obstacle walking:
  measure foot slip, support/contact error, pelvis pops, turn responsiveness and
  stride-to-world-velocity agreement.
- Passive versus active ragdoll on the same old body/limit/damping setup: ground
  tumble, car strike, repeated impact, aerial fall, steep hill and confined space.
  Measure peak energy/velocity, joint error, penetration, time to control, ability
  to brace/roll/get up and how much authority the player actually has.
- Conscious recovery controls may request bounded forces/torques. Unconscious
  state, cooldown, support/ground probes, energy caps, passive fallback and
  accessibility auto-assist must be explicit. No animation transform writes to
  authoritative bodies; no instant posing through cars or jetpack-like input.
- Compare rotation-following, support-aware balancing and procedural foot IK
  independently before combining them. Keep the accepted unassisted ragdoll as a
  selectable control fixture and require user approval for default changes.

## Road Rash reference

The local arkits/road-rash snapshot is inspected at
`cd5504caea48aee59806dbfd11c920f33df21200`. race.cpp's collide_traffic/contact_car
path passes both objects' motion/separation state into collision handling. That
supports studying relative movement and explicit contact response; it does not
provide a ready-made modern SI rigid-body flip. Original units, separation laws,
callbacks and crash state transitions need independent calibration. No C++ copied.

In the current game, impact detection samples BikeState.impact_velocity before the
solver, but previously detached bodies could be initialized from post-solver
Rider.motion_velocity. Correct that handoff before attributing every backward
crash to the anatomical ragdoll. For a rider at 100 and a same-direction car at
60 km/h, 40 is closing speed, not automatically ground speed after impact;
opposing travel gives 160 closing. Contact normal, mass/kinematic assumptions and
the chosen arcade crash response remain necessary inputs.

## Synty/POLYGON-style game observations

The art reference is Synty's POLYGON family. RoadDash currently uses independently
authored character art; it is not a purchased Synty character package.

The publisher's [Made with Synty series](https://syntystore.com/blogs/blog) names
**Tabletop Tavern** as a game made with Synty assets. Its
[Hide and Seek game-jam showcase](https://syntystore.com/blogs/blog/showcase-hide-and-seek-birthday-game-jam-winners)
identifies **But where is he?** and highlights the busy NPC mall setting. These
are verified leads for studying coherent stylized characters and world activity.
They do not establish which exact character pack, locomotion algorithm or source
code a game uses. The exact game in the user's earlier screenshots remains
unidentified. No unsupported claim that a particular game has terrain IK is made.

V2-02-003 requires actual locomotion footage/developer evidence: inspect additional
matching games and compare starts/stops, grounded feet, slope gait, upper-body
weight, interaction reach and crowd motion. Record observations separately from
inferences about implementation. Games are visual/behavior references, not code
or raw asset donors.

## Pinned repository evidence

- [sergioabreu-g/active-ragdolls](https://github.com/sergioabreu-g/active-ragdolls/tree/459cbab9489c7481e65bf16ac93fcfff98e6a594) — `459cbab9489c7481e65bf16ac93fcfff98e6a594`; Apache-2.0; last push observed 2021-02-02T14:51:32Z; archived=False. [License evidence](https://github.com/sergioabreu-g/active-ragdolls/blob/459cbab9489c7481e65bf16ac93fcfff98e6a594/LICENSE).
- [hairibar/Hairibar.Ragdoll](https://github.com/hairibar/Hairibar.Ragdoll/tree/ba7488ad7abe86dc94ae10c5e8e25dc9c1688366) — `ba7488ad7abe86dc94ae10c5e8e25dc9c1688366`; MIT; last push observed 2021-04-21T07:19:57Z; archived=False. [License evidence](https://github.com/hairibar/Hairibar.Ragdoll/blob/ba7488ad7abe86dc94ae10c5e8e25dc9c1688366/LICENSE.md).
- [jrouwe/JoltPhysics](https://github.com/jrouwe/JoltPhysics/tree/ea4599d5044d92ba91b18664ec1df96255d5eb31) — `ea4599d5044d92ba91b18664ec1df96255d5eb31`; MIT; last push observed 2026-10-31T20:58:23Z; archived=False. [License evidence](https://github.com/jrouwe/JoltPhysics/blob/ea4599d5044d92ba91b18664ec1df96255d5eb31/LICENSE).
- [Kurble/bevy_mod_inverse_kinematics](https://github.com/Kurble/bevy_mod_inverse_kinematics/tree/5c037a33b55e8056ec24222d2968972f59e009d5) — `5c037a33b55e8056ec24222d2968972f59e009d5`; Declared MIT OR Apache-2.0 in README/Cargo; root grant files not located; last push observed 2026-09-01T11:26:27Z; archived=False. 
- [guillaumeblanc/ozz-animation](https://github.com/guillaumeblanc/ozz-animation/tree/744eb9d99f606eda849acb0b1204f7a3dc20bca1) — `744eb9d99f606eda849acb0b1204f7a3dc20bca1`; MIT (grant manually read; API label NOASSERTION); last push observed 2026-08-01T21:34:54Z; archived=False. [License evidence](https://github.com/guillaumeblanc/ozz-animation/blob/744eb9d99f606eda849acb0b1204f7a3dc20bca1/LICENSE.md).
