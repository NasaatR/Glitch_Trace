#define ProductVersion "1.0.0"
#define DisplayVersion "1.0 Preview 1"
[Setup]
AppId={{385171E8-3D9F-4955-8763-641EC3DD1733}
AppName=GlitchTrace
AppVersion={#ProductVersion}
AppVerName=GlitchTrace {#DisplayVersion}
AppPublisher=NasaatR
AppPublisherURL=https://github.com/NasaatR/Glitch_Trace
AppSupportURL=https://github.com/NasaatR/Glitch_Trace/issues
DefaultDirName={commonpf}\Adobe\Common\Plug-ins\7.0\MediaCore\GlitchTrace
DefaultGroupName=GlitchTrace
DisableProgramGroupPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0.22000
PrivilegesRequired=admin
OutputDir=..\dist
OutputBaseFilename=GlitchTrace_v1.0-preview.1_windows-x64_setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
UninstallDisplayName=GlitchTrace {#DisplayVersion}
CloseApplications=yes
CloseApplicationsFilter=*.aex
RestartApplications=no
SetupLogging=yes
LicenseFile=..\LICENSE
InfoBeforeFile=..\docs\INSTALL-NOTICE.txt

[Files]
Source: "..\dist\GlitchTrace.aex"; DestDir: "{app}"; Flags: ignoreversion; BeforeInstall: BackupPrevious
Source: "..\dist\glitchtrace-diagnostics.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\docs\*"; DestDir: "{app}\docs"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "..\PROJECT_STATE.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\THIRD_PARTY_NOTICES.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\dist\third-party-licenses\*"; DestDir: "{app}\third-party-licenses"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Collect Diagnostics"; Filename: "{app}\glitchtrace-diagnostics.exe"
Name: "{group}\GlitchTrace Help"; Filename: "https://github.com/NasaatR/Glitch_Trace#readme"
Name: "{group}\Uninstall GlitchTrace"; Filename: "{uninstallexe}"

[Code]
procedure BackupPrevious;
var Existing, Folder, Backup: String;
begin
  Existing := ExpandConstant('{app}\GlitchTrace.aex');
  if FileExists(Existing) then begin
    Folder := ExpandConstant('{app}\Backups');
    Backup := Folder + '\GlitchTrace-' + GetDateTimeString('yyyymmdd-hhnnss', '-', ':') + '.aex.bak';
    if (not ForceDirectories(Folder)) or (not FileCopy(Existing, Backup, True)) then
      RaiseException('Could not back up the existing GlitchTrace plugin. Installation stopped before replacement.');
    Log('Previous GlitchTrace binary backed up.');
  end;
end;
