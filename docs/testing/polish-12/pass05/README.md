# Pass05 — shadow contact and cascade transitions

Follows e87ed2be8b6594d07ae7f923cd151698d391baee, isolated RoadDash-polish
worktree / codex/gameplay-graphics-polish. ADR109; P15-004 evidence extension,
no new IDs, moved tasks or broad checklist closures.

## Implementation and investigation

Gameplay used implicit Bevy0.17.3 default normal bias1.8, depth bias0.02,
four2048px cascades with10m first split and150m reach,20% overlap. Receiver
normal bias is scaled by each cascade's texel size: a generic large offset
separates contact shadows and the effect grows in coarser maps. Set normal
bias1.5 while retaining depth bias; increase overlap to25% to blend a wider
boundary region. Preserve the same near resolution, far coverage, map allocation
and caster set. This is a bounded contact adjustment, not a new shadow effect.
Wheel/rider, foundation and tree contact all use the same existing renderer.
No per-frame component mutation or extra entity scanning is introduced.

The installed cascade.rs already rounds cascade diameter and snaps its center
to texels for stability. Do not add another snapper or modify authoritative
transforms. Existing ornament/frontage and coarse-facade shadow exclusions stay;
resident building envelopes and collision remain independent. No indiscriminate
caster enabling, PCSS/SSAO, new lights/maps, texture enlargement, materials/atlas,
lighting/exposure, rig/simulation, saves/controls/audio or dependencies changed.
City-preview's separate gallery light remains unchanged.

Source research: local pinned bevy_light0.17.3 directional_light.rs/cascade.rs
and existing world/facade paths. No external repository, copied upstream code,
new assets or media. Updated existing world source provenance hash only.

## Verification

Run `python3 docs/testing/polish-12/pass05/run_native.py before` against pass04
release, then before-recovery. Rebuild with shared CARGO_TARGET_DIR and run after
and after-recovery. Every native process uses its own save directory and cwd;
process snapshots verify no concurrent compiler/game. Scripted stations are
matched1440×900 native engine captures, with moving actor/cloud phases differing.
Compile/authoring work never overlaps measurements. `run_checks.py` records
mandatory commands and exits. World integration assertions verify installed
normal/depth bias,4 cascade bounds and overlap while retaining solid scenery.
Existing differential physics/recovery tests remain in the full workspace suite.

Static captures cannot establish temporal shimmer elimination or racing-speed
acceptance. The overlap trades a little extra transition sampling/coverage for
blending; allocation counts alone do not measure GPU cost. Native wall-frame
samples cover total work, not isolated GPU timestamps. Human art, target16.7ms,
full-route/crash performance, temporal camera motion and Windows/hardware gates
remain open under P15-004/PERF-08. No next-pass work started.

The initial bias1.0 trial was rejected after remount inspection showed faint
repeating terrain self-shadow patterns. Retained rejection capture/metrics
under rejected-bias1*. Final comparison uses the subsequent conservative1.5
candidate, not those rejected timings. This tradeoff is why depth bias remains
unchanged and why final moving-camera/hardware acceptance stays open.

Final1.5 remount/getting-up captures were inspected: wheel/rider contacts remain
connected and the obvious repeated field pattern from the1.0 trial is absent
in that view. This is a subtle bias adjustment; no quantitative perceptual
improvement or elimination of all acne is claimed. Native recovery exit0,
reverse−1.898m/s and both pickup errors0.0000m; fixed recovery stages all true.
No rig changes, so broad paired baseline/anatomy native labs from pass04 were
not repeated; mandatory workspace differential tests cover both paths.

## Final native cost and review

Fresh matched pass04→final pass05 survey:19.459→19.073ms average,
37.082→36.298ms p95. Both miss16.7ms; one pair does not establish a speedup,
regression or isolated shadow GPU cost. Census is identical:2,909,113 render
triangles,2,613,119 collision triangles,3067chunks,2531collision batches,
19,068buildings,1494props,122parked cars,96walkers. `comparison.json` contains
exact metrics; each native directory has command/exit and process receipts.

Representative original engine images retained for farm,entry,city,finish:
[before](before/screenshots/metro-01.png) / [after](after/screenshots/metro-01.png),
[entry](after/screenshots/metro-03.png),[city](after/screenshots/metro-09.png),
[finish](after/screenshots/metro-13.png). Farm/entry/finish and recovery views
were visually inspected. [Final remount](after-recovery/screenshots/06-remounting.png)
and [rejected1.0](rejected-bias1-recovery/screenshots/06-remounting.png) retain
the bias tradeoff. Static vegetation shadow coverage remains existing and
bounded; no broad botanical or temporal art approval is inferred.

`field-diagnostic.json` supplements visual trial rejection: in the raw remount
field rectangle x1000–1180/y320–430, mean adjacent horizontal RGB difference
is0.445 before,0.608 for rejected1.0,and0.444 for final1.5 (0–255 values).
This simple flat-field variation diagnostic is not photometry or a perceptual
score; phases/poses can differ. It supports the visible pattern tradeoff without
claiming a global image-quality improvement.

Final verification: fmt,strict workspace/locked/all-target Clippy,421 workspace
tests (1 pre-existing ignored soak),locked debug build,audit,5roadmap tests and
progress check all exit0 (`checks.json`). Locked release build exit0 and current
shared release binary holds final pass05. Model differential180-tick/body/joint
and crash/run/remount tests pass. No unrun labs/GPU/hardware checks claimed.
Files: src/world.rs; README,PROGRESS,DECISIONS,KNOWN_ISSUES,CHANGELOG_DEV,
third_party/ASSETS.csv; pass05 receipts/scripts/raw screenshots. No task IDs,
placement/count changes or checklist closures. Git status captured in
`git-status.txt`; parent integration pending, no commit/push/merge.
