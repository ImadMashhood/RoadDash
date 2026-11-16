# Hills cloud field and race presentation

- Race instruments share a bottom-right panel with centered speed and an
  independent RPM bar, including stationary countdown revs.
- A low scene shimmer replaces the old heat sprites in the western town and
  dunes. [Instrument and shimmer evidence](../hills-shimmer-hud/REPORT.md).
- Hills now has 24 spatial cumulus banks with varied width, altitude and
  orientation across both sides of the route. The stable world positions
  follow a seeded, jittered field with eight depth bands. Different offsets
  in each band break the route-aligned line and leave gaps of open sky.

Full quality uses 24 banks and 12 density steps. Low quality uses a
distributed subset of 16 banks and 6 steps. Off uses the existing baked sky.
The banks continue to share one density texture and the existing frustum
culling and distance fade.

## Verification

The [matched screenshots](clouds-comparison.jpg) show the foothills,
western town, and dunes from fixed points in the existing moving-camera
survey. Each of the old layout, full-quality layout, and low-quality layout
wrote six 1440 × 900 screenshots on the RTX 3080 Vulkan session.
[Placement receipts](captures.json).

The cloud tests covered both sides of Hills, altitude variation, volume
count, the shared density texture, and the Metropolis layout. The sky
integration and countdown RPM tests passed. The native Hills smoke test
reached the results screen after two remounts. The fixture ran alongside
other processes, so full-route performance remains open under RFU-SKY-05.

Formatting, strict workspace Clippy, locked builds, roadmap tests, and the
asset audit passed. The full workspace run reproduced the two known
biomechanical impact and Metropolis static-chunk support failures.
