#!/usr/bin/env python3
"""Run actual host/two-client graybox gameplay with a150ms ordered TCP RTT.

Uses --online-test-node from the game, with public discovery/router mapping off.
This is an application-delay fixture, not Internet/NAT or packet-loss evidence.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import time


def free_port():
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


def endpoint_code(port):
    # The game's base32 endpoint encodes48bits into10digits with two leading0bits.
    number = int.from_bytes(bytes([127, 0, 0, 1]) + port.to_bytes(2, 'big'), 'big')
    alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567'
    return ''.join(alphabet[(number >> shift) & 31] for shift in range(45, -1, -5))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('binary', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=True)
    if any((args.output / name).exists() for name in ['host.json', 'client1.json', 'client2.json', 'receipt.json']):
        raise RuntimeError('Use a fresh output directory; old receipts cannot validate a new run')
    binary = args.binary.resolve()
    temp = args.output.resolve() / 'tmp'
    temp.mkdir(exist_ok=True)
    env = dict(os.environ, ROADDASH_DHT='0', ROADDASH_UPNP='0',
               TMPDIR=str(temp), ROADDASH_LOG_DIR=str(args.output.resolve() / 'performance'))
    for key in ['ROADDASH_ONLINE_BOOT', 'ROADDASH_HOST_SECRET', 'ROADDASH_TEST_RELOAD_MAP']:
        env.pop(key, None)
    processes, files = [], []

    def launch(label, command):
        stream = (args.output / f'{label}.log').open('w')
        files.append(stream)
        process = subprocess.Popen(command, env=env, stdout=stream, stderr=subprocess.STDOUT)
        processes.append(process)
        return process

    def await_text(label, prefix):
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline:
            for line in (args.output / f'{label}.log').read_text(errors='replace').splitlines():
                if line.startswith(prefix):
                    return line[len(prefix):]
            time.sleep(.05)
        raise RuntimeError(f'{label} did not become ready')

    try:
        host_port, proxy_port = free_port(), free_port()
        while proxy_port == host_port:
            proxy_port = free_port()
        host = launch('host', [str(binary), '--online-test-node', 'host',
                              f'127.0.0.1:{host_port}', str(args.output / 'host.json')])
        code = await_text('host', 'HOST_CODE=')
        proxy = launch('proxy', [sys.executable, str(Path(__file__).with_name('online_latency_proxy.py')),
                                '--listen', str(proxy_port), '--target', str(host_port),
                                '--delay-ms', '75', '--seconds', '55',
                                '--output', str(args.output / 'proxy.json')])
        await_text('proxy', 'Latency fixture ready')
        code = endpoint_code(proxy_port) + code[10:]
        guests = [launch(name, [str(binary), '--online-test-node', 'client', code,
                               str(args.output / f'{name}.json')]) for name in ['client1', 'client2']]
        for process in [host, *guests, proxy]:
            if process.wait(timeout=60) != 0:
                raise RuntimeError('fixture process failed; inspect logs')
        reports = {name: json.loads((args.output / f'{name}.json').read_text())
                   for name in ['host', 'client1', 'client2']}
        host_samples = {row[0]: row[1] for row in reports['host']['samples']}
        matches = {}
        for name, report in reports.items():
            assert report['max_players'] == 3, (name, report['max_players'])
            assert not report['errors'], (name, report['errors'])
            for key in ['remote_attack', 'remote_crash', 'remote_foot', 'remote_remount']:
                assert report[key], (name, key)
            compared = [(tick, digest) for tick, digest, _ in report['samples'] if tick in host_samples]
            assert len(compared) >= 250, (name, len(compared))
            assert all(host_samples[tick] == digest for tick, digest in compared), name
            matches[name] = len(compared)
        receipt = dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                       delay_ms_per_direction=75, matched_snapshot_ticks=matches,
                       result='pass', limitation='Loopback application delay; no packet loss, Internet or NAT claim')
        (args.output / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
        print(json.dumps(receipt, indent=2))
    finally:
        for process in processes:
            if process.poll() is None:
                process.terminate()
        for process in processes:
            try:
                process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
        for stream in files:
            stream.close()


if __name__ == '__main__':
    main()
