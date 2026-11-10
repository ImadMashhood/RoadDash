#!/usr/bin/env python3
"""Loopback-only integration fixture: ordered framed TCP with one-way delay.

Not a production relay, packet-loss emulator, or throughput benchmark. Never logs
payloads/capability codes. Delay is scheduled per frame, so a stream of frames
does not incorrectly accumulate another full delay at every read.
"""
import argparse
import asyncio
import json
from pathlib import Path

FRAME_LIMIT = 512 * 1024


async def run(args):
    stats = dict(connections=0, frames=0, bytes=0, max_queue=0,
                 one_way_ms=args.delay_ms, errors=[])
    clients = set()

    async def pipe(reader, writer):
        queue = asyncio.Queue(maxsize=16)

        async def receive():
            try:
                while True:
                    prefix = await reader.readexactly(4)
                    size = int.from_bytes(prefix, 'big') & 0x7fffffff
                    if not 0 < size <= FRAME_LIMIT:
                        raise ValueError('invalid framed packet size')
                    payload = await reader.readexactly(size)
                    await queue.put((asyncio.get_running_loop().time() + args.delay_ms / 1000,
                                     prefix + payload))
                    stats['max_queue'] = max(stats['max_queue'], queue.qsize())
            except asyncio.IncompleteReadError:
                pass
            await queue.put(None)

        async def transmit():
            while (item := await queue.get()) is not None:
                deadline, data = item
                await asyncio.sleep(max(0, deadline - asyncio.get_running_loop().time()))
                writer.write(data)
                await writer.drain()
                stats['frames'] += 1
                stats['bytes'] += len(data)

        tasks = [asyncio.create_task(receive()), asyncio.create_task(transmit())]
        try:
            await asyncio.gather(*tasks)
        finally:
            for task in tasks:
                task.cancel()
            await asyncio.gather(*tasks, return_exceptions=True)

    async def accept(reader, writer):
        task = asyncio.current_task()
        clients.add(task)
        upstream = None
        pipes = []
        try:
            remote_reader, upstream = await asyncio.open_connection('127.0.0.1', args.target)
            stats['connections'] += 1
            pipes = [asyncio.create_task(pipe(reader, upstream)),
                     asyncio.create_task(pipe(remote_reader, writer))]
            done, _ = await asyncio.wait(pipes, return_when=asyncio.FIRST_COMPLETED)
            for ended in done:
                ended.result()
        except (ConnectionError, OSError, ValueError) as error:
            stats['errors'].append(type(error).__name__)
        finally:
            for pending in pipes:
                pending.cancel()
            await asyncio.gather(*pipes, return_exceptions=True)
            writer.close()
            if upstream:
                upstream.close()
            clients.discard(task)

    server = await asyncio.start_server(accept, '127.0.0.1', args.listen)
    print(f'Latency fixture ready on127.0.0.1:{args.listen}', flush=True)
    try:
        async with server:
            await asyncio.sleep(args.seconds)
    finally:
        pending = list(clients)
        for task in pending:
            task.cancel()
        await asyncio.gather(*pending, return_exceptions=True)
        Path(args.output).write_text(json.dumps(stats, indent=2) + '\n')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--listen', type=int, required=True)
    parser.add_argument('--target', type=int, required=True)
    parser.add_argument('--delay-ms', type=float, default=75)
    parser.add_argument('--seconds', type=float, default=60)
    parser.add_argument('--output', required=True)
    args = parser.parse_args()
    if not (1 <= args.listen <= 65535 and 1 <= args.target <= 65535
            and 0 <= args.delay_ms <= 1000 and 0 < args.seconds <= 300):
        parser.error('fixture ports, delay or duration out of bounds')
    asyncio.run(run(args))
