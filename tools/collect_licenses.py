"""Bundle license/notice files from the exact locked dependency source cache."""
import os
from pathlib import Path
import shutil
import tomllib

root = Path(__file__).resolve().parents[1]
lock = tomllib.loads((root/'Cargo.lock').read_text(encoding='utf-8'))
cache = Path(os.environ.get('CARGO_HOME', Path.home()/'.cargo'))/'registry'/'src'
out = root/'dist'/'third-party-licenses'
out.mkdir(parents=True, exist_ok=True)
missing = []
for p in lock['package']:
    if not p.get('source', '').startswith('registry+'): continue
    name = f"{p['name']}-{p['version']}"
    candidates = list(cache.glob('*/'+name))
    if not candidates:
        # Cargo fetch --locked ensures target-specific packages are present too.
        raise SystemExit('Missing locked dependency source: '+name)
    source = candidates[0]
    files = [f for f in source.iterdir() if f.is_file() and any(f.name.upper().startswith(k) for k in ('LICENSE','COPYING','NOTICE','UNLICENSE'))]
    manifest = tomllib.loads((source/'Cargo.toml').read_text(encoding='utf-8'))['package']
    if not files:
        # Some crates ship only a license declaration, not an upstream notice file.
        # When MIT is explicitly offered, bundle its unmodified standard terms
        # and the actual package authors; do not invent a copyright/year.
        if 'MIT' in manifest.get('license', '').split():
            pass
        else:
            missing.append(name)
    dest = out/name
    dest.mkdir(exist_ok=True)
    for f in files: shutil.copy2(f, dest/f.name)
    if not files and 'MIT' in manifest.get('license', '').split():
        terms = (root/'LICENSE').read_text(encoding='utf-8').split('Permission is hereby granted', 1)[1]
        authors = ', '.join(manifest.get('authors', [])) or 'See upstream package source'
        (dest/'LICENSE-MIT.txt').write_text(
            'MIT License\nUpstream package authors: '+authors+'\n'
            'The package declares MIT; no separate license file was shipped in the crate.\n\n'
            'Permission is hereby granted'+terms, encoding='utf-8')
    (dest/'PACKAGE.txt').write_text(f"{name}\nLicense: {manifest.get('license','See package source')}\nSource: https://crates.io/crates/{p['name']}/{p['version']}\n", encoding='utf-8')
if missing:
    raise SystemExit('Dependency has no top-level license file; inspect before distribution: '+', '.join(missing))
print('Bundled licenses for locked dependencies.')
