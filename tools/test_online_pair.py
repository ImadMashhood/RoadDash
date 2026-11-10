#!/usr/bin/env python3
"""Run real native peers without public discovery/router changes; verify receipts.

This is functional validation, not a frame-time benchmark. Keep this supervising
process alive: map changes intentionally replace the native processes.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time


def run(args):
    binary = Path(args.binary).resolve()
    output = Path(args.output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    for name in ("host.json", "guest.json"):
        if (output / name).exists():
            raise RuntimeError("Use a fresh output directory; old receipts cannot validate a new run")
    endpoint = f"127.0.0.1:{args.port}"
    (output / "tmp").mkdir(exist_ok=True)
    (output / "performance").mkdir(exist_ok=True)
    env = dict(os.environ, TMPDIR=str(output / "tmp"), ROADDASH_LOG_DIR=str(output / "performance"), ROADDASH_UPNP="0", ROADDASH_DHT="0",
               ROADDASH_ADVERTISE_ADDR=endpoint, ROADDASH_SAVE_DIR=str(output / "saves"))
    env["ROADDASH_ONLINE_TEST_SECONDS"] = str(args.race_seconds)
    env.pop("ROADDASH_ONLINE_BOOT", None)
    env.pop("ROADDASH_HOST_SECRET", None)
    if args.reload:
        env["ROADDASH_TEST_RELOAD_MAP"] = args.reload
    else:
        env.pop("ROADDASH_TEST_RELOAD_MAP", None)
    processes = []
    logs = []

    def start(role, address, name):
        log = (output / f"{name}.log").open("w")
        logs.append(log)
        process = subprocess.Popen([str(binary), "--online-native-test", role, address,
                                    str(output / f"{name}.json")], stdout=log,
                                   stderr=subprocess.STDOUT, env=env, start_new_session=True)
        processes.append(process)
        return process

    started = time.monotonic()
    host = start("host", endpoint, "host")
    try:
        code = None
        while time.monotonic() - started < args.timeout:
            text = (output / "host.log").read_text(errors="replace")
            codes = [line.split("=", 1)[1].strip() for line in text.splitlines()
                     if line.startswith("HOST_CODE=")]
            if codes:
                code = codes[0]
                break
            if host.poll() is not None:
                raise RuntimeError(f"Host stopped before readiness: exit {host.returncode}; inspect host.log")
            time.sleep(.2)
        if code is None:
            raise RuntimeError("Host assets/lobby never became ready")
        print("Host ready; starting independent native guest", flush=True)
        start("client", code, "guest")
        next_update = time.monotonic() + 20
        while not all((output / f"{name}.json").exists() for name in ("host", "guest")):
            if time.monotonic() - started >= args.timeout:
                raise RuntimeError("Native peers did not complete before timeout; inspect logs")
            if time.monotonic() >= next_update:
                print("Native race/recovery fixture still running", flush=True)
                next_update += 20
            time.sleep(.2)
        reports = {name: json.loads((output / f"{name}.json").read_text()) for name in ("host", "guest")}
        for name, report in reports.items():
            assert report["max_players"] >= 2, (name, "no real peer")
            assert report["remote_max_speed"] > 5, (name, "no shared movement")
            assert not report["errors"], (name, report["errors"])
            for field in ("remote_attack", "remote_crash", "remote_foot", "remote_remount"):
                assert report[field], (name, field)
        authority = {tick: digest for tick, digest, _ in reports["host"]["samples"]}
        common = [(tick, digest) for tick, digest, _ in reports["guest"]["samples"] if tick in authority]
        assert len(common) > 100, "Insufficient overlapping authoritative snapshots"
        assert all(authority[tick] == digest for tick, digest in common), "Shared state mismatch"
        assert reports["host"]["first_snapshot_wall_seconds"] >= 14.5, "Countdown shortened"
        count = len(reports["host"]["samples"])
        average = reports["host"]["serialized_bytes"] / count
        codes = [line.split("=", 1)[1].strip() for line in (output / "host.log").read_text(errors="replace").splitlines()
                 if line.startswith("HOST_CODE=")]
        if args.reload:
            assert len(codes) >= 2 and codes[0] == codes[-1], "Private capability changed during reload"
        summary = {"passed": True, "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                   "shared_matching_snapshots": len(common), "reload": args.reload,
                   "host_snapshot_mean_bytes": average,
                   "host_snapshot_max_bytes": reports["host"]["largest_snapshot"],
                   "uncompressed_snapshot_bytes_per_second_at_20hz": (average + 4) * 20,
                   "encoded_frame_bytes_per_second_at_20hz": reports["host"]["encoded_frame_bytes"] / count * 20,
                   "largest_encoded_frame": reports["host"]["largest_encoded_frame"],
                   "note": "Encoded frame estimate excludes lobby/TCP/IP overhead; no performance or WAN claim"}
        (output / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
        print(json.dumps(summary), flush=True)
    finally:
        # A relaunch remains in its original process group; clean up descendants too.
        import signal
        for process in processes:
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
        for process in processes:
            process.wait(timeout=10)
        for log in logs:
            log.close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--port", type=int, default=47671)
    parser.add_argument("--reload", choices=("valley", "city", "metro"))
    parser.add_argument("--race-seconds", type=int, default=45)
    parser.add_argument("--timeout", type=int, default=360)
    run(parser.parse_args())
