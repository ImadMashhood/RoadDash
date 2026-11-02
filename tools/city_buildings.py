"""Independent full-building grammar: front runs, roofs, rear walls and end returns.
Uses actual RoadDash geometric facade modules; never the rejected flat-art study.
"""
import json
import math
import random
from collections import defaultdict
from pathlib import Path
from build_facade_prototype import Model
ROOT=Path(__file__).resolve().parents[1];KIT=ROOT/'art/buildings/mesh_assets'


def pools(course):
    catalog=json.loads((KIT/'districts/districts.json').read_text());assets={a['id']:a for a in catalog['assets']}
    designs={d['id']:d for d in json.loads((KIT/'catalog_designs.json').read_text())['designs']}
    result={g['id']:[designs[i]for i in g['asset_ids']if assets[i]['asset_type']=='facade'and course in assets[i]['original_course_family_load_matches']]for g in catalog['districts']}
    return result,assets


def validate_config(c):
    length=c['length_m'];pitch=c['block_pitch_m'];cross=c['cross_street_width_m'];sw=c['sidewalk_width_m']
    keys=['length_m','block_pitch_m','cross_street_width_m','road_width_m','sidewalk_width_m','parallel_road_offset_m','parallel_road_width_m','building_depth_m','tree_spacing_m','terrain_margin_m']
    if any(not math.isfinite(c[k])or c[k]<=0 for k in keys):raise ValueError('Finite positive city dimensions required')
    if c['terrain_margin_m']<200:raise ValueError('Terrain runoff must support off-road ejections')
    if length%pitch or cross+2*sw+16>=pitch:raise ValueError('Whole blocks with developable sites required')
    front=c['road_width_m']/2+sw;rear=c['parallel_road_offset_m']-c['parallel_road_width_m']/2-sw
    if rear-front<2*c['building_depth_m']+4:raise ValueError('Rear court clearance violated')
    available,_=pools(c['original_course']);cursor=0
    for zone in c['district_zones']:
        if zone['start_m']!=cursor or zone['end_m']<=cursor or zone['end_m']%pitch or not available.get(zone['district']):raise ValueError('Contiguous whole-block districts with eligible models required')
        cursor=zone['end_m']
    if cursor!=length:raise ValueError('Districts must cover complete route')


class Assembly:
    def __init__(self):self.parts=defaultdict(Model)
    def box(self,material,name,center,size):self.parts[material].box(name,center,size,(255,255,255))
    def wall(self,material,name,origin,u,normal,length,height,holes):
        # Segment solids around real apertures, rather than paint windows onto a cube.
        bounds=sorted({0.0,height,*[max(0,min(height,z))for x,z,w,h in holes for z in [z-h/2,z+h/2]]})
        def solid(a,b,lo,hi):
            if b-a<.001 or hi-lo<.001:return
            self.box(material,name,(origin[0]+u[0]*(a+b)/2,(lo+hi)/2+.16,origin[1]+u[1]*(a+b)/2),
                ((b-a)if u[0]else .22,hi-lo,(b-a)if u[1]else .22))
        for lo,hi in zip(bounds,bounds[1:]):
            spans=sorted((max(0,x-w/2),min(length,x+w/2))for x,z,w,h in holes if z-h/2<(lo+hi)/2<z+h/2)
            start=0
            for a,b in spans:solid(start,a,lo,hi);start=max(start,b)
            solid(start,length,lo,hi)
        for x,z,w,h in holes:
            cx=origin[0]+u[0]*x;cz=origin[1]+u[1]*x
            def detail(mat,label,along,y,width,vertical,depth,offset):
                self.box(mat,label,(cx+u[0]*along+normal[0]*offset,y+.16,cz+u[1]*along+normal[1]*offset),
                    (width if u[0]else depth,vertical,width if u[1]else depth))
            detail('black_metal','recessed_rear_glass',0,z,w,h,.045,-.04)
            for side in [-1,1]:detail('ivory','rear_window_jamb',side*(w/2+.045),z,.09,h+.18,.15,.12)
            for side in [-1,1]:detail('ivory','rear_sill_lintel',0,z+side*(h/2+.055),w+.24,.11,.22,.14)
            detail('ivory','rear_window_mullion',0,z,.045,h,.11,.13)
            detail('ivory','rear_window_transom',0,z,w,.045,.11,.13)


def assembly(lot,design):
    a=Assembly();front=lot['front_x'];side=lot['side'];inward=side if lot['row']=='avenue'else-side
    depth=lot['depth'];width=lot['z_end']-lot['z_start'];h=lot['height_m'];centre=front+inward*depth/2;mid=(lot['z_start']+lot['z_end'])/2
    material=design.get('masonry','ochre_brick');material=material+'_masonry_tile'if (KIT/'materials'/(material+'_masonry_tile_albedo.png')).exists()else material
    # The slab, two side walls and rear walls share one whole-lot footprint and roof.
    a.box('concrete','shared_floor_slab',(centre,.21,mid),(depth,.1,width))
    a.box('slate','shared_closed_roof',(centre,h+.16,mid),(depth,.2,width))
    module_width=width/lot['bays'];return_length=math.floor((depth-.8)/module_width)*module_width
    for z,exposed in [(lot['z_start']+.11,lot['exposed_start']),(lot['z_end']-.11,lot['exposed_end'])]:
        spans=[(0,.4),(.4+return_length,depth)]if exposed else[(0,depth)]
        # The real returned facade supplies its own apertures; never hide its glass behind a solid wall.
        for start,end in spans:
            if end-start>.01:a.box(material,'shared_end_wall',(front+inward*(start+end)/2,h/2+.16,z),(end-start,h,.22))
        a.box('ivory','end_coping',(centre,h+.38,z),(depth,.18,.22))
    rear=front+inward*(depth-.11);holes=[];floors=max(2,design.get('storeys',3))
    for level in range(1,floors):
        y=(4.8+(level-1)*2.6)*lot['facade_scale']
        if y+.8>=h-.3:continue
        for i in range(lot['bays']):holes.append(((i+.5)*width/lot['bays'],y,min(1.4,width/lot['bays']*.65),1.55*lot['facade_scale']))
    holes.append((width*.5,1.12,1.0,2.24))
    a.wall(material,'segmented_rear_wall',(rear,lot['z_start']),(0,1),(inward,0),width,h,holes)
    a.box('ivory','rear_coping',(rear,h+.38,mid),(.24,.18,width))
    # One service door, downpipes and shared rooftop equipment, not one plant per facade bay.
    a.box('green_metal','rear_service_door',(rear+inward*.1,1.28,mid),(.1,2.24,1))
    a.box('black_metal','rear_door_handle',(rear+inward*.19,1.3,mid+.32),(.07,.04,.14))
    for z in [lot['z_start']+.22,lot['z_end']-.22]:a.box('black_metal','rear_downpipe',(rear+inward*.18,h/2+.16,z),(.07,h,.07))
    a.box('concrete','shared_hvac',(centre,h+.7,mid),(1.1,1.0,1.5))
    for i in range(5):a.box('black_metal','hvac_louvre',(centre+.57,h+.36+i*.12,mid),(.05,.045,1.2))
    a.box(material,'shared_chimney',(centre-inward*depth*.25,h+.9,mid+width*.23),(.55,1.5,.65))
    return a


def generate(c):
    validate_config(c);rng=random.Random(c['seed']);available,assets=pools(c['original_course']);length=c['length_m'];pitch=c['block_pitch_m'];cross=c['cross_street_width_m'];rw=c['road_width_m'];sw=c['sidewalk_width_m'];po=c['parallel_road_offset_m'];pw=c['parallel_road_width_m']
    chunks=[];lots=[];ground=Model();ground.box('supported_city_ground',(0,-.18,length/2),(2*(po+c['terrain_margin_m']),.3,length+1000),(75,86,69))
    for x,width in [(0,rw),(-po,pw),(po,pw)]:ground.box('continuous_road',(x,-.015,(length-140)/2),(width,.08,length+380),(43,47,51))
    for z in range(10,int(length)-10,9):
        if min(z%pitch,pitch-z%pitch)<cross/2+4:continue
        for x in [-.13,.13]:ground.box('yellow_centre',(x,.09,z),(.1,.025,5.5),(195,148,46))
        for x in [-rw/4,rw/4]:ground.box('lane_dash',(x,.09,z),(.1,.025,3),(173,171,153))
    for block in range(int(length/pitch)+1):
        z=block*pitch;ground.box('crossroad',(0,.035,z),(2*(po+20),.06,cross),(43,47,51))
        for side in [-1,1]:
            for x in [rw*n/8 for n in [-3,-1,1,3]]:ground.box('crosswalk',(x,.075,z+side*(cross/2+1.5)),(rw/4-.6,.025,.55),(196,192,172))
    ground.box('start_line',(0,.09,3),(rw,.025,.35),(210,210,198));ground.box('finish_line',(0,.09,length-3),(rw,.025,.35),(210,210,198))
    chunks.append(dict(name='roads',boxes=ground.boxes));cursors=defaultdict(int)
    for block in range(int(length/pitch)):
        zone=next(z for z in c['district_zones']if z['start_m']<=block*pitch<z['end_m']);district=zone['district'];choices=available[district]
        start=block*pitch+cross/2+sw;end=(block+1)*pitch-cross/2-sw;wrapper=defaultdict(Model);furniture=Model()
        for side in [-1,1]:
            for road_x,width in [(0,rw),(side*po,pw)]:
                for edge in ([side]if road_x==0 else[-1,1]):
                    x=road_x+edge*(width/2+sw/2);furniture.box('sidewalk',(x,.08,(block+.5)*pitch),(sw,.16,pitch-cross),(143,141,129));furniture.box('curb',(road_x+edge*(width/2+.08),.12,(block+.5)*pitch),(.16,.24,pitch-cross),(179,176,159))
            for z in [block*pitch+cross/2+sw/2,(block+1)*pitch-cross/2-sw/2]:furniture.box('cross_street_sidewalk',(side*(rw/2+po-pw/2)/2,.08,z),(po-pw/2-rw/2,.16,sw),(143,141,129))
            for z in range(math.ceil(start+10),math.floor(end-10),int(c['tree_spacing_m'])):
                x=side*(rw/2+.8);furniture.box('tree_pit',(x,.18,z),(1.25,.04,1.25),(50,47,37));furniture.box('trunk',(x,2.1,z),(.23,4,.23),(77,58,43));furniture.box('canopy',(x,4.6,z),(2.2,1.7,2.2),(51,75,46));furniture.box('lamp',(side*(rw/2+.35),3.4,z+12),(.12,6.8,.12),(43,48,47))
            for row in ['avenue','outer']:
                counts=[];remaining=end-start
                while remaining>20:
                    n=rng.choice([4,5,6,7]);counts.append(n);remaining-=n*2.7
                if not counts:counts=[4]
                if remaining>0:counts[-1]+=max(1,round(remaining/2.7))
                bay=(end-start)/sum(counts);cursor=start
                for index,n in enumerate(counts):
                    d=choices[cursors[district]%len(choices)];cursors[district]+=1;scale=bay/d['width'];width=n*bay;depth=c['building_depth_m'];front=side*(rw/2+sw)if row=='avenue'else side*(po-pw/2-sw);inward=side if row=='avenue'else-side
                    lot=dict(id=f'{block}:{side}:{row}:{index}',family=d['id'],district=district,block=block,row=row,side=side,z_start=cursor,z_end=cursor+width,front_x=front,center_x=front+inward*depth/2,depth=depth,bays=n,height_m=d['height']*scale,facade_scale=scale,exposed_start=index==0,exposed_end=index==len(counts)-1,source_sheet='GLB:'+d['id'])
                    a=assembly(lot,d)
                    for material,model in a.parts.items():wrapper[material].boxes.extend(model.boxes)
                    lots.append(lot);cursor+=width
        chunks.append(dict(name=f'furniture_{block}',boxes=furniture.boxes))
        chunks.extend(dict(name=f'assembly_{block}_{mat}',material=mat,boxes=m.boxes)for mat,m in sorted(wrapper.items()))
    used=sorted({l['family']for l in lots});eligible=sorted({d['id']for z in c['district_zones']for d in available[z['district']]})
    if set(used)!=set(eligible):raise ValueError('Configured districts did not exercise every eligible facade')
    return dict(schema_version=3,seed=c['seed'],length_m=length,config=c,lots=lots,chunks=chunks,facades=[],texture_sources=[],geometry_mode='assembled_glb_buildings',model_design_ids=used,district_zones=c['district_zones'],original_course=c['original_course'],
        assembly_rules=dict(front='actual geometric facade runs',roof='one shared closed roof per lot',rear='segmented masonry around recessed openings',exposed_ends='shared walls and matching geometric facade returns',side_streets='reserved sidewalks and crossroad returns',source='independent interpretation of original facade/roof/end/side-street roles'))
