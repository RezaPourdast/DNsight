$version = "1.0.0"
$packageName = "DNsight-$version-portable"
$packageDir = "release-package"

Write-Host "Creating release package..." -ForegroundColor Green

if (Test-Path $packageDir) { Remove-Item $packageDir -Recurse -Force }
if (Test-Path "$packageName.zip") { Remove-Item "$packageName.zip" -Force }

New-Item -ItemType Directory -Force -Path $packageDir | Out-Null

Write-Host "Copying executable..." -ForegroundColor Yellow
Copy-Item "target\release\dnsight.exe" -Destination "$packageDir\DNsight.exe"

Write-Host "Copying assets..." -ForegroundColor Yellow
Copy-Item "asset" -Destination "$packageDir\asset" -Recurse

Write-Host "Creating ZIP archive..." -ForegroundColor Yellow
Compress-Archive -Path "$packageDir\*" -DestinationPath "$packageName.zip" -Force

Remove-Item $packageDir -Recurse -Force

Write-Host "`nRelease package created: $packageName.zip" -ForegroundColor Green
Write-Host "Files ready for GitHub Release:" -ForegroundColor Cyan
Write-Host "  - installer\DNsight-Setup-$version.exe" -ForegroundColor White
Write-Host "  - $packageName.zip" -ForegroundColor White


