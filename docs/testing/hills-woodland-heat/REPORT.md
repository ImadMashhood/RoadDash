# Hills woodland density and heat tuning

The rounded tree crowns include authored broadleaf trees. The separate
backdrop woodland uses six-triangle pointed silhouettes. Its batches fade
in between 305 and 365 metres from the camera to keep simplified geometry
out of close views.

The mountain verge starts with intermittent stands. The next rows fill in
progressively and the distant forest adds clustered trees, with clearings
from a shared density pattern. Farm, route, bridge, and existing placement
checks constrain the appropriate woodland layers. The new backdrop uses
the same batched mesh approach and six triangles per tree.

Ground refraction uses 20% of its previous strength. Exhaust refraction
follows engine RPM, including stationary grid revs, and reaches half the
previous maximum at the drivetrain RPM limit. The assisted and remote
fallback uses estimated RPM when drivetrain telemetry is absent.

## Verification

The initial graded layout counted 11,752 backdrop trees, up from 8,432.
The follow-up increases the distant sampling grid from 16 to 28 positions
per tile edge. The new census counted 36,126 trees in 109 batches and
216,756 triangles, 3.07 times the initial graded count. Focused tests
verified deterministic placement, ground support, route clearance, rising
density and the RPM ceiling.

The native countdown capture measured 7,444 RPM, zero wheel speed and
an exhaust setting of 0.341 on the original strength scale. The native
dunes capture measured a ground shimmer setting of 0.20.
[Capture values](captures.json).

The [road-view comparison](woodland-comparison.jpg) shows the same
foothills sightline before and after the density change. Full-route GPU
timing remains open.

Formatting, strict workspace Clippy, locked builds, roadmap tests, and the
asset audit passed. The Hills smoke test completed with the initial graded
layout. The tripled layout passed placement tests and rendered the matched
foothills shot. Workspace tests reproduced the known physical impact and
Metropolis static-chunk support failures.
