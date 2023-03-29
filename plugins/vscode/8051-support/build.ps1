param (
    [Parameter(HelpMessage="Increments a Major version number after building vsix")]
    [switch]$IncrementMajor,

    [Parameter(HelpMessage="Increments a Minor version number after building vsix")]
    [switch]$IncrementMinor,

    [Parameter(HelpMessage="Stops script from incrementing a Patch version number after building vsix")]
    [switch]$IncrementPatch,

    [Parameter(HelpMessage="Stops script from rebuilding a LSP Server")]
    [switch]$DontRebuildServer,

    [Parameter(HelpMessage="Cleans the server target dir")]
    [switch]$Clean
)

Set-Location "$PSScriptRoot"

$package = Get-Content "$PSScriptRoot/package.json"
$package = $package | ConvertFrom-Json

$version = $package.version

# create dirs or remove old files
foreach($dir in @("$PSScriptRoot/out/bin", "$PSScriptRoot/../out")) {
    New-item -ItemType Directory -Path $dir -Force

    foreach($file in Get-ChildItem -Path $dir -Name) {
        Remove-Item "$dir/$file"
    }
}

Invoke-Expression -Command "yarn"
Invoke-Expression -Command "$PSScriptRoot/node_modules/typescript/bin/tsc"
if (Test-Path -Path "$PSScriptRoot/out/documentation/views/styles") {
    Remove-item -Recurse -Force "$PSScriptRoot/out/documentation/views/styles"
}
Copy-Item -Recurse -Path "$PSScriptRoot/src/documentation/views/styles" -Destination "$PSScriptRoot/out/documentation/views/styles"


if (-Not ($DontRebuildServer)) {
    #build server
    Invoke-Expression -Command "$PSScriptRoot/../../../scripts/buildReleaseServer.ps1 -BuildFor Linux64,Windows32,Windows64 $(if($Clean) { "-Clean" })" -Verbose
}

$binaries = @(
    @("x86_64-pc-windows-gnu", "asm8051_lsp.exe", "win32-x64"), # windows 64 bit
    @("i686-pc-windows-gnu", "asm8051_lsp.exe", "win32-ia32"),    # windows 32 bit
    @("x86_64-unknown-linux-gnu", "asm8051_lsp", "linux-x64")       # linux 64 bit
)

# build vsix
foreach($bins in $binaries) {
    $dir    = $bins[0]
    $bin    = $bins[1]
    $target = $bins[2]
    Copy-Item "$PSScriptRoot/../../../server/asm8051_lsp/target/$dir/release/$bin" -Destination "$PSScriptRoot/out/bin/$bin" 

    Invoke-Expression -Command "vsce.ps1 package --target $target --pre-release --out `"$PSScriptRoot/../out/asm8051_$target-$version.vsix`""

    Remove-Item "$PSScriptRoot/out/bin/$bin" 
}

Write-Output "current version: $version"
$version = [System.Version]::Parse([string]$version)
$major = $version.Major
$minor = $version.Minor
$patch = $version.Build

if ($IncrementMajor) { $major = ($major + 1) }
if ($IncrementMinor) { $minor = ($minor + 1) }
if ($IncrementPatch) { $patch = ($patch + 1) }

$version = New-Object System.Version($major, $minor, $patch)
$version = ($version.ToString(3))
Write-Output "new version: $version"

$package.version = $version

$package = $package | ConvertTo-Json -Depth 100
$package | Out-File -FilePath "$PSScriptRoot/package.json" -Force
