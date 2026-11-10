# Grid, rider planning and road-edge correction — 2026-11-10

ADR139 follows the reported rear-grid obstruction in Hills and stopped/non-passing
Metropolis riders plus the green road-edge strip. Existing HILLS-01,
AI-QUEUE-02 and CITY-CIVIC-01 cover these corrections; no new requirements or
checklist closures. README remains the live checklist.

## Changes and reproduction

- Hills previously extrapolated its starting grid straight backward while the
  imported street curved. The actual player slot at -176 m failed road coverage.
  Import the source approach through -300 m, sample negative stations from that
  path, extend the city slice and let its surface own the apron. Grid distances,
  timing line and spacing stay intact. `src/track.rs`, track Hills/Metropolis
  adapters, Hills city-slice and geometry integration.
- Metropolis AI compared future road envelopes measured from different street
  centres. Twenty-three sampled stations had false empty corridors, including
  2940 m, immediately ahead of the screenshot's approximately 2920 m position.
  The runtime planner now uses street-centred bounds, obstacle coordinates and
  persistent passing memory, then converts the plan back to current track
  coordinates. Future static-clearance checks follow future street centres.
  `src/game.rs`; core avoidance rules and physical rigs retained.
- The soil ribbon began at 6.88 m while asphalt ended at 6.8 m. Extend its inner
  render edge beneath asphalt to 6.7 m, retaining the original access exclusion
  footprint and independent collision. Actual mesh stays at 19,818 triangles;
  roads, buildings and sidewalks remain protected. Metropolis landscape source
  only; no facade material/atlas, texture or asset bake changes.

## Evidence

`before.txt` reproduces the off-street Hills spawn. `after.txt` checks every slot,
both routes and each slot's next 180 m against actual road triangles and building
footprints, including bike-width/length samples and progress projection.

`rider-envelope-before.txt` records the false empty corridors. Final envelope,
production ECS stopped-leader passing and existing Dynamic slow-rider/car queue
controls are in the corresponding `rider-*-after.txt`/`rider-dynamic-control.txt`.
The new exact-location mobility/pass fixture uses Assisted; the Dynamic queue
control separately verifies physical sustained passes without health damage.

`road-edge-tests.txt` verifies actual soil coverage at the asphalt seam and
retained lane/building/sidewalk exclusions. Optional `ROADDASH_ROAD_EDGE_VIEWS=1`
adds Metropolis survey views at 2920/2940 m for the reported location.

Final checks pass: 594 workspace tests (421 game library, one binary, 57 content,
115 core), one existing ignored soak; strict format/Clippy, locked debug/release
builds, declaration audit and five roadmap tests. Logs are alongside this report.

All three native receipts pass with identical release binary SHA256
`4c9875bc767b53398bbb5e4de6f9ca198c5267953296c5d66105a632c67e65aa`:
Metropolis fourteen-view survey (including 2920/2940 m seam views), Hills west
with current anatomy, and Hills east with baseline physical rig. Both recovery
runs remounted twice with zero pickup error and exceptional rescues `[0,0]`.
No render/shader errors. Native screenshot `native-hills/screenshots/02-racing.png`
shows the rear-position rider travelling on the city street with the other
racers ahead; `native-metro/screenshots/metro-01.png` and `metro-02.png` show the
closed asphalt/soil seam. These do not replace a full human race or hardware review.

## Boundaries

No handling/ragdoll tuning, dependency changes, external media or reference code.
Original procedural sources remain registered in docs/third_party/ASSETS.csv.
No new license clearance is implied. Broad human race/art/performance review,
Windows/controller/network hardware coverage and sustained60FPS remain open.
No next major part is started. Remote publishing remains dependent on local Git
credentials; no push is claimed.
