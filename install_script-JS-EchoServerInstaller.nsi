; Based off the example1.nsi and example2.nsi scripts
;--------------------------------

; Define the installer name, version, and output executable
Name "JS Echo Server-eQualitie test"
OutFile "JSEchoServerWinInstaller.exe"
InstallDir $PROGRAMFILES\EchoServer

; Request application privileges for Windows
RequestExecutionLevel admin
; Can add compression here

;--------------------------------

; Pages

Page directory
Page instfiles

;--------------------------------

; Section to copy files
Section "Install Echo Server"

  ; Set output path to the installation directory.
  SetOutPath $INSTDIR
  
  ; Put file there
  File "target\release\echo_server.exe"
  ; Add shortcut on desktop
  CreateShortcut "$DESKTOP\Echo Server.lnk" "$INSTDIR\echo_server.exe"

  ; Write the uninstall keys for Windows
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Example2" "DisplayName" "NSIS Example2"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Example2" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Example2" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Example2" "NoRepair" 1
  WriteUninstaller "$INSTDIR\uninstall.exe" 
  
SectionEnd

; Set the RUST_LOG environment variable globally
Section -SetEnvVar

  WriteRegStr HKCU "Environment" "RUST_LOG" "info"

SectionEnd

; Uninstaller
Section "Uninstall"

  Delete "$INSTDIR\echo_server.exe"
  Delete "$DESKTOP\Echo Server.lnk"
  RMDir "$INSTDIR"

SectionEnd