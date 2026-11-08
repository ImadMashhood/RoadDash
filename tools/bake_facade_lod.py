#!/usr/bin/env python3
"""Bake original authored storeys to orthographic PBR LOD tiles. Run with Blender's
bundled Python/numpy: blender -b --python tools/bake_facade_lod.py.
Uses actual source triangles, UVs, material factors and depth; no retail media.
"""
import hashlib,json,sys
from pathlib import Path
import bpy
import numpy as np
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
from verify_building_mesh_assets import read_glb,accessor
KIT=ROOT/'art/buildings/mesh_assets/storey_kit';OUT=KIT.parent/'far_facades'
TILE=128; PAD=16; INNER=TILE-2*PAD; WIDTH=2048; HEIGHT=4096; LEVELS=5
catalog=json.loads((KIT/'MANIFEST.json').read_text()); cache={}
def texture(doc,index):
    uri=doc['images'][doc['textures'][index]['source']]['uri']
    path=(KIT/uri).resolve()
    if path not in cache:
        im=bpy.data.images.load(str(path),check_existing=True)
        im.colorspace_settings.name='Non-Color'
        p=np.empty(len(im.pixels),dtype=np.float32);im.pixels.foreach_get(p)
        # Blender pixel origin is bottom left; GLTF UV(0,0) is image top left.
        cache[path]=p.reshape(im.size[1],im.size[0],4)[::-1].copy()
        bpy.data.images.remove(im)
    return cache[path]
def sample(im,uv):
    u=(uv[...,0]%1.0)*im.shape[1]-.5;v=(uv[...,1]%1.0)*im.shape[0]-.5
    x=np.floor(u).astype(int);y=np.floor(v).astype(int);fx=(u-x)[...,None];fy=(v-y)[...,None]
    return ((im[y%im.shape[0],x%im.shape[1]]*(1-fx)+im[y%im.shape[0],(x+1)%im.shape[1]]*fx)*(1-fy)
        +(im[(y+1)%im.shape[0],x%im.shape[1]]*(1-fx)+im[(y+1)%im.shape[0],(x+1)%im.shape[1]]*fx)*fy)
def linear(c):return np.where(c<=.04045,c/12.92,((c+.055)/1.055)**2.4)
def srgb(c):return np.where(c<=.0031308,c*12.92,1.055*np.maximum(c,0)**(1/2.4)-.055)
def raster(doc,binary,mesh,bounds):
    albedo=np.zeros((INNER,INNER,4),np.float32);normal=albedo.copy();packed=albedo.copy()
    depth=np.full((INNER,INNER),-np.inf);lo=np.array(bounds['min']);hi=np.array(bounds['max']);span=hi-lo
    if span[0]<1e-7 or span[1]<1e-7:return [albedo,normal,packed]
    for primitive in doc['meshes'][mesh]['primitives']:
        attrs=primitive['attributes'];pos=np.array(list(accessor(doc,binary,attrs['POSITION'])))
        uv=np.array(list(accessor(doc,binary,attrs['TEXCOORD_0'])));norm=np.array(list(accessor(doc,binary,attrs['NORMAL'])))
        indices=np.array(list(accessor(doc,binary,primitive['indices']))).reshape(-1,3)
        screen=(pos[:,:2]-lo[:2])/span[:2]*INNER;screen[:,1]=INNER-screen[:,1]
        mat=doc['materials'][primitive['material']];pbr=mat.get('pbrMetallicRoughness',{});factor=np.array(pbr.get('baseColorFactor',[1,1,1,1]))
        tex=texture(doc,pbr['baseColorTexture']['index']) if 'baseColorTexture' in pbr else None
        mr=texture(doc,pbr['metallicRoughnessTexture']['index']) if 'metallicRoughnessTexture' in pbr else None
        for ix in indices:
            pts=screen[ix];mn=np.maximum(np.floor(pts.min(0)).astype(int),0);mx=np.minimum(np.ceil(pts.max(0)).astype(int),INNER-1)
            if np.any(mn>mx):continue
            a,b,c=pts;den=(b[1]-c[1])*(a[0]-c[0])+(c[0]-b[0])*(a[1]-c[1])
            if abs(den)<1e-9:continue
            yy,xx=np.mgrid[mn[1]:mx[1]+1,mn[0]:mx[0]+1];x=xx+.5;y=yy+.5
            w0=((b[1]-c[1])*(x-c[0])+(c[0]-b[0])*(y-c[1]))/den
            w1=((c[1]-a[1])*(x-c[0])+(a[0]-c[0])*(y-c[1]))/den;w2=1-w0-w1
            weights=np.stack([w0,w1,w2],-1);z=weights@pos[ix,2]
            hit=(weights.min(-1)>=-1e-6)&(z>depth[yy,xx]+1e-7)
            if not hit.any():continue
            color=sample(tex,weights@uv[ix]) if tex is not None else np.ones((*z.shape,4))
            color[...,:3]=srgb(linear(color[...,:3])*factor[:3]);color[...,3]*=factor[3]
            if mat.get('alphaMode')=='MASK':hit&=color[...,3]>=mat.get('alphaCutoff',.5)
            n=weights@norm[ix];n/=np.maximum(np.linalg.norm(n,axis=-1,keepdims=True),1e-8);n=np.where(n[...,2:3]<0,-n,n)
            # Quad V runs down, so its bitangent is -Y.
            n*=np.array([1,-1,1]);n=n*.5+.5
            packing=np.ones_like(color);packing[...,1]=pbr.get('roughnessFactor',1);packing[...,2]=pbr.get('metallicFactor',1)
            if mr is not None:packing*=sample(mr,weights@uv[ix])
            albedo[yy[hit],xx[hit]]=color[hit];normal[yy[hit],xx[hit],:3]=n[hit];normal[yy[hit],xx[hit],3]=1
            packed[yy[hit],xx[hit]]=packing[hit];depth[yy[hit],xx[hit]]=z[hit]
    return [albedo,normal,packed]
def save(array,path):
    image=bpy.data.images.new(path.stem,width=array.shape[1],height=array.shape[0],alpha=True)
    image.colorspace_settings.name='Non-Color';image.pixels.foreach_set(np.clip(array[::-1],0,1).astype(np.float32).reshape(-1))
    image.filepath_raw=str(path);image.file_format='PNG';image.save();bpy.data.images.remove(image)
def mip(image,kind):
    h,w=image.shape[:2];block=image.reshape(h//2,2,w//2,2,4);alpha=block[...,3:4]
    out=block.mean((1,3))
    if kind=='albedo':
        rgb=(linear(block[...,:3])*alpha).sum((1,3))/np.maximum(alpha.sum((1,3)),1e-8);out[...,:3]=srgb(rgb)
    elif kind=='normal':
        n=((block[...,:3]*2-1)*alpha).sum((1,3));n/=np.maximum(np.linalg.norm(n,axis=-1,keepdims=True),1e-8);out[...,:3]=n*.5+.5
    return out
atlases=[np.zeros((HEIGHT,WIDTH,4),np.float32) for _ in range(3)];records=[];slot=0
for design in catalog['records']:
    path=KIT/f"{design['id']}_lod2.glb";data=path.read_bytes();digest=hashlib.sha256(data).hexdigest();assert digest==design['lods'][2]['sha256']
    doc,binary=read_glb(path)
    for band in range(3):
        nodes=doc['scenes'][band]['nodes']
        if not nodes:continue
        bounds=design['lods'][2]['bounds'][band]; images=raster(doc,binary,doc['nodes'][nodes[0]]['mesh'],bounds)
        x=(slot%(WIDTH//TILE))*TILE;y=(slot//(WIDTH//TILE))*TILE;assert y+TILE<=HEIGHT
        for atlas,im in zip(atlases,images):atlas[y:y+TILE,x:x+TILE]=np.pad(im,((PAD,PAD),(PAD,PAD),(0,0)),mode='edge')
        records.append(dict(design=design['id'],band=band,source_sha256=digest,bounds=bounds,uv_min=[(x+PAD+.5)/WIDTH,(y+PAD+.5)/HEIGHT],uv_max=[(x+PAD+INNER-.5)/WIDTH,(y+PAD+INNER-.5)/HEIGHT],covered_pixels=int((images[0][...,3]>.25).sum())))
        slot+=1
    print('BAKED',design['id'],flush=True)
files=[]
for kind,atlas in zip(['albedo','normal','packed'],atlases):
    for level in range(LEVELS):
        path=OUT/f'{kind}-mip{level}.png';save(atlas,path);files.append(dict(kind=kind,level=level,path=path.name,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),width=atlas.shape[1],height=atlas.shape[0]))
        if level+1<LEVELS:atlas=mip(atlas,kind)
manifest=dict(schema_version=1,width=WIDTH,height=HEIGHT,tile_size=TILE,gutter=PAD,mip_levels=LEVELS,source_manifest_sha256=hashlib.sha256((KIT/'MANIFEST.json').read_bytes()).hexdigest(),generator_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),records=records,images=files)
(OUT/'MANIFEST.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('FINISHED',len(records),'authored bands',len(files),'mip images',flush=True)
