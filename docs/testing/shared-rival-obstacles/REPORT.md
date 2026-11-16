# Shared rival obstacle discovery

The prior Hills trace recorded about 408 precise AI sphere sweeps per active
60 Hz tick across the field, with higher rates in the busy opening. Rival
forecasts now share at most one static discovery scan per fixed tick. It
checks eligible static colliders within a shared rider envelope. The field
reuses a 2 m grid with conservative vertical ranges while the collider list,
shapes, poses and covered region match. Batched meshes contribute small
hierarchy bounds so gaps between their solids remain useful.

A segment is clear only when its entire expanded bounding box is inside
the scanned volume and all its cells are empty. Occupied, uncovered and
over-budget cases use the existing exact Rapier sweeps. Moving traffic
prediction, rider collision, tracing and suspension remain separate.
Set ROADDASH_AI_SHARED_SCAN=0 to compare with the exact-only path.


The initial prototype rebuilt a coarser grid every tick. The Hills comparison
had identical racer outputs, but that build skipped only about 2.5% of casts
and increased the time spent planning. The final implementation caches the
grid and tracks heights separately to reduce that cost and improve clearance.


## Verification

A same-binary, seeded 620-frame Hills replay on an RTX 3080 under Xvfb
produced identical parsed racer output with sharing off and on. In the busy
logger windows, exact casts averaged about 410 → 367 per tick. The shared
scan ran 244 times across the 244 sampled active ticks and certified 10,308
segments clear. Sampled planning time was 1.58 → 1.67 ms per tick. Background
compilation and another game instance overlapped the run, so these numbers
establish query counts and route parity. Repeatable CPU and frame-time gains
remain open. Sixty scans per second is the shared discovery ceiling. Precise
fallback casts are additional.

Formatting, strict clippy, asset audit, roadmap consistency and locked
debug/release builds pass. Five focused regressions on the final code cover
scan lifetime, mesh gaps, pose and height changes, and physical queues.
The earlier integration candidate passed the Hills and Metropolis release
smokes. Its workspace run recorded 757 passing tests and the two pre-existing
Rapier failures in high-speed ejection and Metropolis static-collider parity.
