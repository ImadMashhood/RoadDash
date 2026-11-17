# Local rider preview

The native preview loads a local standing and seated rider bundle and color
atlas. It uses the game's existing skin and animation path. The preview uses
the supplied palette on non-police rider skins.

Place a compatible Blender rider in `art/hunyuan-rider/rider.blend` with a
`rider-base-color.png` atlas beside it. It must use the project's original
rider authoring skeleton and standing pose. From the repository root:

```sh
blender -b --python-exit-code 1 --python tools/export_hunyuan_preview.py
cargo build --locked
ROADDASH_RIDER_PREVIEW="$PWD/art/hunyuan-rider" ./target/debug/road-dash --hills
```

The anatomical ragdoll preview fits a convex head collider to the helmet mesh.
The baseline ragdoll and production character continue to use their normal
colliders. Bike and standing-player contact envelopes remain the production ones.
Customization, handlebar alignment and performance still need production review.

The local Hunyuan output uses [Tencent's terms](https://github.com/Tencent-Hunyuan/Hunyuan3D-2/blob/f8db63096c8282cb27354314d896feba5ba6ff8a/LICENSE).
The generated rider files and images belong in the local review workspace.

The local Hills smoke run rendered the seated and standing meshes, reached
race results and completed two remounts with zero measured pickup error.
The fitted hull was about 32 × 30 × 34 cm in rider model space.
