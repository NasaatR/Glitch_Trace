"""Meaningful archive integrity/recovery checks, no network or Git required."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
import zipfile

SCRIPT = Path(__file__).resolve().parents[1]/'tools'/'Pack-Return.py'

class HandoffTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.work = self.root/'workspace'
        self.project = self.work/'project'
        (self.project/'tools').mkdir(parents=True)
        shutil.copy2(SCRIPT, self.project/'tools'/'Pack-Return.py')
        (self.work/'handoff').mkdir()
        (self.work/'reference').mkdir()
        (self.work/'reference'/'MyTracePlugin.aex').write_bytes(b'reference-fixture')
        (self.project/'OFFLINE_SESSION.json').write_text(json.dumps({'github_base_sha':'test-base'}))
        (self.project/'changed.rs').write_text('new')
        (self.project/'added.rs').write_text('added')
        (self.project/'.env').write_text('SECRET=excluded')
        (self.project/'target').mkdir()
        (self.project/'target'/'cache').write_text('exclude')
        (self.project/'vendor'/'fixture').mkdir(parents=True)
        (self.project/'vendor'/'fixture'/'public-test.pem').write_text('public vendor fixture')
        # Registry source archives can legitimately carry Unix-epoch mtimes.
        os.utime(self.project/'vendor'/'fixture'/'public-test.pem', (0,0))
        baseline = {'archive_start_sha256':{'changed.rs':hashlib.sha256(b'old').hexdigest(),'deleted.rs':'old'},'vendor_start_sha256':{}}
        (self.work/'handoff'/'source_baseline.json').write_text(json.dumps(baseline))
    def tearDown(self): self.temp.cleanup()
    def run_pack(self,*args):
        return subprocess.run([sys.executable,str(self.project/'tools'/'Pack-Return.py'),*map(str,args)],capture_output=True,text=True)
    def test_no_export_without_request(self):
        target=self.root/'not-created.zip'
        result=self.run_pack('--output',target)
        self.assertNotEqual(result.returncode,0);self.assertFalse(target.exists())
    def test_complete_source_change_delete_and_exclusions(self):
        target=self.root/'return.zip'
        result=self.run_pack('--requested-by-user','--output',target)
        self.assertEqual(result.returncode,0,result.stderr)
        with zipfile.ZipFile(target) as z:
            names=z.namelist();self.assertEqual(len(names),len(set(names)))
            report=json.loads(z.read('GlitchTrace_Offline_Workspace/handoff/RETURN_REPORT.json'))
            self.assertIn('changed.rs',report['modified_source']);self.assertIn('added.rs',report['added_source'])
            self.assertIn('deleted.rs',report['deleted_source'])
            self.assertTrue(report['vendor_changed_paths'])
            self.assertIn('GlitchTrace_Offline_Workspace/reference/MyTracePlugin.aex',names)
            self.assertTrue(any(n.endswith('public-test.pem') for n in names))
            self.assertFalse(any(n.endswith('.env') or '/target/' in n for n in names))
            manifest=json.loads(z.read('GlitchTrace_Offline_Workspace/handoff/RETURN_MANIFEST.json'))
            for entry in manifest['files']:
                data=z.read('GlitchTrace_Offline_Workspace/'+entry['path'])
                self.assertEqual(hashlib.sha256(data).hexdigest(),entry['sha256'])
        self.assertNotEqual(self.run_pack('--requested-by-user','--output',target).returncode,0)
    def test_output_cannot_recurse_inside_workspace(self):
        result=self.run_pack('--requested-by-user','--output',self.work/'bad.zip')
        self.assertNotEqual(result.returncode,0)
        self.assertFalse((self.work/'bad.zip').exists())

if __name__=='__main__': unittest.main()
