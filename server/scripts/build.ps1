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

cd ..
$Server_root = Get-Location

#getting "target" dir from cargo
cd ..
cd target
$target_dir = Get-Location

#getting dir of platform target
mkdir $Platform
cd $Platform
if($package){
    mkdir release
    cd release
} else {
    mkdir debug
    cd debug
}

$platform_binary = Get-Location

#return to server root
cd $Server_root

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

# still need to copy files to folders
# also need to start db server and do cleaning of it

# return to scripts folder
cd $server_root
cd scripts
