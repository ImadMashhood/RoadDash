# Pass11 handoff

RoadDash-polish / codex/gameplay-graphics-polish only. Previous commit
7263342d182b1f2b7bf274e74b7c0000af1475d0. Original RoadRash dirty UI untouched;
shared target cache used only for builds. Parent reviews/integrates; no commit.

Corrected render-only boot pivots0.50→0.5580462m using existing source ankle
endpoints; paired inverse binds update at load. No vertex/weight/topology,
proportion/contact targets, neutral wrist, clip, physical16/11body, muscle,
recovery/control/handling/audio/save/world/LOD/race changes. No arbitrary twist,
engine/import/assets/dependency or UI work. World_pose/locomotion gain crate
visibility solely for cross-module model tests; implementation unchanged.

Negative contacts-before.txt reproduces58.046mm pivot,34.308mm distal-flex,
69.278mm sole-landmark error (both binds ×seven modes). Corrected matched sole
error0.395µm; final adds live gait/stop, max0.603µm. Grips remain1.002µm,
held grip0.819µm, held peg0.585µm. Actual neutral vertices max0.427µm,
weights/joint IDs exact. Landmarks are not whole boot-surface clearance.
Both model180tick physics comparisons max positional difference0; full recovery
fixture passes. Tests cover bindings/actual source offsets, contacts/neutral
surface and existing connected chains/wrists/morphs/physical components.

All mandatory fmt, strict locked workspace/alltarget Clippy,434tests(one existing
ignored), locked debug build, audit, five roadmap tests/progress pass. Fresh
release build53.50s. Full native runs serialized after builds with frozen hashes,
isolated saves; all exit0 and captures nonblank/no shader errors/panics.
Before/after each38matched pose images:13modes,3angles except normal/front omitted
by pre-existing fixture. No frames pruned; historical stdout says11motions.
Initial runner nonrecursive glob incorrectly rejected nested images; fixed and
before rerun successfully while prior binary still intact. Evidence transparent.

Both native recovery rigs/routes0/1 pass reverse−1.898m/s, both pickups0.0000m,
crash/get-up/run/remount/retry/results and5audio sinks. Both human/cow actor
smokes pass, plus12image close walk/impact/fall NPC survey. All18paired lab cases
finite; anatomy15crashes240/240sleep. No broad hardware/performance/art claim.
No added runtime operations/buffers/draws; no measured GPU speedup claim.

Inspected matched walking/tuck side images: differences limited boots/shadows;
standing side exactly pixel-identical. Reviewed integrated run/remount and
NPC walk00. Civilian faceted shoulder/waist/wrist seams and hovering appearance
remain visible; NPC-SEAMS-01 stays OPEN. Coarse pilot knee/clothing joins and gait
character still need human art/motion review. Full deformation/clearance,
production distal skins, crowd/GPU/Windows/controller gates remain open.

ADR115; README ART-04+generated PROGRESS updated. No newIDs/state/moves/counts;
220verified/245open/465IDs. TASK_PLACEMENT unchanged correctly. Provenance internal;
source-parity.json includes all biomechanical modules and original model/content/
Cargo hashes. Nine tracked files modified+pass11evidence; git-status.txt exact.

Stop11. Proposed commit:
fix(animation): align boot skin pivots with authored ankle contacts

Pass12 owns punch/kick hit reactions/feedback; preserve this contact adapter,
velocity-muscle default and11body control. No automatic next-major-part work.
