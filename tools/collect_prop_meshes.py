#!/usr/bin/env python3
"""Collect complete group exports into an ID-mapped mesh catalog."""
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];OUT=ROOT/'art/props/mesh_assets'

def collect():
 designs=json.loads((OUT/'designs.json').read_text())['designs'];records=[];sources=[];renders=[];clay_renders=[]
 for group in sorted({d['group']for d in designs}):
  part=json.loads((OUT/'manifests'/(group+'.json')).read_text());records+=part['records'];sources.append(dict(file=part['source'],sha256=part['source_sha256']))
  render=OUT/'manifests'/(group+'_renders.json')
  if render.exists():renders+=json.loads(render.read_text())
  clay=OUT/'manifests'/(group+'_clay_renders.json')
  if clay.exists():clay_renders+=json.loads(clay.read_text())
 assert {r['id']for r in records}=={d['id']for d in designs}
 assert len(records)==len(designs)
 result=dict(schema_version=1,stage='draft_volumetric_mesh_catalog_not_production_approved',
  coordinates='GLB Y up, front +Z; Blender Z up, front -Y; metres',
  authoring='Original parametric volumes, separate semantic parts, edge bevel geometry, original embedded material tiles; no concept-image cards or sprite projection',
  authoring_script_sha256=hashlib.sha256((ROOT/'tools/build_prop_meshes.py').read_bytes()).hexdigest(),
  designs_sha256=hashlib.sha256((OUT/'designs.json').read_bytes()).hexdigest(),
  records=records,sources=sources,renders=renders,clay_renders=clay_renders,
  limitations=['Draft silhouette/material likeness needs art review','Human/animal meshes are not skinned or animated',
   'Conservative bounding proxies are authoring metadata, not gameplay colliders','No runtime racing replacements or accepted physics changes'])
 (OUT/'MANIFEST.json').write_text(json.dumps(result,indent=2)+'\n')
 print(json.dumps(dict(models=len(records),lod_files=sum(len(r['lods'])for r in records),source_scenes=len(sources),render_sheets=len(renders))))
if __name__=='__main__':collect()
