# Pass10 handoff

Work only RoadDash-polish / codex/gameplay-graphics-polish. Previous commit
0a23465c406eeb8096ad2341a6de0efd165838fa. Parent reviews/integrates; no commit made.

Non-HDR visible sky now has its own display cube, shared exposure/Reinhard horizon
and proper Z-reflected world solar direction. Broad original cloud banks/lobes,
sunward shading and faint high veil improve shape/depth; directional warmth/clouds
fade exactly to shared fog horizon. Original256radiance and16diffuse cubes/inputs,
lighting/exposure/fog remain unchanged. No HDR/custom pass/geometry/physics edits.

Remove incorrect entire sky/environment wind rotation. Clouds are STATIONARY;
independent cloud-only animation remains open. Legacy wind config is inactive and
documented. Future runtime exposure changes must rebuild display cube/fog together.
Do not call this animated-cloud or full landscape completion. SBDT fallback adapted
from Tomasz Stachowiak's installed Bevy implementation with MIT/Apache notices,
source attribution/provenance; selected/native-tested operator remains Reinhard.

Fresh release metro/recovery exit0, all images nonblank, inputs frozen, no shader/
pipeline errors. Reverse−1.898m/s; pickup0.0000m both. All14detail audits/resident
census identical. Matched00farm/03street/10aerial visually inspected: cyan horizon
boundary gone, clouds retain shading, near street contrast remains. Sun alignment
is analytic-tested, not a close-up native solar-angle measurement. No user approval.

Extra2,097,144byte display cube per CPU/GPU copy; generation120.588ms once at native
recovery startup. No extra draw/sample/per-frame upload/system. Wall18.238avg/
32.108p95→18.493avg/33.548p95ms; one pair, no causal/GPU affordability claim; both
miss16.7ms. cost.json/comparison.json/receipts retain detail. No broad gate closure.

ADR114; README+PROGRESS synced;220verified/245open remain. No newIDs, state/placement/
count changes. Full landscape/cloud motion/art/hardware/performance gates open.
No external media, dependencies, models/facades/atlas/layout/rig/control/save/audio
or race changes. Runtime removes only sky drift system. No whole-rig lab sweep
because physics unchanged; mandatory suite includes model differential fixtures.

Stop10. Proposed commit: feat(render): align procedural sky with haze and sunlight

Final mandatory fmt/strict locked alltarget Clippy/432tests(one existing ignored)/
locked build/audit/five roadmap tests/progress all exit0; fresh locked release
passes. git diff --check clean. Git10tracked modified+pass10evidence untracked.
Exact status git-status.txt; exact commands/exits checks.json. No commit/push/merge.
