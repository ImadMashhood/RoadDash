# Facade kit first authoring milestone — 2026-11-01

Scope: user approved the first catalog plus three procedural families before map
integration. Branch codex/procedural-facade-kit. Existing gameplay is not edited.

## Evidence and coverage

Inspected pinned arkits/road-rash aa9cb6d3181e20736af71022cd4d6aac0e2788d1:
family.cpp, course_scene.cpp::building_group, building_panels.cpp. Category 1
resolves building groups; selector high bits choose family slot and low six bits
choose family entry. Size variants belong to texture groups, not necessarily
independent facade designs. Roof/end panels and side-street fronts are separate.
No reference source was copied; root source license remains unverified.

A prior extraction exists in the user's other local workspace,
/home/imad/Documents/ChatGPT/Rust Roadrash/assets/facades/reference_index.json.
The catalog records that index's hash, 39 names, categories and resource keys.
These are inherited extraction claims, not independently verified complete disc
coverage. Original pixels stay in that workspace. Inspected WRHS1, OFFC1 and
VICT1A images locally: industrial gridded windows, office horizontal bands and
arched upper-window cues, residential masonry and framed openings. Metric depth,
floor counts and hidden sides are authored interpretations.

## Working deliverable

Run `python3 tools/build_facade_prototype.py`. It reads grammar.json and exports
12 OBJ/MTL models (four variants each), a connected frontage and a generated PNG
preview under art/buildings/prototype/generated. No network or retail file is
required. Pillow is used for the preview only; mesh generation is standard Python.
OBJ coordinates are metres with Y up; front faces -Z. Blender can import these
models directly. This milestone does not require or alter the game asset loader.

Grammar splits facades into constant-width bays and constant-height floors.
Walls are segmented around openings; glass/doors sit behind the facade plane.
Jambs, sills, transoms, mullions, floor bands and cornices are separate geometry.
Corner variants repeat the family on a second exposed side. A connected row
positions buildings by their exact widths; foundation depth/support starts at Y0.
Widths vary by bay count, not by stretching windows. Exported materials are
original solid colors, with no original textures or external PBR assets embedded.

This is a geometric draft: family-specific arches, pediments, signage, brick
materials, roof variety, rear detail, LOD, collision export, Bevy loading and art
acceptance are unfinished. The three families intentionally start with a shared
rectangular-opening vocabulary; they are not exact reproductions of the originals.
The connected row has closed boxes/shared walls, not unioned topology or interiors.
Final party-wall culling and corner overlap cleanup belong to runtime preparation.

## Verification and next work

Four authoring tests: window counts/constant dimensions/recess, corner coverage
and positive support, OBJ index/material validity, invalid dimension rejection.
Preview inspected after generation. A generated preview is not native game visual
acceptance. No performance or complete-source-catalog claim is made.

V2-04-001 is partial (39 inherited city names; full source coverage/provenance
verification pending), V2-04-002 remains open (original production textures),
V2-04-003/004 partial (working first grammar/mesh kit), V2-04-005/006/007 open
(runtime support/navigation/performance and user art review). No new checklist IDs
or completed checkboxes were introduced. Next: verify the source catalog, match
family-specific silhouettes and materials, review this kit, then export runtime
assets with independent collision/LOD and test placed city/recovery access.
