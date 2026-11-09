# Part08 primary review — 2026-11-08

Fresh-context worker `part08_police` implemented the scoped development pass on
`codex/part08-police-loop`, commit `7ed8b82d50613f924357d374c612cc67181f788d`.
The user's standing instruction permits development passes with unmet gates open.
No Part09 was started and no publication or merge was requested or performed.

Primary inspected pure policy, content validation, shared driving/recovery,
confirmed-hit witnesses, live foot targeting, state/contact scheduling, payout
guards and the native BUSTED image/raw state receipt. Review corrections include
vector relative speed, per-fixed-tick arrest safety, bounded foot grace, safe
dismount clearance, actual approach/park/walk integration, counterattacks against
the live officer, finish/pause/third-hit priority and real retry input. Native
fixture placement and the temporary LOS barrier remain explicitly documented;
they are not evidence of unassisted human balance or moving capture.

Primary independently passed the final full workspace suite:362 tests,1 existing
ignored. Also passed fmt, strict workspace Clippy, debug build,4 focused core
police tests,7 runtime police tests, asset audit,5 roadmap tests, progress sync
and whitespace checks. Worker release/native/lab receipts were reviewed in
[results](../testing/part08-police/RESULTS.md), including18 finite paired lab
cases and actual all-seven-state arrest/escape/retry. No physical rig tuning or
dependency migration. R01 remains reference-only with reuse permission unverified;
no source/media was imported. Architecture is ADR086.

P08-006 alone is verified:207 complete/241 open,448 IDs preserved. Nine broader
Part08 requirements remain open for bespoke art, discretion, moving capture,
human/controller fairness, performance/hardware and offline exit acceptance.
Native frame timings still miss16.7ms; ray/state timing is not whole-police cost.
No new task IDs. Playtest the loop and address those gates before exit approval.

Run from the repository: `cargo run --release --locked`.
Reproduce the scripted fixture: `cargo run --release --locked -- --police-smoke`.
Implementation, changed files, tuning, research and exact limitations are linked
from [the development report](../testing/part08-police/README.md).
