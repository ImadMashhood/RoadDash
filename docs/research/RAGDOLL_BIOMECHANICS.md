# Ragdoll source investigation — 2026-11-01

All repositories below were cloned and actual source inspected outside RoadDash.
No source or assets were copied. These are REFERENCE ONLY investigations; a
public repository is not an asset license. Pinned versions make the observations
reproducible. Last-commit dates are observations, not a promise of maintenance.

| Repository / inspected commit | License checked | Source inspected / decision |
| --- | --- | --- |
| [bhubbard/active-ragdoll-rs](https://github.com/bhubbard/active-ragdoll-rs/tree/a2fc2f1ad8c08590c4571d5b731ac478f1a29e78) | [MIT](https://github.com/bhubbard/active-ragdoll-rs/blob/a2fc2f1ad8c08590c4571d5b731ac478f1a29e78/LICENSE-MIT) OR [Apache-2.0](https://github.com/bhubbard/active-ragdoll-rs/blob/a2fc2f1ad8c08590c4571d5b731ac478f1a29e78/LICENSE-APACHE) | Cargo.toml, src/pd.rs, src/rapier/adapter.rs. Version 0.1.1; Rapier 0.36 / glam 0.33, incompatible with the installed Rapier 0.31 / Bevy math types without an adapter/migration. Study, no dependency adoption. |
| [ashleve/ActiveRagdoll](https://github.com/ashleve/ActiveRagdoll/tree/749cd80dbccd73dffb7bb1d3827ad0e719b84f1c) | [MIT](https://github.com/ashleve/ActiveRagdoll/blob/749cd80dbccd73dffb7bb1d3827ad0e719b84f1c/LICENSE), copyright Lucas Z. | AnimationFollowing.cs, SlaveController.cs; Unity 2020.1.3f1. Study muscle state separation and clamped drives; not a Rust dependency. |
| [sergioabreu-g/active-ragdolls](https://github.com/sergioabreu-g/active-ragdolls/tree/459cbab9489c7481e65bf16ac93fcfff98e6a594) | [Apache-2.0](https://github.com/sergioabreu-g/active-ragdolls/blob/459cbab9489c7481e65bf16ac93fcfff98e6a594/LICENSE) | ActiveRagdoll.cs, PhysicsModule.cs, ConfigurableJointExtensions.cs, Auxiliary.cs; Unity 2020.1.9f1. Study target/physical rig separation and joint-space coordinates; not imported. |
| [AshimPoudel7/ProceduralLocomotion](https://github.com/AshimPoudel7/ProceduralLocomotion/tree/8c866b44671732faa57a6df2c6cd0a52a1bc81d1) | UNVERIFIED: no repository license file found at inspected commit; [root](https://github.com/AshimPoudel7/ProceduralLocomotion/tree/8c866b44671732faa57a6df2c6cd0a52a1bc81d1) | BalanceController.cs, PerturbationLogger.cs, PlayerPush.cs; Unity 6000.3.10f1. Behavioral study only; no reuse permission assumed. |
| [pemmyz/js_rapier_ragdoll_v1](https://github.com/pemmyz/js_rapier_ragdoll_v1/tree/7b84ece887a0fcf2644fa1949f486fdcfe7da7f0) | [MIT](https://github.com/pemmyz/js_rapier_ragdoll_v1/blob/7b84ece887a0fcf2644fa1949f486fdcfe7da7f0/LICENSE) | script.js capsule construction and limited revolute joints. Study, not a Bevy dependency. |

## Findings translated into the existing engine

The Rust primary reference separates master animation targets, physical limb
bindings and control outputs. Its inspected Rapier adapter applies COM velocity
changes and torque impulses; its optional joint argument wakes/accesses joints,
but does not configure the native spring drives. We do not transplant that COM
tracking: it would interfere with RoadDash's measured ejection momentum. Instead,
our independently written integration uses native capped ForceBased joint motors.
Its PD helper differentiates stored error; Rapier provides solver-relative angular
velocity damping directly, avoiding a second noisy finite-difference controller.

ActiveRagdoll demonstrates independent strength loss/recovery and a dead mode.
We separate active activation from passive tissue gains: zero active strength
never removes a limit or disables passive damping. No integral controller is
needed; there is no error accumulation or integral windup in our native PD drives.

The modular Unity example constructs local joint bases before converting target
rotations. Its targetRotation convention is Unity-specific and inverse-oriented;
copying that calculation would be wrong for Rapier. Inspection of the installed
Rapier 0.31 joint_constraint_builder.rs establishes its actual angular chart:
2*asin(relative quaternion imaginary components), with a consistent hemisphere.
RoadDash's motor target conversion matches that chart, not Euler angles. Wide
independent shoulder bounds previously approached the hemisphere singularity;
the new validator rejects unsafe combined ranges and defaults stay conservative.

ProceduralLocomotion applies cross-product upright torque and angular damping to
a root body and logs controlled perturbations. We use reproducible perturbation
fixtures, but deliberately do not add an upright root torque during free tumbling.
That would fight crash momentum and make unconscious bodies seek a standing pose.

The JavaScript sample uses capsules, limited elbow/knee hinges and freely
spherical shoulders/hips. The latter do not meet this task's anatomical bounds.
Its hinge signs are not portable to RoadDash's forward/local frame conventions.
We retain RoadDash's negative elbow flexion / positive knee flexion axes.

## Maintenance, assets and actual reuse

Last inspected commit timestamps: Rust primary 2026-10-01; ActiveRagdoll
2023-03-05; modular example 2021-02-02; ProceduralLocomotion 2026-08-25; JS example
2026-04-13. These samples are not compatibility or production-quality guarantees.
MIT/Apache source reuse is possible subject to notices and file-level exceptions;
none was performed. Their embedded models, animations and media were not cleared
for RoadDash and remain excluded. No new Cargo dependency or engine migration.
