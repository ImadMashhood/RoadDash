# Pass07 handoff

Only RoadDash-polish / codex/gameplay-graphics-polish. Previous commit
3bdd9fb668718501cec6fd5e32cceb0dc7c77cf7. Parent reviews/commits; no commit made.

Original offline named material policy coordinates24 placed families:
masonry94% source factor, subtly neutral concrete, dielectric painted metal,
restrained gold, slightly lighter opaque glass/interiors. Existing pale entries
and geometry remain.72front+72storey GLBs change material headers only; allBIN
bytes/nonmaterial JSON fingerprint-identical. Source models/blends/textures and
unplaced facade source finishes untouched. Generator consumes same policy.

Baker now consumes runtime's exact provenance profiles/quantized roughness;
383bands/15mips + Metropolis crops regenerated. Atlas schema requires profile
input digest. Metro previously only albedo/roughness.84/metallic0: now one extra
linear packed Groughness/Bmetallic sample with identical UVs/gradients/mips and
wrapped gutters.6,285,312added pixel bytes, no extra mesh/draw/light groups.
Original near/far normals/parallax approximations remain. All worlds/props/roads,
collision/envelopes/quotas, physics/rigs/controls/saves/audio/race untouched.

7authoring groups pass, including24family source policy/144geometry fingerprints,
34actual roughness maps,128design/384kitGLB verification,383bands/15mips hashes,
all24×3×5Metro albedo+packed crops and BUSS4upper fallback. Rust adds runtime
roughness-fixture and actual packed-format/mip/wrapped-gutter coverage tests.

Final fresh-release Metro survey and native reverse/recovery pass exit0, no
shader/pipeline errors. Reverse−1.898m/s, two pickup errors0.0000m. Native census
identical. Before19.925avg/39.788p95 → after20.062avg/41.238p95ms, observed increase
not causalGPU claim; both miss16.7ms. Single-run wall timing limits retained.

Legacy LOD passes on identical final legacy material/atlas/shaders: Valley53
crossfades/12cache/132panels/150434triangles; City387crossfades/48cache/8transitions.
Later code change only adds Metro packed material, separately validated final.
Intermediate albedo-only survey retained. Intermediate recovery failed because
old binary loaded newly edited Metro binding102shader; final rebuild/frozen
artifact runs resolve it. Never edit shaders/assets while native binary runs.

Inspected final metro09 matched street,metro05frontage,06remount and legacy
transition captures. Dither/parallax/production art/ground-floor signage, human,
long-streaming, hardware and GPU/performance gates remain open. No external code
or media imported; local source+installedBevy research. No newIDs/moves/closures;
MAT-03 evidence extended, README+PROGRESS synced. ADR111/history/issues/provenance.

Stop07. Next08 owns district coherence; preserve these source material/atlas
rules when adding families. After any facade source change regenerate far atlas
and both Metro crop maps with `tools/bake_metropolis.py --facades-only` (or normal
full bake if geometry also changes). Use existing layout venv and Blender path
in README. Preserve exact source geometry/approved near identity and readyLOD.

Proposed commit: `feat(render): synchronize authored facade finishes across distance`.

Final mandatory: fmt/strict locked workspace all-target Clippy/426tests (one
pre-existing ignored)/locked debug build/audit/5roadmap tests/progress all exit0;
locked release and7authoring groups also pass. Source differential180tick/body/
joint and crash/run/remount checks pass. No broad native paired rig labs rerun
for this presentation-only pass. All final shader logs clean. git diff --check
passes. Git status:175tracked modifications plus6untracked top-level entries
(including evidence directory and new packed atlas/tools); no commits/push/merge.
