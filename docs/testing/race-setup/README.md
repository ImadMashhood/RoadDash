# Rider colors, nitrous and wide starting route — 2026-11-08

Branch `codex/rider-colors-wide-grid`, parent432ab33, ADR087. Scope is the user’s
rider palette and opening-road/grid request, not Part09 or a physics rewrite.

- Revisited the fifteen default reference rider schemes. Adjusted independent
  shirt/bike swatches, including Axle’s blue-gray panel and Hog’s blue/green
  scheme; retained helmet identities. Original leather neutralization gain1.4→2.6
  makes tint readable under the existing renderer. Geometry, normal/roughness,
  animations and accepted physical rigs remain unchanged. Shared materials serve
  seated/on-foot/ragdoll riders and shirt-colored minimap dots. No original media
  or reference code is shipped. Exact RGB/PBR and alternate career remaps are not
  claimed; final costume/color art approval remains open.
- `content/handling.json` sets `player_nitro_enabled:false`. Assisted and dynamic
  simulation both enforce it; keyboard/controller bindings are retained, HUD/help
  say disabled. The reserve/save format is unchanged; acceleration/top-speed
  formulas remain available for future tuning.
- Fifteen riders have20m successive station gaps, alternating outbound lane
  centres−4.5/−1.5m, player last at−288m. Apron asphalt extends340m. Track projection
  now caches a spawn-derived rear extent instead of clamping at−250m; schema
  verifies actual first-control-point width and bounds grids to340m.
- Both routes:4lanes from0–3350m,3lanes on each fork3350–5362.5m,4lanes thereafter.
  Lanes are3m:2outbound/2oncoming shared,2outbound/1oncoming fork. Lane centres
  retain100m tapers. City restores the existing full12m carriageway within the
  original reservation; farm pavement13.6m includes0.8m shoulders, and handling
  width is12m. Fork pavement retains shoulders beside its9m marked lanes.
- Race paths, grade, gates, field geometry, all19,068 buildings and secondary
  streets stay byte-for-value identical. Primary sidewalks return to2.4m;
  fine curb placement reserves1.5m. Larger furnishings move to nearby secondary
  streets:995props/13types retained.30checked parked cars remain on secondary
  streets instead of65 parked in the live race corridor. Moving fleet remains64.

`tools/bake_metropolis.py` regenerates the runtime geometry. Independent
`tools/audit_race_setup.py` checks4,792traffic footprints,15grid footprints,
parking footprints,4/3/4profiles and preserved geometry against432ab33. Run with
an authoring Python containing Shapely2.1+ and Pillow; the existing local
`/tmp/roaddash-layout-venv/bin/python` was used. Geometry tests are not human
traffic, art or performance acceptance.

## Verification and remaining gates

- `cargo fmt --all --check`, strict workspace Clippy, workspace locked tests,
  debug/release builds passed; receipts alongside this report. **364 tests passed,
  1 existing soak ignored**. Includes exact Shift/no-Shift acceleration/position
  parity with the disabled flag on both adapters, extended grid projection on
  both routes, shared physical rig/body differential and recovery regressions.
- Two Shapely sidewalk/furniture fixtures passed. Independent geometry audit:
  **4,792 traffic bodies and15 starting bikes**, zero pavement-clearance failures;
  all30parked bodies fit and13furniture types remain. Geometry baseline432ab33.
- Final native Metropolis survey exited0: all13camera views include the newly
  painted starting apron and both fork streets; resident geometry/collision,
  authored facades and64moving/30crossing traffic validated. Selected actual
  views and census under native/metro/. View00 is the grid;03city entry;04/05fork;
  06merged avenue;09aerial. [Starting grid](native/metro/metro-00.png) and
  [fork street](native/metro/metro-04.png).
- Native default velocity-muscle route0 passed real buffered club pickup/contact/
  drop, reverse−1.799m/s, two zero-error live remounts,7racers/7killers,4audio sinks
  and results fixture (native/smoke.txt). Rendered HUD explicitly says nitrous
  disabled; [player view](native/racing.png). This preceded the final apron-only
  paint extension; the final survey and route1 run use the final renderer.
- Final native eleven-body control route1 passed both recoveries/remounts and
  results, exit0 (native/baseline.txt). An earlier run printed every assertion
  success but returned143 during teardown; its receipt is retained, not counted
  as a clean exit. Final rerun establishes clean shutdown. These scripted results
  are not a human complete-race, hardware/controller or art certificate.
- Police regression initially reached arrest but its fixture asserted an empty
  profile against this host’s existing9races. The existing profile was not
  changed (smoke persistence is disabled). Rerun uses a fresh ROADDASH_SAVE_DIR;
  initial failure log is retained separately. The isolated final run exited0,
  covering all7police states, escape, physical crash/getup/foot chase, safe bust,
  no payout and actual retry. New police.json/busted.png are archived here;
  historical Part08 evidence was restored unchanged.
- Release paired ragdoll lab: all18cases finite; original body/muscle construction
  and collision/contact formulas were not edited. Audit/5roadmap tests/progress
  sync/whitespace checks passed. No proprietary source or assets were introduced.

Native Linux/X11/RTX3080/Vulkan frame samples still miss16.7ms: default smoke
p9522.514ms, final control smoke49.118ms, city survey44.819ms. Different fixtures,
positions and loading phases prevent a controlled performance-delta claim. Full
raw logs/metrics remain under native/. No overall FPS improvement is claimed.

## Tracking, files, research and integration

New **RACE-SETUP-01**, originating Part01 with world04/05 support, verifies the
narrow requested integration (reason/dependencies/acceptance in README). Counts:
208verified/241open/449IDs. Human color/PBR/costume approval, full-race aggression /
traffic balance, frame budget and broader hardware remain under existing open
art/AI/performance gates; no broad milestone closed or Part09 started.

[FILES.txt](FILES.txt) lists changed existing files; new files are this evidence
folder and `tools/audit_race_setup.py`. Code categories: world palettes, authored
texture preparation, handling config/validator, both drive adapters, UI/help,
track/grid validators/projection, baked Metropolis/lane profiles, sidewalk and
street-life authoring, apron markings/survey. Documentation updates README and
PROGRESS together, placement ledger, ADR087, known issues, chronology and source /
asset provenance. No dependencies, original rigs, animation data, collision mass,
body limits, save format or legacy Valley geometry changed.

R01 `arkits/road-rash` commit7aecf6044f2dd773b925b325823f30d648d61c89 was studied
locally for remap/slot meaning; root reuse license remains unverified, reference
only. No code/media imported. The fifteen visual schemes informed independent
swatches, not a shipped executable/palette extraction. Full research ledger is
`docs/third_party/REPOSITORIES.md`; derivative atlas hash is in ASSETS.csv.

Next: playtest the widened grid/fork, review the fifteen palettes in ordinary
lighting and tune from user feedback. Branch `codex/rider-colors-wide-grid`;
local Conventional Commit only, no push/merge performed. Run from repository:
`cargo run --release --locked`. No automatic next major development part.
