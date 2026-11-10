# Farm verge and placement follow-up — 2026-11-09

User review: the road/grass edge was too sudden, cows overlapped fences, and
existing original farmland props should populate the area. Branch
`codex/farm-verges-and-props`, base `12ceb81faede78c4fce604198ba275a36441f9fb`.

## Implementation

- Approximate1.5m light gravel shoulder outside the actual6.8m pavement, followed
  by a graded mown verge and irregular grass edge. Three opaque surface bands,
  shared mipmapped ground texture and actual paved/building polygon clipping.
  Tall tuft candidates begin beyond9.5m lateral distance, keeping gravel clear.
- Same16cows and unchanged animal rigs, now inside fields with a2.5m whole-body
  clearance disk plus1m margin to field rings/fences. Preserve reset/spawn owners;
  place at the field surface, not through fence rails or above the field.
- Four small groups of existing BRNGRN2 green barns, WINDVN2 windmills,
  HAYDOUB2/HAYTRIP2 hay stacks:16props total. Reuse original catalog assets and
  existing prop LOD lifecycle. Each has a fixed fitted collision owner; detail
  visibility never removes that collider. Placement excludes source access/road/
  building polygons, old barns, field holes, other occupants and fence margins.
  Grass excludes new farm occupants too.

Source layout, authored road/terrain collision, resident buildings, legacy maps,
player rigs, controls, saves, audio and race rules remain unchanged. Added16prop
colliders are explicit; do not describe total collision count as unchanged.
These are existing-model runtime placements alongside the existing native farm
scenery; no model/material rebake or source-layout bake is necessary. No facade
changes, dependency migration, downloaded assets or new reference-code intake.
Original asset/catalog provenance remains in docs/third_party/ASSETS.csv and
art/props records. ADR118 documents the choice.

## Verification and evidence

`checks.json` and logs contain exact commands/results. The new whole-footprint
regression checks cow population, field boundary clearance and separation of all
occupants. Existing actual-geometry tests check clipped shoulder winding, pavement
exclusion and untouched field collision triangles. `after/screenshots/farm-placements.json`
records actual native placement inputs and ready prop scenes; the independent
Shapely `verify_placements.py` validates those positions against source polygons.
Final native results and visual findings are appended below after execution.

Before: [prior road](../landscape-graphics/after/screenshots/metro-01.png),
[prior fence/cow close-up](../landscape-graphics/shoulder/screenshots/metro-01.png).
These are retained baseline captures from the immediate parent implementation.

## Tracking and limitations

README/PROGRESS remain220verified/245open/465IDs, no new IDs or closures. This
extends existing V2-06-003/006/007, ENV-008 and WORLDART-01; no whole-map acceptance.
User art review, full-route streaming, hardware/Windows, GPU attribution and
performance remain open. Previous p95 gates were unmet. Static catalog windmills
and existing animal meshes are reused; no new animation, wind or animal rig fix is
claimed. No push/default-branch merge or automatic next roadmap part.

## Final results and review

444workspace tests passed (one existing ignored), fmt, strict all-target clippy,
locked debug/release builds, audit, progress sync and five roadmap tests passed.
Five serial native runs passed: road survey, low shoulder survey, route0 default
recovery/reverse, route1 baseline recovery/reverse, and independent human/cow actor
smoke. The rendered survey waits for all16farm scenes to be ready. Actual road01
and low shoulder01 images were reviewed: gravel separates road/grass, cow stands
inside the field, existing green barn/windmill appear beyond its boundary.
No claim of user approval or exhaustive visual approval of every image.

Independent Shapely clearance passed on all32native placements; minimum
whole-footprint-to-field-boundary clearance4.533m, road/access separation and
pairwise occupancy checks pass. Initial audit assumptions treated later exterior
rings as holes; inspection of bake_metropolis.py showed flattened MultiPolygon
rings. The audit now reconstructs even/odd ring fill without repairing/changing
source data. That was a verification-script issue, not a claimed source-map defect.

Final census:16cows,16new fixed prop owners,48,644tufts/333batches/
243,220grass triangles and19,818clipped shoulder triangles. Source resident
buildings/chunks/terrain collision remain unchanged; total active colliders4426
includes the16new prop owners (previous4410). No collision LOD removal. Geometry
preparation378.552ms once in the road survey. Final bounded survey wall average
17.488ms/p9533.464ms;16.7ms gate remains unmet. This is not a GPU/full-route test,
or proof of a speedup over the previous pass. See native-results.json and
`after/screenshots/metro-performance.json` for precise receipts.

[After road](after/screenshots/metro-01.png) ·
[After shoulder/cow](shoulder/screenshots/metro-01.png) ·
[Native placement audit](placement-verification.json) ·
[Reused prop IDs/source hashes](reused-props.json) · [Changed files](changed-files.txt).

No new external research or code/media import: inspected existing original prop
catalog, authoring ring format, loader and NPC ownership. Exact retained input
hashes are in preserved-inputs.json. Runtime edits are farms.rs, landscape.rs,
metropolis mod.rs and survey.rs; documentation/provenance updated alongside them.
Reviewed and committed on the focused branch; no push/default merge. Original
checkout remains separate. Next acceptance is user review of the verge/props;
full-map artwork, performance and hardware gates stay open.
