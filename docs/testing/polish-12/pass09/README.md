# Pass09 — Atmospheric depth

2026-11-09, `codex/gameplay-graphics-polish`; previous commit
`75ccf0dc270d1f7166b1e50b3c77cdc0a61f378f`. ADR113. Existing SKY-01 evidence
extended; no IDs, task moves, closures or count changes. Parent reviews and
commits; original RoadRash/UI work untouched.

The old linear180–1050m fog hid all scene contrast at1050m and blended toward
an unexposed fixed grey. The new built-in exponential-squared profile has a
smooth onset, retains99.44% contrast at100m and53.79% at1050m, and converges at
longer distances. Far buildings become more visible; no visibility threshold,
LOD transition, detail budget, resident envelope or collision is changed.
`falloff.json` gives exact analytic samples. This is not a human detection test.

The existing horizon colour0.68/0.76/0.82 is now authored once and shared by the
cube and fog. Fog scales its linear colour by actual sky brightness/exposure.
The warm34.7degree sun, cool fill, EV12.7 and Reinhard profile remain. Typed
rr_content checks reject non-finite/invalid authoring and bound density so every
accepted value retains over99% contrast at100m. Native adapter tests check actual
installed fog and matching exposed horizon radiance.

No volumetrics, extra textures, directional scattering, meshes, physical rules,
rig changes, controls, saves, audio, content layouts or race changes. Facade
materials and all atlas data/hashes/mips remain unchanged. Review the separate
PROVENANCE.md for inspected pinned Bevy source; no media or source-code intake.

## Horizon pipeline limitation and rejected HDR trial

The non-HDR candidate recovered distant building layers and lightened the dark
boundary, but does not completely match the visible sky horizon. Pinned Bevy's
non-HDR PBR shader applies Reinhard to fog while the skybox writes exposed colour
directly. Equal radiance therefore does not imply equal display colour.

Trialing Hdr to use one postprocess tonemap produced black frames throughout the
native survey, including00and10, despite exit0 and clean shader/pipeline logs.
`rejected-hdr` retains the captures and receipt. The core tonemap plugin is
present and Reinhard does not require LUT media; the exact failure mechanism is
unresolved. Reject the trial and retain the supported non-HDR path. No added HDR
target/full-screen pass ships. Pass10 must consider coherent non-HDR sky output
with the same analytic operator/profile, without inverse-fog colour compensation.

The final implementation only substitutes an exponential evaluation for the
existing fog clamp and shares authored colour/exposure math at installation.
No new texture, pass, mesh, volume or light allocation. Candidate-non-hdr*
receipts preserve the first evaluation. Its initial audit caught a changed
original-cloud-daylight source hash; the provenance hash is corrected before
final verification. No complete horizon-seam or user art acceptance is claimed.

## Reproduction

Run all stages serially; keep assets/shaders frozen throughout native runs.
Runner records process state and uses isolated saves. From this worktree:

```
python3 docs/testing/polish-12/pass09/run_native.py before
CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo build --release --locked
python3 docs/testing/polish-12/pass09/run_native.py after
python3 docs/testing/polish-12/pass09/run_native.py after-recovery
python3 docs/testing/polish-12/pass09/compare_native.py
python3 docs/testing/polish-12/pass09/run_checks.py
```

Before capture requires the previous executable/content. Surveys cover farm,
city entry, both branches and downtown with resident geometry assertions and
ready detail census. Recovery smoke checks reverse, crash, run, pickup and
remount. Mandatory workspace tests include the preserved model differential
physics/recovery fixtures; no new paired physics lab sweep is warranted by this
camera-only change.

## Remaining acceptance

Analytic distance haze does not implement height-dependent weather. Static
views cannot prove racing-speed readability, temporal quality, target GPU cost,
Windows/controller coverage or user art approval. Full art/hardware/performance
and landscape gates remain open. No requirement is checked just from this pass.
Cloud design and the existing rotating sun disc are reserved for pass10.

## Final native evidence and measured cost

Final fresh release survey and recovery exit0 with frozen binary/config/world/
shader hashes, no blank images, and no shader/pipeline errors or panics. Reverse
−1.898m/s and both pickup errors0.0000m. Both routes remain integrated.
Resident census is identical:19,068buildings,3,067chunks,2,909,113render triangles,
2,613,119collision triangles and2,531static collision batches. All14native views
have byte-equal detail draw audits before/after, within existing geometry budgets.
This verifies fog did not substitute for changing source geometry or LOD policy.

Bounded whole-app wall frames before18.984ms average/35.355ms p95 versus final
18.236ms average/32.766ms p95. Both miss16.7ms. This is one paired observation,
not a causal speedup or isolated GPU measurement. Whole survey timing primarily
samples the finish; retained per-window telemetry can straddle camera moves.
The rejected black HDR run is not valid performance evidence and is excluded.

Inspected matched final farm00, entry03 and downtown aerial10 against before.
Nearby racers, street traffic, lane marks and shadowed facade details remain
readable in these static views. Aerial10 shows more layers of resident buildings
through the gradual haze. Farm skyline fades toward a lighter horizon, with the
known non-HDR display mismatch still visible. No user art approval is inferred.

- [Before farm](before/screenshots/metro-00.png) / [after farm](after/screenshots/metro-00.png)
- [Before entry](before/screenshots/metro-03.png) / [after entry](after/screenshots/metro-03.png)
- [Before aerial horizon](before/screenshots/metro-10.png) / [after aerial horizon](after/screenshots/metro-10.png)
- [Final recovery](after-recovery/screenshots/06-remounting.png)

Implementation files: src/sky.rs, rr_content world_art validator and
content/world/art.json. Records: README/PROGRESS, ADR113, known issues,
chronological history, source provenance and this evidence directory. No task
placement/state/count changes; SKY-01/V2-06-007 already cover this scope.

## Final mandatory checks and status

Fmt, strict locked workspace/all-target Clippy,430workspace tests(one existing
ignored), locked debug build, asset/dependency audit, five roadmap tests and
progress synchronization all exit0. Fresh locked release build exits0. Model
180tick/body/joint differential and recovery/ground-support tests remain in the
workspace suite. No unrun paired rig/native legacy/hardware checks are claimed.
`git diff --check` passes. Exact commands/exits: checks.json; release.txt and
native receipts. Initial provenance failure and rejected HDR are retained.

Git:9tracked modified files plus this untracked pass09 evidence directory;
branch codex/gameplay-graphics-polish. No commit, push or merge. Parent reviews
and integrates. No new IDs, checklist closures, placement or count changes.
