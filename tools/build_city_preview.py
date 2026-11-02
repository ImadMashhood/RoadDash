#!/usr/bin/env python3
"""Deterministic, metre-scale city design study. Uses the unchanged new facade artwork on textured relief fronts.
The Bevy viewer batches these boxes into chunk meshes. No retail media is loaded.
"""
import argparse
import json
import math
import random
from pathlib import Path
from build_facade_prototype import Model
from facade_surfaces import Surfaces
import hashlib

ROOT = Path(__file__).resolve().parents[1]
DEFAULT = ROOT / 'art/buildings/city_preview'


def validate(c):
    for key in ['length_m', 'block_pitch_m', 'cross_street_width_m', 'road_width_m',
                'sidewalk_width_m', 'parallel_road_offset_m', 'parallel_road_width_m',
                'building_depth_m', 'tree_spacing_m']:
        if not math.isfinite(c[key]) or c[key] <= 0:
            raise ValueError(f'{key} must be finite and positive')
    if c['length_m'] % c['block_pitch_m'] or c['cross_street_width_m'] >= c['block_pitch_m'] - 12:
        raise ValueError('Whole blocks with room for lots required')
    if not .5 <= c['height_scale_range'][0] <= c['height_scale_range'][1] <= 1.5:
        raise ValueError('Invalid facade height scale')
    front = c['road_width_m']/2+c['sidewalk_width_m']
    rear = c['parallel_road_offset_m']-c['parallel_road_width_m']/2-c['sidewalk_width_m']
    if rear-front < 2*c['building_depth_m']+4:
        raise ValueError('Two building rows need a minimum four-metre rear court')
    if c['tree_spacing_m'] < 1 or c['tree_spacing_m'] != int(c['tree_spacing_m']):
        raise ValueError('Tree spacing must be a positive whole metre')


def load_assets(c):
    assets=json.loads((ROOT/c['facade_catalog']).read_text())['assets']
    if not assets or len({a['id'] for a in assets})!=len(assets):
        raise ValueError('Unique, nonempty facade asset catalog required')
    checked=set()
    for a in assets:
        path=ROOT/a['sheet']
        if a['sheet'] not in checked:
            digest=hashlib.sha256(path.read_bytes()).hexdigest()
            if digest != a['source_sha256']:
                raise ValueError(f"Facade image changed: {a['sheet']}")
            checked.add(a['sheet'])
        x0,y0,x1,y1=a['rect_px'];w,h=a['pixels']
        if not (0<=x0<x1<=w and 0<=y0<y1<=h):raise ValueError('Invalid facade UV bounds')
    return assets


def shell(width,height,depth,color):
    m=Model()
    m.box('rear',(0,height/2,depth/2-.12),(width,height,.24),color)
    for x in [-width/2+.12,width/2-.12]:
        m.box('party_wall',(x,height/2,0),(.24,height,depth),color)
    m.box('front_backing',(0,height/2,-depth/2+.5),(width,height,.15),color)
    m.box('roof_slab',(0,height+.06,0),(width,.12,depth),(57,57,54))
    for x in [-width/2+.1,width/2-.1]:
        m.box('roof_side',(x,height+.2,0),(.2,.4,depth),color)
    for z in [-depth/2+.1,depth/2-.1]:
        m.box('roof_edge',(0,height+.2,z),(width,.4,.2),color)
    m.box('roof_plant',(width*.2,height+.5,2),(1.3,.9,1.7),(89,88,79))
    return m


def build(c, modeled=False):
    if modeled:
        from city_buildings import generate
        return generate(c)
    validate(c)
    rng = random.Random(c['seed'])
    if modeled:
        designs=json.loads((ROOT/'art/buildings/mesh_assets/catalog_designs.json').read_text())['designs']
        designs=[d for d in designs if d['asset_type']=='facade']
        for design in designs:
            for lod in range(3):
                if not (ROOT/f"art/buildings/mesh_assets/models/{design['id']}_lod{lod}.glb").is_file():
                    raise ValueError('Export actual building GLBs with build_building_meshes.py first')
        assets=[dict(id=d['id'],height_m=d['height'],side_color=[128,115,95],sheet='GLB:'+d['id']) for d in designs]
    else:
        assets=load_assets(c)
    surfaces=Surfaces()
    chunks, lots = [], []
    # First pass guarantees every actual facade appears; subsequent choices are seeded.
    choices=assets.copy();rng.shuffle(choices);choice_index=0
    ground = Model()
    length, pitch, cross = c['length_m'], c['block_pitch_m'], c['cross_street_width_m']
    rw, sw, po, pw = [c[k] for k in ['road_width_m','sidewalk_width_m','parallel_road_offset_m','parallel_road_width_m']]
    asphalt, paving = (43,47,51), (143,141,129)
    ground.box('ground', (0,-.18,length/2), (2*(po+30),.3,length+70), (75,86,69))
    for x,width in [(0,rw),(-po,pw),(po,pw)]:
        ground.box('road', (x,-.015,length/2), (width,.08,length), asphalt)
    for i in range(int(length/pitch)+1):
        z = i*pitch
        ground.box('crossroad', (0,.035,z), (2*(po+20),.06,cross), asphalt)
        # Stop lines and crosswalks are clear of the carriageway's centre.
        for side in [-1,1]:
            for x in [rw*n/8 for n in [-3,-1,1,3]]:
                ground.box('crosswalk', (x,.075,z+side*(cross/2+2.5)), (rw/4-.6,.025,.55), (196,192,172))
    for z in range(10,int(length)-10,9):
        if min(z%pitch,pitch-z%pitch) < cross/2+4: continue
        for x in [-.13,.13]:
            ground.box('yellow_centre', (x,.09,z), (.1,.025,5.5), (195,148,46))
        for x in [-rw/4,rw/4]:
            ground.box('lane_dash', (x,.09,z), (.1,.025,3), (173,171,153))
    chunks.append(('roads',ground))
    for block in range(int(length/pitch)):
        start,end = block*pitch+cross/2, (block+1)*pitch-cross/2
        for side in [-1,1]:
            furniture = Model()
            for road_x, width in [(0,rw),(side*po,pw)]:
                for edge in ([side] if road_x == 0 else [-1,1]):
                    x=road_x+edge*(width/2+sw/2)
                    furniture.box('sidewalk',(x,.08,(start+end)/2),(sw,.16,end-start),paving)
                    furniture.box('curb',(road_x+edge*(width/2+.08),.12,(start+end)/2),(.16,.24,end-start),(179,176,159))
            for z in range(math.ceil(start+12),math.floor(end-8),int(c['tree_spacing_m'])):
                x=side*(rw/2+.8)
                furniture.box('tree_pit',(x,.18,z),(1.25,.04,1.25),(50,47,37))
                furniture.box('trunk',(x,2.1,z),(.23,4,.23),(77,58,43))
                for y,size,offset in [(4.4,2.4,0),(5.2,1.8,.4)]:
                    furniture.box('canopy',(x+offset,y,z),(size,size*.65,size),(51,75,46))
                lx=side*(rw/2+.35)
                furniture.box('lamp',(lx,3.4,z+14),(.12,6.8,.12),(43,48,47))
                furniture.box('lamp_head',(lx-side*.35,6.75,z+14),(.9,.15,.35),(199,186,137))
            chunks.append((f'furniture_{block}_{side}',furniture))
            for row in ['avenue','outer']:
                model=Model()
                # Exact partition fills each block. Fixed bay counts; slight shared bay-width
                # adjustment closes the row without arbitrary stretched final buildings.
                counts=[];remaining=end-start
                while remaining>11:
                    n=rng.choice([2,3,3,4]);counts.append(n);remaining-=n*2.6
                if remaining>0: counts[-1]+=max(1,round(remaining/2.6))
                bay=(end-start)/sum(counts)
                cursor=start
                for index,n in enumerate(counts):
                    f=choices[choice_index%len(choices)];choice_index+=1
                    height=f['height_m']*rng.uniform(*c['height_scale_range'])
                    width=n*bay;depth=c['building_depth_m']
                    face=side*(rw/2+sw) if row=='avenue' else side*(po-pw/2-sw)
                    inward = side if row=='avenue' else -side
                    centre_x=face+inward*depth/2
                    m=Model() if modeled else shell(width,height,depth,f['side_color'])
                    centre_z=cursor+width/2
                    # Local -Z points to the respective street. Orthogonal transform,
                    # preserving width/depth and all recessed front details.
                    for name,(x,y,z),size,color in m.boxes:
                        sx,sy,sz=size
                        model.box(name,(centre_x+inward*z,y+.16,centre_z-inward*x),(sz,sy,sx),color)
                    owner=f'{block}:{side}:{row}:{index}'
                    for panel in range(0 if modeled else n):
                        z=cursor+(panel+.5)*bay
                        surfaces.panel(f,[face,.16,z],[-inward,0,0],bay,height,owner)
                        surfaces.panel(f,[centre_x+inward*depth/2,.16,z],[inward,0,0],bay,height,owner,'rear')
                    # Actual artwork returns along exposed cross-street ends.
                    for end_z,sign,exposed in [(start,-1,index==0),(end,1,index==len(counts)-1)]:
                        if exposed and not modeled:
                            side_count=max(1,round(depth/3.2))
                            for part in range(side_count):
                                x=face+inward*(part+.5)*depth/side_count
                                surfaces.panel(f,[x,.16,end_z],[0,0,sign],depth/side_count,height,owner,'end')
                    lots.append(dict(id=f'{block}:{side}:{row}:{index}',family=f['id'],block=block,
                                     row=row,side=side,z_start=cursor,z_end=cursor+width,
                                     front_x=face,center_x=centre_x,depth=depth,bays=n,height_m=height,source_sheet=f['sheet']))
                    cursor+=width
                chunks.append((f'buildings_{block}_{side}_{row}',model))
    # A proof gallery away from the city: six exact new facade images at human scale.
    gallery=Model()
    gallery.box('proof_ground',(2010,-.1,14),(45,.2,40),(90,89,81))
    for i,f in enumerate([] if modeled else assets[:6]):
        w,h,depth=4.0,9.2,7.0;face=2000.0;z=i*4.5+2
        m=shell(w,h,depth,f['side_color'])
        for name,(x,y,lz),size,color in m.boxes:
            sx,sy,sz=size;gallery.box(name,(face+depth/2+lz,y,z-x),(sz,sy,sx),color)
        surfaces.panel(f,[face,0,z],[-1,0,0],w,h,'proof:'+f['id'])
    chunks.append(('proof_gallery',gallery))
    return {'schema_version':2,'seed':c['seed'],'length_m':length,'config':c,'lots':lots,
            'chunks':[{'name':name,'boxes':m.boxes} for name,m in chunks],
            'facades':surfaces.records(),'texture_sources':[] if modeled else assets,
            'geometry_mode':'actual_glb_models' if modeled else 'rejected_flat_study',
            'model_design_ids':[a['id'] for a in assets] if modeled else []}


def main():
    ap=argparse.ArgumentParser();ap.add_argument('--config',type=Path,default=DEFAULT/'config.json');ap.add_argument('--output',type=Path,default=DEFAULT/'generated');ap.add_argument('--legacy-flat-study',action='store_true');a=ap.parse_args()
    c=json.loads(a.config.read_text());scene=build(c,modeled=not a.legacy_flat_study);a.output.mkdir(parents=True,exist_ok=True)
    (a.output/'scene.json').write_text(json.dumps(scene,separators=(',',':'))+'\n')
    report={k:scene[k] for k in ['seed','length_m','lots']}
    report.update(buildings=len(scene['lots']),chunks=len(scene['chunks']),boxes=sum(len(x['boxes']) for x in scene['chunks']),facade_assets=len(scene['texture_sources']),texture_sheets=len(scene['facades']),textured_quads=sum(len(b['quads']) for b in scene['facades']),geometry_mode=scene['geometry_mode'],model_design_ids=scene['model_design_ids'],status='Full-depth buildings assembled from geometric facades, closed shared roofs, rear walls and corner returns' if not a.legacy_flat_study else 'Rejected flat facade study retained for comparison')
    (a.output/'layout.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:v for k,v in report.items() if k!='lots'},indent=2))
if __name__=='__main__':main()
