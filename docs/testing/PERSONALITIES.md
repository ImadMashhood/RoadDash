# Individual rival personalities and rear grid — 2026-10-31

The previous AI-01 implementation gave all rivals the same hunting window and
used rider IDs for speed. AI-02 replaces that behavior with validated individual
profiles and target-specific relationships observed in the original level-zero
rider/personality records. It preserves the modern physical controls, direct
three-hit knockout rule, recovery and saves.

Each rival now has an authored cruise/minimum pace, kick-selection denominator,
pursuit reach, strength, stamina and weight. Normalized relationship scores select
race/pass (<16), follow (16–32), or fight (>=32). Jim starts with no hostile
relationships; Mike is hostile toward Pearl; Bose toward Axle. A hostile racer
only attacks an alongside target in reach. Downed hunts require a hostile
relationship rather than a rider-ID modulo. No general periodic attack rolls.

Hit reactions use each victim's observed net relationship response. Some react
with increased hostility; some back off. Modified relationships ease back to
baseline. Strength scales AI kick shove and attack cooldown, stamina bounds attack
bursts, weight affects chassis mass/suspension/forces. Player kick strength and
three-direct-hit knockouts remain the established rules. Cruise is calibrated
using 0.44704 m/s per observed display-speed unit; this is an explicit RoadDash
mapping, not an instruction-level physics replay.

The six CITY spare riders use the original fallback personality zero, with their
own rider strength/stamina/weight records. The original career-start ±6 speed
jitter is not invented as six new personalities. Their base speed is shared,
their combat/weight stats differ. All original reference code remains outside
the repository; these are independently implemented rules and numeric metadata.

Both authored tracks place the player last and give each racer 12 m longitudinal
clearance (formerly 4 m per pair). Valley's approach road now extends past the
rear-most spawn. Projection preserves negative progress on that approach; this
prevents all racers being projected onto distance zero before crossing the start.
The terrain flattening uses the same extended first segment, so remote ground
cannot rise through the approach or obstruct its camera. The strengthened grid
test rejects terrain intrusion and requires a road hit near the expected height.
Legacy eight-spawn import remains accepted; retry handles missing extra slots.

Research: arkits/road-rash commit 4e052aca9552a5690c2b145b0f53856d5c31e980,
race.cpp read_personalities, original_opponents, ai_replan, ai_transition,
ai_relationship and ai_think. Local user-supplied ROADRASH.EXE numeric metadata
was read using tools/measure_reference_rivals.py. No sprites, binary resource
tables or reference implementation source imported. R01 remains REFERENCE ONLY
with unresolved implementation reuse permission, as recorded in REPOSITORIES.md.

Normalized net scores reproduce the baseline decision thresholds. The full
three-axis relationship interpolation, all collision/relationship event sources,
career levels, random-stream parity, weapon variants and original rubber-band
tables remain beyond this correction and explicitly open under AI-03. Human
difficulty acceptance and Windows validation are not claimed.

Files: content/rival_profiles.json and both tracks; rr_content rival validation,
track rules; rr_core pursuit; game/combat/vehicle; environment road extent and
track projection; world physics tests; measurement tool and project records.
No dependencies or files removed. ADR-026 explains tradeoffs/rollback.

## Current-level profile observations

| Rival | Cruise units | Strength | Stamina | Weight lb | Starts hostile toward |
|---|---:|---:|---:|---:|---|
| Mike | 99 | 7 | 3000 | 215 | Pearl |
| Cydney | 104 | 5 | 2000 | 108 | Bose |
| Jim | 102 | 6 | 2500 | 142 | Nobody |
| Pearl | 99 | 5 | 2500 | 118 | Nobody |
| Jon | 100 | 7 | 3500 | 215 | Cydney |
| Rhonda | 97 | 6 | 3500 | 145 | Jim |
| Bose | 96 | 8 | 4000 | 230 | Axle |
| Axle | 95 | 7 | 3000 | 183 | Jon |
| Eric | 87 | 4 | 2200 | 215 | Cydney |
| Buzz | 87 | 2 | 1600 | 108 | Cydney |
| Druid | 87 | 3 | 2000 | 142 | Cydney |
| Hog | 87 | 2 | 1800 | 118 | Cydney |
| Serge | 87 | 6 | 2500 | 215 | Cydney |
| Slash | 87 | 3 | 2000 | 145 | Cydney |

## Verification results

120 automated tests passed, one optional soak ignored. Full workspace Clippy with
warnings denied, formatting, locked debug/release builds, source-input checks,
tracker sync and provenance audit passed. New profile/decision checks distinguish
pass/follow/fight and hostile downed targeting. Expanded-grid physics checks all
15 spawns, player-last ordering, 12 m separation, negative progress/lane roundtrip,
terrain below the road and correct-height ground support. Existing steering,
combat, momentum, camera, walking/remount and race regressions pass.

An initial native check exposed terrain intruding into the longer approach and
blocking camera/driving; corrected the terrain's first-segment projection, then
reran the full checks. Final reviewed racing screenshot shows YOU at P15,
separated rivals, normal acceleration and a clear camera. Native crash → remount →
city/hills → hill crash → remount → finish passes with four audio sinks initialized.
World inspection and all 15 survey sections pass; bounded warmed city p95 is
**14.74 ms at 1920×1080** on this Linux/Vulkan host (16.7 ms target).
Transient warmup frames and other host activity are not a universal performance
guarantee. Driver/mapping warnings remain in native receipts.

AI-02 checked; AI-03 remains open. No dependencies or original source/media reused.
Next: human gameplay feedback on individual rivals, then the separately tracked
career/event/random parity work. This session stays on codex/city-combat-followup;
no push or PR update performed.
