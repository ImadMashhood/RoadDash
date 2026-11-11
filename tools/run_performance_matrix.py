#!/usr/bin/env python3
"""Run cold control/candidate then reverse warm pair under an optional verified indexer pause.

Acquire the shared native flock externally. This pauses only explicitly requested
same-user Baloo processes via pidfds and restores them on completion/cancellation.
"""
import argparse
import os,signal,subprocess,json,time
from pathlib import Path
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary',type=Path,required=True)
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--indexer-pid',type=int,help='Explicit long-lived baloo_file parent PID; validate UID/executable/starttime before pausing.')
parser.add_argument('--gpu',action='store_true')
args=parser.parse_args()
root=Path(__file__).resolve().parents[1]
args.binary=args.binary.resolve()
args.output=args.output.resolve()
args.output.mkdir(parents=True,exist_ok=False)
proof=args.output/'indexer-control.json'

paused=[]
record={'started_unix':time.time(),'processes':[],'resumed':False,'cases':[]}
child=None
def interrupt(signum,frame):raise InterruptedError(f'signal {signum}')
signal.signal(signal.SIGTERM,interrupt)
signal.signal(signal.SIGINT,interrupt)
def pause(pid,expected):
 p=Path('/proc')/str(pid);s=(p/'stat').read_text().split();parent=Path('/proc')/s[3]
 uid=int(next(l for l in (p/'status').read_text().splitlines() if l.startswith('Uid:')).split()[1])
 row=dict(pid=pid,uid=uid,name=(p/'comm').read_text().strip(),exe=str((p/'exe').resolve()),cwd=str((p/'cwd').resolve()),start_ticks=s[21],parent_pid=int(s[3]),parent_exe=str((parent/'exe').resolve()),parent_name=(parent/'comm').read_text().strip())
 assert uid==os.getuid() and row['name']==expected
 assert Path(row['exe']).name==('baloo_file' if expected=='baloo_file' else 'baloo_file_extractor')
 if expected=='baloo_file_extr':assert row['parent_pid']==args.indexer_pid
 fd=os.pidfd_open(pid)
 assert (p/'stat').read_text().split()[21]==row['start_ticks']
 signal.pidfd_send_signal(fd,signal.SIGSTOP);paused.append((fd,row));record['processes'].append(row)
try:
 if args.indexer_pid is not None:
  pause(args.indexer_pid,'baloo_file')
 for p in Path('/proc').iterdir() if args.indexer_pid is not None else []:
  if not p.name.isdigit():continue
  try:
   if (p/'comm').read_text().strip()=='baloo_file_extr' and (p/'stat').read_text().split()[3]==str(args.indexer_pid):pause(int(p.name),'baloo_file_extr')
  except FileNotFoundError:pass
 proof.write_text(json.dumps(record,indent=2)+'\n')
 configs=[('cold','control',['--terrain-control','--cold-assets']),('cold','candidate',['--fixed-executor','single','--load-jobs','8','--cold-assets']),('warm','candidate',['--fixed-executor','single','--load-jobs','8']),('warm','control',['--terrain-control'])]
 for cache,kind,flags in configs:
  cmd=['python3','tools/run_performance_profiles.py','--binary',str(args.binary),'--output',str(args.output/cache/kind),'--runs','1',*(['--gpu'] if args.gpu else []),*flags]
  child=subprocess.Popen(cmd,cwd=root,start_new_session=True)
  code=child.wait();child=None
  record['cases'].append({'cache':cache,'kind':kind,'exit_code':code});proof.write_text(json.dumps(record,indent=2)+'\n')
  if code:raise RuntimeError(f'case failed {cache}/{kind}')
finally:
 if child is not None and child.poll() is None:
  os.killpg(child.pid,signal.SIGTERM)
  try:child.wait(timeout=15)
  except subprocess.TimeoutExpired:os.killpg(child.pid,signal.SIGKILL);child.wait()
 signal.signal(signal.SIGTERM,signal.SIG_IGN)
 signal.signal(signal.SIGINT,signal.SIG_IGN)
 for fd,row in reversed(paused):
  try:signal.pidfd_send_signal(fd,signal.SIGCONT);row['resume_signal_sent']=True
  except ProcessLookupError:row['process_exited_before_resume']=True
  finally:os.close(fd)
 record['resumed']=True;record['finished_unix']=time.time();proof.write_text(json.dumps(record,indent=2)+'\n')
