# xfetch installer script for Windows

Write-Host "Installing xfetch..." -ForegroundColor Cyan

# Check for Rust/Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust (cargo) is not installed." -ForegroundColor Red
    Write-Host "Please install Rust from https://rustup.rs/ and try again." -ForegroundColor Yellow
    exit 1
}

$RepoUrl = "https://github.com/xscriptordev/xfetch.git"
$TempDir = Join-Path $env:TEMP "xfetch_install"

# Clean previous temp dir
if (Test-Path $TempDir) {
    Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue
}
New-Item -Path $TempDir -ItemType Directory | Out-Null

# Clone
Write-Host "Cloning repository..." -ForegroundColor Cyan
try {
    git clone --depth 1 $RepoUrl $TempDir
}
catch {
    Write-Host "Failed to clone repository. Ensure git is installed." -ForegroundColor Red
    exit 1
}

# Install
Set-Location $TempDir
Write-Host "Building and installing xfetch..." -ForegroundColor Cyan
cargo install --path .

# Config
$ConfigDir = Join-Path $env:APPDATA "xfetch"
if (-not (Test-Path $ConfigDir)) {
    Write-Host "Setting up default configuration..." -ForegroundColor Cyan
    New-Item -Path $ConfigDir -ItemType Directory | Out-Null
    
    # Copy config
    Copy-Item "configs\config_11_pacman.jsonc" (Join-Path $ConfigDir "config.jsonc")
    
    # Copy logos
    $LogosDir = Join-Path $ConfigDir "logos"
    New-Item -Path $LogosDir -ItemType Directory | Out-Null
    Copy-Item "logos\*" $LogosDir -Recurse
}

# Cleanup
Set-Location $env:USERPROFILE
Remove-Item -Path $TempDir -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "Installation complete! Run 'xfetch' to start." -ForegroundColor Green
