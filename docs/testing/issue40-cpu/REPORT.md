# Issue 40 candidate A: fixed-step traffic observation reuse

Status: isolated CPU candidate on `codex/issue-40-cpu`, based on `e2a3191876d57f266b1e36c967a48c9e8e5667ae`. This is not the four-way selection or a 60 FPS acceptance. PERF-08 and RFU-PERF-02 remain open.

## Decision and implementation

`drive` planned each AI rider against a full Bevy traffic query. For each rider it repeated the same traffic collider extent, route orientation, and world-to-road lane projection even though traffic is advanced only after the rider loop. Build a tick-local observation list before rider planning; admit a car when it is in any active AI rider's original 520 m coarse window, then reuse its exact placed lane, center offset and extents. The final rider-specific 500 m check, source iteration order, planner, colliders, body transforms, 60 Hz schedule, recovery and save data remain unchanged. The coarse admission uses the original closed-course wrap and corridor rule. The list is dropped after the tick. No dependency, asset, authored content, density or physics coefficient changes.

The benchmark gate itself had a dead Cargo process (`cargo <defunct>`, PID961689) that `ps` reported with state text `<defunct> ZN`; a one-line harness correction ignores zombie states. Active compiler/game/indexer rejection is unchanged. This is explicit common-harness maintenance, not a timing flag change. Two earlier warm attempts were discarded: one overlapped Cargo metadata from audit; one overlapped a Git worktree checkout. Their incomplete evidence is retained outside the repository. No result below uses them.

## Reproduction and provenance

Linux Ryzen 5 3600, RTX 3080, NVIDIA 615.78.08, 1440×900 survey; seed25. Frozen clean baseline release binary SHA256 `682b328291655b18b812c2182ed5366bdd1bdf5bf948ca6731fee64863911536`; candidate release SHA256 `7fb2efdf1be33d1d1bdcafb3a9916c53af55cfee304dff8b4c935920727f0634`. Baseline was rebuilt after cleaning all three workspace packages in a private target. Both binaries are read-only with adjacent provenance JSON. Exact commands used the existing `tools/run_performance_profiles.py` from this checkout, `--track both --runs 2 --fixed-executor single --load-jobs 8 --gpu --indexer-pid 986317`, and `flock -x /tmp/roaddash-native.lock`. The harness isolates settings/saves/logs, disables screenshots, records bounded raw frames and suspends/resumes the verified Baloo extractor with pidfd. Both warm batches exited 0 with no detected contaminants.

The [checked-in evidence](evidence/) includes all 16 native summaries, compressed raw JSONL frame vectors, stdout, survey manifests and receipts. Original uncompressed run directories remain at:

- Baseline warm: `/home/imad/roaddash-issue40/baseline-warm-final/`
- Candidate warm: `/home/imad/roaddash-issue40/cpu-warm/`
- Baseline cold asset-page repeat: `/home/imad/roaddash-issue40/baseline-cold-final/`
- Candidate cold asset-page repeat: `/home/imad/roaddash-issue40/cpu-cold/`

Cold mode evicts checkout asset file pages with `POSIX_FADV_DONTNEED`; it does not reset executable, metadata or GPU driver caches and is not a cold boot. Both cold batches exited 0 with no detected contaminants, using the same flags plus `--cold-assets`.

## Warm native results

Times in ms except readiness seconds and VmHWM MiB. Raw p50/p95/p99/max are individual post-warmup racing frames. `game/tick` and `physics/tick` are weighted CPU spans; CPU spans include wait/dispatch. `opaque GPU` is the median of asynchronous latest available 3D opaque-pass timestamp samples per one-second window, not a full-frame GPU percentile. No unsupported GPU value was filled with zero.

| Binary | Map/run | Ready s | Raw p50 | Raw p95 | Raw p99 | Raw max | Game/tick | Physics/tick | VmHWM MiB | Opaque GPU |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Baseline | Hills 1 | 20.306 | 15.900 | 18.864 | 20.828 | 34.124 | 2.734 | 4.719 | 2151.0 | 1.067 |
| Baseline | Hills 2 | 20.406 | 15.941 | 18.943 | 21.130 | 27.012 | 2.724 | 4.682 | 2154.2 | 0.914 |
| Candidate | Hills 1 | 20.498 | 15.767 | 18.725 | 20.807 | 38.360 | 2.656 | 4.673 | 2153.9 | 0.936 |
| Candidate | Hills 2 | 20.354 | 15.816 | 18.645 | 20.691 | 23.905 | 2.676 | 4.626 | 2198.6 | 1.013 |
| Baseline | Metro 1 | 7.215 | 20.119 | 24.719 | 26.874 | 32.478 | 2.734 | 8.805 | 5973.6 | 2.040 |
| Baseline | Metro 2 | 6.945 | 20.022 | 23.675 | 26.006 | 31.458 | 2.739 | 8.555 | 5875.6 | 2.370 |
| Candidate | Metro 1 | 6.783 | 19.921 | 23.465 | 25.249 | 28.866 | 2.669 | 8.580 | 5868.5 | 1.649 |
| Candidate | Metro 2 | 7.186 | 20.393 | 24.073 | 26.316 | 31.091 | 2.780 | 8.879 | 5972.7 | 1.960 |

Every warm case has zero >50 ms racing frames and zero detected contaminants. Mean raw p95 is 18.904→18.685 ms Hills (1.2% lower) and 24.197→23.769 ms Metro (1.8% lower). The small Metro gain overlaps run-to-run variation; candidate run 2 has higher fixed gameplay and physics spans than both baseline runs. Readiness and opaque GPU samples also vary in both directions. This is evidence of a bounded simulation work reduction, not a proven whole-frame FPS gain or the 16.7 ms target.

## Cold asset-page results

Same units and sampling as the warm table; `fincore` recorded zero resident checkout asset bytes after every targeted eviction. This is a separate cache condition, not a certified cold-machine boot.

| Binary | Map/run | Ready s | Raw p50 | Raw p95 | Raw p99 | Raw max | Game/tick | Physics/tick | VmHWM MiB | Opaque GPU |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Baseline | Hills 1 | 20.486 | 15.880 | 18.567 | 20.385 | 31.979 | 2.711 | 4.665 | 2193.2 | 0.959 |
| Baseline | Hills 2 | 20.641 | 15.956 | 18.788 | 20.411 | 23.770 | 2.714 | 4.683 | 2204.4 | 1.065 |
| Candidate | Hills 1 | 20.303 | 15.898 | 18.922 | 20.864 | 38.080 | 2.667 | 4.698 | 2187.4 | 0.938 |
| Candidate | Hills 2 | 20.309 | 15.877 | 18.588 | 20.461 | 25.014 | 2.675 | 4.721 | 2175.9 | 0.968 |
| Baseline | Metro 1 | 6.919 | 20.798 | 24.807 | 26.471 | 28.365 | 2.866 | 8.937 | 5891.0 | 2.367 |
| Baseline | Metro 2 | 6.815 | 20.191 | 24.250 | 26.296 | 37.551 | 2.766 | 8.679 | 5985.2 | 1.723 |
| Candidate | Metro 1 | 6.980 | 20.047 | 23.831 | 25.502 | 27.818 | 2.691 | 8.536 | 5982.0 | 2.078 |
| Candidate | Metro 2 | 7.143 | 20.568 | 24.555 | 26.942 | 42.450 | 2.744 | 8.980 | 5902.9 | 2.072 |

Mean cold raw p95 is 18.678→18.755 ms Hills (0.4% higher) and 24.529→24.193 ms Metro (1.4% lower). The candidate does not consistently lower raw p95 or readiness under both cache conditions. No case exceeds 50 ms, but the candidate's Metro cold maximum of 42.450 ms exceeds the baseline cold maximum of 37.551 ms; these extrema are one-frame observations, not a repeatable stall finding. The full 16.7 ms p95 gate remains unmet.

Strict Clippy identified the measured candidate's lazy `bool::then` car construction. The final source expresses the same lazy branch as an explicit `if`/`Some`, with identical values and admission order. The final release binary is SHA256 `f2ee87bca21f09abbdec9408ab618fc940e893174914832723787e1d9e1d5537`; the profiles above belong to the prior SHA256 `7fb2…`, not the final hash. Treat the small speed differences as exploratory until final-source profiling in the integrated comparison.

## Content and behavior checks

All 36 matching warm and cold survey JSON pairs (one Hills and eight Metro files in each of two runs per cache condition) agree exactly after removing timing-only `metrics` and `load_to_material_ready_seconds`: Hills `hills-revision.json`; Metro `farm-placements`, `urban-extra-cars`, `pedestrian-crossings`, `metro-pedestrians`, `metro-street-life`, `metro-authored-facades`, `metro-survey`, and `metro-performance`. Metro retains 19,108 buildings, 2,671,458 collision triangles, 2 physical routes, 58 crossing cars, 1,521 street props, 122 parked cars and 38 walking pedestrians. This is a deterministic survey/content comparison, not a full race/combat replay.

Final-source `cargo fmt --all --check`, `cargo clippy --workspace --locked --all-targets -- -D warnings`, `cargo test --workspace --locked` (617 passed, two pre-existing ignored), `cargo build --locked`, `cargo build --release --locked`, `python3 tools/audit.py` (580 dependency declarations, 198 runtime asset records) and `python3 tools/sync_progress.py --check` pass. [Paired final-source native recovery](evidence/final-recovery/receipts.json) passes 4/4 Hills/Metro × current/eleven-body-baseline cases with no render errors or exceptional Hills rescue; these are functional checks, not performance runs. The integrated four-way comparison remains pending. No source repository or media was imported. Installed Bevy/Rapier remain pinned by `Cargo.lock`; the local performance skill was workflow guidance only.
