"""Blender authoring/export/render worker, one reviewed reference sheet per process."""
import bpy
import hashlib
import json
import math
import sys
from pathlib import Path
from mathutils import Vector,Matrix
sys.path.insert(0,str(Path(__file__).resolve().parent))
import build_building_meshes as C
from catalog_mesh_geometry import make_facade
ROOT=C.ROOT;OUT=C.OUT
LIBRARY=OUT/'Catalog_Materials.blend'
CATALOG=json.loads((OUT/'catalog_designs.json').read_text())
NEW_COLORS={'blue_brick':(.065,.12,.18),'moss_brick':(.11,.17,.075),'purple_brick':(.17,.065,.21),
 'brown_stone':(.19,.135,.085),'blue_plaster':(.12,.23,.29),'green_plaster':(.22,.30,.20),
 'stone':(.31,.29,.23),'terracotta':(.43,.13,.055),'straw':(.50,.37,.12),'blue_plastic':(.02,.14,.34),
 'white_siding':(.63,.61,.53),'clear_glass':(.14,.20,.22)}


def variants():
    for name,original in list(C.MATERIALS.items()):
        if name.endswith(('_dark','_light','_masonry_tile')):continue
        for suffix,factor in [('_dark',.82),('_light',1.1)]:
            if name+suffix in C.MATERIALS:continue
            m=original.copy();m.name=name+suffix
            p=m.node_tree.nodes.get('Principled BSDF');tex=next((n for n in m.node_tree.nodes if n.type=='TEX_IMAGE'),None)
            if tex:
                mix=m.node_tree.nodes.new('ShaderNodeMixRGB');mix.blend_type='MULTIPLY';mix.inputs[0].default_value=1.;mix.inputs[2].default_value=(factor,factor,factor,1)
                m.node_tree.links.new(tex.outputs['Color'],mix.inputs[1]);m.node_tree.links.new(mix.outputs[0],p.inputs['Base Color'])
            else:p.inputs['Base Color'].default_value=(*(min(1,v*factor)for v in p.inputs['Base Color'].default_value[:3]),p.inputs['Base Color'].default_value[3])
            C.MATERIALS[name+suffix]=m


def init_materials():
    C.clean()
    with bpy.data.libraries.load(str(OUT/'RoadDash_Building_Models.blend'),link=False)as(src,dst):dst.materials=src.materials
    C.MATERIALS.update({m.name:m for m in bpy.data.materials})
    for name,color in NEW_COLORS.items():
        C.MATERIALS[name]=C.material(name,color,texture=name!='clear_glass')
        if name.endswith('brick') or name in ['stone','brown_stone']:
            C.MATERIALS[name+'_masonry_tile']=C.material(name+'_masonry_tile',color,brick=True)
    clear=C.MATERIALS['clear_glass'];clear.node_tree.nodes.get('Principled BSDF').inputs['Alpha'].default_value=.28
    clear.surface_render_method='DITHERED';clear.use_backface_culling=False
    variants()
    for m in C.MATERIALS.values():m.use_fake_user=True
    bpy.ops.wm.save_as_mainfile(filepath=str(LIBRARY))
    print('CATALOG_MATERIAL_LIBRARY_READY',len(C.MATERIALS),flush=True)


def load_materials():
    C.clean()
    with bpy.data.libraries.load(str(LIBRARY),link=False)as(src,dst):dst.materials=src.materials
    C.MATERIALS.update({m.name:m for m in bpy.data.materials})


def bounds(objects):
    bpy.context.view_layer.update()
    points=[o.matrix_world@Vector(p) for o in objects for p in o.bound_box]
    return Vector(tuple(min(v[i]for v in points)for i in range(3))),Vector(tuple(max(v[i]for v in points)for i in range(3)))


def main(sheet):
    load_materials();designs=[d for d in CATALOG['designs'] if d['reference_sheet']==sheet and d['builder']!='accepted_first_sheet']
    if not designs:raise ValueError('No unconverted design in selected sheet')
    cols=[];records=[];object_sets=[]
    for d in designs:
        if d['asset_type']=='facade':col,objects,info=make_facade(d)
        else:
            from catalog_structure_geometry import make_structure
            col,objects,info=make_structure(d)
        cols.append(col);object_sets.append(objects)
        for other in cols:other.hide_render=True
        col.hide_render=False
        info['lods']=[C.export_mesh(objects,d['id'],lod)for lod in range(3)]
        lo,hi=bounds(objects);info['source_bounds_m']={'min':list(lo),'max':list(hi)}
        records.append(info);print('CATALOG_EXPORTED',d['id'],[l['triangles']for l in info['lods']],flush=True)
    for col in cols:col.hide_render=True
    cam=C.studio();scene=bpy.context.scene;scene.cycles.samples=20
    for col,objects,d in zip(cols,object_sets,designs):
        col.hide_render=False;lo,hi=bounds(objects);centre=(lo+hi)/2;extent=max((hi-lo).z,(hi-lo).x*1.15,(hi-lo).y*1.1)
        cam.location=centre+Vector((1.25,-1.8,.95))*extent;cam.rotation_euler=(centre-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.ortho_scale=extent*1.4
        for lamp in [o for o in scene.objects if o.type=='LIGHT']:
            direction=Vector((-1.0,-1.5,1.7)) if lamp.name.startswith('Key') else Vector((1.7,-.3,.9)) if lamp.name.startswith('Fill') else Vector((.2,1.5,1.5))
            lamp.location=centre+direction*extent;lamp.data.energy=1700*(extent/9)**2;lamp.data.size=extent*.8;lamp.rotation_euler=(centre-lamp.location).to_track_quat('-Z','Y').to_euler()
        scene.render.resolution_x=680;scene.render.resolution_y=840;scene.render.filepath=str(OUT/'catalog_renders'/f"{d['id']}.png")
        bpy.ops.render.render(write_still=True);col.hide_render=True
    # One contact sheet rendered from the geometry itself, with normalized review scale.
    for i,(objects,d) in enumerate(zip(object_sets,designs)):
        lo,hi=bounds(objects);size=hi-lo;scale=8/max(size.x,size.y,size.z);offset=Vector(((i%3-1)*7.5,0,(1-i//3)*11.5))
        base=Vector(((lo.x+hi.x)/2,(lo.y+hi.y)/2,lo.z));rotation=Matrix.Rotation(-.35,4,'Z')
        matrix=Matrix.Translation(offset)@rotation@Matrix.Diagonal((scale,scale,scale,1))@Matrix.Translation(-base)
        for src in objects:
            obj=src.copy();scene.collection.objects.link(obj);obj.matrix_world=matrix@src.matrix_world;obj.hide_render=False;obj['gallery_only']=True
        curve=bpy.data.curves.new('Catalog label '+d['id'],'FONT');curve.body=d['id'];curve.align_x='CENTER';curve.size=.45
        obj=bpy.data.objects.new('Catalog label '+d['id'],curve);scene.collection.objects.link(obj);obj.location=(offset.x,-4.5,offset.z-.55);obj.rotation_euler=(math.pi/2,0,0);obj['gallery_only']=True;curve.materials.append(C.MATERIALS['ivory'])
    for lamp in [o for o in scene.objects if o.type=='LIGHT']:
        direction=Vector((-12,-18,28)) if lamp.name.startswith('Key') else Vector((14,-6,20)) if lamp.name.startswith('Fill') else Vector((2,10,28))
        lamp.location=direction;lamp.data.energy=6500;lamp.data.size=18;lamp.rotation_euler=(Vector((0,0,10))-lamp.location).to_track_quat('-Z','Y').to_euler()
    cam.location=(0,-52,24);cam.rotation_euler=(Vector((0,0,10))-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.ortho_scale=26
    scene.render.resolution_x=1500;scene.render.resolution_y=1600;scene.cycles.samples=20;scene.render.filepath=str(OUT/'catalog_renders'/sheet)
    bpy.ops.render.render(write_still=True)
    for obj in list(bpy.data.objects):
        if obj.get('gallery_only'):bpy.data.objects.remove(obj,do_unlink=True)
    for i,col in enumerate(cols):col.hide_render=i!=0;col.hide_viewport=i!=0
    # Each sheet has a bounded editable source; no giant all-catalog scene required.
    sourcefile=OUT/'catalog_sources'/(Path(sheet).stem+'.blend');bpy.ops.wm.save_as_mainfile(filepath=str(sourcefile))
    code=['build_catalog_batch.py','catalog_mesh_geometry.py','catalog_structure_geometry.py','build_building_meshes.py']
    report=dict(schema_version=2,sheet=sheet,reference_sha256=hashlib.sha256((ROOT/'art/buildings/concept_3d'/sheet).read_bytes()).hexdigest(),source_blend=sourcefile.relative_to(OUT).as_posix(),source_sha256=hashlib.sha256(sourcefile.read_bytes()).hexdigest(),records=records,
                authoring_hashes={p:hashlib.sha256((ROOT/'tools'/p).read_bytes()).hexdigest()for p in code if(ROOT/'tools'/p).is_file()},
                scope='Actual modeled draft assets. All-family user art approval and production integration remain separate.')
    (OUT/'catalog_manifests'/(Path(sheet).stem+'.json')).write_text(json.dumps(report,indent=2)+'\n')
    print('CATALOG_SHEET_COMPLETE',sheet,len(records),flush=True)


if __name__=='__main__':
    args=sys.argv[sys.argv.index('--')+1:]if '--'in sys.argv else []
    if args==['--materials-only']:init_materials()
    elif len(args)==1:main(args[0])
    else:raise ValueError('Use --materials-only or a reference sheet PNG name')
