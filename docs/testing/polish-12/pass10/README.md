# Pass10 — Skybox and clouds

2026-11-09, `codex/gameplay-graphics-polish`, previous commit
`0a23465c406eeb8096ad2341a6de0efd165838fa`. ADR114. Work stays in RoadDash-polish;
original RoadRash dirty UI work untouched. Parent reviews and commits.

The old visible cube bypassed the non-HDR PBR tone operator, clipping cloud whites
and creating a bright cyan horizon against tonemapped haze. It also sampled a
north/south-mirrored sun and rotated both the sun and lighting environment as wind.
The supported non-HDR path now uses a separate display-referred cube generated
with the same authored exposure and analytic tone operator as the scene. Its
brightness cancels only Bevy's built-in exposure multiplier. Horizon radiance,
fog density, sunlight/cool fill and original environment map inputs stay intact.
No HDR migration, custom shader or inverse-fog compensation ships.

Original continuous direction-domain cloud banks now combine broad masses,
smaller lobes, sunward shading and a faint high veil. A deeper overhead blue and
restrained warm solar neighborhood converge to the exact shared horizon color.
Direction-domain sampling avoids longitude seams and planar horizon stretching.
The display cube reverses Z to match Bevy's cube sampler, keeping the sun in the
same world direction as its34.7degree directional light. Legacy lighting cubes
remain unchanged; their existing approximate reflection distribution is not
claimed as exact physical sky integration.

Cloud motion is **stationary**, not newly animated. Removing the old drift system
keeps the solar disc, horizon and environment stable without per-frame texture
rebakes. The legacy wind field remains compatible but inactive, with a source
comment. Independent cloud-only animation remains landscape work. This pass does
not close V2-06 landscape/art tasks or reinterpret static screenshots as temporal
or human approval. Fixed exposure is authored; later runtime exposure changes
must rebuild the display cube and fog together.

## Verification and reproduction

Run stages serially with no compiler/game/bake overlap and frozen inputs:

```
python3 docs/testing/polish-12/pass10/run_native.py before
CARGO_TARGET_DIR=/home/imad/Documents/ChatGPT/RoadRash/target cargo build --release --locked
python3 docs/testing/polish-12/pass10/run_native.py after
python3 docs/testing/polish-12/pass10/run_native.py after-recovery
python3 docs/testing/polish-12/pass10/compare_native.py
python3 docs/testing/polish-12/pass10/run_checks.py
```

Before requires the previous committed executable. Native runner isolates saves,
records frozen input hashes, rejects blank/missing screenshots and retains logs.
Sky tests compare64horizon directions against actual fog/Reinhard output, all24
cube-face edge pairings, dense finite/range/cloud/detail-continuity samples,
world/cube solar conversion and installed exposure/light/shadow/environment
invariants. Workspace tests include the accepted model differential physics and
recovery fixtures. No paired whole-rig lab sweep is warranted by a sky-only edit.

One additional256square six-face RGBA8 cube with nine mips stores2,097,144bytes
(~2MiB) each in CPU asset memory and GPU texture storage. Existing256specular and
16diffuse cubes remain; total pixel payload4,202,472bytes per CPU/GPU side.
Generation occurs once at installation and is logged in native.txt. No additional
sky draw, shader sample, light, mesh/entity or per-frame upload/update is added.
No performance gain or universal GPU affordability is inferred from this design.

All geometry, facade near/far material/atlas hashes/mips and budgets, resident
envelopes/collision, route/recovery, physical rigs, controls, saves, audio and race
logic remain unchanged. Source provenance, including licensed SBDT adaptation,
is explicit in PROVENANCE.md and third-party records. No new media/assets intake.

No IDs, checkbox states, placements or counts change (220verified/245open).
README SKY-01 evidence and generated PROGRESS index stay synchronized; existing
V2-06-005/006/007/008 retain full landscape, art/hardware/performance acceptance.

## Native review and measured cost

Final fresh release metro survey and recovery both exit0, with unchanged frozen
inputs, nonblank captures and no shader/pipeline errors or panics. Reverse remains
−1.898m/s and both pickup errors0.0000m. Existing retry,15rider roster, audio and
farm/city recovery assertions pass. All14actual ready-detail draw audits and the
resident census are equal before/after:19,068buildings,3,067chunks,
2,909,113render triangles,2,613,119collision triangles and2,531static batches.
No fog/sky improvement substitutes for missing geometry or altered LOD policy.

Inspected matched native farm00, street-entry03 and aerial10. The cyan horizon
boundary is removed in these views, clouds retain shading rather than clipped
white centers, and street faces/traffic retain near contrast. The aerial horizon
now meets the haze without the previous bright band. The256px cloud field still
has soft edges: this is economical original procedural presentation, not a claim
of photoreal volumetric clouds or user-approved final landscape art. The sun's
world alignment is verified analytically; these forward route views are not a
native close-up solar-angle measurement. Static views cannot prove temporal or
Windows/hardware quality. Nondefault SBDT compiles but was not native-captured.

One-time display-cube generation measured120.588ms in recovery startup. Native
whole-app wall frames before18.238ms average/32.108ms p95 versus after18.493ms
average/33.548ms p95. Both miss16.7ms. One pair does not isolate sky GPU cost or
establish a causal regression/speedup; no affordability/performance gate closes.
See cost.json, comparison.json and native receipts for exact samples and limits.

- [Before farm](before/screenshots/metro-00.png) / [after farm](after/screenshots/metro-00.png)
- [Before street](before/screenshots/metro-03.png) / [after street](after/screenshots/metro-03.png)
- [Before horizon](before/screenshots/metro-10.png) / [after horizon](after/screenshots/metro-10.png)
- [Recovery](after-recovery/screenshots/06-remounting.png)

Implementation: src/sky.rs, src/runtime.rs and the retained wind-field comment in
rr_content/world_art.rs. Records: README/PROGRESS, ADR114, known issues, development
history, ASSETS.csv, repository research and this evidence directory. No task
placement/count/state changes. No commit/push/merge; parent integrates.

## Final mandatory checks and git status

Fmt, strict locked workspace/all-target Clippy,432workspace tests(one existing
ignored), locked debug build, source/dependency audit, five roadmap tests and
progress synchronization all exit0. Fresh locked release build also exits0.
See checks.json and individual transcripts. Existing model differential/recovery
checks are included in the workspace suite; no unrun hardware/legacy/native
whole-rig checks are claimed. `git diff --check` is clean.

Git status:10tracked modified files plus untracked pass10 evidence directory on
codex/gameplay-graphics-polish. Exact list: git-status.txt. No commit/push/merge.
Proposed commit: `feat(render): align procedural sky with haze and sunlight`.
