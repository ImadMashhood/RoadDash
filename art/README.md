# Authored rider and motorcycle source

User-approved, independently generated Blender geometry from this chat's earlier
asset-authoring step. No original game mesh, texture, sprite, executable or disc
resource is included. Art/source contains the original authored GLBs, the static
seat pose 0003 and five authoring contact frames; it is separate from runtime art.
These project-authored files are supplied under the repository's MIT license.
Design references do not establish permission to ship original game IP; the
existing RIGHTS-002 release review remains open.

Rebuild with Blender 4.5.1 LTS and Python/Pillow:

```
blender --background --python tools/export_riding_models.py
python3 tools/prepare_rider_textures.py
```

The export is presentation-only. Blender axes +Z up/-Y forward become game +Y up/
+Z forward; both seated rider and motorcycle use 1.2 uniform art scale and +0.03 m
vertical offset. Body physics, road, cameras and steering values are not scaled.
The source bike's tires align at the current 0.42 m visual hub height. The detailed
fairing overhangs the unchanged simplified collision proxy slightly; it does not
add collision mesh complexity or alter the wreck's center of mass.

Rider: 13,896 triangles; motorcycle: 44,871. The binary contains two rider bind
surfaces (standing and seated) plus motorcycle parts. Live actors select one
rider surface, never both. Source 20-bone influences map to the eleven existing
visual drivers. Connected render-only joints derive anatomical endpoints from
those drivers; native GPU skinning uses source standing/seated inverse binds. This retains procedural punches, kicks,
walking and physical crashes; it does not implement the later 130-pose animation
library. Fingers and deformation polish remain open.

Tint masks split the atlas into outfit, helmet, visor and fixed surfaces. Roster
colors remain shared with the minimap. Red pigment zones are neutralized in the
runtime atlas to permit existing per-racer color variants; source GLBs retain the
approved burgundy/red finish. Normal/roughness atlases are extracted from our
source GLB without using external game media. Runtime texture references are
local; launch from the repository or ship its assets directory with the binary.

RDMESH01 contains an eight-byte magic, little-endian JSON-header length, validated
JSON records, then per-record 56-byte vertices (12 f32, four u16 joint IDs) and u32
triangle indices. Header counts and data are bounded in rr_content::riding_model.
No physics or executable behavior can be carried in this asset format.

## Connected visual-bind correction

The current exporter also writes assets/models/rider-bindings.json: actual
standing/seated source pivots and the render parent graph. The runtime now uses
those inverse binds on a separate connected visual skeleton, driven by existing
body/pose rotations. Limb translations propagate through source-anatomy forward
kinematics; elbows/knees have bounded visual hinge flex. Physical bodies, joints,
motors and damping are not replaced. The old segment-center binding is historical;
do not pair its inverse binds with the corrected source-pivot graph. See ADR-029
and docs/testing/MOTION_REPAIRS.md. Full terrain locomotion/foot IK and authored
clips remain in the new execution roadmap, not this attachment repair.
