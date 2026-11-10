# Pass11 — Character movement and visual continuity

2026-11-09, codex/gameplay-graphics-polish; previous7263342d182b1f2b7bf274e74b7c0000af1475d0.
Only RoadDash-polish is edited; original RoadRash dirty UI work is untouched.
Parent reviews/integrates. ADR115; no new requirement IDs or closed broad gates.

## Reproduction and correction

Both visual ankle bind pivots used0.50m from the shin origin. The original
motion.json chain has0.558046m shin-to-foot offsets. As a result, distal boot flex
rotated around a point58.046mm above its authored ankle. A0.6rad flex moved the
anatomical attachment34.308mm. Shortened skin pivots also displaced source-space
sole landmarks during animated contacts, despite the animation solver itself
meeting its endpoint constraints. Prior chain/contact tests only checked the
animation solver and missed the skin/inverse-bind boundary.

Use the actual authored ankle distance for both runtime visual binds and their
paired inverse binds. Keep vertex positions, weights, mesh topology, authored
clips, wrists, contact targets, base11frames, physical16/11body rigs and recovery
exactly as before. Current neutral wrists already inherit animated parents; no
wrist twist is added. Animation world-pose/gait helpers gain crate visibility
only so the model test can exercise the real current gait alongside authored clips.
No other runtime behavior changes.

Two new tests exercise the actual rendering adapter:121samples per mode/bind,
normal/tuck/steering blends, original run/get-up/both pickup clips, plus current
walk/run/deceleration. Sole/grip landmarks are defined independently at the real
authored endpoints and transformed through the actual skin inverse-bind matrices.
Held landmarks additionally compare against authored grip/peg targets. The
neutral-surface test deforms every original rider vertex and checks that weights
and joint IDs exactly equal the prior0.50m binding, preserving neutral proportions.
Existing extreme connected chains, neutral wrists, shared glove morphs and both
physics/recovery differential fixtures remain in the suite.

The negative reproduction is contacts-before.txt (exit101); the earlier minimal
flex failure is reproduction-before.txt. contacts-after.txt is the matched seven
mode correction; models-final.txt adds the live gait and neutral-surface checks.
Before/after contact numbers are **landmarks**, not closest-surface mesh clearance
or complete foot-ground collision measurements. Testing both binds on every clip
also includes diagnostic combinations that are not ordinarily visible gameplay.

## Native and cost scope

The before animation lab captures38images of the existing13modes across3camera
angles (the fixture omits the first normal/front image): stand, walk,
run, steering, tuck and mounted actions. The first runner found no images because
it used a nonrecursive glob; screenshots actually existed in the nested lab
folder. The corrected runner was rerun successfully before changing the release
binary. The lab's historical stdout says eleven motions; filenames cover13modes.

Reproduction uses the existing release target cache, fresh rebuild, isolated
save directories and frozen model/motion/hand/world/shader/binary hashes. No
compiler/native overlap. See run_native.py, individual receipts and screenshots.
Recovery/actor checks exercise the integrated game; pose lab alone is not a race
or physical simulation. This pass does not change actor source meshes/weights,
NPC gait or civilians' intersecting-volume seams. NPC-SEAMS-01 stays open.

There are no extra vertices, triangles, weights, textures, bones, entities, draws,
shader samples or per-frame operations. Two bind translations change at model
load; GPU skinning buffer dimensions and operation counts are identical. No
measured GPU speedup, universal affordability,15racer benchmark, final motion/art
approval or Windows/controller/hardware certification is claimed.

## Commands

```
python3 docs/testing/polish-12/pass11/run_native.py before-poses
CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo test --locked --lib models::tests -- --nocapture
python3 docs/testing/polish-12/pass11/run_checks.py
CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo build --release --locked
python3 docs/testing/polish-12/pass11/run_native.py after-poses
python3 docs/testing/polish-12/pass11/run_native.py anatomy-recovery
python3 docs/testing/polish-12/pass11/run_native.py baseline-recovery
python3 docs/testing/polish-12/pass11/run_native.py anatomy-actors
python3 docs/testing/polish-12/pass11/run_native.py baseline-actors
/home/imad/Documents/ChatGPT/RoadRash/target/release/road-dash --ragdoll-lab --output docs/testing/polish-12/pass11/lab.json
```

Before requires the previous committed executable. Final test receipts and native inspection results are recorded below. README ART-04
and PROGRESS remain synchronized:220verified/245open,465IDs. TASK_PLACEMENT is
unchanged because no requirements move. All broad ANIM-09/ART-04/RAG-05,
NPC-SEAMS-01/human/performance/hardware gates remain open. Provenance is internal
source only (PROVENANCE.md), with no assets or dependencies introduced.

## Measured contact and regression results

Matched seven-mode samples, both binds (metres converted to millimetres):

| Metric | Before | Corrected |
|---|---:|---:|
| Ankle pivot / authored endpoint |58.046341mm|0.000550mm|
| Ankle point movement under0.6rad flex |34.307722mm|0.000123mm|
| Sole landmark retarget |69.278099mm|0.000395mm|
| Held peg landmark |39.926898mm|0.000585mm|
| Grip landmark retarget |0.001002mm|0.001002mm|
| Held grip landmark |0.000819mm|0.000819mm|

Relevant per-bind examples: standing-bind get-up51.542mm and pickup34.631mm
sole error; seated-bind tuck5.248mm. Both binds are deliberately tested across
all clips, so the maximum is not an assertion that every diagnostic combination
occurs in the game. The final expanded live gait/deceleration suite has maximum
sole error0.000603mm. Source neutral-surface maximum0.000427mm across actual
rider vertices, with exact equality of all weights and joint IDs. These numbers
certify the adapter math and preserve contact anchors; they do not measure every
faceted boot triangle's signed clearance from the ground or motorcycle.

Model suite11tests passes, including exact zero positional delta over180ticks
for both baseline and anatomical physical comparisons and full recovery/cleanup.
The mandatory suite passes434tests (one existing ignored), strict locked
workspace/all-target Clippy, fmt, locked debug build, audit, five roadmap tests
and progress sync. Fresh locked release build passes. Exact commands/exits:
checks.json, release-build.txt, models-final.txt. Source parity receipt proves
unchanged meshes/binds/motion/hand contacts, physics modules and Cargo files.

Runtime cost change: zero additional per-frame operations or buffer dimensions;
only two static bind translations differ. No performance benefit or broad
frame-budget gate closure is claimed. Original0.50m source distance is retained
only in the negative-control neutral-surface test.

## Final native inspection and remaining gates

All final native commands exit0, hashes remain frozen, images are nonblank and
logs contain no shader errors or panics. Exact receipts: native-results.json and
final-summary.json. Both recovery rigs complete farm/city crashes, get-up/run,
pickup/remount, retry and results on Metropolis routes0/1; reverse−1.898m/s and
pickup0.0000m for both mounts in both modes. Five audio sinks initialize.
Both actor smokes retain the independent human16/11body and cow10body paths.
The close NPC survey captures walking/impact/falls for AVGMSM06,CTM1A00,JGW100.
All18paired lab cases remain finite; the anatomical15crash case ends with240/240
bodies asleep. Lab results are physical-regression evidence, not visual approval
or a new performance benchmark.

Before/after animation labs each contain the same38filenames. No frame was
pruned; the existing fixture omits normal/front but covers rear/side normal and
all3angles of the other12modes. Matched side walking/tuck images were inspected:
the boot and shadow differences are localized, upper-body shape is retained.
The standing side image is exactly pixel-identical; normal side differs by only
two red-channel intensity units across the image. See native-image-comparison.json.
Current coarse knee/costume joins and crouched gait character still warrant
human motion review; this pass does not call them finished. Integrated running
and remount images were also inspected, with retained bike/rider placements.

The close NPC walk00 image visibly retains faceted shoulder/waist/wrist seams
and a hovering appearance. Those original civilian surface/support issues were
not repaired by the pilot boot pivot correction. NPC-SEAMS-01 remains open;
actor lifecycle smoke is expressly insufficient to close it. Complete skin
surface/terrain/cloth/motorcycle clearance, gait/transition polish, human art,
Windows/controller, full-crowd GPU and broad performance gates remain open.
No new assets, engine, muscle choice, collision policy, world/LOD, save/control/
audio/race changes; no UI art. No whole-rig or broad gait rewrite.

Selected actual native captures:

- [Before walk](before-poses/screenshots/animation-part03/09-WALKING-2.png) / [after walk](after-poses/screenshots/animation-part03/09-WALKING-2.png)
- [Before tuck](before-poses/screenshots/animation-part03/01-FAST-TUCK-2.png) / [after tuck](after-poses/screenshots/animation-part03/01-FAST-TUCK-2.png)
- [Native running](anatomy-recovery/screenshots/05-running-to-bike.png) / [remount](anatomy-recovery/screenshots/06-remounting.png)
- [Remaining NPC source issues](npc-skin/screenshots/actor-skin-walk-00.png)

Files: src/models/articulation.rs, models/tests.rs; crate-visible test access to
existing animation world_pose/locomotion helpers; README/PROGRESS, ADR115,
KNOWN_ISSUES, CHANGELOG_DEV and this evidence directory. Source assets/physics
and Cargo hashes remain unchanged (source-parity.json). Git status:9tracked
modified files plus this new evidence directory; exact git-status.txt.
No commit/push/merge. Proposed commit:
`fix(animation): align boot skin pivots with authored ankle contacts`.
Stop pass11; parent review/integration precedes pass12 hit-reaction work.
