#!/usr/bin/env python3
"""Author the RoadDash traffic presets and phase-aligned loops for Engine Simulator."""
import argparse
from array import array
import math
import re
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
TWIN = ROOT / "docs/testing/demo-combat-effects-audio/engine.mr"
CLIPS = ROOT / "assets/audio/traffic"
MODELS = {
    # Cylinders, bore/stroke mm, chamber cc, flywheel lb, exhaust tube mm,
    # exhaust volume L, intake/exhaust flow scale, redline, muffler
    "car": (4, 81, 88, 55, 13, 440, 4.5, 1.25, 6200, "minimal_muffling_01"),
    "truck": (6, 99, 100, 93, 23, 640, 7.0, 1.8, 4600, "minimal_muffling_02"),
    "tractor": (3, 103, 127, 53, 42, 760, 4.8, 1.9, 2800, "minimal_muffling_01"),
    "semi": (6, 115, 132, 153, 72, 1050, 12., 2.6, 2500, "minimal_muffling_02"),
}


def preset(model):
    n, bore, stroke, chamber, flywheel, tube, volume, flow, redline, muffler = MODELS[model]
    starter_torque = 45 + flywheel * (3 if model == "semi" else 2)
    original = TWIN.read_text()
    base = original[original.index("    engine engine("):original.index("    cylinder_bank bank(")]
    values = {
        '689 cc 270-degree Twin': model.title(),
        '45 * units.lb_ft': f'{starter_torque} * units.lb_ft',
        '650 * units.rpm': '450 * units.rpm', '10000 * units.rpm': f'{redline} * units.rpm',
        'noise: 0.24': 'noise: 0.19', 'jitter: 0.1': 'jitter: 0.04',
        '68.6 * units.mm': f'{stroke} * units.mm', '102 * units.mm': f'{round(stroke * 1.62)} * units.mm',
        '5 * units.lb': f'{flywheel} * units.lb', '8 * units.lb': f'{round(flywheel * 1.6)} * units.lb',
        '4 * units.lb_ft': f'{round(flywheel * 0.25)} * units.lb_ft',
        '0.055': f'{flywheel * 0.0105:.3f}', '80 * units.mm': f'{bore} * units.mm',
        '360 * units.g': f'{round(bore**2 * 0.07)} * units.g',
        '310 * units.g': f'{round(bore**2 * 0.06)} * units.g',
        '2.5 * units.L': f'{2.5 * flow:.2f} * units.L',
        '13 * units.cm2': f'{13 * flow:.2f} * units.cm2',
        'k_carb(280)': f'k_carb({round(280 * flow)})', 'k_carb(230)': f'k_carb({round(230 * flow)})',
        'k_carb(360)': f'k_carb({round(360 * flow)})', 'k_carb(250)': f'k_carb({round(250 * flow)})',
        '110 * units.mm': f'{round(90 + tube * 0.1)} * units.mm',
        '450 * units.mm': f'{tube} * units.mm', '2.2 * units.L': f'{volume} * units.L',
        'minimal_muffling_02': muffler,
    }
    # Apply substitutions against the unmodified template, including displacements.
    pattern = '|'.join(re.escape(x) for x in sorted(values, key=len, reverse=True))
    base = re.sub(pattern, lambda match: values[match.group()], base)
    base = base.replace('    twin_wires wires()\n', '').replace('    label second_fire(270 * units.deg)\n', '')
    journals = ''.join(
        f'    rod_journal j{i}(angle: {angle % 360} * units.deg)\n'
        f'    crank.add_rod_journal(j{i})\n'
        for i, angle in enumerate(range(0, 720, 720 // n))
    )
    base = base[:base.index('    rod_journal first')] + journals + base[base.index('\n    piston_parameters'):]
    result = 'import "engine_sim.mr"\n\nunits units()\nimpulse_response_library ir_lib()\n\n'
    result += 'private node traffic_engine {\n    alias output __out: engine;\n' + base
    result += '    cylinder_bank bank(bank_parameters, angle: 0 * units.deg)\n'
    cylinder = original[original.index('        .add_cylinder('):original.index('        .add_cylinder(', original.index('        .add_cylinder(') + 1)]
    for i in range(n):
        result += f'    ignition_wire wire{i}()\n    bank\n'
        result += cylinder.replace('rod_journal: first', f'rod_journal: j{i}').replace('wires.first', f'wire{i}')
    cams = original[original.index('    harmonic_cam_lobe intake_lobe('):original.index('    function advance(')]
    cams = cams[:cams.index('    intake_cam.add_lobe(')] + ''.join(
        f'    intake_cam.add_lobe({470 + angle} * units.deg)\n'
        f'    exhaust_cam.add_lobe({250 + angle} * units.deg)\n'
        for angle in range(0, 720, 720 // n)
    ) + cams[cams.index('    bank.set_cylinder_head('):]
    result += cams.replace('32.84 * units.cc', f'{chamber} * units.cc')
    result += f"""
    function advance(500 * units.rpm)
    advance.add_sample(0 * units.rpm, 12 * units.deg)
           .add_sample(1200 * units.rpm, 17 * units.deg)
           .add_sample(2200 * units.rpm, 23 * units.deg)
    ignition_module ignition(
        timing_curve: advance,
        rev_limit: {redline + 150} * units.rpm,
        limiter_duration: 0.06
    )
"""
    result += ''.join(
        f'    ignition.connect_wire(wire{i}, {angle} * units.deg)\n'
        for i, angle in enumerate(range(0, 720, 720 // n))
    )
    result += '    engine.add_ignition_module(ignition)\n}\n\n'
    result += original[original.index('private node road_transmission'):].replace('road_twin()', 'traffic_engine()')
    return result


def pcm(source, model, rpm, layer, start):
    period = 48_000 * 120 / rpm
    index = round(start * 48_000)
    assert index > 6 and index + 19 * period < len(source), "record 19 crank cycles"
    index = max(range(index, index + round(period)), key=lambda j: sum(source[j - 6:j + 7]))

    def read(position):
        j = int(position)
        return source[j] + (source[j + 1] - source[j]) * (position - j)

    data = []
    for frame in range(4096):
        position = index + frame * period / 256
        weight = 0.5 - 0.5 * math.cos(math.pi * min(1., frame / 512))
        value = read(position)
        if frame < 512:
            value = value * weight + read(position + 16 * period) * (1. - weight)
        data.append(value)
    mean = sum(data) / len(data)
    peak = max(abs(value - mean) for value in data)
    rms = math.sqrt(sum((value - mean) ** 2 for value in data) / len(data))
    assert peak > 600 and rms > 100, (model, rpm, layer, peak, rms)
    target = 6000 if layer == "pull" else 3500
    gain = min(target / rms, 27_500 / peak)
    output = array("h", (round((value - mean) * gain) for value in data))
    if sys.byteorder != "little":
        output.byteswap()
    CLIPS.mkdir(parents=True, exist_ok=True)
    (CLIPS / f"{model}-{rpm}-{layer}.pcm").write_bytes(output.tobytes())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("model", choices=MODELS)
    parser.add_argument("--capture", type=Path, help="48 kHz mono signed 16-bit LE raw dyno capture")
    parser.add_argument("--windows", nargs="+", metavar="RPM:LAYER:SECONDS")
    args = parser.parse_args()
    if args.capture is None:
        print(preset(args.model), end="")
        return
    source = array("h", args.capture.read_bytes())
    if sys.byteorder != "little":
        source.byteswap()
    for entry in args.windows or parser.error("--capture requires --windows"):
        rpm, layer, seconds = entry.split(":")
        if layer not in ("pull", "coast"):
            parser.error(f"invalid layer {layer}")
        pcm(source, args.model, int(rpm), layer, float(seconds))


if __name__ == "__main__":
    main()
