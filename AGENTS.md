# GlitchTrace project guidance

## Start and ownership

Read [docs/START_HERE.md](docs/START_HERE.md), then the current
[PROJECT_STATE.md](PROJECT_STATE.md). Fetch the actual remote default branch before
editing; stale chat context or a downloaded ZIP is not the current source.
The user alternates between two ChatGPT accounts. Only one session writes at a
time. Follow [docs/COORDINATION.md](docs/COORDINATION.md) for takeover and handback.
Never assume the other session's local files, attachments, tools or authentication
are available here. Do not spawn agents unless the current user authorizes it.

## Scope and product identity

- Preserve GlitchTrace, `Nasaat GlitchTrace`, category Nasaat, the installer AppId,
  settings/parameter identity, existing licenses and small Made by Nasaat credit.
- Russian conversation; English code, comments, documentation, GitHub and new UI.
- Baseline fidelity to the supplied reference comes before new text/background
  functionality. Follow [docs/ROADMAP.md](docs/ROADMAP.md) and the actual user scope.
- The preview is not a 100% clone. Do not relabel unknown behavior as verified,
  and do not equate core tests, Windows loading or CI with an After Effects run.
- No original binary, shader source, font atlas, activation library or keys in
  this public repository. Reference inputs are described in
  [docs/REFERENCE_INPUTS.md](docs/REFERENCE_INPUTS.md).

## Implementation constraints

- `src/core.rs`: pure premultiplied RGBA processing; no host, UI, network or file IO.
  `src/host.rs`: AE parameter/selector adapter. Keep these boundaries.
- `Params` enum names are hashed into persistent parameter IDs by the pinned
  binding. Do not rename/reorder/remove released parameters casually. Add new
  parameters without shifting existing indices; plan/test migration if required.
- Use the exact locked dependency source/API and native SDK interfaces. Current
  `after-effects` 0.4.0 differs from upstream main; inspect before updating.
- Honor rowbytes, image origin, AE's 32768 16-bit channel scale, alpha and preview
  scale. Whole-image detection is not pixel-independent. Do not set that flag.
- SmartFX needs a full input dependency plus correct output ROI/origin mapping,
  checkout/checkin cleanup and error preservation. Float RGB must preserve HDR.
- MFR needs a binding/global-state audit and actual concurrent host tests before
  advertising the flag. `Mutex` fields or `Sync` alone do not prove safety.
- Never call arbitrary project/UI suites from render threads or mutate scenes to
  test parsing/rendering. Preserve original errors through cleanup/reporting.
- UI preferences: compact neutral native controls, useful names, zeros preserved,
  coherent groups. Test opened states, narrow controls and clipped labels in AE.

## Checks and delivery

Run relevant checks from the repository root; see README for prerequisites:

```sh
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
```

On Linux these tests cover the core, not a runnable AE plugin. Windows CI handles
native linking, PE/PiPL, LoadLibrary, installer lifecycle and packages. Inspect the
exact workflow SHA/event/attempt; do not call a stale run evidence for new code.
Use [docs/HOST_CHECKS.md](docs/HOST_CHECKS.md) for actual AE acceptance.

Commit coherent checked units to the authorized default branch, preserve concurrent
changes and visibility, and never force-push or bypass protection. If a connector
is needed, preserve the base tree and use the current branch head as commit parent;
reject/reconcile a changed head. No chat credentials or user-local Git setup.
Do not add Codex/AI coauthor attribution. Repo guidance cannot grant a connection
write access; follow the current user's authorization and actual tool results.

Before a new binary release follow [docs/RELEASING.md](docs/RELEASING.md): current
workflow names/tag are hardcoded to Preview 1 and will skip an existing release.
Never replace an issued asset under its old version. Keep the installer-first path
and bundled notices; signing and AE validation must be described honestly.

Update PROJECT_STATE at meaningful milestones/handback. Append a concise evidence-
based entry to docs/HANDOFF_LOG.md. Leave the active-session status ready for the
next account and name one actionable next step. See COORDINATION for the template.
