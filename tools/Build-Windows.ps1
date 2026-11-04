# Build/install/package on a Windows build host; GitHub and Git are not required.
[CmdletBinding()]
param([switch]$Offline, [switch]$RunInstallerChecks)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$root = Split-Path $PSScriptRoot -Parent
$oldBuild = $env:GLITCHTRACE_BUILD_SHA
function Invoke-Checked([string]$Program, [string[]]$Arguments) {
    & $Program @Arguments
    if ($LASTEXITCODE -ne 0) { throw "$Program failed with exit code $LASTEXITCODE" }
}
Push-Location $root
try {
    if ($env:OS -ne 'Windows_NT' -or -not [Environment]::Is64BitProcess) { throw 'Use a Windows x64 build host.' }
    foreach ($command in @('cargo','python')) {
        if (-not (Get-Command $command -ErrorAction SilentlyContinue)) { throw "Build prerequisite missing: $command. See docs/OFFLINE_WORK.md." }
    }
    $iscc = Join-Path ${env:ProgramFiles(x86)} 'Inno Setup 6/ISCC.exe'
    if (-not (Test-Path $iscc)) { throw 'Build prerequisite missing: Inno Setup 6. This is not required for end-user installation.' }
    if (Test-Path vendor) { $Offline = $true }
    $resolve = @('--locked')
    if ($Offline) { $resolve += '--offline' }
    if (-not $env:GLITCHTRACE_BUILD_SHA) {
        $sources = @('Cargo.toml','Cargo.lock','build.rs','.cargo/config.toml') + @(Get-ChildItem src -File -Recurse | ForEach-Object FullName)
        $hashes = ($sources | Get-FileHash -Algorithm SHA256 | ForEach-Object Hash) -join ''
        $digest = [Security.Cryptography.SHA256]::Create().ComputeHash([Text.Encoding]::UTF8.GetBytes($hashes))
        $id = ([BitConverter]::ToString($digest)).Replace('-','').ToLower().Substring(0,16)
        $env:GLITCHTRACE_BUILD_SHA = "archive-source:$id"
    }
    Invoke-Checked 'cargo' @('fmt','--all','--','--check')
    Invoke-Checked 'cargo' (@('test') + $resolve)
    Invoke-Checked 'cargo' (@('clippy') + $resolve + @('--all-targets','--','-D','warnings'))
    Invoke-Checked 'cargo' (@('build','--release') + $resolve)
    Invoke-Checked 'cargo' (@('fetch') + $resolve)
    New-Item dist -ItemType Directory -Force | Out-Null
    Copy-Item target/release/glitchtrace.dll dist/GlitchTrace.aex -Force
    Copy-Item target/release/glitchtrace-diagnostics.exe dist/ -Force
    Invoke-Checked 'python' @('tools/verify_pe.py','dist/GlitchTrace.aex')
    Invoke-Checked 'python' @('tools/check_windows_load.py','dist/GlitchTrace.aex')
    Invoke-Checked 'python' @('tools/collect_licenses.py')
    Invoke-Checked $iscc @('installer/GlitchTrace.iss')
    $iss = Get-Content installer/GlitchTrace.iss -Raw
    $match = [regex]::Match($iss,'(?m)^OutputBaseFilename=(.+)_setup\s*$')
    if (-not $match.Success) { throw 'Installer OutputBaseFilename must end in _setup.' }
    $prefix = $match.Groups[1].Value.Trim()
    $setup = (Resolve-Path "dist/${prefix}_setup.exe").Path
    if ($RunInstallerChecks) {
        $installDir = Join-Path ([IO.Path]::GetTempPath()) ('GlitchTrace-check-' + [guid]::NewGuid().ToString('N'))
        $installArgs = @('/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART','/SP-',('/DIR="{0}"' -f $installDir))
        $first = Start-Process $setup -ArgumentList $installArgs -Wait -PassThru
        if ($first.ExitCode -ne 0) { throw "Installation failed: $($first.ExitCode)" }
        $expected = (Get-FileHash dist/GlitchTrace.aex).Hash
        if ((Get-FileHash "$installDir/GlitchTrace.aex").Hash -ne $expected) { throw 'Installed AEX differs.' }
        $second = Start-Process $setup -ArgumentList $installArgs -Wait -PassThru
        if ($second.ExitCode -ne 0) { throw "Reinstall failed: $($second.ExitCode)" }
        $backups = @(Get-ChildItem "$installDir/Backups/*.aex.bak")
        if ($backups.Count -ne 1 -or (Get-FileHash $backups[0].FullName).Hash -ne $expected) { throw 'Backup differs.' }
        $remove = Start-Process "$installDir/unins000.exe" -ArgumentList '/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART' -Wait -PassThru
        if ($remove.ExitCode -ne 0 -or (Test-Path "$installDir/GlitchTrace.aex")) { throw 'Uninstall failed.' }
        if (-not (Test-Path $backups[0].FullName)) { throw 'Uninstall removed the backup.' }
        # Only this uniquely named temporary test installation is removed.
        Remove-Item $installDir -Recurse -Force
    }
    Copy-Item README.md,LICENSE,THIRD_PARTY_NOTICES.md,PROJECT_STATE.md dist/ -Force
    # Replace only our generated staging directory, not user documents.
    if (Test-Path dist/docs) { Remove-Item dist/docs -Recurse -Force }
    Copy-Item docs dist/docs -Recurse
    $report = [ordered]@{
        schema_version = 1; package_version = ((Select-String Cargo.toml '^version = "(.+)"$').Matches.Groups[1].Value)
        build_id = $env:GLITCHTRACE_BUILD_SHA; target = 'Windows x64'; vendored_offline_build = [bool]$Offline
        source_checks = 'format, tests, clippy, release link'; pe_and_windows_load = 'passed'
        installer_lifecycle = $(if ($RunInstallerChecks) {'passed on this build host'} else {'not run'})
        after_effects_execution = 'not performed by this build script'
    }
    $report | ConvertTo-Json | Set-Content dist/BUILD_REPORT.json -Encoding UTF8
    $portable = "dist/${prefix}_portable.zip"
    $files = @('dist/GlitchTrace.aex','dist/glitchtrace-diagnostics.exe','dist/README.md','dist/LICENSE','dist/THIRD_PARTY_NOTICES.md','dist/PROJECT_STATE.md','dist/BUILD_REPORT.json','dist/docs','dist/third-party-licenses')
    Compress-Archive -Path $files -DestinationPath $portable -Force
    Get-FileHash @($setup,$portable,'dist/GlitchTrace.aex','dist/glitchtrace-diagnostics.exe') -Algorithm SHA256 |
        ForEach-Object { "$($_.Hash.ToLower())  $([IO.Path]::GetFileName($_.Path))" } | Set-Content dist/SHA256SUMS.txt -Encoding UTF8
    Write-Host "Built $setup and $portable. Review BUILD_REPORT.json; no AE run is implied."
} finally {
    $env:GLITCHTRACE_BUILD_SHA = $oldBuild
    Pop-Location
}
