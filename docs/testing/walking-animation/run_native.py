#!/usr/bin/env python3
"""Run functional gait/recovery review from an explicitly frozen parent binary."""
import hashlib,json,os,subprocess,sys
from pathlib import Path
from PIL import Image
out=Path(__file__).resolve().parent
name=sys.argv[1]
folder=out/name;folder.mkdir(exist_ok=True)
binary=Path(os.environ['ROADDASH_TEST_BINARY']).resolve()
args=['--animation-lab-view','--animation-lab-smoke'] if name=='after' else ['--smoke-test']
if 'baseline' in name:args.append('--ragdoll-baseline')
env={**os.environ,'DISPLAY':':0','RUST_LOG':'warn','ROADDASH_TRACK':'metro','ROADDASH_SAVE_DIR':str(folder/'save'),'ROADDASH_SETTINGS':str(folder/'settings.json'),'ROADDASH_LOG_DIR':str(folder/'performance'),'ROADDASH_SMOKE_ROUTE':'1' if 'baseline' in name else '0','ROADDASH_VERIFY_REPAIRS':'1','ROADDASH_VERIFY_REVERSE':'1'}
if name=='after':env['ROADDASH_GAIT_REVIEW']='1'
command=[str(binary),*args];digest=hashlib.sha256(binary.read_bytes()).hexdigest()
with (folder/'native.txt').open('w') as log:
 result=subprocess.run(command,cwd=folder,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=240)
frames=list(folder.rglob('*.png'));assert frames
blank=[str(p.relative_to(folder)) for p in frames if all(hi-lo<2 for lo,hi in Image.open(p).convert('RGB').getextrema())]
receipt=dict(command=command,exit=result.returncode,binary_sha256=digest,binary_unchanged=digest==hashlib.sha256(binary.read_bytes()).hexdigest(),frames=len(frames),blank=blank,scope='functional visual review; not a performance benchmark')
(folder/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(receipt)
assert result.returncode==0 and not blank
