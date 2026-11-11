# Issue #40 — four-candidate performance comparison

Status: four independent experiments and draft PRs are reviewable; no candidate
has a verified combined-content win. RFU-PERF-06/07 remain open. No performance
candidate was merged.

## Frozen comparison

The common source is main `e2a3191876d57f266b1e36c967a48c9e8e5667ae`.
Its frozen release executable SHA256 is
`682b328291655b18b812c2182ed5366bdd1bdf5bf948ca6731fee64863911536`.
Runs used the same Linux Ryzen 5 3600 / RTX 3080 host, seed 25, 1440×900,
single fixed executor, eight loading material jobs, isolated saves/settings,
Hills and Metropolis surveys, and the same native profiling harness. Warm
baseline cases were repeated twice per map. The accepted timing batches exclude
active game, compiler and indexer contaminants; a verified Baloo extractor was
suspended and restored. Cold asset-page trials use file-page eviction, not a
cold reboot. GPU numbers are asynchronous latest-pass samples, not full-frame
GPU percentiles. Each candidate report preserves raw receipts and provenance.

ADR150 preregistered the selection: compare raw gameplay p95 twice as strongly
as required-assets-ready time on each map, then review p99/max, memory, GPU,
visual and gameplay correctness as vetoes. A repeatable >5% p95 regression on
either map rejects a candidate. A lower candidate/baseline geometric ratio is
better, but small point estimates within run variation are exploratory.

| Candidate | Draft PR | Warm Hills p95 (ms) | Warm Metro p95 (ms) | Warm ready Hills / Metro (s) | Weighted ratio | Review |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| CPU traffic snapshot reuse | [#44](https://github.com/ImadMashhood/RoadDash/pull/44) | 18.904 → 18.685 | 24.197 → 23.769 | 20.356 → 20.426 / 7.080 → 6.985 | ~0.9886 | Nominal warm lead; cold Hills p95 worsened 18.678 → 18.755; measured binary predates final lint-only source edit. |
| Sparse facade visibility sync | [#49](https://github.com/ImadMashhood/RoadDash/pull/49) | 18.904 → 19.188 | 24.197 → 23.953 | 20.356 → 20.509 / 7.080 → 7.127 | 1.0040 | Hills regressed and had one 89.716 ms frame; rejected. |
| Loading admission / height cache | [#46](https://github.com/ImadMashhood/RoadDash/pull/46) | Not selected | Not selected | Jobs 2 → 8 had mixed warm/cold readings | — | Per-material height cache worsened warm Metro p95 ~5.4%, Hills readiness ~2.6%, with a 240 ms outlier; runtime experiment reverted. |
| Grass-wind shader arithmetic | [#45](https://github.com/ImadMashhood/RoadDash/pull/45) | 18.904 → 18.807 | 24.197 → 23.976 | 20.356 → 20.542 / 7.080 → 6.778 | 0.98955 | About 1.05% warm nominal gain, but Hills max 30.568 → 38.057 ms and opaque GPU pass sample 0.991 → 1.039 ms. |

The CPU ratio is computed from rounded report medians and is approximate;
the GPU and facade ratios use their reports' unrounded values. Candidate A and
D differ by less than a tenth of one percent in this score. Neither proves a
repeatable whole-frame gain. Candidate C's final source contains no retained
runtime optimization. The best measured warm Hills and Metro p95 values still
exceed 16.7 ms, so none meets the requested broad performance target. Lighting
bake, physics workload, new content cost and Bevy 0.20 migration were not
established as gains by these four experiments.

## Correctness and limits

Each draft report contains strict format/Clippy, workspace test, build, audit,
content-census and native evidence for its own source, with limitations stated
there. The CPU final source passed 617 tests with two ignored but was not
reprofiled after its lint-driven source edit. The facade candidate passed 617
tests with two ignored and matched structural surveys; its Hills tail spike has
no established exclusive cause. The GPU candidate passed 617 tests with two
ignored and had close Metropolis visual views; the exact binary provenance of
one comparison capture is missing. The loading candidate retained its failed
trials and reverted the slower runtime changes. These are separate branches;
none was tested as the combined city/FX/guardrail/AI world.

Keep the four drafts unmerged. After content and AI branches integrate, freeze a
new source/assets/binary baseline and repeat warm and cold, both-map trials for
the most plausible candidate(s), including exact final source, p95/p99/max,
readiness, memory, supported GPU samples, content/physics parity and visual
checks. Select only a repeatable winner without the vetoes above. The existing
PERF-08, RFU-PERF-02, P16-003 and 16.7 ms goals remain open.

No external repository code or media was copied. The installed engine remains
Bevy 0.17.3 with Rapier 0.32.0; Bevy 0.20 migration is separate issue #37.
