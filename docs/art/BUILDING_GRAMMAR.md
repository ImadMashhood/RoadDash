# Valley architecture grammar — Part 04

2026-11-06. This is the implemented building grammar and a partial world-art
bible, not final art approval. ADR052 records the implementation decision.

## Construction contract

The canonical 247 quota buildings and 410 existing neighborhood buildings keep
their IDs, lots, level pads, roof proxies, collider owners and road setbacks.
`content/world/architecture_rules.json` maps all 19 roles to original authored
mesh families. A portable fixed-width hash of section/block/role and lot ID
selects a repeatable variant. No random startup seed changes geometry.

`tools/build_architecture_kit.py` derives ground/upper/crown scenes from the
existing 100 facade drafts and 28 structure drafts. The accepted first-sheet
fronts have taller ground floors than the remaining catalog. Cuts follow their
authored window/storefront levels. Upper scenes repeat to fill actual building
height; windows are not stretched across an entire tower. Ground/roof height and
bay width still adapt to immutable lot dimensions. These are authored exterior
fronts around solid buildings, not usable interior rooms. Surface samples remain
materials/modules, not additional building identities.

Every generated GLB has three scenes and three LOD files per design. Identical
textures are stored once; exact vertex-tuple deduplication changes storage, not
shape. The source model GLBs remain unchanged. Manifest source/output SHA256,
per-band bounds and triangle counts are independently checked against binary data.

## Visual vocabulary and district rules

| Valley area | Implemented families | Intended read | Remaining art work |
| --- | --- | --- | --- |
| Farms S01–05 | HOUSEB/RUSTIC, BARN, HAYSHED | White siding, red timber, broad low roofs and open fields | Full barn volumes and close material review |
| Rural edge S06–07 | CLSTGA, VICT | Smaller homes, porches and pitched roofs | Transition and yard variation |
| Suburb S08 | BUSS, SHOP/GAS, warehouse | Low commercial frontage and service buildings | Bespoke service sites and forecourts |
| Inner/core city S09–10 | VICT/RUNDWN, OFFC, BUSS/CHINA | Continuous streets, repeated upper floors and stronger vertical silhouettes | Party-wall infill skins, signage and dedicated parking profiles |
| City edge S11 | Mixed commercial/urban role pools | Existing shared grid and lot transitions | Art-directed district transitions |
| Hills S12–15 | RUSTIC/HOUSE/SHOP/BARN | Lower isolated structures and existing rolling terrain | Terrain/material coherence |
| Coast, cliff, woodland | No new implementation in this part | Future art-bible identities | Define palettes, shapes, weathering and reference views before acceptance |

Original own-authored masonry tiles are reused for structural shells so their
palette follows the selected facade. Detailed fronts provide real storefront,
window, trim, porch/cornice geometry. Do not replace these with retail sprites or
flat concept cards. Draft fidelity and all-catalog material acceptance remain open.

## Runtime budget and physical independence

The renderer checks distance at 10 Hz and selects the nearest eligible lots.
Both pending loads and loaded scenes reserve the building/panel/triangle budgets.
Only the two camera-facing exterior faces get detailed scenes. Existing complete
structural near/far masses remain underneath and beyond the detail radius.
LOD switching has spatial hysteresis. Used coarse designs and masonry remain resident (108 current coarse file/LODs); the additional high-LOD cache is limited to48 entries. One hidden preparation group per selected lot is separately bounded to750 panels; a scene swaps only after instantiation completes. Display and preparation never draw together.
Coarse details do not cast duplicate ornament shadows; the structural shell still
casts the main building shadow. Near/mid geometry keeps detail shadows.

Default bounds are in `architecture_rules.json`: 20 buildings, 750 panels,
1,000,000 triangle instances, 170 m radius. These are ceilings, not guaranteed
population or a measured performance claim. See the native survey receipt.
No collider is imported from a scene. Visual slots share their owner's origin;
streaming cannot move a physical building or the character/bike/ragdoll.

## Rebuild and inspect

```
python3 tools/build_architecture_kit.py
python3 tools/verify_architecture_kit.py
cargo run -- --inspect-world content/world/valley_world_v1.json content/world/building_archetypes_v1.json content/tracks/valley.json --output /tmp/valley-inspection.json
cargo run -- --world-survey
```

The `--ids` compiler option replaces selected records in an existing full manifest;
run without it when changing the overall band contract. No Blender or
additional Rust dependency is needed to derive this kit.

Open gates: artist/user fidelity review, visible seams and texel density,
party-wall infill treatment, bespoke parking/service geometry, full world-art
bible, supported interiors/navigation, production streaming/pop tests, broader
hardware and racing performance. Part 05 is not started by this change.
