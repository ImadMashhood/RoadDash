# Issue 25 performance and loading evidence (ADR140)

Status: local paired candidates measured; repeat/native recovery verification pending. RFU-PERF-01..05
remain open. This evidence does not certify sustained 60 FPS, Windows, other GPUs,
full-race combat or the later combined content/engine build.

## Method and controls

Native Linux, Ryzen 5 3600 (12 logical threads), RTX 3080 10 GiB, NVIDIA615.78.08,
1440×900. Both course surveys move the camera through representative course views
with live street actors. Their manual60Hz simulation clock makes paired geometry
and simulation workloads reproducible; this is not an ordinary variable-wall-time
free-driving session. The final harness disables Metropolis screenshot capture to
avoid measuring image readback/writes. The earlier instrument-only baseline still
captures screenshots, so it cannot establish a final runtime speedup.

`tools/run_performance_profiles.py` records the binary SHA256, source HEAD/tracked
diff/untracked source hashes, exact environment, loading spans, exit status,
contaminating game/compiler processes and optional targeted asset-page eviction.
Every case uses isolated saves/settings/logs and seed25. Known Baloo indexer
PID696657 is suspended reversibly only within each approved exclusive measurement
batch and restored in `finally`; no global cache flush or process termination.
Renamed frozen RoadDash binaries and Rust test executables count as contaminants.

Raw frame vectors are bounded at4096samples per logging window. The first180race
frames are excluded independently of logger-window exclusions; p50/p95/p99/max and
stall counts derive from retained individual frame durations. Legacy window logs
never invent raw percentiles. CPU spans can contain executor/wait overhead. GPU
pass measurements are latest asynchronous timestamp samples per logger window,
not full-frame GPU percentiles; absent GPU metrics remain absent. Process RSS/HWM
and swap are sampled from `/proc/self/status`, not estimates of GPU allocations.

Loading spans nest and must not be added indiscriminately. `required_assets_ready`
is the existing gate; `first_race_update` is a main-world milestone, not a GPU
presentation timestamp. Cold-asset mode uses POSIX_FADV_DONTNEED on this checkout's
asset files and reports fincore residency before/after. It does not reset metadata,
executable, shader/driver or whole-machine caches and is not a certified cold boot.

## Initial profiling

`initial/{hills,metro}` retains clean instrument-only samples and compressed raw
logs. Hills world setup29.204s, generation29.032s (terrain4.632s, props9.442s,
countryside14.476s); readiness32.902s. Metropolis setup3.237s, geometry3.041s
(meshbuild1.526s, collision0.706s, grass0.455s); readiness10.964s. These identify
repeated terrain support work and the postconstruction asset gate as candidates.
The exploratory original Hills run contaminated by indexing is excluded.

CPU sampling in `cpu-sample.txt` uses perf cycles:u at99Hz after12s delay, with
zero lost samples. Host policy denied callchains; no policy change was made.
Flat symbols suggest executor dispatch/polling as an experiment, but cannot prove
call ancestry or distinguish all waited work. Timing under perf sampling is not
compared against unprofiled runs. See RESEARCH.md for lighting, budget and prior
rejected physics proposals.

## Build provenance

The fresh candidate SHA256 is
`6fe419d28e8833e2c4777b94b1cd642b04e8a76ae0303cced84b74636371ad44`.
Before building it, workspace package artifacts were cleaned explicitly in both
debug and release profiles. The release build rebuilt rr_core, rr_content and
road-dash from the issue25 checkout. Dependency artifacts were retained. Shared
worktree Cargo fingerprints previously allowed stale workspace objects; the
untrusted pre-clean candidate is excluded from comparisons.

## First paired warm-process results

One clean run per configuration, same fresh binary and instrument settings, in
exclusive sequential windows. No competing game/compiler was observed. The known
Baloo process had exited before these batches, so no process was suspended.
Values below are measurements, not repeated confidence bounds.

| Course/configuration | World setup s | Asset readiness s | Raw frame p50 / p95 / p99 ms | Max ms | >50ms stalls |
| --- | ---: | ---: | --- | ---: | ---: |
| Hills original queries, multi dispatch,2load jobs |29.910|33.886|22.00 /25.98 /28.22|41.81|0|
| Hills exact reuse, multi dispatch,2jobs |18.595|22.309|21.91 /25.56 /27.65|33.12|0|
| Hills exact reuse, single dispatch,2jobs |18.379|22.121|16.72 /20.45 /22.56|33.17|0|
| Metro original queries, multi dispatch,2jobs |3.221|11.458|23.56 /28.40 /30.56|32.91|0|
| Metro exact reuse, multi dispatch,2jobs |2.366|10.132|22.89 /28.66 /39.71|56.00|1|
| Metro exact reuse, single dispatch,2jobs |2.341|10.003|20.13 /24.76 /26.94|28.74|0|
| Metro exact reuse, multi dispatch,8jobs |2.382|6.950|24.02 /31.05 /41.54|73.26|3|

Exact reuse reduces world setup37.8% Hills/26.6% Metro in the first pair.
Hills cache records695937hits,26687entries against a131072entry cap; its storage
is released after generation. No runtime speedup is attributed to this cache.
Single fixed dispatch reduces raw p95 by20.0% Hills/13.6% Metro versus the cached
multi-dispatch sample. Internal Rapier parallel solving,60Hz timing,substeps and
system order are unchanged. Hills physics span averages5.223ms/fixed step with
single dispatch (sync0.784ms); executor/wait work is included in these spans.
Eight loading jobs reduce Metro readiness31.4% versus cached/two jobs. This is
loading-only admission; gameplay remains bounded to two material jobs.

Metro census structures are identical in control/cache/single samples. Hills
revision structures differ only in timing metrics: road/support/collision and
content fields match. These survey fixtures are not the dedicated recovery gate.
All598 workspace tests pass in both the normal configuration and explicit single
dispatch/eight-loading-job configuration, with one existing ignored soak; four new tests
check exact terrain/triangle/tint outputs, cache capacity, bypass and isolation.
Debug and fresh release builds, Clippy with warnings denied, formatting, provenance audit and README/progress
consistency pass. Three Python evidence tests pass. Raw/census receipts are in
`paired/` and `experiments/`; first instrument-only results remain in `initial/`.

## Outstanding verification

Repeated/cold-asset samples and dedicated native recovery checks are pending.
No experimental executor or loading admission policy is selected as a new
production default yet. The local p95 samples remain above16.7ms. Full-course
CPU/GPU/hardware acceptance and final combined-content budgets remain separate
from these measurements. No issue-wide performance requirement is checked off.

## Pre-existing CI failures under investigation

Parent inspection found the same Ubuntu pileup sleep failure on unchanged
main754f9ddb (workflow42908967393/job130154255924) and PR34:208 of240bodies
sleep after900updates. Local full suites and a two-CPU targeted execution pass.
This does not identify the cause; parallel solver/platform floating-point behavior
and late settling remain hypotheses. Test-only diagnostics now print complete
measurements and per-owner sleep/velocity/control/support data on failure, plus
an independent1800-update diagnostic. The original900-update assertions remain
unchanged; no physical coefficients, sleep thresholds or simulation order change.
These additional Rust diagnostics await compilation/CI; no resolution is claimed.

Windows roadmap failure used locale-default Path.read_text decoding for UTF-8
README/JSON files. Repository reads now specify UTF-8; sync_progress already did.
All five roadmap checks pass with PYTHONUTF8=0, PYTHONCOERCECLOCALE=0, LC_ALL=C.
That complete check also caught stale Part01/16 table counts introduced by this
branch; both counts now match the unchanged stable-ID placement index.
