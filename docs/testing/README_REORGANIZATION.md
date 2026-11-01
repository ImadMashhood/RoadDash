# Completed-first README verification — 2026-10-31

Scope: documentation/tracker organization only. README is the sole live task
source. All **128 checked tasks** precede all **220 open tasks**, with no status
change. Part 00 groups verified work; Parts 01–08 follow the user's requested
sequence, then original Parts 09/10 and remaining production requirements.

## Files and preservation

- README: all 348 unique task IDs, current section count/navigation table,
  completed groups and unfinished execution sequence. Corrected stale included
  racer count to fifteen and visual-bind description to current connected joints.
- PROGRESS and sync_progress.py: generate from README only. Historical Part
  headings in older receipt prose are explicitly distinguished from current order.
- roadmap/TASK_PLACEMENT.json: exact current section for every task.
- roadmap/README_REORGANIZATION_STATUS.json: statuses before moving tasks.
- roadmap/SUPPORTING_CONTRACTS.md: retained dependency proposals, source map,
  environment/audio inventories, budgets and rights/session context.
- Historical archive, EXECUTION_V2, AGENTS, decisions/issues/history/test matrix:
  clarify the single live checklist and ongoing acceptance/maintenance rules.
- tools/test_roadmap.py: added completed-first/placement/coverage and count checks.

A direct comparison with both task sources at parent commit 458074b verifies all
348 IDs, states, requirement text and normalized evidence links are exactly
preserved. All local Markdown links in README, PROGRESS and supporting contracts
resolve. Original 294 IDs and 54 V2 IDs remain represented. No new checkbox IDs,
completion claims, researched repositories, dependencies or assets were added.

## Verification

- Eleven Python tracker/source tests pass (including two new ordering/count tests).
- Canonical README/progress sync check passes.
- Provenance audit passes: 519 dependency declarations, 27 asset records,
  27 reference observations. This is a declaration/hash audit, not legal clearance.
- `git diff --check` passes.
- Required Rust format, workspace Clippy, workspace tests and locked build results
  are recorded in [readme-reorganization-checks.json](readme-reorganization-checks.json).

No native runtime smoke was repeated: this change modifies no gameplay, assets or
Rust code. Prior motion/runtime evidence remains in MOTION_REPAIRS.md. This does
not close outstanding Windows, human acceptance, new crash choreography, complete
animation, world-art, weapon, police or networking gates.

## Maintenance

When a requirement is verified, retain its ID/text/evidence, mark it checked,
move it to Part 00, update TASK_PLACEMENT and the summary counts, then regenerate
PROGRESS. Leave partial tasks in their current unfinished part. Frozen snapshots
are documentary history; never update them as parallel live checklists. Broad
original and detailed V2 requirements can overlap: fulfilling a subset never
closes the broader gate. No new implementation part started in this session.
