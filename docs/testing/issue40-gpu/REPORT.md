# Issue 40 candidate D: one-wave grass shader

Status: independent draft candidate, **not selected**. This is a presentation-only
shader experiment on base `e2a3191876d57f266b1e36c967a48c9e8e5667ae`.
The first clean repeated comparison is too small to establish a frame-budget gain.
PERF-08, RFU-PERF-04 and P16-003 remain open. No new requirement ID, checklist
closure or milestone move is proposed; README and PROGRESS carry the same partial
link and counts remain unchanged.

## Change and source

The original farm grass vertex shader evaluates `sin(phase)` for X sway and
`cos(0.73*phase)` for Z sway. Candidate D reuses one `sin(phase)` value for both
axes. `uv.y²` still fixes each root and keeps the maximum tip displacement at
`sqrt(0.07² + (0.07*0.45)²) = 0.0768m`, within the existing 0.08m culling
expansion. Six curved blades, opaque PBR material, shadow-caster exclusion,
65–110m visibility range, batching, world geometry and collision are unchanged.
The motion direction changes; native visual review is required before adoption.

No dependency, physics, save, input, audio, facade material or atlas change is
included. `docs/third_party/ASSETS.csv` records the new shader hash. The code
remains original RoadDash work; no external shader or media was imported.

## Matched native warm comparison

The frozen baseline binary SHA256 is
`682b328291655b18b812c2182ed5366bdd1bdf5bf948ca6731fee64863911536`.
Candidate binary SHA256 is
`e7384423dc37afea57aa4b5f20836b61170856753ecbea9e75d045ddf7ab0e63`.
Both are release builds from the same source commit. Candidate workspace packages
were cleaned and rebuilt in a private target so `CARGO_MANIFEST_DIR` points to
this checkout's shader. Their adjacent provenance JSON records source and build.
The candidate manifest captured the tracked diff at binary freeze; the harness
zombie exclusion and this documentation were edited afterward. The shader hash
in the manifest matches the asset used for every candidate run, and neither
later edit is compiled game code or a runtime asset.

The same `tools/run_performance_profiles.py` harness used seed 25, single fixed
executor, eight loading jobs, `--gpu`, identical survey flags and fresh per-run
settings/saves. A verified Baloo extractor was suspended for each run and restored.
No game/compiler/indexer contaminant was detected. The baseline is the canonical
`/home/imad/roaddash-issue40/baseline-warm-final`; candidate raw data is
`/home/imad/roaddash-issue40/gpu-warm-final`. Four cases each cover two Hills and
two Metropolis surveys. Baseline cold-asset controls are in
`/home/imad/roaddash-issue40/baseline-cold-final`; this shader-only candidate did
not repeat cold asset loading. The launch attempt in `gpu-warm` stopped before
timing because the older local harness counted a defunct Cargo process. The local
harness now has the same zombie exclusion as the canonical baseline harness.
Portable warm receipts, per-case summaries and compressed raw JSONL logs are
in `evidence/{baseline,candidate}/`.

| Course | Raw gameplay p95, baseline → candidate (ms) | Required assets ready (s) | Raw p99 (ms) | Raw max (ms) | Opaque GPU pass latest-sample median (ms) |
| --- | ---: | ---: | ---: | ---: | ---: |
| Hills | 18.904 → 18.807 | 20.356 → 20.542 | 20.979 → 20.830 | 30.568 → 38.057 | 0.991 → 1.039 |
| Metropolis | 24.197 → 23.976 | 7.080 → 6.778 | 26.440 → 26.451 | 31.968 → 29.466 | 2.205 → 1.853 |

Values are medians of the two per-run summaries, not a pooled percentile. The
predeclared comparison weights each map's p95 ratio twice and its readiness ratio
once: geometric candidate/baseline score `0.98955` (about 1.0% lower). Neither
map has a >5% repeated p95 regression. The small wall-time change overlaps run
variation. Hills opaque GPU median rises about 4.8%; Metropolis falls about
16.0%, but those are asynchronous pass samples once per logging window, not
full-frame GPU p95 or proof of shader attribution. Hills' larger single-frame
maximum and Metropolis RSS variation need regression review. All four candidate
surveys exit 0 with 0 detected contaminants; no 16.7ms p95 gate passes.

## Visual check and outstanding decision

Both frozen binaries passed an untimed `--metro-survey` with
`ROADDASH_LANDSCAPE_VIEWS=1`. The same frame 135/185 captures are retained in
`visual/`: `baseline-135.png`, `baseline-185.png`, `candidate-135.png` and
`candidate-185.png`. Direct inspection finds the same rooted grass coverage,
road shoulder and fence clearance, with subtly different blade lean. Neither
run logged a shader error or panic. Another release build overlapped these
visual-only runs, so none of their wall times are compared. This is bounded
visual inspection, not human art approval or a frame-by-frame motion guarantee.

`cargo fmt --all --check`, strict all-target workspace Clippy, locked workspace
tests (617 passed, two existing ignored), `cargo build --locked`, release build,
`python3 tools/audit.py` (580 dependency declarations, 198 runtime asset records,
28 research observations), five roadmap tests, three performance harness tests
and `git diff --check` pass. The first workspace test attempt was interrupted
during link to release another candidate's timing slot; the complete rerun
passed. The native two-map surveys and paired grass captures exit 0. This
verifies the candidate build and bounded scenarios only.

The root four-candidate comparison will decide whether to retain, revise or
reject this branch. Long-race combat, damaged course, Windows and other GPU
hardware remain untested. Draft [PR #45](https://github.com/ImadMashhood/RoadDash/pull/45)
contains the exact local tree. HTTPS Git push waited for a missing username and
SSH lacked a key, so the connected GitHub account uploaded Git blobs/tree/commit
and advanced `codex/issue-40-gpu`. The first remote commit has the exact local
tree SHA `6bea97e1e18528726672c8a21c91772e8ff32750`; its commit SHA differs
because the connector authored a separate commit object. No merge is claimed.
