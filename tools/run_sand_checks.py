#!/usr/bin/env python3
"""Reproduce the integrated sand and recovery native functional receipts."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / "target/release/road-dash"
OUTPUT = ROOT / "output/testing/issue29-native"
CASES = (
    ("surface-hills", "hills", "0", ["--surface-survey"]),
    ("surface-metro", "metro", "0", ["--surface-survey"]),
    ("recovery-anatomy", "hills", "0", ["--smoke-test"]),
    ("recovery-baseline", "hills", "1", ["--smoke-test", "--ragdoll-baseline"]),
)


def main() -> None:
    binary_hash = hashlib.sha256(BINARY.read_bytes()).hexdigest()
    OUTPUT.mkdir(parents=True, exist_ok=True)
    for name, track, route, flags in CASES:
        folder = OUTPUT / name
        (folder / "screenshots").mkdir(parents=True, exist_ok=True)
        env = dict(
            {key: value for key, value in os.environ.items() if not key.startswith("ROADDASH_")},
            ROADDASH_TRACK=track,
            ROADDASH_SMOKE_ROUTE=route,
            ROADDASH_VEHICLE="dynamic",
            ROADDASH_VERIFY_HILLS_RESCUE="1" if name.startswith("recovery") else "0",
            ROADDASH_SETTINGS=str(folder / "settings.json"),
            ROADDASH_SAVE_DIR=str(folder / "save"),
            ROADDASH_LOG_DIR=str(folder / "performance"),
            ROADDASH_DHT="0",
            ROADDASH_UPNP="0",
        )
        output = folder / "screenshots" / ("surface-survey.json" if name.startswith("surface") else "smoke-metrics.json")
        output.unlink(missing_ok=True)
        started = time.monotonic()
        with (folder / "native.txt").open("w") as log:
            result = subprocess.run([str(BINARY), *flags], cwd=folder, env=env,
                                    stdout=log, stderr=subprocess.STDOUT, timeout=600)
        data = json.loads(output.read_text()) if output.exists() else None
        if name.startswith("surface"):
            passed = result.returncode == 0 and isinstance(data, list) and len(data) == 2 and all(
                row["grounded"] == [True, True] and row["speed_mps"] > 2 for row in data
            )
            if name == "surface-hills" and data:
                passed = passed and data[0]["surface"] == "asphalt" and data[1]["surface"] == "sand"
            if name == "surface-metro" and data:
                passed = passed and data[0]["surface"] == "asphalt" and data[1]["surface"] != "sand"
        else:
            passed = (result.returncode == 0 and isinstance(data, dict)
                      and all(data.get("fixed_recovery_stages", []))
                      and len(data.get("fixed_recovery_stages", [])) >= 2
                      and data.get("exceptional_recovery", {}).get("unassisted_both") is True)
        receipt = dict(case=name, track=track, route=route, flags=flags,
                       binary_sha256=binary_hash, elapsed_wall_s=time.monotonic() - started,
                       exit_code=result.returncode, output_present=output.exists(), passed=passed,
                       limits="Functional native check, not a frame-time or human-feel measurement.")
        (folder / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        print(json.dumps(receipt), flush=True)
        if not passed:
            raise SystemExit(f"Native case failed: {name}")


if __name__ == "__main__":
    main()
