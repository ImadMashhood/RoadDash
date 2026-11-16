#!/usr/bin/env python3
"""Bake grounded woodland templates from the original Hills roadside GLBs."""
import argparse
import hashlib
import io
import json
import math
import struct
from collections import defaultdict
from functools import cache
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
CATALOG = ROOT / "content/props.json"
PROPS = ROOT / "art/props/mesh_assets"
OUTPUT = ROOT / "content/hills_tree_lods.json"
NAMES = (
    "TREE4A2", "TREE4B2", "TREE5A2", "TREEF2", "TREEU12",
    "TREEH2", "TREOAKA2", "TREOAKB2", "TREEK2", "TREEL2",
)


def linear(value):
    return value / 12.92 if value <= .04045 else ((value + .055) / 1.055) ** 2.4


def bake(prop):
    path = PROPS / prop["files"][2]
    data = path.read_bytes()
    size = struct.unpack_from("<I", data, 12)[0]
    gltf = json.loads(data[20:20 + size])
    binary = memoryview(data)[28 + size:]

    def view(index):
        bv = gltf["bufferViews"][index]
        start = bv.get("byteOffset", 0)
        return binary[start:start + bv["byteLength"]]

    def accessor(index, fmt):
        acc = gltf["accessors"][index]
        return list(struct.iter_unpack(
            fmt, view(acc["bufferView"])[acc.get("byteOffset", 0):]
            [:acc["count"] * struct.calcsize(fmt)],
        ))

    @cache
    def color(material):
        pbr = gltf["materials"][material]["pbrMetallicRoughness"]
        factor = pbr.get("baseColorFactor", [1, 1, 1])
        src = gltf["textures"][pbr["baseColorTexture"]["index"]]["source"]
        pixels = Image.open(io.BytesIO(view(gltf["images"][src]["bufferView"]))).convert("RGB")
        histogram = pixels.histogram()
        # Average linear light to match Bevy's sRGB texture sampling.
        return [sum(count * linear(value / 255) for value, count in enumerate(channel))
                / (pixels.width * pixels.height) * gain
                for channel, gain in zip(
                    (histogram[i:i + 256] for i in (0, 256, 512)), factor
                )]

    offset = list(prop["center"])
    offset[1] -= prop["size"][1] * .5
    trunk = []
    crowns = []
    for node in gltf["nodes"]:
        part = node["extras"]["semantic_part"]
        if part not in {"trunk", "layered_conifer_crown", "leaf_crown"}:
            continue
        assert set(node).isdisjoint({"matrix", "rotation", "scale", "translation"}), path
        for primitive in gltf["meshes"][node["mesh"]]["primitives"]:
            points = accessor(primitive["attributes"]["POSITION"], "<fff")
            # Exported hard-normal and UV seams duplicate vertices. Weld only
            # for component discovery, keeping separate crowns separate.
            keys = [tuple(round(v, 5) for v in pos) for pos in points]
            parents = {key: key for key in keys}

            def root(key):
                while parents[key] != key:
                    parents[key] = parents[parents[key]]
                    key = parents[key]
                return key

            indices = primitive["indices"]
            fmt = {5123: "<H", 5125: "<I"}[gltf["accessors"][indices]["componentType"]]
            indexed = [keys[i] for (i,) in accessor(indices, fmt)]
            for a, b, c in zip(indexed[::3], indexed[1::3], indexed[2::3]):
                parents[root(b)] = parents[root(a)]
                parents[root(c)] = parents[root(a)]
            components = defaultdict(list)
            for p in parents:
                components[root(p)].append(tuple((x - t) / s for x, t, s in zip(p, offset, prop["size"])))
            for vertices in components.values():
                if len(vertices) >= 4:
                    bounds = tuple((min(axis), max(axis)) for axis in zip(*vertices))
                    (trunk if part == "trunk" else crowns).append((bounds, color(primitive["material"])))

    assert len(trunk) == 1 and crowns, path
    return {
        "name": prop["name"], "source_sha256": hashlib.sha256(data).hexdigest(),
        "tiers": [template(trunk[0], crowns, prop["builder"] == "conifer", level)
                  for level in range(3)],
    }


def template(trunk, crowns, conifer, level):
    xyz = []
    rgb = []
    indices = []

    def shell(bounds, color, sides, profile):
        (x0, x1), (y0, y1), (z0, z1) = bounds
        rings = []
        for height, scale in profile:
            rings.append(list(range(len(xyz), len(xyz) + (sides if scale else 1))))
            for side in range(sides if scale else 1):
                angle = math.tau * (side + .125) / sides
                xyz.append([
                    (x0 + x1 + math.cos(angle) * (x1 - x0) * scale) / 2,
                    y0 + (y1 - y0) * height,
                    (z0 + z1 + math.sin(angle) * (z1 - z0) * scale) / 2,
                ])
                rgb.append(color)
        for low, high in zip(rings, rings[1:]):
            for i in range(sides):
                j = (i + 1) % sides
                if len(low) > 1:
                    indices.append([low[i], high[i % len(high)], low[j]])
                if len(high) > 1:
                    indices.append([low[j % len(low)], high[i], high[j]])

    shell(*trunk, (5, 4, 3)[level], [(0, 1), (1, .48)])
    crowns = sorted(crowns, key=lambda part: part[0][1][0])
    if conifer:
        # Retain the original stepped skirt heights and their alternating greens.
        if level != 0:
            crowns = [crowns[i] for i in [0, len(crowns) // 2, len(crowns) - 1]]
        for bounds, color in crowns:
            shell(bounds, color, (6, 5, 4)[level], [(0, 0), (.03, 1), (1, 0)] if level < 2 else [(0, 1), (1, 0)])
    elif level == 2:
        # Coalesce the crown lobes into a small rounded shell with the authored silhouette bounds.
        bounds = tuple((min(p[0][i][0] for p in crowns), max(p[0][i][1] for p in crowns)) for i in range(3))
        volume = lambda part: math.prod(high - low for low, high in part[0])
        weights = [volume(part) for part in crowns]
        color = [sum(p[1][i] * w for p, w in zip(crowns, weights)) / sum(weights) for i in range(3)]
        shell(bounds, color, 4, [(0, 0), (.33, .94), (.70, 1), (1, 0)])
    else:
        if level == 1:
            # Keep spread across the crown by selecting the lobes nearest each horizontal extreme.
            bounds_x = [p[0][0] for p in crowns]
            bounds_z = [p[0][2] for p in crowns]
            keep = {min(range(len(crowns)), key=lambda i: bounds_x[i][0]),
                    max(range(len(crowns)), key=lambda i: bounds_x[i][1]),
                    min(range(len(crowns)), key=lambda i: bounds_z[i][0]),
                    max(range(len(crowns)), key=lambda i: bounds_z[i][1]),
                    len(crowns) - 1}
            crowns = [p for i, p in enumerate(crowns) if i in keep]
        for bounds, color in crowns:
            shell(bounds, color, (5, 4)[level], [(0, 0), (.52, 1), (1, 0)])

    return {"positions": [[round(x, 6) for x in p] for p in xyz],
            "colors": [[round(x, 6) for x in c] for c in rgb], "triangles": indices}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--check", action="store_true")
    args = ap.parse_args()
    records = {p["name"]: p for p in json.loads(CATALOG.read_text())["records"]}
    families = [bake(records[name]) for name in NAMES]
    encoded = json.dumps(families, separators=(",", ":")) + "\n"
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != encoded:
            raise SystemExit("Hills tree templates are stale")
    else:
        OUTPUT.write_text(encoded)
    for f in families:
        print(f["name"], *(len(t["triangles"]) for t in f["tiers"]))


if __name__ == "__main__":
    main()
