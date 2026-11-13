# Course presentation polish

The 3D chase camera supplies the backdrop for the splash, main, street and
course menus. The UI root is transparent on these pages and existing gradient
shading keeps labels readable. The camera follows a periodic route with lateral
and vertical motion. Static menu scenery and anti-nausea settings freeze it.

Hills terrain uses shared edge normals and mipmapped original grit. The
shoulder shares the terrain texture and matches the color and lighting at its
outer edge. Meadow and dune palette transitions ease in and out.
The resident terrain and road collider meshes are unchanged. Hills meadow
batches are denser, Metropolis gains more farm and civic grass, and Valley
meadow blades follow resident terrain triangles outside structural reservations.
City Run uses flat urban materials and is included in the pedestrian update.

The Metropolis crossing bake yielded 16 clear paths, up from 13. The
crossing-only mode verifies the approved source hash and resident walker data.
All other manifest records are preserved. Hills reduces the prior imported
city population from 28 to 14, prioritizing authored crossings. Hills, Valley and
City Run gain additional roadside walkers through the existing paced release
system. Newly placed crossings check fixed support and capsule clearance.
Mounted Hills cowboys and crossing cattle use the road and shoulder surface
heights. Hills farm groups have five cows each and crossing herds have six.

The implementation reuses RoadDash assets and the approved Metropolis planning
source. No new third-party assets or dependencies were introduced.

## Verification

- [Native splash capture](live-menu.jpg) shows the world and display bike behind
  the entry UI. The [frontend walkthrough](frontend.json) and [course selector
  handoff](course-selector.json) completed, including retry and return to menu.
- The Hills selector card now uses the updated native tight-curves capture.
- The periodic camera and mounted shoulder support regression tests passed.
- Formatting, strict workspace Clippy, locked build, provenance audit and
  roadmap consistency passed. Core/content packages passed 228 tests with
  one ignored. The initial integrated run recorded 487 passing game tests,
  one ignored and the two known Rapier failures. The differential model and
  recovery checks passed in that run.
- [Hills native recovery](hills-recovery.json) completed both remounts with
  zero exceptional rescues. The native log recorded an audio underrun.
- The Metropolis survey completed with [16 authored crossings](metro-crossings.json).
  One crossing began during the survey window. The focused fixed-step fixture
  verified an entire grounded crossing. The native [grass and population
  counts](grass-and-population.json) record 111,779 Hills and 75,516 Metropolis
  tufts in 64 and 478 batches respectively.
- [Metropolis native recovery](metro-recovery.json) exited successfully after
  both remounts and results. The native log recorded an audio underrun.
- Valley's explicit `--inspect-world` run passed. The first `--world-survey`
  produced all 15 section captures before reaching its time limit.
- City Run exposed a Bevy mesh-lifetime panic in the tangent queue. The queue
  now uses fallible mesh access for render-only assets that have released
  their CPU data, preserving tangent generation for resident assets.
- [Final dune capture](dunes.jpg) shows the shared terrain/shoulder texture and
  the removed flat color strip. The selected Hills card uses the same build.
  The final Hills survey produced all 15 route views before its time limit.
  The earlier survey with the same route and NPC changes exited successfully.
- The [City Run LOD rerun](city-lod.json) passed with 429 direct crossfades and
  the existing 48-entry cache and 8-transition caps. Both focused tangent
  tests passed after the fix. The final terrain workspace run recorded 489
  passing game tests, one ignored and the same two known Rapier failures.
