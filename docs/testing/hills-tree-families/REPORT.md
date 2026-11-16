# Hills woodland tree families

The Hills woodland uses three simplified meshes for each of the ten detailed
roadside trees. Five are conifers and five are broadleaves, including rust-colored
oaks. The bake uses the authored crown and trunk meshes to fit each silhouette
and samples the original foliage and bark textures in linear color space.
A placement shares its family, position, scale, and orientation across tiers.

| Family | Near triangles | Middle triangles | Far triangles |
| --- | ---: | ---: | ---: |
| Conifers | 70 | 38 | 18 |
| Broadleaves | 80 | 48 | 22 |

The runtime puts each tier into the existing spatial batches and uses the
roadside catalog dimensions. Trunk collision is managed independently.
Run `python3 tools/bake_hills_tree_lods.py --check` to check the committed
templates against their source models. Full-route frame timing on target
hardware remains part of HILLS-TREE-LOD-02.

[Tree family gallery](tree-families.jpg) and [native aerial comparison](woodland-comparison.jpg).

## Verification

The native Hills survey placed 37,680 trees and completed 1,506 road support
probes and 11 captures. The backdrop placement test counted 534,622 triangles
across the full backdrop inventory of 25,851 trees and 109 batches. That
inventory is a geometry count. The survey camera sees a subset of the batches.
[Native census](census.json).

The three focused woodland tests, template reproducibility check, locked debug
and release builds, formatting, strict Clippy, roadmap tests, and asset audit
passed.
The native Hills render smoke test passed through two remounts and the results
screen. The workspace suite reproduced the existing biomechanical impact and
Metropolis static-chunk support failures.
