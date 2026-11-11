# Issue 25 research and bounds — 2026-11-10

Primary installed sources examined: Bevy0.17.3 render diagnostics and lightmaps,
bevy_rapier3d0.32.0 collider synchronization; existing RoadDash terrain generators,
material/tangent intake and ADR089/134 performance evidence. No external code,
media, physics replacement or asset-license clearance is implied.

The installed Bevy skill collection identifies MIT and assumes0.20; the installed
game-development performance skill package identifies Apache-2.0. Only its
measurement discipline is applied to the pinned engine. Actual local source,
Cargo.lock and compilation determine supported APIs.

Registry APIs inspected:
- https://crates.io/api/v1/crates/bevy/0.20.1: published2026-10-28, Rust1.97.1,
  MIT OR Apache-2.0.
- https://crates.io/api/v1/crates/bevy_rapier3d/0.37.0/dependencies: Bevy^0.20.

The parent coordinates the user-requested migration after current branches;
this pass preserves0.17.3/0.32.0 as its measured control. Three engine migrations
include shader/scene/resource changes and require physical parity/recovery tests.

## Lighting candidate

Bevy's pinned [lightmap example](https://github.com/bevyengine/bevy/blob/v0.17.3/examples/3d/lightmaps.rs)
attaches baked textures to specific meshes and supports forward/deferred use.
The installed Lightmap API requires a secondary UV layout. RoadDash's generated
chunks currently use material UVs plus source-band facade identities; a proper
bake needs separate nonoverlapping lightmap UVs, source/lighting hashes, gutters,
LOD correspondence, packaging and invalidation. A global material override would
violate MAT-03 and is not a valid shortcut.

Current shadow GPU measurements are now captured separately from CPU physics.
For the initial fixed-camera traversal, medians of independently sampled shadow
passes sum to about1.00ms Hills and2.07ms Metropolis; these sums of medians are
context, not a timestamp for a whole frame. Main opaque medians are0.95/2.43ms,
while CPU physics averages7.96/10.86ms per fixed step. This supports prioritizing
CPU construction/runtime investigation on this Ryzen3600/RTX3080; it does not
establish lightmaps as worthless on other hardware.

A hybrid candidate would bake static indirect illumination/AO and retain dynamic
sun/contact shadows for racers, traffic, pedestrians and destruction. Full static
direct-shadow baking would need care for moving blockers and damaged props.
An illustrative1024²RGBA8 atlas with all mips occupies about5.33MiB uncompressed;
100such pages add about533MiB before renderer overhead. These are dimensional
estimates, not measured RoadDash lightmap allocation, file sizes or load times.
No production lightmap assets or visual-quality acceptance exists in this pass;
RFU-PERF-03 remains open. No baked-lighting migration is selected.

## Content budget contract

Existing ceilings remain the starting point, not available performance headroom:

| Presentation family | Current ceiling | Existing control / follow-up |
| --- | --- | --- |
| Hills volume clouds |8banks,12march steps, shared48³density bytes |ROADDASH_HILLS_CLOUDS=0 uses baked fallback |
| Hills dust |96pooled particles |No new per-impact allocations |
| Hills birds |24actors |New flock/density controls require matched measurements |
| Mounted horse/cowboys |5pairs |Keep authoritative motion/collision at60Hz |
| Metropolis grass |120000tufts /600batches |Visibility90–125m/production policy; retain ground/collision |
| Metropolis facade detail |28selected faces plus8retiring;6000meshes/1.5Mtriangles total |Overlap counts; closed envelopes stay resident |
| Directional shadows |4cascades;500m on both courses |ROADDASH_SHADOW_DISTANCE_M=150 retains control |

New clouds/birds/vegetation/props/herds must declare maximum actors, mesh/triangle
instances, texture residency, fixed/render work, activation/reset and low-detail
policy. Count controls must affect optional presentation, not erase live collision
or hazards. Growth requires matched raw-frame/GPU/load evidence; the current
16.7ms p95 gate is unmet. Coordinated content-family agents own their new controls;
RFU-PERF-05 remains open until those are implemented and measured.

## Offline caching / streaming decision

Start with exact in-memory generation reuse: no stale disk cache/schema or extra
packaged asset bytes. Memoized terrain support uses exact float-bit inputs and
releases bounded storage at scope exit. Offline baked terrain/collision is a later
candidate if measured CPU startup remains dominant; it needs source hashes,
engine/serializer versioning, validated fallback and deterministic source rebake.
Retain resident collision and near/far identity, independently of scene streaming.
Existing shared handles/instancing and LOD caps are retained. Prior experiments
in ADR134 found group masks cannot remove Rapier static contact-graph edges;
standalone breakable bodies failed debris settling and were reverted. Do not
repeat either experiment as an assumed optimization.
