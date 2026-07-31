# Official AIR.SKILLS Installation Script for Windows PowerShell
$ErrorActionPreference = "Stop"

$Repo = "Chethankumar443/AIR-SKILLS"
$Version = "1.0.0"
$Target = "x86_64-pc-windows-msvc"

Write-Host "────────────────────────────────────────────────" -ForegroundColor Cyan
Write-Host "        AIR.SKILLS Windows Installer (v$Version)" -ForegroundColor Cyan
Write-Host "────────────────────────────────────────────────" -ForegroundColor Cyan

$InstallDir = "$env:LOCALAPPDATA\AIR.SKILLS\bin"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$ZipUrl = "https://github.com/$Repo/releases/download/v$Version/air-$Target.zip"
$TempZip = "$env:TEMP\air-$Target.zip"

Write-Host "Downloading AIR CLI binary..." -ForegroundColor Yellow
Invoke-WebRequest -Uri $ZipUrl -OutFile $TempZip

Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item -Path $TempZip -Force

# Add to User PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::GetEnvironmentVariable("Path", "User") + ";$InstallDir" | Set-Content env:Path
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "✓ Added $InstallDir to User PATH." -ForegroundColor Green
}

Write-Host "────────────────────────────────────────────────" -ForegroundColor Cyan
Write-Host " ✓ AIR.SKILLS CLI successfully installed to $InstallDir\air.exe" -ForegroundColor Green
Write-Host " Restart PowerShell or terminal and run 'air --help'." -ForegroundColor Green
Write-Host "────────────────────────────────────────────────" -ForegroundColor Cyan
