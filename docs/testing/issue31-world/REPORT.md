# Hills world follow-up — issue31 / ADR146

Implemented on codex/issue-31-world, integrated with main e776605f (sand,
traffic, guardrails, city activity and shared atmosphere). All seven RFU-WORLD
requirements remain open for human art, full-course and broad performance
review. No publication or hardware claim is implied here.

- Original low-poly conifer woodland extends to110–300m from mountain roads,
  with deterministic clearings, at most58 spatial mesh/compound-collision batches,
  a shared material and850–1100m render fade. Trunks stay physical when culled.
- Grass grows in wider irregular bands; existing farmland, mounted paths and
  bridge reservations remain clear. The existing grass distance budget remains.
- Original smooth western mesas begin outside205m from the continuous road
  datum; existing authored road elevations and central settlement remain.
- Existing storefront batches gain shop goods, lanterns and four larger
  frontage displays (produce counter, saloon seating, freight and hay). Displays
  have matching resident collision and remain within checked road-clear lots.
  A second guarded layer alternates striped stalls, handcarts and hitch
  rails/barrel pairs at the road-facing edge; each has a resident solid.
  Backlots gain two supported low shrubs or cacti within each reserved parcel.
  Original cacti/rocks extend to western outskirts.
- The canyon bed rises12m, carrying the existing terrain-seated river and bank
  strips with it. Water remains presentation-only,0.18m above exact terrain
  triangles; the suspended deck and open edges remain. This intentionally changes
  canyon geometry, not rider/bike physics or either accepted physical rig.

## Research and provenance

Read the repository's Hills ADR135–139, existing farmland/river/dressing and
physical-placement contracts, plus installed level-design, procedural-gen,
performance-optimization and Bevy router skills. Existing Cargo.lock pins
Bevy0.17.3/Rapier0.32; no0.20 API or dependency migration is made. No outside
repository code, assets or media imported. New geometry and landform rules are
original RoadDash MIT source. Existing prop catalog licensing stays unchanged.

## Final integrated evidence (2026-11-11)

`cargo fmt --all --check`, strict workspace Clippy, 663 locked workspace tests,
locked debug/release builds, `tools/audit.py`, five roadmap tests and the
README/progress index check pass on the e776-integrated source. Native release
binary `bd6cee8e7a79eeb559bcaa02e41b3fd13fbb002a045e893df2103ee9d73399ea`
passes six cases: Hills active revision/fall, Hills survey, west and east
recovery, east baseline-rig recovery, and Metropolis recovery control. Receipts
are in [native-receipt.json](native-receipt.json), with representative
[western street](western-west.png), [backlots](western-backlots.png) and
[suspension canyon](suspension-canyon.png) captures. Full local captures are in
`output/testing/issue31-final-native/`. The active case
records 17 captures, a 70.49 m physical canyon fall, 0 exceptional rescues,
224 guarded frontage groups in 227 existing storefront batches and 164 low
plants in 82 existing backlot batches. It retains 58 woodland batches/2,594
trees, 6 farms, 295 guardrail segments and 5,408 collider entities. New groups
add no mesh or collider entities; the existing compound colliders gain parts.

The prior f057 binary and final e776 binary each supplied 38 racing performance
windows after ten warmup samples on the same assisted Hills revision script.
Median wall time per frame is 17.123→17.117 ms; p95 of window wall time is
17.394→17.443 ms. Median physics time per frame is 4.759→4.811 ms; p95 window
physics is 6.122→6.017 ms. These figures do **not** isolate the prop cost:
the shared atmosphere changed between binaries. They are CPU/wall samples,
not GPU timestamps or sustained full-race/target-hardware evidence. Source:
`output/testing/issue31-merged-native/active/performance/` and
`output/testing/issue31-final-native/active/performance/`; condensed values
are in [performance-comparison.json](performance-comparison.json).

Valley `--world-survey` wrote all 15 sections and 657 roots but exited 1 at
17.94 ms 1080p city p95, above the still-open 16.7 ms gate. Prior unchanged
Valley surveys recorded 18.597 and 19.208 ms p95 in `docs/KNOWN_ISSUES.md`;
this result cannot establish a Hills-caused regression. Exact Valley
`--inspect-world content/world/valley_world_v1.json
content/world/building_archetypes_v1.json content/tracks/valley.json` passes:
247 planned buildings (177 near, 70 secondary), 15 sections, the original
`fnv1a64:f866d824acfb111f` allocation fingerprint and the complete archived
assignment ID sequence agree. Historical geometry fingerprints differ after
subsequent main-world revisions, so this check establishes quotas/IDs only.

## Remaining gates

Human close/wide and full-course art review, issue32 wildlife integration,
isolated and broad GPU/performance comparison, target hardware and the 16.7 ms
Valley survey gate remain open. Backlot greenery is still patterned in an aerial
view. Allocation/triangle counts alone do not establish placed geometry or
subjective quality. No RFU-WORLD ID is marked complete.
