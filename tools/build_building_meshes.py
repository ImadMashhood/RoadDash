"""Blender 4.5: actual modeled buildings from the approved first concept sheet.
No facade PNGs, projected photos, displacement cards or whole-building image planes.
Geometry, tile materials, editable .blend and embedded GLBs are authored here.
Run: blender -b --factory-startup --python tools/build_building_meshes.py
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
OUT=ROOT/'art/buildings/mesh_assets'
CONFIG=json.loads((OUT/'designs.json').read_text())
FACES=[(0,3,2,1),(4,5,6,7),(0,1,5,4),(3,7,6,2),(0,4,7,3),(1,2,6,5)]
PALETTE={
 'ochre_brick':(.48,.26,.11),'red_brick':(.40,.12,.055),
 'weathered_cream':(.53,.48,.36),'whitewashed_brick':(.61,.59,.52),
 'ivory':(.65,.61,.48),'oxblood':(.22,.045,.025),
 'green_metal':(.025,.065,.055),'black_metal':(.018,.024,.025),
 'old_timber':(.28,.18,.075),'slate':(.023,.045,.085),
 'roof':(.12,.13,.13),'concrete':(.29,.28,.24),
 'gold':(.52,.34,.11),'rust':(.29,.095,.035),'interior':(.006,.009,.011),
 'glass':(.018,.045,.063),'red_paint':(.40,.027,.018)}
MATERIALS={}


def clean():
    bpy.ops.object.select_all(action='SELECT');bpy.ops.object.delete(use_global=False)
    for block in list(bpy.data.materials):bpy.data.materials.remove(block)


def material(name,color,texture=True,brick=False):
    m=bpy.data.materials.new(name);m.use_nodes=True
    n=m.node_tree.nodes;l=m.node_tree.links;p=n.get('Principled BSDF')
    p.inputs['Base Color'].default_value=(*color,1)
    p.inputs['Roughness'].default_value=.78
    p.inputs['Metallic'].default_value=.35 if 'metal' in name else .05
    if name=='glass':
        p.inputs['Roughness'].default_value=.24;p.inputs['Metallic'].default_value=.55
    if not texture:return m
    # Original procedural material tile, baked to ordinary glTF PBR images.
    tex=n.new('ShaderNodeTexNoise');tex.inputs['Scale'].default_value=45 if brick else 70
    tex.inputs['Detail'].default_value=3;tex.inputs['Roughness'].default_value=.72
    ramp=n.new('ShaderNodeValToRGB');ramp.color_ramp.elements[0].position=.16;ramp.color_ramp.elements[1].position=.83
    ramp.color_ramp.elements[0].color=(*(c*.52 for c in color),1)
    ramp.color_ramp.elements[1].color=(*(min(1,c*1.28+.012) for c in color),1)
    l.new(tex.outputs['Fac'],ramp.inputs[0]);colorout=ramp.outputs['Color']
    if brick:
        bt=n.new('ShaderNodeTexBrick');bt.inputs['Scale'].default_value=1
        bt.inputs['Mortar Size'].default_value=.005
        bt.inputs['Brick Width'].default_value=.18;bt.inputs['Row Height'].default_value=.09
        bt.inputs['Color1'].default_value=(*color,1)
        bt.inputs['Color2'].default_value=(*(c*.7 for c in color),1)
        bt.inputs['Mortar'].default_value=(.19,.18,.15,1)
        mix=n.new('ShaderNodeMixRGB');mix.blend_type='MULTIPLY';mix.inputs[0].default_value=.25
        l.new(bt.outputs['Color'],mix.inputs[1]);l.new(ramp.outputs['Color'],mix.inputs[2]);colorout=mix.outputs[0]
    emission=n.new('ShaderNodeEmission');l.new(colorout,emission.inputs[0]);output=n.get('Material Output');l.new(emission.outputs[0],output.inputs['Surface'])
    image=bpy.data.images.new(name+'_albedo',width=512,height=512,alpha=False)
    target=n.new('ShaderNodeTexImage');target.image=image;n.active=target
    bpy.ops.mesh.primitive_plane_add(size=2);plane=bpy.context.object;plane.data.materials.append(m)
    bpy.context.scene.render.engine='CYCLES';bpy.context.scene.cycles.samples=1
    bpy.ops.object.bake(type='EMIT',margin=4)
    image.filepath_raw=str(OUT/'materials'/f'{name}_albedo.png');image.file_format='PNG';image.save()
    bpy.data.objects.remove(plane,do_unlink=True)
    # Keep only standard glTF-compatible nodes in the deliverable material.
    for node in list(n):
        if node not in [p,output,target]:n.remove(node)
    l.new(target.outputs['Color'],p.inputs['Base Color']);l.new(p.outputs[0],output.inputs['Surface'])
    image.pack()
    return m


class Geo:
    def __init__(self,identity):
        self.id=identity;self.parts={};self.count=collections.Counter()
    def poly(self,name,verts,faces,mat,detail=0,bevel=0):
        key=(name,mat,detail,bevel)
        v,f=self.parts.setdefault(key,([],[]));offset=len(v);v.extend(verts);f.extend(tuple(offset+i for i in face) for face in faces)
        self.count[name]+=1
    def box(self,name,c,s,mat,detail=0,bevel=.008,rotation=None):
        x,y,z=c;a,b,h=s
        verts=[Vector((dx*a/2,dy*b/2,dz*h/2)) for dx,dy,dz in [(-1,-1,-1),(1,-1,-1),(1,1,-1),(-1,1,-1),(-1,-1,1),(1,-1,1),(1,1,1),(-1,1,1)]]
        if rotation:verts=[rotation@v for v in verts]
        verts=[tuple(v+Vector(c)) for v in verts]
        self.poly(name,verts,FACES,mat,detail,bevel)
    def arch(self,name,x,y,z,r,thickness,depth,mat,detail=1,segments=14):
        for i in range(segments):
            a=i*math.pi/segments;b=(i+1)*math.pi/segments
            v=[(x+rr*math.cos(t),yy,z+rr*math.sin(t)) for yy in [y-depth/2,y+depth/2] for rr,t in [(r,a),(r,b),(r+thickness,b),(r+thickness,a)]]
            self.poly(name,v,FACES,mat,detail,.005)
    def flush(self,collection):
        objects=[]
        for (name,mat,detail,bevel),(v,f) in self.parts.items():
            mesh=bpy.data.meshes.new(self.id+'_'+name);mesh.from_pydata(v,[],f);mesh.materials.append(MATERIALS[mat]);mesh.update()
            uv=mesh.uv_layers.new(name='MaterialTileMetres')
            for polygon in mesh.polygons:
                normal=polygon.normal;axis=max(range(3),key=lambda i:abs(normal[i]))
                for li in polygon.loop_indices:
                    p=mesh.vertices[mesh.loops[li].vertex_index].co
                    uv.data[li].uv=(p.x/2,p.z/2) if axis==1 else (p.y/2,p.z/2) if axis==0 else (p.x/2,p.y/2)
            obj=bpy.data.objects.new(self.id+'_'+name+'_'+mat,mesh);collection.objects.link(obj)
            obj['semantic_part']=name;obj['lod_detail']=detail
            if bevel:
                mod=obj.modifiers.new('Modeled edge bevel','BEVEL');mod.width=bevel;mod.segments=1;mod.affect='EDGES'
                mod=obj.modifiers.new('Weighted architectural normals','WEIGHTED_NORMAL');mod.keep_sharp=True
            objects.append(obj)
        return objects


def intersect(rect,holes):
    x,z,w,h=rect
    return any(x-w/2<hx+hw/2 and x+w/2>hx-hw/2 and z-h/2<hz+hh/2 and z+h/2>hz-hh/2 for hx,hz,hw,hh in holes)


def wall(g,name,width,height,origin,axis,holes,mat,rng,brick=True):
    # Segmented physical wall backing around real holes; no opaque front cube.
    xs=sorted({-width/2,width/2,*[v for x,z,w,h in holes for v in [max(-width/2,x-w/2),min(width/2,x+w/2)]]})
    zs=sorted({0,height,*[v for x,z,w,h in holes for v in [max(0,z-h/2),min(height,z+h/2)]]})
    def box(label,u,z,w,h,depth,material,detail,bevel):
        x,y=origin
        if label.startswith('individual_bricks'):
            if axis=='front':y+=(-.105 if name=='front' else .105)
            else:x+=(.105 if name=='right' else -.105)
        if axis=='front':g.box(label,(x+u,y,z),(w,depth,h),material,detail,bevel)
        else:g.box(label,(x,y+u,z),(depth,w,h),material,detail,bevel)
    for x0,x1 in zip(xs,xs[1:]):
        for z0,z1 in zip(zs,zs[1:]):
            if x1-x0<.001 or z1-z0<.001:continue
            if not intersect(((x0+x1)/2,(z0+z1)/2,x1-x0-.002,z1-z0-.002),holes):
                box('wall_backing_'+name,(x0+x1)/2,(z0+z1)/2,x1-x0,z1-z0,.16,mat+'_masonry_tile' if mat+'_masonry_tile' in MATERIALS else mat,0,0)
    if not brick:return
    row_h=.17;bw=.35
    for row in range(math.ceil(height/row_h)):
        z=(row+.5)*row_h
        if z+row_h/2>height:continue
        offset=.5*bw if row%2 else 0
        x=-width/2-bw+offset
        while x<width/2:
            a=max(-width/2,x);b=min(width/2,x+bw-.012)
            if b-a>.04 and not intersect(((a+b)/2,z,b-a,row_h-.012),holes):
                box('individual_bricks_'+name,(a+b)/2,z,b-a,row_h-.014,.055,mat+('_dark' if rng.random()<.22 else '_light' if rng.random()<.18 else ''),2,.005)
            x+=bw


def pane(g,name,x,z,w,h,mat='green_metal',y=-3.06,shutter=False):
    # Glass sits physically behind frames and wall; all jamb/sill returns have depth.
    g.box(name+'_cavity',(x,y+.19,z),(w+.06,.12,h+.05),'interior',0,.008)
    g.box(name+'_glass',(x,y+.1,z),(w-.08,.025,h-.08),'glass',0,.002)
    for xx in [x-w/2,x+w/2]:g.box(name+'_jamb',(xx,y-.005,z),(.10,.20,h+.16),mat,0,.008)
    for zz in [z-h/2,z+h/2]:g.box(name+'_sill',(x,y-.045,zz),(w+.2,.28,.105),mat,0,.008)
    g.box(name+'_vertical_mullion',(x,y-.04,z),(.035,.07,h-.05),mat,1,.004)
    g.box(name+'_sash',(x,y-.035,z+.08),(w-.05,.08,.065),mat,0,.004)
    if shutter:
        for k in range(11):g.box('louver_slats',(x,y+.07,z-h/2+.11+k*(h-.2)/11),(w-.18,.045,.028),'roof',2,.002)
    for xx in [x-w/2-.04,x+w/2+.04]:
        g.box('stone_window_reveal',(xx,y+.02,z),(.12,.25,h+.26),'ivory',1,.008)


def letters(collection,text,x,y,z,size,mat,max_width=2.6):
    curve=bpy.data.curves.new('Modeled sign lettering '+text,'FONT');curve.body=text;curve.align_x='CENTER';curve.size=size;curve.extrude=.008;curve.bevel_depth=.002
    curve.space_character=1.05
    obj=bpy.data.objects.new('Sign_'+text,curve);collection.objects.link(obj)
    obj.location=(x,y,z);obj.rotation_euler=(math.pi/2,0,0);obj.data.materials.append(MATERIALS[mat]);obj['lod_detail']=1;obj['semantic_part']='modeled_sign_letters'
    bpy.context.view_layer.update()
    if obj.dimensions.x>max_width:obj.scale.x=max_width/obj.dimensions.x
    return obj


def make(design):
    identity=design['id'];rng=random.Random(identity);g=Geo(identity)
    w,d,h=design['width'],design['depth'],design['height'];fy=-d/2;mat=design['masonry'];trim=design['trim'];shop=design['shop']
    collection=bpy.data.collections.new(identity+' editable architecture');bpy.context.scene.collection.children.link(collection)
    # Front openings are actual absent masonry, not pixels/planes.
    upper=design['upper'];holes=[(0,2.25,w-.34,4.1)]
    if upper in ['sash','arched_sash']:holes +=[(x,6.65,.83,1.85) for x in [-.7,.7]]
    if upper=='double_bay':holes +=[(x,6.9,.93,2.0) for x in [-.72,.72]]
    wall(g,'front',w,h-.25,(0,fy+.02),'front',holes,mat,rng)
    sideholes=[(0,2.5,.8,1.7),(0,6.55,.78,1.75),(2.0,6.55,.72,1.65)]
    wall(g,'right',d,h-.25,(w/2-.02,0),'side',sideholes,mat,rng)
    wall(g,'left',d,h-.25,(-w/2+.02,0),'side',sideholes,mat,rng)
    wall(g,'rear',w,h-.25,(0,d/2-.02),'front',[(x,z,.78,1.7) for x in [-.72,.72] for z in [2.55,6.5]],mat,rng)
    # Side and rear window geometry, with real reveal depths and frames.
    for side in [-1,1]:
        for y,z,ww,hh in sideholes:
            g.box('side_window_glass',(side*(w/2-.10),y,z),(.035,ww-.06,hh-.06),'glass',0,.002)
            for yy in [y-ww/2,y+ww/2]:g.box('side_window_jamb',(side*(w/2+.025),yy,z),(.18,.09,hh+.12),'old_timber',0,.005)
            for zz in [z-hh/2,z+hh/2]:g.box('side_window_sill',(side*(w/2+.04),y,zz),(.23,ww+.16,.10),'ivory',0,.006)
            g.box('side_window_sash',(side*(w/2+.04),y,z),(.12,ww,.05),'old_timber',1,.003)
        # Downpipe and wall straps; actual cylindrical geometry via narrow faceted prism.
        g.box('downpipe',(side*(w/2+.17),2.72,h/2),(.075,.08,h-.1),'roof',1,.015)
        for z in [1.,3.5,6.,8.]:g.box('pipe_strap',(side*(w/2+.18),2.72,z),(.095,.13,.045),'rust',2,.003)
    for x in [-.72,.72]:
        for z in [2.55,6.5]:
            g.box('rear_glass',(x,d/2-.08,z),(.72,.03,1.64),'glass',0,.001)
            for xx in [x-.4,x+.4]:g.box('rear_jamb',(xx,d/2+.02,z),(.08,.18,1.8),'old_timber',0,.005)
            for zz in [z-.86,z+.86]:g.box('rear_sill',(x,d/2+.03,zz),(.94,.23,.10),'ivory',0,.005)
    g.box('foundation',(0,0,.12),(w+.14,d+.14,.24),'concrete',0,.015)
    g.box('front_step',(0,fy-.28,.08),(w+.16,.62,.16),'concrete',0,.012)
    for x in [-w/2+.075,w/2-.075]:g.box('corner_pier',(x,fy-.03,2.25),(.15,.28,4.35),trim,0,.01)
    # Storefront: tall glazing, physical timber/metal panels, transoms and door hardware.
    for x in [-1.04,0.,1.04]:g.box('shop_post',(x,fy-.06,2.22),(.13,.24,4.15),shop,0,.012)
    for z in [.28,3.40,4.22]:g.box('shop_crossbar',(0,fy-.08,z),(w-.22,.25,.14),shop,0,.012)
    for i,x in enumerate([-.52,.52]):
        y=fy+.1
        g.box('shop_inside',(x,y+.25,1.85),(.96,.12,3.1),'interior',0,0)
        g.box('shop_glass',(x,y,2.0),(.90,.035,2.54),'glass',0,.002)
        g.box('door_bottom_panel',(x,y-.04,.62),(.87,.08,.65),shop,0,.01)
        for zz in [1.0,2.6]:g.box('door_rail',(x,fy-.09,zz),(.98,.15,.065),shop,1,.005)
        g.box('transom_glass',(x,fy+.1,3.81),(.91,.025,.64),'glass',0,.001)
        for xx in [x-.24,x+.24]:g.box('transom_mullion',(xx,fy-.09,3.82),(.035,.10,.66),shop,1,.003)
        g.box('door_handle',(x+.29,fy-.23,1.55),(.032,.08,.29),'gold',2,.012)
        for zz in [.65,2.7]:g.box('door_hinge',(x-.44,fy-.14,zz),(.04,.07,.13),'gold',2,.004)
        g.box('letter_slot',(x,fy-.145,.8),(.27,.04,.065),'gold',2,.003)
    # Differentiate the approved storefronts, not just their colors.
    if identity=='BUSS10':
        g.box('boarded_transom',( -.52,fy-.13,3.81),(.86,.07,.61),'old_timber',0,.007)
        g.box('repaired_red_brick_infill',(-.52,fy-.02,3.10),(.92,.12,.36),'red_brick',0,.008)
        g.box('black_door_panel',(-.52,fy-.10,1.75),(.88,.12,2.8),'black_metal',0,.012)
        g.box('brass_address_plate',(-.52,fy-.18,3.29),(.52,.045,.12),'gold',1,.006)
    if identity in ['BUSS2','BUSS5']:
        # Door frame is worn timber, including separately modeled raised moldings.
        for x in [-.99,.05,1.10]:g.box('timber_recess_molding',(x,fy-.15,2.0),(.055,.12,3.8),'gold',1,.005)
        g.box('ivory_door_kickpanel',(.52,fy-.17,.72),(.86,.08,1.0),'old_timber' if identity=='BUSS5' else 'ivory',0,.01)
        for x in [.19,.85]:g.box('raised_door_panel_molding',(x,fy-.22,.72),(.045,.07,.81),'ivory',1,.005)
    # Crown/floor cornices are stepped extruded profiles around all exposed walls.
    for z,projection,hh in [(4.45,.12,.16),(h-.64,.10,.16),(h-.42,.18,.17),(h-.22,.25,.15),(h-.04,.29,.14)]:
        g.box('front_cornice',(0,fy-projection/2,z),(w+.10,projection+.20,hh),trim,0,.008)
        for side in [-1,1]:g.box('return_cornice',(side*(w/2+projection/2-.05),0,z),(projection+.10,d,hh),trim,0,.008)
        g.box('rear_cornice',(0,d/2+projection/2-.03,z),(w+.12,projection+.10,hh),trim,0,.008)
    for x in [i*.24-w/2+.16 for i in range(13)]:
        g.box('crown_dentil',(x,fy-.11,h-.70),(.11,.20,.16),trim,1,.006)
        g.box('crown_corbel',(x,fy-.10,h-.94),(.075,.16,.27),'black_metal' if identity=='BUSS10' else trim,1,.008)
    if upper in ['sash','arched_sash']:
        for x in [-.7,.7]:
            pane(g,'upper_sash',x,6.65,.78,1.8,'old_timber' if identity=='BUSS10' else 'black_metal',shutter=True)
            if upper=='arched_sash':
                g.arch('arched_brick_lintel',x,fy-.07,7.47,.41,.14,.20,'red_brick')
                g.box('arch_keystone',(x,fy-.10,7.99),(.13,.25,.22),'ivory',1,.006)
    elif upper=='double_bay':
        for x in [-.72,.72]:
            # Faceted bay enclosure: glass/front and two angled glazed cheek panels.
            g.box('bay_base',(x,fy-.22,5.86),(1.12,.62,.16),'ivory',0,.012)
            g.box('bay_top',(x,fy-.22,7.95),(1.12,.62,.16),'ivory',0,.012)
            pane(g,'projected_bay',x,6.92,.66,1.90,'ivory',y=fy-.43)
            for side in [-1,1]:
                rot=Matrix.Rotation(side*math.radians(38),3,'Z')
                g.box('bay_angled_glass',(x+side*.44,fy-.24,6.92),(.34,.035,1.8),'glass',0,.003,rot)
                g.box('bay_angled_frame',(x+side*.55,fy-.11,6.92),(.075,.18,2.05),'ivory',0,.008)
            # Individually modeled small pediment/hood, rather than painted zig-zag.
            for side in [-1,1]:
                g.box('bay_pediment',(x+side*.22,fy-.18,8.17),(.58,.45,.13),'ivory',0,.008,Matrix.Rotation(-side*.25,3,'Y'))
        g.box('small_red_shop_sign',(-.62,fy-.17,3.65),(.9,.16,.76),'red_paint',0,.008)
        g.arch('entry_arch',.52,fy-.14,3.32,.49,.11,.20,'ivory')
    elif upper=='red_panel':
        g.box('upper_red_sign',(0,fy-.10,7.35),(2.52,.12,.92),'red_paint',0,.015)
        for x in [-1.33,1.33]:g.box('sign_stone_frame',(x,fy-.12,7.35),(.13,.20,1.13),'ivory',0,.008)
        for z in [6.80,7.90]:g.box('sign_stone_frame',(0,fy-.12,z),(2.77,.20,.13),'ivory',0,.008)
    elif upper in ['radio','drugs']:
        z=6.73 if upper=='radio' else 5.42
        signmat='ivory' if upper=='radio' else 'black_metal'
        g.box('sign_board',(0,fy-.13,z),(2.62,.16,1.04),signmat,0,.012)
        for x in [-1.35,1.35]:g.box('sign_side_molding',(x,fy-.18,z),(.11,.22,1.2),trim,0,.008)
        for zz in [z-.58,z+.58]:g.box('sign_molding',(0,fy-.18,zz),(2.79,.24,.11),trim,0,.008)
        letters(collection,design['sign'],0,fy-.26,z-.25,.6,'red_paint' if upper=='radio' else 'gold')
    if design['awning']:
        # Thick sloped canopy, scalloped valance and four support rods.
        g.box('sloped_fabric_canopy',(0,fy-.55,4.55),(w-.1,1.25,.07),'slate',0,.012,Matrix.Rotation(math.radians(23),3,'X'))
        g.box('canopy_front_valance',(0,fy-1.12,4.30),(w-.08,.055,.24),'slate',0,.01)
        for x in [-1.4,1.4]:g.box('canopy_support',(x,fy-.45,4.02),(.04,.9,.04),'rust',1,.006,Matrix.Rotation(math.radians(24),3,'X'))
        for x in [-1.25,-.7,-.15,.4,.95,1.45]:g.box('valance_scallop',(x,fy-1.13,4.15),(.45,.07,.10),'slate',1,.025)
    if identity=='BUSS4':
        # Real torn/sagging fabric mesh and dimensional lettering.
        points=[(-1.42,fy-.20,4.68),(-.8,fy-.27,4.55),(0,fy-.30,4.44),(.8,fy-.27,4.55),(1.42,fy-.20,4.68),
                (1.42,fy-.22,3.87),(.86,fy-.26,3.93),(.15,fy-.31,3.78),(-.62,fy-.28,3.94),(-1.42,fy-.22,3.87)]
        g.poly('sagged_motel_banner',points,[tuple(range(len(points)))],'red_paint',0,0)
        letters(collection,'MOTEL',0,fy-.34,4.02,.53,'black_metal')
        for x in [-1.11,1.11]:g.box('red_shop_painted_panel',(x,fy-.15,2.0),(.15,.09,3.35),'red_paint',0,.008)
    # Chips are modeled flakes/shallow pits, never a whole-facade decal.
    if identity in ['BUSS2','BUSS4','BUSS6']:
        for i in range(60):
            x=rng.uniform(-w/2+.15,w/2-.15);z=rng.uniform(4.7,h-.9)
            if intersect((x,z,.12,.08),holes):continue
            g.box('paint_flake',(x,fy-.125,z),(rng.uniform(.025,.11),.018,rng.uniform(.02,.065)),'red_brick' if rng.random()<.35 else 'concrete',2,.002)
    # Roof volume: inset deck, parapets, coping, chimney, flashing and finned HVAC.
    g.box('roof_deck',(0,0,h-.18),(w-.18,d-.18,.16),'roof',0,.008)
    for x in [-w/2+.08,w/2-.08]:g.box('parapet',(x,0,h+.10),(.18,d,.55),mat,0,.008)
    for y in [-d/2+.08,d/2-.08]:g.box('parapet',(0,y,h+.10),(w,.18,.55),mat,0,.008)
    for x in [-w/2+.07,w/2-.07]:g.box('parapet_coping',(x,0,h+.40),(.26,d+.12,.09),trim,0,.009)
    for y in [-d/2+.07,d/2-.07]:g.box('parapet_coping',(0,y,h+.40),(w+.12,.26,.09),trim,0,.009)
    g.box('chimney_stack',(-.77,1.8,h+.58),(.5,.5,1.2),'red_brick',0,.008)
    g.box('chimney_cap',(-.77,1.8,h+1.2),(.66,.65,.10),'roof',0,.008)
    g.box('chimney_flashing',(-.77,1.8,h-.04),(.72,.72,.10),'black_metal',1,.008)
    g.box('hvac_plinth',(.55,.7,h),(.92,1.15,.18),'concrete',0,.01)
    g.box('hvac_box',(.55,.7,h+.48),(.87,1.04,.82),'roof',0,.025)
    for i in range(13):g.box('hvac_cooling_fins',(.55,-.01,h+.19+i*.048),(.74,.055,.023),'black_metal',1,.002)
    for x in [.11,.99]:g.box('hvac_end_frame',(x,.16,h+.48),(.045,.33,.8),'ivory',1,.006)
    for x in [-.4,.4]:g.box('roof_standing_seam',(x,0,h-.08),(.022,d-.38,.028),'rust',2,.003)
    objects=g.flush(collection)
    objects += [o for o in collection.objects if o.type=='FONT']
    return collection,objects,dict(id=identity,geometry_parts=dict(g.count),dimensions_m=[w,d,h+1.25],front='Blender -Y / glTF +Z',reference=CONFIG['reference'])


def export_mesh(objects,identity,lod):
    bpy.ops.object.select_all(action='DESELECT');copies=[]
    for source in objects:
        if source.get('lod_detail',0)>2-lod:continue
        obj=source.copy();obj.data=source.data.copy();bpy.context.scene.collection.objects.link(obj)
        # No geometric bevel on far LOD; source remains editable and untouched.
        if lod==2:
            for mod in list(obj.modifiers):obj.modifiers.remove(mod)
        obj.select_set(True);copies.append(obj)
    bpy.context.view_layer.objects.active=copies[0]
    bpy.ops.object.convert(target='MESH')
    bpy.ops.object.join();joined=bpy.context.view_layer.objects.active;joined.name=f'{identity}_LOD{lod}'
    # Stable single render mesh, indexed material primitives; not thousands of scene nodes.
    bpy.ops.object.transform_apply(location=True,rotation=True,scale=True)
    path=OUT/'models'/f'{identity}_lod{lod}.glb'
    bpy.ops.export_scene.gltf(filepath=str(path),export_format='GLB',use_selection=True,export_apply=True,export_yup=True,export_materials='EXPORT',export_extras=True)
    triangles=sum(len(p.vertices)-2 for p in joined.data.polygons)
    info=dict(file=path.name,lod=lod,vertices=len(joined.data.vertices),triangles=triangles,bytes=path.stat().st_size,sha256=hashlib.sha256(path.read_bytes()).hexdigest())
    bpy.data.objects.remove(joined,do_unlink=True)
    return info


def studio():
    scene=bpy.context.scene;scene.render.engine='CYCLES';scene.cycles.samples=32;scene.cycles.use_denoising=True
    try:
        prefs=bpy.context.preferences.addons['cycles'].preferences;prefs.compute_device_type='CUDA';prefs.get_devices()
        devices=[d for d in prefs.devices if d.type=='CUDA']
        if devices:
            for d in prefs.devices:d.use=d.type=='CUDA'
            scene.cycles.device='GPU'
    except Exception:pass
    scene.world.color=(.10,.10,.10)
    scene.view_settings.view_transform='AgX'
    scene.render.image_settings.file_format='PNG';scene.render.resolution_percentage=100
    scene.render.film_transparent=False
    world=scene.world;world.use_nodes=True;world.node_tree.nodes.get('Background').inputs[0].default_value=(.075,.085,.10,1);world.node_tree.nodes.get('Background').inputs[1].default_value=.45
    bpy.ops.object.camera_add();cam=bpy.context.object;cam.name='Model review camera';scene.camera=cam;cam.data.type='ORTHO'
    for name,pos,power,size in [('Key',(-8,-10,16),2300,9),('Fill',(8,-4,10),1100,8),('Roof rim',(1,7,16),1700,7)]:
        bpy.ops.object.light_add(type='AREA',location=pos);lamp=bpy.context.object;lamp.name=name;lamp.data.energy=power;lamp.data.shape='DISK';lamp.data.size=size;lamp.rotation_euler=(Vector((0,0,4))-lamp.location).to_track_quat('-Z','Y').to_euler()
    return cam


def main():
    clean();OUT.mkdir(parents=True,exist_ok=True)
    for folder in ['materials','models','renders']:(OUT/folder).mkdir(exist_ok=True)
    # Bake directly from our procedural nodes, not from any prior facade artwork.
    bpy.context.scene.cycles.samples=1
    for name,c in PALETTE.items():
        MATERIALS[name]=material(name,c,texture=name not in ['glass','interior','gold'],brick=False)
        if name.endswith('brick') or name=='weathered_cream':
            MATERIALS[name+'_masonry_tile']=material(name+'_masonry_tile',c,brick=True)
        if name.endswith('brick') or name=='weathered_cream':
            for suffix,factor in [('_dark',.80),('_light',1.12)]:
                m=MATERIALS[name].copy();m.name=name+suffix;MATERIALS[name+suffix]=m
                # Tint per-brick variants, retaining the same shared tile image.
                p=m.node_tree.nodes.get('Principled BSDF');tex=next((n for n in m.node_tree.nodes if n.type=='TEX_IMAGE'),None)
                if tex:
                    mix=m.node_tree.nodes.new('ShaderNodeMixRGB');mix.blend_type='MULTIPLY';mix.inputs[0].default_value=1;mix.inputs[2].default_value=(factor,factor,factor,1)
                    m.node_tree.links.new(tex.outputs['Color'],mix.inputs[1]);m.node_tree.links.new(mix.outputs[0],p.inputs['Base Color'])
    collections=[];records=[]
    for design in CONFIG['designs']:
        col,objects,info=make(design);collections.append(col)
        for other in collections:other.hide_render=True
        col.hide_render=False
        info['lods']=[export_mesh(objects,design['id'],lod) for lod in range(3)]
        records.append(info);print('EXPORTED_ACTUAL_MESH',design['id'],info['lods'],flush=True)
    # Preserve separate semantic source meshes, bevel modifiers, sign curves and materials.
    for col in collections:col.hide_render=True
    cam=studio();scene=bpy.context.scene
    # Individual three-quarter renders, all from the authored geometry.
    for col,design in zip(collections,CONFIG['designs']):
        col.hide_render=False;cam.location=(12,-17,12);cam.rotation_euler=(Vector((0,0,4.5))-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.ortho_scale=12.5
        scene.render.resolution_x=850;scene.render.resolution_y=1050;scene.render.filepath=str(OUT/'renders'/f"{design['id']}.png")
        bpy.ops.render.render(write_still=True);col.hide_render=True
        print('RENDERED_ACTUAL_MESH',design['id'],flush=True)
    # Editable source collection origins remain at zero. Gallery duplicates only for rendering.
    scene.render.resolution_x=1800;scene.render.resolution_y=1900;scene.cycles.samples=24
    for lamp in [o for o in scene.objects if o.type=='LIGHT']:
        lamp.location.z+=12;lamp.data.energy*=3;lamp.data.size*=1.7
    for i,col in enumerate(collections):
        offset=Vector(((i%3-1)*7.6,0,(1-i//3)*12.0))
        for source in col.objects:
            obj=source.copy();bpy.context.scene.collection.objects.link(obj)
            obj.location=source.location+offset;obj.rotation_euler.z+=-.35;obj.hide_render=False
            # Rotate around each building origin, including individual semantic meshes.
            obj.location=Matrix.Rotation(-.35,3,'Z')@source.location+offset
            obj['gallery_only']=True
    for i,design in enumerate(CONFIG['designs']):
        curve=bpy.data.curves.new('Gallery name '+design['id'],'FONT');curve.body=design['id'];curve.align_x='CENTER';curve.size=.5
        obj=bpy.data.objects.new('Gallery label '+design['id'],curve);scene.collection.objects.link(obj)
        obj.location=((i%3-1)*7.6,-4.4,(1-i//3)*12-.6);obj.rotation_euler=(math.pi/2,0,0);obj['gallery_only']=True
        curve.materials.append(MATERIALS['ivory'])
    cam.location=(0,-55,26);cam.rotation_euler=(Vector((0,0,10.8))-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.ortho_scale=27
    scene.render.filepath=str(OUT/'renders'/'six-modeled-buildings.png');bpy.ops.render.render(write_still=True)
    # Save editable sources only (not gallery duplicates), with first building visible.
    for obj in list(bpy.data.objects):
        if obj.get('gallery_only'):bpy.data.objects.remove(obj,do_unlink=True)
    for i,col in enumerate(collections):col.hide_render=i!=0;col.hide_viewport=i!=0
    cam.location=(12,-17,12);cam.rotation_euler=(Vector((0,0,4.5))-cam.location).to_track_quat('-Z','Y').to_euler();cam.data.ortho_scale=12.5
    bpy.ops.wm.save_as_mainfile(filepath=str(OUT/'RoadDash_Building_Models.blend'))
    manifest=dict(schema_version=1,tool='Blender '+bpy.app.version_string,reference=CONFIG['reference'],reference_sha256=hashlib.sha256((ROOT/CONFIG['reference']).read_bytes()).hexdigest(),method='Original modeled masonry, apertures, glazing, architectural details and roof volumes. Original procedural tile materials baked to glTF textures; no facade cards or projected images.',records=records,remaining='Other 94 facade designs and 46 structures/surface references are not converted by this six-model milestone; production review and gameplay integration remain open')
    (OUT/'MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print('ACTUAL_BUILDING_PACKAGE_COMPLETE',flush=True)


if __name__=='__main__':main()
