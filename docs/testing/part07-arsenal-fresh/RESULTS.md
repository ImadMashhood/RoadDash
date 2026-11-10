# Validation evidence

Commands run from the isolated Part07 worktree with the shared Cargo target under
an exclusive lease; TMPDIR points at repository output because /tmp capacity is
constrained. No platform/hardware certification or performance result is implied.

- `cargo fmt --all --check`: passed (`fmt.txt`).
- `python3 tools/audit.py`: passed:523 dependency declarations,144 runtime asset
  records,27 research observations (`audit.txt`). Intake source hashes updated;
  asset GLB/source media hashes remain unchanged. This is declaration/hash audit,
  not legal clearance or vulnerability scanning.
- Initial focused build compiled the correct worktree packages and exposed a
  private-interface warning (fixed by matching Kind visibility to public systems)
  and a new harness assertion on a144Hz zero-fixed-tick drop frame. The corrected
  fixture waits for the preserved edge and separates the rival so a30Hz second
  tick cannot validly pick up the just-dropped item. No controls behavior changed.
  Exact initial output retained as `initial-zero-tick-fixture.txt`.

`cargo test --workspace --locked`: passed (`tests.txt`):270 game-library tests,
1 native-launch entry test,48 content tests and81 core tests;400 passed total,
1 intentionally ignored. Existing differential physics/recovery tests pass.
Final run has no compiler warnings. `cargo fmt --all --check` and the source/hash
audit were repeated after the last fixture correction and pass. `git diff --check`
passes.

The first full run also exposed the same zero-fixed-tick assumption in the new
fixture's attack-start assertion; its269 existing tests passed. Exact initial
output is retained in `initial-attack-zero-tick-fixture.txt`. The final fixture
observes strike creation and inventory consumption, with a bounded timeout,
through the real buffered keyboard path at all four render rates.

Clippy/build/native combined verification belongs to the root integration pass;
they have not been rerun separately here, avoiding redundant shared-target work.
The full tests include existing model differential physics/recovery checks, all
family keyboard pickup/attack/range/drop cases at30/60/120/144Hz, reset arsenal
coverage, shared Club LOS/immunity/side/retaliation and snapshot boundaries,
content validation/migration and deterministic seeded family identity.

Native family fixtures are available, but have not yet been run by this subtask.
Use integration receipts for any later actual native result. Human art, flex,
playtest, production sweeps/animation, GPU/platform and performance gates remain
open as described in README.md and DECISION-AND-CHECKLIST.md.
