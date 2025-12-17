
<#
"clean" decides if you want to cargo clean or just build again
"package" if true packages the compiled bin into a zip file with everything needed in it an the other folders removed
#>

param(
    [bool]$clean,
    [bool]$package
)

cd ..
$Server_root = Get-Location
Write-Output "server root: $($Server_root)"

if($clean) {
    cargo clean
}

#check what platform

$Platform = "x86_64-unknown-linux-gnu" #default is linux
if ($PSVersionTable.Platform -eq 'Win32NT') {
    $Platform = x86_64-pc-windows-msvc
}
rustup target add $Platform
Write-Output "building for: $(Platform)"

#check for debugg or release build
if($package) {
    cargo build --release --target $Platform
} else {
    cargo build --target $Platform
}

# still need to copy files to folders
# also need to start db server and do cleaning of it

# return to scripts folder
cd $server_root
cd scripts
