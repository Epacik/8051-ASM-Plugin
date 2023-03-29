
$binaries = @(
    @("asm8051_linux-x64", "win32-x64"),
    @("asm8051_win32-ia32", "win32-ia32"),
    @("asm8051_win32-x64", "linux-x64")
);

$outPath = "$PSScriptRoot/out"

foreach ($b in $binaries) {
    $name, $os, $_ = $b;


    if (Test-Path "$outPath/$name*") {
        Write-Host "Publishing for $os"
        $item = Get-item -Path "$outPath/$name*"
        if ($null -ne $item) {
            vsce publish --packagePath $item.ResolvedTarget --no-update-package-json --no-git-tag-version --noVerify
        }
    }
}