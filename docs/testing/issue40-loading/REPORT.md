# Issue 40 candidate C: asset preparation at startup

Status: **rejected**. No loading or rendering code from either experiment remains in
the branch tip. This is candidate evidence for the four-way comparison, not a
60 FPS, full-race, cold-boot, GPU memory, or hardware acceptance result.

## Controlled method

The common control is clean `e2a3191876d57f266b1e36c967a48c9e8e5667ae`,
release binary SHA256 `682b328291655b18b812c2182ed5366bdd1bdf5bf948ca6731fee64863911536`.
The tangent-admission experiment binary is
`a845797a67db3c10294478db424a8ab84dbcb70ee35a768e51964f9f0b290044`;
the exact-height-cache experiment binary is
`b0e5c0797277a1f2810004918de14aa07e27db0f46ac43917a9cc4f246950d07`.
All three were built from isolated checkouts with private release targets and
cleaned workspace package artifacts. Adjacent frozen binary provenance files
were captured before later documentation edits. Binary files are retained in
`/home/imad/roaddash-issue40/` on this host; their hashes and all run metadata
are in [raw-evidence.tar.gz](raw-evidence.tar.gz), with one row per accepted run
in [METRICS.csv](METRICS.csv). The archive SHA256 is
`f021065ea7b4c08f31468fd315854d371b972bb6a89cc3fdf493aba46d73805b`;
verify with `sha256sum raw-evidence.tar.gz` and extract with
`tar -xzf raw-evidence.tar.gz`. [EVIDENCE_MANIFEST.json](EVIDENCE_MANIFEST.json)
records the archive contents and reconstruction command.

`tools/run_performance_profiles.py` ran `--track both --fixed-executor single
--load-jobs 8 --gpu`, seed 25, Hills performance mode, 1440×900, fixed-camera
course traversal, isolated saves/settings/logs and screenshot capture disabled.
Every accepted case used `flock -F /tmp/roaddash-native.lock`; active
game/compiler/indexer processes were excluded. The exact verified Baloo
extractor was suspended by pidfd for each batch and resumed afterward. Cold
asset runs used POSIX_FADV_DONTNEED on the checkout actually read by each
binary and logged `fincore` residency; they did not clear shader, driver,
executable, metadata or global filesystem caches. Each summary records the
checkout, binary hash, environment, exit status, loading milestones, raw frame
distribution, asynchronous renderer samples, process memory and contaminants.
Run JSON and native logs inside the archive are individually gzip-compressed
byte-for-byte; decompress them to inspect the original output.
The GPU samples are per-pass latest samples, not whole-frame GPU percentiles.

The harness adds an explicit `--tangent-load-jobs {2,8}` control for the first
experiment. It also excludes zombie process rows (`Z` or `<defunct>`) and only
the already identity-verified/suspended extractor PID during the per-frame
contaminant scan. Other active extractors and compilers remain contaminants.
These monitor fixes do not change the game workload. Preflight failures from
the original zombie check and an initial 0.03-second self-indexer rejection are
not accepted timing cases. The self-indexer failure is retained in
archive member `evidence/discarded/load-warm-control-clean/`. A later height-cache warm case
was interrupted after a concurrent small evidence copy was reported; its
partial log is retained in archive member `evidence/discarded/load-derived-warm/`. The exact
suspended extractor was checked and resumed by pidfd after that interruption.
The entire warm batch was then repeated cleanly.

## Experiment 1: more tangent jobs during Loading

Existing material derivation already admits eight jobs during Boot/Loading and
two during gameplay. The first candidate independently raised tangent
preparation from two to eight during Boot/Loading, keeping gameplay at two.
One clean warm control→candidate pair and reverse-order cold candidate→control
pair on each map gave these **exploratory** readings (seconds for readiness,
milliseconds for raw gameplay p95):

| Cache/map | Ready 2→8 | Raw p95 2→8 |
| --- | ---: | ---: |
| Warm Hills | 20.335→20.455 | 18.64→18.75 |
| Warm Metropolis | 7.094→6.892 | 24.50→23.97 |
| Cold Hills | 20.425→20.434 | 18.91→18.81 |
| Cold Metropolis | 6.909→6.872 | 23.49→24.76 |

The small loading change did not repeat in cold Metropolis and p95 moved in
both directions. Tangent admission was reverted before the second experiment.
Raw runs are under archive members `evidence/load-{warm,cold}-{control,eight}-run1/`.

## Experiment 2: reuse exact source heights per material job

The original normal-map derivation converts a source RGB texel to height four
times for its neighboring normals. The candidate cached each conversion in a
job-local `Vec<f32>` and performed the same neighbor arithmetic. A differential
test compared every output byte of normal and roughness maps against the prior
function across all 34 authored profiles and sizes 2, 8 and 32; it passed.
The runtime material source and its provenance CSV hash were committed for the
experiment, then reverted in a separate commit after measurement. No media,
LOD, distant material atlas, collision, simulation, save or dependency changed.

The table shows medians of **two clean runs per map and cache state**. Ready is
`required_assets_ready` from process start in seconds; p95 and p99 come from
individual gameplay frames after the harness warm-up exclusion, in ms.

| Cache/map | Ready baseline→candidate | Raw p95 baseline→candidate | Raw p99 baseline→candidate | Peak HWM MiB baseline→candidate |
| --- | ---: | ---: | ---: | ---: |
| Warm Hills | 20.356→20.884 | 18.904→19.463 | 20.979→21.630 | 2152→2163 |
| Warm Metropolis | 7.080→7.128 | 24.197→25.512 | 26.440→31.221 | 5924→5836 |
| Cold Hills | 20.564→20.958 | 18.678→18.814 | 20.398→20.828 | 2199→2175 |
| Cold Metropolis | 6.867→6.996 | 24.529→24.236 | 26.384→26.430 | 5938→5929 |

Hills readiness regressed 1.9–2.6%. Warm Hills p95 regressed 3.0% and warm
Metropolis p95 regressed 5.4%, crossing the predeclared per-map 5% veto.
The predeclared weighted geometric baseline/candidate score (p95 weight two,
readiness weight one, equal map coverage) is 0.968 warm and 0.995 cold; values
below one favor the baseline. Warm Metropolis candidate run 1 recorded a
240.118 ms maximum and two >50 ms stalls, versus no such baseline stalls;
attribution to this source change is unproven, so the observation is retained
as a regression review signal. Every accepted run exited zero with an empty
contaminant list. The cold candidate evictions recorded zero resident asset
bytes; one baseline cold Hills eviction retained 262,144 bytes out of the
checkout set, a small limitation on exact coldness.

Metropolis cold run 1 `metro-survey.json` is exactly equal between baseline
and candidate. Hills `hills-revision.json` is equal outside its time-varying
`metrics` field; collider/entity counts in that field agree and fixed tick
counts differ by one. These are bounded census checks, not proof of full-course
visual or physics equivalence. Both-map/both-rig recovery was not rerun for the
reverted final branch tip; the candidate made no physical change.

## Verification and decision

The exact-height-cache source passed `cargo fmt --all --check`, strict workspace
Clippy, `cargo test --workspace --locked` (road-dash 434 passed/1 ignored plus
the other workspace suites), locked debug and release builds, the four Python
performance-profile tests and `python3 tools/audit.py` after its source hash
update. Source was reverted after these checks and the final branch tip has
the baseline runtime. On that final tip, formatting, strict Clippy, locked
debug build, audit, roadmap consistency and all 617 workspace tests (two
ignored) passed; the complete test output is archive member
`evidence/final-tests.txt.gz`.
The two source commits retain chronological experiment and rollback history.
No requirement was checked off. `RFU-PERF-01`, `PERF-08`
and `P16-003` remain open for representative combined-content, full-race,
cold-machine and target-hardware acceptance. There is no new task ID or
numbered ADR; this report is the candidate-specific decision record.

The local implementation source and installed Bevy 0.17.3 lockfile were the
only technical inputs investigated. No external repository code, assets or
media were copied. The earlier issue25 controlled report supplies the
measurement limitations and existing loading baseline context.
