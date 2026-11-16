# Hills rider contact, bridge floor, cattle and tree LODs

The latest Hills log, `performance-486933.jsonl`, recorded Rachel falling
and recovering near 1,009 m, then braking at low speed while mounted near
8,861–8,884 m. She was riding again at 8.7 m/s near station 8,910 m at the
last sample. The log records rider state and plan but cannot identify the
car or collider that caused an individual stop.

An escape search with no route now returns control to the ordinary traffic
planner. Real escape routes still check swept clearance. Hills riders
stalled beyond the road edge seek paved re-entry across the course.

An actual mounted-bike contact ejects a descending upper rider when
vertical separation and pre-solver relative descent indicate a landing.
The normal crash and remount pipeline handles the ejection. A confirmed kick adds 25% more sideways velocity, changing the base
push from 8 to 10 m/s before the existing AI strength multiplier and
physics response. Kick hit range and timing retain their existing values.

Both cattle crossings keep 20 solid cows. Their travel extends 48 m,
with stronger seeded per-cow offsets and different diagonal staggers
within each band. The layout tests sample the full crossing and verify
usable gaps and support through synchronized turns.

The bridge deck asphalt uses one continuous render and collision ribbon.
The structural shell closes below the asphalt across the span and both
abutments. Existing support beams, open edges and the canyon remain.

The woodland uses a six-triangle far tier behind the near and middle
trees. Backdrop meshes use that lighter silhouette as well. A denser
placement grid produced 8,432 grounded backdrop trees across 108 batches
at 50,592 triangles. Near woodland colliders remain independent of LOD.

## Verification

A dynamic bike dropped vertically onto another upright bike in Rapier.
The descending rider fell and the lower rider stayed mounted. Separate
rule tests reject side and rising contacts. The kick fixtures confirmed
a 10 m/s sideways push and an out-of-range miss, including buffered
contact timing at multiple render rates.

The bridge support regression sampled 2,425 rays across the roadway, old
chunk joins and both approaches. Native deck and aerial captures showed
the bridge floor and open sides. The revision survey’s deliberate
off-deck fall reached the river and logged the existing terminal Wasted
state. The survey’s remount-based finish condition is incompatible with
that outcome, so the run was stopped after the captures.

No matched full-field replay established Rachel’s exact blocker or a
queue reduction. The new LOD’s placed count and triangle budget were
checked. End-to-end frame timing remains open.

The Hills native smoke passed two remounts with zero exceptional rescues.
Strict Clippy, formatting, the asset and roadmap audits, and locked debug
and release builds passed. The workspace suite retains the two existing
Rapier failures for wall-impact bounds and Metropolis static-chunk
contact/CCD parity.
