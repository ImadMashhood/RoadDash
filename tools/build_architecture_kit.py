#!/usr/bin/env python3
"""Slice our authored 3D fronts into reusable storeys/crowns with shared texture files.
Original GLBs and retail reference media are never modified or copied into this kit.
"""
import argparse, copy, hashlib, json, math, struct
from pathlib import Path
from verify_building_mesh_assets import read_glb, accessor
ROOT=Path(__file__).resolve().parents[1]; SOURCE=ROOT/'art/buildings/mesh_assets'; OUT=SOURCE/'storey_kit'

def clip(poly, axis, plane, above):
    result=[]
    for a,b in zip(poly,poly[1:]+poly[:1]):
        da=(a[axis]-plane)*(1 if above else -1); db=(b[axis]-plane)*(1 if above else -1)
        if da>=-1e-7:result.append(a)
        if (da>=0)!=(db>=0):
            t=da/(da-db); result.append(tuple(x+(y-x)*t for x,y in zip(a,b)))
    return result

def derive(design,lod):
    identity=design['id']; facade=design['asset_type']=='facade'
    path=SOURCE/('front_modules' if facade else 'models')/f'{identity}_lod{lod}.glb'
    d,b=read_glb(path); floor=design['height']/max(1,design.get('storeys',round(design['height']/3)))
    # Cut at the authored floor boundaries, never through upper windows/arches.
    # The accepted first sheet uses taller storefronts than the remaining catalog.
    if facade:
        ground, upper_floor = (4.6, 3.6) if design['builder'] == 'accepted_first_sheet' else (3.9, 2.6)
        crown = ground + upper_floor if design['builder'] == 'accepted_first_sheet' else max(ground, design['height'] - .6)
        bands = [(0.0, ground), (ground, ground + upper_floor), (crown, 1000.0)]
        heights = [ground, upper_floor]
    else:
        # Structure fronts are single compositions (barn gables, church towers),
        # not repeatable floors. Preserve the full body below the roof cap.
        crown=max(floor,design['height']-0.6)
        bands=[(0.0,crown),(0.0,floor),(crown,1000.0)]
        heights=[crown,floor]
    front=0.0 if facade else design['depth']/2
    positions=[]
    for p in d['meshes'][0]['primitives']:
        positions.extend(accessor(d,b,p['attributes']['POSITION']))
    if not facade:front=min(front,max(p[2]for p in positions))
    upper=max(p[1]for p in positions); width=max(abs(p[0])for p in positions)*2
    binary=bytearray(); views=[]; accessors=[]; meshes=[]; bounds=[]; counts=[]
    def view(raw,target=None):
        binary.extend(b'\0'*(-len(binary)%4)); start=len(binary);binary.extend(raw)
        v=dict(buffer=0,byteOffset=start,byteLength=len(raw))
        if target:v['target']=target
        views.append(v);return len(views)-1
    def array(values,width,kind,component):
        pack=struct.Struct('<'+('f'if component==5126 else 'I')*width)
        a=dict(bufferView=view(b''.join(pack.pack(*v)for v in values),34962 if component==5126 else 34963),componentType=component,type=kind,count=len(values))
        if kind=='VEC3':a.update(min=[min(v[k]for v in values)for k in range(3)],max=[max(v[k]for v in values)for k in range(3)])
        accessors.append(a);return len(accessors)-1
    for low,high in bands:
        primitives=[]; points=[]; triangles=0
        for p in d['meshes'][0]['primitives']:
            attrs=p['attributes'];pos=list(accessor(d,b,attrs['POSITION']));normal=list(accessor(d,b,attrs['NORMAL']));uv=list(accessor(d,b,attrs['TEXCOORD_0']));indices=[i[0]for i in accessor(d,b,p['indices'])]
            data=[];ix=[]
            for at in range(0,len(indices),3):
                poly=[tuple(pos[j])+tuple(normal[j])+tuple(uv[j])for j in indices[at:at+3]]
                if not facade:poly=clip(poly,2,front-.6,True)
                if not poly:continue
                poly=clip(clip(poly,1,low,True),1,high,False)
                if len(poly)<3:continue
                start=len(data)
                for v in poly:
                    n=math.sqrt(sum(x*x for x in v[3:6])) or 1
                    data.append((v[0],v[1]-low,v[2]-front,*(x/n for x in v[3:6]),*v[6:8]))
                for i in range(1,len(poly)-1):ix.extend([(start,),(start+i,),(start+i+1,)])
            if not ix:continue
            points.extend(v[:3]for v in data);triangles+=len(ix)//3
            primitives.append(dict(attributes={'POSITION':array([v[:3]for v in data],3,'VEC3',5126),'NORMAL':array([v[3:6]for v in data],3,'VEC3',5126),'TEXCOORD_0':array([v[6:8]for v in data],2,'VEC2',5126)},indices=array(ix,1,'SCALAR',5125),material=p['material']))
        meshes.append(dict(primitives=primitives));counts.append(triangles)
        bounds.append({'min':[min((p[k]for p in points),default=0)for k in range(3)],'max':[max((p[k]for p in points),default=0)for k in range(3)]})
    images=[]
    for image in d.get('images',[]):
        v=d['bufferViews'][image['bufferView']];raw=b[v.get('byteOffset',0):v.get('byteOffset',0)+v['byteLength']]
        digest=hashlib.sha256(raw).hexdigest();name=f'{digest}.png';target=OUT/'textures'/name
        if not target.exists():target.write_bytes(raw)
        images.append(dict(uri='textures/'+name))
    # Empty crowns are valid flat-roof designs; do not create empty mesh scenes.
    nodes=[dict(mesh=i,name=['ground','upper','crown'][i])for i in range(3)]
    result=dict(asset=dict(version='2.0',generator='RoadDash authored facade storey compiler v1'),scene=0,
        scenes=[dict(nodes=[i]if meshes[i]['primitives']else [])for i in range(3)],nodes=[n if meshes[i]['primitives']else dict(name=n['name'])for i,n in enumerate(nodes)],
        meshes=[m if m['primitives']else dict(primitives=[])for m in meshes],materials=copy.deepcopy(d['materials']),textures=copy.deepcopy(d.get('textures',[])),images=images,samplers=d.get('samplers',[]),accessors=accessors,bufferViews=views,buffers=[dict(byteLength=len(binary))])
    # glTF forbids empty primitive arrays; unused slots get an innocuous non-mesh node.
    mapping={old:new for new,old in enumerate(i for i,m in enumerate(meshes)if m['primitives'])}
    result['meshes']=[m for m in meshes if m['primitives']]
    for i,n in enumerate(result['nodes']):
        if 'mesh'in n:n['mesh']=mapping[i]
    if 'extensionsUsed'in d:result['extensionsUsed']=d['extensionsUsed']
    binary.extend(b'\0'*(-len(binary)%4));result['buffers'][0]['byteLength']=len(binary)
    header=json.dumps(result,separators=(',',':')).encode();header+=b' '*(-len(header)%4)
    target=OUT/f'{identity}_lod{lod}.glb';target.write_bytes(struct.pack('<4sII',b'glTF',2,28+len(header)+len(binary))+struct.pack('<II',len(header),0x4e4f534a)+header+struct.pack('<II',len(binary),0x004e4942)+binary)
    from compact_architecture_kit import compact
    compact(target)
    return dict(lod=lod,sha256=hashlib.sha256(target.read_bytes()).hexdigest(),source_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),triangles=counts,bounds=bounds,bytes=target.stat().st_size),[*heights,max(upper-crown,0)],width

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--ids',nargs='*');args=ap.parse_args()
    OUT.mkdir(exist_ok=True);(OUT/'textures').mkdir(exist_ok=True)
    designs=json.loads((SOURCE/'catalog_designs.json').read_text())['designs'];records=[]
    for design in designs:
        if design['asset_type'] not in ['facade','structure'] or args.ids and design['id']not in args.ids:continue
        lods=[]
        for lod in range(3):r,heights,width=derive(design,lod);lods.append(r)
        masonry=design.get('masonry',{'barn':'red_paint','house':'white_siding','cabin':'old_timber','hay_shelter':'old_timber'}.get(design['builder'],'concrete'))
        width=max(width,max(max(abs(v)for v in [l['bounds'][band]['min'][0],l['bounds'][band]['max'][0]])*2 for l in lods for band in range(3)))
        heights[2]=max(l['bounds'][2]['max'][1]for l in lods)
        records.append(dict(id=design['id'],asset_type=design['asset_type'],masonry=masonry,width=width,heights=heights,lods=lods))
        print('COMPILED',design['id'],flush=True)
    if args.ids:
        manifest=OUT/'MANIFEST.json'
        if not manifest.exists():
            raise ValueError('--ids requires an existing full manifest; run without --ids first')
        by_id={r['id']:r for r in json.loads(manifest.read_text())['records']}
        by_id.update({r['id']:r for r in records})
        records=[by_id[d['id']]for d in designs if d['asset_type']in ['facade','structure']]
    temporary=OUT/'MANIFEST.json.tmp'
    temporary.write_text(json.dumps(dict(schema_version=1,records=records),indent=2)+'\n')
    temporary.replace(OUT/'MANIFEST.json')
    print('MODELS',len(records),flush=True)
if __name__=='__main__':main()
