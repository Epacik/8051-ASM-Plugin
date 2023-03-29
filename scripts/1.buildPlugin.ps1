param (
    [Parameter(HelpMessage="Increments a Major version number")]
    [switch]$IncrementMajor,

    [Parameter(HelpMessage="Increments a Minor version number")]
    [switch]$IncrementMinor,

    [Parameter(HelpMessage="Increments a Patch version number")]
    [switch]$IncrementPatch,

    [Parameter(HelpMessage="Stops script from rebuilding a LSP Server")]
    [switch]$DontRebuildServer,

    [Parameter(HelpMessage="Build release version")]
    [switch]$Release,

    [Parameter(HelpMessage="For which targets to build")]
    [ValidateSet("Linux64", "Windows32", "Windows64")]
    [string[]]$Targets,

    [Parameter(HelpMessage="Use cargo instead of cross")]
    [switch]$UseCargo,

    [Parameter(HelpMessage="Publish packages")]
    [switch]$Publish
)

function Remove-AllItems ([string] $directory) {
    foreach($file in Get-ChildItem -Path $directory -Name) {
        Remove-Item "$directory/$file" -Force -Recurse
    }
}

$vscodePluginDir = "$PSScriptRoot/../plugins/vscode/8051-support";
$vscodeOutDir = "$vscodePluginDir/out"
$serverDir = "$PSScriptRoot/../server/asm8051_lsp";
$outDir = "$PSScriptRoot/out";

[Flags()]
enum Targets {
    None      = 0;
    Linux64   = 1;
    Windows32 = 2;
    Windows64 = 4;

    All       = (1 + 2 + 4);
}

$targetsToBuild = if ($null -eq $Targets || $Targets.Count -eq 0) {
    [Targets]::All;
}
else {
    [Targets]$Targets;
}

Set-Location "$PSScriptRoot";

$packageInfoPath = "$vscodePluginDir/package.json";
$packageInfo = Get-Content $packageInfoPath | ConvertFrom-Json;
$version = $packageInfo.version;

#region update version number

Write-Output "Updating version number"

# get version object
$version = [System.Version]::Parse([string]$version);
$major = $version.Major;
$minor = $version.Minor;
$patch = $version.Build;

# inc
if ($IncrementMajor) { 
    $major = ($major + 1);
    $minor = 0;
    $patch = 0;
}
if ($IncrementMinor) { 
    $minor = ($minor + 1) 
    $patch = 0;
}
if ($IncrementPatch) { $patch = ($patch + 1) }

# convert back to string
$version = (New-Object System.Version($major, $minor, $patch)).ToString(3);
Write-Output "new version: $version";

# update version number in file
$packageInfo.version = $version;
$packageInfo = $packageInfo | ConvertTo-Json -Depth 100;
$packageInfo | Out-File -FilePath $packageInfoPath -Force;
#endregion


Write-Output "Creating or cleaning output directories"
#create output dir
New-item -ItemType Directory -Path $outDir -Force
Remove-AllItems $outDir

#create vscode plugin out dir
New-item -ItemType Directory -Path $vscodeOutDir -Force
Remove-AllItems $vscodeOutDir
New-item -ItemType Directory -Path "$vscodeOutDir/bin" -Force

#region build typescript

Write-Output "Building typescript for plugin, and copying styles"

Set-Location $vscodePluginDir
npm install
tsc
Copy-Item -Recurse -Path "$vscodePluginDir/src/documentation/views/styles" -Destination "$vscodePluginDir/out/documentation/views/styles"
Set-Location "$PSScriptRoot";

#endregion

#region build server and make vsix's

Write-Output "Building language server, and packaging for $targetsToBuild"
$buildBin = if ($UseCargo) { "cargo" } else { "cross" };
$targetDir = if ($Release) { "release" } else { "debug" };

$binaries = @(
    @([Targets]::Windows64, "x86_64-pc-windows-gnu", "asm8051_lsp.exe", "win32-x64"),
    @([Targets]::Windows32, "i686-pc-windows-gnu", "asm8051_lsp.exe", "win32-ia32"),
    @([Targets]::Linux64,   "x86_64-unknown-linux-gnu", "asm8051_lsp", "linux-x64")
);

Set-Location $serverDir

foreach ($binary in $binaries) {
    $target, $osTriple, $exeName, $os, $_ = $binary;
    
    if ($targetsToBuild.HasFlag($target)) {
        
        Set-Location $serverDir

        Write-Output "Cleaning server"
        &"$buildBin" clean

        Write-Output "Building for $osTriple";
        if ($Release) {
            &"$buildBin" build --release --target $osTriple;
        }
        else {
            &"$buildBin" build --target $osTriple;
        }
        Write-Output "`n`n";

        Set-Location $vscodePluginDir

        Write-Host "`n Exe path: $serverDir/target/$osTriple/$targetDir/$exeName"

        Copy-Item "$serverDir/target/$osTriple/$targetDir/$exeName" -Destination "$vscodePluginDir/out/bin/" -Force
        $outPath = "$outDir/asm8051_$os-$version.vsix"
        vsce package --target $os --pre-release --out $outPath
        Remove-Item "$vscodePluginDir/out/bin/$exeName"
        
        if ($Publish) {
            Write-Host "Publishing for $os"
            vsce publish --packagePath $outPath --no-update-package-json --no-git-tag-version --noVerify
        }
    }
}

Set-Location $PSScriptRoot
#endregion