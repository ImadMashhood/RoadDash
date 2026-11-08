# Distant facade identity atlas

128 original authored designs / 383 non-empty storey bands. These are distant
LOD proxies projected from the actual RoadDash storey GLBs, not retail sprites
or replacements for the close 3D buildings. Source meshes are unchanged.

Rebuild with Blender 4.5.9 bundled Python/numpy:
`blender -b --python tools/bake_facade_lod.py`

Verify with `python3 tools/verify_facade_lod.py` (Python + Pillow). MANIFEST.json
pins the generator, source manifest, each GLB and all 15 PNG mip images. There
are 128-pixel cells with 96-pixel interiors and 16-pixel guards; the fifth mip
still has a one-pixel guard. Albedo mips use linear-light, alpha-weighted filtering;
normal mips renormalize, and roughness/metallic data stays linear. Runtime joins
these into three resident mipmapped images, 134,086,656 bytes total, and releases
individual mip source handles. Samplers clamp to mip4. No gameplay-time rebaking.

Provenance follows the existing independently authored building catalog; no new
third-party art or source copied. Blender is an external build tool, not shipped.
Project source/output license records and the wider rights gates remain in the
asset ledger; this is not a new commercial-rights certification.
