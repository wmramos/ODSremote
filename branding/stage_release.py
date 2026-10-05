#!/usr/bin/env python3
import hashlib
import os
from pathlib import Path
import re
import shutil
import struct
import sys


def stage_release(incoming, destination, version):
    if not re.fullmatch(r'\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?', version):
        raise ValueError('Invalid upstream version')
    verified = []
    for edition, suffix in [('qs', 'qs.exe'), ('agent', 'agent-install.exe')]:
        folder = Path(incoming) / f'ODSremote-{version}-windows-x64-{edition}-unsigned'
        name = f'ODSremote-{version}-windows-x64-{suffix}'
        binary = folder / name
        data = binary.read_bytes()
        if len(data) < 64 or data[:2] != b'MZ':
            raise ValueError(f'{name}: not a Windows executable')
        pe = struct.unpack_from('<I', data, 60)[0]
        if pe + 6 > len(data) or data[pe:pe + 4] != b'PE\0\0' or struct.unpack_from('<H', data, pe + 4)[0] != 0x8664:
            raise ValueError(f'{name}: not a Windows x64 executable')
        checksum = hashlib.sha256(data).hexdigest()
        expected = (folder / 'SHA256SUMS.txt').read_text(encoding='utf-8-sig').strip()
        if expected != f'{checksum}  {name}':
            raise ValueError(f'{name}: checksum mismatch')
        for notice in ['LICENCE', 'ODSREMOTE.md']:
            if not (folder / notice).is_file():
                raise ValueError(f'{name}: missing {notice}')
        verified.append((binary, checksum))
    output = Path(destination)
    output.mkdir(parents=True, exist_ok=True)
    for binary, _ in verified:
        shutil.copy2(binary, output / binary.name)
    notices = verified[0][0].parent
    for notice in ['LICENCE', 'ODSREMOTE.md']:
        for binary, _ in verified[1:]:
            if (binary.parent / notice).read_bytes() != (notices / notice).read_bytes():
                raise ValueError(f'Editions contain different {notice}')
        shutil.copy2(notices / notice, output / notice)
    (output / 'SHA256SUMS.txt').write_text(''.join(f'{checksum}  {binary.name}\n' for binary, checksum in verified), encoding='utf-8')


if __name__ == '__main__':
    stage_release(sys.argv[1], sys.argv[2], os.environ['ODSREMOTE_VERSION'])
