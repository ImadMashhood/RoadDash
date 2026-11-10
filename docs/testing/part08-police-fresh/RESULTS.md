# Verification — 2026-11-09

| Command | Actual result |
| --- | --- |
| `cargo fmt --all --check` | Passed after implementation. |
| `cargo test --workspace --locked police` | Passed17:10client,1content,6core; all prior integrated police regressions retained. |
| `cargo test --workspace --locked` | Passed400:271client,1city-preview binary,46content,82core.1existing600-s turning soak ignored. Full output: tests.txt. Includes `models::tests` differential physical-rig/recovery fixtures. |
| `git diff --check` | Passed. |
| `python3 tools/audit.py` | Failed expected source-provenance hash check `original-police-presentation` after modifying src/police.rs. Exact pre-update failure: audit-before-provenance.txt. Root owns the combined provenance update and final rerun; this is not a passed audit. |

Builds used the root's exclusively leased shared target, CARGO_BUILD_JOBS=2 and
root output/testing/parallel-fixes/tmp. Targeted build rebuilt local dependencies
for this worktree and completed in4m56s; this is compile time, not runtime
performance. Full suite used those compiled artifacts.

Clippy/build/native smoke, paired lab/native recovery, final source/provenance
audit and combined branch checks are delegated back to root for the integrated
Part06/07/08 result. They were not run by this worker. No Windows, physical
controller, external human fairness, broader hardware, production60FPS,
complete offline human loop or600-s soak claim. No push/merge.
