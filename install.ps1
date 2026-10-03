param(
    [string]$BinaryPath,
    [string]$InstallDir = (Join-Path $HOME '.local/bin')
)

$ErrorActionPreference = 'Stop'

if (-not $BinaryPath) {
    $BinaryPath = Join-Path $PSScriptRoot 'termarius.exe'
    if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
        $BinaryPath = Join-Path $PSScriptRoot 'target/release/termarius.exe'
    }
}

if (-not (Test-Path -LiteralPath $BinaryPath -PathType Leaf)) {
    throw 'No prebuilt Termarius binary found. Place termarius.exe beside this script or pass -BinaryPath.'
}

New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
$destination = Join-Path $InstallDir 'termarius.exe'
Copy-Item -LiteralPath $BinaryPath -Destination $destination -Force
Write-Host "`nInstalled Termarius: $destination"

if (($env:PATH -split [IO.Path]::PathSeparator) -contains $InstallDir) {
    Write-Host 'Run: termarius'
} else {
    Write-Host "Add $InstallDir to your user PATH to run termarius."
}
