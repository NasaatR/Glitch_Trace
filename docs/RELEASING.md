# Build and release handoff

## Current released mapping (verify again before changing it)

| Surface | Preview 1 value / location |
|---|---|
| Product display | GlitchTrace 1.0 Preview 1 |
| Cargo | `1.0.0-preview.1` in Cargo.toml; lock package entry in Cargo.lock |
| PiPL | 1.0.0, Develop, build 1 in build.rs |
| About text | Hardcoded display/capabilities in src/host.rs |
| Installer | AppVersion 1.0.0, DisplayVersion 1.0 Preview 1 in installer/GlitchTrace.iss |
| Release tag | `v1.0-preview.1` in .github/workflows/windows.yml |
| Asset prefix | `GlitchTrace_v1.0-preview.1_windows-x64` in installer/workflow |
| Release source | `fdb70d1a7f179a6fbf123194b35f45e6a0d39db1` |
| Release | https://github.com/NasaatR/Glitch_Trace/releases/tag/v1.0-preview.1 |

The current workflow **does not automatically issue the next version**. It builds
on a main push and skips publishing if `v1.0-preview.1` already exists. It can
therefore produce a new Actions artifact with an old label while leaving the
public installer unchanged. A successful run is not proof of a new user release.
Fix/update this mapping as one coherent task before the next binary delivery.

## Before the next binary

1. Inspect actual tags, releases, highest native version and the current main
   head. Choose a genuinely newer version preserving the existing mapping.
   A second preview may use Cargo 1.0.0-preview.2, PiPL Develop build 2 and
   tag/file labels v1.0-preview.2; validate ordering rather than assuming SemVer
   applies to every host/installer field. Do not renumber an existing download.
2. Update Cargo package/lock, PiPL, About, installer, all workflow artifact/tag/glob
   strings, release notes, install notice and affected README instructions.
   Keep historical evidence/log entries intact. Inventory with:

   ```sh
   rg -n '1\.0|preview\.1|Preview 1' Cargo.toml Cargo.lock build.rs src/host.rs installer .github/workflows docs README.md
   ```

3. Preserve match name/category, installer AppId, original notices and parameter
   IDs/indices. Explain/test any schema migration and prior-version recovery.
4. Run the relevant format/core/native/build checks from repo root. Inspect
   unfamiliar build scripts/dependency changes and primary API sources first.
5. Update publication logic if needed. Existing releases/assets must not be
   overwritten. A docs-only handover does not need a new product version.

## Windows pipeline contract

`windows-2025` currently supplies Rust, Visual Studio/Windows SDK, Python and
Inno Setup 6. Recheck runner/tool availability if a build fails or targets change.
End users install the generated setup; they do not install Git/Rust/SDK tools.

The workflow checks formatting, nine current synthetic test cases, clippy, release
linking, x64 PE exports and PiPL, Windows LoadLibrary/FreeLibrary, license bundle,
installer compilation, silent install/reinstall with hash-verified backup, uninstall
with retained backup, portable package and SHA-256 sums, then uploads/publishes.
The Windows server runner has no AE. Its installer test is not a Windows 11 or
After Effects compatibility test. New host scenarios need separate evidence.

- Native files: `target/release/glitchtrace.dll` -> `dist/GlitchTrace.aex` and
  `target/release/glitchtrace-diagnostics.exe`.
- Static CRT is configured in `.cargo/config.toml`; Cargo must see the repo root.
- Dependency notices are read as UTF-8, not Windows' default code page. Do not
  remove the explicit encoding fix or license packaging checks.
- After each native command in a multi-command PowerShell step, preserve/check
  its exit code so a later successful command cannot hide its failure.
- Release CI has job-scoped contents write permission via GitHub's runner token;
  never request/paste a token in a conversation.

## Delivery verification

Verify the exact workflow head SHA, push/manual event, attempt and all relevant
steps. For a successful release verify tag target and assets, download/inspect
the archive, check digests/checksums, required docs/licenses, and absence of private
inputs, caches, PDBs and unrelated files. Check PE/PiPL for the packaged AEX itself.

Then update PROJECT_STATE and HANDOFF_LOG with links and verified limits. Keep
actual AE load/render/save/reopen/update and reference comparisons separate from
CI. If artifacts exist but release publication failed, say that explicitly and
finish publication through the authorized supported route; do not claim a remote
update from local output. Inspect uncertain side effects before retrying.
