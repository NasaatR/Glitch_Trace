# Start here: continue GlitchTrace

## What this project is

Native Windows After Effects effect: color-region detection and procedural boxes,
coordinate labels, markers, curves, dashes and arrows. The user wants baseline
functional/visual behavior matching a specific supplied AEX, followed by richer
text and configurable text backgrounds. The current independent CPU preview is
not the completed baseline. Do not start a new scaffold or clone Tracery 2.

Canonical source: https://github.com/NasaatR/Glitch_Trace, default branch `main`.
Current published package: [1.0 Preview 1](https://github.com/NasaatR/Glitch_Trace/releases/tag/v1.0-preview.1).
Always read the actual remote head and state; these links are not a frozen head.

## Read in this order

1. Root `AGENTS.md`: stable project constraints, commands and identity.
2. `PROJECT_STATE.md`: current owner/status, checked work, gaps and next action.
3. `docs/HANDOFF_LOG.md`: the latest handback; use Git diffs for the actual changes.
4. `docs/ROADMAP.md`: milestone acceptance and priority. It is not a claim of work done.
5. `docs/REFERENCE_INPUTS.md` and `docs/REFERENCE_ANALYSIS.md`: exact reference and
   what can/cannot be established. Use `docs/reference_manifest.json` for inventory.
6. Only task-relevant source from the map below. Read `docs/RELEASING.md` before
   changing version/build/package/publication behavior.

No need to reread all files or run all unchanged checks merely because the account
changed. First verify access, head, local changes and the relevant prior evidence.

## Source map

| Path | Responsibility |
|---|---|
| `src/core.rs` | CPU mask, Gaussian, independent connected components, coverage/compositing, boxes, numeric font, curves, markers/arrows |
| `src/host.rs` | Persisted parameter enum, AE controls, input/output pixel conversion, cancellation, last-render/error report |
| `src/clipboard.rs` | Windows clipboard ownership and hidden-window fallback |
| `src/bin/glitchtrace-diagnostics.rs` | Explicit local Windows/GPU/AE environment report; no automatic upload |
| `src/lib.rs` | Pure core exports and Windows-only host module |
| `build.rs` | PiPL effect identity, native version, flags, panic guard and build SHA |
| `Cargo.toml`, `Cargo.lock` | Package version, exact dependency resolution and release settings |
| `.cargo/config.toml` | Windows MSVC static CRT flag; run Cargo from repo root |
| `tests/render.rs` | Nine synthetic renderer test cases; not AE or parity tests |
| `examples/render_fixture.rs` | Repeatable three-region PPM rendered by the real core |
| `installer/GlitchTrace.iss` | Installer identity, files, backup/update/uninstall behavior |
| `.github/workflows/windows.yml` | Windows tests/build/PE checks/install lifecycle/packages/first-release publication |
| `tools/verify_pe.py` | Static x64 DLL export and PiPL verification |
| `tools/check_windows_load.py` | Windows LoadLibrary/FreeLibrary only, no AE execution |
| `tools/collect_licenses.py` | License bundle from locked Cargo sources; explicit UTF-8 reads |
| `tools/analyze_reference.py` | Read-only metadata/UI/uniform/PiPL inventory of the external reference |
| `docs/HOST_CHECKS.md` | AE acceptance procedure and result expectations |
| `docs/RELEASE_NOTES.md`, `docs/INSTALL-NOTICE.txt` | User-facing release/installation facts; update when issuing binaries |

## First session after transfer

- Verify this account's GitHub connection can read the named repository; make
  actual authorized writes through a supported route. A permissions schema is
  not proof of successful publishing. Do not ask the user for tokens.
- Follow COORDINATION. Confirm the current head/state before continuing. A local
  scratch checkout from a previous chat is not assumed to exist.
- Identify whether AE is actually available. A GitHub/Adobe connector does not
  automatically expose the user's desktop. Complete independent source work
  while clearly labeling unavailable host checks.
- Start with roadmap M0: test the released effect in AE if possible, preserve the
  original error and fix any load/render regression first. If the reference is
  missing, it blocks comparative claims, not all independent implementation work.
- Report and preserve evidence in the repository before handing back.

## Suggested instruction for the other account

> Continue https://github.com/NasaatR/Glitch_Trace. Read the current main branch's
> AGENTS.md, docs/START_HERE.md, PROJECT_STATE.md and latest HANDOFF_LOG entry.
> Follow the sequential ownership protocol. First close the baseline gaps in
> ROADMAP.md; do not call Preview 1 a 100% clone. Then implement the previously
> requested arbitrary text/text-background features. Commit and push coherent
> checked units to main, verify the relevant CI/release, update state/log and
> hand back ready for the next account. Ask for missing reference evidence only
> where it blocks the dependent comparison, and keep progressing independently.
