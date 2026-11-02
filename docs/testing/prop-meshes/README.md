# Draft prop mesh export evidence — 2026-11-01

This verifies PROP-02 (export/inspection infrastructure), not finished concept art.
The generated geometry is a rough, shared-archetype pass and does not faithfully
reproduce the approved visual direction. V2-05-002 and V2-06-001 remain open.

1. **Implementation:** all 369 queue IDs have editable volumetric drafts, three
   self-contained GLB LODs each, original 128px material tiles, named semantic parts
   and conservative authoring bounds. Thirteen Blender category scenes retain
   editable geometry/materials/bevels; 66 proof sheets cover all IDs after GLB
   re-import. Three additional parked-vehicle sheets use a texture-free clay
   material. The native prop_preview binary inspects individual delivered files.
2. **Checklist:** PROP-02 newly verified in Part00; 144 verified /227 open,371 IDs.
   Actual final art, faithful silhouettes/materials, rigs, colliders and placement
   remain incomplete under V2-05-002/V2-06-001. No major part exit gate closed.
3. **Files:** art/props/mesh_assets/{designs,MANIFEST}.json, models/,sources/,
   manifests/,renders/,README.md,index.html; src/bin/prop_preview.rs; new planner,
   builder, coordinator, renderer, collector, verifier, packager and authoring tests
   under tools/*prop_meshes.py and tools/run_prop_renders.py. Existing prop queue
   planner/reference tests/concept verifier updated to represent draft geometry.
   README/progress, task placement, decisions/issues/history, test matrix, research
   notes and asset provenance updated. No preexisting file removed.
4. **Research/provenance:** reused the existing local building export/validation
   workflow and Bevy static-scene registration. No newly investigated repository,
   new dependency, copied reference source or proprietary pixels. Original recipes
   and material tiles; visual/design release review remains unverified, shipped=false.
5. **New ID:** PROP-02, originating Part05. Reason/dependencies/acceptance and
   verification in README. It verifies draft volume/LOD/material/source/inspector
   coverage only. Existing later task IDs own all unfinished production work.
6. **Tests:** cargo fmt --all --check, locked strict workspace Clippy, locked
   workspace tests (176 pass, one preexisting soak ignored), locked build pass.
   Six mesh-authoring tests, nine catalog tests and five roadmap tests pass;
   sync_progress --check and audit pass. verify_prop_meshes independently checks
   all1,107 GLBs: indexed triangle ranges, finite positions/normals/UVs, positive
   extents, material-tile size/self containment, byte hashes, strict LOD reduction,
   exact source-ID coverage, source hashes and proof-input hashes. Invalid GLB
   headers/truncation and misleading source-name exceptions are regression tested.
7. **Architecture:** ADR047 separates draft art intake from gameplay. GLB Y up,
   front+Z/metres. Original material tiles replace the initial invalid color-attribute
   export. Three edge/resolution/detail levels produce actual triangle reduction.
   Bounds metadata never overrides accepted physical bodies or collision policy.
8. **Known limitations:** simple actor anatomy/disconnected authoring parts,
   unskinned humans/quadrupeds, approximate animal coat patterns, generic pictograms
   on several signs, incomplete bespoke billboard art, shared vehicle silhouettes,
   simplified foliage, topology/intersection cleanup and material polish. No final
   art acceptance, manifold certification, measured racing performance or gameplay
   replacement. Selected proof sheets inspected, not every asset artist-approved.
9. **Next:** refine source-mapped meshes against the actual concepts, consolidate
   duplicate view banks, author/validate rigs and semantic collision/placement intake.
   Preserve accepted motorcycle/ragdoll behavior during later visual integration.
10. **Git:** codex/prop-mesh-catalog; uncommitted, no push/merge requested or done.
    Existing dirty city/collision work preserved. Only new isolated viewer code and
    authoring/docs/assets changed by this mesh-authoring turn.

## Native evidence and failure corrections

native-fire-engine.png, native-cow.png and native-human.png are actual Bevy GLB
captures on this Linux/NVIDIA desktop. The initial prop launch found missing
reflected scene component types; reusing the existing explicit registration plugin
fixed the panic. Earlier material proof showed invalid green/magenta attributes;
replaced with original embedded surface tiles, re-exported and re-rendered. Fixed
hidden reflector detail, checked bollard/metal barrier geometry and a truck cab
shell gap. Native camera/scene loading/capture checked; every keyboard interaction,
Windows and runtime race replacement are not claimed. Current race physics was
not modified; full Rust regressions retain the accepted-model differential checks.

Native logs contain existing Vulkan ICD/validation-layer and controller-mapping
warnings. Captures completed and processes exited successfully. The game itself
was not rerun for this isolated authoring viewer; no claim of race integration.

## Evidence files

- verification.json / verify-summary.txt: all model/source/render binary evidence.
- collection.txt:369 IDs,1,107GLBs,13 sources,66 color proof sheets.
- package.json: portable archive path, size, hash; zip CRC checked.
- fmt.txt / clippy.txt / rust-tests.txt / build.txt: required Rust checks.
- python-mesh-tests.txt / python-catalog-tests.txt / roadmap-tests.txt: authoring QA.
- roadmap-sync.txt / audit.txt / concept-regression.txt: tracking/provenance continuity.
- logs/: Blender group export/re-import/render outputs, including clay proofs.

Model zip: /home/imad/Downloads/RoadDash_3D_Prop_Drafts.zip. Actual rendered gallery:
../../../art/props/mesh_assets/index.html. Interactive inspector command:
`cargo run --locked --bin prop_preview`.
