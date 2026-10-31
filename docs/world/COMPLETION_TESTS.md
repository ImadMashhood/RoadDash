# Structural world completion receipt — 2026-10-30

Scope: BW-B03–B10 and discovered verification tasks BW-X01–X03.
Source baseline: foundation `0a2279c` / PR #6. Original B00–B02 receipts and
source inputs remain unchanged. Code is integrated into the normal Valley startup.

## Measured world and reproducibility

[Summary](completion/summary.json), [full generated census](completion/census.json.gz),
[independent validation](completion/validation.json.gz),
[native startup census](completion/native-census.json.gz),
[comparison receipt](completion/comparison.json), [building CSV](completion/buildings.csv),
[actual footprint plan](completion/valley-plan.svg).
Large JSON evidence is gzip-compressed after comparison; Python's `gzip.open`
or `gzip -dc` reads it. Normal CLI commands emit ordinary JSON.

| Requirement | Evidence |
| --- | --- |
| Original open course preserved | 6847.307617 m, eight grid spawns, nine race gates, seed 17031; existing route/gate tests pass |
| Exact placed count | 247 real building roots/colliders, zero legacy building masses |
| Rows / registry / sections | 177 near + 70 secondary; all 19 archetypes and 15 sections |
| Family budgets | 35 farm / 63 residential / 46 commercial / 73 urban / 30 hill |
| Ground / street organization | 42 field parcels, nine junctions, 16 block faces, sidewalks and secondary alleys |
| Terrain-aware placement | 51 relocated candidates, 43 explicitly retained level pads; no quota skipped |
| Foundation support | Every expanded footprint corner raycasts onto its actual datum within 0.035 m; exact pad cuts/planes |
| Whole-lot clearance | Every lot validated against all road/junction reservations and other lots; corrupt layout rejected |
| Walking access | 247 structural escape routes exported; longest sampled 3D route 100.56 m; three actual capsules traverse farm, city alley and hill routes |
| Ground continuity | Full-route road rays, crossing-stub support, internal terrain seam rays, and existing bank-clearance suite pass |
| Determinism | Two generated layouts/scenes compare exactly; independent CLI and native reports agree exactly |
| Logical geometry fingerprint | `fnv1a64:31b6850d96707a1d` |
| Measured building fingerprint | `fnv1a64:3c7df42c929474f7` |
| Environment-only scene census | 4453 mesh entities, 2156 collider entities, 86 road / 160 terrain chunks |
| Recorded geometry counters | 948279 render / 809199 collision triangles in ribbon/terrain counters, including visual terrain LOD; not total building/surface triangles or draw calls |

The original environment measured 198 masses and 7298 mesh entities; current
mesh-entity count is lower even with near/far variants. This is not a draw-call
benchmark. Physical collision remains resident when presentation changes LOD.
Fingerprints cover deterministic logical records/settings, not shader pixels or
cross-platform floating-point lockstep. Source hashes/provenance are separate.

## Automated verification

[Exact command log](completion/local-checks.txt):

- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --locked --all-targets -- -D warnings`: passed.
- `cargo test --workspace --locked`: **106 passed** (83 client, 12 content,
  11 core), one optional soak initially ignored.
- Explicit `ten_minute_aggressive` soak: **passed**, 600 simulated seconds.
- Locked debug/release builds and offline/debug-physics feature checks: passed.
- Both existing track validators, handling validator, full world validation and
  real-scene inspection: passed.
- Four source-input Python checks and two existing audit tests: passed.
- Footprint SVG exporter and final relative-link/roadmap checks: passed.
- Declaration/hash audit: 519 dependency declarations, 18 runtime asset records,
  27 historical research observations; no new dependency or lockfile change.

New coverage includes pure construction authoring without Bevy/GPU, invalid
junctions/sampling dimensions, exact layout/quotas, corrupted overlaps, foundation
support, chunk seams, real capsule movement and rescue behavior. Rescue testing
verifies that motion/routes clear and a visible five-second penalty occurs once.
The existing controls/combat/traffic/crash/momentum/remount/camera/grass/boost/
race/save suites remain in the full run.

An atlas review exposed an LOD origin error during development. The
[rejected pre-fix atlas](completion/lod-origin-before.jpg) is retained as a bug
receipt, not completion evidence. Terrain now uses local vertices with identical
near/far tile origins; building variants use identical root origins. Regression
assertions inspect all 160 tile pairs, their normalized vertices and all building
children. Final visual and performance evidence below uses the corrected meshes.

## Native scenes and performance

[Final 15-section atlas](completion/section-atlas.jpg), individual `S01.jpg` through
`S15.jpg`, [city performance view](completion/city-benchmark.jpg),
[survey log](completion/native-survey.txt) and [metrics](completion/native-survey.json).
All fifteen views were reviewed with continuous ground and staged rural → city →
hill massing. The SVG footprint plan was rendered and reviewed as well.

The final survey recorded actual **1920×1080** pixels, 247 logical building roots,
and a warmed city **10.28 ms p95** wall-frame sample against the proposed ≤16.7 ms
target. Near/far visible-part counts are recorded. Hardware: Fedora Linux,
NVIDIA RTX 3080 / Vulkan; existing rendering, no new production quality preset.

The survey uses AutoNoVsync and continuous updates while focused or unfocused.
This avoids Bevy's 60 Hz background idle timer; normal play retains its existing
power policy. Earlier requested-size/idle-capped and pre-origin-fix measurements
were rejected. This final bounded wall sample is not GPU timestamp profiling,
a universal hardware guarantee, or proof of a future detailed-art configuration.
The workstation was not isolated from other applications.

[Race smoke log](completion/race-smoke.txt) passed farm crash → get up/run/remount
→ city/hills → hill crash/remount → finish; four audio playback sinks initialized.
[Race metrics](completion/race-metrics.json) and `race-*.jpg` preserve that sequence.
[Actor smoke](completion/actor-smoke.txt) passed independent human/quadruped
ragdolls, solid contacts and bounded bodies. The actor fixture injects direct
hits and is not a manual driving collision test. These scripted checks do not
claim a complete human playtest or subjective listening/controller acceptance.

The driver reported missing optional Vulkan extensions, default mapping for an
unmapped controller and a monitor-selection fallback at creation. Actual full
resolution and native checks succeeded; no controller mapping certification is
inferred.

## Platform / final gate

Local implementation, automated, geometry, native survey and performance gates
are satisfied. The updated Ubuntu/Windows CI run must finish before BW-B10 is
checked; its result will be recorded here after publishing the verified source.
Existing Windows GPU/controller/manual fun gates in other development parts
remain separate. No future major development part was started.
