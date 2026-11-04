"""Read-only reference inventory. Emits metadata/labels, never code or license data.
Usage: python tools/analyze_reference.py /path/to/reference.aex > inventory.json
The reference itself must remain outside this repository.
"""
import datetime
import hashlib
import json
import re
import struct
import sys
from pathlib import Path
b = Path(sys.argv[1]).read_bytes()
pe = struct.unpack_from('<I', b, 0x3c)[0]
assert b[:2] == b'MZ' and b[pe:pe+4] == b'PE\0\0'
start = b.find(b'Key Color\0')
end = b.find(b"Couldn't load suite.\0", start)
assert start >= 0 and end > start, 'Expected reference UI strings were not found'
labels = [m.group().decode('ascii') for m in re.finditer(rb'[ -~]{3,}', b[start:end])]
uniforms = sorted(set(m.group(1).decode('ascii') for m in re.finditer(rb'uniform\s+(?:sampler2D|float|int|bool|vec4|mat4)\s+([A-Z_a-z0-9]+)\s*;', b)))
print(json.dumps({
    'sha256': hashlib.sha256(b).hexdigest(),
    'bytes': len(b),
    'machine': hex(struct.unpack_from('<H', b, pe+4)[0]),
    'pe_timestamp_unverified_utc': datetime.datetime.fromtimestamp(struct.unpack_from('<I', b, pe+8)[0], datetime.timezone.utc).isoformat(),
    'identity_markers': [x for x in ['DRAGOY Tracery','Tracery','Dragoy','GLator.aex'] if x.encode() in b],
    'controls_and_option_strings': labels,
    'shader_uniform_names': uniforms,
    'shader_box_capacity': 512 if re.search(rb'bboxes\[512\]', b) else None,
    'background_switch_occurrences': b.count(b'SHOW_TEXT_BACKGROUND'),
    'limitations': ['Static binary inventory, not a host run or complete decompilation.', 'Labels/uniforms do not establish defaults, parameter IDs or complete CPU algorithms.']
}, indent=2))
