# Helmeted clothing and curved wind grass — 2026-11-09

Branch `codex/stylized-humans-grass`, base
`6039be4e68e4722f5fe84445265b878ac6ca826d`. User explicitly chose helmets with
stylized clothing after supplying pastoral low-poly references. ADR119.

## Implementation and limits

Player receives matte rust-red clothing, charcoal trousers, dark boots/gloves,
faceted normals and broad vertex cloth variation. Rivals keep separate colors;
helmets remain. Canonical standing-height cloth partitions fix mixed original
jacket/leg material regions, preserving original vertices, triangles and skin
influences. Original authored silhouette, source model and glove morph pathway
are retained; this is a material/faceting pass, not newly sculpted civilian
clothing or a literal copy of the reference. Human prop skins use cached matte,
flat-normal render variants. Cow render materials and all physical rigs remain.

Grass uses six narrow curved blades per tuft with darker roots, mixed green/dry
tips and original GPU wind. Roots stay fixed, tip displacement <0.078m;0.08m
expanded batch bounds cover sway. Opaque two-sided standard PBR, no grass shadow
casters or per-frame CPU mesh uploads. Existing32m batches,65–110m dither range,
0.5m near candidates and all road/fence/building/farm-occupant exclusions retained.
54,053tufts /333batches /972,954triangles. This increases geometry cost from the
parent's five single-triangle blades; it is not a performance improvement claim.
The farm corridor is bounded; full-field vegetation and new terrain silhouettes
remain open. Gravel verges, fence-clear cows and16existing farm props carry forward.

Files: `src/models{.rs,/style.rs,/tests.rs}`, `src/props.rs`,
`src/world_gen/metropolis/geometry/{landscape.rs,grass_material.rs}`,
`assets/shaders/grass_wind.wgsl` and plugin/setup/survey plumbing in `src/lib.rs`,
`src/world.rs`, `src/world_gen/metropolis/{mod.rs,geometry.rs,survey.rs}`.
No physics/input/audio/save/controls/dependency or facade-atlas changes.

## Research and provenance

[Foxtrot](https://github.com/janhohenheim/foxtrot),
[warbler_grass](https://github.com/EmiOnGit/warbler_grass) and
[Saddle Rendering Grass](https://github.com/julien-blanchon/saddle-rendering-grass)
informed research into grass batching, curved blades and wind. Pinned source
manifest/README/license receipts are in `research/`; precise revisions and
version/license findings are recorded in ../../third_party/REPOSITORIES.md.
Warbler's README lists Foxtrot usage; current Foxtrot uses a newer Bevy version.
No upstream implementation/media was copied. Original code uses the installed
Bevy0.17.3 PBR API instead of upgrading the engine for an incompatible grass crate.
User screenshots guide palette/faceting only; no proprietary pixels/models are
shipped. New originals and refreshed source hashes are in ASSETS.csv; existing
Bevy MIT/Apache notices remain. This is provenance, not blanket legal clearance.

## Evidence

`rejected-first/` preserves the initial mixed-region clothing, overly broad grass,
and old skin-section count test failure. Final corrected native captures:

- `recovery/screenshots/02-racing.png`: actual player outfit and farm road.
- `poses/screenshots/animation-part03/`: existing11pose/multi-camera fixture.
- `shoulder/screenshots/metro-01.png`: thinner curved grass and carried-forward verge.
- `shoulder/screenshots/grass-wind-{135,185}.png`: fixed-camera wind samples.

Each native folder has isolated settings/saves/logs, process inventory,
input hashes, command/exit receipt and nonblank-image checks. Native logs and
metrics are evidence of bounded scenarios only; screenshot generation is not
user approval. No GPU-stage attribution, Windows or broader hardware claim.

## Checklist and next gates

README/PROGRESS:220verified/245open/465IDs; no new task IDs, moves or closures.
Existing ART-04, V2-06-007 and WORLDART-01 stay partial. User appearance review,
newly sculpted garment silhouettes, NPC seam/art acceptance, broader vegetation,
full-route streaming and16.7ms/hardware gates remain open. No next major part,
push or main merge is authorized by this pass.

## Native results and measured cost

Seven serial native runs passed: default recovery/reverse (route0), baseline
recovery/reverse (route1), actor smoke, close actor skin survey, multi-angle pose
lab, shoulder survey and ordinary Metropolis survey.124images, no blank capture
or logged ERROR/panic.18paired headless ragdoll cases completed with finite state.
No lab speed comparison is claimed. Independent Shapely native-position audit
using `/tmp/roaddash-layout-venv/bin/python verify_placements.py` again confirms
16cows/16ready props, no paved-access or pairwise footprint intersection and
minimum4.533m clearance to field boundaries. Plain system Python lacks Shapely;
the existing layout environment supplies it. Sources/rigs/lockfile/layout equality
is recorded in preserved-inputs.json. Current tests do not grant user art approval.

Visual review: revised jacket removes the initial gray waist band in the riding
view; grass is now slender rather than broad/leafy. Standing lab uses a rival
palette, so its purple clothing is intentional. Existing NPC waist gaps remain
visible in close walking views; faceting alone does not fix those source/rig seams.
New source garment shapes and reference-like civilian silhouettes remain open.

Ordinary survey wall average18.012ms/p9535.341ms versus retained parent
17.488ms/33.464ms; both miss16.7ms. These separate bounded captures do not isolate
GPU grass cost or establish a statistically controlled regression. Shoulder
survey average18.256ms/p9534.791ms. Native grass preparation426ms (shoulder run).
Resident grass triangles increase243,220 ->972,954; batch count remains333.
Packed attribute/index payload estimate increases49,616,880 ->115,457,208bytes
(position/normal/UV/color/tangent,64bytes/vertex plus U32 indices). This excludes
allocator padding, duplicated CPU/GPU residency and engine overhead, and is not
measured VRAM. Visibility limits submitted geometry, not resident mesh memory.
Broad performance/LOD optimization remains an explicit unmet gate.

## Final verification

445workspace tests passed, one pre-existing ignored. Mandatory `cargo fmt --all
--check`, strict all-target workspace Clippy, locked workspace tests, locked debug
and release builds, asset audit, README/progress sync and five roadmap tests
passed. `checks.json` records exact command/exit/duration; release.txt records the
native binary build. Targeted model suite also passed, including exact zero
180-tick trajectory difference for both physical rigs and recovery guards.
Only the expected presentation-section count changed (20 ->28); physical
assertions were retained. Cloth partition regression preserves source positions,
joint indices and weights; native captures verify the actual integrated shader.

Decision ADR119 and provenance/checklist/history/known-issue records updated.
No task closure, new stable ID, dependency change or source model replacement.
Work is committed on the focused branch; no push/main merge. Original checkout
was clean on inspection. Next step is user review of this first material/faceting
pass; full reference-like garment geometry and performance remain open.
