import subprocess,json,os,hashlib,time
from pathlib import Path
root=Path('/home/imad/Documents/ChatGPT/RoadDash-issue-25')
binary=root/'output/testing/issue25-final/road-dash'
out=root/'output/testing/issue25-final-native'
out.mkdir()
subprocess.run(['python3',str(root/'tools/run_performance_recovery.py'),'--binary',str(binary),'--output',str(out/'recovery'),'--production-defaults'],cwd=root,check=True)
receipts=[]
for track in ['metro','hills']:
 folder=out/track;folder.mkdir();(folder/'screenshots').mkdir()
 env={k:v for k,v in os.environ.items() if not k.startswith('ROADDASH_')}
 env.update(ROADDASH_TRACK=track,ROADDASH_RACE_SEED='25',ROADDASH_LOAD_PROFILE='1',ROADDASH_RAW_FRAMES='1',ROADDASH_SURVEY_CAPTURE='0',ROADDASH_HILLS_PERF='1',ROADDASH_LOG_DIR=str(folder/'logs'),ROADDASH_SETTINGS=str(folder/'settings.json'),ROADDASH_SAVE_DIR=str(folder/'save'),ROADDASH_DHT='0',ROADDASH_UPNP='0')
 flag='--metro-survey' if track=='metro' else '--hills-revision-survey'
 with (folder/'native.txt').open('w') as log:
  r=subprocess.run([str(binary),flag],cwd=folder,env=env,stdout=log,stderr=subprocess.STDOUT,timeout=900)
 text=(folder/'native.txt').read_text()
 assert r.returncode==0 and 'panicked at' not in text and 'ERROR' not in text,(track,r.returncode)
 files=list((folder/'logs').rglob('*.jsonl'))
 windows=[]
 for f in files:
  for line in f.read_text().splitlines():
   try: entry=json.loads(line)
   except json.JSONDecodeError: continue
   if 'raw_frame_ms' in entry:windows.append(entry)
 assert windows and windows[-1]['fixture_complete'],(track,files)
 race=sum(len(w.get('raw_racing_frame_ms') or []) for w in windows)
 assert all(w['requested_fixed_executor']=='single' for w in windows)
 assert windows[-1]['racing'] is False
 assert not any(w['dropped_log_samples'] for w in windows)
 if track=='metro': assert race==985,race
 receipt=dict(track=track,exit_code=r.returncode,production_defaults=True,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),race_frames=race,terminal_fixture_marker=True,terminal_racing=False,dropped_log_samples=0,timing_claim=False,flags=[flag])
 receipts.append(receipt);(out/'survey-receipts.json').write_text(json.dumps(receipts,indent=2)+'\n');print(json.dumps(receipt),flush=True)
