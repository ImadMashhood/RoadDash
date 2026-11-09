# Part09 — first development pass

The next numbered milestone is the vertical-slice playtest gate. Start with a reproducible **scripted receipt runner**, not invented human playtest evidence. The approved default remains the 12km Metropolis farm-to-city course with fifteen racers. Historical three-minute/four-racer/town-farm-cliff requirements remain open; this pass does not replace the user's course or silently claim those criteria.

Build once with `cargo build --release --locked`, then run:

```bash
python3 tools/run_slice_checks.py --output /tmp/roaddash-slice-entry-01 --suite entry --seed 1234
```

Choose a new output directory each time. Existing evidence is never overwritten. Entry mode exercises the actual native frontend/race/pause/results/return fixture, checks the seed/seven armed rivals/layout receipt, and preserves screenshots and process logs. Player profile/preferences are isolated per scenario. The fixture forces finish to test the results path; it does not complete a real race. A nonzero exit, timeout, absent or invalid report fails the receipt. Three Python tests cover parser failure, isolation, overwrite refusal, exit failure and timeout.

`--suite full` exposes existing additional --police-smoke and --smoke-test fixtures on Metropolis. Their combined current-course run is **not yet verified**; it remains P09-AUTO-01. The user can set --timeout for slow machines. Manifest wall_seconds is process duration, not frame time or FPS. No ten external sessions, finish/retry/replay percentages, loss-of-control metrics, top-five-frustration report or 60FPS claim exists yet. No Part10 transport was added.

Tasks: P09-AUTO-01 remains open until the complete current-course suite and report are verified. P09-001..007 and the inherited gates remain open. Next work is the full recovery/police combined run, actual player attack/collision/control-time telemetry and genuine external sessions with the requested comparisons and feedback. Prior animation/art/performance/hardware gates remain unmet. Native entry results: run-01/receipt.json and runner.txt. ADR093 records scope and tradeoffs.

## Actual first receipt

Entry mode passed natively with seed1234 and process exit0. All22 scripted states, seven default equipped rivals/held attachments and five actual window/layout receipts were validated. Saved report/screenshots are under run-01/entry. The full mode was not run in this first Part09 pass; P09-AUTO-01 stays unchecked. Python failure/isolation/timeout tests passed3/3. Existing on-foot finish foundations were reconciled in P07-004 without closing its broader slice acceptance.
