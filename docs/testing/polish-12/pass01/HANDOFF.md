# Pass01 handoff

Ready for parent review/commit; no commits made by agent.

Implementation: Metropolis authored facade selected-only radius/rank bonuses,
2m plane-exit deadband, two regressions including full28-slot ECS update loop.
No physics/content/material changes or atlas rebake. ADR105; existing PERF-08
remains unchecked, no new IDs or placement changes. Source-ledger hash refreshed.

Verification: fmt/strict Clippy/415 Rust tests (1 existing ignored)/locked build/
audit/five roadmap tests/progress sync all passed. Controlled paired14-view metro,
NPC simultaneous crashes and farm/city recovery native fixtures all exited0;
both LOD surveys passed (City391/Valley54 direct crossfades). Paired18-case lab
all finite, including15 simultaneous rider crashes for both rigs.

Measured survey: median independent window16.69→16.43ms, weighted physics/fixed
step8.33→8.01ms. No established speedup. Initial measurements overlapped user's
game and are explicitly excluded; replacement runs have process snapshots.
GPU timestamps, sustained downtown/full-route/multi-rider native stress, target
p95, human temporal visual acceptance and hardware coverage remain open.

Source/docs changed: src/world_gen/metropolis/facades.rs, README.md,
docs/PROGRESS.md, DECISIONS.md, KNOWN_ISSUES.md, CHANGELOG_DEV.md,
docs/third_party/ASSETS.csv, and this pass01 evidence directory (~29MiB).
Actual native screenshots retained for farm, city entry, downtown, crashes and
legacy LOD; see README.md for reproduction, limitations and exact check receipts.

IMPORTANT: shared target/release/road-dash currently contains the rebuilt baseline
used for comparison. target/debug/road-dash is the final updated source build.
Run cargo build --release --locked in this worktree before future release tests.
Temporary updated executable removed after completed comparisons.

Proposed commit: fix(render): stabilize Metropolis facade detail selection
