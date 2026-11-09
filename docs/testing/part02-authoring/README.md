# Part02 — hand profiles, sockets and editable pose review

ANIM-16 development pass,2026-11-08; ADR080. All15 remaining Part02 requirements
stay unchecked. No new requirement IDs, runtime clips, physical changes, push or
merge. This pass improves the original authoring library, not Animation A approval.

The static Context_Fist compression is replaced by the runtime's validated
content/hand-contacts.json recipe: left/right grip/fist keys, the same hinges,
curls, thumb fold and calibrated Grip/Weapon sockets. Source dimensions convert
through the existing1.2 renderer scale. The wrist frame inherits the forearm at
.27 renderer metres; authoring hand rotation already carries the seated basis.
The adapter never applies seated_basis twice. Initial measurement exposed a
10.467mm source socket-to-bike gap from the legacy raw wrist-offset recipe.
The authoring IK now fits the installed calibrated socket instead. Its original
fixed-length two-link solver stays unchanged; declared held contacts over5mm are
rejected, and targets are never reached by stretching bones. Source hand/forearm
pose files and83 affected review renders are refreshed; existing runtime clips
are intentionally not re-exported in this static authoring pass. Long-edge sampling is restricted
to distal hand-weighted edges; source GLBs and the physical rig are untouched.

All138 human poses/41 sequences and16 bike layouts retain their IDs, source
slots, mirror flags and logical counters. Complete four-key assembly hand states
load without leaking a previous attack; held grips curl, punches fist, weapon
handles grip and released/resting hands open. Original club/chain proxies now
originate at the weapon socket; its +Y handle axis follows the explicitly authored
silhouette direction. These directions are draft interpretations, not recovered
native joint matrices. Weapon gameplay stays in Part07.

Open the [18 original source-hand views](index.html), the refreshed
[154-layout context gallery](../../../art/animation/context/gallery/index.html)
and the existing [33 native hand views](../hand-contacts/gallery/index.html).
Source hand views isolate open/grip/fist on both sides from palm/back/side;
they use the neutral authoring rig. Native views use the existing physical-driver
retarget. These different rigs/views do not constitute exact silhouette matching.

The editable file is
`~/Downloads/RoadDash_Context_Review_v2/RoadDash_Context_Poses.blend`;
the actual exported/reloaded original asset is adjacent as
`RoadDash_Hands_Roundtrip.glb`. They are reproducible local outputs; their hashes,
source hashes, capture hashes and measured errors are in [roundtrip.json](roundtrip.json).
Load a numbered pose with the repository's tools/load_context_pose.py; it restores
body/bike/props and all four shape values. The one-frame Actions remain static.
No authoring skeleton or exported GLB replaces the runtime physical rig. Source fitting uses the calibrated socket landmark; native retarget/frame clearance remains a separate review.

## Verification

The external Blender4.5.9 LTS (8bf95cbd38d1) already used by this project was found
under /tmp/roaddash-blender-run. This pass uses actual export/import, not a missing
PATH assumption. No Blender source, add-on, bundled artwork, external character,
retail media or repository implementation was imported. Existing tool provenance
is in docs/third_party/REPOSITORIES.md.

Reproduce from the repository root:

```sh
BLENDER=/tmp/roaddash-blender-run/blender-4.5.9-linux-x64/blender
$BLENDER -b --threads 8 --python tools/build_context_poses.py -- --render all
$BLENDER -b --python tools/verify_context_poses.py
$BLENDER -b --threads 8 --python tools/review_authoring_hands.py
python3 tools/build_context_gallery.py
python3 tools/test_animation_context.py
python3 tools/check_authoring_hands.py
python3 tools/report_animation_coverage.py --check
```

The Blender/Python numeric fold fixtures also run against Rust's actual runtime
fold, covering wrist/hinge boundaries, both thumb signs and finger tips. Position
tolerance is3e-7m; normal-direction tolerance is1e-6 (one measured cross-language
boundary difference was about4e-7). No runtime recipe was changed to satisfy it.
The GLB verifier evaluates Basis and each independent target after reload and
measures imported surface error, allowing glTF's material/UV seam splits. It also
checks four morphs, skin, four sockets, actual UV values and finite normals. Exported socket translation/rotation errors, source recipe-frame error and posed socket-to-bike errors are recorded separately.

## Results and receipts

All326workspace tests passed; one existing ten-minute soak remains ignored. This
includes src/models/tests.rs differential physical trajectories/recovery checks.
fmt, locked workspace clippy/all-targets with warnings denied, locked debug build,
asset audit, roadmap tests/progress sync, seven Python context tests, coverage and
both review provenance checks pass. Authoring CLI validates138poses/41use cases.
Compact Blender receipts and complete test/native logs are alongside this report.

| Measured quantity | Maximum error |
| --- | --- |
|138-pose FK matrix reload |6.56e-7 matrix component |
| Source socket versus configured neutral wrist frame |2.87e-7m |
| GLB socket translation reload |4.37e-7m |
| GLB socket rotation matrix reload |1.99e-7 component |
| GLB Basis and all four target surface reloads |9.91e-7m |
| Actual UV-value reload |2.99e-8 UV units |
| Calibrated held source sockets, neutral/tuck/opposite-hand punches |3.20e-7 renderer metres |
| Original chain/club proxy versus socket placement |1.60e-7 renderer metres |

These rows measure different things. Source contact fitting no longer retains
the initial10.467mm gap. Sub-micrometre export/pose errors still do not approve
thumb contours, native retarget silhouettes or body/handle clearance.

Native Metropolis route0 scripted smoke passes retry, audio initialization,
reverse−1.827m/s, farm/city crashes, both0.0000m pickup/remount errors and results.
Both the33-view hand lab and ordinary11-motion/3-view animation lab pass.
These are existing release gameplay paths with no behavioral changes in this
pass, not a full human race or a new hardware/performance certificate.

A fresh original-only review package is generated at
`~/Downloads/RoadDash_Hand_Authoring_Part02` by tools/package_authoring_hands.py.
It includes the current .blend/.glb, profile, full pose data, galleries, working
loader and SHA256 list. Prior private retail comparisons stay outside this package
and the repository. No source/reference media rights are inferred from tool use.

## Requirement disposition

| Requirement | This pass / remaining acceptance |
| --- | --- |
| V2-02-001 | Source catalog/runtime coverage preserved; unresolved callers and full use-case/contact approval remain. |
| V2-02-002 | Existing decoded source observations retained; full category/native-execution review remains. |
| V2-02-003 | Existing licensed/provenance research retained; no new pack or internal technique asserted. |
| V2-02-005 |138 draft human poses now carry complete hand states and socket-aligned weapon proxies; full pose fidelity remains. |
| V2-02-006 | Existing running/recovery/bike layouts preserved; grounded recovery, specialist and non-rider anatomy approval remains. |
| V2-02-007 | Refreshed numbered context renders plus18 hand close-ups; anatomical/art review remains. |
| V2-02-008 | No user/keyframe/catalog approval claimed. |
| ANIM-02 | Shared grip/fist/socket authoring implemented; angular finger/thumb contours and clearance still need refinement/review. |
| ANIM-03 | Phase/side/view/mirror/counter semantics preserved; unknown callers and native timing stay explicitly unresolved. |
| ANIM-04 | Recovery/support bodies unchanged; supported get-up/lift/mount, equip/drop/arrest and animal review remain. |
| ANIM-06 | Consistent hand states/socket proxies improve contact presentation; full per-action silhouette and alias approval remains. |
| ANIM-16 | Shared profile and real Blender/GLB reload implemented; exact runtime/source tessellation/normals, silhouettes and user review remain. |
| P11-003 | Hand export verifies morph/skin/UV/socket subset; full production naming/material/collision/LOD/axis contract remains. |
| P11-006 | Hash-bound authoring evidence added; full asset ownership/release/skeleton/texture/LOD validation remains. |
| ARCH-006 | Existing provenance/source-vs-original-design separation retained; complete production catalog/rights/QA contract remains. |

Blender uses conforming edge subdivision while runtime uses triangle splitting;
those topologies are deliberately not claimed identical. Morph normals are
rederived on Blender export; original hard-normal parity is not proven. Isolated
close-ups expose angular contours and potential self-overlap. Handlebar/body/
weapon clearance, final support poses, explicit art approval, broad performance,
Windows/GPU/controller coverage and human full-race review remain open. A file
count, successful reload or scripted race fixture cannot close those gates.
