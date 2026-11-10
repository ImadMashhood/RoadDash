#!/usr/bin/env python3
"""Repeat isolated native course surveys and preserve raw loading/frame evidence.

This is a fixed-camera traversal with live street actors, not a free-driving,
combat or hardware certification. Optional cold-asset runs evict only checkout file pages, never global caches.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import time
from summarize_performance import summarize


def processes(exclude=()):
    rows = subprocess.check_output(['ps', '-eo', 'pid=,comm='], text=True)
    return [(int(pid), name) for row in rows.splitlines()
            for pid, name in [row.strip().split(None, 1)]
            if int(pid) not in exclude and
            (name in ('cargo', 'rustc', 'clippy-driver', 'road-dash') or name.startswith(('road-dash', 'road_dash-', 'rr_core-', 'rr_content-', 'issue25-')))]


def evict_asset_pages(root):
    """Best-effort clean-page eviction for this checkout only; never global drop_caches."""
    files = [p for folder in [root/'assets', root/'art/props/mesh_assets', root/'art/buildings/mesh_assets']
             for p in folder.rglob('*') if p.is_file()]
    def residency():
        total = 0
        for start in range(0, len(files), 128):
            result = json.loads(subprocess.check_output(
                ['fincore', '--json', '--bytes', '--output', 'RES,SIZE,FILE',
                 *map(str, files[start:start+128])], text=True))
            total += sum(int(row['res']) for row in result['fincore'])
        return total
    before = residency()
    for path in files:
        fd = os.open(path, os.O_RDONLY)
        try:
            os.posix_fadvise(fd, 0, 0, os.POSIX_FADV_DONTNEED)
        finally:
            os.close(fd)
    return dict(files=len(files), resident_bytes_before=before, resident_bytes_after=residency(),
                method='POSIX_FADV_DONTNEED on checkout asset files, verified with fincore; executable, metadata and GPU driver caches are not reset')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--track', choices=['hills', 'metro', 'both'], default='both')
    parser.add_argument('--runs', type=int, default=2)
    parser.add_argument('--indexer-pid', type=int)
    parser.add_argument('--gpu', action='store_true')
    parser.add_argument('--cold-assets', action='store_true', help='Evict only this checkout asset file pages and record fincore residency (Linux).')
    parser.add_argument('--terrain-control', action='store_true')
    parser.add_argument('--fixed-executor', choices=['single','multi'], default='multi')
    parser.add_argument('--load-jobs', choices=[2,8], type=int, default=2)
    args = parser.parse_args()
    if args.runs < 1:
        parser.error('runs must be positive')
    binary = args.binary.resolve()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    assert not processes(), 'another game/compiler/test invalidates timing'
    suspended = False
    receipts = []
    root = Path(__file__).resolve().parents[1]
    source = dict(head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip(),
                  tracked_diff_sha256=hashlib.sha256(subprocess.check_output(['git', 'diff', 'HEAD'], cwd=root)).hexdigest(),
                  checkout=str(root))
    source['untracked_source_sha256'] = {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
        for name in subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard', 'src', 'crates'], cwd=root, text=True).splitlines()
        for p in [root/name] if p.is_file()}
    try:
        if args.indexer_pid:
            name = Path(f'/proc/{args.indexer_pid}/comm').read_text().strip()
            assert name in ('baloo_file_extr', 'baloo_file'), 'only exact Baloo indexer PIDs are accepted'
            os.kill(args.indexer_pid, signal.SIGSTOP)
            suspended = True
        for run in range(args.runs):
            for track in (['hills', 'metro'] if args.track == 'both' else [args.track]):
                folder = output / f'{track}-{run+1}'
                folder.mkdir()
                (folder / 'screenshots').mkdir()
                env = {k: v for k, v in os.environ.items() if not k.startswith('ROADDASH_')}
                env.update(ROADDASH_TRACK=track, ROADDASH_RACE_SEED='25',
                           ROADDASH_FIXED_EXECUTOR=args.fixed_executor, ROADDASH_LOAD_JOBS=str(args.load_jobs),
                           ROADDASH_LOAD_PROFILE='1', ROADDASH_RAW_FRAMES='1',
                           ROADDASH_GPU_PROFILE='1' if args.gpu else '0',
                           ROADDASH_HILLS_PERF='1', ROADDASH_SURVEY_CAPTURE='0',
                           ROADDASH_TERRAIN_CACHE='0' if args.terrain_control else '1',
                           ROADDASH_LOG_DIR=str(folder/'logs'),
                           ROADDASH_SETTINGS=str(folder/'settings.json'),
                           ROADDASH_SAVE_DIR=str(folder/'save'),
                           ROADDASH_DHT='0', ROADDASH_UPNP='0')
                flags = ['--hills-revision-survey' if track == 'hills' else '--metro-survey']
                cache_probe = evict_asset_pages(Path(__file__).resolve().parents[1]) if args.cold_assets else None
                start = time.monotonic()
                contaminants = set()
                with (folder/'native.txt').open('w') as out:
                    child = subprocess.Popen([str(binary), *flags], cwd=folder, env=env,
                                             stdout=out, stderr=subprocess.STDOUT)
                    try:
                        while child.poll() is None:
                            contaminants.update(processes([child.pid]))
                            if time.monotonic()-start > 600:
                                raise TimeoutError('native case exceeded 600 seconds')
                            time.sleep(.5)
                    finally:
                        if child.poll() is None:
                            child.terminate()
                            try:
                                child.wait(timeout=10)
                            except subprocess.TimeoutExpired:
                                child.kill()
                                child.wait()
                marks = [json.loads(line.split('LOAD_PROFILE ', 1)[1])
                         for line in (folder/'native.txt').read_text().splitlines()
                         if 'LOAD_PROFILE ' in line]
                report = dict(track=track, run=run+1, exit_code=child.returncode, source=source,
                              binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                              flags=flags, environment={k:v for k,v in env.items() if k.startswith('ROADDASH_')},
                              duration_s=time.monotonic()-start, loading=marks,
                              contaminants=sorted(contaminants), indexer_suspended=args.indexer_pid,
                              asset_page_cache=cache_probe,
                              cache_state='fresh process; targeted asset pages evicted' if args.cold_assets else 'fresh process; OS/driver caches retained, not certified cold disk',
                              measurement='fixed-camera traversal/live street actors; manual 60Hz simulation clock; raw wall frames and CPU spans; asynchronous GPU metrics only when present')
                logs = list((folder/'logs').glob('*.jsonl'))
                if logs:
                    report['runtime'] = summarize(logs[0], racing_only=True, skip_racing_samples=3, raw_skip_frames=180)
                (folder/'summary.json').write_text(json.dumps(report, indent=2)+'\n')
                receipts.append(report)
                (output/'receipts.json').write_text(json.dumps(receipts, indent=2)+'\n')
                print(json.dumps(dict(track=track, run=run+1, duration_s=report['duration_s'], exit_code=child.returncode, contaminants=sorted(contaminants))), flush=True)
                if child.returncode or contaminants:
                    raise RuntimeError('failed or contaminated run; evidence retained, do not claim a clean measurement')
    finally:
        if suspended:
            try:
                os.kill(args.indexer_pid, signal.SIGCONT)
            except ProcessLookupError:
                pass


if __name__ == '__main__':
    main()
