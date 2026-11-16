# Hills slopes, tree LODs and default visuals

Three infill stands cover the marked slopes around the mountain farm and bridge
approach. Candidate trees must clear the road, existing middle woodland,
fields, bridge and wildlife paths, and get the same terrain support checks as
the middle woodland.

The closest woodland tier uses the original catalog LOD2 trunk and canopy
triangles. The reduced variants sample vertices from the same tapered conifer
skirts and rounded broadleaf lobes. Conifer geometry uses 120, 80 and 28
triangles across its three tiers. Broadleaf geometry uses 272, 188 and 56. All
tiers reuse sampled foliage and bark colors from the catalog textures.
Each batch uses the union of the three mesh bounds for frustum culling and
adds its actual extent to the outer fade distance. The backdrop can stay
visible when approached from off-road.

The Hills launcher defaults to the approved preset of countdown revs, exhaust
smoke, camera shake, daylight grading, speed wind and heat shimmer, with Poly
Haven asphalt detail. The game reads the same preset when launched directly.
A bare `./play-bevy20.sh` opens Hills. `ROADDASH_TRACK=metro ./play-bevy20.sh`
opens Metropolis. Explicit comparison flags and comfort settings still apply.
The original assets and licenses are recorded in the
[graphics comparison report](../graphics-polish/REPORT.md).

## Traffic log

The reported Hills run shows Beats (slot 7) and Hack (slot 14) crawling or
stopped around station 4,360 m with zero-speed requests. At 120.4 race seconds
both were at 0.2 m/s, around 4 m apart. Beats stayed below 3 m/s from 118.3 to
127.4 seconds. Rider telemetry did not include the identity of the lead
vehicle. Rivals with a blocked low-speed plan can now use the swept escape
planner to look for a lane beside a stopped semitrailer. When there is no
candidate lane, they can back off using the existing rear clearance sweep.
In the physical test, two riders passed the stopped Hills semitrailer and
both kept full health.

## Native woodland survey

The 4,200 m aerial view shows infill on the left foreground and both slopes
above the farm. The survey recorded 1,098 new trees and 38,778 total, along
with 1,506 road support probes and 11 finished captures. The five focused
woodland tests cover grounded planting, coverage of the marked stands, source
mesh bounds and the outer distance margin. Backdrop geometry increased to
1,208,956 triangles across all 109 backdrop batches. Full-race frame timing
remains open.
[Matched native view](woodland-comparison.jpg) ·
[Tree tier gallery](tree-families.jpg) ·
[Survey census](census.json).

The native default-preset smoke test completed two remounts and reached the
results screen. The launcher argument checks covered bare Hills, an explicit
Metropolis override, and Hills smoke-test arguments.

The locked debug and release builds, strict Clippy, formatting, tree bake
verification, roadmap tests and asset audit passed. The workspace suite
reproduced the known biomechanical wall-impact and Metropolis static-chunk
support failures.
