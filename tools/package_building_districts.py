#!/usr/bin/env python3
"""Small overlay package: district indexes/previews and relative links, no mesh duplication."""
import hashlib
import json
import os
import stat
import zipfile
from pathlib import Path
from group_building_districts import ROOT,OUT,build


def package():
    result=build()
    if json.loads((OUT/'districts.json').read_text())!=result:raise ValueError('Regenerate stale districts first')
    destination=Path.home()/'Downloads/RoadDash_District_Sets.zip'
    paths=[p for p in OUT.rglob('*')if p.is_file()or p.is_symlink()]
    paths.extend([ROOT/'docs/research/building-districts/source-associations.json',ROOT/'docs/testing/building-districts/README.md'])
    paths.extend(ROOT/'tools'/name for name in ['group_building_districts.py','measure_reference_districts.py','package_building_districts.py','test_building_districts.py'])
    with zipfile.ZipFile(destination,'w',zipfile.ZIP_DEFLATED)as z:
        for path in sorted(paths):
            name=path.relative_to(ROOT).as_posix()
            if path.is_symlink():
                info=zipfile.ZipInfo(name);info.create_system=3;info.external_attr=(stat.S_IFLNK|0o777)<<16;z.writestr(info,os.readlink(path))
            else:z.write(path,name)
    with zipfile.ZipFile(destination)as z:
        assert z.testzip()is None
        links=sum(stat.S_ISLNK(i.external_attr>>16)for i in z.infolist());assert links==438
        assert all(stat.S_ISLNK(i.external_attr>>16)for i in z.infolist()if i.filename.endswith('.glb'))
        count=len(z.infolist())
    report=dict(file=str(destination),bytes=destination.stat().st_size,sha256=hashlib.file_digest(destination.open('rb'),'sha256').hexdigest(),entries=count,relative_mesh_links=links,copied_mesh_binaries=0,zip_integrity_passed=True)
    (ROOT/'docs/testing/building-districts/package.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report))


if __name__=='__main__':package()
