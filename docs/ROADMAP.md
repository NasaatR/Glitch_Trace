# GlitchTrace roadmap and acceptance

This is the implementation order, not a completion report. PROJECT_STATE is the
live status; HANDOFF_LOG records transitions. Follow newer explicit user steering
and reconcile the plan instead of silently discarding the original goal.

## User outcome

1. An editable, maintainable native GlitchTrace effect on GitHub, matching the
   supplied reference's functionality and practical workflow as closely as the
   evidence establishes. A 100% claim needs demonstrated agreement, not strings.
2. Easy Windows installation/update/recovery and a useful copyable bug report.
3. After the baseline: alternate/custom text instead of coordinates only, plus
   an independently enabled and configurable text background sized to the label.
4. Continued improvements without breaking projects, installation or identity.

Existing source/build is a preview, not the finished outcome. Two accounts work
sequentially from the same repository. No new backend/relay is required for this.

## Milestones

### M0 — Establish actual AE behavior and fix blocking regressions

- Inspect current main and the published package. Load/apply on the user's AE
  build if accessible; AE 26.5x89/en_US/Windows x64 was reported in another AE
  task, so treat it as a candidate to confirm, not this plugin's tested host.
- Execute HOST_CHECKS on a saved copy/synthetic comp. Preserve precise errors,
  settings and environment; fix load/render/crash issues before new features.
- If no host is available, finish available source/build checks and provide a
  short reproducible user-run test. Do not invent an AE connection.
- Acceptance: known load/render failures addressed with reproduction evidence;
  unrun checks explicitly marked. A CI/LoadLibrary pass alone does not close M0.

### M1 — Reference-driven detection, defaults and graphics fidelity

- Verify the exact AEX digest; capture original defaults/ranges and output.
- Resolve region connectivity, filtering/minimum size, ordering, sampling/quality
  and transparent-pixel behavior. Current eight-connected policy and strides
  1/2/3/4/6/8 are independent choices, not recovered reference behavior.
- Match coordinates, font metrics, gaps/fills, marker dimensions, curves/dash
  phase/arrow geometry and layer composition with controlled fixtures.
- Convert observed requirements into meaningful regression tests/golden inputs.
  Do not reuse proprietary extracted font/shader assets as GlitchTrace source.
- Acceptance: matched controls/behavior documented per fixture; remaining visual,
  numeric and timing differences listed. Do not invent a tolerance and call it
  user-approved 100% fidelity. Agree on material unavoidable differences.

### M2 — SmartFX and 32-bpc path

- Use the exact binding/SDK contract; design whole-image input dependencies while
  mapping cropped/offset output ROIs correctly. Handle checkout/checkin on every
  error/cancel path and keep the original operation error.
- Preserve HDR float RGB and correct alpha; don't apply the integer path's clamps
  to floats. Respect rowbytes, origins, downsample and empty worlds.
- Add flags only alongside a complete implementation. Keep legacy 8/16 paths.
- Acceptance: Windows build plus host renders at 8/16/32 bpc, ROI/crop/edges,
  full/half/quarter/asymmetric scale, cancellation and repeated renders. Record
  actual before/after results; compilation is only one layer of evidence.

### M3 — GPU rendering and measured performance

- Measure the CPU/reference bottleneck first with fixed input, resolution,
  settings and hardware/driver. Original uses its own OpenGL path; that is not
  automatically equivalent to AE's GPU suites or MFR.
- Choose a documented host-compatible backend only after proving context/device,
  resource lifetime, pixel/color transfer and a supported build/install route.
- Reuse deterministic CPU math as an oracle; ensure graceful failures/fallback
  where the chosen backend contract permits it. Preserve user settings on errors.
- Acceptance: matched CPU/GPU outputs with documented differences, performance
  comparisons, multiple instances, driver/context recovery and cancellation in AE.
  Do not claim speedup without measurement or ship a renderer flag alone.

### M4 — Thread safety and Multi-Frame Rendering

- Audit the pinned after-effects 0.4.0 global callback/state/sequence lifetimes,
  especially mutable callbacks and pointer aliasing. Current upstream main has
  changed APIs; an upgrade requires checking both wrappers and actual behavior.
- Make render-time data local/immutable; isolate UI/clipboard/report state. Use
  host-legal callbacks and thread-safe resource ownership. Prevent stale frame
  reports from being represented as the current instance's result.
- Acceptance: concurrent instances and out-of-order frames in actual AE with
  MFR on/off produce matching results; no data races/crashes/cross-instance state.
  Advertise MFR only after the host gate; Rust Sync/Mutex alone is insufficient.

### M5 — Baseline acceptance and dependable releases

- Review M0-M4 evidence against REFERENCE_ANALYSIS; update every capability status.
  Distinguish implemented, automated-tested, host-tested and reference-compared.
- Verify save/reopen, parameter identity, update/recovery, clean installation and
  diagnostics on the target Windows/AE build. Close material known gaps.
- Update version mapping/build/publication before a new binary; current workflow
  is hardcoded to Preview 1. See RELEASING. Never overwrite released assets.
- Acceptance: exact release commit, passing relevant CI, inspected assets, useful
  first-use steps, explicit remaining limits and recorded user/reference review.
  Only then label the baseline complete; a 100% label needs corresponding proof.

### M6 — Requested text and text backgrounds

Start after baseline acceptance, or earlier only if the user explicitly changes
that priority. This functionality was requested already, not a new upsell.

- Keep coordinates and add user-entered text/alternate display content. Font,
  Unicode/Cyrillic, long labels, multiline behavior and text entry method need
  an actual supported AE control/host design, not a disconnected browser mockup.
- Independent background toggle and automatic fit to the rendered label.
- Proposed initial controls: background color/opacity, X/Y padding, corner radius,
  optional border and text/background offset. These are implementation proposals,
  not all separately confirmed requirements. Keep the native UI compact.
- Preserve existing parameter IDs/indices and saved projects. Define text/schema
  persistence, invalid-input recovery, undo/apply behavior and font fallback.
- Acceptance: coordinates/custom text, background off/on, empty/long/Cyrillic text,
  transparency, bounds/preview scales, save/reopen and repeated changes in AE.

### M7 — Maintainable expansion and focused polish

Use measured/report-driven priorities. Factor only repeated structures (parameter
registration/specs, renderer primitives, text layout and diagnostics). Keep UI,
processing and host IO separate. Add no speculative settings/backend/framework.
Every new function needs a clear location in native controls, project persistence,
relevant tests, installable release and an updated state/handback.

## Evidence status vocabulary

| Status | Meaning |
|---|---|
| Planned | Work is specified; no implementation claim |
| Implemented | Real source path exists; host result may be untested |
| Automated checked | Named checks passed for the exact recorded input/revision |
| Host checked | Named action/output verified in the actual AE environment |
| Reference compared | Same inputs/settings compared against the exact AEX |
| Accepted | Defined acceptance met; material limits and decisions recorded |

Advance the status only when evidence supports it. Source availability, builds,
installation, host rendering and fidelity are separate gates.
