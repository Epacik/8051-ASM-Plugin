param (
    [Parameter(HelpMessage="Increments a Major version number after building vsix")]
    [switch]$IncrementMajor,

    [Parameter(HelpMessage="Increments a Minor version number after building vsix")]
    [switch]$IncrementMinor,

    [Parameter(HelpMessage="Stops script from incrementing a Patch version number after building vsix")]
    [switch]$DontIncrementPatch,

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

if (-Not ($DontRebuildServer)) {
    #build server
    Invoke-Expression -Command "$PSScriptRoot/../../../scripts/buildReleaseServer.ps1 $(if($Clean) { "-Clean" })"
}

$binaries = @(
    @("x86_64-pc-windows-gnu", "lsp_server_8051_asm.exe", "win32-x64"), # windows 64 bit
    @("i686-pc-windows-gnu", "lsp_server_8051_asm.exe", "win32-ia32"),    # windows 32 bit
    @("x86_64-unknown-linux-gnu", "lsp_server_8051_asm", "linux-x64")       # linux 64 bit
)

# build vsix
foreach($bins in $binaries) {
    $dir    = $bins[0]
    $bin    = $bins[1]
    $target = $bins[2]
    Copy-Item "$PSScriptRoot/../../../server/lsp_server_8051_asm/target/$dir/release/$bin" -Destination "$PSScriptRoot/out/bin/$bin" 

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
if (-Not ($DontIncrementPatch)) { $patch = ($patch + 1) }

$version = New-Object System.Version($major, $minor, $patch)
$version = ($version.ToString(3))
Write-Output "new version: $version"

$package.version = $version

$package = $package | ConvertTo-Json -Depth 100
$package | Out-File -FilePath "$PSScriptRoot/package.json" -Force
