# Animation A — reference/keyframe authoring evidence

Date: 2026-10-31 (packaging completed 2026-11-01). Branch:
`codex/animation-reference-keyframes`. User requested the first animation stage,
without waiting for previous PR CI; no merge or CI success is claimed here.

## Deliverable and scope

247 numbered **draft static candidates**, 27 mapped use cases; 247 primary renders,
57 supplementary views across 19 selected poses, thirteen contact sheets and an
offline filterable gallery. 130 earlier mounted candidates retain their IDs;
18 running-view slots share six candidate phases, rather than being eighteen
recovered timed frames. 99 additional designs cover locomotion, terrain stances,
reactions, recovery, seated crash/knockoff proposals and a cow review proxy.
Source references, entry/exit states, contacts, confidence and unknown durations
are explicit. Source GLBs, runtime art, physics and dependencies are unchanged.

## Commands actually run

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | pass |
| `cargo test --workspace --locked -- --nocapture` | 140 regular tests pass; one optional soak ignored |
| `cargo build --locked` | pass |
| `target/debug/road-dash --validate-animation art/animation/manifest.json` | pass, 247 poses / 27 use cases |
| `python3 -m unittest discover -s tools -p 'test_*.py'` | 14 tests pass |
| Blender 4.5.1 `tools/verify_animation_library.py` | all 247 pose JSON reloads pass |

Native build/checks used the existing host shim
`PKG_CONFIG_PATH=/tmp/roaddash-native/lib/pkgconfig`; dependencies remain locked.
The four new rr_content tests reject malformed rigs/transforms/contact claims,
unknown payload fields, cycles and invalid catalog links. Three new Python tests
cover synthetic DAT parsing/truncation/duplicates, reference confidence/no-timing
metadata and catalog/render/semantic coverage. No proprietary test fixture added.

The independent Blender reload compares 247 poses, 247 one-frame Actions and
50,670 curve channels. Maximum matrix error: **4.76837158203125e-7**. Every key
is at frame one with constant interpolation. No rigid bodies exist in the
library. [Machine-readable receipt](../../art/animation/round-trip-validation.json).
The existing isolated model-physics regression still reports eleven bodies,
180 ticks and **zero maximum positional difference**. No native gameplay smoke
was rerun for this authoring-only stage; earlier collision/native receipts remain
in COLLISION_RESPONSE.md. No Windows, performance or final-art acceptance claim.

## Visual and research review

Viewed the local original rider/police sheets; only numeric inventories and
confidence notes are distributed. Reviewed the generated overview, running
flight side view, recovery draft and quadruped proxy. Browser-tested eight-pose
run filtering, pose detail, held/released contacts and side-view switching.
The gallery dialog shows full pose metadata and marks every pose DRAFT.
Measured and corrected three inherited mounting knees plus two grip reaches;
held hand/foot contact errors are bounded by the 5 mm authoring validator.

Five pinned animation/IK/ragdoll source repositories were compared; license,
compatibility and actual-use decisions are in third_party/REPOSITORIES.md and
research/ANIMATION_AND_RAGDOLL.md. No source or external assets incorporated.

## Open gates

V2-02-001/002/003/005/006/007/008 remain incomplete: full specialist coverage,
sequence timing/ambiguous labels, measured gameplay gait study, finger grips,
clothing/intersection/deformation review and user approval. In particular,
get-up-kneel/sit are preliminary crouch layouts, not approved supported get-ups;
cow poses are proxy art. ANIM-02/03/04 track these gaps with acceptance criteria.
Part 03 playback/blending/terrain IK/active recovery has not started. No static
vault design authorizes physical launch energy or changes to accepted ragdolls.
