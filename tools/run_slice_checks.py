#!/usr/bin/env python3
"""Isolated offline integration receipts. These are not human playtest/fun metrics."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
SCENARIOS = {"entry": [("entry", "--frontend-smoke")],
             "full": [("entry", "--frontend-smoke"), ("police", "--police-smoke"),
                      ("recovery", "--smoke-test")]}


def validate_entry(report, seed):
    if not isinstance(report, dict):
        raise ValueError("missing native entry report")
    if report.get("stages") != 22 or not report.get("ended_in_garage"):
        raise ValueError("incomplete entry/race/results/return sequence")
    if report.get("race_seed") != seed:
        raise ValueError("native race seed differs from requested seed")
    ids = report.get("armed_rivals", [])
    if len(ids) != 7 or len(set(ids)) != 7 or any(type(i) is not int or not 1 <= i <= 14 for i in ids):
        raise ValueError("expected seven distinct non-player, non-police armed rivals")
    layouts = report.get("layouts", [])
    if len(layouts) != 5 or any(r.get("root") != r.get("window") or r.get("camera") != r.get("window") for r in layouts):
        raise ValueError("UI camera/root/window bounds differ")


def run(binary, output, suite="entry", timeout=240, seed=1234):
    binary = Path(binary).resolve()
    if not binary.is_file():
        raise ValueError("Build first with cargo build --release --locked")
    if timeout <= 0 or not 0 <= seed < 2**64:
        raise ValueError("timeout must be positive and seed must fit u64")
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=False)  # never overwrite an existing receipt
    receipt = {"schema_version": 1, "suite": suite, "seed": seed,
               "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(), "scenarios": [],
               "limits": "Scripted fixture checks only. No human play sessions, full race completion, fun targets or 60 FPS certification."}
    for name, flag in SCENARIOS[suite]:
        case = output / name
        case.mkdir()
        env = dict({k: v for k, v in os.environ.items() if not k.startswith("ROADDASH_")}, ROADDASH_TRACK="metro", ROADDASH_RACE_SEED=str(seed),
                   ROADDASH_SAVE_DIR=str(case / "test-save"), ROADDASH_SETTINGS=str(case / "test-settings.json"))
        row = {"name": name, "command": [str(binary), flag], "passed": False}
        started = time.monotonic()
        with (case / "native.log").open("w") as log:
            try:
                result = subprocess.run(row["command"], cwd=case, env=env, stdout=log,
                                        stderr=subprocess.STDOUT, timeout=timeout, check=False)
                row["exit_code"] = result.returncode
                if result.returncode:
                    raise ValueError(f"native fixture exited {result.returncode}")
                if name == "entry":
                    report = json.loads((case / "screenshots/frontend-report.json").read_text())
                    validate_entry(report, seed)
                    row["native_report"] = str(Path(name) / "screenshots/frontend-report.json")
                row["passed"] = True
            except (subprocess.TimeoutExpired, OSError, ValueError, TypeError, KeyError) as error:
                row["error"] = str(error)
        row["wall_seconds"] = round(time.monotonic() - started, 3)
        receipt["scenarios"].append(row)
    receipt["passed"] = all(row["passed"] for row in receipt["scenarios"])
    (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    return receipt


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/road-dash")
    parser.add_argument("--output", type=Path, required=True, help="New directory; existing receipts are preserved")
    parser.add_argument("--suite", choices=SCENARIOS, default="entry")
    parser.add_argument("--timeout", type=float, default=240)
    parser.add_argument("--seed", type=int, default=1234)
    args = parser.parse_args()
    try:
        receipt = run(args.binary, args.output, args.suite, args.timeout, args.seed)
    except (ValueError, OSError) as error:
        parser.error(str(error))
    print(json.dumps(receipt, indent=2))
    return 0 if receipt["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
