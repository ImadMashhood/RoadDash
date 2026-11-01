# Context-first animation correction — 2026-11-01

Branch: codex/animation-sequence-context. V1 action labels are superseded because
its “documented-helper-anchor” evidence was an inspection-based approximation.
This session traced source selectors to EXE descriptors to CANS camera cells and
RRAN/BOB graphic indices before drafting replacements. No runtime art/physics,
Cargo dependency or gameplay code changes.

## Metadata evidence

Independent bounded Python PE/resource/CANS reader, with exact retail hash
rejection and explicit observed layout facts. The external pinned reference
library was compiled outside RoadDash for comparison. 194 descriptor counts,
logical durations, controls, flags and rear selections agree across both banks.
[Receipt](../animation/context/validation.json). No reconstructed C++ imported.
This is not native original executable validation.184 populated HIBOB dimensions
match2x RRAN,33 populated LOCOP dimensions match directly; the two far HIBOB
sprites have documented rounding differences. Empty BOB slots have native graphics
placeholders and are not treated as missing temporal frames.

104 bank/code slots:48 rider sequences /159 descriptors and11 police sequences /
35 descriptors available. All326 rider/45 police slots retain usage/empty status.
Slot64 is populated but absent from recovered CANS groups. Source phase order,
mirrors, reused images, camera variants, controls/loop target, fallbacks and timing
counters are stored. Actual punch, kick, chain/club/backhand mappings have explicit
regression expectations. Original pixels/palettes/executable are not in the repo.

## Authoring evidence

138 source-linked human poses across41 sequences,16 bike-only layouts. Five empty
code43 references are labeled without invented art.138 single-frame constant
Blender Actions round-trip independently,27,600 channels, maximum matrix error
5.364418029785156e-7; zero physics bodies. [Art receipt](../../art/animation/context/validation.json).
Headless authoring CLI checks connected rigid frames, elbow/knee limits and5mm
held-grip tolerance. Props/whole-bike transforms and fist shape keys have explicit
assembly recipes and a complete loader; bone-only Action assets are insufficient.
All files remain DRAFT: no user art approval or runtime motion claims.

## Commands / results

- `python3 tools/read_animation_context.py <local retail> --output /tmp/context-redecoded.json`, then byte comparison: reproducible.
- `python3 tools/catalog_animation_context.py`: reproducible context catalog.
- `python3 -m unittest discover -s tools -p 'test_*.py'`:19 tests pass, including5 new tests for PE bounds, synthetic CANS phase/view separation, resource truncation, combat/mirror/repetition/alias semantics and exact pose links.
- Blender 4.5.1 build/load/independent verify:138 human /16 bike layouts, all renders and138 Actions verified.
- `target/debug/road-dash --validate-animation art/animation/context/manifest.json`:138 poses /41 use cases pass.

Format, locked workspace Clippy with warnings denied,140 regular Rust tests
(one optional soak ignored) and locked build pass. Exact commands/results:
[command receipts](animation-context-checks.json). Tracker/hash/provenance checks
also pass. A native gameplay rerun
is not required for isolated authoring; no native-original/Windows/performance/CI
success is implied. Existing model-physics differential tests preserve the passive
baseline. Source/recreation strips and browser gallery are reviewed as drafts.

## Open

Code7 and some mount/wreck aliases have unresolved callers; fallback availability
is explicit. Full collision/cloth/deformation, hand/finger detail and support
contacts are not established by a valid rigid skeleton. Get-up/mount/crash and
weapon-hand silhouettes need further art review and multi-angle approval. Police
poses are not authored. ANIM-06 and existing Part02 full-art/approval gates remain
open. Part03 interpolation/terrain/active recovery has not started.
