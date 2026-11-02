"""Purpose-built standalone structures and usable surface modules, from reviewed sheets."""
import math
import random
import bpy
from mathutils import Matrix,Vector
import build_building_meshes as C
from catalog_mesh_geometry import rod,lathe,beam,pointed,rail,flower_box,gable,pagoda


def plank_wall(g,name,width,height,origin,axis,holes,mat,rng,horizontal=False):
    C.wall(g,name,width,height,origin,axis,holes,mat,rng,False)
    step=.18 if not horizontal else .22
    count=math.ceil((height if horizontal else width)/step)
    for i in range(count):
        u=0 if horizontal else -width/2+(i+.5)*width/count
        z=(i+.5)*height/count if horizontal else height/2
        ww=width if horizontal else width/count-.012;hh=height/count-.012 if horizontal else height
        # Split planks against opening boundaries, rather than painting openings on them.
        if horizontal:
            edges=sorted({-width/2,width/2,*[x for hx,hz,hw,hhole in holes for x in [hx-hw/2,hx+hw/2]if -width/2<x<width/2]})
            spans=[((a+b)/2,b-a)for a,b in zip(edges,edges[1:])]
        else:
            edges=sorted({0,height,*[zz for hx,hz,hw,hhole in holes for zz in [hz-hhole/2,hz+hhole/2]if 0<zz<height]})
            spans=[((a+b)/2,b-a)for a,b in zip(edges,edges[1:])]
        for centre,length in spans:
            x0,z0,w0,h0=(centre,z,length,hh)if horizontal else(u,centre,ww,length)
            if C.intersect((x0,z0,w0-.002,h0-.002),holes):continue
            face=-1 if name=='front' else 1
            color=mat+('_dark'if rng.random()<.18 else '_light'if rng.random()<.12 else '')
            if axis=='front':g.box('siding_plank',(origin[0]+x0,origin[1]+face*.10,z0),(w0,.07,h0),color,0,.004)
            else:g.box('siding_plank',(origin[0]+(-.10 if name=='left'else .10),origin[1]+x0,z0),(.07,w0,h0),color,0,.004)


def house_body(g,w,d,h,mat,rng,front_holes,side_windows=True,horizontal=False):
    fy=-d/2
    wood=mat in ['old_timber','red_paint','white_siding']
    wall=plank_wall if wood else C.wall
    for name,width,origin,axis,holes in [('front',w,(0,fy+.02),'front',front_holes),('rear',w,(0,d/2-.02),'front',[(0,2.0,1,1.3)]),('left',d,(-w/2+.02,0),'side',[(y,2.3,1.0,1.5)for y in [-d*.25,d*.25]]if side_windows else []),('right',d,(w/2-.02,0),'side',[(y,2.3,1.0,1.5)for y in [-d*.25,d*.25]]if side_windows else [])]:
        if wood:wall(g,name,width,h,origin,axis,holes,mat,rng,horizontal)
        else:wall(g,name,width,h,origin,axis,holes,mat,rng,True)
    g.box('structure_foundation',(0,0,.15),(w+.25,d+.25,.3),'concrete',0,.02)
    if side_windows:
        for side in [-1,1]:
            for y in [-d*.25,d*.25]:
                g.box('structure_side_glass',(side*(w/2-.03),y,2.3),(.04,.95,1.45),'glass',0,.002)
                for yy in [y-.53,y+.53]:g.box('structure_window_jamb',(side*(w/2+.1),yy,2.3),(.2,.1,1.6),'ivory',0,.006)
                for z in [1.52,3.08]:g.box('structure_window_sill',(side*(w/2+.13),y,z),(.25,1.16,.10),'ivory',0,.006)
    for z in [0.4,2.2,h-.3]:
        for side in [-1,1]:g.box('wall_corner_hardware',(side*(w/2-.1),fy-.12,z),(.13,.08,.12),'rust',2,.003)
    return fy


def entry(g,x,y,width,height=2.6,mat='old_timber',glazed=False):
    C.pane(g,'structure_entry',x,height/2+.14,width,height,mat,y=y-.07)
    if not glazed:g.box('structure_door_panel',(x,y-.06,height/2+.12),(width-.08,.09,height-.1),mat,0,.008)
    for z in [.55,height-.3]:g.box('structure_door_rail',(x,y-.13,z),(width,.09,.09),mat,1,.006)
    g.box('door_latch',(x+width*.34,y-.23,1.2),(.04,.10,.2),'gold',2,.006)


def porch(g,w,fy,h,depth=1.8,mat='old_timber',railing=True):
    g.box('porch_floor',(0,fy-depth/2,.22),(w+1,depth,.25),'old_timber',0,.01)
    for x in [-w*.45,w*.45]:
        g.box('porch_column',(x,fy-depth+.12,h/2+.24),(.16,.16,h),mat,0,.008)
        beam(g,'porch_brace',(x,fy-depth+.12,h-.45),(x+( .45 if x<0 else -.45),fy-depth+.12,h),.10,.10,mat,1)
    g.box('porch_roof',(0,fy-depth*.45,h+.15),(w+1,depth+.3,.12),'rust',0,.01,Matrix.Rotation(-.1,3,'X'))
    if railing:
        for x in [-w*.45,w*.45]:
            for y in [fy-depth+.15,fy-.2]:g.box('porch_rail_post',(x,y,.85),(.12,.12,1.25),mat,0,.006)
            for z in [.6,1.3]:g.box('porch_side_rail',(x,fy-depth/2,z),(.09,depth-.2,.09),mat,1,.004)
            beam(g,'porch_diagonal',(x,fy-depth+.2,.6),(x,fy-.2,1.3),.05,.05,mat,1)
    for i in range(3):g.box('porch_step',(0,fy-depth-.35+i*.16,.07+i*.08),(1.5,.7-i*.15,.14),'concrete',0,.008)


def gas_pump(g,x,y,z=.12):
    g.box('pump_base',(x,y,z+.12),(.65,.65,.24),'concrete',0,.015)
    g.box('pump_body',(x,y,z+.8),(.48,.45,1.4),'ivory',0,.045)
    g.box('pump_head',(x,y,z+1.53),(.61,.5,.55),'ivory',0,.045)
    g.box('pump_meter',(x,y-.27,z+1.58),(.43,.035,.24),'black_metal',0,.003)
    for i in range(4):g.box('pump_meter_digit',(x-.14+i*.095,y-.295,z+1.58),(.045,.01,.11),'ivory',1,.002)
    g.box('pump_red_band',(x,y-.246,z+1.2),(.46,.035,.11),'red_paint',1,.003)
    points=[(x+.25,y,z+1.6),(x+.55,y-.05,z+1.3),(x+.60,y-.1,z+.5),(x+.42,y-.12,z+.3),(x+.30,y-.15,z+.7)]
    for a,b in zip(points,points[1:]):rod(g,'fuel_hose',a,b,.022,'black_metal',1,8)
    g.box('fuel_nozzle',(x+.29,y-.17,z+.84),(.05,.09,.28),'roof',1,.009,Matrix.Rotation(.35,3,'Y'))


def tank(g,x,y,h,r,mat='whitewashed_brick',cone=True):
    lathe(g,'tank_shell',(x,y,0),[(r,0),(r,h*.96),(r*.97,h)],mat,0,24)
    for z in [.12,h*.25,h*.5,h*.75,h-.10]:lathe(g,'tank_band',(x,y,z),[(r+.035,0),(r+.035,.06)],'rust',1,24)
    if cone:lathe(g,'conical_tank_roof',(x,y,h),[(r+.12,0),(.10,r*.5)],'rust',0,24)
    else:lathe(g,'tank_lid',(x,y,h),[(r,0),(r*.9,.18),(r*.4,.26)],'roof',0,24)
    for side in [-1,1]:rod(g,'tank_ladder_rail',(x+side*.24,y-r-.08,.2),(x+side*.24,y-r-.08,h-.1),.025,'rust',1)
    for i in range(int(h/.3)):rod(g,'tank_ladder_rung',(x-.24,y-r-.1,.25+i*.3),(x+.24,y-r-.1,.25+i*.3),.018,'roof',1)
    for i in range(16):
        a=i*2*math.pi/16
        lathe(g,'tank_rivet',(x+(r+.05)*math.cos(a),y+(r+.05)*math.sin(a),h*.5),[(.025,0),(.025,.045)],'roof',2,6)


def geometry_surface(g,s,rng):
    w,d,h=s['width'],s['depth'],s['height'];kind=s['builder'];fy=-d/2
    if kind in ['brick_pier','concrete_pier','vent_pier']:
        w=1.25;d=1.15;h=3
        mat='blue_brick' if kind=='brick_pier'else'concrete'
        g.box('surface_pier_body',(0,0,h/2),(w,d,h),mat,0,.015)
        for z in [.1,h-.08]:g.box('surface_pier_coping',(0,0,z),(w+.18,d+.16,.16),'ivory',0,.012)
        if kind=='brick_pier':
            for i in range(15):g.box('pier_brick_course',(0,-d/2-.03,.2+i*.18),(w,.08,.13),'brown_stone'if i%3==0 else'blue_brick',2,.008)
        if kind=='vent_pier':
            for i in range(5):g.box('pier_louver',(0,-d/2-.08,2.2+i*.08),(.80,.16,.035),'roof',1,.003)
        else:
            for z in [.8,1.0,1.2]:g.box('pier_banding',(0,-d/2-.04,z),(w+.1,.15,.08),'ivory',1,.008)
    elif kind in ['stone_panel','stone_strip','vent_panel']:
        g.box('stone_panel_backing',(0,0,h/2),(w,d,h),'stone',0,.01)
        for row in range(max(2,int(h/.23))):
            z=.12+row*.23
            for i in range(max(3,int(w/.35))):
                x=-w/2+.18+i*.35
                g.box('raised_stone',(x,fy-.07,z),(.31,.13,.19),'moss_brick'if (row+i)%5==0 else'brown_stone'if i%3==0 else'stone',2,.04)
        if kind=='vent_panel':
            for row in range(3):
                for i in range(4):
                    x=-w/2+(i+.5)*w/4;z=h*.65+row*h*.1
                    g.box('vent_block_jamb',(x-w/8+.04,fy-.07,z),(.08,.23,.30),'concrete',0,.006)
                    g.box('vent_block_lintel',(x,fy-.07,z+.14),(w/4,.23,.07),'concrete',1,.006)
        if s['id']=='TOP3A':g.box('staggered_cap_course',(w*.06,-.03,h+.07),(w*.88,d+.04,.15),'stone',1,.01)
    elif kind in ['metal_panel','metal_strip','aggregate_panel','concrete_panel','aggregate_strip']:
        mat='green_metal'if kind=='metal_strip'else'concrete'if 'concrete'in kind or 'aggregate'in kind else'roof'
        g.box('surface_panel_body',(0,0,h/2),(w,d,h),mat,0,.012)
        bands=[h*.22,h*.35]if s['id']=='TUNN32RC'else[h*.15,h*.85]
        for z in bands:g.box('surface_strap',(0,fy-.045,z),(w,.10,.10),'rust'if kind=='metal_panel'else'ivory',1,.006)
        if kind=='metal_panel':
            for x in [-w*.22,0]:g.box('hinge_bar',(x,fy-.08,h/2),(.10,.13,h),'black_metal',0,.008)
            beam(g,'diagonal_metal_brace',(-w*.45,fy-.13,.18),(w*.45,fy-.13,h-.18),.11,.08,'ivory',1)
            if s['id']=='METL22CC':lathe(g,'gate_hinge',(0,fy-.17,h*.2),[(.09,0),(.09,h*.6)],'roof',0,12)
        for i in range(12):
            x=-w*.44+(i%6)*w*.176;z=h*(.1 if i<6 else .9)
            g.box('surface_bolt',(x,fy-.12,z),(.055,.04,.055),'gold',2,.008)
    elif kind in ['wood_strip','timber_panel']:
        g.box('wood_panel_backing',(0,0,h/2),(w,d,h),'old_timber',0,.008)
        vertical=kind=='timber_panel';n=max(3,int((w if vertical else h)/.18))
        for i in range(n):
            if vertical:g.box('timber_board',(-w/2+(i+.5)*w/n,fy-.04,h/2),(w/n-.01,.12,h),'old_timber_dark'if i%3==0 else'old_timber',0,.006)
            else:g.box('timber_board',(0,fy-.04,(i+.5)*h/n),(w,.12,h/n-.01),'old_timber_dark'if i%3==0 else'old_timber',0,.006)
            for x in [-w*.43,w*.43]:g.box('timber_bolt',(x,fy-.12,h*.2+i*.13),(.05,.05,.05),'roof',2,.006)
    elif kind=='aggregate_slab':
        g.box('aggregate_slab',(0,0,h/2),(w,d,h),'concrete',0,.015)
        for i in range(150):
            x=rng.uniform(-w*.47,w*.47);y=rng.uniform(-d*.47,d*.47);r=rng.uniform(.025,.065)
            lathe(g,'exposed_aggregate',(x,y,h-.025),[(r,0),(r*.6,.055),(r*.1,.065)],'stone'if i%3 else'brown_stone',2,6)
        for side in [-1,1]:g.box('slab_edge_bevel',(side*w*.49,0,h*.25),(.04,d,h*.45),'concrete',1,.006)
    # Extra wear is geometry, and guarantees fine detail is independently removable.
    for i in range(16):g.box('surface_wear_chip',(rng.uniform(-w*.45,w*.45),fy-.025,rng.uniform(.05,max(.06,h-.05))),(.03,.03,.035),'rust',2,.003)


def make_structure(s):
    g=C.Geo(s['id']);rng=random.Random(s['id']);w,d,h=s['width'],s['depth'],s['height'];kind=s['builder'];features=s['features']
    col=bpy.data.collections.new(s['id']+' editable structure');bpy.context.scene.collection.children.link(col)
    if s['asset_type']=='surface_module':geometry_surface(g,s,rng)
    elif kind in ['barn','shed','hay_shed']:
        mat='red_paint'if 'red_timber'in features else'old_timber';fy=-d/2
        if kind=='hay_shed':
            for x in [-w*.45,0,w*.45]:
                for y in [-d*.45,d*.45]:
                    g.box('hay_shed_post',(x,y,h/2),(.20,.20,h),'old_timber',0,.008)
                    beam(g,'hay_shed_diagonal',(x,y,h-.8),(x+( .6 if x<0 else -.6),y,h),.10,.10,'old_timber',1)
            gable(g,0,0,h,w,d,1.8,'old_timber','rust')
            for row in range(3):
                for i in range(5):
                    for j in range(2 if row<2 else 1):
                        x=-w*.36+i*w*.18;y=j*1.1+.7;z=.55+row*.95
                        g.box('hay_bale',(x,y,z),(w*.16,.95,.87),'straw',0,.05)
                        for off in [-.25,.25]:g.box('bale_binding',(x+off,y-.49,z),(.025,.025,.84),'old_timber',1,.002)
        else:
            holes=[(0,1.9,3.1,3.6)] if kind=='barn'else[(-w*.2,1.7,w*.55,3.1)]
            house_body(g,w,d,h,mat,rng,holes,kind=='barn')
            if kind=='barn':
                for side in [-1,1]:
                    x=side*.77;entry(g,x,fy,1.45,3.5,mat)
                    beam(g,'barn_door_cross_brace',(x-.68,fy-.18,.25),(x+.68,fy-.18,3.5),.12,.10,'ivory'if mat=='red_paint'else'old_timber',0)
                for x in [-1.65,1.65]:g.box('barn_door_frame',(x,fy-.13,1.9),(.15,.24,3.85),'ivory',0,.008)
            else:
                entry(g,w*.32,fy,.90,2.55,'old_timber')
                g.box('work_bench',(-w*.2,fy+.65,1.05),(2.6,.75,.12),'old_timber',0,.008)
                for x in [-w*.2-1,-w*.2+1]:g.box('bench_leg',(x,fy+.65,.54),(.12,.55,1.05),'old_timber',1,.005)
            gable(g,0,0,h,w,d,2.2,mat,'rust'if 'rust_roof'in features else'slate')
            if 'lean_to'in features or 'raised_centre'in features:
                for side in [-1,1]:g.box('lean_to_roof',(side*(w/2+.4),0,h*.57),(2.0,d,.12),'slate',0,.008,Matrix.Rotation(side*.35,3,'Y'))
            if 'cupola'in features:
                g.box('barn_cupola',(0,.7,h+2.45),(1.2,1.3,.9),'red_paint',0,.01)
                gable(g,0,.7,h+2.9,1.45,1.45,.6,'red_paint')
            if 'clerestory'in features:
                for x in [-w*.35,0,w*.35]:g.box('clerestory_vent',(x,fy-.1,h-.65),(1.1,.15,.4),'black_metal',1,.004)
            if 'fenced_pen'in features:
                for x in [-w*.35,w*.35]:
                    for y in [fy-3,fy-1]:g.box('pen_post',(x,y,.75),(.18,.18,1.5),'old_timber',0,.008)
                    for z in [.5,1.0,1.4]:g.box('pen_side_rail',(x,fy-2,z),(.10,2.0,.10),'old_timber',1,.005)
                for z in [.5,1,1.4]:g.box('pen_front_rail',(0,fy-3,z),(w*.75,.1,.1),'old_timber',1,.005)
    elif kind in ['house','cabin','school','shop','ice_shop','grocery','snack_shop','winery','church']:
        mat='stone'if kind=='winery'else'old_timber'if 'old_timber'in features else'white_siding'if 'white_siding'in features else'whitewashed_brick'
        fy=-d/2;holes=[(0,1.55,1.3,2.8)]+[(x,2.05,1.35,1.7)for x in [-w*.3,w*.3]]
        house_body(g,w,d,h,mat,rng,holes,True,mat=='white_siding')
        entry(g,0,fy,1.2,2.65,'old_timber',kind in ['shop','ice_shop','snack_shop'])
        for x in [-w*.3,w*.3]:C.pane(g,'structure_front_window',x,2.05,1.25,1.6,'old_timber',fy-.10)
        if kind in ['grocery','shop','snack_shop']:
            g.box('flat_shop_roof',(0,0,h),(w+.2,d+.2,.22),'roof',0,.01)
            for y in [-d/2,d/2]:g.box('shop_fascia',(0,y,h+.15),(w+.3,.2,.45),'ivory',0,.01)
        elif kind=='winery':
            v=[(-w*.55,-d*.55,h),(w*.55,-d*.55,h),(w*.55,d*.55,h),(-w*.55,d*.55,h),(-w*.25,-d*.2,h+2.5),(w*.25,-d*.2,h+2.5),(w*.25,d*.2,h+2.5),(-w*.25,d*.2,h+2.5)]
            g.poly('winery_hip_roof',v,C.FACES,'terracotta',0,.012)
            for x in [-w*.28,w*.28]:g.arch('winery_window_arch',x,fy-.15,2.8,.65,.16,.2,'ivory',0)
        else:gable(g,0,0,h,w+.3,d+.3,2.3,mat,'terracotta'if kind=='church'else'rust'if 'rust_roof'in features else'slate')
        if kind in ['house','cabin','winery','grocery']:porch(g,w*.55,fy,2.8,1.8,'ivory'if mat!='old_timber'else'old_timber',kind!='grocery')
        if kind=='school':
            porch(g,w*.28,fy,3.0,1.6,'ivory',False);gable(g,0,fy-.6,3.1,w*.37,2.1,.9,'white_siding','roof')
            rod(g,'school_flagpole',(w*.62,fy-.3,0),(w*.62,fy-.3,h+2),.045,'ivory',0)
            for i in range(7):g.box('flag_stripe',(w*.62+.5,fy-.3,h+1.7-i*.09),(1,.025,.09),'red_paint'if i%2==0 else'ivory',1,.001)
            g.box('flag_field',(w*.62+.22,fy-.32,h+1.55),(.43,.03,.34),'slate',1,.002)
        if kind=='church':
            tower=2.8;ty=fy-.15
            g.box('bell_tower',(0,ty,h*.65),(tower,tower,h*1.3),'white_siding',0,.01)
            for z in [1.,3.,5.,h*1.3]:g.box('tower_band',(0,ty,z),(tower+.16,tower+.16,.15),'ivory',1,.009)
            entry(g,0,ty-tower/2,1.2,2.5,'black_metal')
            for side in [-1,1]:
                C.pane(g,'belfry_window',side*.6,h*1.1,.55,1.3,'black_metal',ty-tower/2-.10)
            lathe(g,'church_spire',(0,ty,h*1.3),[(1.7,0),(1.5,.25),(.04,4.5)],'roof',0,4)
            for side in [-1,1]:
                for y in [-d*.28,0,d*.28]:
                    g.box('church_side_gothic_glass',(side*(w/2+.1),y,3.5),(.08,.8,2.3),'glass',0,.003)
            pointed(g,'church_pointed_entry',0,ty-tower/2-.15,.2,1.5,3.,'ivory',0)
        if kind in ['shop','ice_shop','snack_shop','grocery']:
            label={'shop':'MUSIC','ice_shop':'ICE','snack_shop':'SNACK SHOP','grocery':'GROCERY'}[kind]
            C.letters(col,label,0,fy-.19,h-.62,.55,'black_metal',w-.5)
            for x in [-w*.3,w*.3]:
                for z in [.8,1.4,2.0]:
                    g.box('shop_shelf',(x,fy+.25,z),(1.15,.25,.05),'old_timber',1,.004)
                    for i in range(4):g.box('shelf_product',(x-.42+i*.28,fy+.14,z+.15),(.16,.16,.25),'gold'if i%2 else'oxblood',2,.008)
        if kind=='ice_shop':
            x=w*.34;fy-=.9;g.box('ice_freezer',(x,fy,1.5),(2.3,1.2,2.7),'ivory',0,.025)
            C.letters(col,'ICE',x,fy-.63,1.0,.75,'red_paint',1.9)
            for xx in [x-1.1,x+1.1]:g.box('freezer_hinge',(xx,fy-.65,1.5),(.08,.08,.45),'roof',1,.005)
        if kind in ['grocery','snack_shop']:
            g.box('serving_canopy',(0,fy-.7,3.5),(w+.4,1.6,.12),'green_metal'if kind=='grocery'else'roof',0,.009)
        if kind=='cabin':
            rod(g,'stove_pipe',(w*.34,.5,h-1),(w*.34,.5,h+2),.09,'rust',0,10)
            if 'fuel_pump'in features:gas_pump(g,w*.2,fy-2.1)
        if kind in ['house','winery']:
            for x in [-w*.3,w*.3]:C.pane(g,'upper_house_window',x,h-.4,1.0,1.2,'old_timber',fy-.1)
    elif kind in ['gas_station']:
        fy=-d/2;mat='red_brick'if'red_brick'in features else'whitewashed_brick'
        holes=[(-w*.3,1.8,1.2,3.0),(w*.15,1.8,w*.50,3.)]
        house_body(g,w,d,h,mat,rng,holes,True)
        entry(g,-w*.3,fy,1.1,2.8,'old_timber',True)
        if 'garage'in features:
            for i in range(14):g.box('garage_roller',(w*.18,fy-.02,.22+i*.2),(w*.48,.10,.16),'roof',1,.004)
        else:C.pane(g,'station_showroom',w*.16,1.8,w*.48,2.9,'old_timber',fy-.09)
        g.box('station_roof',(0,0,h),(w+.4,d+.4,.25),mat,0,.012)
        canopy_y=fy-2.1
        g.box('forecourt_canopy',(0,canopy_y,h-.1),(w+1,4,.3),'ivory',0,.015)
        for x in [-w*.43,w*.43]:g.box('canopy_steel_post',(x,canopy_y-1.2,(h-.3)/2),(.15,.15,h-.3),'roof',0,.009)
        count=3 if'three_pumps'in features else 2
        for i in range(count):gas_pump(g,(i-(count-1)/2)*1.2,canopy_y-1.0)
    elif kind in ['bus_shelter','portable_toilet','outhouse','phone_shack']:
        fy=-d/2
        if kind=='bus_shelter':
            for x in [-w*.45,w*.45]:
                for y in [-d*.43,d*.43]:g.box('shelter_frame_post',(x,y,h/2),(.09,.09,h),'roof',0,.008)
            g.box('shelter_back_glass',(0,d*.43,h*.49),(w-.2,.035,h-.3),'clear_glass',0,.001)
            for x in [-w*.45,w*.45]:g.box('shelter_side_glass',(x,0,h*.49),(.035,d-.15,h-.3),'clear_glass',0,.001)
            g.box('shelter_bench',(0,d*.22,.7),(w*.7,.45,.10),'old_timber',0,.008)
            for x in [-w*.25,w*.25]:g.box('bench_support',(x,d*.22,.35),(.09,.35,.7),'roof',1,.006)
            g.box('route_board',(w*.26,-.1,1.7),(.8,.07,1.4),'ivory',0,.008)
            for i in range(6):beam(g,'route_map_line',(w*.26-.3,-.15,1.1+i*.15),(w*.26+.3,-.15,1.5+i*.11),.025,.02,'red_paint'if i%2 else'green_metal',1)
            for i in range(10):
                x=-w/2+(i+.5)*w/10;zz=h+.22*math.sin((i+.5)*math.pi/10)
                g.box('curved_shelter_roof',(x,0,zz),(w/10+.03,d+.22,.12),'roof',0,.01,Matrix.Rotation((i-4.5)*.04,3,'Y'))
            g.box('shelter_floor',(0,0,.08),(w+.15,d+.15,.16),'concrete',0,.01)
        elif kind=='portable_toilet':
            g.box('toilet_body',(0,0,h*.46),(w,d,h*.91),'blue_plastic',0,.06)
            g.box('toilet_base',(0,0,.08),(w+.12,d+.12,.16),'roof',0,.02)
            g.box('toilet_door',(0,fy-.035,h*.46),(w*.82,.08,h*.81),'blue_plastic',0,.035)
            for z in [.3,h-.45]:g.box('door_rib',(0,fy-.1,z),(w*.72,.055,.06),'blue_plastic',1,.008)
            for i in range(5):g.box('toilet_vent',(0,fy-.10,h-.33+i*.045),(w*.55,.04,.025),'black_metal',1,.002)
            g.box('toilet_latch',(w*.28,fy-.15,1.2),(.08,.08,.16),'ivory',2,.01)
            for x in [-w*.4,w*.4]:g.box('door_hinge',(x,fy-.1,.65),(.06,.08,.2),'ivory',2,.005)
            for i in range(8):g.box('rounded_toilet_cap',(-w/2+(i+.5)*w/8,0,h+.13*math.sin((i+.5)*math.pi/8)),(w/8+.03,d+.10,.12),'ivory',0,.025)
        else:
            house_body(g,w,d,h,'red_paint'if kind=='outhouse'else'old_timber',rng,[(0,1.2,.8,2.3)],False)
            entry(g,0,fy,min(w-.2,.9),2.2,'old_timber')
            if kind=='outhouse':gable(g,0,0,h,w+.15,d+.15,.45,'old_timber')
            else:
                for i in range(9):g.box('irregular_roof_plank',(-w/2+(i+.5)*w/9,0,h+rng.uniform(-.08,.13)),(w/9-.01,d,.09),'old_timber',0,.006)
                g.box('phone_unit',(0,fy+.4,1.55),(.35,.18,.55),'roof',1,.025)
    elif kind=='gate':
        for x in [-w*.36,w*.36]:
            g.box('gate_stone_pier',(x,0,h*.42),(.85,d*.65,h*.84),'stone',0,.015)
            g.box('gate_base',(x,0,.22),(1.3,d*.8,.44),'concrete',0,.02)
            for z in [1.,2.1,3.2,4.3]:g.box('gate_pier_relief',(x,-d*.35,z),(.6,.13,.7),'ivory',1,.015)
        g.box('gate_lintel',(0,0,h*.83),(w*.88,d*.65,.38),'oxblood',0,.01)
        pagoda(g,0,0,h*.88,w,d+1.,'green_metal')
        for i in range(14):
            x=-w*.3+i*w*.6/13
            beam(g,'gate_lattice',(x,-.1,h*.77),(x+.3,-.1,h*.83),.025,.05,'gold',1)
    elif kind in ['silos','water_tank','septic_tank']:
        if kind=='silos':
            for x,hh,r in [(-3,5.8,1.2),(0,7.1,1.35),(3,8,1.5)]:tank(g,x,0,hh,r)
            tank(g,4.8,.7,2.8,.65)
        elif kind=='water_tank':tank(g,0,0,h,w*.46)
        else:
            tank(g,0,0,h,w*.48,'black_metal',False)
            for z in [i*.12 for i in range(1,int(h/.12))]:lathe(g,'septic_rib',(0,0,z),[(w*.49,0),(w*.49,.035)],'roof',1,24)
            for x in [-.55,.55]:lathe(g,'tank_access_lid',(x,0,h+.12),[(.28,0),(.28,.12),(.24,.16)],'roof',0,12)
            rod(g,'tank_inlet',(0,-w*.48,.55),(0,-w*.65,.55),.18,'gold',0,12)
    elif kind=='fuel_tank':
        # Lathed upright tank rotated onto X, then supported by steel saddle frames.
        oldparts=set(g.parts)
        lathe(g,'horizontal_tank',(0,0,0),[(1.35,0),(1.45,.12),(1.45,w-.12),(1.35,w)],'ivory',0,24)
        for key in list(g.parts):
            if key in oldparts:continue
            verts,faces=g.parts[key];g.parts[key]=([(z-w/2,y,2.4-x)for x,y,z in verts],faces)
        for x in [-w*.34,w*.34]:
            for side in [-1,1]:beam(g,'tank_support_leg',(x,side*1.35,.1),(x,side*.8,1.8),.16,.16,'rust',0)
            g.box('tank_support_crossbar',(x,0,.4),(.18,3,.18),'rust',0,.01)
        for x in [-w/2,w/2]:
            for i in range(12):
                a=i*2*math.pi/12;g.box('tank_end_bolt',(x,1.25*math.cos(a),2.4+1.25*math.sin(a)),(.08,.055,.055),'rust',2,.006)
    elif kind=='stone_tower':
        v=[(-w/2,-d/2,0),(w/2,-d/2,0),(w/2,d/2,0),(-w/2,d/2,0),(-w*.35,-d*.35,h),(w*.35,-d*.35,h),(w*.35,d*.35,h),(-w*.35,d*.35,h)]
        g.poly('tapered_tower_body',v,C.FACES,'stone',0,.008)
        for row in range(int(h/.3)):
            z=(row+.5)*.3;extent=w/2*(1-.3*z/h)
            for i in range(max(3,int(extent*2/.4))):
                x=-extent+.2+i*.4
                if abs(x)<.55 and z<2.5:continue
                g.box('tower_stone_block',(x,-extent-.025,z),(.36,.10,.25),'brown_stone'if i%3 else'stone',2,.025)
        g.box('tower_coping',(0,0,h+.1),(w*.82,d*.82,.25),'ivory',0,.012)
        entry(g,0,-d/2-.05,1.1,2.5,'old_timber')
    else:raise ValueError(f'No actual geometry builder for {s["id"]}: {kind}')
    # Small removable hardware guarantees a meaningful near-to-mid geometry difference.
    if kind in ['barn','house','cabin','school','shop','ice_shop','grocery','snack_shop','gas_station','outhouse','portable_toilet']:
        for i in range(10):
            # Chips hug actual corner wall strips, clear of doors/open bays.
            side=-1 if i%2 else 1
            g.box('structure_weathering_chip',(side*w*.46,-d/2-.13,.3+(i//2)*min(.2,h/8)),(.045,.04,.05),'rust',2,.003)
    if kind=='hay_shed':
        for x in [-w*.45,0,w*.45]:
            for z in [h-.5,h-.25]:g.box('timber_post_bolt',(x,-d*.45-.115,z),(.055,.035,.055),'roof',2,.006)
    if kind=='gate':
        for x in [-w*.36,w*.36]:
            for z in [h*.78,h*.83]:g.box('gate_beam_bolt',(x,-d*.35-.08,z),(.08,.05,.08),'gold',2,.01)
    if kind=='bus_shelter':
        for x in [-w*.25,w*.25]:g.box('bench_mount_bolt',(x,d*.22-.23,.68),(.045,.04,.045),'gold',2,.005)
    objects=g.flush(col)+[o for o in col.objects if o.type=='FONT']
    return col,objects,dict(id=s['id'],asset_type=s['asset_type'],builder=kind,reference_sheet=s['reference_sheet'],geometry_parts=dict(g.count),description=s['review_note'],body_dimensions_m=[w,d,h])
