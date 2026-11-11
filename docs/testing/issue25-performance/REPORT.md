# Issue 25 performance and loading evidence (ADR140)

Status: controlled repeats, paired lab parity and final rebuilt-default recovery/surveys verified. RFU-PERF-01..05
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
Every case uses isolated saves/settings/logs and seed25. Initial instrument-only samples reversibly suspended extractor PID696657.
The exploratory first pairs missed its replacement and are excluded. The primary
controlled matrix later paused verified parent PID2060 via pidfd and restored it
in `finally`; no indexing service/config change or global cache flush.
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

## Exploratory first pairs — excluded from performance acceptance

One run per configuration with the same binary/settings; the monitor excluded
competing games/compilers but missed a replacement Baloo extractor. Its observed
lifetime later overlapped these batches. These figures are retained as exploratory
evidence only, excluded from performance acceptance. No indexer was suspended
for them; subsequent controlled replacements below supersede their timing claims.

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

## Controlled cold-asset and reverse-order warm repeats

Primary evidence is `controlled/`: eight successful cases, same frozen binary,
settings/seed and instrumentation, no detected competing games/compilers/indexers.
Cold control→candidate and warm candidate→control order reduces fixed-order bias.
A verified same-user Baloo parent (PID2060, `/usr/libexec/kf6/baloo_file`, start
2121ticks) was suspended via pidfd for the complete batch and resumed in `finally`;
/proc confirmed it running/sleeping normally afterward. No service/config change.
The proof records UID, executable, cwd, parent lineage, starttime and actual
SIGCONT. Earlier extractor739071 and short-lived836380 exited before identity
capture and received no signal. Two rejected attempts are explicitly preserved.

Asset-page eviction covers3329checkout files. Fincore reports zero resident bytes
after each cold eviction. This is targeted asset-page coldness, not a cold boot,
shader/driver-cache reset or globally cold filesystem. Warm runs retain OS caches.

Control uses original queries/multi dispatch/two load jobs; candidate uses exact
reuse/single dispatch/eight loading jobs. Gameplay job admission stays two.

| Map / cache | Setup s control → candidate | Ready s | Raw p95 ms | Raw p99 ms | Raw max ms |
| --- | --- | --- | --- | --- | --- |
| Hills / cold | 29.031 → 18.073 | 32.824 → 20.457 | 24.333 → 19.211 | 25.933 → 21.349 | 34.582 → 35.938 |
| Hills / warm | 29.097 → 17.866 | 32.836 → 20.297 | 24.535 → 18.796 | 26.444 → 21.087 | 32.073 → 34.346 |
| Metro / cold | 3.344 → 2.317 | 11.218 → 7.074 | 28.617 → 24.941 | 30.700 → 28.432 | 37.744 → 61.423 |
| Metro / warm | 3.353 → 2.389 | 11.341 → 6.915 | 28.416 → 23.827 | 31.152 → 26.332 | 35.363 → 33.708 |

Readiness improves37.7%/36.9% cold and38.2%/39.0% warm for Hills/Metro; raw p95
improves21.1%/12.8% cold and23.4%/16.2% warm. All four census/support structures
per map match exactly after removing only Hills timing metrics; hashes are saved.
Only cold candidate Metro records a>50ms frame (61.423ms), none exceed100ms.
The16.7ms p95 gate remains unmet; these are local traversal fixtures, not full
combat/free-driving or final combined-content/engine budgets.

The61.423ms event is the final recorded racing frame:986racing frames correspond
exactly to survey frames436..1421. Frame1421performs terminal farm/pedestrian/
facade census audits and JSON writes before exit. Material/tangent queues are
empty before and at that window; camera is unchanged, physics averages8.45ms,
latest opaque GPU2.48ms, and paging counters do not change. This strongly points
to fixture completion work, not continuing eight-job loading. GPU samples are
not frame-aligned proof of its exclusive cause. The raw event remains reported.

The logger may leave a partial final window unflushed. A secondary equal-window
comparison uses the same180-frame warmup and common available length across ALL
four cases of each map (4315Hills/768Metro frames). Metro p95 remains28.617→25.127ms
cold and28.416→23.827ms warm; candidate cold maximum37.435ms in that common gameplay
cohort. `common-frame-cohorts.json` preserves all four cases, not selective outlier
removal. Next-build fixture completion markers stop gameplay-frame collection
before terminal audits, with an explicit verification span for confirmation.

Dedicated frozen-binary native recovery also passes both rigs on both maps,
including the alternate Hills route; all four completion markers present, no
render errors, Hills exceptional rescues[0,0]. These functional runs overlap
bounded compilation and make no timing claim. Explicit paired lab parity now passes: all18cases have identical non-timing
measurements under actual multi/single dispatch. Only executor labels and wall
time differ. Source/binary manifest and JSON comparison are retained in lab/.
Final rebuilt-default native checks also pass (final-native/).

## Outstanding verification

Controlled repeated/cold-asset samples and four dedicated native recovery checks
pass. Explicit paired lab and final rebuilt-default native checks pass.
Measured single dispatch/eight loading-job defaults are selected locally after
repeats, native recovery, explicit paired lab parity and final rebuilt checks. The local p95 samples remain above16.7ms. Full-course
CPU/GPU/hardware acceptance and final combined-content budgets remain separate
from these measurements. No issue-wide performance requirement is checked off.

## Pre-existing CI deadline and UTF-8 corrections

Unchanged main754f9ddb (workflow42908967393/job130154255924), PR34 and the
initial diagnostic PR35 Ubuntu run all reported208/240sleeping bodies at900
updates; local full suites and a two-CPU targeted execution passed. PR35 diagnostic
workflow42925593704/job130211132723 showed all15rigs Resting,240finite bodies,
maximum anchor error0.00217m and residual maximum speed0.00639m/s/angular0.1373rad/s
at900updates. Its1800-update diagnostic reached240sleeping bodies with zero
velocities. The retained CI extract is checks/ubuntu-ci-original-sleep.txt.

The corrected test continues the SAME fixture to a bounded1800-update natural
sleep deadline under BOTH explicit executors and records the first all-sleep
update. At the original900-update checkpoint it still requires240finite bodies,
15Resting rigs, anchor error<0.05m and strict low-motion bounds<0.1m/s and<0.5rad/s.
At1800 all240must sleep,15remain Resting and linear/angular speed must be zero.
No forced sleep, physical coefficient, joint, motor or backend threshold changes.
This is a platform-dependent fixture deadline correction, not a runtime physics
performance improvement. Local paired checks pass; final-head CI remains separate.

Windows roadmap failure used locale-default Path.read_text decoding for UTF-8
README/JSON files. Repository reads now specify UTF-8; sync_progress already did.
All five roadmap checks pass with PYTHONUTF8=0, PYTHONCOERCECLOCALE=0, LC_ALL=C.
That complete check also caught stale Part01/16 table counts introduced by this
branch; both counts now match the unchanged stable-ID placement index.

## Receipt provenance clarification

Initial receipts record checkout HEAD/diff at measurement time, not a compiler
manifest. Subsequent receipts name that field `measurement_checkout_source` and
read an optional binary sidecar whose SHA256 must match. The frozen candidate's
record is `frozen-binary-provenance.json`: clean release rebuild from uncommitted
source based on754f9ddb, release-equivalent to review commita5966b81 (a cfg(test)
Track construction correction and evidence/harness/docs changes followed build).
An exact build-tree snapshot was not saved; this limitation is explicit. Later
CI test diagnostics and shared lab helper are absent from the frozen binary.

The earlier double workspace run is a general regression check under both env
settings, not proof that all independently constructed MinimalPlugins fixtures
used single dispatch. The shared native/lab selector and actual executor field now verify explicit
paired lab parity on the rebuilt selected-default binary.
The matched native surveys do use the selected production executor. Their fresh
settings retain default VSync; comparisons are of this same presentation policy.

## Memory and selected policy

CPU VmHWM in the controlled runs remains roughly2.1GiB Hills/5.8GiB Metro.
Candidate Metro peak increases about82MiB cold/29MiB warm versus control; no
GPU-memory or memory-reduction claim is made. VmSwap is zero in all eight cases.
Eight loading jobs remain bounded and affect only Boot/Loading; gameplay retains
two, and ROADDASH_LOAD_JOBS=2 remains the lower-admission control. Single fixed
dispatch retains ROADDASH_FIXED_EXECUTOR=multi and unchanged internal Rapier
parallel solving, fixed frequency and substeps. The final source-frozen release also passes both-map/both-rig recovery with all
three optimization override variables absent; production defaults are exercised.

## Final-window logging correction

The selected source flushes a partial logging window when a survey marks fixture
completion. Raw all-frame data still retains the audit frame; raw gameplay data
excludes it, and a separate span times verification. The bounded writer drains
on resource destruction, so normal exit cannot silently discard queued evidence.
No disk wait is introduced during gameplay frames. Native confirmation passes on both maps with no dropped log samples; controlled
performance comparisons above
remain the original frozen-binary measurements with the terminal event retained.

## Final local verification

Final Rust source passes Clippy with warnings denied and598workspace tests
(425game,1binary,57content,115core; one pre-existing ignored600ssoak). The focused
same-fixture pileup test first reaches all240naturally asleep at update877 in
BOTH explicit executors; checkpoints900/1800 pass. Formatting, provenance audit,
three performance-summary tests and five roadmap tests pass. Exact logs are in
checks/. Final debug and release builds pass. Final default-only native proof
passes on frozen binary e643e48766dc4536b0d151ddde7e1b034ca972225033514d77bdec2d487a3fba,
from clean source58290861355a49de10f1b3595659d4b69ab983f9/tree39497a996af7db507c80df15c7e50fed6d65d047.

Four source-frozen production-default recovery cases pass with explicit completion
markers and no render errors; Hills route0/alternate route1 report zero exceptional
rescues. Final Metro/Hills survey logs retain the terminal audit, classify it as
non-racing, flush the partial window and report zero dropped samples. Exactly985
Metro/4530Hills gameplay frames are retained. Metro's explicit verification span
is37.15ms in this functional run; these runs overlap bounded compilation and make
no timing comparison. All source/tree/binary manifests, receipts and compressed
logs are retained in final-native/. Final-head CI and actual Windows/GPU hardware
coverage are not claimed complete.
