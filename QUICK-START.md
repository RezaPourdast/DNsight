# Quick Start Guide - Updating & Publishing DNsight

## 🚀 Quick Update Workflow

### 1. Update Version (Automated)
```powershell
.\update-version.ps1 -Version "1.1.0"
```

### 2. Build & Create Installer
```powershell
.\build-release.ps1
```

### 3. Publish (GitHub Example)
```powershell
git add .
git commit -m "Release v1.1.0"
git tag v1.1.0
git push origin main --tags
# Then create GitHub Release and upload installer
```

---

## 📋 Complete Update Checklist

- [ ] Update version: `.\update-version.ps1 -Version "X.Y.Z"`
- [ ] Make code changes
- [ ] Build: `.\build-release.ps1`
- [ ] Test the executable
- [ ] Test the installer
- [ ] Commit changes: `git commit -m "Release vX.Y.Z"`
- [ ] Create git tag: `git tag vX.Y.Z`
- [ ] Push: `git push origin main --tags`
- [ ] Create release on GitHub/your platform
- [ ] Upload installer file
- [ ] Write release notes

---

## 📝 Version Numbering

Use Semantic Versioning: `MAJOR.MINOR.PATCH`

- **1.0.0 → 1.0.1**: Bug fix
- **1.0.0 → 1.1.0**: New feature
- **1.0.0 → 2.0.0**: Major change

---

## 📚 More Information

- **Full Update Guide**: See `UPDATE.md`
- **Build Instructions**: See `RELEASE.md`
- **User Documentation**: See `README.md`

---

## 🛠️ Helper Scripts

| Script | Purpose |
|--------|---------|
| `update-version.ps1` | Updates version in all files |
| `build-release.ps1` | Builds release and creates installer |

---

## ⚠️ Important Notes

- Always test as administrator
- Ensure `redist\vc_redist.x64.exe` exists before building installer
- Test installer on clean Windows system before publishing
- Keep release notes clear and helpful

