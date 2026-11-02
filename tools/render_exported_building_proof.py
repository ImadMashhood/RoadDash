"""Render geometry re-imported from the delivered GLB, with all texture images removed."""
import bpy
from pathlib import Path
from mathutils import Vector
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'art/buildings/mesh_assets'
bpy.ops.wm.read_factory_settings(use_empty=True)
bpy.ops.import_scene.gltf(filepath=str(OUT/'models/BUSS1_lod0.glb'))
meshes=[o for o in bpy.context.scene.objects if o.type=='MESH']
assert len(meshes)==1 and len(meshes[0].data.vertices)>10000
clay=bpy.data.materials.new('Geometry proof - no images');clay.diffuse_color=(.42,.45,.47,1);clay.use_nodes=True
clay.node_tree.nodes.get('Principled BSDF').inputs['Base Color'].default_value=(.42,.45,.47,1)
clay.node_tree.nodes.get('Principled BSDF').inputs['Roughness'].default_value=.75
for obj in meshes:
 obj.data.materials.clear();obj.data.materials.append(clay)
 for p in obj.data.polygons:p.material_index=0
scene=bpy.context.scene;scene.render.engine='CYCLES';scene.cycles.samples=24;scene.cycles.use_denoising=True
try:
 prefs=bpy.context.preferences.addons['cycles'].preferences;prefs.compute_device_type='CUDA';prefs.get_devices()
 for d in prefs.devices:d.use=d.type=='CUDA'
 scene.cycles.device='GPU'
except Exception:pass
world=bpy.data.worlds.new('Proof studio');scene.world=world;world.use_nodes=True;world.node_tree.nodes['Background'].inputs[0].default_value=(.06,.07,.08,1);world.node_tree.nodes['Background'].inputs[1].default_value=.5
# Blender importer converts the GLB back to Z-up and restores the modeled axes.
for pos,power,size in [((8,-10,16),2200,9),((-8,-2,10),1200,8),((1,7,15),1600,7)]:
 bpy.ops.object.light_add(type='AREA',location=pos);o=bpy.context.object;o.data.energy=power;o.data.size=size;o.rotation_euler=(Vector((0,0,4.5))-o.location).to_track_quat('-Z','Y').to_euler()
bpy.ops.object.camera_add(location=(12,-17,12));cam=bpy.context.object;cam.rotation_euler=(Vector((0,0,4.5))-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.type='ORTHO';cam.data.ortho_scale=12.5;scene.camera=cam
scene.render.resolution_x=850;scene.render.resolution_y=1050;scene.render.resolution_percentage=100;scene.render.image_settings.file_format='PNG';scene.render.filepath=str(OUT/'renders/BUSS1-exported-clay.png')
bpy.ops.render.render(write_still=True)
print('EXPORTED_GLB_CLAY_PROOF_PASSED',len(meshes[0].data.vertices),flush=True)
