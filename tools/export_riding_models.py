"""Blender 4.5.1: bake user-authored models into a small, renderer-only mesh bundle.

Run: blender --background --python tools/export_riding_models.py
No physics geometry, mass, joint, motion or recovery data is generated here.
"""
import bpy, json, math, pathlib, struct
from mathutils import Matrix, Vector, Quaternion
ROOT = pathlib.Path(__file__).resolve().parents[1]
SCALE = 1.2
records = []
buffers = []

def converted(v, normal=False):
    return Vector((v.x, v.z, -v.y)) if normal else Vector((v.x*SCALE, v.z*SCALE+.03, -v.y*SCALE))

def mapped_bone(name):
    if name in ['root','pelvis']: return 2
    if name in ['neck','head']: return 1
    if name.startswith(('spine','chest','clavicle')): return 0
    side = name.split('.')[-1]
    base = 3 if side == 'R' else 5
    if name.startswith('upper_arm'): return base
    if name.startswith(('forearm','hand')): return base+1
    base = 7 if side == 'R' else 9
    return base if name.startswith('thigh') else base+1

def export_mesh(obj, prefix, skinned):
    deps = bpy.context.evaluated_depsgraph_get()
    ob = obj.evaluated_get(deps); mesh = ob.to_mesh(); mesh.calc_loop_triangles()
    normal_matrix = ob.matrix_world.to_3x3().inverted().transposed()
    groups={}
    for tri in mesh.loop_triangles:
        mat=mesh.materials[tri.material_index]
        if skinned:
            region='shirt' if mat.name.startswith('Burgundy leather') else 'helmet' if mat.name.startswith('Helmet ') else 'visor' if mat.name.startswith('Mirrored silver visor') else 'body'
            key=prefix+'-'+region
            descriptor={'region':region}
        else:
            region=mat.name
            key=prefix+'-'+region
            bsdf=mat.node_tree.nodes.get('Principled BSDF')
            color=list(bsdf.inputs['Base Color'].default_value)
            descriptor={'region':'paint' if 'yellow' in region.lower() else 'fixed','color':color,'metallic':bsdf.inputs['Metallic'].default_value,'roughness':bsdf.inputs['Roughness'].default_value,'alpha':color[3]}
        group=groups.setdefault(key,{'vertices':[],'indices':[],'lookup':{},'material':descriptor})
        for li in tri.loops:
            loop=mesh.loops[li];v=mesh.vertices[loop.vertex_index]
            p=converted(ob.matrix_world @ v.co)
            n=converted((normal_matrix @ mesh.corner_normals[li].vector).normalized(),True)
            uv=mesh.uv_layers.active.data[li].uv if mesh.uv_layers.active else (0,0)
            # Blender images use bottom-left UV; Bevy's texture origin is top-left.
            weights={}
            if skinned:
                for membership in v.groups:
                    name=obj.vertex_groups[membership.group].name
                    joint=mapped_bone(name)
                    weights[joint]=weights.get(joint,0)+membership.weight
            ws=sorted(weights.items(),key=lambda item:-item[1])[:4] or [(0,1)]
            total=sum(w for _,w in ws);ws += [(0,0)]*(4-len(ws))
            vertex=tuple(p)+tuple(n)+(uv[0],1-uv[1])+tuple(w/total for _,w in ws)+tuple(j for j,_ in ws)
            vertex=tuple(round(x,7) if isinstance(x,float) else x for x in vertex)
            if vertex not in group['lookup']:
                group['lookup'][vertex]=len(group['vertices']);group['vertices'].append(vertex)
            group['indices'].append(group['lookup'][vertex])
    for key,g in groups.items():
        vb=b''.join(struct.pack('<12f4H',*v) for v in g['vertices'])
        ib=struct.pack('<'+'I'*len(g['indices']),*g['indices'])
        records.append({'name':key,'group':prefix,'vertices':len(g['vertices']),'indices':len(g['indices']),'material':g['material']})
        buffers.extend([vb,ib])
    ob.to_mesh_clear()

bpy.ops.wm.read_factory_settings(use_empty=True)
bpy.ops.import_scene.gltf(filepath=str(ROOT/'art/source/rider.glb'))
rig=next(o for o in bpy.context.scene.objects if o.type=='ARMATURE')
rider=next(o for o in bpy.context.scene.objects if o.type=='MESH')
for p in rig.pose.bones:p.matrix_basis=Matrix.Identity(4)
for side,s in [('L',1),('R',-1)]:
    for name,axis,angle in [('upper_arm.'+side,(0,1,0),s*math.radians(72)),('forearm.'+side,(0,0,1),s*math.radians(-7))]:
        b=rig.data.bones[name].matrix_local.to_quaternion()
        rig.pose.bones[name].rotation_quaternion=b.inverted() @ Quaternion(axis,angle) @ b
bpy.context.view_layer.update();export_mesh(rider,'standing',True)
pose=json.loads((ROOT/'art/source/seated-0003.json').read_text())
rig.matrix_basis=Matrix(pose['rig_matrix'])
for name,rows in pose['bone_basis'].items():rig.pose.bones[name].matrix_basis=Matrix(rows)
bpy.context.view_layer.update();export_mesh(rider,'seated',True)
bpy.ops.import_scene.gltf(filepath=str(ROOT/'art/source/motorcycle.glb'))
for obj in list(bpy.context.scene.objects):
    if obj.type!='MESH' or obj==rider or any(c.name=='glTF_not_exported' for c in obj.users_collection):continue
    group='front-wheel' if obj.name=='Front_Wheel_Pivot_Geometry' else 'rear-wheel' if obj.name=='Rear_Wheel_Pivot_Geometry' else 'motorcycle'
    export_mesh(obj,group,False)
manifest={'version':1,'scale':SCALE,'visual_offset_y':.03,'vertex_stride_bytes':56,'records':records,'physics_data':False}
header=json.dumps(manifest,separators=(',',':')).encode()
output=ROOT/'assets/models/riding.rdmesh'
output.write_bytes(b'RDMESH01'+struct.pack('<I',len(header))+header+b''.join(buffers))
(ROOT/'art/export-report.json').write_text(json.dumps({'mesh_records':len(records),'triangles':sum(r['indices']//3 for r in records),'bytes':output.stat().st_size,'source_scale':SCALE,'physics_changed':False,'records':records},indent=2))
print('EXPORTED',output,len(records),flush=True)
