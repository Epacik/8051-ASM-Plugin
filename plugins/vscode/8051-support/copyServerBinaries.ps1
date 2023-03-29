Write-Output "copying server"
try {
    $process = Get-Process -Name "asm8051_lsp" 
    if($null -ne $process){
        Stop-Process -InputObject $process
    }
}
catch {}

New-Item "$PSScriptRoot/out/bin/" -Force -ItemType Directory

if ($IsLinux) {
    $path = "$PSScriptRoot/../../../server/asm8051_lsp/target/x86_64-unknown-linux-gnu/release/asm8051_lsp";

    if (!(Test-Path $path)){
        $path = "$PSScriptRoot/../../../server/asm8051_lsp/target/release/asm8051_lsp";
    }
    
    Copy-Item $path "$PSScriptRoot/out/bin/asm8051_lsp" 
}
elseif ($IsWindows && [System.Environment]::Is64BitOperatingSystem) {
    $path = "$PSScriptRoot/../../../server/asm8051_lsp/target/x86_64-pc-windows-gnu/release/asm8051_lsp.exe";

    if (!(Test-Path $path)){
        $path = "$PSScriptRoot/../../../server/asm8051_lsp/target/release/asm8051_lsp.exe";
    }

    Copy-Item $path "$PSScriptRoot/out/bin/asm8051_lsp.exe" 
    
}
elseif ($IsWindows) {
    $path = "$PSScriptRoot/../../../server/asm8051_lsp/target/i686-pc-windows-gnu/release/asm8051_lsp.exe";

    if (!(Test-Path $path)){
        $path = "$PSScriptRoot/../../../server/asm8051_lsp/target/release/asm8051_lsp.exe";
    }

    Copy-Item $path "$PSScriptRoot/out/bin/asm8051_lsp.exe" 
}