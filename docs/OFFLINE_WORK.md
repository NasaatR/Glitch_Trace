# Work from a supplied archive without GitHub

This mode is selected by the user's current instruction and an `OFFLINE_SESSION.json`
file in the project directory. It overrides connected-account steps in AGENTS,
START_HERE, COORDINATION and RELEASING. Do not require GitHub login, a connector,
Git, tokens, remote pushes or a fresh checkout. The connected originating agent
owns later GitHub integration. The archived Git SHA is provenance, not a live lock.

## Incoming workspace

Read the outer START_HERE first, then this file, USER_PREFERENCES_AE, project state,
latest log and ROADMAP. Keep the complete workspace together. The supplied source
is authoritative for this session; do not replace it with an older public release.
Reference AEX, existing released installer and user instructions are separate from
editable project source. Private material must never be uploaded to public GitHub.

Continue normal implementation/debugging with the user. Keep PROJECT_STATE and
HANDOFF_LOG accurate at milestones; preserve source through the available durable
file/workspace route. Do not assume scratch storage survives indefinitely.
Do not automatically produce a return/handoff ZIP after each task. Produce it
**only when the user explicitly asks**. Normal versioned installers for requested
product changes are a separate deliverable and remain authorized.

## Build and deliver without GitHub

The transfer includes Cargo dependencies under `project/vendor` and a scoped Cargo
source override. These remove GitHub/crates downloads from the build. They do not
include a Rust compiler, Windows SDK, Visual Studio or Inno Setup executable.

On an available Windows x64 build host with Rust stable + rustfmt/clippy, Visual
Studio C++ Build Tools/Windows SDK, Python 3.11+ and Inno Setup 6, run from project:

```powershell
./tools/Build-Windows.ps1 -Offline -RunInstallerChecks
```

The script has no GitHub/Git dependency. Windows Server installer checks require
an OS satisfying the installer minimum and privileges for the test installation.
Omit RunInstallerChecks only when the environment cannot run them and state that
gap. Read scripts before running. Use primary native API evidence for source changes.

A Windows build is a real prerequisite for a changed `.aex`. A Linux-only/Python-only
ChatGPT environment cannot be assumed to provide it. Inspect actual capabilities
first. If unavailable, finish source/core checks and name the exact missing build
step; never rename a Linux library, patch/rename an old AEX, or present Preview 1
as a newly built update. Do not make the user install a development stack merely
to use a finished plugin. The supplied released setup installs the existing version.

For each new delivered binary update the version mapping before building, retain
identity/settings, inspect packages/checksums and give a ready setup plus concise
install -> first action -> expected result -> update/recovery -> report steps.
The old GitHub publication workflow is not required in this mode. Keep release
notes locally and distinguish local delivery from GitHub publication.

## Persist, then return only on request

On ordinary tasks keep source, state, validation evidence and user-facing outputs
up to date. If only ephemeral storage is available, tell the user before work is
lost and use available persistent file storage; do not invent completed saves.

When explicitly asked to transfer back:

1. Update PROJECT_STATE and HANDOFF_LOG with changes, exact checks/gaps, versions,
   delivered asset hashes, next action and any unfinished work.
2. Put new install packages and build reports in outer `deliverables/`; put only
   approved sanitized evidence in `evidence/`. Keep complete current source and
   vendor data. Do not include credentials, caches/toolchains or private footage.
3. Run the supplied `tools/Pack-Return.py` from the project directory:

   ```sh
   python tools/Pack-Return.py --requested-by-user --output /path/outside/workspace/GlitchTrace_Return.zip
   ```

4. Inspect the ZIP/checksums and save it through the current environment's durable
   artifact route. Give one downloadable file. This script creates no network
   upload or automatic GitHub operation. It needs no Git installation.

The return includes a source change/deletion report against the starting archive,
whole current source, reference, instructions, and outputs. The originating agent
must compare against both the recorded base and current GitHub before integrating;
never blindly overwrite a newer main. Original reference/personal files stay private.
