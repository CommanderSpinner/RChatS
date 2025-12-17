
<#
"platform" specifies the build target
"clean" decides if you want to cargo clean or just build again
"package" if true packages the compiled bin into a zip file with everything needed in it an the other folders removed
#>
param(
    [string]$platform,
    [bool]$clean
    [bool]$package
)

if ()
