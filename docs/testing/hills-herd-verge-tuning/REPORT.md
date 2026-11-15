# Hills herd spacing, sand pace and city-verge stalls

The second race in `performance-468358.jsonl` recorded up to 51 sampled
seconds below 2 m/s within 100 m of the 8,870 m crossing and up to 45 near
the 11,250 m crossing. These totals are per rider and may include braking,
recovery, or stationary time. Diamond stopped around race time 46.5 s at
station 831.19 m, lane 8.62 m, and stayed there through the 444.9 s sample.
Her stored passing plan requested lane 8.05 m and zero target speed. The
old telemetry cannot identify a contact object or reliably distinguish a
mounted stall from a terminal recovery state.

Both 20-cow crossings retain four rows of five. The wider center spacing
and seeded per-cow offsets increase the conservative cross-row collider
gap to more than 4.35 m. Along-row collider gaps exceed 5 m. Each crossing
uses a distinct stable seed and retains shared movement phase so the
spacing survives turns, retries, and multiplayer replication. The fixed
loop still reads the ground cache.

The sand speed cap rises from 24 to 30 m/s. Drive scale rises from 0.70 to
0.82, and rolling resistance drops by about 22%. Existing grip and entry
momentum behavior remain in force. After 20 s of full throttle in the flat
fixture, the dynamic adapter reached 26.44 m/s and assisted reached 29.95 m/s.
The adapter tests also passed grass/asphalt comparisons, coasting, reversal,
and the transition back to asphalt.

The Hills opening-city escape planner chooses a paved-road target when a
rider outside the carriageway stalls. The target is evaluated through the
same swept-solid route search. The regression places a solid exclusion
around a synthetic sidewalk goal at Diamond’s recorded location. The old
goal is unreachable and the new paved goal is reachable. A replay through
the exact scene state and a full-field cattle queue comparison remain open.
Future race logs now record each rider’s recovery stage and wasted cause.

## Verification

The workspace suite recorded 769 passing tests, two ignored tests and the
two existing Rapier failures for wall impact and Metropolis static chunks.
Formatting, strict Clippy, locked debug and release builds, roadmap
synchronization and the asset audit passed.

The matched native second-crossing fixture placed one AI-controlled rider
at 11,090 m with the clock at 408 s. At the common 422.98 s cutoff, both
builds had hit a cow and remounted. The old layout rider was moving at
4.39 m/s at 11,238.27 m. The tuned one was mounted at 11,233.14 m and
0.01 m/s. Neither cleared the herd within 900 frames. The geometric gaps
are larger, while this run gives no evidence that the queues are fixed.

Extending the fixture to 1,800 frames let both runs clear the herd. The
first sample beyond 11,275 m was at race time 429.48 s with the old layout
and 433.71 s with the tuned layout. This one approach took about 4 s longer
with the wider formation. Full-field queue acceptance stays open. The
Hills native smoke passed both remounts with zero exceptional rescues.
