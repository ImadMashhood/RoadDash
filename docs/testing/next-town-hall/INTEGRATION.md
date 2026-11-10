# Proposed shared documentation text

README/PROGRESS: City Hall now uses original batched limestone stonework, complete
window surrounds, bronze doors, terrace balustrades, four clock faces, a ribbed
copper lantern and standing-seam roofs. Five resident meshes add19,764triangles;
existing park3 pose, grand massing, open arches, full lawn,31context buildings,
120trees and architectural compound collision are unchanged. Native draft review
led to rear upper-window/roof articulation and a clear entrance review camera.
Evidence: testing/next-town-hall. CITY-CIVIC-01 remains open for user art approval;
no new IDs or closures.

Decision: Keep civic ornament as five resident opaque material batches over the
unchanged physical landmark. Do not turn each window/stone into an entity or
couple detail to collision/streaming. No source atlas/facade family or planning
material changed, so MAT-03 does not require a rebake for this separate resident
landmark. All geometry is original code; no external assets or dependencies.

Known issues: decorative mouldings/balusters have no separate snag colliders;
the retained persistent architectural solids remain. Doors are visual against
the existing solid hall, not a new walkable interior. Resident ornament increases
geometry; its bounded count does not establish FPS gains. Subjective art,
full-race performance and hardware coverage stay open. Base-lighting native art
captures precede parent verification of the combined lighting/hall changes.

Assets: refresh tracked source SHA256 rows for civic.rs/mod.rs/survey.rs and
register hall_detail.rs as original procedural project source. No imported media.
