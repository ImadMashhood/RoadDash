# Performance pass and UI generation prompts — 2026-11-08

## Implementation

This is a focused PERF-08 development pass, not closure of the target-hardware
performance gate. UI-03 is verified; UI-02 artwork/approval/Bevy integration stays
open. No generated artwork, external assets, engine migration or dependency
changes. No next major roadmap part started.

- Immutable Metropolis chunk collision uses Rapier standalone static colliders,
  avoiding explicit Fixed-body bookkeeping/writeback. Breakable props, actors,
  traffic, parked cars and physical rigs retain their existing body ownership.
- Building sides/roofs batch collision by shared gameplay role and 256m cell
  independently of their render materials. Preserve every triangle, winding,
  position and friction. Road, rideable sidewalk, other ground and farm fence
  roles retain separate collision batches/bounds. Resident envelopes, all render
  meshes, authored detail, atlas/mips, source layout and legacy Valley are intact.
- Settled facade visibility ranges are stable/unbounded; only transitions use
  camera-relative dither ranges. Hidden/unchanged visibility, props and HUD text
  stop producing redundant change notifications. Fade readiness/budgets remain.
- Local JSONL adds backend synchronization, simulation step and writeback spans;
  independent logging-window wall mean; and instantaneous contact-graph,
  manifold, awake-body and joint counts. Census collection runs once per logging
  window. The asynchronous bounded writer and 16 MiB rotation remain unchanged.
- The summary tool filters startup/paused windows and normalizes simulation work
  by actual fixed steps. It explicitly separates rolling metrics, per-render
  work and per-fixed-step work; rejects malformed/nonfinite required fields.
- [28 screen prompts](../../ui/SCREEN_GENERATION_PROMPTS.md) and
  [142 per-image prompts](../../ui/IMAGE_GENERATION_PROMPTS.md) cover all 66 UI
  master families. Subject, composition, quiet UI zones, output/alpha, variants
  and exact replacement filenames are specified. Per-image aspect ratios stay
  source-matched; separate full-page briefs target 3840×2160. Live text, digits,
  bars and focus states remain engine/vector elements. Prompts are not artwork.

Artist downloads: `/home/imad/Documents/ChatGPT/RoadDash-UI-Prompts.zip`, plus the
updated private `RoadDash-UI-References` folder/ZIP. Original references remain
outside Git and runtime; prompt-only ZIP contains no extracted media. The artist
desk links both prompt documents; packaging regenerates/copies them in future.

## Diagnosis and measured limits

The most recent local user run available at task start was
`/tmp/roaddash-performance/performance-2247031.jsonl`, archived as
[user-race.jsonl](user-race.jsonl). After six startup samples and racing-only
filtering, 191 windows show median rolling frame mean **42.69 ms**, median
physics work **32.39 ms per render frame**, and **2.31 fixed steps per render
frame** overall. Weighted physics work is **11.27 ms per fixed step**. Rendering
extract median was 0.67 ms; latest render-schedule CPU/wait median 8.88 ms. These
spans overlap and are not additive GPU timings. This supports investigating
physics/catch-up, not a claim that GPU utilization explains the slowdown.
[Summary and selection](user-summary.json).

The identical scripted native `--metro-survey` visits 13 camera views with the
same live farm racers, urban render load and unchanged 1440×900 window settings.
Run each process separately, with no Rust compilation overlapping the paired
runs. Exclude first four logging windows and non-racing windows. All processes
exited0. [Raw comparison](comparison.json):

| Stage | Median independent window frame mean | Weighted physics / fixed step | Weighted writeback / fixed step |
| --- | ---: | ---: | ---: |
| Render cleanup, before static changes (`paired-before`) |16.83 ms|9.20 ms|0.72 ms|
| Standalone immutable colliders only (`paired-after`) |16.16 ms|8.88 ms|0.22 ms|
| Final, including semantic collision batches (`final`) |14.67 ms|7.36 ms|0.21 ms|

The earlier diagnostic-only baseline (`profile`) has median rolling frame mean
17.80 ms and weighted physics10.11 ms/fixed step. It predates the independent
window metric, so it is not mixed into the independent-window comparison. The
2-worker Rayon exploratory run is also retained; its small/noisy difference
was insufficient to justify changing the existing parallel defaults. Some early
exploratory checks overlapped authoring/compiler activity; they are not used as
controlled speedup evidence.

Final immutable collider batches are **2,529 versus 2,920 source material batches**;
all **2,606,411 collision triangles**, **2,902,405 render triangles**, **19,068
buildings** and **3,065 resident render chunks** are retained. Backend colliders
fall from 4,219 to3,828; 2,920 immutable Fixed-body components are avoided.
Remaining backend bodies total 1,299 in the final survey. End contact-graph pairs
fall from 44,779 to32,817; these are potential/inactive graph edges, not 44,779
actual impacts. Actual active contacts/manifolds must be read separately.
[Final census](final/metro/metro-survey.json).

**The full frame-budget gate is still unmet.** The final survey's last rolling
p95 is 36.11 ms, despite improved median windows; loading/streaming and live full
race/crash spikes remain. This camera survey keeps the player near the start
while visiting urban views, so it does not reproduce the user's entire driven
race at city chainage3777m. Different real-time tick histories, GPU clocks and
other desktop activity limit causal/performance generalization. No raw-frame
p95, GPU timestamp, Windows/controller or broad hardware certificate is claimed.
Re-run the actual lagging route using the new phases before further solver/GPU
changes. Accepted rider feel, 60 Hz, substeps, CCD, joint/muscle limits, mass,
momentum rules, animation timing, controls and save format are unchanged.

## Verification and regression protection

- `cargo fmt --all --check`, strict workspace/all-target Clippy,
  `cargo test --workspace --locked`, `cargo build --locked`, and release build:
  all passed; 367 tests passed and one pre-existing test is ignored. Receipts
  are in this directory, including tests.txt.
- New tests verify exact ordered triangle soup after batching; separate support /
  fence roles; 240-step contact/CCD/drop/support-ray trajectories with explicit
  versus standalone static bodies; and no settled facade change notifications
  when the camera moves, with live fade/hidden behavior retained.
- Nine [Python tests](python-tests.txt): prompt coverage, exact aspect/export /
  identity rules, deterministic rebuild without original files; catch-up
  normalization, startup/paused selection, malformed logs and zero-tick safety.
  Existing 17 UI-kit tests also pass ([receipt](ui-tests.txt)). All 142 image and 15
  component art checkboxes remain open.
- Final native 13-view Metropolis survey exited0: same source/collision/render
  census, both fork routes, prepared authored detail,64 main cars/30 cross cars,
  street props and no disappearing resident envelopes. Actual screenshots under
  final/metro/; view06 was visually inspected for continuous visible facades.
- Native default velocity-muscle route0 smoke exited0: seven racers/seven
  killers, four audio sinks, actual farm/city crashes, two zero-error remounts,
  and results. [Log](recovery-default/native.txt)
  and [metrics](recovery-default/smoke-metrics.json).
- Explicit extended default route0 smoke also exited0 with
  `ROADDASH_VERIFY_STRIKES=1 ROADDASH_VERIFY_CLUB=1 ROADDASH_VERIFY_REVERSE=1`: actual
  buffered strike contact, club pickup/drop and reverse−1.799m/s, followed by both
  recoveries/results. [Log](recovery-extended/native.txt),
  [metrics](recovery-extended/smoke-metrics.json).
- Native eleven-body baseline route1 smoke exited0: both recoveries, zero-error
  pickups/remounts and results. [Log](recovery-control/native.txt),
  [metrics](recovery-control/smoke-metrics.json). No user profile was written;
  smoke persistence is disabled and a separate save-directory path was supplied.
- Paired headless ragdoll lab: all 18 cases finite across control/new paths;
  [JSON](ragdoll-lab.json) and [log](ragdoll-lab.txt). This is regression coverage,
  not new ragdoll tuning or full-race performance approval.
- Five roadmap tests, progress sync, asset/dependency audit and whitespace checks
  are recorded alongside the runtime checks. Runtime asset record count stays 106.

## Files, decisions, research and tracking

Runtime files: `src/diagnostics.rs`, `src/diagnostics/perf.rs`, `src/props.rs`,
`src/ui.rs`, `src/world_gen/metropolis/{mod,geometry,facades}.rs`. Tools:
`summarize_performance.py`, `build_ui_prompts.py`, `test_performance_ui_prompts.py`
and artist packaging hook in `build_ui_kit.py`. Artist outputs/index/guide,
README/PROGRESS/placement, provenance, known issues, decisions, development log
and testing matrix updated together. No files removed.

ADR089 records alternatives, exact-geometry/static semantics, rollback and the
separate art handoff. Only installed primary source for existing R05 Bevy 0.17.3 /
R10 bevy_rapier3d 0.32.0 and locked Rapier 0.31.0 was investigated; existing dependency
license entries stand. No new repository, unlicensed source or media incorporated.
Original prompt JSON is a non-runtime project-MIT authoring record.

New verified UI-03 originates in Part15 and is listed in completed Part00.
PERF-08 remains partial/open, including actual driven-route/GPU/hardware/frame
budget gates. Counts: 210 verified / 242 open / 452 IDs. UI-02 and every production
art task remain open. Next: create a consistent main-menu master using the
screen prompt, while collecting a new full-race log from this performance build.
Branch `codex/performance-ui-prompts`; local commit only, no push/merge requested.

## Run and capture the actual slow race

From the repository:

```bash
ROADDASH_LOG_DIR="$PWD/performance-logs" cargo run --release --locked
```

Drive the farm-to-city route until the slowdown, then exit through the window
close button so the process ends. The terminal prints the exact JSONL path.
Summarize that file, replacing PID with the printed number:

```bash
python3 tools/summarize_performance.py performance-logs/performance-PID.jsonl --skip-samples 6 --racing-only
```

Keep the raw log: sync/step/writeback, backend counts, position and fixed-step
multiplication are more useful than a screenshot of aggregate CPU/GPU usage.
Do not run another game/build while measuring. PERF logging remains enabled by
default; ROADDASH_PERF_LOG=0 disables it for a later overhead comparison.
