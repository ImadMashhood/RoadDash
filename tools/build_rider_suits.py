#!/usr/bin/env python3
"""Bake original suit finishes into the existing rider UV layout."""
from pathlib import Path

from PIL import Image

textures = Path(__file__).resolve().parents[1] / "assets" / "textures"
base = Image.open(textures / "rider-base.png").convert("RGB")
source = base.tobytes()

for name, accent, trim in [
    ("ember", (225, 84, 40), (36, 24, 20)),
    ("glacier", (61, 170, 206), (14, 36, 51)),
]:
    result = bytearray(source)
    for offset in range(0, len(source), 3):
        channels = source[offset:offset + 3]
        if max(channels) - min(channels) >= 20 or max(channels) < 10:
            continue
        light = sum(channels) / 3
        bright = min(1, max(0, (light - 75) / 130))
        fabric = min(1, max(0, (light - 8) / 24)) * (1 - bright)
        y, x = divmod(offset // 3, base.width)
        pattern = (x + y) % 18 < 3 if name == "ember" else (x // 4 + y // 4) % 2 == 0
        for channel, original in enumerate(channels):
            value = original * (1 - bright) + accent[channel] * (0.78 + light / 1150) * bright
            value += fabric * trim[channel] * (0.35 + 0.23 * pattern)
            result[offset + channel] = min(255, int(value))
    Image.frombytes("RGB", base.size, bytes(result)).save(
        textures / f"rider-suit-{name}.png", optimize=True
    )
