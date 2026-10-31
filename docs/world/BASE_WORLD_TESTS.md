# Base-world foundation verification — 2026-10-30

Scope: BW-B00/B01/B02 only. No B03–B10 geometry/performance completion claim.
All paths below refer to actual artifacts collected on this host.

## Baseline / census

Source main `78d6a828268d85abd73df28b72c23df0407c4ee3`, identical tree to
handling commit `bd6712c`. The only edits inside the legacy building spawning
paths add diagnostic tags; transforms, mesh/materials and colliders are unchanged.
[Measured census](evidence/census.json) and [native startup census](evidence/native-startup-census.json)
compare exactly, including all 198 existing building records and sorted geometry
fingerprint `fnv1a64:3370c4518b47abf0`. The unit test independently reconstructs
that environment twice and compares every census field. The three source inputs
are retained unchanged with SHA-256 digests in `source/MANIFEST.json`.

| Measurement | Result |
| --- | --- |
| Open baked route | 6847.307617 m, nine gates, eight grid spawns |
| Authored world length / terminal snap | 6847.3 m / +0.0078125 m in f32 |
| Current legacy building masses | 198 (not 247 newly placed buildings) |
| Environment-only mesh / collider entities | 7298 / 1817 |
| Road / terrain chunks | 86 / 160 |
| Existing ribbon/terrain render / collision triangle counters | 144620 / 133540 (not total world triangles) |
| Planned target | 247 assignments, 177 near / 70 secondary |
| Registry / sections | 19 archetypes / 15 sections |
| Family targets | 35 farm / 63 residential / 46 commercial / 73 urban / 30 hill |
| Allocation-only fingerprint | `fnv1a64:f866d824acfb111f` |

The actual current hill sections S12–S15 have zero legacy buildings. Exact
per-section existing/target counts are exported in the census. Validation and
inspection executed in independent processes and produced identical planned
assignment arrays/fingerprints. This does not verify future spatial layout hashes.

## Automated results

[Complete command log](evidence/local-checks.txt):

- `cargo fmt --all --check` passed.
- `cargo clippy --workspace --locked --all-targets -- -D warnings` passed.
- `cargo test --workspace --locked` passed: **100 tests** (78 client, 11 content,
  11 core), one existing optional soak ignored. The soak was not rerun for this
  additive content/diagnostic change; prior handling evidence is separate.
- `cargo build --locked` and `cargo build --release --locked` passed.
- Offline and debug-physics feature checks passed.
- Both existing track validators and handling validator passed.
- New world validation and real environment census commands passed.
- Four `tools/test_world_inputs.py` and two existing Python audit tests passed.
- Declaration/hash audit passed: 519 dependency declarations, 15 runtime asset
  records, 27 existing research observations. No dependency/lockfile changes.
- README/progress checkbox synchronization and `git diff --check` passed.

New Rust cases cover every section/family/row/archetype budget, exact assignment
repeatability and seed variation, unique IDs, all section-border ownership/blend
limits, invalid versions/IDs/gaps/overlap/counts/dimensions, unknown fields/rules,
wrong track/open topology/seed/length and baked endpoint projection. Existing
controls/camera/combat/collision/ragdoll/remount/grass/boost/race/save regressions
remain in the full suite. Python compares all source CSV/JSON section fields and
all converted names/IDs/quotas/height ranges and the source manifest digests; added
structural tunables are authored.

[Five CLI failure probes](evidence/invalid-cli.txt) returned exit 2 for missing
arguments, mismatched track, missing input file, invalid output directory and a
gap in section stations. No successful validation was inferred from compilation.

## Native results and visual review

[Before release log](evidence/before-native.txt) / [after release log](evidence/after-native.txt)
both passed farm crash → get up/run/remount → city/hills → hill crash/remount →
finish, with four initialized audio playback sinks. The new plan is initialized
once and its measured startup census equals the headless export exactly.
[Actor smoke](evidence/after-actors.txt) passed independent human/quadruped
ragdolls, contacts and bounded bodies. It injects direct hit messages and is not
a manual driving collision test.

Reviewed before/after farm, city and hill views; inspected the post-change F4
preview. Existing world geometry remains and the hill preview shows blue district
edges together with the original centerline/normal/bounds guides:

| View | Before | After |
| --- | --- | --- |
| Farm riding | [image](evidence/before-02-racing.jpg) | [image](evidence/after-02-racing.jpg) |
| City near camera | [image](evidence/before-08-city.jpg) | [image](evidence/after-08-city.jpg) |
| Hill far camera | [image](evidence/before-09-hills.jpg) | [image](evidence/after-09-hills.jpg) |
| F4 overlay | existing baseline geometry | [district/track preview](evidence/after-12-track-inspector.jpg) |

Host: Fedora Linux, NVIDIA RTX 3080, Vulkan, 1440×900 window. Driver reported
missing optional Vulkan extensions and an unmapped controller UUID/default
mapping; neither prevented the native scene checks. Audio sink initialization
and playback state were tested; no subjective listening session is claimed.
Before bounded wall-frame sample: 6.98 ms average / 14.26 ms p95; after: 6.99 ms /
9.44 ms p95. [Before](evidence/before-metrics.json) and
[after](evidence/after-metrics.json) contain ticks/entities/colliders. These short
scripted captures have different dynamic actor histories; they do not establish
an optimization win, a GPU benchmark, a 1080p performance gate, or timing for a
future 247-building layout. No Windows hardware or new full-map atlas verified.

## Open gates

BW-B03–B10: generated ground masks, junction reservations/stubs, real parcels,
247 rooted masses, shared foundation grading, on-foot access, LOD/chunks and
complete-world survey/performance. The source plan's full appendix remains
unfulfilled until those stages pass. Physical controller, external fun/readability
and prior platform gates also remain open. No new repository/dependency/media
adoption; supplied design files are retained as user-provided design evidence.
