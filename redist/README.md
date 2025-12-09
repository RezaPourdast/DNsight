# Visual C++ Redistributable

This folder should contain the Visual C++ Redistributable installer.

## Download Instructions

1. Download **VC++ 2015-2022 Redistributable (x64)** from Microsoft:
   - Direct download: https://aka.ms/vs/17/release/vc_redist.x64.exe
   - Or visit: https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist

2. Save the file as `vc_redist.x64.exe` in this folder.

3. The Inno Setup installer will automatically include and install this if needed on the target system.

## File Location

After downloading, this folder should contain:
```
redist/
  └── vc_redist.x64.exe
```

**Note**: This file is excluded from git (see .gitignore) as it's a large binary file (~30MB).

