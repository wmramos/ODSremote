import hashlib
from pathlib import Path
import struct
import tempfile
import unittest

from stage_release import stage_release


class StageReleaseTests(unittest.TestCase):
    def fixtures(self, root):
        for edition, suffix in [('qs', 'qs.exe'), ('agent', 'agent-install.exe')]:
            folder = root / f'ODSremote-1.5.0-windows-x64-{edition}-unsigned'
            folder.mkdir()
            data = bytearray(256)
            data[:2] = b'MZ'
            struct.pack_into('<I', data, 60, 128)
            data[128:132] = b'PE\0\0'
            struct.pack_into('<H', data, 132, 0x8664)
            name = f'ODSremote-1.5.0-windows-x64-{suffix}'
            (folder / name).write_bytes(data)
            (folder / 'SHA256SUMS.txt').write_text(f'{hashlib.sha256(data).hexdigest()}  {name}\n')
            (folder / 'LICENCE').write_text('Test license fixture')
            (folder / 'ODSREMOTE.md').write_text('Test notices fixture')

    def test_stages_both_versioned_files_with_matching_checksums(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            self.fixtures(root)
            stage_release(root, root / 'release', '1.5.0')
            self.assertEqual(len(list((root / 'release').glob('*.exe'))), 2)
            self.assertEqual(len((root / 'release/SHA256SUMS.txt').read_text().splitlines()), 2)
            self.assertTrue((root / 'release/LICENCE').is_file())

    def test_checksum_failure_prevents_staging(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            self.fixtures(root)
            folder = root / 'ODSremote-1.5.0-windows-x64-agent-unsigned'
            (folder / 'SHA256SUMS.txt').write_text('invalid checksum')
            with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
                stage_release(root, root / 'release', '1.5.0')
            self.assertFalse((root / 'release').exists())

    def test_missing_edition_prevents_staging(self):
        with tempfile.TemporaryDirectory() as root:
            root = Path(root)
            self.fixtures(root)
            binary = root / 'ODSremote-1.5.0-windows-x64-agent-unsigned/ODSremote-1.5.0-windows-x64-agent-install.exe'
            binary.unlink()
            with self.assertRaises(FileNotFoundError):
                stage_release(root, root / 'release', '1.5.0')
            self.assertFalse((root / 'release').exists())


if __name__ == '__main__':
    unittest.main()
