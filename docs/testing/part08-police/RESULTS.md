# Part08 verification — 2026-11-08

Bevy0.17.3/Rapier0.32.0, unchanged dependency manifest/lockfile. Final production
source and binary hashes are in PROVENANCE.json. No dependency migration,
proprietary media/code reuse, physics-rig tuning, push/merge or Part09 work.

| Command / fixture | Actual result |
| --- | --- |
| `cargo fmt --all --check` | Passed; fmt.txt. |
| `cargo clippy --workspace --locked --all-targets -- -D warnings` | Passed; clippy.txt. Primary independently repeated both checks. |
| `cargo test --workspace --locked` | **362 passed,1 existing soak ignored**:250client library,1binary,44content,67core. tests.txt is the primary's independent final full-source rerun; the worker's matching full run also passed. |
| `cargo build --locked` | Passed; build.txt. |
| `cargo build --release --locked` | Passed; release.txt. |
| `python3 tools/audit.py` | Passed; audit.json.523dependency declarations/106runtime asset records/27reference observations. Declaration/hash validation is not legal or vulnerability clearance. |
| `python3 tools/test_roadmap.py` | All5 passed; roadmap.txt.448IDs:207verified/241open. P08-006 alone moves toPart00;9Part08 requirements remain open. |
| `python3 tools/sync_progress.py --check`; `git diff --check` | Passed; progress-check.txt / diff-check.txt. |
| Release `--ragdoll-lab --output docs/testing/part08-police/ragdoll-lab.json` | All18paired baseline/biomechanical cases finite; JSON/text retained. This includes15-rider piles. Headless timings are not race FPS. |
| Final release `--police-smoke` | Passed actual all7states; warning/confirmed LOS pursuit/intercept, temporary fixture barrier→loss/search/escape, real R cleanup, accepted physical falling/getup, deliberate police foot chase, safe arrest with recent-remount foot grace expiry, BUSTED screenshot, real R→clean Countdown15racers+1officer, profile races0/bestnull/no award. native/receipt.txt / police.json / busted.png. |
| Final release `--smoke-test`, `ROADDASH_VERIFY_CLUB=1 ROADDASH_VERIFY_STRIKES=1 ROADDASH_VERIFY_REVERSE=1` | Metropolis route0 passed seven racers/seven killers,4audio sinks, real keyboard club pickup/timed attack/drop, contact.233333s against.232778s threshold, reverse−1.800m/s, two zero-error live remounts and results fixture. native/game/receipt.txt. |
| Final release `--ragdoll-baseline --smoke-test`, `ROADDASH_SMOKE_ROUTE=1` | Metropolis route1 eleven-body control passed both farm/city recoveries,2zero-error remounts,7+7roster,4audio sinks and results. native/baseline/receipt.txt / smoke-metrics.json. Different branches are explicitly not a controlled physics/performance A/B. |
| Final release `--actor-smoke-test` | Passed independent human/quadruped ragdolls, solid contacts and bounded bodies; native/actors/receipt.txt. |
| Final release `--metro-survey` | Passed resident chunk/building/collision invariance, both route views, traffic directions/count, ready authored facade geometry and source coverage. native/metro/*.json. Performance target remains open. |
| Release legacy `--inspect-world ...valley_world_v1.json ...building_archetypes_v1.json ...valley.json` | Validated immutable Valley plan/quotas and measured headless report; valley-inspect.json / .txt. No Valley geometry was edited. |

The full suite retains the source/body/joint/180-tick differential model check,
complete crash/run/remount check, checkpoint/branch integrity, input edges across
zero-tick30/60/120/144Hz frames, timed/frozen club/kick contacts, save corruption,
reverse, prop/cow/NPC and world/LOD regressions. New police coverage comprises
4pure policy cases,1content validation and8client integration cases: full normal
controller approach/park/walk/arrest/retry; vector-relative/opposed motion and
unsafe contact interruption; actual wall loss/escape; real pause/restart and
committed finish priority; same-tick third-strike interruption; noncompetitor /
no-award guards; witnessed versus unseen confirmed assault; and real buffered
club attacks against a live foot officer, with third-hit repeat-runner knockdown.
No player transform pinning manufactures the controller approach result.

Native environment: Linux/X11 DISPLAY=:0, RTX3080/Vulkan, isolated save path,
smoke persistence disabled. No Windows hardware, physical-controller matrix,
external human three-minute race, moving-foot tackle/capture or broad soak was
run. The existing600-s aggressive-turning soak remains ignored, not passed.
Stationary-foot native arrest and same-speed foot pursuit do not prove full
running capture/fairness. The finish portion of the stock smoke is a results
fixture; real ordered gate/branch finish behavior is covered by headless tests.

## Measurements and limits

Final native police fixture:30,076scene entities/4,255colliders at its snapshot,
1,402render frames/1,401fixed ticks. Its bounded wall-frame sample averages
**21.057ms**, p95 **43.031ms**;16.7ms is missed. This scenario includes script
placement/retry, occlusion barrier and crash/recovery; it is not steady-state
human racing or a controlled before/after benchmark.

The police's conditional ray/state evaluation span measured **31.286µs mean,
54µs maximum**,154evaluations,54detection perceptions and121safe-contact checks
in the stopped-crash/arrest scenario. It excludes common rider queries, normal
drive/navigation/physics/render and does **not** establish whole-police overhead.
Detection is10Hz; qualifying arrest contacts independently revalidate at60Hz.

The final normal route0 game's last one-second log sample reports wall p95
**23.047ms** (mean20.340ms); baseline route1 reports **23.126ms** (mean19.471ms).
These are different scripted routes, not an A/B delta or unchanged-cost claim.
Full one-second CPU/wall/memory logs are retained per scenario; no GPU timestamps,
60FPS claim, broad hardware certificate or causal regression magnitude.

Native prop intake was archived into this pass's native folders and removed
from the shared docs/testing/props-in-game/native-metro-run-01.json location.
Only generated task evidence is replaced; prior hand/weapon galleries, models,
source rigs and accepted physics remain preserved.

## Investigated failures and corrections

Retained failures show real fixture problems and the boundaries they exposed:
initial uncontrolled speeding ran through traffic and could reacquire the cop,
so it was replaced with an explicit fixture-only solid LOS barrier. Native R
injection through ButtonInput::press in Update lost its just-pressed edge; the
final fixture uses the existing real KeyboardInput retry helper. Wall fixtures
must enter active physics before query assertions. The controller fixture now
matches the flat authored road's.1m support and relocates through the backend /
history reset helper; its earlier mismatched floor created false6m/s vertical
motion. Seeded hit-chain contacts now respect the existing.25s duplicate guard.
Local foot steering no longer treats the pursuit target as a generic avoidance
wall; KCC collision remains solid. Parked wrecks retain ordinary wreck mass and
contact settings. All final fixtures pass; failures/ is historical evidence.

P08-006 verifies only implemented/testable police states. The remaining9Part08
requirements retain original art, broader discretionary encounters/traffic,
manual/controller/human fairness, moving capture, full offline/performance and
exit acceptance. No new task IDs or automatic next major part.
