param (
    [switch]$Clean,
    [Parameter(HelpMessage="For which targets to build")]
    [ValidateSet("Linux64", "Windows32", "Windows64")]
    [string[]]$BuildFor
)


function HasTarget {
    param (
        [Parameter(ValueFromPipeline=$True)] [Targets]$targets,
        [Parameter(Position=0)] [Targets]$flag
    )
    return ($targets -band $flag) -eq $flag
}

[Flags()]
enum Targets {
    None      = 0;
    Linux64   = 1;
    Windows32 = 2;
    Windows64 = 4;
}

$targets = [Targets]::None
foreach ($target in $BuildFor) {
    $targets += [Targets]$target
}

$location = Get-Location

Set-Location "$PSScriptRoot"

if($Clean){
    Invoke-Expression -Command "cargo clean"
}

if ($targets | HasTarget Windows64 ) {
    Write-Output "Building for x86_64-pc-windows-gnu"
    Set-Location "$PSScriptRoot/load_documentation"
    Invoke-Expression -Command "cargo build --release --target x86_64-pc-windows-gnu"
    Set-Location "$PSScriptRoot"
    Invoke-Expression -Command "cargo build --release --target x86_64-pc-windows-gnu"
    Write-Output ""
    Write-Output ""
}

if ($targets | HasTarget Windows32) {
    Write-Output "Building for i686-pc-windows-gnu"
    Set-Location "$PSScriptRoot/load_documentation"
    Invoke-Expression -Command "cargo build --release --target i686-pc-windows-gnu"
    Set-Location "$PSScriptRoot"
    Invoke-Expression -Command "cargo build --release --target i686-pc-windows-gnu"
    Write-Output ""
    Write-Output ""
}

if ($targets | HasTarget Linux64) {
    Write-Output "Building for x86_64-unknown-linux-gnu"
    Set-Location "$PSScriptRoot/load_documentation"
    Invoke-Expression -Command "cargo build --release --target x86_64-unknown-linux-gnu"
    Set-Location "$PSScriptRoot"
    Invoke-Expression -Command "cargo build --release --target x86_64-unknown-linux-gnu"
    Write-Output ""
    Write-Output ""
}

Set-Location "$location"