#!/usr/bin/env python3
"""Original meter-scale weapon sculpture; Blender 4.5, no imported geometry/media.
Blender Z maps to runtime +Y. Grip origin is (0,0,0); exports preserve it.
"""
import bpy, math, json, hashlib
from pathlib import Path
from mathutils import Vector
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'assets/weapons'; ART=ROOT/'art/weapons'; PROOF=ROOT/'docs/testing/new-weapon-models'
for p in [OUT,ART,PROOF]:p.mkdir(parents=True,exist_ok=True)
def mat(name,color,metal=0,rough=.5):
 m=bpy.data.materials.new(name);m.diffuse_color=(*color,1);m.use_nodes=True
 p=m.node_tree.nodes.get('Principled BSDF');p.inputs['Base Color'].default_value=(*color,1);p.inputs['Metallic'].default_value=metal;p.inputs['Roughness'].default_value=rough;return m
steel=mat('Brushed gunmetal',(.22,.27,.29),.82,.31); edge=mat('Worn warm steel',(.52,.48,.35),.75,.33)
leather=mat('Oxide leather',(.14,.038,.019),0,.65); dark=mat('Charcoal grip',(.027,.032,.035),0,.77)
wood=mat('Honey maple',(.53,.27,.075),0,.42); lightwood=mat('End grain maple',(.72,.45,.19),0,.5)
red=mat('Oxide red lacquer',(.48,.055,.025),.05,.4); ivory=mat('Cream painted mark',(.85,.76,.49),0,.6)
def add(obj,name,material):
 obj.name=name;obj.data.materials.append(material);return obj
def lathe(name,rings,material,n):
 v=[];f=[]
 for z,r in rings:
  v += [(r*math.cos(i*math.tau/n),r*math.sin(i*math.tau/n),z) for i in range(n)]
 for j in range(len(rings)-1):
  for i in range(n):a=j*n+i;b=j*n+(i+1)%n;f.append((a,b,b+n,a+n))
 f += [tuple(reversed(range(n))),tuple((len(rings)-1)*n+i for i in range(n))]
 mesh=bpy.data.meshes.new(name);mesh.from_pydata(v,[],f);mesh.update();o=bpy.data.objects.new(name,mesh);bpy.context.collection.objects.link(o);add(o,name,material)
 for p in mesh.polygons:p.use_smooth=len(p.vertices)==4
 return o
def tube(name,points,r,material,n):
 vs=[];fs=[]
 for j,point in enumerate(points):
  p=Vector(point);t=Vector(points[min(j+1,len(points)-1)])-Vector(points[max(0,j-1)])
  t.normalize();reference=Vector((1,0,0)) if abs(t.y)>.9 else Vector((0,1,0));u=t.cross(reference).normalized();v=t.cross(u)
  vs += [p+r*(math.cos(i*math.tau/n)*u+math.sin(i*math.tau/n)*v) for i in range(n)]
 for j in range(len(points)-1):
  for i in range(n):a=j*n+i;b=j*n+(i+1)%n;fs.append((a,b,b+n,a+n))
 fs += [tuple(reversed(range(n))),tuple((len(points)-1)*n+i for i in range(n))]
 me=bpy.data.meshes.new(name);me.from_pydata(vs,[],fs);me.update();o=bpy.data.objects.new(name,me);bpy.context.collection.objects.link(o);add(o,name,material)
 for p in me.polygons:p.use_smooth=len(p.vertices)==4
 return o
def ring(z,r,material,n,width=.008):lathe('Turned collar',[(z-width/2,r),(z+width/2,r)],material,n)
def build(kind,lod):
 n=[20,12,8][lod]
 if kind=='baseball_bat':
  lathe('Maple bat continuous turned silhouette',[(-.075,.026),(-.069,.031),(-.055,.031),(-.047,.018),(.12,.018),(.23,.023),(.39,.039),(.57,.052),(.70,.054),(.75,.049),(.77,.035),(.777,.012)],wood,n)
  for j in range([13,8,5][lod]):ring(-.037+j*.012,.0205,dark,n,.006)
  ring(.205,.024,red,n,.022);ring(.236,.025,ivory,n,.008)
  # Raised, non-text commercial-free maker's diamond, clearly original.
  bpy.ops.mesh.primitive_cube_add(size=1,location=(0,-.040,.47));o=bpy.context.object;o.scale=(.018,.003,.030);o.rotation_euler[1]=math.pi/4;add(o,'Cream maker diamond',ivory)
 elif kind=='rolling_pin':
  lathe('Maple barrel and grip handles',[(-.08,.015),(-.074,.023),(-.04,.026),(.045,.022),(.08,.016),(.095,.035),(.11,.061),(.13,.067),(.43,.067),(.45,.061),(.465,.035),(.48,.016),(.515,.022),(.60,.026),(.635,.023),(.64,.015)],wood,n)
  for z in [.112,.446]:ring(z,.062,lightwood,n,.01)
  ring(.001,.026,red,n,.012);ring(.56,.027,red,n,.012)
  # Long carved scoring marks give this kitchen implement its own silhouette/material rhythm.
  if lod<2:
   for a in range(6):
    angle=a*math.tau/6;tube('Shallow wood grain',[(.0671*math.cos(angle),.0671*math.sin(angle),z) for z in [.15,.22,.31,.41]],.0008,lightwood,4)
 elif kind=='chain':
  lathe('Leather wrapped palm bar',[(-.075,.022),(-.06,.025),(.07,.025),(.085,.022)],dark,n)
  for z in [-.07,.08]:ring(z,.029,edge,n,.014)
  for j in range([10,7,5][lod]):ring(-.052+j*.012,.026,leather,n,.004)
  for j in range(13):
   z=.105+j*.059;x=.032*math.sin(j*.24)
   points=[]
   steps=[24,16,12][lod]
   for k in range(steps+1):
    a=k*math.tau/steps;u=.030*math.cos(a);v=.046*math.sin(a)
    points.append((x+u if j%2==0 else x,0 if j%2==0 else u,z+v))
   tube('Forged alternating welded link %02d'%j,points,.0075,steel if j%3 else edge,[8,6,4][lod])
 elif kind=='whip':
  lathe('Weighted leather stock',[(-.075,.019),(-.07,.027),(-.05,.027),(-.04,.022),(.14,.021),(.17,.016),(.19,.014)],leather,n)
  for z in [-.055,.145,.178]:ring(z,.024,edge,n,.012)
  for j in range([18,12,7][lod]):ring(-.035+j*.01,.023,dark,n,.003)
  # Rest pose sweep; entirely rigid presentation, never a simulated collision rope.
  points=[(.10*math.sin(t*2.5),.025*math.sin(t*4),.19+t*.91) for t in [i/48 for i in range(49)]]
  for i in range(len(points)-1):tube('Tapered plait %02d'%i,points[i:i+2],.011*(1-i/52),leather,[8,6,4][lod])
  if lod==0:
   for phase in [0,math.pi]:tube('Spiral leather braid',[(p[0]+.009*math.cos(i*.9+phase),p[1]+.009*math.sin(i*.9+phase),p[2])for i,p in enumerate(points[:36])],.0018,dark,4)
  tube('Cracker tassel',[points[-1],(.045,.012,1.15),(.024,0,1.18)],.0025,dark,4)
records=[]
for kind,label in [('chain','Forged road chain'),('whip','Oxide leather whip'),('rolling_pin','Luigi maple pizza rolling pin'),('baseball_bat','Abeyomi maple baseball bat')]:
 lods=[]
 for lod in range(3):
  bpy.ops.object.select_all(action='SELECT');bpy.ops.object.delete(use_global=False);build(kind,lod)
  objs=list(bpy.context.scene.objects);vertices=[o.matrix_world@Vector(v) for o in objs for v in o.bound_box]
  bounds=[[min(v[i] for v in vertices)for i in range(3)],[max(v[i] for v in vertices)for i in range(3)]]
  tris=sum(sum(len(p.vertices)-2 for p in o.data.polygons)for o in objs)
  # Export a single bounded mesh with material slots, not dozens of runtime nodes.
  bpy.ops.object.select_all(action='SELECT');bpy.context.view_layer.objects.active=objs[0];bpy.ops.object.join();o=bpy.context.object;o.name=kind
  bpy.ops.object.transform_apply(location=False,rotation=False,scale=True);bpy.context.scene.cursor.location=(0,0,0);bpy.ops.object.origin_set(type='ORIGIN_CURSOR')
  path=OUT/f'{kind}_lod{lod}.glb';bpy.ops.export_scene.gltf(filepath=str(path),export_format='GLB',use_selection=True,export_yup=True)
  lods.append(dict(file=f'weapons/{path.name}',sha256=hashlib.sha256(path.read_bytes()).hexdigest(),triangles=tris,bounds_y_up=[[bounds[0][0],bounds[0][2],-bounds[1][1]],[bounds[1][0],bounds[1][2],-bounds[0][1]]]))
  if lod==0:
   bpy.ops.wm.save_as_mainfile(filepath=str(ART/f'{kind}.blend'))
   # Actual model proof, neutral studio and two complementary views.
   sc=bpy.context.scene;sc.render.engine='CYCLES';sc.cycles.samples=24;sc.render.resolution_x=800;sc.render.resolution_y=1000;sc.render.resolution_percentage=100
   sc.world.color=(.18,.18,.18)
   for loc,power,size in [((2,-3,4),450,3),((-2,1,3),600,2)]:
    bpy.ops.object.light_add(type='AREA',location=loc);bpy.context.object.data.energy=power;bpy.context.object.data.shape='DISK';bpy.context.object.data.size=size;bpy.context.object.rotation_euler=(Vector((0,0,.45))-bpy.context.object.location).to_track_quat('-Z','Y').to_euler()
   for view,loc in [('front',(1.25,-2.4,1.15)),('rear',(-1.6,2.4,1.0))]:
    bpy.ops.object.camera_add(location=loc);cam=bpy.context.object;cam.rotation_euler=(Vector((0,0,.49))-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.type='ORTHO';cam.data.ortho_scale=1.42;sc.camera=cam;sc.render.filepath=str(PROOF/f'{kind}-{view}.png');bpy.ops.render.render(write_still=True)
 records.append(dict(id=kind,name=label,grip_origin=[0,0,0],axis='+Y',presentation_only=True,lods=lods))
catalog=dict(version=1,license='MIT; independently authored geometry; no external assets',records=records)
(OUT/'catalog.json').write_text(json.dumps(catalog,indent=2)+'\n')
(ART/'PROVENANCE.json').write_text(json.dumps(dict(source='tools/build_weapon_models.py',source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),blender=bpy.app.version_string,authoring_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in ART.glob("*.blend")},imports=[],catalog_sha256=hashlib.sha256((OUT/'catalog.json').read_bytes()).hexdigest()),indent=2)+'\n')
