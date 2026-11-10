# Pass06 handoff

Only RoadDash-polish / codex/gameplay-graphics-polish. Previous commit
9f89e6005805c213d228a50966ad6ca30466a745. Parent reviews/commits; no commit made.

Default Metropolis had flat roads because legacy asphalt installer cannot match
its separate render/collision owners. Shared original PBR now installed directly
for Metropolis; legacy installer skips unused image allocation. World8m UVs set
before256m rebasing, actual tangents generated. Quiet metre-scale repair patches,
fine aggregate, short seals, no repeated tire stripes. Opaque0.5m faded paint and
existing1m paving slabs/seams with restrained tones/edge dirt. No new geometry,
overlays, LOD swaps, collision/friction, facade source/atlas, dependencies or
licensed media. All accepted physics/content/controls/save/audio/race unchanged.
Paving joints are texture; actual drain hardware/curb runoff art remains open.

Native final metro/recovery exit0, reverse−1.898m/s, both pickup errors0.0000m.
Before19.606avg/33.708p95 → after19.972avg/37.740p95ms. Both miss16.7ms; measured
whole-app increase retained, not claimed causal GPU cost. Exact unchanged
census incl3067chunks,2,909,113render/2,613,119collision triangles,2531static
collision batches,19068buildings. comparison.json records raw metrics.
Initial lower-base-tone dark trial rejected; only final after used for comparison.
Inspected exact farm/shaded-city/remount captures, retained representative views.
No broad human art/tiling/temporal/hardware acceptance. No compile/native overlap.
Isolated native saves. Texture payload asphalt4,194,300bytes now actually used;
small paint sets43,688bytes. No GPU memory/timestamps claimed.

424workspace tests pass (1pre-existing ignored), including model differential,
pavement support and new world-UV/tangent/opaque-mip/no-unused-allocation fixtures.
Final mandatory/legacy verification summary appended below when complete.
ADR110, P11-004 evidence extension; README/PROGRESS synced, no new IDs, task
placement/count changes or closures. Provenance hashes updated for3original
source files. Changed runtime files: src/road_detail.rs,
src/world_gen/metropolis/{geometry,streetscape}.rs. Docs: README,PROGRESS,
DECISIONS,KNOWN_ISSUES,CHANGELOG_DEV,third_party/ASSETS.csv,pass06 evidence.

Stop06. Next07 owns building materials and matching near/far rebake.
Proposed commit: `feat(render): integrate measured-scale road and paving materials`.

Mandatory final: fmt/Clippy/424tests/locked debug build/audit/5roadmap tests/
progress all exit0; locked release build exit0. checks.json has exact commands.
No broad rig native labs rerun because no rig/simulation changes. Source-level
model differential180-tick/body/joint and crash/run/remount checks passed.

Legacy Valley world survey:15sections/657roots retained;17.157ms average /
19.938ms p95 at1080p. Exit1 is the16.7ms performance gate, not a structural
failure. Source survey JSON and native log retained. Default --inspect-world
exits0:19,068buildings,36,635surfaces,two routes,12km. Representative Valley
S05 capture inspected; full temporal/legacy art acceptance remains open.

Git status captured in git-status.txt;9tracked files modified plus new pass06 evidence. No commit/push/merge; original dirty RoadRash untouched.
