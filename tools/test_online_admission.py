#!/usr/bin/env python3
"""Exercise real listen authority with bounded raw protocol peers on loopback only."""
import argparse, hashlib, json, os, select, socket, struct, subprocess, time, zlib
from pathlib import Path

class Peer:
    def __init__(self, endpoint):
        self.socket = socket.create_connection(endpoint, timeout=2)
        self.socket.setblocking(False)
        self.buffer = bytearray()
        self.closed = False
        self.sequence = 0
    def send(self, message):
        data = json.dumps(message, allow_nan=False).encode()
        self.socket.sendall(struct.pack('!I', len(data)) + data)
    def poll(self):
        messages = []
        while not self.closed:
            try:
                data = self.socket.recv(65536)
                if not data:
                    self.closed = True
                    break
                self.buffer.extend(data)
            except ConnectionResetError:
                self.closed = True
                break
            except BlockingIOError:
                break
        while len(self.buffer) >= 4:
            prefix, = struct.unpack('!I', self.buffer[:4])
            size = prefix & 0x7fffffff
            assert 0 < size <= 524288
            if len(self.buffer) < size + 4:
                break
            data = bytes(self.buffer[4:4+size]); del self.buffer[:4+size]
            if prefix & 0x80000000:
                decoder = zlib.decompressobj(-15)
                data = decoder.decompress(data, 524289)
                assert len(data) <= 524288 and decoder.eof
            messages.append(json.loads(data))
        return messages
    def input(self, steer=0):
        self.sequence += 1
        self.send({'Input': {'sequence': self.sequence, 'vehicle': {'throttle': True,
            'brake': False, 'steer': steer, 'nitro': False, 'throttle_amount': 1.,
            'brake_amount': 0.}, 'attack': None, 'foot': [0.,0.], 'foot_forward': [0.,1.],
            'interact': False, 'pickup': False, 'drop': False}})
    def close(self):
        self.socket.close()

def main(args):
    output = Path(args.output).resolve(); output.mkdir(parents=True, exist_ok=False)
    for folder in ('tmp','performance'): (output/folder).mkdir()
    endpoint = ('127.0.0.1', args.port)
    env = dict(os.environ, ROADDASH_DHT='0', ROADDASH_UPNP='0',
        ROADDASH_ADVERTISE_ADDR=f'{endpoint[0]}:{endpoint[1]}', TMPDIR=str(output/'tmp'),
        ROADDASH_LOG_DIR=str(output/'performance'))
    log = (output/'host.log').open('w')
    process = subprocess.Popen([args.binary, '--online-test-node', 'host',
        f'{endpoint[0]}:{endpoint[1]}', str(output/'host.json')], env=env, stdout=log, stderr=log)
    peers = []; events = []; started = time.monotonic(); observer = None
    try:
        code = None
        while time.monotonic()-started < 20:
            lines = (output/'host.log').read_text(errors='replace').splitlines()
            codes = [line.split('=',1)[1] for line in lines if line.startswith('HOST_CODE=')]
            if codes: code = codes[0]; break
            assert process.poll() is None, 'Host stopped before readiness'
            time.sleep(.05)
        assert code and len(code)==36
        def connect(secret=code[10:], protocol=2):
            peer = Peer(endpoint); peers.append(peer)
            peer.send({'Hello': {'protocol':protocol, 'name':'Admission fixture', 'secret':secret}})
            return peer
        for name, secret, protocol in [('wrong-secret','A'*26,2),('wrong-protocol',code[10:],999)]:
            peer = connect(secret, protocol); replies=[]; deadline=time.monotonic()+2
            while not peer.closed and time.monotonic()<deadline:
                replies += peer.poll(); time.sleep(.01)
            assert peer.closed and not any('Welcome' in m for m in replies), name
            events.append(name+' rejected'); peer.close()
        peer=Peer(endpoint); peers.append(peer); peer.send({'Probe':{'protocol':2,'nonce':42}})
        replies=[]; deadline=time.monotonic()+2
        while not peer.closed and time.monotonic()<deadline:
            replies += peer.poll(); time.sleep(.01)
        assert any(m.get('Offer',{}).get('available') is False for m in replies), replies
        events.append('private probe unavailable'); peer.close()
        observer=connect(); latest=None; roster=None; first=None; late=None; late_slot=None
        late_before=None; late_after=None; dropped=False; bot_seen=False; next_input=0
        deadline=time.monotonic()+32
        while time.monotonic()<deadline:
            now=time.monotonic()
            if now>=next_input:
                observer.input()
                if late and not dropped: late.input()
                next_input=now+.05
            for message in observer.poll():
                if 'Snapshot' in message:
                    latest=message['Snapshot']
                    if first is None: first=now-started
                if 'Lobby' in message: roster=message['Lobby']['lobby']
                if 'Welcome' in message: assert message['Welcome']['slot']==1
            if latest and latest['elapsed']>=3 and late is None:
                late_before=next(r for r in latest['racers'] if r['slot']==2); late=connect()
            if late and not dropped:
                for message in late.poll():
                    if 'Welcome' in message: late_slot=message['Welcome']['slot']; assert late_slot==2
                    if 'Snapshot' in message and late_after is None:
                        late_after=next(r for r in message['Snapshot']['racers'] if r['slot']==2)
                if late_after and latest['elapsed']>=6:
                    assert abs(late_after['progress']-late_before['progress'])<100
                    before_pos=late_before['pose']['p']; after_pos=late_after['pose']['p']
                    assert sum((a-b)**2 for a,b in zip(before_pos,after_pos)) < 400, (late_before,late_after)
                    late.close(); dropped=True; events.append('late join preserves moving bot state')
            if dropped and roster and roster['players'][2]['bot']:
                bot_seen=True; events.append('disconnect restored bot roster'); break
            time.sleep(.005)
        assert first and first>=14.5, first
        assert late_slot==2 and bot_seen
        malicious=connect(); welcomed=False; deadline=time.monotonic()+2
        while time.monotonic()<deadline and not welcomed:
            observer.input()
            welcomed=any('Welcome' in m for m in malicious.poll())
            observer.poll(); time.sleep(.02)
        assert welcomed
        malicious.input(steer=2.)
        deadline=time.monotonic()+2
        while not malicious.closed and time.monotonic()<deadline:
            malicious.poll(); observer.input(); observer.poll(); time.sleep(.02)
        assert malicious.closed, 'Invalid input accepted'
        events.append('out-of-range input disconnected')
        filling=[connect() for _ in range(13)]; slots=set(); deadline=time.monotonic()+3
        while len(slots)<13 and time.monotonic()<deadline:
            observer.input(); observer.poll()
            for peer in filling:
                for message in peer.poll():
                    if 'Welcome' in message: slots.add(message['Welcome']['slot'])
            time.sleep(.01)
        assert slots==set(range(2,15)), slots
        overflow=connect(); replies=[]; deadline=time.monotonic()+2
        while not overflow.closed and time.monotonic()<deadline:
            replies += overflow.poll()
            observer.input(); observer.poll()
            for peer in filling: peer.poll()
            time.sleep(.01)
        assert overflow.closed and not any('Welcome' in m for m in replies)
        events.append('all14 guest slots unique and full host rejects overflow')
        process.terminate(); process.wait(timeout=5)
        deadline=time.monotonic()+2
        while not observer.closed and time.monotonic()<deadline:
            observer.poll(); time.sleep(.01)
        assert observer.closed, 'Host disconnect left transport alive'
        events.append('host termination closes remaining guest')
        receipt={'passed':True,'events':events,'countdown_to_first_snapshot_seconds':first,
            'late_slot':late_slot,'bot_progress_before_takeover':late_before['progress'],
            'human_progress_after_takeover':late_after['progress'],
            'bot_pose_before_takeover':late_before['pose'], 'human_pose_after_takeover':late_after['pose'],
            'binary_sha256':hashlib.sha256(Path(args.binary).read_bytes()).hexdigest(),
            'limitations':'Raw peers exercise actual authority; native rendering/recovery is separate.'}
        (output/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n'); print(json.dumps(receipt))
    finally:
        for peer in peers: peer.close()
        if process.poll() is None: process.terminate(); process.wait(timeout=5)
        log.close()

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary',required=True); parser.add_argument('--output',required=True)
    parser.add_argument('--port',type=int,default=47673)
    main(parser.parse_args())
