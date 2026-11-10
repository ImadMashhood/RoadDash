# Hills shadow-distance comparison — 2026-11-10

Final binary SHA-256: `9b9b0d2025eee25c4167086d30b320317a68b215382b55fa627e89a1749e3eeb`.

Same release binary, Dynamic vehicle adapter, spatial clouds enabled, no screenshot capture. The four cases ran in **150 / 500 / 500 / 150 m** order. Each executes the same 15-view Hills survey, including its opening city slice; this is not a dedicated Metropolis benchmark. Twelve available CPU threads and parallel physics were reported; no Rayon thread override was requested.

Each run discards its first 10 valid one-second logging windows and then selects racing windows. The median and P95 below describe **logging-window mean milliseconds per rendered frame**, not raw-frame percentiles. P95 uses sorted index `ceil((n−1) × 0.95)`. Physics is weighted by rendered frames and divided by actual fixed steps; it includes synchronization, simulation and writeback spans.

| Run | Range | Selected windows | Median window mean (ms) | P95 window mean (ms) | Weighted physics (ms/fixed step) |
|---|---:|---:|---:|---:|---:|
| perf-near-a | 150 m | 93 | 21.852 | 23.891 | 7.941 |
| perf-city-a | 500 m | 93 | 21.977 | 23.814 | 7.942 |
| perf-city-b | 500 m | 91 | 21.509 | 23.904 | 7.805 |
| perf-near-b | 150 m | 92 | 21.844 | 23.398 | 7.906 |

Pooling the two runs per setting, 150 m measured **21.848 / 23.544 ms** median/P95 window mean; 500 m measured **21.773 / 23.814 ms**. Descriptive changes are **-0.35% / +1.15%**. Weighted physics measured 7.924 versus 7.873 ms/fixed step.

The actual installed four cascade bounds were **[10, 24.6621, 60.8220, 149.99998] m** for the original-distance control and **[10, 24.6621, 111.0453, 500] m** for the expanded setting. Both retained normal bias 1.5 and overlap 0.25, with the first two near splits unchanged. Wider outer coverage trades distant texel density and potential caster work for longer shadows; it does not add shadow maps.

All four native surveys completed with zero reported render errors and the identical final binary hash. One-second process monitoring observed one game per case and no concurrent cargo, rustc or road_dash test processes. Running Baloo processes were suspended by exact PID and resumed in a finally block after each case. The comparison JSON records PIDs, sample counts, native receipts and source log paths. No contaminated cases were observed or excluded.

The first-10-window exclusion can remove part of city-driving; selected views are time-weighted rather than equally weighted. Dynamic traffic and scheduler variation remain, and one-second monitoring can miss brief overlap. These descriptive paired results do not isolate GPU shadow cost, establish a speedup, verify full Metropolis performance, or certify 60 FPS or other hardware. Native warning logs retain Vulkan extension/presentation and unmapped-controller warnings.

Evidence: [comparison.json](comparison.json), per-case `summary.json`, native receipts, survey reports and raw performance JSONL files in this directory.
