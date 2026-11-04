# GlitchTrace

Native After Effects effect that finds color regions and draws boxes, XY labels,
center markers, curved connections and arrows. Made by Nasaat.

**1.0 Preview 1:** independent implementation of the supplied reference's exposed
feature set. Exact detection, appearance and performance parity have **not** been
verified. This preview has not been loaded in After Effects. See
[reference analysis](docs/REFERENCE_ANALYSIS.md) for the evidence and gaps.

## Continue development from another account

Read [Start here](docs/START_HERE.md) and [AGENTS.md](AGENTS.md), then the current
[project state](PROJECT_STATE.md) and [handoff log](docs/HANDOFF_LOG.md).
The [roadmap](docs/ROADMAP.md) separates baseline fidelity from later text/background
features. Use [sequential coordination](docs/COORDINATION.md); one account writes
at a time. Original reference/AE access must be supplied separately as described
in [reference inputs](docs/REFERENCE_INPUTS.md). Check [release instructions](docs/RELEASING.md)
before issuing another binary; Preview 1 publication is currently hardcoded.

## Download and install

Download the installer from [Releases](https://github.com/NasaatR/Glitch_Trace/releases).
Successful Windows builds also retain packages under
[Actions](https://github.com/NasaatR/Glitch_Trace/actions/workflows/windows.yml).
If there is no successful release, do not rename another binary to GlitchTrace.

1. Close After Effects.
2. Run `GlitchTrace_v1.0-preview.1_windows-x64_setup.exe`.
3. Keep the default shared Adobe plug-ins folder, or select an AE Plug-ins folder.
4. Reopen AE, select the footage layer and apply **Effects > Nasaat > GlitchTrace**.
5. Set **Key Color**, enable **Show Mask**, adjust **Threshold**, then disable
   **Show Mask** and choose the graphics you need.

End users do not need Rust, Git, an SDK, a terminal, accounts or activation.
Windows may display an unknown-publisher prompt because the installer is unsigned.
Compare the download with `SHA256SUMS.txt`; code signing is not included.
The installer targets Windows 11 x64; AE 25.6 / 26.x are the initial host targets,
not a tested compatibility matrix. Intel/AMD x64 only; Windows ARM64 is unverified.

For portable installation, extract the ZIP and copy its files together into a
`GlitchTrace` subfolder of AE's Plug-ins folder. Keep the bundled licenses.

## Controls

| Group | Controls |
|---|---|
| Detection | Key Color, Threshold, Show Mask, Blur Strength, Alpha Layer, six quality levels |
| Bounding boxes | Enable, border color/thickness, fill color/opacity, gaps and gap size |
| Text | Enable, XY coordinates, color, scale, vertical offset |
| Center markers | Dot/Plus/Cross, color, size, line thickness |
| Connections | Enable, color/opacity/thickness, curvature, dashed lines, dash/gap lengths |
| Arrows | Enable, size, angle, position, filled/outline |

Alpha Layer renders only graphics on transparency. Connections link successive
region centers and close the chain for three or more regions. Detection is
per-frame color segmentation; persistent object identities are not implemented.

Additional text types, arbitrary labels and text backgrounds are deferred until
the baseline comparison. The reference's licensing/watermark is not part of this
independent product. The original effect and its saved projects are not replaced
or migrated.

## Updates and recovery

Close AE and run the newer installer over the existing installation. Setup backs
up an existing `GlitchTrace.aex` under the installation's `Backups` folder before
replacement. It stops if the backup cannot be made. Backups are retained when
uninstalling. For manual recovery, close AE, move the current binary out of the
Plug-ins directory and restore one backup as `GlitchTrace.aex`. Never leave two
copies of the same effect installed. Project parameter schemas must also be
compatible; a binary backup alone cannot undo edits saved into a project.

AE saves the effect's parameters with the project. Preserve parameter enum names
and order when extending the code; they define the binding's persisted IDs.
Uninstall through Windows Settings > Apps or the Start menu shortcut.

## Report a problem

- In the effect click **Diagnostics > Copy Diagnostic Report**, then paste the
  report into the chat or a [GitHub issue](https://github.com/NasaatR/Glitch_Trace/issues).
- For Windows, GPU driver and installed AE builds, run **GlitchTrace > Collect
  Diagnostics** from Start. It saves a local text report, opens it in Notepad and
  attempts to copy it. Reports are under `%LOCALAPPDATA%\GlitchTrace\Diagnostics`.
- Add exact steps, expected/actual results, AE build, and a screenshot if useful.
  An external Windows/GPU report does not include the plugin's in-memory error;
  include both reports when possible. Review them before sharing.

The effect has no network client, activation, telemetry or automatic upload.
Diagnostics queries local Windows hardware/AE version information on explicit
request. The last render/error report is in memory and can describe another
instance of the effect; capture it immediately after reproducing the issue.

## Development

The Windows target uses `after-effects` / `after-effects-sys` 0.4.0 (pre-generated
AE SDK 25.6 bindings) and `pipl` 0.1.1. Cargo.lock pins dependencies. No SDK archive
is distributed. Windows builds require Rust stable, Visual Studio C++ Build Tools
with Windows SDK, Python 3.11+, and Inno Setup 6. These are build prerequisites,
not installation prerequisites. GitHub Actions supplies the Windows build host.

```sh
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --release --locked
```

On Windows the library output is `target/release/glitchtrace.dll`; packaging
renames it to `GlitchTrace.aex`, checks PE exports/PiPL, bundles dependency notices
and compiles the installer. On Linux only the independent core is built/tested;
it is not an AE plugin target. A Linux-to-Windows `cargo check` verifies types,
not resource compilation or final linking.

See [host checks](docs/HOST_CHECKS.md), [project state](PROJECT_STATE.md), and
[release notes](docs/RELEASE_NOTES.md). CPU rendering supports 8/16 bpc; 32-bpc,
SmartFX, GPU and MFR flags are not advertised. Large blur kernels and dense
regions can be slow. Output is clipped to the source layer bounds.
