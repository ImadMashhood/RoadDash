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
    def fitted(position):
        return tuple((axis - center) / size for axis, center, size in zip(position, offset, prop["size"]))

    parts = []
    for node in gltf["nodes"]:
        part = node["extras"]["semantic_part"]
        if part not in {"trunk", "branch", "layered_conifer_crown", "leaf_crown"}:
            continue
        assert set(node).isdisjoint({"matrix", "rotation", "scale", "translation"}), path
        for primitive in gltf["meshes"][node["mesh"]]["primitives"]:
            points = accessor(primitive["attributes"]["POSITION"], "<fff")
            # Weld the export's flat-normal and UV splits only for component
            # discovery. The closest tier retains its original face topology.
            keys = [tuple(round(v, 5) for v in pos) for pos in points]
            parents = {key: key for key in keys}

            def root(key):
                while parents[key] != key:
                    parents[key] = parents[parents[key]]
                    key = parents[key]
                return key

            indices = primitive["indices"]
            fmt = {5123: "<H", 5125: "<I"}[gltf["accessors"][indices]["componentType"]]
            values = [i for (i,) in accessor(indices, fmt)]
            faces = list(zip(values[::3], values[1::3], values[2::3]))
            for a, b, c in faces:
                parents[root(keys[b])] = parents[root(keys[a])]
                parents[root(keys[c])] = parents[root(keys[a])]
            components = defaultdict(list)
            for face in faces:
                components[root(keys[face[0]])].append(face)
            for faces in components.values():
                vertices = dict.fromkeys(i for face in faces for i in face)
                parts.append((part, [fitted(points[i]) for i in vertices], list(faces),
                              dict(zip(vertices, range(len(vertices)))), color(primitive["material"])))

    return {
        "name": prop["name"], "source_sha256": hashlib.sha256(data).hexdigest(),
        "tiers": [template(parts, level) for level in range(3)],
    }


def source_rings(vertices):
    rings = defaultdict(dict)
    for x, y, z in vertices:
        rings[round(y, 5)][round(x, 5), round(z, 5)] = (x, y, z)
    return [list(ring.values()) for _, ring in sorted(rings.items())]


def ring_center(ring):
    return tuple(sum(p[axis] for p in ring) / len(ring) for axis in range(3))


def resample(ring, sides):
    if len(ring) < 2:
        return ring
    sides = min(sides, len(ring))
    x, _, z = ring_center(ring)
    ordered = sorted(ring, key=lambda p: math.atan2(p[2] - z, p[0] - x))
    return [ordered[(i * len(ordered) + sides // 2) // sides] for i in range(sides)]


def template(parts, level):
    xyz, rgb, triangles = [], [], []

    def append(vertices, faces, color):
        start = len(xyz)
        xyz.extend(vertices)
        rgb.extend([color] * len(vertices))
        triangles.extend([[start + i for i in face] for face in faces])

    def append_face(vertices, color):
        append(vertices, [(0, i, i + 1) for i in range(1, len(vertices) - 1)], color)

    def join(rings, color, flat=False):
        offsets = []
        vertices = []
        for ring in rings:
            offsets.append(list(range(len(vertices), len(vertices) + len(ring))))
            vertices.extend(ring)
        faces = []
        for low, high in zip(offsets, offsets[1:]):
            sides = max(len(low), len(high))
            for i in range(sides):
                j = (i + 1) % sides
                face = list(dict.fromkeys([low[i % len(low)], high[i % len(high)],
                                           high[j % len(high)], low[j % len(low)]]))
                faces.append(face)
        for ring, reverse in ((offsets[0], False), (offsets[-1], True)):
            if len(ring) > 1:
                faces.append(list(reversed(ring)) if reverse else ring)
        if flat:
            for face in faces:
                append_face([vertices[i] for i in face], color)
        else:
            append(vertices, [(face[0], face[i], face[i + 1]) for face in faces
                              for i in range(1, len(face) - 1)], color)

    leaves = [part for part in parts if part[0] == "leaf_crown"]
    if level == 2 and leaves:
        # Retain the central/top crown and the most widely spread outer lobes.
        centers = [ring_center(part[1]) for part in leaves]
        kept = [max(range(len(leaves)), key=lambda i: centers[i][1])]
        for _ in range(3):
            kept.append(max((i for i in range(len(leaves)) if i not in kept),
                            key=lambda i: min(math.dist(centers[i], centers[j]) for j in kept)))
        leaves = [leaves[i] for i in kept]

    for part in parts:
        name, vertices, faces, remap, color = part
        if level == 0 and name != "branch" or level == 1 and name == "trunk":
            append(vertices, [[remap[i] for i in face] for face in faces], color)
        elif name == "branch":
            continue
        elif name == "leaf_crown":
            if not any(part is leaf for leaf in leaves):
                continue
            rings = source_rings(vertices)
            # Keep the upper and lower shoulder rings. The previous proxies
            # discarded these and made rounded leaves read as diamonds.
            if level == 2:
                rings = [rings[0], rings[1], rings[-2], rings[-1]]
            rings = [resample(ring, 3 if level == 2 else 4) for ring in rings]
            join(rings, color)
        elif name == "layered_conifer_crown":
            rings = source_rings(vertices)
            # Every skirt remains present at every distance, at its real
            # height and taper. Collapse only its tiny far-away upper rim.
            lower = resample(rings[0], 4 if level == 1 else 3)
            upper = resample(rings[1], 4) if level == 1 else [ring_center(rings[1])]
            join([lower, upper], color, flat=True)
        else:
            # The authored trunk leans slightly, so its two ends have
            # distinct Y coordinates within each end ring.
            ranked = sorted(set(vertices), key=lambda p: p[1])
            half = len(ranked) // 2
            join([resample(ranked[:half], 3), resample(ranked[half:], 3)], color, flat=True)

    return {"positions": [[round(x, 6) for x in p] for p in xyz],
            "colors": [[round(x, 6) for x in c] for c in rgb], "triangles": triangles}


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
