"""Pack an offline workspace only on explicit user request. Python 3.11+, no Git.
The archive baseline and this report aid review; hashes are not signatures.
"""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import zipfile

EXCLUDED_DIRS = {'.git', 'target', '__pycache__', 'node_modules', '.venv', 'venv', 'toolchain'}

def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''): h.update(chunk)
    return h.hexdigest()

def excluded(path):
    if path.parts[:2] == ('project', 'vendor'): return False  # Locked public dependency source, including fixtures.
    if path.as_posix() in {'handoff/RETURN_REPORT.json', 'handoff/RETURN_MANIFEST.json'}: return True
    if any(p in EXCLUDED_DIRS for p in path.parts): return True
    if 'dist' in path.parts: return True  # Put selected user builds in deliverables.
    name = path.name.lower()
    return name in {'.ds_store', 'thumbs.db', 'credentials', 'credentials.toml'} or name.startswith('.env') or name.endswith(('.pyc', '.pem', '.key', '.pfx', '.p12'))

def inventory(workspace):
    included, skipped = [], []
    for p in sorted(workspace.rglob('*')):
        relative = p.relative_to(workspace)
        if p.is_symlink():
            if not excluded(relative): raise ValueError('Symlink needs manual review: '+str(relative))
            continue
        if not p.is_file(): continue
        if excluded(relative): skipped.append(relative.as_posix()); continue
        included.append((p, relative.as_posix()))
    return included, skipped

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--requested-by-user', action='store_true', help='Use only after the user explicitly asks for a return archive.')
    parser.add_argument('--output', required=True, type=Path)
    args = parser.parse_args()
    if not args.requested_by_user: parser.error('No archive created. Wait for an explicit user return/handoff request.')
    project = Path(__file__).resolve().parents[1]
    marker = project/'OFFLINE_SESSION.json'
    if not marker.is_file(): parser.error('No offline session marker; do not guess a workspace.')
    session = json.loads(marker.read_text(encoding='utf-8'))
    workspace = project.parent
    output = args.output.resolve()
    if output.is_relative_to(workspace): parser.error('Output must be outside the workspace to avoid recursive archives.')
    if output.exists(): parser.error('Output exists; choose a new filename instead of overwriting a delivered archive.')
    base = json.loads((workspace/'handoff'/'source_baseline.json').read_text(encoding='utf-8'))
    included, skipped = inventory(workspace)
    # Compare only source; vendor is preserved but never treated as GitHub edits.
    current = {n[len('project/'):]: digest(p) for p, n in included if n.startswith('project/') and not n.startswith('project/vendor/') and n != 'project/OFFLINE_SESSION.json'}
    initial = base['archive_start_sha256']
    old_vendor = base.get('vendor_start_sha256', {})
    current_vendor = {n[len('project/'):]: digest(p) for p, n in included if n.startswith('project/vendor/')}
    vendor_changed = sorted(n for n in set(old_vendor)|set(current_vendor) if old_vendor.get(n) != current_vendor.get(n))
    changed = sorted(n for n in current if n in initial and current[n] != initial[n])
    added = sorted(set(current)-set(initial))
    deleted = sorted(set(initial)-set(current))
    now = datetime.datetime.now(datetime.timezone.utc).isoformat()
    report = {'schema_version': 1, 'created_at_utc': now, 'originating_git_base': session['github_base_sha'],
              'vendor_changed_paths': vendor_changed, 'modified_source': changed, 'added_source': added, 'deleted_source': deleted,
              'excluded_files': skipped, 'host_validation': 'Read PROJECT_STATE, HANDOFF_LOG and supplied evidence; not inferred by packaging.',
              'private_archive': True, 'warning': 'Contains private reference/instructions. Do not publish the full archive to GitHub.'}
    entries = [{'path': n, 'size': p.stat().st_size, 'sha256': digest(p)} for p, n in included]
    manifest = {'schema_version': 1, 'created_at_utc': now, 'files': entries, 'source_report': report}
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_suffix(output.suffix+'.partial')
    if temporary.exists(): parser.error('A partial output exists; inspect it before retrying.')
    try:
        with zipfile.ZipFile(temporary, 'w', zipfile.ZIP_DEFLATED, compresslevel=6, strict_timestamps=False) as z:
            for p, n in included: z.write(p, 'GlitchTrace_Offline_Workspace/'+n)
            z.writestr('GlitchTrace_Offline_Workspace/handoff/RETURN_REPORT.json', json.dumps(report, indent=2))
            z.writestr('GlitchTrace_Offline_Workspace/handoff/RETURN_MANIFEST.json', json.dumps(manifest, indent=2))
        with zipfile.ZipFile(temporary) as z:
            if z.testzip() is not None: raise ValueError('Archive CRC check failed')
        temporary.replace(output)
    except Exception:
        # Preserve partial output for inspection; never pretend it was delivered.
        raise
    print(json.dumps({'file': str(output), 'sha256': digest(output), 'files': len(entries),
                      'modified': len(changed), 'added': len(added), 'deleted': len(deleted)}, indent=2))

if __name__ == '__main__': main()
