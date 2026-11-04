# Reference inputs and cross-account transfer

## Exact reference

| Field | Value |
|---|---|
| User filename | `MyTracePlugin.aex` |
| SHA-256 | `108bb818afc250c4076e24efbb297498e963b4ae97b00455ee2ca298dd5a4f7a` |
| Internal identity | Tracery / Dragoy / `DRAGOY Tracery` |
| Platform | Windows x64 PE32+ native AE effect |
| Inventory | `docs/reference_manifest.json` |
| Static findings / unknowns | `docs/REFERENCE_ANALYSIS.md` |

The original file was attached in the originating conversation. It is **not**
in this repository, the portable package or another account's conversation.
The originating scratch path is deliberately not a portable source reference.
Ask the user to attach that same AEX directly in the new conversation, then
verify its digest before dependent analysis/comparison. Do not silently replace
it with current Tracery/Tracery 2 or a random third-party download.

Read-only inventory command, from the repo root:

```sh
python tools/analyze_reference.py /path/to/MyTracePlugin.aex
```

Use the supplied file only as the reference within the requested work. Do not
commit it, extracted shader/font assets, activation data or private build paths
into this public repository. If local storage is needed, keep it outside the
checkout or in the ignored `reference-private/` directory.

## Evidence required for the fidelity comparison

The AEX alone does not establish a successful run. Use the same footage, frame,
AE version/build, project color management, bit depth, preview scale and settings
for both effects. Capture:

- Original effect controls expanded, with default values/ranges/units recorded.
- Synthetic test composition/input dimensions and frame timing, preferably
  created from public-safe geometry/colors with no user media.
- Original and GlitchTrace outputs at matched frames, and the exact parameters.
- Detection counts/bounds/order separately from text/shape rasterization.
- Repeated/scrubbed frames, disappearing regions, transparent/edge objects,
  quality modes, >512 regions, large blur, CPU/GPU timings and cancellation.
- Exact original errors and environment reports for failures.

`docs/HOST_CHECKS.md` is the checklist. Record sanitized results in a small
`docs/validation/<descriptive-name>.md` when evidence actually exists; do not
create placeholder 'passed' records. Keep private footage, AE project files and
unsanitized reports out of GitHub. The user may attach them privately to a chat.
No automatic upload from the plugin is part of the design.

## Already available without the original file

All GlitchTrace source, exact dependency lock, installer/build workflow, core
tests, reference inventory, known parity gaps and prior Windows build evidence
are in GitHub. The second account can inspect/debug/implement independent work
with these. Missing reference/host inputs block only the affected claims and
comparative acceptance; do not fabricate them or stop all unrelated work.
