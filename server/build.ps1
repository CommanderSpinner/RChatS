#!/usr/bin/env pwsh

<#
"clean" decides if you want to cargo clean or just build again
"package" if true packages the compiled bin into a zip file with everything needed in it an the other folders removed
#>

param(
    [switch]$clean,
    [switch]$package,
    [String]$platform="x86_64-unknown-linux-gnu"
)

$ErrorActionPreference = 'Stop'

# Assign defaults if not provided
if ($null -eq $clean) { $clean = $false }
if ($null -eq $package) { $package = $false }

Set-Location $PSScriptRoot
$Server_root = Get-Location

#getting "target" dir from cargo
Set-Location ..
mkdir target
Set-Location target
if($package) {
    $target_dir = "$(Get-Location)/$platform/release"
} else {
    $target_dir = "$(Get-Location)/$platform/debug"
}
#getting dir of platform target
mkdir $Platform
Set-Location $Platform
if($package){
    mkdir release
    Set-Location release
} else {
    mkdir debug
    Set-Location debug
}

$platform_binary = Get-Location

#return to server root
Set-Location $Server_root

Write-Output "server root: $($Server_root)"
Write-Output "platform target dir: $($platform_binary)"

if($clean) {
    Write-Output "cleaning"
    cargo clean
}

rustup target add $Platform 
Write-Output ("building for: {0}" -f $Platform)

#check for debugg or release build
if($package) {
    Write-Output "release build"
    cargo build --release --target $Platform 
} else {
    Write-Output "debug build"
    cargo build --target $Platform
}

# copy files to target, excluding db_server(its only files for debuging)
Write-Output "Copying data folder..."

$data_source = Join-Path $Server_root "data"
$data_target = Join-Path $target_dir "data"

if (Test-Path $data_source) {
    # Create target data directory
    New-Item -ItemType Directory -Path $data_target -Force | Out-Null

    # Copy everything recursively except db_server
    Get-ChildItem -Path $data_source -Force |
        Where-Object { $_.Name -ne "db_server" } |
        Copy-Item -Destination $data_target -Recurse -Force

    Write-Output "Data copied to: $data_target"
    Write-Output "Excluded: $data_source/db_server"
} else {
    Write-Output "No data directory found at: $data_source"
}


# also need to start db server and do cleaning of it - -- - - - -- - -


if($package){
    #unfinnished
} else {
    Set-Location $target_dir
    Start-Process "server"
}

Set-Location $server_root
