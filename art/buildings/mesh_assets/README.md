# Complete architectural mesh catalog

The catalog now covers **146 IDs: 100 facades, 28 standalone structures and 18
surface modules**, with three self-contained GLB LODs per ID (438 files).
The first six accepted models remain unchanged. These are actual volumes,
openings, roof assemblies and material meshes, not facade PNGs on cards.

## Playable City Run integration

The default race now uses85 City-family-eligible designs across a5.3km generated
city:977 full-depth exterior lots,53 blocks/eight zones. Own geometric front
modules in `front_modules/` assemble with shared roofs, detailed rears and corner
returns; original438 catalog GLBs remain unchanged. See
[playable-city evidence](../../../docs/testing/city-5300/README.md) and run
`cargo run --release`. Later references to1km/six-model milestones are historical.

## District/location organization

[District sets](districts/README.md) organize all146 IDs into13 primary
neighbourhood/support groups, with full contact sheets and unchanged canonical
GLB links. Five additional original-course eligibility views are based on measured
named-CEL/family load associations, rather than visual guesses. District labels
are authored interpretations; original exact placement is not reconstructed.

## Catalog files

- `catalog_designs.json`: explicit parameters and reference sheet for every ID.
- `CATALOG_MANIFEST.json`: complete reconciled coverage, LOD hashes, source and render paths.
- `catalog_manifests/`: per-sheet export evidence and editable source hashes.
- `models/`: all 438 GLBs, Y up, front +Z, ground-centred origins.
- `catalog_sources/`: 24 editable sheet Blender files; the original six remain
  in `RoadDash_Building_Models.blend` (25 source scenes total).
- `catalog_renders/`: actual individual and numbered-sheet mesh renders.
- `Catalog_Materials.blend` and `materials/`: reusable original material tiles.

Facade designs include bays, arches, Gothic/pedimented/gabled/turret roofs,
East Asian roof assemblies, gates, shutters, industrial glazing, balconies and
storefront variants. Standalone models include barns, shelters, churches,
stations, tanks and towers. Surface IDs are extruded stone/metal/timber/concrete
modules, not miscounted as buildings. Read each design's type and notes.

## Reproduce and inspect

```sh
python3 tools/plan_building_catalog.py
/path/to/blender -b --factory-startup --python-exit-code 1 --python tools/build_catalog_batch.py -- --materials-only
python3 tools/run_building_catalog.py --group all --workers 2 --resume
python3 tools/collect_building_catalog.py
python3 tools/verify_building_catalog.py
python3 tools/build_city_preview.py
cargo run --locked --bin city_preview
```

The coordinator also accepts `--group facades` or `--group structures`. It runs
bounded Blender processes and writes one completion manifest only after export,
render and source-save success. Resume checks presence; after changing authoring
code, explicitly rebuild affected sheets rather than relying on resume.

The kilometre preview uses all 100 facade IDs across 416 touching lots and 1,360
modules. 1–4: map views; 5: gallery; PageUp/PageDown: all 25 catalog pages;
WASD/right mouse/R/F/Shift: inspect. Detailed meshes load on demand with LOD
hysteresis. `--capture PATH` records five map/proof views;
`--catalog-survey --capture PATH` records all 25 actual GLB gallery pages.
Structures and surface modules stay in the separate catalog gallery.

This completes draft mesh coverage, not artist approval or production integration.
Materials are cleaner than concept art; wear, exact proportions, small trim
intersections, more side/rear variety, colliders, navigation, terrain placement,
production performance and release provenance review remain open. Existing racing
physics/ragdolls, saves and controls are untouched. No new dependency or copied
reference source is introduced. No proprietary facade pixels are embedded.

The historical first-six milestone below remains as evidence; its statements
about unmodeled IDs describe that earlier milestone, superseded by this catalog.

---

# Actual building meshes — first approved concept sheet

This package contains real standalone 3D building geometry, not the earlier image
sheets, generic window-lattice approximations or facade-image cards. The first
six approved designs are authored: BUSS1, BUSS10, BUSS2, BUSS4, BUSS5 and BUSS6.

- **models/**: 18 self-contained GLBs, three geometric detail levels per design.
  Each file has one joined mesh node, indexed material primitives and embedded
  original 512×512 material tiles. No external images or source facade PNG required.
- **RoadDash_Building_Models.blend**: editable semantic parts, bevel modifiers,
  text curves, source material nodes and packed images. Collections for the six
  buildings remain separate; only the first is visible initially to avoid overlap.
  Unhide the desired collection in Blender's Outliner to edit another design.
- **renders/**: images rendered from actual modeled geometry. Individual views,
  six-building gallery, and an independently re-imported GLB clay render with
  every image texture removed. These are not AI-generated model screenshots.
- **designs.json**, **MANIFEST.json**: editable design parameters, reference hash,
  semantic component counts, geometry/LOD statistics, output file hashes and scope.

## Architecture represented

All six have segmented masonry around real apertures, beveled brick courses,
recessed glazing, jambs/sills/sashes, dimensional storefront posts/panels/transoms,
door hardware, stepped cornices, dentils/corbels, side/rear windows, downpipes,
roof decks, parapet/coping, chimney/flashing and finned rooftop HVAC geometry.
BUSS1 has arched brick lintels; BUSS10 has the boarded/repaired black shopfront;
BUSS2 has the projecting sloped canopy and red inset sign; BUSS4 has dimensional
RADIO/MOTEL lettering and sagged fabric; BUSS5 has the framed DRUGS sign; BUSS6
has projecting faceted bay windows and pediments. These are distinct design
features built into the meshes, not images painted over a common cube.

Materials are independently authored procedural grain/brick/paint/timber tiles
baked in Blender to standard glTF base-color textures with metallic/roughness
parameters. No full facade artwork or concept render is embedded in the models.
This first mesh set is cleaner than the concept renders: detailed aging/grime,
cloth/paint damage, corner seam cleanup and final art fidelity remain open.
Neither pixel-perfect reconstruction nor production art approval is claimed.

## Files and coordinates

Blender4.5.1 LTS, source Z up, front -Y. GLB Y up, front +Z. Origin at ground
under the building centre. Nominal body width3.2m, depth6m, height9m; cornices,
steps and chimneys extend beyond that body. Exact exported bounds are in
../../../docs/testing/building-meshes/glb-validation.json.
LOD0 is approximately 108k–122k triangles with individual masonry/hardware;
LOD1 approximately 9k–20k without fine brick/paint/slat meshes; LOD2 approximately
2.6k–3.2k with simplified structural features. Material tiles retain distant
masonry after individual brick geometry is omitted.

## Reproduce / view

```sh
/path/to/blender -b --factory-startup --python tools/build_building_meshes.py
python3 tools/verify_building_mesh_assets.py
python3 tools/build_city_preview.py
cargo run --locked --bin city_preview
```

Default city_preview now loads these GLBs through Bevy's glTF scene importer,
with shared handles and distance LOD/hysteresis. It reads no facade-image cards.
1–4 select city views; 5 shows the six mesh assets; WASD moves, right mouse looks,
R/F changes height and Shift speeds movement. Existing default racing game and
accepted ragdoll are unchanged. The 1km scene reserves 416 lots and places 1,360
model modules within their frontage; those are repeated instances of six designs,
not 1,360 unique authored buildings or 100 converted families.

The previous flat-facade study can only be regenerated explicitly with
`python3 tools/build_city_preview.py --legacy-flat-study`; do not run that before
the current viewer, which expects actual-model lot IDs. Current scene.json has
geometry_mode=actual_glb_models and no facade image batches.

## Still required

The remaining **94 facade designs and 46 structure/surface references are not
modeled in this first-sheet milestone**. Preserve those requirements. Additional
family modeling, art/material polish and user acceptance, improved material maps,
watertight/manifold cleanup where appropriate, chunk/LOD performance, collider/
foot navigation intake and integration into the canonical Valley map remain open.
The preview has no physical building collisions or traffic. Detailed prototype
performance is not a production racing benchmark. These are local authoring
assets; reference-derived visual rights are not asserted as release clearance.

Verification: ../../../docs/testing/building-meshes/README.md. ADR041 records
architecture and rollback. No source/assets from third-party repositories copied.
