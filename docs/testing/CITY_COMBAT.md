# City and combat completion — 2026-10-31

Scope: CITY-01, COMBAT-01, RIDE-01. This follows the user's correction of the
previous sparse city and finishes the partially implemented input/camera work.

## Implementation

- 247 canonical buildings remain, plus **410 separately identified additions**
  (657 physical building roots). Four side avenues in each direction connect
  through block cross streets and existing junctions. There are 1872 sampled
  street segments, batched into two material surfaces rather than 1872 draw calls.
  This is a grid following the curved corridor; only the main course has race gates.
- City streets have curb-adjacent 3 m sidewalks, no extra 4.5 m gravel gap, and
  closer urban frontage. The original disc comparison and explicit 3D adaptations
  are documented in ../research/CITY_PROPORTIONS.md.
- New lots participate in collision, supported earthworks, clearance checks,
  recovery obstacle routing and visual LOD. Ground coverage extends to the outer
  neighborhood. Shared window meshes add facades without one entity per window.
- Space buffers an automatic nearest-reachable-side kick. The selected side
  drives a leg extension; the target receives a 3 m/s sideways push. It does not
  count toward a rider punch knockout. Mouse left/right punch left/right; J/K remain.
- Rival roots no longer collapse the camera boom. Supported riding yaw is bounded
  and damped after contact, while linear push/contact momentum is retained.
  Airborne/wreck rotation remains physical. Steering alone drives symmetric
  continuously interpolated visual lean up to about 37 degrees each side.

## Automated evidence

Workspace format, Clippy with warnings denied, locked tests, debug/release builds
and provenance audit are required. The current suite has **112 passed**, one
optional 600-second soak ignored. Focused coverage includes kick target selection,
no kick damage to riders, buffered input, side selection, camera crowd obstruction,
yaw disturbance recovery, seven symmetric bank poses, dense lot clearance,
all-building foundation support, shared LOD origins, recovery and race regressions.
The original four source-input tests also pass; source quota files are unchanged.
Exact command logs and native receipts are in city-combat/.

## Native evidence and limits

Native 1920×1080 Vulkan survey captures all 15 sections, a road-height city view,
seven bank poses and both kick poses. The measured census is 657 building roots.
The initial completed-layout survey measured **9.07 ms p95** on this RTX 3080 host;
see the final native JSON for the subsequent capture's measurement. Performance
is a bounded wall-frame sample, not Windows/GPU certification or universal FPS.
Race smoke exercises actual crash/get-up/walk/remount, hills and finish. Unit tests
exercise the crowd-camera and yaw faults; scripted screenshots alone do not
establish subjective handling satisfaction or a human playtest.

## Research, provenance and decisions

R01 revisited at 4e052aca; format and visual behavior only, no C++ or original
media incorporated. The user's verified ISO remains outside Git. New graphics
are original procedural geometry. No dependency or lockfile change. ADR-021/022
record the camera, street grid, frontage and supported-yaw decisions. New roadmap
IDs remain CITY-01, COMBAT-01 and RIDE-01; no next major production part started.

## Files and remaining scope

Core/content: construction schema and steering/input domain state. Client:
world_gen neighborhood/layout/ground/surfaces/buildings/navigation/report/survey,
world/environment, vehicle, game/camera, combat, controls and UI. Tests cover these
changes. Tools add disc metadata measurement and extend the footprint exporter.
README, progress, decisions, issues, changelog, test matrix and provenance updated.
No files removed. Building interiors, unique production art for all archetypes,
traffic-light simulation and complete original handling parity remain future work.
Exact sprite pixel matching is not claimed. This completes the requested city
and combat correction, not those broader production milestones.
