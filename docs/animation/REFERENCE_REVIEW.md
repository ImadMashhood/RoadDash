# Original animation reference review — 2026-10-31

> **Semantic labels superseded (2026-11-01):** the old helper anchors were inspection-based approximations. Use the [context-first catalog and corrected action sequences](context/README.md). V1 files below are historical drafts, not motion-reference authority.

Local research build: user-supplied 1997 rerelease disc. HIBOB SHA-256
`a5699b34f5371ecd4659bb137feeaa3580dfdd1a9b620611cdefb2f7c4837f2d`;
LOCOP `2fafb8eac3cd26074c41f8bc2dd03a75f1e01fcf0aeee572d7466c984a2fcd49`.
No disc pixels, palettes, executables, sounds or source sheets are included.
The independent numeric reader is tools/measure_reference_animation.py;
classification is tools/classify_animation_reference.py. Local inspection sheets
remain outside the repository. Unused slots are retained, not renumbered.

HIBOB has 326 slots: 186 populated, 140 empty. LOCOP has 45 slots: 33 populated,
12 empty. All slot dimensions, offsets, hashes, duplicate relationships and
confidence-rated families are recorded in reference-slots.json / police-slots.json.
Exact duplicate pixels are not proof of sequence aliasing; low-resolution or
view-dependent images are not extra motion samples. The original 130 numbered
mounted poses are now preserved as draft candidates, with ambiguous labels still
requiring art/reference review. Three inherited mount poses over-flexed the right
knee; the authoring solver refits that released leg within the 2.45 rad flex bound.

| Observed slots | Visual family / use | Confidence and caution |
| --- | --- | --- |
| 0–21, 22–23 | Riding banks/view differences; small distance variants | Visual family; helper confirms neutral 3, strong right 4, strong left 11. Do not interpolate view/depth duplicates as time. |
| 41–59 | Raised arms and fists | Helper anchors right 44→45 and left 51→52 windup/strike; other meanings/phase order remain tentative. |
| 60–70 | Side kicks | Helper strike anchors right 60, left 66; order/timing needs descriptor linkage. |
| 81–89, 100–117 | Chain gestures | Helper supports right 101→102, left 105→106. Several alternate views are visible. |
| 140–143, 155–162, 176–179 | Club gestures | Helper supports left 140→142, right 141→143. Broader views are visual observations. |
| 191–198, 213–220 | Additional unarmed arm gestures/views | Do not assign exact attack phases solely from numbered order. |
| 225–232 | Low reach/contact reactions | Family observed; exact native state binding is unresolved. |
| 241–248, 252–255 | Ground crash and mounted loss of balance | Distinguish the shared riding assembly from detached body targets. |
| 256–276, 282 | Airborne/rolling/sliding body silhouettes | Used as original-pose reference; not joint matrices, impulses or a recovered 3D trajectory. |
| 283–286, 289 | Support/crouch/get-up silhouettes | Front/back support choices need modern terrain/contact review. |
| 291–296, 297–302, 303–308 | Running, front/rear/side view groups | Six phase candidates per group; view ordering is observed, recovered timing is not claimed. |
| 309–313 | Beside-bike/leg-over/mount silhouettes | Preserve original IDs; clearance and exact native recovery mapping remain open. |
| 319–322 | Separate fallen bike | These are bike-only references, not human animation frames. |
| LOCOP populated slots | Police banks, club poses and detached tumble | Indexed separately; new police gameplay is deferred. |

Source research remains arkits/road-rash commit
`cd5504caea48aee59806dbfd11c920f33df21200`, REFERENCE ONLY. No C++ copied.
Relevant files: race_helpers.cpp (riding/attack helper anchors), actor_animation.cpp
(playback/state selection), actor_descriptors.cpp (view/descriptor distinction),
crash.cpp (recovery transitions). The source uses a separate descriptor bank:
riding has multiple speed/view choices; crash bike states select different
animations from detached rider tumble/get-up/recovery. A sprite slot is therefore
not an animation-state number or duration. CANS/RRAN/EXE sequence linkage and
actual native timing extraction are still required for full reference fidelity.

Reproduce metadata with a local file, never a committed disc resource:

```
python3 tools/measure_reference_animation.py /local/HIBOB.DAT --output docs/animation/reference-slots.json
python3 tools/measure_reference_animation.py /local/LOCOP.DAT --slots 45 --output docs/animation/police-slots.json
python3 tools/classify_animation_reference.py
```

The authored library never loads proprietary input. Animation A can be reviewed
from our own meshes and static JSON without an installed original game.
