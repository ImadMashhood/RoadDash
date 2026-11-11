# Metropolis atmosphere — issue30 / ADR145

Status: combined sand/racer-AI source passed locked checks and native fixture
review; publication and user visual acceptance remain pending. RFU-SKY-01..04
remain open. No GPU/full-course60FPS or broad hardware claim.

Audit: ADR138 already installs eight world-space cumulus volumes in Metropolis
and Hills. The bird/dust spawn function returned early outside Hills, so Metro
had no birds. Reuse both systems; do not duplicate the atmosphere. Default cloud
appearance is retained pending review. The shared 48³ R8 texture remains110592
bytes, eight banks and twelve maximum steps; low uses six steps, off the prior
baked display sky. The sun/exposure/IBL/shadows remain unchanged.

Metropolis now has six four-bird flocks, shared original body/wing meshes and a
single feather material. Six roof-clear origins are computed once at startup across the rural approach, city entry, two urban streets, City Hall grounds and fountain park;
whole47m orbits clear nearby resident roof envelopes. Relative phases keep groups
coherent; staggered flap/glide cycles avoid synchronized wing motion. Hills keeps
its old stations and flap cadence. Per-mesh Bevy VisibilityRange fades450–650m
using final propagated camera origins.24 analytic poses run independently of
camera parenting/teleports. Hills alone retains96dustpuffs; no Metro dust assets
or extra ground-contact queries. Pause/restart reuse the existing visual state.

Controls: ROADDASH_CLOUDS=full|low|off, with full default; historical
ROADDASH_HILLS_CLOUDS=0 remains a fallback alias. ROADDASH_BIRDS=0 disables bird
allocation independently. Startup-only settings follow the existing process
relaunch course lifecycle; saves and authoritative simulation are unaffected.

Verification fixture: `--atmosphere-survey`, optionally `--hills`. The fixture
selects a1440×900 window with AutoNoVsync after loading. It moves along rural,
intermediate and urban stretches in paired riding-height/raised-sightline views.
Six captures are stored under screenshots/atmosphere-0..5.png. Timing mode
ROADDASH_ATMOSPHERE_PERF=1 suppresses screenshots and reports1080wall-frame
intervals after180warmupframes, plus cloud/bird budgets. Existing performance
JSONL records provide load/readiness and process-memory context. Use matched
quiet release runs with identical settings and independent bird/cloud switches;
wall-frame time is not GPU time. Inspect actual views before accepting visuals.

Source files: world spawn, shared Hills atmosphere/clouds, existing WGSL shader,
opt-in survey and lib/main registration. No dependency, physical rig, collision,
world envelope/facade atlas, player control or timestep changes. Source rights
and version notes are in PROVENANCE.md.

Placement review: the first six Metropolis captures showed birds over the rural
approach but none apparent in urban views. Origins now include two urban street
approaches and the authored City Hall/fountain park openings, with the same
roof-clear orbit and 24-bird cap. Updated native captures remain the visual gate.

## Combined source verification (2026-11-11)

Base `0572053e` (racer AI and sand integrated). `cargo fmt --all --check`,
`cargo clippy --workspace --locked --all-targets -- -D warnings`,
`cargo test --workspace --locked` (661 passed, two explicitly ignored),
`cargo build --locked`, `cargo build --release --locked`,
`python3 tools/audit.py`, `python3 tools/sync_progress.py --check` and
`python3 tools/test_roadmap.py` passed. Test log: `checks/workspace-tests.log`.
Release binary SHA-256: `93ab384bc93f72fd367b464e574201e93787351d38e0686739ca61e8f37aa8cd`.

Final Metropolis native fixture saved six captures, 24 birds and eight full
cloud banks. [Rural view](native-final-metro/atmosphere-0.png),
[raised urban street 1](native-final-metro/atmosphere-3.png), and
[raised urban street 2](native-final-metro/atmosphere-5.png) show the bounded
flocks and spatial clouds; [JSON](native-final-metro/atmosphere.json) records
1080 moving-camera samples. Low street-canyon views remain roof-occluded, so
this is technical visibility evidence, not final user art acceptance. Hills
control saved six captures and its existing birds/clouds remain visible:
[approach](native-f28-hills/atmosphere-0.png),
[town](native-f28-hills/atmosphere-3.png),
[desert](native-f28-hills/atmosphere-5.png),
[JSON](native-f28-hills/atmosphere.json). The Hills visual receipt predates
sand integration; combined-source Hills quality modes below passed.

### Matched local release comparison

Eight single-pass 1440×900 native runs used the same moving camera and
AutoNoVsync, each with 180 warmup frames and 1080 measured wall-frame
intervals. `ROADDASH_ATMOSPHERE_PERF=1` suppressed screenshots.
`/usr/bin/time -v` recorded whole-process elapsed time and peak RSS, including
loading; [per-run JSON, logs, time and summary](performance/comparison.json)
retain the measurements.

| Map | Mode | Birds | Cloud banks/steps | Median ms | P95 ms | Whole process s | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Metropolis | full | 24 | 8/12 | 23.016 | 28.374 | 37.595 | 6006 |
| Metropolis | low | 24 | 8/6 | 23.299 | 29.188 | 38.000 | 5996 |
| Metropolis | off | 24 | 0/0 | 23.321 | 28.072 | 37.645 | 6007 |
| Metropolis | birds off, clouds full | 0 | 8/12 | 23.284 | 28.745 | 37.690 | 6015 |
| Hills | full | 24 | 8/12 | 16.337 | 17.973 | 42.140 | 2159 |
| Hills | low | 24 | 8/6 | 16.504 | 18.268 | 42.394 | 2179 |
| Hills | off | 24 | 0/0 | 16.404 | 18.077 | 42.342 | 2162 |
| Hills | birds off, clouds full | 0 | 8/12 | 16.405 | 18.136 | 42.298 | 2149 |

These single-run differences are mixed and small relative to run variation;
they do not establish a frame-speed or load improvement. Metropolis remains
over the 16.7 ms frame target in this fixture. This is CPU wall timing, not
GPU timestamps or a full-race/hardware profile. First readiness samples in
`performance/comparison.json` are simulation-clock sample times, not wall
loading durations. No Windows or broad GPU verification was run.
