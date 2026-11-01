# Animation A — reference and static keyframe review

This is the first stage of the user-requested animation overhaul. The user
explicitly authorized starting it while handling the previous PR merge; no CI
wait or merge is performed for them. Remaining Part 01 vault/swerve acceptance
stays open. Part 03 playback, interpolation, terrain IK and active-ragdoll control
are not implemented here. No current game controls, models or physics are changed.

Open [the interactive pose gallery](../../art/animation/gallery/index.html) locally.
Filter by use case, family or number. Click a pose for contacts/reference confidence
and optional front/rear/side views. The gallery works offline without a server.
[Overview](../../art/animation/gallery/review-overview.jpg) and thirteen numbered
sheets show our own 3D models. All 247 candidates remain **draft**, not approved.

## What exists

- **247 static poses / 27 mapped use cases**: the earlier 130 numbered mounted
  candidates, 18 original-slot running candidates, and 99 new walk/run/sprint,
  terrain, reaction, recovery, seated crash/tuck/brake and quadruped layouts.
- Each pose has a stable number/name, original slot IDs when applicable, actor/
  skeleton family, full connected bone matrices, held/released contacts, measured
  attachment/contact error, reference confidence and review status.
- The manifest defines states, entry/exit poses, semantic phases, interruption
  categories and use-case notes. Durations remain unassigned; no recovered native
  timing is fabricated. The same source ID can identify a view variant rather
  than another animation phase.
- An editable Blender library contains 247 one-frame, constant-interpolation
  Actions marked as pose assets. The canonical JSON also contains assembly/root
  transforms, which a bone-only pose asset cannot represent for the whole bike.
  Use the supplied loader when reviewing complete rider/bike contacts.
- Original 3D review renders: 247 primary-view images, 57 front/rear/side images
  across 19 selected candidates, thirteen sheets and a summary overview.
- A separate ten-bone original cow **review proxy** explores quadruped poses. It
  is not final NPC art and is not a replacement for the current game's animal.
- HIBOB's 326 and LOCOP's 45 slots have independent numeric inventories and visual
  family/confidence labels. Raw retail images stay local. See [reference review](REFERENCE_REVIEW.md).
- Source-level comparison of five pinned ragdoll/IK repositories and verified
  Synty-family game/pack leads; see [research](../research/ANIMATION_AND_RAGDOLL.md).
  No external source/assets/dependencies are incorporated.

## Rebuild and validate

Source and authoring units: metres, Blender +Z up / -Y forward / +X rider left.
Native Bevy conversion/scale remains the separate existing export workflow.

```
python3 tools/plan_animation_library.py
blender --background --threads 10 --python tools/build_animation_library.py -- --render all
blender --background --threads 10 --python tools/render_animation_views.py
python3 tools/build_animation_gallery.py
cargo run --locked -- --validate-animation art/animation/manifest.json
```

The builder imports only the repository's own rider/bike GLBs. The preserved
legacy-mounted.json is its static source for old numbered poses; generated poses
are not fed back into rebuilding themselves. New poses are specified in pose-plan.
Three old mounting knees and two new brace/brake grip reaches were corrected by
bounded analytic two-bone fitting, not by stretching limbs. The Rust validator
independently checks rigid/finite matrices, connected parent offsets, elbow/knee
flex, 5 mm held-contact error, unique IDs, reference bounds, skeleton separation,
paths and unknown fields. Physics/executable payloads are rejected.

The standalone editable package is generated under
`~/Downloads/RoadDash_Animation_Keyframes_v1/`. Open its .blend, then use
`tools/load_animation_pose.py` in Blender (set POSE_ID), or:

```
blender RoadDash_Animation_Keyframes.blend -b --python tools/load_animation_pose.py -- 1012
```

The JSON, sources, gallery and scripts are included in the downloadable package;
the large .blend is generated rather than checked into the game repository.

## Deliberate open review work

Part 02 is **not signed off**. The broader V2-02-005/006/007/008 gates remain open:

- Fingers have no articulated finger bones. Punch fists, grip wrapping and weapon
  sockets need mesh/rig refinement rather than a misleading hand transform.
- Full clothing/body/bike clearance and deformation need art review. The current
  validator proves connected rigid frames and selected endpoints, not absence of
  all mesh intersections or natural motion. Static support labels are intended
  constraints; terrain foot locking and moving supports remain Part 03.
- Some old broad mounted labels are speculative. Side/weapon/phase associations
  outside the documented anchors need descriptor/visual review. CANS/RRAN/EXE
  sequence linkage and native durations are not yet decoded into the catalog.
- Get-up/lift/vault/knockoff layouts are candidates. Dedicated arrest/restraint,
  weapon equip/drop, some get-up support choices and final animal anatomy need
  further authoring. No pose grants permission to inject a physical launch.
- Commercial Synty examples/pack documentation were verified; controlled gameplay/
  footage-based gait measurements and the user's screenshot-game identification
  remain unresolved. No unsupported claim of that game's internal IK is made.
- User pose/gallery approval remains required before interpolation. The current
  accepted passive rig and the collision fixes are preserved as test fixtures.

The catalog/validator and source comparison can be verified independently of final
art approval. README checkboxes distinguish those deliverables from the open
complete-library and approval gates; new ANIM review tasks retain the gaps.
