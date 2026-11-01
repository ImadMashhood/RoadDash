#!/usr/bin/env python3
"""Execute bounded retail x86 crash blocks against a Rust-generated report.

Optional research tool: pip install unicorn==2.1.4 in a disposable environment.
Requires the user's own exact supported PE32; exports no binary/media payloads.
This is isolated arithmetic verification, not running or certifying the whole game.
"""
import argparse
import hashlib
import json
import struct
from pathlib import Path
from read_animation_context import pe_data, Bytes

SHA256 = '73d7f9744b5b1a9482b3909950cdf3fe34119fe13d023b031e9c6739a60f18a2'

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('exe', type=Path)
    parser.add_argument('report', type=Path)
    args = parser.parse_args()
    raw = args.exe.read_bytes()
    if hashlib.sha256(raw).hexdigest() != SHA256:
        parser.error('Unsupported executable SHA256; no address guessing allowed')
    from unicorn import Uc, UC_ARCH_X86, UC_MODE_32, UC_HOOK_CODE, __version__
    if __version__ != "2.1.4":
        parser.error("Use pinned unicorn==2.1.4 for this verification receipt")
    from unicorn.x86_const import UC_X86_REG_EAX, UC_X86_REG_EBX, UC_X86_REG_ECX, UC_X86_REG_EDX, UC_X86_REG_ESI, UC_X86_REG_ESP, UC_X86_REG_EIP
    b = Bytes(raw)
    pe = b.u32(60)
    base = b.u32(pe+52)
    uc = Uc(UC_ARCH_X86, UC_MODE_32)
    uc.mem_map(base, 0x100000)
    table = pe+24+b.u16(pe+20)
    for p in range(table, table+b.u16(pe+6)*40, 40):
        rva, size, start = b.u32(p+12), b.u32(p+16), b.u32(p+20)
        uc.mem_write(base+rva, b.take(start, size))
    uc.mem_map(0x1000000, 0x20000)
    stack, actor, other, sentinel = 0x1010000, 0x1001000, 0x1002000, 0x1003000
    def put(address, value):
        uc.mem_write(address, struct.pack('<I', value & 0xffffffff))
    def get(address):
        return struct.unpack('<i', uc.mem_read(address, 4))[0]
    def reg(r, value):
        uc.reg_write(r, value & 0xffffffff)
    def reset():
        uc.mem_write(0x1000000, bytes(0x20000))
        reg(UC_X86_REG_ESP, stack)
    report = json.loads(args.report.read_text())['native_verification']
    checks = 0
    # Native reason multipliers: no function calls in either bounded range.
    for case in report['scale_cases']:
        reset()
        reg(UC_X86_REG_EBX, case['value'])
        put(stack+0x30, case['value'])
        uc.emu_start(0x4052f1 if case['reason'] == 2 else 0x40531d, 0x40533b, count=100)
        assert uc.reg_read(UC_X86_REG_EBX) == case['scaled'] & 0xffffffff, case
        checks += 1
    # Default/punch parameter selection; stop BEFORE begin_crash's call.
    for reason, key in [(1, 'damage_pair'), (4, 'punch_pair')]:
        reset()
        reg(UC_X86_REG_EDX, 0)
        # edi selects reason, and esi supplies the bike pointer.
        from unicorn.x86_const import UC_X86_REG_EDI
        reg(UC_X86_REG_EDI, reason)
        reg(UC_X86_REG_ESI, actor)
        uc.emu_start(0x40535f, 0x4053b2, count=100)
        rider_pointer = uc.reg_read(UC_X86_REG_EDX)
        bike_pointer = get(uc.reg_read(UC_X86_REG_ESP))
        for pointer, expected in zip([rider_pointer, bike_pointer], report[key]):
            assert [get(pointer+i*4) for i in range(3)] == [expected[k] for k in ['forward','lateral','vertical']]
        checks += 1
    # Large contact gap pair, stopping before call. EAX is zero on this path.
    reset()
    reg(UC_X86_REG_EAX,0)
    uc.emu_start(0x404ee7,0x404f13,count=100)
    pointers = [uc.reg_read(UC_X86_REG_EDX),get(uc.reg_read(UC_X86_REG_ESP))]
    for pointer,expected in zip(pointers,report['speed_gap_pair']):
        assert [get(pointer+i*4) for i in range(3)] == [expected[k] for k in ['forward','lateral','vertical']]
    checks += 1
    # Signed speed gate is an isolated subtraction/CMP/JLE. Probe both sides.
    for own, target, high in [(12000,6000,False),(12000,5999,True),(7450,-2483,True)]:
        reset()
        reg(UC_X86_REG_EBX,actor);reg(UC_X86_REG_ESI,other)
        put(actor+0xec,own);put(other+0x108,target)
        uc.emu_start(0x404ed1,0x404ee7,count=5)
        assert (uc.reg_read(UC_X86_REG_EIP) == 0x404ee7) == high
        checks += 1
    # Full scalar drag function; only stub the unrelated surface classifier to
    # choose surface zero. All arithmetic, stores and returns execute retail x86.
    def surface(uc, address, size, _):
        if address == 0x43a0c0:
            reg(UC_X86_REG_EAX,0)
            sp = uc.reg_read(UC_X86_REG_ESP)
            reg(UC_X86_REG_EIP,get(sp));reg(UC_X86_REG_ESP,sp+4)
    hook = uc.hook_add(UC_HOOK_CODE, surface, begin=0x43a0c0,end=0x43a0c0)
    # Set surface zero's coefficient to the test's explicit zero input.
    uc.mem_write(0x46dce4,b'\x00')
    for case in report['drag_cases']:
        reset()
        reg(UC_X86_REG_ECX,actor);put(stack,sentinel)
        for offset,value in [(0xec,case['speed']),(0x18,1),(0x160,int(case['grounded'])),(0x150,130),(0x154,70)]:
            put(actor+offset,value)
        uc.emu_start(0x4390c0,sentinel,count=500)
        assert get(actor+0xec) == case['next'],case
        assert get(0x4c3cec) == case['acceleration'],case
        checks += 1
    uc.hook_del(hook)
    # Numeric object fields verified directly against the same hashed PE.
    expected = {9:130,10:70,11:37,12:-2560,13:256,14:74,15:0,16:24,17:6400,18:16,19:7680,20:192,21:25}
    words = struct.unpack('<26i',pe_data(raw,0x463c10,104))
    assert all(words[index] == value for index,value in expected.items())
    checks += 1
    print(json.dumps({'passed':checks,'exe_sha256':SHA256,'engine':'Unicorn 2.1.4',
        'scope':'bounded retail x86 arithmetic and data checks; not full-game parity'},indent=2))

if __name__ == '__main__':
    main()
