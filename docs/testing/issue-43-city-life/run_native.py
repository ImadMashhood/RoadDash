#!/usr/bin/env python3
"""Run the local city candidate with isolated saves and recorded input hashes."""
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[3]
OUT = Path(__file__).resolve().parent / sys.argv[1]
OUT.mkdir(exist_ok=True)
BINARY = Path(os.environ["ROADDASH_BINARY"])
MODE = "--smoke-test" if sys.argv[1].startswith("recovery") else "--metro-survey"
COMMAND = [str(BINARY), MODE]
FROZEN = [BINARY, ROOT / "content/world/metropolis.json", ROOT / "content/world/living_city.json"]


def hashes():
    return {str(path): hashlib.sha256(path.read_bytes()).hexdigest() for path in FROZEN}


before = hashes()
env = {
    **os.environ,
    "DISPLAY": ":0",
    "RUST_LOG": "warn",
    "ROADDASH_TRACK": "metro",
    "ROADDASH_CROSSING_SURVEY": "1",
    "ROADDASH_CITY_PEDESTRIAN_VIEWS": "1",
    "ROADDASH_UPNP": "0",
    "ROADDASH_SAVE_DIR": str(OUT / "save"),
    "ROADDASH_SETTINGS": str(OUT / "settings.json"),
    "ROADDASH_LOG_DIR": str(OUT / "performance"),
    "ROADDASH_VERIFY_REPAIRS": "1",
    "ROADDASH_VERIFY_REVERSE": "1",
}
with (OUT / "native.txt").open("w") as log:
    run = subprocess.run(COMMAND, cwd=OUT, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=420)

shots = sorted((OUT / "screenshots").glob("*.png"))
blank = [path.name for path in shots if all(hi - lo < 2 for lo, hi in Image.open(path).convert("RGB").getextrema())]
errors = [line for line in (OUT / "native.txt").read_text(errors="replace").splitlines() if "ERROR" in line or "panicked" in line]
report = {
    "command": COMMAND,
    "exit": run.returncode,
    "frozen_inputs": before,
    "inputs_unchanged": hashes() == before,
    "images": len(shots),
    "blank_images": blank,
    "error_lines": errors,
    "limits": "Scripted native survey/recovery and wall timing; no GPU timestamp, full race or hardware certification.",
}
(OUT / "receipt.json").write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps(report, indent=2))
assert run.returncode == 0 and shots and not blank and not errors and report["inputs_unchanged"]

if MODE == "--metro-survey":
    life = json.loads((OUT / "screenshots/metro-street-life.json").read_text())
    crossing = json.loads((OUT / "screenshots/pedestrian-crossings.json").read_text())
    assert life["main_street_cars"] == 144 and life["walking_pedestrians"] == 480
    assert crossing["authored"] == 13 and crossing["completed"] >= 2
    assert crossing["active"] <= crossing["max_active"] == 2
    print({"life": life, "crossings": {k: v for k, v in crossing.items() if k != "walkers"}})
