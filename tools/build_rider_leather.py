#!/usr/bin/env python3
"""Lift dark rider atlas texels so the leather takes a separate palette tint."""
from pathlib import Path

from PIL import Image

root = Path(__file__).resolve().parents[1] / "assets" / "textures"
base = Image.open(root / "rider-base.png").convert("RGB")
# Keep dark seams and baked highlights while lifting the leather's albedo.
leather = base.point([round(255 * (channel / 255) ** 0.50) for channel in range(256)] * 3)
leather.save(root / "rider-leather.png", optimize=True)
