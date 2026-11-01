# Context-first reconstruction — replaces V1 semantic labels

The previous gallery attached speculative meanings to sprite slots before tracing
native sequence selection. That was the wrong basis for rebuilding actions.
In particular, `race_helpers.cpp::attack_frame` explicitly describes its two-frame
choices as inspection-based approximations. They are **not authoritative original
punch/kick/weapon sequence anchors**. Old files remain as historical drafts; do not
use their names/confidence strings to approve motion or begin interpolation.

## Recovered context

The supplied retail executable and GLOBAL.RSC were read through a new independent
bounded Python metadata reader. Numeric PE layout facts are documented separately.
The pinned research reconstruction was compiled temporarily outside RoadDash and
used as a cross-check, with no C++ incorporated into this project. All **194
playback descriptors / neutral rear-view selections agree**, across 52 action
slots in each of rider/police banks. This is not a fresh native-original execution
or proof that every reconstructed gameplay caller is exact.

- Rider: 48 available sequences, 159 ordered phase descriptors.
- Police: 11 available sequences, 35 ordered phase descriptors; separate art bank.
- Every phase has `ACTION_01`, `ACTION_02`, etc., source animation code, semantic
  CANS code, source offset, logical duration, control/loop target, descriptor combat
  flags, rear sprite, mirror flag and separate camera/lean cell variants.
- All 326 rider / 45 police slots have a usage ledger. Rider slot64 is populated
  but not referenced by the recovered CANS groups. Empty/unused slots are explicit.
- Unavailable sequences retain fallbacks. Codes20/21 do not become invented steal
  clips: they resolve through fallbacks16/17. Unknown callers/aliases remain named
  as unresolved rather than assigned an invented gameplay function.

See [ordered catalog](CATALOG.md), [machine-readable catalog](catalog.json),
[raw metadata](playback.json), [cross-check receipt](validation.json).

| Action | Ordered rear slots | What was wrong before |
| --- | --- | --- |
| Punch forehand | 176 → 177 → 178 | 44/45 were labeled as punch anchors |
| Punch backhand | 179 → 178 → 178 | Repeated phases and alternate punch were lost |
| Kick | 225 → 226 | These were treated as vague contact/reaction poses |
| Club forehand | 140 → 141 → 142 | Two heuristic images replaced a three-phase action |
| Club backhand | 143 → 142 → 142 | Repeated image still has distinct phase/control |
| Chain forehand | 81 → 82 → 83 → 84 → 85 | View variants were mixed with temporal phases |
| Chain backhand | 85 → 86 → 87 → 88 → 89 | Separate action sequence was missing |
| NPC post-hit gesture | 44 → 45 | Incorrectly called a punch |
| Running | 297 → 298 → 299 → 300 → 301 → 302 | Front/rear/side cells are not eighteen time steps |

Left/right sequences can reuse the same raw pixels with different CANS mirror
flags. Even the first chain phase can share a mirrored selection between target
sides. The catalog records the actual selection rather than mirroring a whole
row by assumption. Combat flags and the engine's retained-descriptor / post-advance
frame-index hit gate are recorded separately from an inferred visual contact time.
Logical durations are reference counters (`delay_raw / 20`), not a prescribed
Bevy animation frame rate.

## New authored poses

`art/animation/context` contains **138 context-labeled human poses / 41 sequences**
and **16 bike-only layouts**, with original rider/bike models, fixed-length IK and
measured held grips. There are authoring-only fist shape keys and original
club/chain proxies. No geometry is fabricated for code43's five empty source
images. Police sequence metadata is complete for the decoded bank; an approved
police model/reconstruction is not claimed.

These are new **draft** reconstructions with the corrected meanings and phase
order, not recovered native joint matrices or approved motion. Source silhouettes
were reviewed action by action before drafting. Full body/cloth clearance,
weapon-hand fidelity, get-up/mount support, crash silhouette polish, secondary
views and human approval remain open under ANIM-06 and existing Part02 gates.
The glove keys are an authoring blockout, not finished finger rigging. Code7 and
some mount/wreck aliases still lack confirmed callers. Part03 has not started.
Runtime assets, models, physics, collisions and the accepted ragdoll are unchanged.

## Reproduce

```
python3 tools/read_animation_context.py /path/to/ROADRASH --output docs/animation/context/playback.json
python3 tools/catalog_animation_context.py
blender -b --threads 10 --python tools/build_context_poses.py -- --render all
blender -b --python tools/verify_context_poses.py
cargo run --locked -- --validate-animation art/animation/context/manifest.json
python3 tools/build_context_gallery.py
```

The reader rejects retail hashes outside the verified build. New builds require
layout research, not blind pointer reuse. The source `.blend` from Animation A is
an input; its assets are independently authored by this project. The old builder
reconstructs that source from source GLBs. The shared authoring IK solver is local
project code, unrelated to the game's physical joint solver.

The new editable file is generated at
`~/Downloads/RoadDash_Context_Review_v2/RoadDash_Context_Poses.blend`.
Open it and run `tools/load_context_pose.py` (set POSE_ID), or:

```
blender RoadDash_Context_Poses.blend -b --python tools/load_context_pose.py -- 2222
```

Bone-only pose Actions cannot restore prop meshes or whole-bike transforms;
the canonical JSON plus assembly/prop recipes and supplied loader do so.

Optional private comparison, using only local retail files:

```
python3 tools/build_context_gallery.py --retail-root /path/to/ROADRASH --output /outside/repository/ContextReview
```

The generator refuses to place original sprites inside the repository. Private
comparison strips retain native mirror selection and explicit action/phase labels.
No retail pixels/palettes/executables or research C++ are distributed in game art.
