#!/usr/bin/env python3
"""Verify the narrow rollback against immutable repository snapshots."""
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
BASE = "f83cfbf7"
ORIGINAL = "6039be4e"
RESTORED = ["src/models.rs", "src/models/tests.rs"]
REMOVED = "src/models/style.rs"

def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)

def digest(data):
    return hashlib.sha256(data).hexdigest()

restored = {}
for name in RESTORED:
    current = (ROOT / name).read_bytes()
    prior = git("show", f"{ORIGINAL}:{name}")
    assert current == prior, name
    restored[name] = digest(current)
assert not (ROOT / REMOVED).exists()
changed = set(git("diff", BASE, "--name-only").decode().splitlines())
unexpected = {name for name in changed - set(RESTORED + [REMOVED])
              if not name.startswith("docs/testing/next-original-outfit/")}
assert not unexpected, unexpected
protected = ["src/models/articulation.rs", "src/models/hands.rs", "src/human.rs", "src/recovery.rs", "src/props.rs", "src/world.rs", "Cargo.lock", "assets/models/riding.rdmesh", "assets/models/rider-bindings.json", "assets/textures/rider-base.png", "assets/textures/rider-normal.png", "assets/textures/rider-roughness.png", "assets/shaders/grass_wind.wgsl"]
unchanged = {}
for name in protected:
    current = (ROOT / name).read_bytes()
    assert current == git("show", f"{BASE}:{name}"), name
    unchanged[name] = digest(current)
print(json.dumps({"base": BASE, "original_outfit": ORIGINAL, "restored_exactly": restored, "removed": REMOVED, "unchanged": unchanged}, indent=2))
