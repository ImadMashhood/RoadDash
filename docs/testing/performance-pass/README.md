# Controlled performance follow-up — 2026-11-09

Scope: existing PERF-08/P16-003, ADR134. No new requirement IDs or checkbox closures.
Previous integrated pass d8824bf was fast-forwarded into local main, then work
continued on codex/performance-profile-pass. Remote push awaits desktop Git
credentials; a local merge does not mean GitHub has been updated.

## Method

Frozen release binaries, Ryzen 5 3600 (6 cores/12 threads), RTX 3080,
NVIDIA 615.78.08, 1440×900 native survey, same authored Metropolis and traffic.
`run.py BINARY NEW_OUTPUT_DIRECTORY` runs --metro-survey with isolated saves,
settings and logs and refuses overlapping compiler/game processes. Fourteen
views, 300 frames each; manual 60Hz simulation runs one fixed tick per rendered
frame. Exclude the first ten one-second samples. Report medians/p95 across
window averages and distinguish these from the rolling 240-frame p95 metric.
No GPU timestamps, real-time full-race load or broad hardware acceptance implied.

The original baseline is d8824bf. Profile adds crossing admission short-circuit
and pair census. Experimental candidate additionally skipped precise damage sweeps when a
conservative swept box contained no breakable; the same-binary control disabled
that shortcut. This experiment was subsequently removed for lack of consistent
benefit. Final retains only the crossing optimization and diagnostics. No visual density, physical tuning,
collider geometry, traffic or pedestrian counts change.

## Investigation

- Baseline gameplay median 4.218 ms/frame; physics approximately 11.077 ms/fixed
  step, dominated by solver/step work. Pair census found 33,581 static-static
  pairs in about 35,698 graph pairs, with only 16 active manifolds in a late
  sample. Graph edges are not active collisions.
- Inspected installed pinned Rapier source: body-type rejection precedes group
  filtering, after graph creation. A group filter would not remove those edges.
- Four/two worker experiments slightly changed physics timing but failed to
  improve total frame time; default worker selection remains unchanged.
- Standalone breakable colliders failed the existing supported-debris settling
  regression. Reverted the entire representation experiment, including tests;
  fixed props and all existing body lifecycle remain intact.
- Experimental prefilter passed physical trajectory/hit/occlusion comparisons,
  but candidate gameplay median 3.744 ms versus control 3.724 ms did not justify
  retaining the extra query. Its code, switch and tests were removed. Crossing
  admission rules remain unchanged.
- Native destruction hit all 13 fixture props, then failed its obsolete hardcoded
  world count (1,534 actual including fixtures versus 1,008 expected). Fixture now
  records the loaded production count and verifies it plus 13 test props remains
  present; damage, budget, actual visuals and reset assertions stay intact.

## Measurements

Milliseconds; window median and p95 are distributions of one-second window
averages, not raw frame percentiles. Raw logs remain in
`/home/imad/roaddash-performance-pass`; adjacent JSON records summaries, SHA-256,
commands and environment for every frozen executable.

| Run | Selected windows | Wall median | Wall window p95 | Gameplay median |
|---|---:|---:|---:|---:|
| Original baseline | 93 | 25.384 | 27.028 | 4.218 |
| Crossing/census profile | 90 | 24.445 | 25.838 | 3.818 |
| Profile, four workers | 91 | 24.647 | 26.079 | 3.863 |
| Profile, two workers | 91 | 24.733 | 26.192 | 3.867 |
| Experimental damage prefilter | 91 | 24.765 | 26.604 | 3.744 |
| Original baseline repeat | 91 | 24.837 | 26.895 | 4.124 |
| Same experimental binary, prefilter off | 91 | 24.818 | 26.802 | 3.724 |
| Final retained source | 91 | 24.681 | 26.821 | 3.816 |

Final gameplay work falls approximately 7–10% versus the two baseline captures. Total frame-time differences are smaller and overlap
run-to-run variation; do not claim a reliable FPS percentage gain. Physics
remains around 11 ms/fixed step. The 16.7 ms frame target stays open.

## Verification and scope

Final **529 workspace tests pass**, with the existing long soak ignored. Formatting,
strict Clippy (all targets), locked debug/release builds, and audit pass; see
[check logs](checks). Audit: 580 dependency declarations, 180 runtime assets.
[Native receipts](native-receipt.json) verify destruction/reset and recovery on
both current and baseline rigs. All 13 prop kinds break and reset; 1,521 production
props remain present, and moving-debris budget remains bounded. The final binary
SHA-256 is `64cd5f3acde7793ff8fa184153f5078db86d56833cc0cde7710bba1043de6210`.
Final city survey also retains 144 main cars/384 pedestrians and completes two
crossings with the original admission limits.
The earlier 530-test log includes the subsequently removed experimental test;
final retained source has the original 529 tests, not a newly closed requirement.

Files: `src/world_gen/metropolis/pedestrians.rs` (admission),
`src/diagnostics/perf.rs` (pair census), `src/destruction/survey.rs` (fixture),
plus README/PROGRESS/DECISIONS/KNOWN_ISSUES/history and these reproducible receipts.
No external assets/code or dependency versions changed. No new IDs; PERF-08 and
P16-003 remain open, with 222 verified / 256 open in the live checklist.

Next performance work should isolate physics step/synchronization and render/GPU
costs in full-route real-time captures, preserving all collisions and populations.
This pass does not establish online bandwidth, Windows, other hardware, or human
playtest acceptance. Local main contains the previous completed integration;
performance work is on its focused branch. Remote push remains uncompleted while
Git waits for authentication.
