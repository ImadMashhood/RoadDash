# Rival pace, launch and combat — 2026-11-11

Branch `codex/rival-pace-combat`, updated from default `main` at `19839421`.
See [reference research](RESEARCH.md), [intermediate trials](TRIALS.md) and ADR168.

## Implementation

- Default police deployment is off; fresh races and retries have no officer or patrol HUD. `ROADDASH_POLICE=1` or `--police-smoke` explicitly enables the retained subsystem.
- The first five seconds hold starting lanes with a bounded actual-speed forecast, preserving full drive and all real obstacle/rider checks. Legitimate closure avoidance remains available.
- Rivals request full throttle below goal throughout the race. Remove the obsolete 155 km/h governor; keep bike-class limits, roster variation and traffic/corner braking. Catch-up uses ordinary engine force.
- Five proactive fighters and exact-attacker retaliation retain moving opponents through short overtakes, align speed/lane and resume racing after a bounded exchange. Reject stopped/downed opponents and officers as melee targets.
- Attack admission uses authored contact timing, signed forward/lateral velocities and the real shared unarmed hit dimensions (0.9 m longitudinal, 1.9 m lateral). It rejects a predicted side crossing. Actual contact, damage and line of sight remain authoritative. No hit volume is enlarged or contact synthesized.
- Start the four-second exchange clock on the first admitted attack; subsequent attacks/separation do not extend it. Passive physics, player controls, saves, recovery and world content are unchanged.

## Verification

Final source: formatting and strict workspace Clippy pass. Locked workspace tests: 474 RoadDash tests pass, one ignored and the two pre-existing physics failures remain. All 12 model/rig differential tests and all focused AI scenarios pass, including actual unarmed kick/punch and chain contacts, 20m catch-up/lateral alignment, both real launch grids, stationary blocker response and police setup/retry. Core (160 pass, one ignored) and content (59 pass) tests pass, as do locked debug/release builds and the provenance audit. Both maps pass 7,200-frame runtime/error checks. Metropolis recovery passes both pickups with zero error; Hills recovery also passes both pickups with zero error and no exceptional rescues. Explicit `--police-smoke` passes even with `ROADDASH_POLICE=0`, verifying the opt-in override and retained pursuit/arrest/retry.

The final release SHA256 is `6940cc790b4f2ea8025805edeae3a14f72b10e906be17b157de4597aa816b025`. [Source hashes](contract-source.json), [command receipts](contract-checks.json), [full library results](contract-workspace-tests.txt) and other `contract-*.txt` logs are retained alongside this report. Commands use the existing Bevy 0.20 target cache, `CARGO_BUILD_JOBS=2`, and `CARGO_INCREMENTAL=0` after a local incremental-cache linker failure; no Cargo/dependency change. Earlier failed/interrupted trials are not passing evidence.

## Native Metropolis result

[Matched and extended summary](final-metro.json): at frame3600 (56.98s), median
rival speed is172.61km/h versus144.28km/h for clean-main control. Opening
sample median is101.06km/h versus43.06km/h. No rival is stopped at frame3600
or7200. Some intermediate samples contain up to four stopped riding rivals;
this does not prove every queue fixed. At116.99s median speed is109.95km/h.
The final candidate records two attack attempts and zero confirmed recipient
incidents. Thus physical contact fixtures pass, but ordinary-map combat
frequency remains insufficiently demonstrated: **AI-ENGAGE-01 stays open**.
No officer15 appears. Neither sample finishes the full race.

## Native Hills result

[Summary](final-hills.json): at frame3600 median rival speed is153.81km/h
versus132.11km/h control; opening sample median100.49 versus26.12km/h. At
frame7200 median is130.13km/h. No rival is stopped at either final snapshot;
up to three are stopped in intermediate samples. One attack attempt and zero
confirmed incidents: this independently keeps AI-ENGAGE-01 open. Neither map
proves every queue fixed or full-race difficulty. Both contain14 rivals and
slot-zero scripted driver, with no officer15.

## Reproduction and retained files

Run `./play-bevy20.sh` from the repository (or add `--hills`). The local
launcher binary is the final verified release; [installation receipt](installed-launcher.json).
Native commands use `python3 tools/run_racer_ai_checks.py --binary <release>
--case race --track metro --frames 7200 --label <unique>`; substitute `hills`, or `--case recovery` for recovery. The exact
validator commands and native flags are preserved in receipts/logs. Counter
comparison: `python3 tools/compare_rival_surveys.py <control-json>
<candidate-json> --output <summary-json>`.

Final evidence: `../issue26-racer-ai/rival-contract-{race,recovery}-{metro,hills}/`
contains logs, binary hashes and selected frames; [opt-in police receipt](police-optin-contract/receipt.json).
Full intermediate/final screenshots and performance JSONL remain locally in
ignored output/testing/rival-pace-combat; compact trial receipts remain here.
No performance improvement is claimed. Source files: rr_core pursuit/rival_ai/lib,
game/vehicle/combat/police adapters, racer AI diagnostics/tests and the explicit
police model fixture; plus roadmap/provenance/testing records and comparison tool.
No dependency, imported media, content/save schema or physical rig changes.

## Checklist and next work

Verified: AI-PACE-01, AI-LAUNCH-01, POLICE-PAUSE-01. Part01 remains open.
Partial: AI-ENGAGE-01 (ordinary-map contact rate), RFU-RACE-03 and
RFU-COMBAT-02/AI-11 (full-race/human challenge). New future Part08 task:
POLICE-REINTRO-01 requires officer/rival flow and user-authorized restoration.
Next: diagnose real encounter lane/LOS/contact losses from these native traces;
do not enlarge hit volumes or disable physical traffic to manufacture contacts.

## Limits

Scripted two-minute map samples are functional evidence, not full-race completion, human challenge approval or a matched performance benchmark. Old controls did not expose attack counters: missing counters remain unknown. Confirmed incidents count recipients, not attacker attribution; counter maxima are lower bounds after resets. Full-course difficulty/fight balance remains RFU-RACE-03, RFU-COMBAT-02 and AI-11. Police return is deferred under POLICE-REINTRO-01.

Two pre-existing Bevy/Rapier tests remain failing: high-speed ragdoll joint bounds and static-chunk contact trajectory parity. This change preserves their tolerances and all physical rig settings.
