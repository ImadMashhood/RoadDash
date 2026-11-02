#!/usr/bin/env python3
"""Original modular facade geometry. No retail pixels or external model inputs.
Y is up; front is -Z. OBJ/MTL models are authoring previews, not runtime assets.
"""
import argparse, json, math
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
FACES = [(0,3,2,1),(4,5,6,7),(0,1,5,4),(3,7,6,2),(0,4,7,3),(1,2,6,5)]
class Model:
    def __init__(self): self.boxes=[]
    def box(self, name, center, size, color):
        assert all(math.isfinite(v) for v in (*center,*size)) and min(size)>0
        self.boxes.append((name,center,size,tuple(color)))
    def vertices(self, box):
        _,(x,y,z),(a,b,c),_=box
        return [(x+dx*a/2,y+dy*b/2,z+dz*c/2) for dx,dy,dz in
                [(-1,-1,-1),(1,-1,-1),(1,1,-1),(-1,1,-1),(-1,-1,1),(1,-1,1),(1,1,1),(-1,1,1)]]
    def export(self,path):
        palette=list(dict.fromkeys(b[3] for b in self.boxes))
        path.with_suffix('.mtl').write_text('\n'.join(f'newmtl material_{i}\nKd {r/255:.6f} {g/255:.6f} {b/255:.6f}\n' for i,(r,g,b) in enumerate(palette)))
        lines=[f'mtllib {path.stem}.mtl'];offset=1
        for i,box in enumerate(self.boxes):
            lines += [f'o {box[0]}_{i}',f'usemtl material_{palette.index(box[3])}']
            lines += ['v '+' '.join(f'{x:.6f}' for x in v) for v in self.vertices(box)]
            lines += ['f '+' '.join(str(offset+j) for j in face) for face in FACES]
            offset+=8
        path.write_text('\n'.join(lines)+'\n')

def generate(family,bays,floors,depth=9,corner=False):
    if not 2<=bays<=12 or not 1<=floors<=12 or depth<3: raise ValueError('Invalid building dimensions')
    m=Model();w=bays*family['bay_width'];h=floors*family['floor_height'];wall=family['color'];trim=tuple(min(255,c+35) for c in wall);glass=(37,56,65)
    m.box('foundation',(0,.12,0),(w,.24,depth),wall)
    m.box('rear',(0,h/2,depth/2-.12),(w,h,.24),wall)
    m.box('party_wall',(-w/2+.12,h/2,0),(.24,h,depth),wall)
    if not corner:m.box('side',(w/2-.12,h/2,0),(.24,h,depth),wall)
    def facade(side,length,count):
        def box(name,x,y,z,sx,sy,sz,color):
            if side: m.box(name,(w/2+z,y,-x),(sz,sy,sx),color)
            else:m.box(name,(x,y,-depth/2-z),(sx,sy,sz),color)
        bay=length/count;fh=family['floor_height'];ww=family['window_width'];wh=family['window_height']
        for floor in range(floors):
            for col in range(count):
                x=-length/2+(col+.5)*bay;y=(floor+.5)*fh
                door=floor==0 and col==count//2
                ow=min(ww*1.25,bay-.4) if door else ww;oh=2.7 if door else wh
                bottom=floor*fh if door else y-oh/2
                top=bottom+oh
                for sign in [-1,1]:box('wall_pier',x+sign*(ow/2+(bay-ow)/4),floor*fh+fh/2,0,(bay-ow)/2,fh,.24,wall)
                if bottom>floor*fh:box('wall_below',x,(floor*fh+bottom)/2,0,ow,bottom-floor*fh,.24,wall)
                if top<(floor+1)*fh:box('wall_above',x,(top+(floor+1)*fh)/2,0,ow,(floor+1)*fh-top,.24,wall)
                box('loading_door' if door and family['entrance']=='loading' else 'door' if door else 'recessed_glass',x,(bottom+top)/2,-.07,ow,oh,.04,(48,48,45) if door else glass)
                for sign in [-1,1]:box('jamb',x+sign*ow/2,(bottom+top)/2,.06,.09,oh,.12,trim)
                for yy in [bottom,top]:box('lintel_sill',x,max(.055,yy),.10,ow+.2,.11,.22,trim)
                for j in range(1,family['mullions']+1):box('mullion',x-ow/2+j*ow/(family['mullions']+1),(bottom+top)/2,.015,.045,oh,.045,trim)
                if not door:box('transom',x,(bottom+top)/2,.015,ow,.05,.045,trim)
            box('floor_band',0,(floor+1)*fh,.07,length,.15,.16,trim)
        box('cornice',0,h+.12,.12,length+.12,.24,family['cornice']+.2,trim)
    facade(False,w,bays)
    if corner:facade(True,depth,max(2,round(depth/family['bay_width'])))
    m.box('roof_slab',(0,h+.08,0),(w,.16,depth), (52,50,49))
    return m

def preview(models,path):
    from PIL import Image, ImageDraw
    image=Image.new('RGB',(1500,1000),(224,225,222));d=ImageDraw.Draw(image)
    for i,(label,model) in enumerate(models):
        col=i%3;row=i//3;origin=(col*500+250,row*250+207)
        verts=[v for b in model.boxes for v in model.vertices(b)];extent=max(max(abs(v[0])+abs(v[2])*.65,v[1]) for v in verts);scale=140/extent
        def project(v):
            x,y,z=v;return (origin[0]+(x*.85+z*.52)*scale,origin[1]+(-y+(z*.34-x*.18))*scale)
        faces=[]
        for box in model.boxes:
            v=model.vertices(box)
            for face in FACES:
                pts=[v[j] for j in face];key=sum(p[2]-p[0]*.4+p[1]*.1 for p in pts)/4
                faces.append((key,[project(p) for p in pts],box[3]))
        for _,points,color in sorted(faces,reverse=True):d.polygon(points,fill=color)
        d.text((col*500+20,row*250+12),label,fill=(32,34,37))
    image.save(path)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--output',type=Path,default=ROOT/'art/buildings/prototype/generated');args=ap.parse_args();args.output.mkdir(parents=True,exist_ok=True)
    config=json.loads((ROOT/'art/buildings/prototype/grammar.json').read_text());models=[];manifest=[]
    for f in config['families']:
        for bays,floors,corner in [(2,2,False),(4,2,False),(3,4,False),(3,3,True)]:
            name=f"{f['id']}_{bays}b_{floors}f"+('_corner' if corner else '')
            model=generate(f,bays,floors,corner=corner);model.export(args.output/(name+'.obj'));models.append((name,model))
            manifest.append({'id':name,'family':f['id'],'reference':f['reference'],'bays':bays,'floors':floors,'corner':corner,'width_m':bays*f['bay_width'],'boxes':len(model.boxes)})
    # A connected row uses exact facade widths, without infill/gaps.
    street=Model();cursor=0
    for f in config['families']:
        m=generate(f,3,3);width=3*f['bay_width']
        for name,(x,y,z),size,color in m.boxes:street.box(name,(x+cursor+width/2,y,z),size,color)
        cursor+=width
    street.export(args.output/'connected_frontage.obj')
    preview(models,args.output/'family_variants.png')
    (args.output/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(f'Generated {len(models)} variants and connected frontage')
if __name__=='__main__':main()
