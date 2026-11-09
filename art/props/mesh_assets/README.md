# Actual prop geometry — draft authoring catalog

369 source IDs have volumetric mesh drafts, three geometric LODs each (1,107
self-contained GLBs), 13 editable Blender category scenes, independent authoring
bounds and 66 numbered proof sheets rendered from re-imported exported GLBs.
Open `index.html` for source-ID search and individual GLB download links.

**This is a rough procedural geometry pass, not a faithful finished conversion of
the AI concepts.** Shared archetypes establish volumes and semantic parts; many
silhouettes, actor proportions, material details and sign graphics need substantial
refinement. Do not treat source-bank count as unique finished models or art approval.
Human/animal meshes are static and unrigged; they are not ready to replace the
accepted animated/ragdoll visuals. These drafts are not installed into the race.

## Inspect the actual models

```sh
cargo run --locked --bin prop_preview
cargo run --locked --bin prop_preview -- --id TRKPGEF2
```

Left/right arrows select an asset; 1/2/3 select LOD; Q/E orbit; +/- zoom.
`--capture /absolute/output.png` saves a native screenshot and exits. The viewer
uses Bevy's glTF importer and explicitly registered scene types. It installs no
race, physics or save systems. Native smoke was checked for a fire engine, cow
and human; all 1,107 GLBs receive independent binary geometry/material validation.

- `models/`: GLB, Y up, front +Z, metre-scale authored dimensions. Mesh parts stay
  separate and named for further editing; no full-image planes, projected sprites
  or embedded concept sheets. Only original 128×128 surface material tiles.
- `sources/`: 13 Blender4.5.1 category scenes, one collection per ID. Most source
  collections are hidden initially to prevent overlap; unhide one ID at a time in
  the Outliner. Semantic mesh parts, UVs, materials and bevels remain editable.
- `designs.json`: stable IDs, explicit semantic recipe, seed, concept file/slot/hash.
- `MANIFEST.json`: three LOD hashes/stats, editable source hashes, proof renders,
  conservative authoring bounds and explicit incomplete rig/production status.
- `renders/`: real exported geometry proofs, including untextured truck proofs.

## Reproduce

```sh
python3 tools/plan_prop_meshes.py
python3 tools/run_prop_meshes.py --blender /path/to/blender --workers 2
python3 tools/run_prop_renders.py --blender /path/to/blender --workers 2
python3 tools/collect_prop_meshes.py
python3 tools/verify_prop_meshes.py
python3 tools/package_prop_meshes.py
```

The default Blender path reuses the existing local authoring installation under
`/tmp/roaddash-character-tools/`; no Blender binary is redistributed. A focused
rebuild accepts `--group CATEGORY`. After any changed GLB, rerender that category
and recollect before verification. Manifest completion follows successful export
and source saving, not allocation counts. Do not regenerate the concept catalog.

## What is still unfinished

- Accurate concept silhouettes and proportions, distinctive per-design features,
  detailed wear/PBR maps, topology cleanup, coherent final art and user approval.
- Original traffic pictograms/lettering and bespoke billboard illustrations:
  several current signs use generic geometric symbols; billboard graphics are
  simplified dimensional scenery emblems rather than the complete concept art.
- Human/quadruped retopology, skinning, anatomy, clothing, rig/animation testing.
- Archetype consolidation: view/size/color reference banks intentionally retain
  individual IDs, and may share geometry recipes. No unique-design-count claim.
- Collision intake: manifest boxes are conservative whole-asset authoring bounds,
  unsuitable as final tree/foliage, hollow-object or compound gameplay colliders.
- Race placement, LOD policy/performance budgets and behavior/audio verification.

No new runtime dependency, proprietary source pixels or reference source code is
incorporated. Original procedural geometry/materials use the concept art as design
reference; release/design rights remain unverified in the asset register. Accepted
rider body specs, masses, joints/motors, momentum and recovery remain unchanged.


## Playable development integration

`cargo run --release` now uses selected catalog meshes in the game through
`src/props.rs` and `content/props.json`. City lamps, trees, furniture, parked cars,
moving/cross traffic and NPC appearances are integrated. The legacy presentation
is available with `ROADDASH_LEGACY_PROPS=1`. The 369-ID catalog is not a promise
that every bank is placed in every map. See actual selected-ID/native receipts in
`docs/testing/props-in-game/`.

Regenerate visual-only human/cow bindings with `python3 tools/prepare_runtime_props.py`;
validate them with `python3 tools/verify_runtime_props.py`. The separate `runtime/`
GLBs do not replace static authoring originals or contain physics data. Original
ragdoll bodies, masses, limits and motors remain authoritative. Rough part weights,
visible seams and final art/release review remain open. A standalone package must
provide this folder as `assets/props` alongside the executable.

## Original damaged furniture drafts (2026-11-08)

`broken/` adds twelve derived low-LOD GLBs for the six Metropolis crush roles,
built from the corresponding original source GLBs by
`python3 tools/bake_broken_props.py`; `--check` verifies exact source/output/recipe
hashes. `python3 tools/verify_broken_props.py` independently validates geometry,
normals, source material/UV/image identity and fitted solid bounds. These are
additional damage derivatives, not changes to the369-ID/1107-GLB draft catalog.
No downloaded art or source texture changes. See
[Part05 evidence](../../../docs/testing/part05-props/README.md); finished damage
art and seven remaining topple roles stay open under DESTRUCTION-02.
