"""Blender 4.5: original volumetric prop meshes, editable parts, three GLB LODs.
No sprite/concept projection, image-card substitutes or runtime physics changes.
Run Blender --background --python-exit-code 1 --python tools/build_prop_meshes.py -- GROUP.
"""
import bpy
import collections
import hashlib
import json
import math
import random
import sys
from pathlib import Path
from mathutils import Vector, Matrix
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'art/props/mesh_assets'
PALETTE={
 'steel':(.28,.31,.32),'chrome':(.48,.51,.52),'black':(.016,.022,.025),
 'rubber':(.022,.025,.026),'glass':(.035,.075,.10),'white':(.67,.65,.57),
 'red':(.46,.035,.022),'orange':(.78,.18,.025),'yellow':(.82,.49,.035),
 'green':(.07,.19,.10),'blue':(.025,.12,.29),'burgundy':(.19,.018,.045),
 'wood':(.28,.16,.073),'bark':(.12,.067,.031),'leaf':(.11,.23,.055),
 'leaf_light':(.23,.32,.08),'leaf_dark':(.035,.12,.055),'autumn':(.47,.22,.035),
 'stone':(.29,.27,.22),'concrete':(.40,.39,.34),'soil':(.24,.15,.075),
 'skin':(.54,.31,.19),'skin_dark':(.19,.095,.055),'hair':(.065,.038,.022),
 'denim':(.035,.065,.115),'cloth':(.24,.25,.22),'fur':(.30,.14,.055),
 'cow_brown':(.32,.15,.065),'cow_black':(.025,.03,.025),'cream_fur':(.62,.58,.43),'water':(.06,.19,.27),'oil':(.018,.022,.027),
 'hay':(.51,.36,.08),'paper':(.63,.60,.51),'ice':(.32,.51,.57)}
MATS={}


def materials():
 for name,color in PALETTE.items():
  m=bpy.data.materials.new(name);m.use_nodes=True;m.diffuse_color=(*color,1)
  p=m.node_tree.nodes.get('Principled BSDF');p.inputs['Base Color'].default_value=(*color,1)
  p.inputs['Roughness'].default_value=.24 if name in {'glass','water','oil','ice'} else .68
  p.inputs['Metallic'].default_value=.75 if name in {'steel','chrome'} else .08
  # Original tile material: deterministic multiscale grain and sparse wear, no reference pixels.
  image=bpy.data.images.new(name+'_authored_surface',width=128,height=128,alpha=False)
  rng=random.Random(name);pixels=[]
  for y in range(128):
   for x in range(128):
    shade=.92+.07*math.sin(x*.15)*math.sin(y*.11)+rng.uniform(-.06,.06)
    if name not in {'glass','water','oil','ice','skin','skin_dark'} and rng.random()<.018:shade*=.48
    pixel_color=color
    if name.startswith('cow_') and math.sin(x*.085)+math.sin(y*.071)+math.cos((x+y)*.06)>.45:pixel_color=PALETTE['white']
    pixels.extend([min(1,c*shade)for c in pixel_color]+[1.])
  image.pixels.foreach_set(pixels);image.pack()
  tex=m.node_tree.nodes.new('ShaderNodeTexImage');tex.image=image
  m.node_tree.links.new(tex.outputs['Color'],p.inputs['Base Color']);MATS[name]=m


class Geo:
 def __init__(self,d,lod):
  self.d=d;self.lod=lod;self.n=[16,10,6][lod];self.parts={};self.texts=[];self.count=collections.Counter();self.rng=random.Random(d['seed'])
 def poly(self,name,verts,faces,mat,detail=0):
  if detail>2-self.lod:return
  v,f=self.parts.setdefault((name,mat),([],[]));off=len(v);v.extend([tuple(x) for x in verts]);f.extend(tuple(off+i for i in face)for face in faces);self.count[name]+=1
 def box(self,name,c,s,mat,detail=0,rot=None):
  v=[Vector((a*s[0]/2,b*s[1]/2,h*s[2]/2)) for a,b,h in [(-1,-1,-1),(1,-1,-1),(1,1,-1),(-1,1,-1),(-1,-1,1),(1,-1,1),(1,1,1),(-1,1,1)]]
  if rot:v=[rot@p for p in v]
  self.poly(name,[p+Vector(c)for p in v],[(0,3,2,1),(4,5,6,7),(0,1,5,4),(3,7,6,2),(0,4,7,3),(1,2,6,5)],mat,detail)
 def cyl(self,name,a,b,r,mat,r2=None,detail=0,n=None):
  n=n or self.n;r2=r if r2 is None else r2;a=Vector(a);b=Vector(b);direction=(b-a).normalized()
  axis=direction.cross(Vector((0,1,0)) if abs(direction.y)<.9 else Vector((1,0,0))).normalized();other=direction.cross(axis)
  v=[c+rr*(axis*math.cos(t*math.tau/n)+other*math.sin(t*math.tau/n))for c,rr in [(a,r),(b,r2)]for t in range(n)]
  f=[tuple(reversed(range(n))),tuple(range(n,2*n))]+[(i,(i+1)%n,(i+1)%n+n,i+n)for i in range(n)]
  self.poly(name,v,f,mat,detail)
 def ball(self,name,c,s,mat,detail=0):
  n=self.n;r=max(4,n//2);v=[(c[0],c[1],c[2]-s[2])]
  for j in range(1,r):
   a=-math.pi/2+j*math.pi/r
   for i in range(n):
    t=i*math.tau/n;v.append((c[0]+s[0]*math.cos(a)*math.cos(t),c[1]+s[1]*math.cos(a)*math.sin(t),c[2]+s[2]*math.sin(a)))
  top=len(v);v.append((c[0],c[1],c[2]+s[2]));f=[(0,1+(i+1)%n,1+i)for i in range(n)]
  for j in range(r-2):
   for i in range(n):a=1+j*n+i;b=1+j*n+(i+1)%n;f.append((a,b,b+n,a+n))
  f += [(top,1+(r-2)*n+i,1+(r-2)*n+(i+1)%n)for i in range(n)]
  self.poly(name,v,f,mat,detail)
 def ring(self,name,c,r,t,mat,axis='X',detail=0):
  n=self.n;small=max(4,n//2);v=[]
  for i in range(n):
   a=i*math.tau/n
   for j in range(small):
    b=j*math.tau/small;p=((r+t*math.cos(b))*math.cos(a),(r+t*math.cos(b))*math.sin(a),t*math.sin(b))
    p=(p[2],p[0],p[1])if axis=='X'else p
    v.append(tuple(c[k]+p[k]for k in range(3)))
  self.poly(name,v,[(i*small+j,((i+1)%n)*small+j,((i+1)%n)*small+(j+1)%small,i*small+(j+1)%small)for i in range(n)for j in range(small)],mat,detail)
 def text(self,body,location,size,mat):
  if self.lod<2:self.texts.append((body,location,size,mat))
 def objects(self,col):
  result=[]
  for (name,mat),(v,f) in self.parts.items():
   mesh=bpy.data.meshes.new(name);mesh.from_pydata(v,[],f);mesh.update();mesh.materials.append(MATS[mat])
   uv=mesh.uv_layers.new(name='Metres')
   rng=random.Random(self.d['seed']+sum(map(ord,name)))
   for p in mesh.polygons:
    shade=rng.uniform(.84,1.07)
    for li in p.loop_indices:
     co=mesh.vertices[mesh.loops[li].vertex_index].co;uv.data[li].uv=(co.x+co.y,co.z)
   obj=bpy.data.objects.new(name,mesh);col.objects.link(obj);obj['semantic_part']=name
   if self.lod < 2 and len(mesh.polygons)>3:
    bevel=obj.modifiers.new('Authored edge rounding', 'BEVEL');bevel.width=.008 if self.lod==0 else .004;bevel.segments=2 if self.lod==0 else 1
   result.append(obj)
  for body,location,size,mat in self.texts:
   curve=bpy.data.curves.new('Dimensional lettering '+body,'FONT');curve.body=body;curve.align_x='CENTER';curve.align_y='CENTER';curve.size=size;curve.extrude=.001
   obj=bpy.data.objects.new('sign_letters_'+body,curve);col.objects.link(obj);obj.location=location;obj.rotation_euler=(math.pi/2,0,0);curve.materials.append(MATS[mat])
   bpy.ops.object.select_all(action='DESELECT');obj.select_set(True);bpy.context.view_layer.objects.active=obj;bpy.ops.object.convert(target='MESH');result.append(obj);self.count['dimensional_lettering']+=1
  return result


def bolts(g,c,axis='Y'):
 for x in [-.12,.12]:g.ball('fasteners',(c[0]+x,c[1],c[2]),(.014,.008,.014),'chrome',2)


def wheel(g,x,y,z,r=.34,width=.18):
 g.ring('tire',(x,y,z),r*.77,r*.23,'rubber')
 g.cyl('rim',(x-width/2,y,z),(x+width/2,y,z),r*.58,'steel')
 g.cyl('hub',(x-width*.6,y,z),(x+width*.6,y,z),r*.20,'chrome',detail=1)
 for k in range(5):
  a=k*math.tau/5;xx=x+(width*.65 if x>0 else -width*.65)
  g.cyl('rim_spokes',(xx,y,z),(xx,y+math.sin(a)*r*.53,z+math.cos(a)*r*.53),.025,'chrome',detail=1,n=6)


def vehicle(g,kind):
 n=g.d['name'];sport=kind=='sports_car';truck=kind in {'pickup','flatbed_truck','work_truck','fire_engine'};van=kind in {'van','camper'}
 w=1.78;length=4.2 if not truck else 6.4;h=1.4 if not van else 2.1
 paint='white'
 if n.startswith(('MAZDA','SAABG','PICKP','TERCLC','VAN','TRKP')):paint='blue'
 if n.startswith(('MUST','VETTE','PICKUP')):paint='red'
 if n.startswith(('HONDAG','GOLFS','TERCLG','SAABS')):paint='green'
 if kind=='taxi':paint='yellow'
 if kind in {'work_truck','flatbed_truck'}:paint='orange' if n!='TRCKWHT2'else 'white'
 if kind=='fire_engine':paint='blue'
 if kind=='car_hulk':paint='wood'
 if sport:h=1.13;length=4.35
 if truck:w=2.25;h=2.5
 g.box('chassis',(0,0,.48),(w*.84,length-.3,.20),'black')
 bodylength=length if not truck else 2.7;bodyy=0 if not truck else -length/2+1.4
 # Shaped sill/bonnet/trunk cross-sections give true sloping body panels.
 sections=[(-bodylength/2,.86,.82),(-bodylength*.28,1.,1.0),(bodylength*.27,1.,.98),(bodylength/2,.90,.84)]
 vv=[]
 for yy,ww,top in sections:
  vv.extend([(-w*ww/2,bodyy+yy,.48),(w*ww/2,bodyy+yy,.48),(w*ww/2,bodyy+yy,top),(-w*ww/2,bodyy+yy,top)])
 ff=[(3,2,1,0),(12,13,14,15)]+[(a*4+i,a*4+(i+1)%4,(a+1)*4+(i+1)%4,(a+1)*4+i)for a in range(3)for i in range(4)]
 g.poly('shaped_body_panels',vv,ff,paint)
 for y in [-length*.31,length*.31]:
  for x in [-w/2,w/2]:wheel(g,x,y,.37,.37 if not truck else .48,.20)
 # Modeled cabin cross-sections, sloping windscreen/rear glass and pillars.
 cy=-.05 if not truck else -length*.27;cz=1.05 if not truck else 1.6;cw=w*.87;cl=1.9 if not van else 3.1
 if truck:cl=1.65
 roofz=h;front=cy-cl/2;back=cy+cl/2
 if truck:g.box('lower_cab_shell',(0,cy,(cz+.9)/2),(w,cl,cz-.9),paint)
 if kind!='car_hulk':
  roof_front=front+.34;roof_back=back-.26
  v=[(-cw/2,front,cz),(cw/2,front,cz),(cw/2,back,cz),(-cw/2,back,cz),(-cw*.43,roof_front,roofz),(cw*.43,roof_front,roofz),(cw*.43,roof_back,roofz),(-cw*.43,roof_back,roofz)]
  g.poly('sloped_cabin_glazing',v,[(0,1,5,4),(1,2,6,5),(2,3,7,6),(3,0,4,7)],'glass')
  if not n.startswith('MIATA'):
   g.poly('roof_panel',v,[(4,5,6,7)],paint)
  else:
   for xx in [-.35,.35]:g.ball('convertible_seats',(xx,cy+.25,1.10),(.22,.22,.27),'black')
  for x in [-cw/2,cw/2]:
   for y in [front,cy,back]:g.cyl('window_pillars',(x,y,cz),(x*.86,(roof_front if y==front else roof_back if y==back else cy),roofz),.035,paint)
   g.box('window_belt',(x,cy,cz),(.05,cl,.06),'black')
   g.box('door_handle',(x*1.05,cy+.2,.91),(.04,.19,.035),'chrome',1)
   g.cyl('mirror_stalk',(x,front+.15,cz+.06),(x*1.17,front+.12,cz+.10),.025,'black',detail=1)
   g.ball('mirror',(x*1.20,front+.12,cz+.13),(.105,.07,.065),paint,1)
 else:
  for x in [-cw/2,cw/2]:
   for y in [front,back]:g.cyl('empty_cabin_frame',(x,y,.9),(x,y,1.5),.035,'wood')
  g.box('dashboard',(0,front+.12,1.0),(w*.78,.2,.13),'black')
 if kind in {'pickup','flatbed_truck','work_truck'}:
  for x in [-w/2,w/2]:g.box('bed_sides',(x,length*.22,1.0),(.08,length*.48,.48),paint)
  g.box('bed_deck',(0,length*.22,.78),(w,length*.48,.12),'wood')
  g.box('tailgate',(0,length*.45,1.0),(w,.10,.48),paint)
 if kind=='fire_engine':
  g.box('equipment_body',(0,1.05,1.3),(w,3.3,1.5),paint)
  for x in [-w/2-.02,w/2+.02]:
   for y in [.0,.9,1.8,2.5]:
    g.box('locker_door',(x,y,1.3),(.06,.75,.95),'steel')
    for z in [1.0,1.2,1.4,1.6]:g.box('locker_slats',(x,y,z),(.07,.70,.025),'chrome',1)
  for x in [-.38,.38]:g.cyl('ladder_rails',(x,-1.6,2.5),(x,2.8,2.5),.045,'chrome')
  for i in range(15):g.cyl('ladder_rungs',(-.38,-1.5+i*.3,2.5),(.38,-1.5+i*.3,2.5),.025,'steel',detail=1)
 if kind=='police_car':
  g.box('black_hood',(0,-length*.34,.99),(w,.95,.035),'black');g.box('black_trunk',(0,length*.36,.99),(w,.7,.04),'black')
  g.box('lightbar',(0,cy,h+.12),(1.15,.26,.09),'black')
  for x,mat in [(-.4,'red'),(.4,'blue')]:g.box('emergency_lenses',(x,cy,h+.21),(.35,.22,.12),mat)
 if kind=='taxi':g.box('taxi_roof_sign',(0,cy,h+.18),(.75,.24,.22),'yellow')
 for y in [-length/2-.03,length/2+.03]:
  g.box('bumper',(0,y,.55),(w+.08,.13,.15),'chrome')
  for x in [-w*.34,w*.34]:g.box('lamp_lenses',(x,y,.84),(.30,.035,.17),'white'if y<0 else 'red')
 g.box('radiator_grille',(0,-length/2-.07,.78),(w*.4,.035,.18),'black')
 for i in range(9):g.box('grille_slats',((i-4)*w*.045,-length/2-.10,.78),(.018,.02,.15),'steel',1)
 if kind=='camper':
  for x in [-.45,.45]:g.ring('rear_bicycle_wheels',(x,length/2+.25,.95),.33,.035,'rubber',axis='Z',detail=1)


def sign(g,kind,offset=(0,0,0)):
 x,y,z=offset;n=g.d['name'];back=any(v in n for v in ['BACK','BCK','BK2'])
 if kind in {'billboard','produce_sign','civic_board'}:
  w=5.0 if kind=='billboard'else 2.8;hh=2.0;bottom=2.2
  for xx in [-w*.32,w*.32]:g.cyl('sign_support',(x+xx,y,z),(x+xx,y,z+bottom+hh*.5),.10,'steel')
  g.box('panel_back',(x,y,z+bottom+hh/2),(w,.17,hh),'steel')
  g.box('painted_sign_face',(x,y-.095,z+bottom+hh/2),(w-.12,.035,hh-.12),'blue'if kind=='billboard'else 'white')
  for xx in [-w/2,w/2]:g.box('frame',(x+xx,y-.08,z+bottom+hh/2),(.10,.10,hh+.08),'wood')
  for zz in [bottom,bottom+hh]:g.box('frame',(x,y-.08,z+zz),(w+.1,.1,.1),'wood')
  # Original dimensional scenic emblem, not projected source pictures.
  for i in range(3):
   xx=x-w*.25+i*w*.25
   g.poly('original_mountain_graphic',[(xx-.5,y-.12,z+bottom+.35),(xx+.5,y-.12,z+bottom+.35),(xx,y-.12,z+bottom+1.1+i*.1)],[(0,1,2)],'white',1)
  return
 if kind=='motel_sign':
  g.cyl('motel_post',(x,y,z),(x,y,z+4.3),.10,'steel');g.box('motel_vertical_sign',(x,y,z+3.0),(.7,.24,2.5),'red');return
 if kind=='information_board':
  for xx in [-.9,.9]:g.box('information_posts',(x+xx,y,z+1.45),(.14,.18,2.9),'wood')
  g.box('map_backboard',(x,y,z+1.9),(1.95,.15,1.65),'wood');g.box('original_map_panel',(x,y-.09,z+1.9),(1.7,.035,1.4),'paper')
  for a in [-1,1]:g.box('pitched_information_roof',(x+a*.5,y,z+3.03),(1.14,.8,.10),'wood',rot=Matrix.Rotation(a*.35,3,'Y'))
  for i in range(5):g.cyl('schematic_map_paths',(x-.6+i*.24,y-.12,z+1.4),(x-.4+i*.24,y-.12,z+2.3),.012,'green',detail=1,n=6)
  return
 hh=2.45;w=.66;h=.72
 if 'SPEED' in n:h=.85;w=.62
 if 'YIELD' in n:h=.75;w=.8
 if kind=='bus_sign':h=.55;w=.42
 g.cyl('sign_post',(x,y,z),(x,y,z+hh),.035,'steel')
 mat='steel' if back else 'red'if n.startswith(('STOP','YIELD','DONT')) else 'white'if 'SPEED'in n or n.startswith(('2H','NO','FW','PARK'))else 'yellow'
 center=z+hh-.35
 sides=8 if n.startswith('STOP')else 3 if n.startswith('YIELD')else 4
 rot=math.pi/4 if kind=='warning_sign'or n.startswith(('CURVE','L-TURN','SIGNZIG','SLIP','LANE','XWALK','DEER'))else 0
 if sides==4 and rot==0:g.box('sign_plate',(x,y,center),(w,.04,h),mat)
 else:
  angle=rot+(-math.pi/2 if sides==3 else math.pi/8 if sides==8 else math.pi/4)
  vv=[(x+math.cos(angle+i*math.tau/sides)*w*.65,yy,center+math.sin(angle+i*math.tau/sides)*h*.65)for yy in [y-.02,y+.02]for i in range(sides)]
  faces=[tuple(reversed(range(sides))),tuple(range(sides,2*sides))]+[(i,(i+1)%sides,(i+1)%sides+sides,i+sides)for i in range(sides)]
  g.poly('shaped_sign_plate',vv,faces,mat)
 if not back:
  # Geometric arrow or warning emblem. Exact final typography is separate art review.
  yy=y-.032
  if any(v in n for v in ['LIGAHD','LIGHT']):
   g.box('traffic_symbol',(x,yy,center),(.15,.01,.42),'black',1)
   for dz,m in [(.13,'red'),(0,'yellow'),(-.13,'green')]:g.ball('traffic_symbol_lens',(x,yy-.01,center+dz),(.045,.008,.045),m,1)
  elif n.startswith('STOP'):
   g.text('STOP',(x,yy-.015,center),.19,'white')
  elif 'SPEED' in n:
   g.text('SPEED LIMIT',(x,yy-.015,center+.20),.068,'black');g.text(n[5:7],(x,yy-.015,center-.07),.25,'black')
  elif n.startswith('YIELD'):
   g.text('YIELD',(x,yy-.015,center),.12,'white')
  elif n.startswith('DONT'):
   g.box('no_entry_bar',(x,yy-.01,center),(.43,.01,.095),'white',1)
  else:
   g.box('symbol_stem',(x,yy,center),(.055,.015,.32),'black',1)
   g.poly('direction_symbol',[(x-.15,yy-.01,center+.07),(x+.15,yy-.01,center+.07),(x,yy-.01,center+.25)],[(0,1,2)],'black',1)
 bolts(g,(x,y-.04,center+.25))


def tree(g,kind,o=(0,0,0),scale=1):
 n=g.d['name'];x,y,z=o;r=g.rng;h=(5.5+(g.d['seed']%9)*.32)*scale
 if n.startswith(('TREEH',)):h=3.4*scale
 trunk=.12*scale if h<4 else .24*scale
 g.cyl('trunk',(x,y,z),(x+.10*scale,y,z+h*.72),trunk,'bark',r2=trunk*.45)
 if kind=='conifer':
  for i in range(5):
   zz=z+h*(.28+i*.13);rr=h*(.25-i*.035)
   g.cyl('layered_conifer_crown',(x,y,zz),(x,y,zz+h*.33),rr,'leaf_dark'if i%2 else 'leaf',r2=.035*scale)
 elif kind=='palm':
  for i in range(9):
   a=i*math.tau/9;root=(x,y,z+h*.85);tip=(x+math.cos(a)*h*.43,y+math.sin(a)*h*.43,z+h*.62)
   mid=Vector(root)*.45+Vector(tip)*.55+Vector((0,0,h*.20))
   g.cyl('palm_frond_spine',root,mid,.035*scale,'bark',detail=1)
   side=Vector((-math.sin(a),math.cos(a),0))*h*.09
   g.poly('palm_frond',[root,mid+side,tip,mid-side],[(0,1,2,3)],'leaf')
 else:
  count=7 if kind!='bare_tree' else 11
  for i in range(count):
   a=i*2.4;level=.44+(i%4)*.12;root=Vector((x,y,z+h*level));tip=Vector((x+math.cos(a)*h*.29,y+math.sin(a)*h*.25,z+h*(.73+(i%3)*.08)))
   g.cyl('branch',root,tip,trunk*.38,'bark',r2=.025*scale)
   if kind!='bare_tree':g.ball('leaf_crown',tip,(h*.23,h*.20,h*.23),'autumn'if n.startswith(('TREEZA','TREEZB','TREOAK','TREEUCA','TREUCA'))else 'leaf_dark'if i%3==0 else 'leaf')
   else:
    for j in [-1,1]:g.cyl('bare_twigs',tip,tip+Vector((j*.35*scale,.25*scale,.5*scale)),.025*scale,'bark',r2=.006*scale,detail=1)
 if n.startswith('TREEH'):
  for dx in [-.28,.28]:g.cyl('support_stake',(x+dx,y,z),(x+dx,y,z+1.9),.025,'wood');g.cyl('tree_tie',(x+dx,y,z+1.7),(x,y,z+1.7),.018,'white',detail=1)
 if n in {'TREEB2','TREEB22'}:g.cyl('tree_planter',(x,y,z),(x,y,z+.5),.50,'concrete',r2=.57)


def rock(g,center=(0,0,.35),size=(.7,.55,.5),name='rock'):
 # Original irregular closed volume, deterministic shape per source ID.
 n=g.n;v=[]
 for a in range(5):
  phi=-math.pi/2+a*math.pi/4
  for i in range(n):
   t=i*math.tau/n;rr=g.rng.uniform(.84,1.12);v.append((center[0]+math.cos(t)*math.cos(phi)*size[0]*rr,center[1]+math.sin(t)*math.cos(phi)*size[1]*rr,max(.0,center[2]+math.sin(phi)*size[2]*rr)))
 f=[tuple(reversed(range(n))),tuple(range(4*n,5*n))]+[(a*n+i,a*n+(i+1)%n,(a+1)*n+(i+1)%n,(a+1)*n+i)for a in range(4)for i in range(n)]
 g.poly(name,v,f,'stone')


def hydrant(g,o=(0,0,0)):
 x,y,z=o;g.cyl('hydrant_base',(x,y,z),(x,y,z+.13),.19,'steel');g.cyl('hydrant_barrel',(x,y,z+.1),(x,y,z+.83),.13,'red');g.ball('hydrant_cap',(x,y,z+.85),(.16,.16,.12),'red')
 for dx in [-.23,.23]:g.cyl('hose_nozzle',(x,y,z+.6),(x+dx,y,z+.6),.08,'red');g.cyl('hose_cap',(x+dx,y,z+.6),(x+dx*1.1,y,z+.6),.095,'steel')
 g.cyl('front_nozzle',(x,y-.05,z+.4),(x,y-.23,z+.4),.10,'red')
 for i in range(5):g.ring('cap_chain',(x+.2,y-.02,z+.48-i*.04),.025,.007,'steel',axis='Z',detail=2)


def barrel(g,o=(0,0,0),mat='red'):
 x,y,z=o;g.cyl('barrel_body',(x,y,z+.04),(x,y,z+.90),.31,mat)
 for zz in [.07,.30,.64,.88]:g.ring('barrel_ribs',(x,y,z+zz),.31,.025,'steel',axis='Z',detail=1)
 g.cyl('barrel_lid',(x,y,z+.90),(x,y,z+.93),.30,mat)


def bench(g,picnic=False,o=(0,0,0)):
 x,y,z=o
 for xx in [-.70,.70]:
  for yy in [-.19,.19]:g.cyl('bench_legs',(x+xx,y+yy,z),(x+xx,y+yy,z+.48),.04,'steel')
 for i in range(5):g.box('seat_slats',(x,y+(i-2)*.10,z+.5),(1.85,.085,.045),'wood')
 if picnic:
  for i in range(7):g.box('table_planks',(x,y+(i-3)*.12,z+.8),(1.9,.105,.055),'wood')
  for yy in [-.55,.55]:
   g.box('picnic_seat',(x,y+yy,z+.47),(1.95,.28,.05),'wood')
   for xx in [-.65,.65]:g.cyl('A_frame',(x+xx,y+yy,z+.02),(x+xx,y,z+.78),.045,'steel')
 else:
  for xx in [-.75,.75]:g.cyl('back_support',(x+xx,y+.25,z+.25),(x+xx,y+.32,z+1.0),.035,'steel')
  for i in range(3):g.box('back_slats',(x,y+.30,z+.67+i*.12),(1.85,.045,.10),'wood')


def box_fixture(g,kind,o=(0,0,0)):
 x,y,z=o
 if kind in {'news_boxes','parcel_box','utility_box','callbox','payphone','payphone_pair','phone_booth','pagoda_phone_booth'}:
  small=kind in {'callbox','payphone','payphone_pair'};w=.38 if small else .70;h=.55 if small else 1.15
  base=.95 if small else .1
  if small:g.cyl('fixture_post',(x,y,z),(x,y,z+base),.055,'steel')
  mat='red'if kind in {'callbox','pagoda_phone_booth'}else 'blue'if kind in {'news_boxes','payphone','payphone_pair'}else 'green'
  if kind in {'phone_booth','pagoda_phone_booth'}:
   for xx in [-.47,.47]:
    for yy in [-.4,.4]:g.box('booth_posts',(x+xx,y+yy,z+1.1),(.08,.08,2.2),mat)
   for zz in [.2,1.0,2.1]:g.box('booth_front_rails',(x,y-.4,z+zz),(1.,.08,.08),mat)
   for xx in [-.45,.45]:g.box('booth_glazing',(x+xx,y,z+1.45),(.025,.75,1.2),'glass')
   g.box('booth_roof',(x,y,z+2.24),(1.15,1.05,.15),mat)
   if kind=='pagoda_phone_booth':
    for a in [-1,1]:g.box('pagoda_roof_wings',(x+a*.35,y,z+2.5),(.85,1.22,.12),'red',rot=Matrix.Rotation(a*.38,3,'Y'))
   box_fixture(g,'payphone',(x,y+.22,z+.15));return
  g.box('cabinet',(x,y,z+base+h/2),(w,.35,h),mat)
  g.box('recessed_front',(x,y-.19,z+base+h*.62),(w*.78,.035,h*.42),'glass'if kind=='news_boxes'else 'black')
  g.box('coin_slot',(x+w*.22,y-.22,z+base+h*.83),(.055,.02,.018),'chrome',1)
  if kind in {'payphone','payphone_pair'}:
   for xx in [-.08,0,.08]:
    for zz in [0,.06,.12]:g.box('phone_keypad',(x+xx,y-.24,z+base+.22+zz),(.04,.02,.028),'chrome',1)
   g.cyl('handset',(x-.13,y-.25,z+base+.12),(x-.13,y-.25,z+base+.42),.045,'black',detail=1)
   for i in range(12):g.ring('phone_cord',(x-.13,y-.23,z+base-.02-i*.018),.018,.006,'black',axis='Z',detail=2)
 elif kind in {'mailbox','mailboxes'}:
  count=1 if kind=='mailbox'else 4
  for i in range(count):
   xx=x+(i-(count-1)/2)*.42
   g.box('mail_post',(xx,y,z+.55),(.08,.08,1.1),'wood');g.box('mailbox_lower',(xx,y,z+1.2),(.34,.57,.23),'steel');g.ball('mailbox_round_roof',(xx,y,z+1.30),(.17,.28,.14),'steel')
   g.box('mailbox_door',(xx,y-.30,z+1.22),(.32,.035,.3),'blue');g.box('mailbox_flag',(xx+.18,y,z+1.35),(.025,.2,.04),'red',1)
 else:
  mat='blue'if kind=='recycling_pair'else 'green'if kind=='street_bin'else 'steel'
  count=2 if kind in {'bin_pair','recycling_pair'}else 1
  for i in range(count):
   xx=x+i*.72
   g.box('waste_container',(xx,y,z+.5),(.60,.55,.95),mat);g.box('bin_lid',(xx,y,z+1.0),(.66,.61,.07),'black')
   for yy in [-.23,.23]:g.box('bin_handle',(xx,yy+y,z+.89),(.25,.045,.04),'steel',1)


def human(g,o=(0,0,0),variant=None):
 n=variant or g.d['name'];x,y,z=o
 female=any(s in n for s in ['AVGW','BCHW','BKW','BUSW','CTW','DVRW','FEM','FLGRL','HHW','JGW','OLDW','TRSTW'])
 worker=n.startswith(('CTM','CTW','FLM'));biker=n.startswith(('BKM','BKR','BKW'));old=n.startswith('OLD');sport=n.startswith(('JGW','MJOG','TNS','SFR','SK'))
 skin='skin_dark'if n.startswith(('BUSM2','BKR2'))else 'skin';shirt='orange'if worker else 'burgundy'if biker and female else 'black'if biker else 'white'if sport else 'blue'if n.startswith(('BUS','TRST'))else 'red'if old else 'cloth'
 pants='white'if worker and female else 'denim'if biker or worker else 'cloth';height=1.69 if female else 1.82
 def p(a,b,c):return(x+a,y+b,z+c*height/1.82)
 # Sculpted intersecting clothing volumes with defined waist, clavicles and hands.
 g.ball('pelvis',p(0,0,.94),(.20,.125,.19),pants)
 g.ball('torso',p(0,0,1.27),(.235 if not female else .205,.135,.31),shirt)
 g.cyl('neck',p(0,0,1.49),p(0,0,1.59),.064,skin)
 g.ball('head',p(0,-.015,1.69),(.10,.094,.135),skin)
 g.ball('hair_cap',p(0,.008,1.77),(.106,.097,.075),'hair')
 g.ball('nose',p(0,-.105,1.69),(.022,.030,.028),skin,1)
 for s in [-1,1]:
  g.ball('ear',p(s*.10,0,1.69),(.023,.018,.035),skin,1)
  g.ball('eyes',p(s*.04,-.099,1.72),(.012,.009,.010),'black',2)
  hip=p(s*.105,0,.94);knee=p(s*.115,-.018,.55);ankle=p(s*.12,0,.13)
  g.cyl('thigh',hip,knee,.10,pants,r2=.075);g.ball('knee',knee,(.075,.075,.078),pants)
  g.cyl('calf',knee,ankle,.073,pants,r2=.045)
  g.ball('shoe',p(s*.12,-.075,.075),(.072,.15,.065),'black')
  shoulder=p(s*.23,0,1.45);elbow=p(s*.29,-.01,1.16);wrist=p(s*.32,-.04,.96)
  g.ball('shoulder',shoulder,(.10,.11,.11),shirt);g.cyl('upper_arm',shoulder,elbow,.073,shirt,r2=.055)
  g.ball('elbow',elbow,(.056,.056,.06),shirt);g.cyl('forearm',elbow,wrist,.053,shirt if biker or worker or not sport else skin,r2=.034)
  g.ball('hand',p(s*.32,-.045,.90),(.04,.027,.065),skin)
  for f in range(4):g.cyl('fingers',p(s*.32+(f-1.5)*.016,-.06,.88),p(s*.32+(f-1.5)*.016,-.06,.83),.008,skin,detail=2,n=6)
  if biker:g.ball('elbow_protection',elbow,(.065,.075,.07),'black',1)
 g.box('belt',p(0,-.01,1.02),(.38,.27,.045),'black',1)
 for zz in [1.13,1.3,1.43]:g.ball('shirt_buttons',p(0,-.139,zz),(.008,.008,.008),'steel',2)
 if worker:
  g.ball('hardhat',p(0,0,1.82),(.125,.117,.08),'yellow');g.cyl('hardhat_brim',p(0,0,1.79),p(0,0,1.805),.145,'yellow')
  for s in [-1,1]:g.box('vest_reflective_tape',p(s*.115,-.14,1.29),(.035,.012,.34),'white',1)
 if n.startswith(('BUSM','BMN','DAV')):g.box('briefcase',p(.40,-.02,.78),(.09,.34,.26),'wood');g.ring('briefcase_handle',p(.40,-.02,.94),.04,.012,'black',axis='X',detail=1)
 if old:
  for dx in [-.24,.24]:
   for yy in [-.30,-.65]:g.cyl('walker_legs',p(dx,yy,.0),p(dx,yy,.88),.018,'chrome')
   g.cyl('walker_handle',p(dx,-.3,.88),p(dx,-.65,.88),.018,'chrome')
  for zz in [.4,.7]:g.cyl('walker_brace',p(-.24,-.65,zz),p(.24,-.65,zz),.018,'chrome')
 if n.startswith(('FLG','FLGRL')):
  g.cyl('flag_pole',p(.36,-.04,.9),p(.36,-.04,1.65),.018,'wood')
  for a in range(4):
   for b in range(4):g.box('checker_flag',p(.36+(a+.5)*.07,-.04,1.37+b*.07),(.07,.018,.07),'black'if(a+b)%2 else 'white',1)
 if n.startswith(('SKR','SKTR')):
  g.ball('skateboard_deck',p(0,-.13,.035),(.38,.11,.024),'wood')
  for dx in [-.24,.24]:
   for yy in [-.08,.08]:g.ball('skate_wheels',p(dx,yy-.13,.025),(.026,.018,.025),'black',1)
 if n.startswith('SFR'):g.ball('surfboard',p(.40,0,.85),(.075,.23,.85),'yellow')
 if n.startswith('TNS'):
  g.cyl('racket_handle',p(.32,-.04,.9),p(.38,-.04,.62),.025,'black');g.ring('tennis_racket',p(.40,-.04,.47),.16,.016,'steel',axis='X')
 if n.startswith('DVR'):
  for dx in [-.36,.36]:g.ball('diving_fins',p(dx,-.02,.63),(.07,.035,.25),'blue',1)


def animal(g,kind,o=(0,0,0),scale=1):
 x,y,z=o;sheep=kind=='sheep';lion=kind=='guardian_lion';mat='stone'if lion else 'cream_fur'if sheep else 'cow_black'if g.d['name'].startswith(('COWA','COWEEK'))else 'cow_brown'
 length=1.2 if sheep else 1.9;ht=.65 if sheep else .95
 g.ball('animal_torso',(x,y,z+ht),(.35,length*.40,.36),mat)
 for dx in [-.23,.23]:
  for yy in [-length*.3,length*.3]:
   g.cyl('upper_leg',(x+dx,y+yy,z+ht-.12),(x+dx,y+yy+.05,z+.35),.075,mat,r2=.05)
   g.cyl('lower_leg',(x+dx,y+yy+.05,z+.35),(x+dx,y+yy,z+.07),.04,mat)
   g.box('hoof',(x+dx,y+yy-.025,z+.055),(.10,.15,.11),'black'if not lion else 'stone')
 g.ball('animal_neck',(x,y-length*.36,z+ht+.10),(.22,.26,.30),mat)
 g.ball('animal_head',(x,y-length*.52,z+ht+(.02 if g.d['name']=='COWC2'else .22)),(.18,.28,.20),'white'if g.d['name']=='COWB2'else mat)
 g.ball('muzzle',(x,y-length*.65,z+ht+(-.08 if g.d['name']=='COWC2'else .12)),(.17,.17,.12),'black'if sheep else mat)
 for dx in [-.21,.21]:
  g.ball('animal_ears',(x+dx,y-length*.48,z+ht+.34),(.11,.055,.065),mat)
  g.ball('animal_eyes',(x+dx*.78,y-length*.60,z+ht+.29),(.022,.018,.023),'black',1)
  if not sheep:g.cyl('horn',(x+dx*.6,y-length*.47,z+ht+.38),(x+dx*1.2,y-length*.45,z+ht+.55),.04,'white',r2=.008,detail=1)
 g.cyl('tail',(x,y+length*.4,z+ht),(x+.08,y+length*.57,z+ht-.35),.027,mat,r2=.015)
 if sheep:
  for i in range(38 if g.lod==0 else 16):
   a=i*2.4;yy=y+g.rng.uniform(-.36,.36);g.ball('wool_tufts',(x+math.cos(a)*.28,yy,z+ht+math.sin(a)*.29),(.13,.15,.12),'cream_fur',1)
 if lion:
  for i in range(12):a=i*math.tau/12;g.ball('stone_mane',(x+math.cos(a)*.25,y-length*.46,z+ht+.24+math.sin(a)*.25),(.13,.15,.13),'stone',1)
  g.box('statue_plinth',(x,y,z+.10),(1.1,1.7,.2),'stone')


def build(g):
 k=g.d['builder'];n=g.d['name'];r=g.rng
 if k in {'sedan','hatchback','sports_car','pickup','van','camper','police_car','taxi','car_hulk','flatbed_truck','work_truck','fire_engine'}:vehicle(g,k)
 elif k in {'traffic_sign','warning_sign','bus_sign','billboard','produce_sign','civic_board','motel_sign','information_board'}:sign(g,k)
 elif k in {'conifer','broadleaf','palm','bare_tree'}:
  count=3 if n.startswith(('TR5','TRE5'))or n in {'TREE4C2','TREE4D2','TREE5C2','TREE5E2'}else 1
  for i in range(count):tree(g,k,((i-(count-1)/2)*2.0,(i%2)*.5,0),1-.1*(i%2))
  if n=='TREJHYD2':hydrant(g,(-.9,-.5,0))
  if n=='TRJ3CMB2':box_fixture(g,'street_bin',(-.9,-.4,0))
 elif k in {'human','biker_group'}:
  for i in range(5 if k=='biker_group'else 1):human(g,((i-2)*.75 if k=='biker_group'else 0,0,0),'BKW100'if k=='biker_group'and i==2 else None)
 elif k in {'cow','sheep','guardian_lion'}:animal(g,k)
 elif k=='roadside_animal':g.ball('small_roadside_animal',(0,0,.06),(.4,.24,.06),'fur');g.ball('small_animal_head',(0,-.28,.06),(.12,.12,.055),'fur')
 elif k=='hydrant':hydrant(g)
 elif k in {'bench','picnic_table'}:
  bench(g,k=='picnic_table')
  if n=='BNCHMUN2':sign(g,'bus_sign',(1.0,.2,0))
 elif k in {'mailbox','mailboxes','news_boxes','parcel_box','utility_box','callbox','payphone','payphone_pair','phone_booth','pagoda_phone_booth','trash_can','street_bin','bin_pair','recycling_pair'}:
  count=3 if k=='news_boxes'else 2 if k=='payphone_pair'else 1
  for i in range(count):box_fixture(g,k,((i-(count-1)/2)*.8,0,0))
 elif k=='fallen_can':g.cyl('fallen_can',(-.5,0,.31),(.4,0,.31),.30,'steel');g.ball('spilled_rubbish',(.7,0,.12),(.55,.4,.12),'paper')
 elif k in {'utility_pole','street_lamp','lamp_sign'}:
  h=7.0 if k=='utility_pole'else 5.2
  g.cyl('pole',(0,0,0),(0,0,h),.12,'wood'if k=='utility_pole'else 'steel',r2=.065)
  g.cyl('pole_base',(0,0,0),(0,0,.25),.22,'steel')
  if k=='utility_pole':
   for zz in [h-.3,h-.8]:
    g.box('pole_crossarm',(0,0,zz),(1.8,.12,.12),'wood')
    for xx in [-.7,0,.7]:g.cyl('insulators',(xx,0,zz),(xx,0,zz+.18),.06,'white',detail=1)
  else:
   g.cyl('lamp_arm',(0,0,h-.2),(0,-.75,h),.05,'steel');g.ball('lamp_head',(0,-.78,h),(.25,.4,.12),'steel');g.ball('light_diffuser',(0,-.78,h-.08),(.20,.32,.06),'white')
  if k=='lamp_sign':sign(g,'traffic_sign',(.2,0,1.0))
 elif k=='parking_meter':
  g.cyl('meter_post',(0,0,0),(0,0,1.15),.035,'steel');g.ball('meter_head',(0,0,1.32),(.13,.08,.22),'steel');g.box('meter_display',(0,-.08,1.38),(.16,.02,.12),'glass');g.box('meter_coin_slot',(0,-.09,1.22),(.04,.02,.008),'black',1)
 elif k=='service_trike':
  g.box('trike_body',(0,0,.65),(.9,1.65,.65),'blue');g.box('trike_cabin',(0,-.35,1.22),(.85,.8,.9),'glass');g.box('trike_roof',(0,-.35,1.72),(.95,.9,.1),'white')
  wheel(g,0,-.75,.25,.26);wheel(g,-.46,.5,.25,.26);wheel(g,.46,.5,.25,.26);g.cyl('amber_beacon',(0,-.3,1.8),(0,-.3,1.96),.10,'orange')
 elif k in {'rock_cluster','rubble_pile'}:
  count=1 if n.startswith('BOULD')else 4 if k=='rock_cluster'else 15
  for i in range(count):
   sx=r.uniform(.35,.75)if k=='rock_cluster'else r.uniform(.1,.34);rock(g,(r.uniform(-.6,.6),r.uniform(-.4,.4),sx*.55),(sx,sx*.8,sx*.8))
  if n=='PILEGRV2':
   for xx in [-1.,1.]:cone(g,(xx,-.3,0))
 elif k in {'water_patch','oil_patch','ice_patch'}:
  mat={'water_patch':'water','oil_patch':'oil','ice_patch':'ice'}[k]
  for i in range(3):g.ball('surface_hazard',(r.uniform(-.45,.45),r.uniform(-.3,.3),.018),(.6,.43,.018),mat)
 elif k in {'shrub','flower_planter','vine'}:
  if k=='flower_planter':g.box('planter',(0,0,.30),(1.45,.55,.6),'concrete')
  if k=='vine':g.cyl('vine_stake',(0,0,0),(0,0,1.6),.04,'wood')
  for i in range(9 if k!='vine'else 7):
   x=r.uniform(-.5,.5);y=r.uniform(-.3,.3);z=r.uniform(.3,.7)+(.45 if k=='flower_planter'else .0)+i*.10*(k=='vine')
   g.cyl('plant_stem',(x*.4,y*.4,.2),(x,y,z),.018,'bark',detail=1)
   g.ball('foliage',(x,y,z),(.30,.24,.25),'autumn'if n in {'BUSHD2','BUSHH2'}else 'leaf')
   if k=='flower_planter':g.ball('flowers',(x,y,z+.15),(.05,.05,.045),'white',1)
 elif k=='tire':g.ring('discarded_tire',(0,0,.17),.34,.13,'rubber',axis='Z')
 elif k in {'log_stack','stump'}:
  for i in range(1 if k=='stump'else 7):
   x=(i%3-1)*.37;y=0;z=.18+(i//3)*.29
   if k=='stump':g.cyl('stump',(0,0,0),(0,0,.65),.45,'bark');g.cyl('cut_growth_rings',(0,0,.65),(0,0,.66),.43,'wood')
   else:
    g.cyl('log',(x,-.6,z),(x,.6,z),.18,'bark');g.cyl('cut_log_end',(x,-.605,z),(x,-.62,z),.17,'wood')
 elif k=='pallets':
  for level in range(3 if n=='SLATS12'else 6):
   z=level*.16
   for xx in [-.45,0,.45]:g.box('pallet_feet',(xx,0,z+.045),(.14,.8,.09),'wood')
   for yy in [-.42,-.21,0,.21,.42]:g.box('pallet_deck',(0,yy,z+.12),(1.2,.15,.055),'wood')
 elif k in {'cone','checked_bollard','bollard','reflector_post'}:
  if k=='cone':cone(g)
  else:
   h=.95 if k=='checked_bollard'else 1.3
   g.cyl('bollard',(0,0,0),(0,0,h),.14 if k=='checked_bollard'else .07,'white'if k=='checked_bollard'else 'yellow'if n.startswith('PST')else 'wood')
   if k=='checked_bollard':
    for i in range(5):
     for j in range(8):
      a=j*math.tau/8;b=(j+1)*math.tau/8;rr=.155
      v=[(rr*math.cos(t),rr*math.sin(t),zz)for zz in [.12+i*.16,.28+i*.16]for t in [a,b]]
      g.poly('checker_pattern',v,[(0,1,3,2)],'black'if(i+j)%2 else 'white',1)
   else:
    for i in range(5):g.cyl('reflective_bands',(0,0,.15+i*.16),(0,0,.23+i*.16),.075,'black'if i%2 else 'white',detail=1)
 elif k=='barrier' and n=='BRRICDD2':
  for xx in [-.85,.85]:
   g.cyl('crowd_barrier_sides',(xx,0,.2),(xx,0,1.25),.035,'steel');g.box('crowd_barrier_feet',(xx,0,.06),(.14,.60,.12),'steel')
  for zz in [.25,1.25]:g.cyl('crowd_barrier_rails',(-.85,0,zz),(.85,0,zz),.035,'steel')
  for i in range(11):g.cyl('crowd_barrier_pickets',((i-5)*.14,0,.25),((i-5)*.14,0,1.25),.018,'steel')
  g.box('road_closed_plate',(0,-.025,.78),(.75,.025,.45),'white');g.text('ROAD\nCLOSED',(0,-.045,.78),.12,'black')
 elif k=='barrier':
  for xx in [-.65,.65]:
   g.box('barrier_leg',(xx,0,.55),(.08,.12,1.1),'steel');g.box('barrier_foot',(xx,0,.04),(.18,.75,.08),'steel')
  g.box('barrier_board',(0,0,.95),(1.7,.12,.32),'white')
  for i in range(6):
   xx=(i-2.5)*.24
   g.poly('painted_diagonal_stripe',[(xx-.08,-.062,.79),(xx+.06,-.062,.79),(xx+.18,-.062,1.11),(xx+.04,-.062,1.11)],[(0,1,2,3)],'red',1)
  if n=='BRRICDB2':g.ball('barrier_beacon',(-.5,0,1.29),(.10,.07,.12),'orange')
 elif k in {'barrel','crash_barrels','barrel_wagon'}:
  for i in range(3 if k=='crash_barrels'and n=='CONTNBY2'else 4 if k=='barrel_wagon'else 1):barrel(g,((i-1.5)*.65 if k=='barrel_wagon'else (i-1)*.64 if n=='CONTNBY2'else 0,0,.4 if k=='barrel_wagon'else 0),'yellow'if k=='crash_barrels'else 'wood'if k=='barrel_wagon'else 'red')
  if k=='barrel_wagon':
   g.box('wagon_deck',(0,0,.35),(2.8,.9,.12),'wood')
   for xx in [-1.,1.]:
    for yy in [-.55,.55]:g.ring('wagon_wheels',(xx,yy,.3),.25,.04,'wood',axis='Z')
 elif k in {'hay_bales','trash_bags','paper_pile','box_pile'}:
  count=6 if k=='hay_bales'else 4 if k in {'trash_bags','box_pile'}else 12
  for i in range(count):
   x=(i%3-1)*.53;y=(i//3)*.42;z=.20
   if k=='hay_bales':
    g.box('hay_bale',(x,y,z),(.5,.4,.4),'hay')
    for yy in [-.12,.12]:g.box('bale_twine',(x,y+yy,z),(.51,.012,.41),'wood',1)
   elif k=='trash_bags':g.ball('rubbish_bag',(x,y,.24),(.27,.23,.24),'black');g.cyl('bag_knot',(x,y,.43),(x+.02,y,.5),.04,'black',r2=.012,detail=1)
   elif k=='box_pile':g.box('cardboard_box',(x,y,z),(.5,.4,.4),'paper');g.box('box_tape',(x,y,z+.205),(.08,.4,.012),'wood',1)
   else:g.box('loose_paper',(r.uniform(-.5,.5),r.uniform(-.4,.4),.01+i*.003),(.32,.22,.004),'paper',rot=Matrix.Rotation(r.random(),3,'Z'))
 elif k=='produce_stand':
  g.box('produce_bin',(0,0,.75),(2.3,.8,.6),'wood')
  for x in [-1.,1.]:g.box('stand_legs',(x,0,.35),(.1,.65,.7),'white')
  for i in range(30):g.ball('produce_fruit',(r.uniform(-1.,1.),r.uniform(-.3,.3),1.05),(.09,.08,.08),'red'if i%3==0 else 'yellow'if i%3==1 else 'green',1)
 elif k=='windmill':
  for dx,dy in [(-.7,-.5),(.7,-.5),(0,.65)]:g.cyl('windmill_tower',(dx,dy,0),(0,0,4.0),.04,'steel')
  for i in range(16):
   a=i*math.tau/16;v=[(.3*math.cos(a),-.05,4+.3*math.sin(a)),(1.1*math.cos(a),-.05,4+1.1*math.sin(a)),(1.1*math.cos(a+.2),-.05,4+1.1*math.sin(a+.2)),(.3*math.cos(a+.2),-.05,4+.3*math.sin(a+.2))];g.poly('windmill_sails',v,[(0,1,2,3)],'steel')
 elif k=='arrow_trailer':
  g.box('trailer_chassis',(0,0,.45),(1.8,1.3,.15),'orange')
  for xx in [-.95,.95]:wheel(g,xx,0,.3,.3)
  for xx in [-.65,.65]:g.box('arrow_board_legs',(xx,0,1.15),(.08,.1,1.4),'orange')
  g.box('arrow_board',(0,0,2.),(2.2,.15,1.0),'black')
  for i in range(7):g.ball('arrow_bulbs',((i-3)*.25,-.09,2.),(.06,.025,.06),'yellow',1)
  for i in range(3):
   for s in [-1,1]:g.ball('arrow_head_bulbs',(.75-i*.22,-.09,2.+s*i*.18),(.06,.025,.06),'yellow',1)
 elif k in {'bus_stop_bin','utility_assembly','abutment','retaining_wall','barn'}:
  if k=='bus_stop_bin':sign(g,'bus_sign',(.5,0,0));box_fixture(g,'street_bin',(-.3,0,0))
  elif k=='utility_assembly':
   box_fixture(g,'utility_box');g.cyl('vent_stack',(-.5,0,0),(-.5,0,1.6),.12,'steel');g.cyl('conduit',(.3,0,1.3),(-.5,0,1.3),.05,'steel')
  elif k in {'abutment','retaining_wall'}:
   g.box('masonry_support',(0,0,1.1),(2.2,.5,2.2),'stone');g.box('coping',(0,0,2.25),(2.35,.65,.15),'concrete')
   for xx in [-1.,0,1.]:g.cyl('railing_posts',(xx,0,2.3),(xx,0,3.1),.035,'steel')
   for zz in [2.7,3.1]:g.cyl('railing_rails',(-1.1,0,zz),(1.1,0,zz),.035,'steel')
   if k=='abutment':g.box('cantilever_deck',(.6,0,2.15),(3.4,1.6,.3),'concrete')
  else:
   g.box('barn_walls',(0,0,1.35),(5.,7.,2.7),'green')
   for s in [-1,1]:g.box('barn_roof',(s*1.25,0,3.2),(2.75,7.4,.12),'wood',rot=Matrix.Rotation(s*.4,3,'Y'))
   g.box('barn_door',(0,-3.52,1.1),(2.1,.12,2.2),'wood')
   for i in range(20):g.box('siding_battens',((i-9.5)*.25,-3.52,1.4),(.035,.04,2.6),'leaf_dark',1)
 else:raise ValueError('Unimplemented semantic builder '+k)


def cone(g,o=(0,0,0)):
 x,y,z=o;g.box('cone_base',(x,y,z+.035),(.45,.45,.07),'rubber');g.cyl('cone',(x,y,z+.07),(x,y,z+.72),.18,'orange',r2=.025)
 g.cyl('cone_reflector',(x,y,z+.37),(x,y,z+.49),.113,'white',r2=.084,detail=1)


def export(d,lod):
 col=bpy.data.collections.new(d['id']+'_LOD'+str(lod));bpy.context.scene.collection.children.link(col)
 g=Geo(d,lod);build(g);objects=g.objects(col)
 if not objects:raise ValueError('Empty builder '+d['name'])
 # Ground all parts together, preserving relative origins across the LOD pair.
 low=min(v.co.z for obj in objects for v in obj.data.vertices)
 # Recipes share ground conventions; do not independently recenter lateral axes.
 if low<0:
  for obj in objects:
   for v in obj.data.vertices:v.co.z-=low
 bpy.ops.object.select_all(action='DESELECT')
 for obj in objects:obj.select_set(True)
 bpy.context.view_layer.objects.active=objects[0]
 path=OUT/'models'/f"{d['id']}_lod{lod}.glb"
 bpy.ops.export_scene.gltf(filepath=str(path),export_format='GLB',use_selection=True,export_yup=True,export_materials='EXPORT',export_extras=True,export_apply=True)
 deps=bpy.context.evaluated_depsgraph_get()
 tris=sum(sum(len(p.vertices)-2 for p in o.evaluated_get(deps).data.polygons)for o in objects)
 bounds=[[min((o.matrix_world@v.co)[i]for o in objects for v in o.data.vertices)for i in range(3)],[max((o.matrix_world@v.co)[i]for o in objects for v in o.data.vertices)for i in range(3)]]
 record=dict(lod=lod,file=str(path.relative_to(OUT)),sha256=hashlib.sha256(path.read_bytes()).hexdigest(),triangles=tris,parts=len(objects),bounds_blender=bounds)
 if lod:
  for obj in objects:bpy.data.objects.remove(obj,do_unlink=True)
  bpy.data.collections.remove(col)
 else:col.hide_render=True;col.hide_viewport=True
 return record,dict(g.count)


def main():
 group=sys.argv[sys.argv.index('--')+1]
 designs=[d for d in json.loads((OUT/'designs.json').read_text())['designs']if d['group']==group]
 bpy.ops.wm.read_factory_settings(use_empty=True);materials()
 for folder in ['models','sources','manifests']:(OUT/folder).mkdir(parents=True,exist_ok=True)
 records=[]
 for d in designs:
  lods=[]
  for lod in range(3):
   rec,parts=export(d,lod);lods.append(rec)
   if not lod:geometry=parts
  bounds=lods[0]['bounds_blender'];extent=[bounds[1][i]-bounds[0][i]for i in range(3)]
  records.append(dict(id=d['id'],name=d['name'],builder=d['builder'],group=group,concept=d['concept'],lods=lods,geometry_parts=geometry,
   collider=dict(status='authoring_bounds_only_not_gameplay_validated',kind='box',size_y_up=[extent[0],extent[2],extent[1]],center_y_up=[(bounds[0][0]+bounds[1][0])/2,(bounds[0][2]+bounds[1][2])/2,-(bounds[0][1]+bounds[1][1])/2]),
   rig_status='not_rigged'if group in {'animals','pedestrians_animals'}else 'not_applicable',production_approved=False))
  print('EXPORTED',d['id'],d['name'],[l['triangles']for l in lods],flush=True)
 source=OUT/'sources'/(group+'.blend')
 first=next((c for c in bpy.data.collections if c.name.endswith('_LOD0')),None)
 if first:first.hide_render=False;first.hide_viewport=False
 bpy.ops.wm.save_as_mainfile(filepath=str(source))
 result=dict(group=group,source=str(source.relative_to(OUT)),source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),tool='Blender '+bpy.app.version_string,records=records)
 (OUT/'manifests'/(group+'.json')).write_text(json.dumps(result,indent=2)+'\n')
if __name__=='__main__':main()
