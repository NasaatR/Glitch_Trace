# GlitchTrace project state

## Goal and canonical source

User requested an independently editable native effect matching the functionality
of the supplied MyTracePlugin.aex, published as GlitchTrace in
https://github.com/NasaatR/Glitch_Trace. Additional text/background features are
explicitly deferred until the baseline. Existing repository is public; preserve
visibility. Default branch: main. Initial repository base: eabdb4d9b5ee945801d61bec579940bf74eb5daa.

## Sequential session handoff

- Status: **ready for archive-only session**.
- Current route: originating account handles GitHub; receiving account works from a private archive without GitHub. One writer at a time.
- Observed source head before this documentation handoff:
  aff2c833adb03ec35cc0a71ecb654b2e1822cfc7.
- Start: if OFFLINE_SESSION.json exists, read the outer START_HERE and docs/OFFLINE_WORK.md first; otherwise use the connected route.
- Coordination: keep source/state/logs durable. Receiving account makes a return ZIP only on explicit user request; no automatic handback exports.
- Plan: docs/ROADMAP.md (M0-M5 baseline; M6 requested text/background; M7 expansion).
- Inputs: docs/REFERENCE_INPUTS.md. Original AEX is not in GitHub; the private archive includes it, preferences, Playbook and released installers. Verify its digest. Host/build access is not transferred.
- Release caution: docs/RELEASING.md. Current workflow is hardcoded to Preview 1;
  a later code push does not automatically replace the public installer.

## Decisions

- Independent implementation; no reference binary, shader/atlas extraction or
  aescripts licensing code in the repository.
- Rust native AE effect via locked after-effects 0.4.0 / sys 0.4.0 and pipl 0.1.1.
- CPU 8/16-bpc reference renderer first. No guessed GPU/MFR/SmartFX flags.
- New effect identity: Nasaat GlitchTrace, Effects > Nasaat > GlitchTrace.
- Product 1.0 Preview 1 / native 1.0.0 development build 1 / Cargo 1.0.0-preview.1.
- main push builds a Windows installer/portable ZIP and creates the first preview
  release only after checks. Published release assets are never overwritten.

## Implemented

All exposed detection/box/text/marker/connection/arrow control groups, transparency,
bounded region count, independent per-frame CPU detection, row-stride-aware AE
8/16-bpc adapter, cancellation, in-effect report copying, external environment
report collector, Windows installer with binary backup, package/license checks,
reference inventory and explicit parity matrix.

## Verification and remaining acceptance

Nine synthetic renderer tests passed locally and on Windows. Windows native
adapter passed clippy and release compilation. Actual PE exports/PiPL and Windows
LoadLibrary/FreeLibrary passed. Installer build, silent installation, reinstall
with checksum-verified backup, uninstallation and retained backup passed on the
Windows Server 2025 runner. This is packaging validation, not a Windows 11/AE run.

Successful workflow: https://github.com/NasaatR/Glitch_Trace/actions/runs/41773124768
Release: https://github.com/NasaatR/Glitch_Trace/releases/tag/v1.0-preview.1
Release target: fdb70d1a7f179a6fbf123194b35f45e6a0d39db1.
Installer SHA-256: abf8a7fdca8feac9beb087a415565d379973822bd9c386d42abb940b3e863119.
Portable SHA-256: 8bf5c518b803872e0eac2fef09e08341f39d0695e49c8cc320ef44c73dbca26e.
Downloaded Actions artifact digests matched the release assets; internal checksums,
217 portable archive entries, documentation/licenses and actual AEX PE were checked.
No reference binary, caches, PDBs or build directories are published in the package.
The first packaging failure was fixed by explicit UTF-8 dependency manifest reads.

No actual AE host or original-vs-new renders are available in this session.
**The requested 100% parity goal is not complete.** Exact original CPU detection,
defaults/ranges, quality sampling and font metrics remain unknown. GPU/performance,
pixel and temporal parity are unverified. Original PiPL declares SmartFX, float
color and MFR; this preview is CPU 8/16 bpc and does not implement/advertise these
capabilities. See docs/REFERENCE_ANALYSIS.md and docs/HOST_CHECKS.md.

## Archive-only build support

Standalone Windows build/test/package script: tools/Build-Windows.ps1. Cargo source
can be vendored for a no-download build; a real Windows Rust/MSVC/Inno environment
is still required. The receiving agent must not relabel an old installer as a new
build if that environment is missing. Request-only return helper:
tools/Pack-Return.py. The same standalone pipeline passed Windows CI with vendored
dependencies and Cargo offline: format/core tests/clippy, target link, PE/PiPL,
LoadLibrary, setup/reinstall/backup/uninstall and package/report generation.
Validation source: f279674c38fa15b4a975745e41bf2649ac9f138f.
Workflow: https://github.com/NasaatR/Glitch_Trace/actions/runs/41883545388.
Three request-only archive tests also passed. Local empty-Cargo-home core tests
passed offline; all 76 vendor packages / 3220 dependency-file checksums verified.
The transfer's source files, private preferences/reference and package inventory
are recorded in its own manifest, not published together to GitHub. The existing
Preview 1 user release remains unchanged; this work adds build/handoff support.

## Next action

Incoming archive session: read the outer START_HERE, preferences and OFFLINE_WORK; start ROADMAP M0 without requiring GitHub.
Establish actual AE load/render on a saved synthetic/test project; fix the first
blocking regression with the original error and environment captured. If host or
reference access is absent, collect the dependent evidence while continuing
independent baseline work. Then follow M1-M5 before declaring full fidelity; keep
new text/background functionality deferred until the baseline or explicit user
reprioritization. Record the result, release/checks and next step before handback.
On resume inspect the current main head, this record and the exact successful
workflow/release target before changing anything. Do not claim 100% parity.
