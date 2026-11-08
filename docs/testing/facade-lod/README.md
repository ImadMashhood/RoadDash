# Facade LOD identity and ready-scene transitions

2026-11-07; `codex/facade-lod-identity`, based on velocity/facade fix 971bf35.
The user explicitly requested finishing facade LOD before Part05 textures.

## Implementation

Valley generic windows/trim are replaced with four batched face proxies per lot,
projected from the actual authored 3D storey triangles, depth, UVs, base color,
normal and metallic/roughness materials. All 128 designs / 383 non-empty bands are
covered. Close geometry remains the existing authored GLBs. Window/door/trim and
color identity no longer switches to a generic pattern when the detail budget
or distance changes. Each face retains its proxy until its ready 3D face covers
it; complementary masks handle both face changes and LOD replacements.

The atlas contains explicit gamma-correct albedo, renormalized normal and linear
packed PBR mips, with a guard at every shipped level. It is prepared during loading
and stays resident. The bounds are 128 MiB for composed maps and 250,000 far triangles;
actual values are 134,086,656 bytes / 226,956 triangles / 2,628 proxy meshes over 657 lots.
These are separate from the unchanged 750-panel / one-million-triangle near-detail caps.
LOD distance now measures the nearest building face, not its lot center.
Settled fallback coverage is cached instead of rebuilding cosmetic state each frame.

Optional City Run keeps an old ready scene while preparing its replacement,
including actual instance, material and tangent readiness. Incoming/outgoing
meshes crossfade at the final render camera. High-detail resources are pinned
while referenced and capped at 48 entries; only eight prepared/retiring groups exist at
once. Initial scene instances must be ready before the menu. The standalone city
viewer shares this path. Structural lots, collision, navigation, saves, controls,
selected velocity muscles, anatomy/masses and dependencies remain unchanged.

## Test and review commands

```sh
cargo run -- --lod-survey
ROADDASH_TRACK=city cargo run -- --lod-survey
cargo run -- --smoke-test
ROADDASH_TRACK=city cargo run -- --smoke-test
cargo run -- --world-survey
python3 tools/verify_facade_lod.py
```

LOD surveys use real native rendering with forward/reverse sweeps and a Valley
all-face orbit; they disable progression writes. JSON and captures are saved
under screenshots/lod-valley and screenshots/lod-city. Inspect the JPGs and
reports alongside this file. These are geometry/source/transition checks, not a
claim of complete production art, perfect parallax or universally invisible LOD.

## Executed evidence

Formatting, strict Clippy, locked workspace tests and build pass: 263 tests, one existing
ignored. Native Valley and City crash/recovery/remount/finish smokes pass. The city
viewer capture run passes. The moving-camera fixtures exercise 53 / 389 direct
crossfades initially; final repeat receipts take precedence where repeated.
City maximum detail cache 48 and transition groups 8. Source hashes, 383 unique tiles,
all 15 mip images/guards, actual proxy meshes and displayed-budget assertions pass.
A face-mask test checks every old/new face combination against all 16 dither pixels.
The complete world inspection and selected-ragdoll outcome comparisons are recorded
separately; they protect physical/world regressions rather than infer visual quality.

The initial full-world survey completed 15 sections / 657 roots and validated actual
geometry, but its 1080p city p95 was 18.45385 ms against 16.7 ms. A later run overlapped
local test compilation and is retained as contaminated evidence, not a controlled
performance comparison. Final isolated receipt/limitations are appended after it runs.

LOD-04 records verified technical integration. LOD-03 remains open for the user's
moving-camera visual acceptance and longer streaming review. Projection can show
flatness/parallax differences at grazing angles; genuine near geometry silhouettes
and dither grain remain. Broad performance/hardware/art gates stay open. Textures
(Part05) are deferred. No merge is implied.

## Final receipts

Final isolated 1080p world survey: p95 **16.80066 ms**, target 16.7 ms still
missed; process deliberately exits 1. See native-world-survey-isolated.json for
the exact value and current geometry/camera census. It completed all 15 sections
and geometry/support assertions. No claim of production 60 FPS or GPU timing.
Initial 18.45385 ms and overlapped 49.09 ms samples remain as historical evidence.

Final Valley moving-camera repeat: **54 direct crossfades**, max 12 cached
detail resources, 128 displayed near panels and 150,434 near triangles.
City: **389 direct crossfades**, max 48 cached resources and eight concurrent
transition groups. Both fixtures exited 0.

Both game smoke tests passed with two crashes, walking recovery/remount and race
finish. The standalone city viewer capture also exited 0. The actual new proxy
meshes/triangle counts and texture budget were validated by the native survey.
All 18 selected-velocity ragdoll outcomes match exactly except timing; the
structural allocation and geometry fingerprints are unchanged.

263 workspace tests passed, one pre-existing test ignored. Formatting, strict
Clippy, build, source/mip verification, asset/dependency audit and five roadmap
tests pass. LOD-04 is verified. LOD-03 remains open for user visual acceptance
and longer streaming review; Part05 textures remain deferred.

New deferred task MAT-03 (Part05) requires rebaking proxies against delivered
close material data when textures change. This prevents the next texture pass
from restoring the identity pop-in corrected here.
