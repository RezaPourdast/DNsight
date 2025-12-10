Write-Host "Clearing Windows Icon Cache..." -ForegroundColor Green
Write-Host ""

$iconCachePath = "$env:LOCALAPPDATA\IconCache.db"
$iconCacheDir = "$env:LOCALAPPDATA\Microsoft\Windows\Explorer"

Write-Host "Attempting to clear icon cache..." -ForegroundColor Yellow

# Stop Windows Explorer temporarily
Write-Host "Stopping Windows Explorer..." -ForegroundColor Yellow
Stop-Process -Name "explorer" -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

# Delete icon cache files
if (Test-Path $iconCachePath) {
    Remove-Item $iconCachePath -Force -ErrorAction SilentlyContinue
    Write-Host "Deleted: $iconCachePath" -ForegroundColor Green
}

# Clear icon cache directory
if (Test-Path $iconCacheDir) {
    Get-ChildItem -Path $iconCacheDir -Filter "iconcache*" -ErrorAction SilentlyContinue | Remove-Item -Force -ErrorAction SilentlyContinue
    Write-Host "Cleared icon cache files in: $iconCacheDir" -ForegroundColor Green
}

# Restart Windows Explorer
Write-Host "Restarting Windows Explorer..." -ForegroundColor Yellow
Start-Process "explorer.exe"

Write-Host ""
Write-Host "Icon cache cleared! Please restart the DNsight application." -ForegroundColor Green
Write-Host "The new icon should now appear in Alt+Tab." -ForegroundColor Cyan

