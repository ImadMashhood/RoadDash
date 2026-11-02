# District/location grouping verification — 2026-11-01

Scope: organize existing models by original-game location associations and
coherent district role. Branch codex/building-district-groups. No mesh geometry,
model material, engine dependencies, gameplay, current preview placement or
canonical world generation was changed.

## Evidence and implementation

Read pinned arkits/road-rash aa9cb6d3181e20736af71022cd4d6aac0e2788d1:
course.cpp, course_scene.cpp, family.cpp, resource.cpp and resource envelope in
explode.cpp. An independent metadata scanner reads user-local FAMILIES.RSC and
five .CRS files; an existing external local Python decoder provides decompression
only. No original media pixels are extracted/written. Source and decoder hashes,
per-course section load sets and all146 named-asset memberships are recorded in
../../research/building-districts/source-associations.json.

All146 names matched resources. Course family eligibility: CITY90, HIWAY24,
MEDLY73, NAPA27, CANYN40 (overlapping counts). Types1/3 load requests count;
2/4 unload/preload requests do not independently establish eligibility. Named
CEL membership inside a loaded family is not proof of selected/visible placement,
route order, frequency or coordinates. Labels/primary district roles are authored
RoadDash art direction rather than a claim of original named neighbourhoods.

Thirteen primary sets cover every ID exactly once;438 relative model links resolve
to unchanged canonical GLBs. Per-district relative manifests, one CSV,13 full
contact sheets, an overview and five course indexes/contact sheets are generated.
The grouped index/preview ZIP overlays the complete model package without copying
meshes. Non-symlink platforms can resolve canonical JSON/CSV paths.

## Checks

- tests.txt: six focused tests for full coverage, BUSS/BUSSTOP classification,
  facade/structure/surface distinctions, measured course membership, all438 mesh
  hashes/links/relative render paths, synthetic load-vs-unload and malformed input.
- python-tests.txt: full tools test suite including roadmap synchronization.
- fmt.txt, clippy.txt, rust-tests.txt, build.txt: required locked workspace checks.
- audit.json: declaration/provenance audit, not legal/platform clearance.
- package.json: index archive count/integrity/hash, no copied GLB binaries.

No native gameplay rerun was needed: only offline indexes, relative links,
research metadata and docs changed. Contact sheets were visually inspected;
meshes and native rendering retain the prior building-catalog receipts.

DISTRICT-01 completed in Part00; V2-04-003 retains world/preview placement and
transitions. ADR043 records the evidence/interpretation split. No other task is
closed, and no next major part is started. Existing uncommitted catalog work is
preserved; no commit, push or merge performed.
