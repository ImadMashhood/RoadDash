# Hills final paired performance — 2026-11-10

Removing fixed-body bookkeeping from immutable static solids reduced the mean of two run window medians from **21.805 to 20.765 ms (4.8%)** with clouds on. Mean weighted physics cost fell **8.342→7.369 ms/step (11.7%)**. Mean of run P95s of window means fell **23.607→22.314 ms (5.5%)**. These results do not meet the 16.7 ms/60 FPS target.

| Run | Selected windows | Window median ms | Window P95 ms | Main-world median ms | Weighted physics ms/step |
| --- | ---: | ---: | ---: | ---: | ---: |
| perf-fixed-a | 91 | 21.669 | 23.323 | 20.940 | 8.301 |
| perf-static-a | 84 | 20.425 | 22.045 | 19.705 | 7.268 |
| perf-nocloud-a | 84 | 20.274 | 22.472 | 19.500 | 7.273 |
| perf-nocloud-b | 86 | 20.865 | 22.823 | 20.151 | 7.415 |
| perf-static-b | 88 | 21.105 | 22.584 | 20.353 | 7.470 |
| perf-fixed-b | 92 | 21.941 | 23.891 | 21.193 | 8.382 |

`fixed` is the fixed-body control; `static` is bodyless immutable collision with volumetric clouds; `nocloud` uses bodyless collision with the baked sky fallback. Each row links through its case directory to `summary.json` and the native scene receipt. Full precision, provenance and configuration are in [performance-comparison.json](performance-comparison.json).

Cloud-on/off means of run medians were **20.765/20.569 ms**. The observed **0.196 ms** difference is smaller than cloud-on/off run spreads of **0.680/0.591 ms**. This does not establish a reliable cloud cost or speedup; GPU timing remains unmeasured.

All six accepted runs used release SHA256 `215cc272371945362b35eb3e8c7f35c61f287afb5af1406810b16f3215166977`, Dynamic vehicle mode, the same scripted camera route and 1440×900 native window, with zero screenshots and complete exit-0 scene verification. The first ten logging samples were discarded and only racing windows selected; every run had one fixed step per render frame. P95 here is the percentile of independent logging-window **means**, not raw-frame P95. Physics includes sync/step/writeback CPU/wait spans. Common tangent filtering and filtered survey queries were held fixed, so no separate gain is assigned to them.

The initial static-a attempt was contaminated when another normal game opened; it is excluded in [invalid-competing-game](invalid-competing-game/status.json). The valid first fixed-a was retained; the remaining five cases were rerun. No competing compiler/game/indexing worker was observed in accepted runs. Baloo was temporarily suspended and restored in both sessions; see [resumed isolation receipt](performance-isolation-resumed.json). Older preliminary runs and historical optimized-debug `before-performance.json` are excluded.

[Hardware inventory](hardware.json): Ryzen 5 3600, six cores/twelve threads; PCI inventory lists RTX 3080, but the selected Vulkan adapter is unverified. This is a two-repeat CPU/wall survey, without GPU timestamps, raw-frame percentiles, confidence intervals, a full driven/damaged race or Windows/broad hardware certification. Performance acceptance remains open.
