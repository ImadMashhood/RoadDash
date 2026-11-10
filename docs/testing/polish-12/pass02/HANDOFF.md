# Pass02 handoff

Ready for parent review/commit. No commits created by agent.

Implemented one narrow defect correction: downed wreck/ragdoll ride-over
eligibility now uses both pre-solver velocity samples. Old code regression
fails on matched2m/s touch with target solver bounce−5m/s; fixed four-case
fixture passes both owned-body kinds and solver-hidden10m/s approach.
Existing rr_core thresholds/energy/cooldown and all rig/handling parameters
unchanged. Car/wall/rear momentum, reverse and pickup audit fixtures pass.

ADR106; existing V2-01-003 evidence extended. No IDs/placement/count changes,
no new assets/dependencies/research imports or atlas/source-hash changes.
README and generated PROGRESS remain synchronized; broad gates stay open.

Verification: fmt/strictClippy/416tests(1existing ignored)/locked debug+release
build/audit/five roadmap tests/progress sync all passed. Both native rigs:
route0 anatomy, route1 baseline; reverse−1.825m/s, four total remounts0.0000m
pickup error, roster/retry/audio pass. Both NPC actor smokes exited0; paired
18-case lab all finite including15 simultaneous rider crashes per rig.
Native logs/JSON/process snapshots/representative actual PNGs retained (~18MiB).
No compiler or other game overlapped native checks. Native performance logs
were incidental and not used to claim a speedup or benchmark.

Limits: synthetic mixed-time defect has no measured ordinary-play incidence;
native fixtures regress general recovery, not the exact synthetic contact.
Human feel/art, arbitrary confined mount/traffic clearance, full-route and
Windows/controller/broad hardware/performance acceptance remain open.

Changed: src/recovery.rs, src/collision_tests.rs, README.md,
docs/PROGRESS.md, DECISIONS.md, KNOWN_ISSUES.md, CHANGELOG_DEV.md, pass02 evidence.
Git branch codex/gameplay-graphics-polish; only these files modified/untracked.
Shared target/release/road-dash NOW contains this pass's worktree build, replacing
pass01's comparison baseline. No original source/save files touched.

Proposed commit: fix(physics): use pre-contact motion for downed ride-over gate
Next authorized pass:03 elevation audit, only after parent integration.
