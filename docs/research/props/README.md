# World prop rebuild catalog

Source inventory and authoring queue, 2026-11-01. User authorized the same pipeline
as buildings for pedestrians, cars, lamps, trees and the wider roadside set.
Branch: codex/prop-reference-catalog. No gameplay or accepted physics changed.

## Delivered

- All269 FAM resources, all54 ANIM resources from the five course CAR libraries,
  and GLOBAL CEL resources scanned.4,172 indexed CEL occurrences accounted for;
  1,310 distinct highest-mip indexed images extracted to the user's Downloads.
-597 review groups, preserving aliases, offsets, category/selector paths, stored
  frame numbers and size cells. These are view/color/size banks, not597 unique
  physical objects.105 numbered3×2 sheets and a searchable offline gallery.
-369 prop/actor rebuild queue entries after keeping existing architecture,
  terrain and global materials in separate cross-reference groups. Eleven entries
  remain unclassified; none is silently dropped or assigned a guessed identity.
-Current runtime inventory and proposed role/collision/semantic placement contracts
  in rebuild-queue.json. All dimensions remain unmeasured rather than inferred
  metres from pixels. Human/quadruped mesh replacements must preserve the accepted
  physical rig; car art replacements must preserve crash/traffic calculation state.
-First six-item AI concept sheet in art/props/concept_3d/props_01.png, using actual
  source subjects plus our building concept style. This is bitmap concept art,
  not exported3D geometry. All six source IDs are in its MANIFEST.json.

## Open locally

- /home/imad/Downloads/RoadDash_Prop_References/index.html
- /home/imad/Downloads/RoadDash_Prop_References/sheets/
- /home/imad/Downloads/RoadDash_Prop_References.zip

Retail reference pixels remain outside the checkout/runtime. The checked-in
source-inventory.json is numeric/text/hash metadata, not original image payloads.
Gallery details show every stored indexed image, not just the three review
thumbnails. Pixel0 is transparent in these reference previews; this is not an
engine material/transparency recreation. BIKERS banks, UI, skies and audio are
outside this world-prop scan; rider banks already have a separate animation catalog.

## Repeatable commands

```sh
python3 tools/catalog_prop_references.py /path/to/ROADRASH --decoder /path/to/local/city_source.py --output /path/outside/repo/Prop_References
python3 tools/package_prop_references.py /path/outside/repo/Prop_References
python3 tools/verify_prop_references.py /path/outside/repo/Prop_References
python3 tools/plan_prop_catalog.py /path/outside/repo/Prop_References
python3 tools/test_prop_references.py
```

The existing external user-local decoder is used for decompression only, with its
hash recorded. It is not copied or installed as a dependency. No network fetch
or retail image embedding is performed by these tools.

## Evidence versus interpretation

Pinned reference aa9cb6d3181e20736af71022cd4d6aac0e2788d1:
family.cpp, animation.cpp, course_scene.cpp and traffic.cpp. Category2 provides
actor graphics/CANS; categories0/4 supply scenery graphics; vehicle ANIM resources
are separate per course. Metadata retains all graphics leaves, even when not in
those categories.3521 non-CEL pixel references remain explicit (legacy/inherited
references, not decoded new images). No action/view semantic labels are guessed
from frame order. Loaded-family matches are course eligibility only; actual
placement/frequency and district assignment require further selector analysis.
Name-pattern categories and gameplay roles are provisional RoadDash interpretation.
Source name 'GOLFC00' is an index, not a production vehicle brand approval.

## Remaining pipeline

V2-05-001: review/consolidate archetypes, identify the11 unknown entries, confirm
roles and source selection/placement rather than claiming family loading proves it.
V2-05-002 / V2-06-001: extend concept/material review per family; author actual
editable geometry, rigs and GLB LODs. Current six concepts are unapproved; vehicle
badge cleanup and character silhouette consistency need review before production.
V2-05-003/004/005: semantic placement with road/sidewalk/whole-lot clearances,
independent solid collision, audio, actor recovery/nav QA, actual native captures,
performance and user art review. No new prop model is loaded by the game yet.
See ../../testing/prop-catalog/README.md for verification.

## Concept batch02

Six more source-mapped sheets/36 subjects generated with image_gen. See ../../../art/props/concept_3d/BATCH_02.html and BATCH_02.json. Including the earlier mixed sheet,39 unique reference IDs have draft concepts. User approval, complete-catalog coverage and all actual mesh/runtime gates remain open. DEER2 was visually confirmed as a crossing sign (not an animal); BIGFNCN2 is a checked bollard. Reclassification reduced the active reference-sheet count from106 to105; superseded sheets are archived locally.


## Full-catalog concept pass

The follow-on all-remaining batch preserves the existing 39 covered IDs and targets
all remaining 330 source groups in 59 sheets. `ALL_REMAINING.json` records exact
prompts, source/style SHA256, slot IDs, generation status and separate source/output
review flags. `ALL_CONCEPTS.html` is the searchable generated-art gallery; the
coverage receipt lists any missing IDs explicitly. Alternate view/color banks are
not distinct approved 3D objects. Source review corrected misleading names in the
generation prompts (cow/slippery-road/light-ahead signs, trail map, pallet stacks,
parking-service tricycle, tree/hydrant and tree/bin composites). Generic authored
advertising replaces corporate billboard logos. All production mesh, material,
rig, collider and placement gates remain open.

Tools: `prepare_remaining_prop_concepts.py` freezes the ID-complete batch before
generation; `record_prop_concept.py` serializes concurrent output receipts without
overwriting drafts; `package_all_prop_concepts.py` reconciles gallery, provenance
and portable zip. Do not regenerate batch allocations after partial queue sync.


## Actual draft geometry

See ../../../art/props/mesh_assets/README.md for the 369-ID volumetric draft catalog,
1,107 GLBs/three detail levels, 13 editable category scenes, exported-mesh renders
and isolated native viewer. Queue records distinguish draft geometry from approval:
`draft_volumetric_mesh_not_approved`; actors remain `not_rigged`, runtime status
`not_integrated`, dimensions/actual collision acceptance still open. This catalog
is a coarse shared-archetype authoring pass, not a faithful finished conversion of
the generated concepts. V2-05-002/V2-06-001 retain the remaining art/rig intake.


2026-11-01 gameplay follow-up: selected draft meshes now attach to existing traffic,
scenery and human/cow body frames. `content/props.json` remains a 369-ID addressable
catalog, not a count of placed objects. Native selected-ID receipts, source skin
hashes and integration evidence live in `docs/testing/props-in-game/`. No new
reference repository or copied source/media was used for this integration.

2026-11-08 Part06 follow-up: investigated only local build_prop_meshes.py,
original human/cow GLBs, current physical-driver interfaces and runtime intake.
No new external repository, downloaded media or copied code. Human derivative
weights/refinement now have source/output/recipe hashes and independent neutral-
surface/posed-triangle evidence in docs/testing/part06-npcs/. Original draft art
and redistribution/user acceptance remain unapproved;8 cow derivatives unchanged.
