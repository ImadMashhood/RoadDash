import json
from pathlib import Path
out=Path(__file__).resolve().parent
before=json.loads((out/'before/screenshots/metro-performance.json').read_text())
after=json.loads((out/'after/screenshots/metro-performance.json').read_text())
assert before['census']==after['census']
errors={}
for folder in ['after','after-recovery','lod-valley','lod-city']:
 text=(out/folder/'native.txt').read_text()
 errors[folder]=[line for line in text.splitlines() if any(x in line for x in ['ERROR','panicked','Validation Error'])]
 assert not errors[folder],errors[folder]
result=dict(census_unchanged=True,before=before['metrics'],after=after['metrics'],
    added_linear_texture_pixel_bytes=6285312,added_existing_material_samples=1,
    added_meshes=0,added_draw_groups=0,native_error_lines=errors,
    limits='Single bounded whole-app wall-frame sample per candidate; not isolated GPU cost, causal regression or broad target-hardware acceptance. Both p95 values exceed16.7ms.')
(out/'comparison.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
