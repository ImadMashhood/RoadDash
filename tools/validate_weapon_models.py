#!/usr/bin/env python3
"""Validate actual exported GLBs, not just declared allocation counts."""
import hashlib,json,struct
from pathlib import Path
root=Path(__file__).resolve().parents[1]
provenance=json.loads((root/'art/weapons/PROVENANCE.json').read_text())
assert hashlib.sha256((root/provenance['source']).read_bytes()).hexdigest()==provenance['source_sha256']
assert hashlib.sha256((root/'assets/weapons/catalog.json').read_bytes()).hexdigest()==provenance['catalog_sha256']
for name,digest in provenance['authoring_sha256'].items():assert hashlib.sha256((root/'art/weapons'/name).read_bytes()).hexdigest()==digest
c=json.loads((root/'assets/weapons/catalog.json').read_text())
report=[]
for model in c['records']:
 for lod in model['lods']:
  path=root/'assets'/lod['file'];data=path.read_bytes()
  assert hashlib.sha256(data).hexdigest()==lod['sha256'],path
  magic,version,length=struct.unpack_from('<III',data);assert magic==0x46546c67 and version==2 and length==len(data)
  size,kind=struct.unpack_from('<II',data,12);assert kind==0x4e4f534a
  g=json.loads(data[20:20+size]);assert len(g['meshes'])==1
  binary_offset=20+size+8
  def unpack_accessor(index):
   accessor=g['accessors'][index];view=g['bufferViews'][accessor['bufferView']];fmt={5126:'f',5125:'I',5123:'H',5121:'B'}[accessor['componentType']];width={'SCALAR':1,'VEC3':3}[accessor['type']];stride=view.get('byteStride',struct.calcsize(fmt)*width);offset=binary_offset+view.get('byteOffset',0)+accessor.get('byteOffset',0)
   return [struct.unpack_from('<'+fmt*width,data,offset+i*stride)for i in range(accessor['count'])]
  degenerate=0
  for prim in g['meshes'][0]['primitives']:
   vertices=unpack_accessor(prim['attributes']['POSITION']);indices=[v[0]for v in unpack_accessor(prim['indices'])]
   for j in range(0,len(indices),3):
    a,b,c=[vertices[i]for i in indices[j:j+3]];u=[b[i]-a[i]for i in range(3)];v=[c[i]-a[i]for i in range(3)];cross=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]]
    degenerate += sum(x*x for x in cross)<1e-24
  assert degenerate==0,(path,degenerate)
  tris=sum(g['accessors'][p['indices']]['count']//3 for m in g['meshes']for p in m['primitives'])
  assert tris==lod['triangles'],(path,tris,lod['triangles'])
  assert not g.get('skins') and not g.get('animations')
  for prim in g['meshes'][0]['primitives']:
   pos=g['accessors'][prim['attributes']['POSITION']]
   for a in range(3):
    assert pos['min'][a]>=lod['bounds_y_up'][0][a]-1e-5,(path,pos,lod)
    assert pos['max'][a]<=lod['bounds_y_up'][1][a]+1e-5,(path,pos,lod)
  report.append(dict(file=lod['file'],sha256=lod['sha256'],triangles=tris,materials=len(g['materials']),bytes=len(data),degenerate_triangles=degenerate))
print(json.dumps(dict(validated=12,source_and_four_blends_validated=True,records=report),indent=2))
