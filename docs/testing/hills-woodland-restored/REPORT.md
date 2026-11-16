# Hills woodland density with the original distribution

The Hills woodland uses the earlier roadside offsets, stand clearings, and
mountain forest coverage. Mountain roadside rows now offer one placement
every 6 m, up from one every 18 m, and the western outskirts use 22 m steps
in place of 65 m. The middle woodland samples 216 candidates per
side and batch across the original footprint. The distant backdrop uses
a 21 × 21 grid per tile, up from 12 × 12.

All layers apply the site's terrain and route constraints. Detailed
trees use the existing original conifer and broadleaf models. Distant
woodland batches use the six-triangle silhouette and the 305–365 m
camera-distance fade.

## Native results

| Layer | Original | Restored at higher density |
| --- | ---: | ---: |
| Roadside and outskirts | 1,110 | 3,334 |
| Middle woodland | 2,803 | 8,495 |
| Distant backdrop | 8,432 | 25,851 |
| Total | 12,345 | 37,680 |

Both native surveys completed 1,506 road support probes. The
[matched road view](woodland-comparison.jpg) shows the bend at station 2,500 m.
The capture runs use the original and current builds' sky settings.
[Survey values](captures.json).

The woodland placement tests, Hills smoke test, locked builds, formatting,
strict Clippy, roadmap tests, and asset audit passed. The workspace tests
reproduced the known biomechanical impact and Metropolis static-chunk
support failures. Full-route render performance remains open.
