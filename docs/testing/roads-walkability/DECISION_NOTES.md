# Notes for the parent-maintained decision/checklist records

Proposed stable task: ROAD-WALK-01, Part04 (recovery support Part03/06), reason:
reported two-lane city entry and inability to walk back onto streets. Dependencies:
RACE-SETUP-01, METRO-01, existing on-foot/recovery and independent collision.
Acceptance: explicit native four-lane entry3250m, three per3350–5362.5m branch,
four after rejoin; paint matches actual paved triangles and has proper divider/
edge/dash continuity; real terrain/road/sidewalk capsules cross both directions
without repeated stalls; solid walls/props/buildings remain blocking; actual
walk-back and pickup/remount complete on both routes and both physical rigs.
Verification: exact-source physical24cases+old-drive control+metre-wall rejection,
4792vehicle+15grid footprint audit, native four-mode walkbacks/remount and14-view
survey, fixed production settings and model/recovery regression checks.

Decision: preserve all authored geometry/routes/rights-of-way, original KCC
0.30m maxstep/0.15m landing probe/slope/nudge, both accepted rigs and physical
falling response. The demonstrated root cause is unconditional5m/s downward
KCC motion even while grounded; it overpowers small horizontal requests as a
capsule climbs thin-mesh support edges. Use previous authoritative controller
output to select0.5m/s grounded adhesion; unsupported remains5m/s. The pure rule
lives in rr_core; adapters consume it before the fixed physics step.

Rejected: stepheight/landing-width tuning alone (residual multi-second stalls),
loosening a traversal timeout, an82k-triangle geometry rebake or terrain/field
redraw, collision disabling, rig/joint/muscle tuning, or claiming current-map
paint burial. Current3066separator-centre samples were all paved and unburied.
The existing “entry” survey at3500m was already inside the three-lane branch;
new3250m view explicitly distinguishes entry from fork. Actual paint emitted
fragmented solid boundaries and dashed separators; fix those lengths and drape
all corners on exact paved triangles. Lane totals and3m widths already matched
the approved4/3/4 source, so no source geometry migration is needed.

Sources/provenance: local approved docs/planning/farms-long-city/LAYOUT.json,
content/world/metropolis.json and street-life settings; installed pinned
Bevy0.17.3/Rapier0.32.0 source inspected for KCC stepping semantics. All new code
is original project code; no external assets/reference code/dependency changes.
Layout SHA256 remains bad1cc469404a171315a32906a68275905e4711c2e315430d79980414e357d96.
Root must refresh ASSETS.csv source hashes for geometry.rs, survey.rs and
modified test_metropolis_street_life.py. Local audit.py is expected to reject
those until parent-owned ledger refresh; it is not a legal/hardware certificate.

Limits: these are bounded scripted fixtures, not human feel/art approval, full
world traversal, long-streaming/production frame budgets or Windows/hardware
verification. Isolated24source-support cases omit buildings/furniture to isolate
support; separate metre-wall and full native clear-placement/remount cases keep
solid world geometry enabled. See final README/receipts for executed results.

New unchecked follow-up requested by parent: FOOT-SPAWN-01, Part01. Reason:
inspection found production getting-up capsule origins use a torso-centre floor
ray without full volume/interior clearance. A surface-only overlap query also
missed an entirely interior capsule in the rejected native TEST setup. Ordinary
crash incidence is unverified, so no production placement/rescue change is made
in this pass. Dependencies: current recovery, physical support and navigation.
Acceptance: obstacle-aware bounded valid get-up origins, preserved legitimate
roof/car supports and original rigs, explicit invalid-state recovery instead of
solid-wall clipping. Verification: ordinary-crash reproduction, occupied/inside/
roof/car/curb fixtures, geometry/physics regression and native recovery evidence.
The parent retains it unchecked; its scope is distinct from verified fixed-input
support crossings and legitimate public-ground native walk-back scenarios.
