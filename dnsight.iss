[Setup]
AppName=DNsight
AppVersion=1.0.0
AppPublisher=Your Name
DefaultDirName={autopf}\DNsight
DefaultGroupName=DNsight
DisableProgramGroupPage=yes
OutputDir=installer
OutputBaseFilename=DNsight-Setup-1.0.0
Compression=lzma
SolidCompression=yes
WizardStyle=modern
Uninstallable=yes
PrivilegesRequired=admin
ArchitecturesAllowed=x64
MinVersion=6.1sp1

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"; Flags: unchecked
Name: "startmenuicon"; Description: "Create a &Start Menu icon"; GroupDescription: "Additional icons:"

[Files]
Source: "target\release\dnsight.exe"; DestDir: "{app}"; Flags: ignoreversion; DestName: "DNsight.exe"
Source: "asset\*.png"; DestDir: "{app}\assets"; Flags: ignoreversion recursesubdirs
Source: "redist\vc_redist.x64.exe"; DestDir: "{tmp}"; Flags: deleteafterinstall; Check: VCRedistNeedsInstall

[Icons]
Name: "{group}\DNsight"; Filename: "{app}\DNsight.exe"; Tasks: startmenuicon
Name: "{userdesktop}\DNsight"; Filename: "{app}\DNsight.exe"; Tasks: desktopicon
Name: "{group}\Uninstall DNsight"; Filename: "{uninstallexe}"

[Run]
Filename: "{tmp}\vc_redist.x64.exe"; Parameters: "/quiet /norestart"; StatusMsg: "Installing Visual C++ Redistributable..."; Check: VCRedistNeedsInstall
Filename: "{app}\DNsight.exe"; Description: "Launch DNsight"; Flags: nowait postinstall skipifsilent

[Code]
function VCRedistNeedsInstall: Boolean;
var
  Version: String;
begin
  // Check for Visual C++ 2015-2022 Redistributable (x64)
  Result := not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64', 'Version', Version);
  // Also check alternative registry location
  if Result then
    Result := not RegQueryStringValue(HKEY_LOCAL_MACHINE,
      'SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x64', 'Version', Version);
end;
