# Part07 verification receipts — 2026-11-08

All commands below ran on the final queued-drop implementation. No dependency
migration: Bevy0.17.3 / Rapier0.32.0 and Cargo.lock remain pinned/unchanged.

| Verification | Actual result |
| --- | --- |
| `cargo fmt --all --check` | Passed; fmt.txt. |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | Passed; clippy.txt. |
| `cargo test --workspace --locked` | **349 passed,1 existing soak ignored**; tests.txt. Includes the original model/body/joint/180-tick differential trajectory and complete crash/run/remount checks. |
| `cargo build --locked` | Passed; build.txt. |
| `cargo build --release --locked` | Passed; release.txt; current binary hash in PROVENANCE.json. |
| `python3 tools/audit.py` | Passed; audit.json. Declarations/hashes are not legal clearance or vulnerability analysis. |
| `python3 tools/test_roadmap.py` | All5 passed; roadmap.txt.448 stable IDs,206verified/242open; no new IDs. |
| `python3 tools/sync_progress.py --check` | Passed; progress-check.txt. |
| `python3 -m unittest tools/test_animation_context.py tools/test_animation_authoring.py` | All10 passed; source-tests.txt. |
| `python3 tools/report_animation_coverage.py --check` | Passed; coverage-check.txt;15 clips, old13 preserved. |
| `python3 tools/package_hand_review.py` and `--check` | Actual fresh39-view hand run; existing33-view gallery repackaged and source/image checks passed; hand-package.txt / hand-provenance-check.txt. |
| `python3 tools/package_weapon_review.py` and `--check` |12 intact club body/hand views plus5 native keyboard/recovery images source-bound; weapon-package.txt / weapon-provenance-check.txt; [gallery](gallery/index.html). |
| `target/release/road-dash --ragdoll-lab --output docs/testing/part07-weapons/ragdoll-lab.json` | All18 baseline/biomechanical paired cases finite; ragdoll-lab.json / .txt. Headless wall times are not native performance. |
| Current release `--smoke-test` with `ROADDASH_VERIFY_CLUB=1 ROADDASH_VERIFY_STRIKES=1 ROADDASH_VERIFY_REVERSE=1` | Passed native real keyboard pickup/swing/drop, seven racers/seven killers, four audio sinks, both farm/city crash/run/remount paths and results. native-metro.txt. Reverse−1.800m/s; both pickup errors0.0000m. Native contact age.233333s against captured authored-derived threshold.232778s. |
| Current release `--animation-lab-view --hand-review --animation-lab-smoke` | Completed39 actual captures/13 modes; native-hand.txt. First33 refresh the existing hand gallery; new6 show held club keys2162/2172 with actual socket geometry. |
| Current release `--animation-lab-view --animation-lab-smoke` | Completed39 actual full-body/live-clip captures/13 modes; native-club.txt. New6 retained in gallery. Inherited console summary still says eleven motions; files/counts distinguish this harmless stale diagnostic. |
| `git diff --check` | Passed; diff-check.txt. |

Native environment: Linux/X11 `DISPLAY=:0`, RTX3080/Vulkan, isolated save and
settings paths, persistence disabled by the smoke. No Windows/hardware-controller
certification. The script's finish is a results fixture after recovery; headless
race/checkpoint tests remain the checkpoint-valid finish evidence. Native club
swing is a real keyboard lifecycle/visible whiff; deterministic moving tests own
landed-contact/LOS/range/AI assertions. No human three-minute fun test was run.
`ROADDASH_VERIFY_REPAIRS` and its legacy manual gesture are not animation proof
for this run. Auto-generated native prop intake was archived as
native/native-assets.json and removed from the shared intake location.

## Measured cost and honest limits

`native/weapon-census.json`:30 prepared held roots,7 ground roots after the
fixture pickup/drop,111 resident club part instances and21 visible parts at the
census (all7 ground clubs). This is148 extra roots/children in that scene,
252 displayed club triangles at that sample. No added GPU mesh/material assets
or physical bodies; shared primitive geometry does not make hierarchy traversal /
residency free. The bound is168 entities/126 mesh instances with at most27
visible clubs/972 triangles; see README for derivation.

`native/smoke-metrics.json`:40,112 total scene entities,4,252 colliders,1,314
render frames/1,313 fixed ticks. The reported bounded rolling wall-frame sample
averages **19.811ms**, p95 **22.633ms** at1440×900. Target16.7ms is missed.
`native/performance.jsonl` retains warm-up and one-second CPU/wall/memory samples;
these are not GPU timestamps. No controlled before/after performance comparison
was run, so unchanged cost or a regression magnitude is not claimed. Human art,
long-streaming/performance and multi-hardware gates remain open.

Rear/front raw hand views show the shaft attached and grip profiles active.
The angular glove/protruding fingers and body-occluded right-side closeup do
**not** establish convincing wrap, clearance or finished original modeled-club
fidelity. Full-body poses and actual in-race club are visible; this is functional
draft presentation only. Preserve raw views in gallery and existing hand gallery.

## Regressions and corrections during this pass

Initial test runs caught a standalone prop fixture missing WeaponRules after the
scheduler gained the resource; a support fixture running disabled Garage physics;
strict equality across a still-smoothing next-frame tuck; and an AI pickup fixture
creating a pickup before its wall entered Rapier. Fixtures/resource wiring were
corrected and the final full suite passed. Primary review also led to shared
ability resources, malformed-clip validation, true queued Q drop, arm-only ready
layering, inherited visibility, live-point support rays and pickup occlusion.
No failure was hidden by checking a broad roadmap box. Initial failure receipts
are retained alongside the final tests.

P05-003 alone is verified/moved into Part00. The remaining ten Part07 requirements
retain their full unmet acceptance under README; no new task IDs, Part08 work,
push, merge, performance/art/legal approval or human-feel approval is implied.
