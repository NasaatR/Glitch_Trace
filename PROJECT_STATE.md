# GlitchTrace project state

## Goal and canonical source

User requested an independently editable native effect matching the functionality
of the supplied MyTracePlugin.aex, published as GlitchTrace in
https://github.com/NasaatR/Glitch_Trace. Additional text/background features are
explicitly deferred until the baseline. Existing repository is public; preserve
visibility. Default branch: main. Observed base: eabdb4d9b5ee945801d61bec579940bf74eb5daa.

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
adapter passed clippy and release compilation; actual PE exports and PiPL passed.
The first CI run stopped during packaging due to Python using Windows cp1252 for
a UTF-8 dependency manifest. Explicit UTF-8 reads and native-command exit handling
were corrected; the next CI run must verify packaging and installation. No actual AE host or original-vs-new renders are
available in this session. Exact original CPU detection rules, defaults/ranges,
quality sampling and font metrics are unknown. GPU/performance/pixel/temporal
parity remains unverified. The original PiPL explicitly declares SmartFX, float
color and MFR, which are known missing capabilities in this preview; see docs/REFERENCE_ANALYSIS.md and docs/HOST_CHECKS.md.

## Next action

Finish Windows CI/package verification, publish the checked preview, then perform
the host acceptance and reference comparison. Keep new functionality deferred.
On resume inspect the current main head, this record and the exact successful
workflow/release target before changing anything. Do not claim 100% parity.
