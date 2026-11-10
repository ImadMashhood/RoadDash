"""Run isolated native real-surface walk-back/remount fixtures and lane survey."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time


def run(binary, output):
    binary = Path(binary).resolve()
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=False)
    receipt = {"binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(), "cases": [],
               "limits": "Scripted Linux native fixtures; no human feel, Windows or performance certificate."}
    cases = [(f"route-{route}-{mode}", ["--smoke-test"] + (["--ragdoll-baseline"] if mode == "baseline" else []), route)
             for mode in ["anatomy", "baseline"] for route in [0, 1]]
    cases.append(("metro-survey", ["--metro-survey"], None))
    for name, args, route in cases:
        case = output / name
        case.mkdir()
        env = {k: v for k, v in os.environ.items() if not k.startswith("ROADDASH_")}
        env.update(ROADDASH_TRACK="metro", ROADDASH_RACE_SEED="1234",
                   ROADDASH_SAVE_DIR=str(case / "test-save"), ROADDASH_SETTINGS=str(case / "test-settings.json"),
                   ROADDASH_LOG_DIR=str(case / "logs"), RUST_LOG="warn")
        if route is not None:
            env.update(ROADDASH_SMOKE_ROUTE=str(route), ROADDASH_VERIFY_WALK_CROSSINGS="1")
        row = {"name": name, "args": args, "passed": False}
        started = time.monotonic()
        with (case / "native.log").open("w") as log:
            try:
                result = subprocess.run([str(binary), *args], cwd=case, env=env,
                                        stdout=log, stderr=subprocess.STDOUT, timeout=240, check=False)
                row["exit_code"] = result.returncode
                if result.returncode:
                    raise ValueError(f"native fixture exited {result.returncode}")
                if route is not None:
                    metrics = json.loads((case / "screenshots/smoke-metrics.json").read_text())
                    assert metrics["walk_crossing_setups"] == [True, True]
                    assert len(metrics["walk_crossing_distances_m"]) == 2
                    assert all(d > 5 for d in metrics["walk_crossing_distances_m"])
                    assert all(metrics["fixed_recovery_stages"])
                    row["walk_crossing_distances_m"] = metrics["walk_crossing_distances_m"]
                row["passed"] = True
            except (subprocess.TimeoutExpired, OSError, ValueError, KeyError, AssertionError) as error:
                row["error"] = str(error)
        row["wall_seconds"] = round(time.monotonic() - started, 3)
        receipt["cases"].append(row)
        (output / "receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
        print(json.dumps(row), flush=True)
        if not row["passed"]:
            return False
    return True


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    raise SystemExit(0 if run(args.binary, args.output) else 1)
