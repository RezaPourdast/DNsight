# Release Build Steps

This document outlines the steps to build and release DNsight.

## Prerequisites

- Rust toolchain installed (via [rustup.rs](https://rustup.rs/))
- [Inno Setup](https://jrsoftware.org/isinfo.php) installed (for creating the installer)
- Visual C++ Redistributable (x64) - download from Microsoft

## Release Build Process

### 1. Clean Previous Builds (Optional)

```powershell
cargo clean
```

### 2. Build Release Binary

```powershell
cargo build --release
```

This will create the optimized executable at:
```
target\release\dnsight.exe
```

### 3. Test the Release Build

Before creating the installer, test the release executable:

```powershell
.\target\release\dnsight.exe
```

**Important**: Run as administrator to test full functionality, as the app requires admin privileges to modify DNS settings.

### 4. Download Visual C++ Redistributable

Before creating the installer, download the Visual C++ Redistributable:

1. Download **VC++ 2015-2022 Redistributable (x64)** from:
   - Direct link: https://aka.ms/vs/17/release/vc_redist.x64.exe
   - Or visit: https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist

2. Create a `redist` folder in the project root:
   ```powershell
   mkdir redist
   ```

3. Place the downloaded file in the `redist` folder:
   ```
   redist\vc_redist.x64.exe
   ```

**Note**: The installer will automatically check if VC++ Redistributable is installed and install it silently if needed.

### 5. Create the Installer

1. Open **Inno Setup Compiler**
2. Open the file `dnsight.iss` from the project root
3. Click **Build > Compile** (or press F9)
4. The installer will be created in the `installer/` directory as:
   ```
   installer\DNsight-Setup-1.0.0.exe
   ```

### 6. Test the Installer

1. Run the installer executable
2. Complete the installation process
3. Launch the application from the Start Menu or desktop shortcut
4. Verify all features work correctly

### 7. Create Release Package

Create a release package containing:
- `DNsight-Setup-1.0.0.exe` (the installer)
- `README.md` (optional, for distribution)

## Version Updates

When releasing a new version:

### Quick Method (Recommended)

Use the automated version update script:

```powershell
.\update-version.ps1 -Version "1.1.0"
```

This automatically updates all version numbers in:
- `Cargo.toml`
- `dnsight.iss`
- `README.md` (if version section exists)

### Manual Method

1. Update version in `Cargo.toml`:
   ```toml
   version = "1.1.0"
   ```

2. Update version in `dnsight.iss`:
   ```iss
   AppVersion=1.1.0
   OutputBaseFilename=DNsight-Setup-1.1.0
   ```

3. Update version in `README.md` (if mentioned)

4. Rebuild and create installer following steps above

**See `UPDATE.md` for complete update and publishing guide.**

## Distribution

- Upload the installer to your release platform (GitHub Releases, website, etc.)
- Include release notes describing new features or fixes
- Tag the release in Git if using version control

