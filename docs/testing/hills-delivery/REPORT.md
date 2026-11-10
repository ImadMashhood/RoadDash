# Hills menu and landscape delivery — 2026-11-10

ADR138 covers this follow-up. The implementation preserves the existing city →
hills → branching western town → dunes course, accepted physical rigs and legacy
courses. README is the live acceptance checklist; the gates below do not imply
full-course art, handling, hardware or production-performance approval.

## Implementation

| Request / ID | Delivered behavior and source |
| --- | --- |
| Course selector — UI-COURSE-01 | Prominent Metropolis/Hills course cards, with Valley Run/City Run retained. Existing process-isolated course loading preserves session preferences and profiles; inherited Hills CLI flags no longer override course/online selection. `src/frontend/course.rs`, `course_smoke.rs`, frontend/core flow and online launch integration. |
| Bridge connections — HILLS-08 | Regular 8 m spacing of thick square hangers, connected to edge girders and transverse deck outriggers. Deck-frame hardware placement avoids terminating supports at unrelated canyon height. No low railings; real fall gaps remain. `src/world_gen/hills/bridge.rs` and physical departure regressions. |
| Gentler farms — HILLS-07 | All six farms retain reused Metropolis props/materials. Placement evaluates the full 72 × 192 m parcel, local slopes, relief, barn support and both-road clearance, choosing contour-oriented natural land outside road-cut influence. No cliff-sided terraformed platforms or road-relief flattening. `countryside.rs`, farm height helper in `src/track/hills.rs`. |
| Canyon river — HILLS-11 | Original bounded winding water reach and bank mesh follow resident canyon ground, with animated flow presentation. No new water collider, invisible deck support or fall-physics change. `src/world_gen/hills/river.rs`. |
| Spatial clouds — HILLS-10 | Eight separate volumetric cumulus banks with taller irregular density lobes, shared 48³ texture and at most twelve march steps. Cloud illumination follows the sun. `clouds.rs`, `assets/shaders/hills_clouds.wgsl`. |
| Visible sun — SKY-SUN-01 | Analytic solar disc/halo aligned with directional illumination and camera-relative placement, distinct from the display sky. `src/sky/solar.rs`, `assets/shaders/solar_disc.wgsl`, sky integration. |
| City shadows — SHADOW-RANGE-01 | Metropolis/Hills outer shadow range extends from 150 m to 500 m with four cascades and unchanged two near splits. Legacy maps retain 150 m; bounded override supports same-binary controls. `src/world.rs`. |

## Verification receipts

- `fmt.txt`: `cargo fmt --all --check` passed.
- `clippy.txt`: locked workspace/all-targets Clippy with `-D warnings` passed.
- `tests.txt`: locked workspace tests passed **590 tests**, with one existing
  ignored soak (417 game library, 1 binary, 57 content, 115 core; zero failures).
  Coverage includes whole-parcel farm slope/clearance/support, bridge structure
  and actual departures, river bounds, cloud volume, sun direction, shadow
  policy, course handoff and existing physics/recovery regressions.
- `build.txt`, `build-release.txt`: locked debug and release builds passed.
- `audit.txt`: asset/dependency declaration audit passed: 580 dependency
  declarations, 198 runtime asset records, 27 reference observations.
- `roadmap.txt`: all five roadmap tooling tests passed.
- `menu/screenshots/course-selector-report.json`: native selector verified
  Metropolis → Hills → Metropolis, return to menu, same-course retry, preserved
  credits and live preferences; six screenshots. Legacy handoffs are covered by
  identity/command tests, not a full driven native race.
- **All seven native cases pass**, with exit code 0 and no render errors:

  | Receipt index | Accepted cases | Binary SHA256 |
  | --- | --- | --- |
  | `native-active-active-dynamic-baseline-east-menu-metro-control-west.json` | menu, active-dynamic, west | `96be350e7fb7a4f1d3a83d1a90ceffa43025c43b30f5374d35c15a7d4e3d0e00` |
  | `native-active-baseline-east-metro-control.json` | active, east, baseline, metro-control | `9b9b0d2025eee25c4167086d30b320317a68b215382b55fa627e89a1749e3eeb` |

  The first batch was interrupted with exit 143 during east; that partial case
  is not counted. Its initial active receipt is superseded by the final active
  run after river texture/camera polish. The active evidence directory contains
  the final run, not the initial captures. Other initial accepted cases remain
  identified by their actual binary hash rather than relabeled as final-build
  runs.
- Assisted departure records an 82.54 m ragdoll fall followed by recovery;
  Dynamic records 82.677 m authoritative chassis descent with 39.218 m/s downward
  speed and remains mounted. Neither uses exceptional rescue. West, east,
  baseline-rig and Metropolis recovery receipts each report `[0, 0]` exceptional
  rescues and zero rescue penalties.
- Final active produced 26 captures. Visual review of all six farm parcels found
  no cliff-sided lots and confirmed distinct sun/cumulus and connected bridge
  supports. The corrected river camera shows no terrain clipping; softer water
  ripples resolve the earlier repetitive bright striping. The river remains an
  intentionally bounded, tapered reach.
- [Matched shadow timing](PERFORMANCE.md) completed four clean final-binary
  runs in 150 / 500 / 500 / 150 m order, with clouds constant. Pooled median
  logging-window mean was 21.848 → 21.773 ms; P95 window mean was
  23.544 → 23.814 ms. Differences are within run variation, not a speedup.
  These are window means, not raw-frame percentiles or GPU timestamps. The
  15-view Hills fixture includes its city opening; it is not a full Metropolis
  benchmark. See [comparison.json](comparison.json) for process monitoring,
  hashes, native receipts and sample selection. No cloud timing claim is made.

## Scope, provenance and remaining gates

No dependency changes, retail media or external code intake. Course artwork uses
existing RoadDash captures; water, cloud density and sun are original procedural
presentation. See `PROVENANCE.json`, `docs/third_party/ASSETS.csv` and ADR138.
The declaration audit is not legal clearance or platform certification.

Water follows terrain as a shallow visual reach; swimming, fluid physics and
water collisions are outside scope. Clouds are bounded presentation, not dynamic
weather or an assumed speed improvement. `ROADDASH_HILLS_CLOUDS=0` remains the
fallback. `ROADDASH_SHADOW_DISTANCE_M=150` restores the baseline shadow policy;
valid overrides span 150–600 m. Farther shadows trade outer resolution/work for
coverage; broader city/GPU/hardware performance and human visual acceptance
remain separate gates.

Native Dynamic canyon departure may remain mounted during physical descent;
scripted falls do not prove every combat approach or return from canyon floor.
Full manual race/art approval, network-session coverage, Windows/controller/GPU
hardware certification and sustained 60 FPS remain open. No new major roadmap
part is started. Git integration is handled separately; remote publishing is
currently blocked on credentials. No successful remote push is claimed here.
