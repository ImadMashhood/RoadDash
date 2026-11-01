# Continuous city fronts — 2026-10-31

The user's CITY-02 correction requires buildings to touch. CITY-01's increased
count still left six-metre gaps between many 18 m buildings on a 24 m pitch.
This change adds **347 solid shared-edge building infills** between neighboring
city buildings, retaining the 247 canonical lots and 410 city additions.

Infill vertices meet both neighbors' exact end faces, including on curved roads.
Roof height meets the lower neighbor, with a full-depth solid base extending
into the shared terrain. Procedural windows continue the facade. Intersections,
main-road reservations, side streets and unrelated buildings are excluded.
These are building volumes, not collisionless screens. Collision persists when
visuals cull. `frontages/plan.svg` includes the actual infill polygons.

Recovery includes the new volumes and searches around longer blocks. Interior
corners are removed from navigation candidates. Former side-yard audit starting
points now inside solid buildings are moved to exposed perimeter points for the
**test fixture only**; runtime recovery movement is not teleported. Existing
physical capsule, crash, remount and finish regressions remain in the full suite.

## Verification

113 automated tests passed, one optional soak ignored. Format, workspace Clippy
with warnings denied, locked debug/release builds, source-input checks and
provenance audit passed. New geometry tests check exact shared edges and street
clearance; physics rays verify the roof of every infill is collidable. Full-world
inspection passes and is stored compressed under frontages/. Native survey and
race smoke receipts/screenshots are retained alongside those automated logs.

## Scope and records

Source changes: world_gen/frontage.rs, layout/navigation and module registration;
world spawning; physical-world regression test; footprint exporter. No controls,
saves, engine dependencies, original source quotas or original media changed.
README/PROGRESS track CITY-02. ADR-023 records the approach and rollback. Asset
provenance records the original procedural source. No new GitHub repository was
investigated; the previously pinned R01 course_scene facade-strip research is
reused (../research/CITY_PROPORTIONS.md).

A city block intentionally remains open at streets/intersections and where
joining would obstruct another building or a reserved roadway. Buildings at the
rural boundary remain detached. Unique production facades/interiors and global
Manhattan-grid authoring remain outside this correction. Windows hardware and a
human subjective playtest are not claimed.

Final native result: **9.79 ms p95 at 1920×1080**, on this Linux/Vulkan host.
Road-height and elevated views reviewed with contiguous facades and clear streets.
Native crash→remount→city/hills→hill crash→remount→finish passed with audio initialized.
These are local/scripted checks, not Windows hardware or human playtest claims.
