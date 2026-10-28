# OmniMem installer (Windows PowerShell)
$ErrorActionPreference = "Stop"

$Repo = "foysalkhan288-rgb/omnimem"
$Version = if ($env:OMNIMEM_VERSION) { $env:OMNIMEM_VERSION } else { "latest" }
$InstallDir = if ($env:OMNIMEM_INSTALL_DIR) { $env:OMNIMEM_INSTALL_DIR } else { "$env:LOCALAPPDATA\omnimem\bin" }

$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$Target = "$Arch-pc-windows-msvc"

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

if ($Version -eq "latest") {
    $Url = "https://github.com/$Repo/releases/latest/download/omnimem-$Target.zip"
} else {
    $Url = "https://github.com/$Repo/releases/download/v$Version/omnimem-$Target.zip"
}

Write-Host "Downloading OmniMem $Version for $Target..."
$tmp = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
try {
    $zip = Join-Path $tmp "omnimem.zip"
    Invoke-WebRequest -Uri $Url -OutFile $zip
    Expand-Archive -Path $zip -DestinationPath $tmp -Force
    Copy-Item (Join-Path $tmp "omnimem.exe") (Join-Path $InstallDir "omnimem.exe") -Force
    Write-Host "Installed to $InstallDir\omnimem.exe"
    & (Join-Path $InstallDir "omnimem.exe") --version
} finally {
    Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}

# Add to PATH hint
if ($env:PATH -notlike "*$InstallDir*") {
    Write-Host "Add $InstallDir to your PATH."
}