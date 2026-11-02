#!/usr/bin/env python3
"""Package verified draft meshes with editable sources and authoring evidence."""
import hashlib
import json
import zipfile
from pathlib import Path
from collect_building_catalog import ROOT,OUT,collect


def package():
    catalog=collect()
    dest=Path.home()/'Downloads/RoadDash_Complete_Building_Catalog.zip'
    paths=set()
    for r in catalog['records']:
        paths.add(OUT/r['source_blend']);paths.add(OUT/r['render'])
        paths.update(OUT/'models'/l['file']for l in r['lods'])
    for pattern in ['*.json','*.md','materials/*.png','catalog_manifests/*.json','catalog_renders/facades_*.png','catalog_renders/structures_*.png']:
        paths.update(OUT.glob(pattern))
    paths.add(OUT/'Catalog_Materials.blend')
    paths.update(ROOT/'tools'/name for name in ['build_building_meshes.py','plan_building_catalog.py','build_catalog_batch.py','catalog_mesh_geometry.py','catalog_structure_geometry.py','run_building_catalog.py','collect_building_catalog.py','verify_building_catalog.py','verify_building_mesh_assets.py','package_building_catalog.py'])
    paths.update([ROOT/'LICENSE',ROOT/'docs/testing/building-catalog/README.md',ROOT/'docs/testing/building-catalog/glb-validation.json'])
    paths.update((ROOT/'art/buildings/concept_3d').glob('*.json'))
    paths.update((ROOT/'art/buildings/concept_3d').glob('*.png'))
    paths.add(OUT/'renders/six-modeled-buildings.png')
    paths.add(OUT/'all-146-models.png')
    with zipfile.ZipFile(dest,'w',zipfile.ZIP_DEFLATED,compresslevel=3)as z:
        for p in sorted(paths):z.write(p,p.relative_to(ROOT))
    with zipfile.ZipFile(dest)as z:
        bad=z.testzip()
        assert bad is None,bad
        assert len([p for p in z.namelist()if p.endswith('.glb')])==438
        files=len(z.namelist())
    result=dict(file=str(dest),bytes=dest.stat().st_size,sha256=hashlib.file_digest(dest.open('rb'),'sha256').hexdigest(),entries=files,glbs=438,assets=146,zip_integrity_passed=True)
    (ROOT/'docs/testing/building-catalog/package.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))


if __name__=='__main__':package()
