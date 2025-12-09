# DNsight Version Update Script
# Automatically updates version numbers across all files

param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

# Validate version format (basic check)
if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    Write-Host "Error: Version must be in format X.Y.Z (e.g., 1.1.0)" -ForegroundColor Red
    exit 1
}

Write-Host "Updating version to $Version..." -ForegroundColor Green

# 1. Update Cargo.toml
$cargoFile = "Cargo.toml"
if (Test-Path $cargoFile) {
    $content = Get-Content $cargoFile -Raw
    $content = $content -replace 'version = "[\d\.]+"', "version = `"$Version`""
    Set-Content $cargoFile -Value $content -NoNewline
    Write-Host "✓ Updated $cargoFile" -ForegroundColor Green
} else {
    Write-Host "✗ $cargoFile not found" -ForegroundColor Red
}

# 2. Update dnsight.iss
$issFile = "dnsight.iss"
if (Test-Path $issFile) {
    $content = Get-Content $issFile -Raw
    # Update AppVersion
    $content = $content -replace 'AppVersion=[\d\.]+', "AppVersion=$Version"
    # Update OutputBaseFilename
    $content = $content -replace 'OutputBaseFilename=DNsight-Setup-[\d\.]+', "OutputBaseFilename=DNsight-Setup-$Version"
    Set-Content $issFile -Value $content -NoNewline
    Write-Host "✓ Updated $issFile" -ForegroundColor Green
} else {
    Write-Host "✗ $issFile not found" -ForegroundColor Red
}

# 3. Update README.md (if version section exists)
$readmeFile = "README.md"
if (Test-Path $readmeFile) {
    $content = Get-Content $readmeFile -Raw
    # Check if version section exists
    if ($content -match '## Version') {
        $content = $content -replace '(## Version\s+)\d+\.\d+\.\d+', "`$1$Version"
        Set-Content $readmeFile -Value $content -NoNewline
        Write-Host "✓ Updated $readmeFile" -ForegroundColor Green
    } else {
        Write-Host "⚠ Version section not found in $readmeFile (skipping)" -ForegroundColor Yellow
    }
} else {
    Write-Host "⚠ $readmeFile not found (skipping)" -ForegroundColor Yellow
}

Write-Host "`nVersion update complete!" -ForegroundColor Green
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Review the changes" -ForegroundColor White
Write-Host "  2. Make your code changes" -ForegroundColor White
Write-Host "  3. Build: cargo build --release" -ForegroundColor White
Write-Host "  4. Create installer" -ForegroundColor White

