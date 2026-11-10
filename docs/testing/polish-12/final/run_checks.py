#!/usr/bin/env python3
"""Run final integrated checks after pass12 native work stops."""
import json, os, subprocess
from pathlib import Path
root = Path(__file__).resolve().parents[4]
out = Path(__file__).resolve().parent
commands = [
    ("fmt", ["cargo", "fmt", "--all", "--check"]),
    ("clippy", ["cargo", "clippy", "--workspace", "--locked", "--all-targets", "--", "-D", "warnings"]),
    ("tests", ["cargo", "test", "--workspace", "--locked"]),
    ("build", ["cargo", "build", "--locked"]),
    ("audit", ["python3", "tools/audit.py"]),
    ("roadmap", ["python3", "-m", "unittest", "discover", "-s", "tools", "-p", "test_roadmap.py"]),
    ("progress", ["python3", "tools/sync_progress.py", "--check"]),
]
commands += [("release", ["cargo", "build", "--release", "--locked"])]
commands += [("authoring-"+name, ["/tmp/roaddash-layout-venv/bin/python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_metropolis_"+name+".py"]) for name in ["districts", "facades", "street_life", "streetscape", "population", "elevation"]]
commands += [("atlas", ["python3", "tools/verify_facade_lod.py"]), ("profiles", ["python3", "tools/verify_material_profiles.py"])]
results = {}
for name, command in commands:
    with (out / (name + ".txt")).open("w") as log:
        result = subprocess.run(command, cwd=root, stdout=log, stderr=subprocess.STDOUT,
            env={**os.environ, "CARGO_TARGET_DIR": "/home/imad/Documents/ChatGPT/RoadRash/target"})
    results[name] = {"command": command, "exit": result.returncode}
    (out / "checks.json").write_text(json.dumps(results, indent=2) + "\n")
    print(name, result.returncode, flush=True)

if any(v["exit"] for v in results.values()):
    raise SystemExit(1)
