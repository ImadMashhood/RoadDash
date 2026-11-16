# Hills tree clearance and tire spray

The roadside, orchard, courtyard, mountain, and backdrop placements share a
spatial index that reserves the crown radius of each placed tree. Planting
rejects sites above a 0.62 local terrain grade and samples the actual Hills
terrain mesh for trunk support. Farms and bridge reservations also apply to the
backdrop. This removes intersections between neighboring stands and leaves
cliff faces open.

The authored roadside scene assets retain their catalog LODs. Orchard and
courtyard trees use the existing baked family templates, with one cached mesh
per family and tier. Backdrop tiles retain the far mesh and promote to one of
two higher tiers near the camera, with hysteresis and at most one added detail
mesh per update.

Rear-wheel spray on grass used a green tint taken from the vegetation.
Grass now emits warm soil-colored dust. Sand retains its location-dependent
color. The contact, speed, and slip gates still control emission.

The native survey captured all 11 viewpoints and confirmed 1,506 road support
probes. The census shows 2,116 roadside trees, 4,359 mountain trees, 644 infill
trees and 12,530 backdrop trees. The count decreased because the shared
clearance pass rejects tree crowns and terrain placements that overlap or sit
on steep grades. Town trees use a separate placement path and are outside
these counts. Full-route frame pacing remains open.
[Matched aerial view](woodland-comparison.jpg) and [census](census.json).
