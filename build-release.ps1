# DNsight Release Build Script
# Automates the build and installer creation process

param(
    [switch]$Clean,
    [switch]$SkipTest,
    [string]$InnoSetupPath = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
)

Write-Host "=== DNsight Release Build ===" -ForegroundColor Cyan
Write-Host ""

# Check if Inno Setup is available
if (-not (Test-Path $InnoSetupPath)) {
    Write-Host "Warning: Inno Setup not found at: $InnoSetupPath" -ForegroundColor Yellow
    Write-Host "You may need to install Inno Setup or specify the path manually." -ForegroundColor Yellow
    Write-Host ""
}

# Step 1: Clean (optional)
if ($Clean) {
    Write-Host "[1/5] Cleaning previous builds..." -ForegroundColor Yellow
    cargo clean
    Write-Host "✓ Clean complete" -ForegroundColor Green
    Write-Host ""
}

# Step 2: Build release
Write-Host "[2/5] Building release binary..." -ForegroundColor Yellow
$buildResult = cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✓ Build successful" -ForegroundColor Green
Write-Host ""

# Step 3: Check if executable exists
$exePath = "target\release\dnsight.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "✗ Executable not found at $exePath" -ForegroundColor Red
    exit 1
}

# Step 4: Test (optional)
if (-not $SkipTest) {
    Write-Host "[3/5] Testing executable..." -ForegroundColor Yellow
    Write-Host "Note: Run as administrator to test DNS functionality" -ForegroundColor Cyan
    Write-Host "Press any key to continue (or Ctrl+C to cancel)..." -ForegroundColor Cyan
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    Write-Host ""
}

# Step 5: Check for VC++ Redistributable
Write-Host "[4/5] Checking prerequisites..." -ForegroundColor Yellow
$redistPath = "redist\vc_redist.x64.exe"
if (-not (Test-Path $redistPath)) {
    Write-Host "⚠ Warning: VC++ Redistributable not found at $redistPath" -ForegroundColor Yellow
    Write-Host "  Download from: https://aka.ms/vs/17/release/vc_redist.x64.exe" -ForegroundColor Cyan
    Write-Host "  Place it in the redist\ folder" -ForegroundColor Cyan
    Write-Host ""
} else {
    Write-Host "✓ VC++ Redistributable found" -ForegroundColor Green
    Write-Host ""
}

# Step 6: Create installer
Write-Host "[5/5] Creating installer..." -ForegroundColor Yellow
if (Test-Path $InnoSetupPath) {
    & $InnoSetupPath dnsight.iss
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✓ Installer created successfully!" -ForegroundColor Green
        
        # Get version from Cargo.toml
        $cargoContent = Get-Content "Cargo.toml" -Raw
        if ($cargoContent -match 'version = "([\d\.]+)"') {
            $version = $matches[1]
            $installerName = "installer\DNsight-Setup-$version.exe"
            if (Test-Path $installerName) {
                $fileInfo = Get-Item $installerName
                $sizeMB = [math]::Round($fileInfo.Length / 1MB, 2)
                Write-Host ""
                Write-Host "Installer: $installerName" -ForegroundColor Cyan
                Write-Host "Size: $sizeMB MB" -ForegroundColor Cyan
            }
        }
    } else {
        Write-Host "✗ Installer creation failed!" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "⚠ Inno Setup not found. Please create installer manually:" -ForegroundColor Yellow
    Write-Host "  1. Open Inno Setup Compiler" -ForegroundColor White
    Write-Host "  2. Open dnsight.iss" -ForegroundColor White
    Write-Host "  3. Build > Compile (F9)" -ForegroundColor White
}

Write-Host ""
Write-Host "=== Build Complete ===" -ForegroundColor Cyan

