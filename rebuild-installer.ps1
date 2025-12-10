Write-Host "Rebuilding DNsight installer..." -ForegroundColor Green
Write-Host ""
Write-Host "IMPORTANT: You need to rebuild the installer using Inno Setup Compiler" -ForegroundColor Yellow
Write-Host ""
Write-Host "Steps:" -ForegroundColor Cyan
Write-Host "1. Open Inno Setup Compiler" -ForegroundColor White
Write-Host "2. Open the file: dnsight.iss" -ForegroundColor White
Write-Host "3. Click Build > Compile (or press F9)" -ForegroundColor White
Write-Host "4. The installer will be created in: installer\DNsight-Setup-1.0.0.exe" -ForegroundColor White
Write-Host ""
Write-Host "Current file status:" -ForegroundColor Cyan
Write-Host "  Logo:        $(Get-Item 'asset\logo.ico' | Select-Object -ExpandProperty LastWriteTime)" -ForegroundColor White
Write-Host "  Executable:  $(Get-Item 'target\release\dnsight.exe' | Select-Object -ExpandProperty LastWriteTime)" -ForegroundColor White
Write-Host "  ZIP:         $(Get-Item 'DNsight-1.0.0-portable.zip' | Select-Object -ExpandProperty LastWriteTime)" -ForegroundColor White
Write-Host "  Installer:   $(Get-Item 'installer\DNsight-Setup-1.0.0.exe' | Select-Object -ExpandProperty LastWriteTime)" -ForegroundColor White
Write-Host ""
Write-Host "The installer needs to be rebuilt to include the updated executable with the new logo." -ForegroundColor Yellow


