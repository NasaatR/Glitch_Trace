"""Check the actual Windows DLL exports, architecture and PiPL without loading it."""
import struct
import sys
from pathlib import Path

def verify(path):
    b = Path(path).read_bytes()
    def u16(p): return struct.unpack_from('<H', b, p)[0]
    def u32(p): return struct.unpack_from('<I', b, p)[0]
    assert b[:2] == b'MZ', 'Missing MZ header'
    pe = u32(0x3c)
    assert b[pe:pe+4] == b'PE\0\0', 'Missing PE signature'
    assert u16(pe+4) == 0x8664, 'Not x64'
    assert u16(pe+22) & 0x2000, 'Not a DLL'
    opt = pe+24
    assert u16(opt) == 0x20b, 'Not PE32+'
    sections = []
    for i in range(u16(pe+6)):
        p = opt+u16(pe+20)+i*40
        size, va, raw_size, raw = struct.unpack_from('<IIII', b, p+8)
        sections.append((va, max(size, raw_size), raw))
    def off(rva):
        for va, size, raw in sections:
            if va <= rva < va+size: return raw+rva-va
        raise AssertionError(f'RVA outside sections: {rva:x}')
    export = off(u32(opt+112))
    count = u32(export+24)
    names = off(u32(export+32))
    exported = []
    for i in range(count):
        start = off(u32(names+4*i))
        exported.append(b[start:b.index(b'\0', start)].decode('ascii'))
    assert 'EffectMain' in exported, exported
    assert 'PluginDataEntryFunction2' in exported, exported
    base = off(u32(opt+112+16))
    payloads = []
    def walk(rel, path):
        p = base+rel
        count = u16(p+12)+u16(p+14)
        for i in range(count):
            name, child = struct.unpack_from('<II', b, p+16+i*8)
            if name & 0x80000000:
                q = base+(name & 0x7fffffff)
                name = b[q+2:q+2+2*u16(q)].decode('utf-16le')
            if child & 0x80000000: walk(child & 0x7fffffff, path+[name])
            else:
                rva, size = struct.unpack_from('<II', b, base+child)
                payloads.append((path+[name], b[off(rva):off(rva)+size]))
    walk(0, [])
    pipl = [data for path, data in payloads if 'PiPL' in path or 'PIPL' in path]
    assert pipl, 'Missing PiPL resource'
    assert any(b'Nasaat GlitchTrace' in p for p in pipl), 'Wrong match name'
    assert any(b'GlitchTrace' in p for p in pipl), 'Wrong effect name'
    print(f'PASS: x64 DLL, EffectMain, registration entrypoint, GlitchTrace PiPL ({len(b):,} bytes)')

if __name__ == '__main__': verify(sys.argv[1])
