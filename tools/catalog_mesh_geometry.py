"""Explicit architectural and structural mesh vocabulary for the reviewed catalog.
Uses real volumes/openings/roof shapes, never complete facade image planes.
Run only through Blender's Python via build_catalog_batch.py.
"""
import math
import random
import bpy
from mathutils import Vector,Matrix
import build_building_meshes as C


def rod(g,name,a,b,r,mat,detail=1,segments=8):
    a,b=Vector(a),Vector(b);axis=(b-a).normalized()
    u=axis.cross(Vector((0,0,1)))
    if u.length<.01:u=axis.cross(Vector((0,1,0)))
    u.normalize();v=axis.cross(u)
    verts=[tuple(p+r*(u*math.cos(i*2*math.pi/segments)+v*math.sin(i*2*math.pi/segments)))for p in [a,b]for i in range(segments)]
    faces=[tuple(reversed(range(segments))),tuple(range(segments,2*segments))]
    faces +=[(i,(i+1)%segments,(i+1)%segments+segments,i+segments)for i in range(segments)]
    g.poly(name,verts,faces,mat,detail,.002)


def lathe(g,name,c,profile,mat,detail=0,segments=16):
    x,y,z=c;verts=[(x+r*math.cos(i*2*math.pi/segments),y+r*math.sin(i*2*math.pi/segments),z+h)for r,h in profile for i in range(segments)]
    faces=[tuple(reversed(range(segments))),tuple(range((len(profile)-1)*segments,len(profile)*segments))]
    for j in range(len(profile)-1):
        faces +=[(j*segments+i,j*segments+(i+1)%segments,(j+1)*segments+(i+1)%segments,(j+1)*segments+i)for i in range(segments)]
    g.poly(name,verts,faces,mat,detail,.002)


def beam(g,name,a,b,width,depth,mat,detail=0):
    a,b=Vector(a),Vector(b);length=(b-a).length
    z=(b-a).normalized();x=z.cross(Vector((0,1,0)))
    if x.length<.01:x=z.cross(Vector((1,0,0)))
    x.normalize();y=z.cross(x);matrix=Matrix((x,y,z)).transposed()
    g.box(name,tuple((a+b)/2),(width,depth,length),mat,detail,.006,matrix)


def pointed(g,name,x,y,base,w,h,mat='ivory',detail=1):
    shoulder=base+h*.62
    for side in [-1,1]:
        points=[(x+side*w/2,y,base),(x+side*w/2,y,shoulder)]
        points +=[(x+side*w/2*(1-t),y,shoulder+(h*.38)*(2*t-t*t))for t in [i/8 for i in range(1,9)]]
        for a,b in zip(points,points[1:]):beam(g,name,a,b,.10,.18,mat,detail)


def rail(g,x,y,z,w,mat='black_metal',balusters=False):
    g.box('balcony_deck',(x,y+.18,z-.08),(w+.25,.85,.14),'ivory',0,.012)
    for level in [z+.1,z+.75]:rod(g,'balcony_rail',(x-w/2,y-.22,level),(x+w/2,y-.22,level),.035,mat,1)
    n=max(3,int(w/.18))
    for i in range(n+1):
        xx=x-w/2+i*w/n
        if balusters:lathe(g,'stone_baluster',(xx,y-.22,z+.10),[(.04,0),(.065,.10),(.04,.22),(.065,.43),(.04,.59)],'ivory',1,8)
        else:rod(g,'balcony_spindle',(xx,y-.22,z+.1),(xx,y-.22,z+.75),.018,mat,1)


def flower_box(g,x,y,z,w):
    g.box('planter_box',(x,y,z),(w,.35,.25),'old_timber',1,.01)
    for i in range(7):
        xx=x-w*.45+i*w*.15
        lathe(g,'planter_foliage',(xx,y,z+.1),[(.03,0),(.12,.12),(.10,.24),(.01,.3)],'moss_brick',2,6)
        if i%2==0:lathe(g,'planter_flower',(xx,y-.03,z+.33),[(.01,0),(.07,.04),(.01,.08)],'red_paint',2,6)


def stepped_roof(g,w,d,h,mat,trim,kind='flat'):
    fy=-d/2
    g.box('roof_deck',(0,0,h-.12),(w,d,.18),'roof',0,.006)
    for side in [-1,1]:g.box('roof_parapet_side',(side*(w/2-.06),0,h+.13),(.15,d,.45),mat,0,.008)
    g.box('roof_parapet_rear',(0,d/2-.06,h+.13),(w,.15,.45),mat,0,.008)
    if kind not in ['steep_gable','gable_parapet','twin_gable','hip_front','pagoda_green','pagoda_slate']:
        g.box('roof_parapet_front',(0,fy+.02,h+.13),(w,.18,.45),mat,0,.008)
    for z,depth in [(h-.36,.18),(h-.16,.25),(h+.04,.32)]:
        g.box('cornice_front',(0,fy-.07,z),(w+.16,depth,.14),trim,0,.008)
        for side in [-1,1]:g.box('cornice_return',(side*(w/2+.04),0,z),(.16,d,.14),trim,0,.008)
    for i in range(max(5,int(w/.23))):
        x=-w/2+.14+i*.23
        if x>w/2:continue
        g.box('cornice_corbel',(x,fy-.1,h-.54),(.09,.22,.23),trim,1,.008)
    g.box('roof_hvac',(w*.18,.7,h+.40),(.72,.86,.64),'roof',0,.02)
    for i in range(9):g.box('hvac_fins',(w*.18,.25,h+.15+i*.055),(.62,.04,.02),'black_metal',1,.002)
    g.box('chimney',(-w*.25,1.7,h+.53),(.42,.45,1.1),'red_brick',0,.008)
    g.box('chimney_cap',(-w*.25,1.7,h+1.1),(.55,.56,.10),'roof',0,.01)
    return fy


def gable(g,x,y,z,w,d,rise,mat='weathered_cream',roof='slate',detail=0):
    # Explicit triangular gable prism.
    v=[]
    for yy in [y-d/2,y+d/2]:v +=[(x-w/2,yy,z),(x+w/2,yy,z),(x,yy,z+rise)]
    g.poly('gable_wall',v,[(0,2,1),(3,4,5),(0,1,4,3),(1,2,5,4),(2,0,3,5)],mat,detail,.006)
    angle=math.atan2(rise,w/2);length=math.hypot(w/2,rise)
    for side in [-1,1]:
        g.box('pitched_roof_plane',(x+side*w/4,y,z+rise/2),(length+.18,d+.22,.10),roof,detail,.008,Matrix.Rotation(side*angle,3,'Y'))
        beam(g,'gable_bargeboard',(x,y-d/2-.1,z+rise+.07),(x+side*(w/2+.08),y-d/2-.1,z-.03),.11,.12,'ivory',detail)
        for j in range(max(3,int(length/.24))):
            t=(j+.5)/max(3,int(length/.24));xx=x+side*w/2*t;zz=z+rise*(1-t)+.08
            g.box('roof_shingle_course',(xx,y,zz),(length/max(3,int(length/.24))+.025,d+.12,.035),roof,2,.003,Matrix.Rotation(side*angle,3,'Y'))


def pagoda(g,x,y,z,w,d,mat='green_metal'):
    # Curved segmented roof, rising tips, individual cylindrical tile ridges.
    segments=7
    for side in [-1,1]:
        for i in range(segments):
            a=i/segments;b=(i+1)/segments
            def p(t):return (x+side*w*.52*t,z+.65*(1-t)+.32*t**5)
            xa,za=p(a);xb,zb=p(b)
            beam(g,'curved_pagoda_roof',(xa,y,za),(xb,y,zb),.085,d,mat,0)
        for j in range(max(6,int(d/.16))):
            yy=y-d/2+j*d/max(6,int(d/.16))
            points=[]
            for i in range(segments+1):
                t=i/segments;points.append((x+side*w*.52*t,yy,z+.65*(1-t)+.32*t**5+.06))
            for a,b in zip(points,points[1:]):rod(g,'pagoda_tile_roll',a,b,.035,mat,1,6)
        beam(g,'upturned_roof_finial',(x+side*w*.52,y-d/2,z+.32),(x+side*w*.60,y-d/2,z+.75),.07,.07,'gold',1)
    for xx in [-w*.35,0,w*.35]:g.box('pagoda_bracket',(x+xx,y,z-.10),(.17,d*.7,.22),'oxblood',1,.01)


def front_roof(g,s,fy,h):
    w=s['width'];kind=s['roof'];mat=s['masonry']
    if kind in ['steep_gable','gable_parapet']:
        gable(g,0,fy+.65,h-.15,w+(.1 if kind=='steep_gable' else -.05),1.5,1.8 if kind=='steep_gable' else 1.2,mat)
    elif kind=='twin_gable':
        for x in [-w/4,w/4]:gable(g,x,fy+.55,h-.15,w/2,1.3,.9,mat)
    elif kind=='hip_front':
        v=[(-w/2,fy-.1,h),(w/2,fy-.1,h),(w/2,fy+1.7,h),(-w/2,fy+1.7,h),(-w*.16,fy+.5,h+1.5),(w*.16,fy+.5,h+1.5),(w*.16,fy+1.2,h+1.5),(-w*.16,fy+1.2,h+1.5)]
        g.poly('hip_roof',v,C.FACES,'terracotta',0,.008)
    elif kind=='turret':lathe(g,'turret_cone',(0,fy-.24,h),[(w*.42,0),(w*.43,.1),(.04,2.0)],'terracotta',0,12)
    elif kind in ['dutch','pediment','arched_pediment']:
        outline=[(-w/2,h-.1),(-w/2,h+.3),(-w*.35,h+.3),(-w*.22,h+.75),(0,h+1.3),(w*.22,h+.75),(w*.35,h+.3),(w/2,h+.3),(w/2,h-.1)]
        v=[(x,y,z)for y in [fy-.12,fy+.10]for x,z in outline];n=len(outline)
        faces=[tuple(reversed(range(n))),tuple(range(n,2*n))]+[(i,(i+1)%n,(i+1)%n+n,i+n)for i in range(n)]
        g.poly('shaped_parapet',v,faces,mat,0,.008)
        for (x,z),(xx,zz) in zip(outline[1:8],outline[2:9]):beam(g,'parapet_profile',(x,fy-.17,z),(xx,fy-.17,zz),.12,.12,'ivory',0)
        if kind=='arched_pediment':
            C.pane(g,'attic_arch',0,h+.15,.62,.72,'old_timber',fy-.21)
            g.arch('attic_arched_hood',0,fy-.3,h+.44,.38,.12,.18,'ivory')
    elif kind=='battlement':
        for x in [-w/2+.12,-w/4,0,w/4,w/2-.12]:g.box('crenellation',(x,fy-.03,h+.45),(.3,.3,.5),'stone',0,.009)
    elif kind.startswith('pagoda_'):pagoda(g,0,fy+.45,h+.1,w+.3,1.5,'green_metal' if 'green' in kind else 'slate')


def bay(g,x,y,z,w,h,mat='ivory',turret=False):
    projection=.36 if not turret else .52
    g.box('bay_base',(x,y-projection/2,z-h/2),(w+.15,projection+.3,.15),mat,0,.008)
    g.box('bay_crown',(x,y-projection/2,z+h/2),(w+.15,projection+.3,.15),mat,0,.008)
    C.pane(g,'bay_front',x,z,w*.62,h-.13,mat,y=y-projection)
    for side in [-1,1]:
        rotation=Matrix.Rotation(side*.62,3,'Z')
        g.box('bay_cheek_glass',(x+side*w*.39,y-projection*.48,z),(w*.30,.035,h-.25),'glass',0,.003,rotation)
        g.box('bay_cheek_post',(x+side*w*.49,y-.04,z),(.085,.20,h+.02),mat,0,.006)


def door(g,s,fy):
    w=s['width'];kind=s['entry'];mat='red_paint' if 'red_door' in s['features'] else 'old_timber'
    count=2 if kind in ['shop','double_arch','gothic_pair','glazed'] else 1
    usable=w-.38;unit=usable/count
    for i in range(count):
        x=-usable/2+(i+.5)*unit
        if kind=='side_door':x=w*.23;unit=.90
        if kind in ['window','sealed','vent']:continue
        if kind in ['garage','barn','loading','passage']:
            if i:continue
            unit=w-.46;x=0
        if kind=='passage':
            g.box('passage_shadow',(0,fy+1.1,1.42),(w-.5,.1,2.84),'interior',0,0)
            g.arch('passage_arch',0,fy-.10,2.0,(w-.5)/2,.16,.3,'red_brick',0)
            for side in [-1,1]:g.box('passage_pier',(side*(w-.4)/2,fy,1.0),(.23,.4,2.0),'red_brick',0,.01)
            continue
        if kind=='garage':
            g.box('garage_backing',(0,fy+.12,1.6),(unit,.08,3.2),'roof',0,.006)
            for k in range(16):g.box('roller_door_slat',(0,fy-.02,.18+k*.19),(unit-.08,.1,.16),'ivory',1,.004)
        elif kind in ['barn','boarded','loading']:
            for k in range(max(5,int(unit/.16))):g.box('door_board',(x-unit/2+(k+.5)*unit/max(5,int(unit/.16)),fy+.01,1.5),(unit/max(5,int(unit/.16))-.01,.12,3.0),mat,0,.004)
            for z in [.45,2.5]:g.box('door_crossbar',(x,fy-.09,z),(unit,.09,.12),'black_metal',1,.006)
            if kind=='barn':beam(g,'door_diagonal_brace',(x-unit/2,fy-.12,.25),(x+unit/2,fy-.12,2.75),.10,.10,mat,1)
            if kind=='loading':
                g.box('loading_platform',(0,fy-.3,.45),(unit+.1,1,.25),'old_timber',0,.009)
                rail(g,0,fy-.65,.58,unit,'old_timber')
        else:
            C.pane(g,'entry',x,1.6,min(unit-.10,1.15),2.85,mat,y=fy-.06)
            g.box('entry_kickpanel',(x,fy-.10,.68),(min(unit-.18,1.05),.08,.72),mat,0,.008)
            g.box('entry_handle',(x+min(unit,.9)*.3,fy-.24,1.45),(.035,.09,.25),'gold',2,.006)
            if kind in ['arch','double_arch']:
                g.arch('entry_arch',x,fy-.15,2.85,min(unit-.1,1.15)/2,.14,.25,'ivory',0)
                for k in range(7):
                    a=k*math.pi/6
                    rod(g,'fanlight_spoke',(x,fy-.18,2.85),(x+min(unit-.1,1.15)/2*math.cos(a),fy-.18,2.85+min(unit-.1,1.15)/2*math.sin(a)),.012,'black_metal',1)
            if kind in ['gothic','gothic_pair']:pointed(g,'gothic_entry',x,fy-.15,.1,min(unit-.08,1.2),3.1,'ivory',0)
    if kind=='small':g.box('small_service_entry',(0,fy-.03,1.15),(.6,.16,1.7),'black_metal',0,.008)
    if kind=='vent':
        for i in range(7):g.box('service_vent',(0,fy-.05,.35+i*.07),(1.1,.1,.035),'roof',1,.003)


def decorate(g,s,fy):
    w,h=s['width'],s['height'];features=s['features']
    if any(x in features for x in ['full_height_piers','vertical_piers']):
        for i in range(s['columns']+1):
            x=-w/2+.11+i*(w-.22)/s['columns']
            g.box('full_height_pier',(x,fy-.14,h*.56),(.14,.26,h*.83),'ivory',0,.008)
    if 'corner_quoins' in features or 'corner_piers' in features:
        for side in [-1,1]:
            for i in range(int(h/.4)):g.box('corner_quoin',(side*(w/2-.12),fy-.10,.2+i*.4),(.26,.25,.35),'ivory',1,.007)
    if any(x in features for x in ['stone_belt','stone_belts','black_belt','black_belts','floor_bands','heavy_bands']):
        for floor in range(1,s['storeys']):
            z=3.3+(floor-1)*2.6
            g.box('floor_belt',(0,fy-.10,z),(w,.24,.18),'black_metal' if 'black_belt' in ','.join(features) else 'ivory',0,.008)
    if 'rusticated_ground' in features or 'stone_ground' in features:
        for z in [.4,1.,1.6,2.2,2.8]:
            for side in [-1,1]:g.box('rusticated_ground_quoin',(side*(w/2-.16),fy-.15,z),(.38,.3,.45),'ivory',0,.01)
    if any('awning' in f for f in features):
        mat='green_metal' if 'green_awning' in features else 'purple_brick' if 'pink_awning' in features else 'old_timber'
        g.box('storefront_awning',(0,fy-.52,3.55),(w+.06,1.1,.09),mat,0,.01,Matrix.Rotation(.3,3,'X'))
        for side in [-1,1]:beam(g,'awning_brace',(side*w*.4,fy,2.9),(side*w*.4,fy-.9,3.3),.04,.04,'black_metal',1)
    if 'sign_board' in features or 'blue_sign' in features:
        g.box('storefront_sign',(0,fy-.13,3.6),(w-.3,.14,.5),'slate' if 'blue_sign' in features else 'black_metal',0,.01)
    if any('corrugated'==f or 'metal_cladding'==f for f in features):
        # Front corrugated metal has real fold/rib volume, not flat new colors.
        for i in range(int(w/.14)):
            x=-w/2+(i+.5)*w/int(w/.14)
            g.box('cladding_vertical_rib',(x,fy-.11,h*.59),(.045,.10,h*.74),'rust' if i%4==0 else s['masonry'],1,.004)
    if 'star_bolts' in features:
        for x in [-w*.32,0,w*.32]:
            for z in [h*.54,h*.68,h*.82]:
                for angle in [0,math.pi/4]:g.box('star_bolt',(x,fy-.13,z),(.24,.07,.045),'roof',2,.004,Matrix.Rotation(angle,3,'Y'))
    if any('boarded' in f for f in features):
        for z in [h*.5,h*.72]:
            for x in [-w*.22,w*.22]:g.box('window_board',(x,fy-.24,z),(.74,.08,.2),'old_timber',1,.005,Matrix.Rotation(.12 if x<0 else -.1,3,'Y'))
    if 'metal_straps' in features or 'rust_belts' in features:
        for z in [h*.3,h*.6,h*.88]:g.box('metal_strap',(0,fy-.13,z),(w,.13,.22),'rust',1,.005)
    if 'diamonds' in features or 'relief_panels' in features or 'crest' in features:
        for z in [3.5,h-.9]:
            g.box('stone_relief_panel',(0,fy-.13,z),(w*.52,.17,.6),'ivory',1,.01)
            g.box('raised_crest',(0,fy-.27,z),(.26,.08,.26),'gold',1,.008,Matrix.Rotation(math.pi/4,3,'Y'))
    if 'steps' in features:
        for i in range(6):g.box('entry_step',(0,fy-1.55+i*.22,.08+i*.13),(1.8,1.6-i*.21,.16),'concrete',0,.008)
        for side in [-1,1]:
            if 'brick_stair_walls' in features:g.box('stair_cheek_wall',(side*1.02,fy-.9,.8),(.27,1.95,1.6),'red_brick',0,.01)
            else:beam(g,'stair_handrail',(side*.9,fy-1.6,.9),(side*.9,fy-.15,1.7),.05,.06,'black_metal',1)
    if 'picket_fence' in features:
        for i in range(15):
            x=-w/2+i*w/14;g.box('fence_picket',(x,fy-.75,.55),(.08,.08,1.05),'ivory',1,.008)
        for z in [.25,.75]:g.box('fence_rail',(0,fy-.7,z),(w,.08,.07),'ivory',1,.004)
    if 'planter' in features:flower_box(g,0,fy-.4,.3,w*.75)
    if 'bollards' in features:
        for side in [-1,1]:lathe(g,'protective_bollard',(side*(w/2-.08),fy-.3,0),[(.08,0),(.08,1),(.06,1.06)],'gold',0,8)
    if 'stone_base' in features:g.box('stone_base_front',(0,fy-.02,.55),(w,.25,1.1),'stone',0,.015)


def make_facade(s):
    g=C.Geo(s['id']);w,d,h=s['width'],s['depth'],s['height'];fy=-d/2;rng=random.Random(s['id'])
    collection=bpy.data.collections.new(s['id']+' editable catalog model');bpy.context.scene.collection.children.link(collection)
    windows=[]
    for floor in range(s['storeys']-1):
        z=4.8+floor*2.6
        for i in range(s['columns']):
            x=(i-(s['columns']-1)/2)*(w-.5)/s['columns'];ww=(w-.65)/s['columns']*.70
            if s['window'] in ['bay','turret_bay']:ww=(w-.65)/s['columns']*.90
            if s['window']=='slender':ww*=.7
            if s['window']=='small':ww*=.63
            if s['window']=='industrial':ww=w-.55
            if s['window'] not in ['none','loading']:windows.append((x,z,ww,1.6))
    holes=[(0,1.65,w-.4,3.3)] if s['entry'] not in ['sealed','window','vent'] else []
    holes +=windows
    masonry=s['masonry'];brick=masonry not in ['blue_plaster','green_plaster','black_metal','rust']
    C.wall(g,'front',w,h,(0,fy+.02),'front',holes,masonry,rng,brick)
    sideholes=[(y,z,.7,1.5)for y in [-1.3,1.4]for z in [2.2,h*.62]]
    side_mat='red_brick' if masonry not in ['stone','red_brick','blue_brick'] else masonry
    for side,name in [(-1,'left'),(1,'right')]:
        C.wall(g,name,d,h,(side*(w/2-.02),0),'side',sideholes,side_mat,rng,True)
        for y,z,ww,hh in sideholes:
            g.box('side_glazing',(side*(w/2-.08),y,z),(.03,ww,hh),'glass',0,.001)
            for zz in [z-hh/2,z+hh/2]:g.box('side_sill',(side*(w/2+.13),y,zz),(.25,ww+.12,.09),'ivory',0,.006)
            for yy in [y-ww/2,y+ww/2]:g.box('side_jamb',(side*(w/2+.10),yy,z),(.20,.07,hh),'old_timber',0,.004)
        g.box('downpipe',(side*(w/2+.17),d/2-.2,h/2),(.07,.08,h),'roof',1,.008)
    C.wall(g,'rear',w,h,(0,d/2-.02),'front',[(0,z,1.,1.5)for z in [2.2,h*.62]],side_mat,rng,True)
    for z in [2.2,h*.62]:g.box('rear_glazing',(0,d/2-.01,z),(.94,.04,1.44),'glass',0,.001)
    g.box('foundation',(0,0,.12),(w+.15,d+.15,.24),'concrete',0,.01)
    g.box('entry_threshold',(0,fy-.23,.08),(w+.1,.52,.16),'concrete',0,.01)
    door(g,s,fy)
    for x,z,ww,hh in windows:
        style=s['window'];mat='green_metal' if 'green_shutters' in s['features'] or s['id'].startswith('CHINA') else 'ivory'
        if style in ['bay','turret_bay']:bay(g,x,fy-.03,z,ww,hh,'ivory',style=='turret_bay')
        else:
            C.pane(g,'catalog_window',x,z,ww,hh,mat,y=fy-.09,shutter=style in ['shutters','boarded'])
            if style=='arched':g.arch('window_arch',x,fy-.18,z+hh/2-.06,ww/2,.12,.23,'ivory',0)
            if style=='gothic':
                pointed(g,'pointed_window',x,fy-.2,z-hh/2,ww,hh+.3,'ivory',0)
                rod(g,'gothic_mullion',(x,fy-.21,z-hh/2),(x,fy-.21,z+hh/2),.025,'ivory',1)
            if style=='lattice' or 'wire_glass' in s['features']:
                for i in [-.3,0,.3]:
                    rod(g,'window_lattice',(x-ww/2,fy-.22,z+i),(x+ww/2,fy-.22,z+i+.45),.012,'gold',1,6)
            if style=='industrial':
                for i in range(1,6):g.box('industrial_grid_vertical',(x-ww/2+i*ww/6,fy-.14,z),(.026,.08,hh),'rust',1,.003)
                for i in range(1,4):g.box('industrial_grid_horizontal',(x,fy-.14,z-hh/2+i*hh/4),(ww,.08,.026),'rust',1,.003)
        if any(b in s['features']for b in ['balcony','balconies']):rail(g,x,fy-.35,z-hh/2-.12,ww+.2,'ivory' if s['id']=='CHINA16' else 'black_metal',s['id']=='CHINA16')
        if 'flower_boxes' in s['features']:flower_box(g,x,fy-.35,z-hh/2-.2,ww)
        if 'triangular_hood' in s['features']:
            for side in [-1,1]:beam(g,'triangular_window_hood',(x,fy-.2,z+hh/2+.4),(x+side*ww*.65,fy-.2,z+hh/2+.05),.08,.18,'oxblood',0)
        if 'scrolls' in s['features'] or 'scroll_pediment' in s['features']:
            for side in [-1,1]:g.arch('scroll_volute',x+side*ww*.35,fy-.2,z+hh/2+.16,.12,.05,.1,'ivory',1,8)
    stepped_roof(g,w,d,h,masonry,s['trim'],s['roof']);front_roof(g,s,fy,h)
    if 'attic' in s['features'] and s['roof'] in ['steep_gable','gable_parapet','twin_gable']:
        C.pane(g,'gable_attic_light',0,h+.2,.55,.45,'old_timber',fy-.12)
    if s['id'].startswith('CHINA'):
        for level in range(1,s['storeys']):pagoda(g,0,fy-.15,3.25+(level-1)*2.6,w+.10,1.1,'green_metal' if 'green' in s['roof'] or s['id'] in ['CHINA14','CHINA15'] else 'slate')
        if 'red_columns' in s['features']:
            for x in [-w*.4,w*.4]:rod(g,'red_store_column',(x,fy-.2,.2),(x,fy-.2,3.2),.09,'red_paint',0,10)
        if 'lanterns' in s['features']:
            for x in [-w*.3,w*.3]:lathe(g,'hanging_lantern',(x,fy-.45,3.0),[(.06,0),(.18,.12),(.20,.3),(.12,.46),(.04,.5)],'red_paint',1,10)
        if 'gold_sign' in s['features']:
            g.box('gold_shop_sign',(0,fy-.25,3.05),(w-.3,.18,.45),'oxblood',0,.012)
            for i in range(6):
                x=-w*.33+i*w*.13
                for j in [-1,0,1]:g.box('embossed_abstract_sign_stroke',(x,fy-.36,3.05+j*.07),(.16,.025,.025),'gold',1,.003)
        if 'pottery' in s['features']:
            for x in [-1.1,.9]:lathe(g,'ceramic_shop_urn',(x,fy-.35,.16),[(.14,0),(.22,.2),(.19,.38),(.1,.5),(.13,.55)],'ivory',1,10)
    decorate(g,s,fy)
    for i in range(25):
        x=rng.uniform(-w*.45,w*.45);z=rng.uniform(.4,h-.5)
        if not C.intersect((x,z,.12,.1),holes):g.box('surface_chip',(x,fy-.13,z),(.05+rng.random()*.08,.02,.03+rng.random()*.06),'concrete',2,.002)
    objects=g.flush(collection)
    info=dict(id=s['id'],asset_type=s['asset_type'],builder=s['builder'],reference_sheet=s['reference_sheet'],geometry_parts=dict(g.count),description=s['review_note'],body_dimensions_m=[w,d,h])
    return collection,objects,info
