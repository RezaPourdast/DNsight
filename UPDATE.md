# Update and Publishing Guide

This guide explains how to update DNsight and publish new versions.

## Quick Update Workflow

1. **Update Version Numbers** (see below)
2. **Make Your Code Changes**
3. **Build and Test**
4. **Create Installer**
5. **Publish Release**

---

## Step-by-Step Update Process

### 1. Update Version Numbers

You need to update the version in **3 places**:

#### A. Update `Cargo.toml`
```toml
[package]
version = "1.1.0"  # Change from 1.0.0 to your new version
```

#### B. Update `dnsight.iss`
```iss
[Setup]
AppVersion=1.1.0
OutputBaseFilename=DNsight-Setup-1.1.0
```

#### C. Update `README.md` (if version is mentioned)
```markdown
## Version
1.1.0
```

**Tip**: Use the provided PowerShell script `update-version.ps1` to automate this!

### 2. Make Your Code Changes

- Add new features
- Fix bugs
- Update dependencies if needed
- Test thoroughly

### 3. Build and Test

```powershell
# Clean previous builds (optional)
cargo clean

# Build release version
cargo build --release

# Test the executable
.\target\release\dnsight.exe
```

**Important**: Always test as administrator to verify DNS modification works!

### 4. Create the Installer

```powershell
# Make sure you have vc_redist.x64.exe in redist\ folder
# Then open Inno Setup Compiler and compile dnsight.iss
# Or use command line:
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" dnsight.iss
```

The installer will be created at:
```
installer\DNsight-Setup-1.1.0.exe
```

### 5. Test the Installer

1. Run the installer on a test machine (or VM)
2. Verify installation works
3. Test all features
4. Check that uninstaller works

### 6. Publish the Release

Choose your publishing platform:

#### Option A: GitHub Releases (Recommended)

1. **Commit your changes**:
   ```powershell
   git add .
   git commit -m "Release v1.1.0"
   git tag v1.1.0
   git push origin main
   git push origin v1.1.0
   ```

2. **Create GitHub Release**:
   - Go to your repository on GitHub
   - Click "Releases" → "Draft a new release"
   - Tag: `v1.1.0`
   - Title: `DNsight v1.1.0`
   - Upload: `installer\DNsight-Setup-1.1.0.exe`
   - Add release notes (see below)

#### Option B: Direct Distribution

- Upload to your website
- Share via cloud storage (Google Drive, Dropbox, etc.)
- Distribute via email or other channels

---

## Version Numbering Guide

Follow [Semantic Versioning](https://semver.org/): `MAJOR.MINOR.PATCH`

- **MAJOR** (1.0.0 → 2.0.0): Breaking changes, major rewrites
- **MINOR** (1.0.0 → 1.1.0): New features, backward compatible
- **PATCH** (1.0.0 → 1.0.1): Bug fixes, small improvements

Examples:
- `1.0.0` → `1.0.1` (bug fix)
- `1.0.0` → `1.1.0` (new feature)
- `1.0.0` → `2.0.0` (major change)

---

## Release Notes Template

When publishing, include release notes:

```markdown
## DNsight v1.1.0

### New Features
- Added feature X
- Improved Y

### Bug Fixes
- Fixed issue with Z
- Resolved crash when...

### Changes
- Updated dependency versions
- Performance improvements

### Installation
1. Download `DNsight-Setup-1.1.0.exe`
2. Run installer (requires admin)
3. Follow installation wizard

### Upgrade from Previous Version
- Uninstall old version first (optional, installer will overwrite)
- Run new installer
- Your saved DNS entries are preserved
```

---

## Automated Version Update Script

Use `update-version.ps1` to automatically update all version numbers:

```powershell
.\update-version.ps1 -Version "1.1.0"
```

This will update:
- `Cargo.toml`
- `dnsight.iss`
- `README.md` (if version section exists)

---

## Pre-Release Checklist

Before publishing, verify:

- [ ] Version numbers updated in all files
- [ ] Code changes committed to git
- [ ] Release build tested and working
- [ ] Installer tested on clean Windows system
- [ ] All features work correctly
- [ ] Release notes prepared
- [ ] Installer file ready (`installer\DNsight-Setup-X.X.X.exe`)
- [ ] Git tag created (if using version control)

---

## Post-Release Tasks

After publishing:

- [ ] Monitor for user feedback
- [ ] Track download statistics
- [ ] Document any issues found
- [ ] Plan next version features

---

## Distribution Platforms

### GitHub Releases
- **Pros**: Free, integrated with git, version history
- **Cons**: Requires GitHub account
- **Best for**: Open source projects

### Direct Website
- **Pros**: Full control, custom branding
- **Cons**: Need hosting, bandwidth costs
- **Best for**: Professional/commercial apps

### Other Options
- **SourceForge**: Free hosting for open source
- **GitLab Releases**: Similar to GitHub
- **Microsoft Store**: Requires certification (not recommended for admin tools)

---

## Troubleshooting

### Installer won't compile
- Check that `redist\vc_redist.x64.exe` exists
- Verify Inno Setup is installed correctly
- Check for syntax errors in `dnsight.iss`

### Version mismatch errors
- Ensure all 3 version locations are updated
- Use `update-version.ps1` script to avoid mistakes

### Users can't install
- Verify VC++ Redistributable is included
- Check Windows version compatibility (Windows 7 SP1+)
- Ensure installer requires admin privileges

---

## Example: Updating from 1.0.0 to 1.1.0

```powershell
# 1. Update version (automated)
.\update-version.ps1 -Version "1.1.0"

# 2. Make code changes
# ... edit your code ...

# 3. Build
cargo build --release

# 4. Test
.\target\release\dnsight.exe

# 5. Create installer
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" dnsight.iss

# 6. Commit and tag
git add .
git commit -m "Release v1.1.0: Added new features"
git tag v1.1.0
git push origin main --tags

# 7. Upload installer to GitHub Releases
```

---

## Need Help?

- Check `RELEASE.md` for build instructions
- Review `README.md` for user documentation
- Test on multiple Windows versions before release

