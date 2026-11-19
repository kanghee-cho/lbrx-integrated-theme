#Requires -Version 7.0
param(
    [ValidateSet('Light', 'Dark')]
    [string] $Mode = 'Light',
    [switch] $NoLaunch
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot

function Assert-LocalItem([string] $Path, [switch] $Directory) {
    $item = Get-Item -LiteralPath $Path -Force
    if ($item.Attributes -band [IO.FileAttributes]::ReparsePoint -or
        $item.PSIsContainer -ne $Directory.IsPresent) {
        throw "Expected a regular local item: $Path"
    }
}

$dev = Join-Path $root '.dev'
$install = Join-Path $dev 'windows-terminal-1.25.3131.0-x64'
Assert-LocalItem $dev -Directory
Assert-LocalItem $install -Directory
$executables = @(Get-ChildItem -LiteralPath $install -Recurse -Filter WindowsTerminal.exe -File)
if ($executables.Count -ne 1) {
    throw "Expected one WindowsTerminal.exe in $install; see Docs\06-windows-terminal.md."
}
$exe = $executables[0]
for ($dir = $exe.Directory; $dir.FullName -ne $install; $dir = $dir.Parent) {
    Assert-LocalItem $dir.FullName -Directory
}
Assert-LocalItem $exe.FullName
Assert-LocalItem (Join-Path $exe.DirectoryName '.portable')

$cargo = Get-Command cargo -CommandType Application -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    $cargo = Get-Command (Join-Path $HOME '.cargo\bin\cargo.exe') -CommandType Application
}
Push-Location -LiteralPath $root
try {
    & $cargo.Source run --locked -- build
    if ($LASTEXITCODE -ne 0) { throw "Theme build failed (exit $LASTEXITCODE)." }
} finally {
    Pop-Location
}

$output = Join-Path $root 'dist\lbrx\windows-terminal'
$schemes = (Get-Content -LiteralPath (Join-Path $output 'schemes.json') -Raw | ConvertFrom-Json).schemes
$font = (Get-Content -LiteralPath (Join-Path $output 'profile-font.json') -Raw | ConvertFrom-Json).font
$sample = Join-Path $PSScriptRoot 'windows-terminal-sample.ps1'
$shell = (Get-Process -Id $PID).Path
$profileIds = @{
    Light = '{02b11b9b-259f-4bb3-8a73-cd4737f6a982}'
    Dark = '{b2b49187-6653-4f87-8698-e770aa5b7d93}'
}
$config = [ordered]@{
    '$schema' = 'https://aka.ms/terminal-profiles-schema'
    defaultProfile = $profileIds[$Mode]
    profiles = @{
        list = @(foreach ($variant in 'Light', 'Dark') {
            $name = "LBRX $variant"
            if (@($schemes | Where-Object name -CEQ $name).Count -ne 1) {
                throw "Expected one generated scheme named $name."
            }
            [ordered]@{
                guid = $profileIds[$variant]
                name = "$name (dev)"
                colorScheme = $name
                font = $font
                commandline = '"{0}" -NoLogo -NoProfile -NoExit -File "{1}"' -f $shell, $sample
            }
        })
    }
    schemes = $schemes
}

$settingsDir = Join-Path $exe.DirectoryName 'settings'
if (-not (Test-Path -LiteralPath $settingsDir)) {
    New-Item -ItemType Directory -Path $settingsDir | Out-Null
}
Assert-LocalItem $settingsDir -Directory
$settings = Join-Path $settingsDir 'settings.json'
if (Test-Path -LiteralPath $settings) { Assert-LocalItem $settings }
$temporary = Join-Path $settingsDir ('.lbrx-' + [guid]::NewGuid() + '.tmp')
try {
    $config | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath $temporary -Encoding utf8
    [IO.File]::Move($temporary, $settings, $true)
} finally {
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary }
}
Write-Output "Portable settings: $settings"
if (-not $NoLaunch) {
    Start-Process -FilePath $exe.FullName -WorkingDirectory $root -ArgumentList "-w new new-tab -p $($profileIds[$Mode])"
}
