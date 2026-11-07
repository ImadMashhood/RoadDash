# Velocity selection and facade pop-in correction

2026-11-07; branch `codex/velocity-popin`, based on velocity candidate `3510cb3`.

The user selected velocity muscles. The native-motor comparison candidate is
excluded. Native anatomical constraints and passive tissue resistance still hold
the body together; active control uses bounded angular impulses and a finite
supported-ground guide. All 18 lab outcomes exactly preserve the selected
candidate, excluding wall time. The worst prone fixture still turns only about
7 degrees: this selection does not complete the full roll-recovery requirement.
Historical comparison branches remain unchanged as evidence.

Run from this worktree: `cargo run`, or `cargo run -- --crash-comparison` for
crash controls. Enter starts; F10 sets speed; F8 tests combat release; F9 tests
direct release. The overlay identifies velocity without a candidate letter.
Comparison mode disables progression writes.

## Facade correction

- Crossfade directly between ready authored models instead of fading through the
  generic fallback every time the LOD or selected faces change.
- Use opposing shader dither branches so the outgoing and incoming patterns
  complement each other. Initial hidden meshes start with zero coverage.
- Recenter temporal ranges after the final render camera and mesh transforms
  propagate, before visibility checks. No meshes spawn in PostUpdate.
- Wait for incoming materials and tangents. Request detail within 130 metres,
  ahead of the 18/55-metre detail thresholds.
- Add four-metre face-selection hysteresis. Pin referenced assets and strictly
  cap the detail cache at 48 entries.
- Count both displayed groups against the existing 750-panel/one-million-triangle
  limits. Reserve 25% planning headroom for overlap; retain the old model when an
  upgrade cannot fit. Collision, lot IDs, support and navigation remain unchanged.

## Executed verification

- Formatting, strict workspace Clippy, locked tests and build: PASS. **258 tests
  passed, one pre-existing test ignored.** Logs: fmt/clippy/tests/build.txt.
- Shader-equivalent complementary coverage across all 16 dither pixels: PASS.
  Actual Bevy PostUpdate camera/transform regression: PASS.
- Native `--smoke-test`: PASS. Two crash/remount cycles, farms/city/hills, finish
  and four initialized audio sinks. Logs: native-smoke.txt/native-smoke-survey.json.
- Paired ragdoll lab: 18 finite scenarios; nine legacy-control rows identical
  excluding timing. Maximum new-rig anchor error: 0.003920718 metres. Fifteen
  simultaneous riders settle with 240 bodies sleeping and 15 resting ragdolls.
- `--inspect-world`: PASS. Structural allocation and geometry fingerprints match
  the existing architecture report. See inspection-verification.json. The complete
  duplicate output remains locally at /tmp/velocity-popin-world-inspection.json.
- Native world survey: all 15 sections and 657 logical roots; geometry/collision
  and actual-mesh assertions passed. Nine direct crossfades, four budget holds,
  731 displayed panels, 731,523 triangles, 48 cached detail levels, 3,717 actual
  mesh entities, and 63 hidden prepared panels. See native-survey.json and JPGs.
- **Survey performance gate FAILED:** 1080p city p95 was 16.94174 ms against
  16.7 ms, so the survey deliberately exited 1. Average wall-frame time was
  14.05263 ms. This is not GPU timing or a controlled before/after comparison.
- Asset/dependency audit and all five roadmap tests: PASS. No dependency or
  runtime media added. Installed Bevy 0.17.3 visibility/render shaders informed
  the correction; no reference source was copied.

RAG-12 and LOD-02 are verified. LOD-03 remains open for distant facade identity,
optional City Run's independent renderer, and extended moving-camera/user review.
Genuine silhouette differences at initial activation and dither grain remain.
Existing production performance and full biomechanical roll gates stay open.
