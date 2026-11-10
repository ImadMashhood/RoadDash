# Roads and physical on-foot crossings — 2026-11-09

The approved Metropolis source is unchanged: four3m lanes before3350m, three on
each branch until5362.5m, four after rejoin. Asphalt, shoulders, sidewalks,
rights-of-way, source paths/grades/gates,19,068buildings, farm fields, legacy
maps and Valley quotas are preserved. No dependency migration or new media.

The prior “city entry” survey at3500m was already150m inside the three-lane
branch. A new3250m view shows the actual four-lane entry. Paint previously emitted
2m strips every4m even for solid double-yellow/edge lines, with fragmented white
dash groups. Rendering now emits continuous solid boundaries and6m dashes with
6m gaps, draping every corner18mm above actual paved triangles. The synthetic
crossfall test proves centre-height paint unsafe in general; the actual current
map audit found **0buried separator centres and0unpaved centres in3066samples**.
We do not claim current-map burial caused the user report.

The independent approved-layout clearance audit passes **4792vehicle footprints
and15grid bikes**, with30supported parked vehicles on secondary streets. Source
layout SHA256 remains
`bad1cc469404a171315a32906a68275905e4711c2e315430d79980414e357d96`.

The physical road-access defect is reproduced: unconditional5m/s downward KCC
input while already grounded overpowers small horizontal requests at thin
pavement rims, repeatedly climbing and slipping back. At3000m, actual asphalt
Y11.662m versus underlying terrainY11.323m adds a0.339m edge; nominal offsets are
terrain−0.20m, road+0.025m, sidewalk+0.14m. Thus terrain-to-sidewalk is nominally
0.34m; the road-to-sidewalk curb is0.115m. Independent8m terrain/32m pavement
triangulation adds local variation.

The final fix uses the **previous authoritative KCC grounded output**: supported
runners receive0.5m/s downward adhesion; unsupported runners retain5m/s. Pure
policy lives in rr_core. Original capsule,0.30m maxstep,0.15m landing probe,
0.30m snap, slope/nudge settings, collisions, both accepted rigs, bike physics,
recovery input/control and saves remain unchanged. Step/landing increases alone
were tested and rejected for residual multi-second sticking.

Final exact-source physics receipts (`crossing-final.txt`, `crossing-summary.json`):

- 24support traversals: farms,3250m entry, both branches, merged and north city;
 both directions,8m and10m lateral starts; all reach18m. Manual speed3.9m/s.
- Active travel4.63–4.85s versus4.62s unobstructed; longest no-new-forward-progress
 streak12ticks/0.20s. Tests reject30ticks/0.50s and incomplete goal traversal.
- Identical old-down-drive control advances6.141m over570walking ticks and has a
370tick/6.17s stall. Earlier270tick fixture advances only0.56m. The old control
is preserved as a successful reproduction test, not accepted behavior.
- A metre-high solid wall stops the capsule at7.500m advance and cannot be climbed.
- Isolated source-support tests omit buildings/furniture to isolate support.
 Full native fixtures require clear capsules on actual terrain/sidewalk owners
 and retain every solid world collider; no collision filtering change ships.

Executed checks: final focused6geometry tests pass, including old control and
wall; strict workspace all-targets Clippy passes; locked debug build passes;
format/diff checks pass;6street-life/streetscape authoring tests pass. A stale
parking fixture now explicitly supplies its intended two-lane scenario instead
of depending on the current4/3/4global profile. One full workspace run had265pass,
1ignored and a too-tight assertion in the new old-control test; that assertion
was calibrated to its measured370tick stall, and all6focused tests then passed.
The parent runs the final integrated full suite. The failed receipt is retained
as `workspace-before-control-calibration.txt`.

`python3 tools/audit.py` currently rejects modified source hashes until the parent
refreshes its owned asset ledger. This is expected bookkeeping, not legal clearance.

Native final verification passes all five cases in `native/receipt.json`:

| Rig / route | Farm walk | City sidewalk walk | Remounts |
| --- | ---: | ---: | --- |
| Anatomy /0 |5.328m |34.587m |2, both0.0000m pickup error |
| Anatomy /1 |5.328m |35.894m |2, both0.0000m pickup error |
| Baseline /0 |5.428m |35.105m |2, both0.0000m pickup error |
| Baseline /1 |5.428m |30.298m |2, both0.0000m pickup error |

The opt-in native fixture uses actual support-owner rays, footprint-clear public
starts and capsule clearance, then real assist/KCC walking and pickup/remount.
It rejects30simseconds on foot so the existing120second rescue cannot mask a
failure. Per-second traces show approximately6m/s physical city advance.

The14-view native survey passes resident geometry/collision/fleet checks. Visual
inspection confirms four lanes at3250m, three on each4300m branch and four at6800m
and11900m. Selected original-size captures and JSON census are retained:

- [Four-lane city entry,3250m](native/metro-survey/metro-03.png)
- [Three-lane branch0,4300m](native/metro-survey/metro-05.png)
- [Three-lane branch1,4300m](native/metro-survey/metro-06.png)
- [Four-lane merged avenue,6800m](native/metro-survey/metro-07.png)
- [Real farm terrain start](native/route-0-anatomy/05-running-to-bike.png)
- [Real branch sidewalk start](native/route-1-anatomy/11-hill-running.png)

No human feel/art approval, broad world traversal, performance/long-streaming or
Windows/hardware certificate is claimed. Native wall-frame samples are receipts,
not proof of a60FPS target; full performance gates stay open.

A rejected native TEST setup at3000m put the runner fully inside a source building.
Solid walls blocked it and the no-rescue guard correctly failed. The final fixture
uses actual farm2800m and navigation footprint clearance, preserving this failure
under `native-invalid-start/`. Production getting-up placement was inspected but
is unchanged: its centre ray lacks volume/interior validation. Ordinary crash
incidence remains unverified; **FOOT-SPAWN-01 stays unchecked** for bounded safe
origins, rooftop/car preservation and explicit invalid-state recovery. This pass
does not weaken collisions or broaden that recovery policy without reproduction.
