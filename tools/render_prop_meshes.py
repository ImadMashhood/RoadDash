"""Render the delivered GLBs after re-import, never the concept PNGs or source scene.
Usage: blender -b --python tools/render_prop_meshes.py -- GROUP [--clay].
"""
import bpy
import hashlib
import json
import math
import sys
from pathlib import Path
from mathutils import Vector
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'art/props/mesh_assets'

def setup():
 bpy.ops.wm.read_factory_settings(use_empty=True)
 s=bpy.context.scene;s.render.engine='CYCLES';s.cycles.samples=12;s.cycles.use_denoising=True
 try:
  prefs=bpy.context.preferences.addons['cycles'].preferences;prefs.compute_device_type='CUDA';prefs.get_devices()
  gpu=[d for d in prefs.devices if d.type=='CUDA']
  if gpu:
   for d in prefs.devices:d.use=d.type=='CUDA'
   s.cycles.device='GPU'
 except Exception:pass
 s.render.resolution_x=1600;s.render.resolution_y=1100;s.render.resolution_percentage=100;s.render.image_settings.file_format='PNG'
 world=bpy.data.worlds.new('Charcoal asset studio');s.world=world;world.use_nodes=True;world.node_tree.nodes['Background'].inputs[0].default_value=(.055,.065,.08,1);world.node_tree.nodes['Background'].inputs[1].default_value=.5
 s.view_settings.view_transform='AgX'
 bpy.ops.object.camera_add(location=(0,-20,8));cam=bpy.context.object;cam.rotation_euler=(Vector((0,0,.3))-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.type='ORTHO';cam.data.ortho_scale=10.2;s.camera=cam
 for pos,power,size in [((-5,-8,12),2100,8),((6,-3,6),1500,7),((0,6,10),2400,6)]:
  bpy.ops.object.light_add(type='AREA',location=pos);o=bpy.context.object;o.data.energy=power;o.data.size=size;o.rotation_euler=(Vector((0,0,1))-o.location).to_track_quat('-Z','Y').to_euler()
 return cam

def render(group,clay=False):
 manifest=json.loads((OUT/'manifests'/(group+'.json')).read_text());records=manifest['records'];renders=[]
 target=OUT/'renders';target.mkdir(exist_ok=True)
 for start in range(0,len(records),6):
  cam=setup();q=cam.rotation_euler.to_quaternion();right=q@Vector((1,0,0));up=q@Vector((0,1,0));toward=q@Vector((0,0,1));cells=[]
  for slot,d in enumerate(records[start:start+6]):
   before=set(bpy.context.scene.objects);bpy.ops.import_scene.gltf(filepath=str(OUT/d['lods'][0]['file']));objects=list(set(bpy.context.scene.objects)-before);meshes=[o for o in objects if o.type=='MESH']
   verts=[o.matrix_world@v.co for o in meshes for v in o.data.vertices]
   lo=Vector(tuple(min(v[i]for v in verts)for i in range(3)));hi=Vector(tuple(max(v[i]for v in verts)for i in range(3)));center=(lo+hi)/2
   scale=2.2/max(hi-lo);offset=right*((slot%3-1)*3.25)+up*(1.9 if slot<3 else -1.45)
   # Apply the same whole-asset transform to every node; child pivots remain intact.
   root=bpy.data.objects.new(d['id']+' review pivot',None);bpy.context.scene.collection.objects.link(root)
   for obj in objects:
    if obj.parent not in objects:obj.parent=root
   root.location=offset-center*scale;root.scale=(scale,scale,scale)
   root.rotation_euler.z=-.35
   # Recompute translation after rotation so the asset's centre stays in its cell.
   root.location=offset-root.rotation_euler.to_matrix()@center*scale
   if clay:
    m=bpy.data.materials.new('Untextured geometry proof');m.diffuse_color=(.45,.48,.52,1);m.use_nodes=True;m.node_tree.nodes['Principled BSDF'].inputs['Base Color'].default_value=(.45,.48,.52,1)
    for obj in meshes:
     obj.data.materials.clear();obj.data.materials.append(m)
     for p in obj.data.polygons:p.material_index=0
   curve=bpy.data.curves.new('ID label','FONT');curve.body=f'{slot+1:02}  {d["name"]}';curve.align_x='CENTER';curve.size=.17
   label=bpy.data.objects.new('Review label',curve);bpy.context.scene.collection.objects.link(label);label.location=offset-up*1.40+toward*.3;label.rotation_euler=q.to_euler()
   cells.append(dict(slot=slot+1,id=d['id'],name=d['name'],glb=d['lods'][0]['file'],sha256=d['lods'][0]['sha256']))
  path=target/f'{group}_{start//6+1:02}{"_clay"if clay else ""}.png';bpy.context.scene.render.filepath=str(path);bpy.ops.render.render(write_still=True)
  renders.append(dict(file=str(path.relative_to(OUT)),sha256=hashlib.sha256(path.read_bytes()).hexdigest(),cells=cells,method='Blender render of re-imported delivered GLBs; no concept image inputs',clay=clay))
  print('RENDERED_EXPORTED_MESHES',path.name,flush=True)
 result=OUT/'manifests'/(group+('_clay'if clay else '')+'_renders.json');result.write_text(json.dumps(renders,indent=2)+'\n')
if __name__=='__main__':
 args=sys.argv[sys.argv.index('--')+1:];render(args[0],'--clay'in args)
