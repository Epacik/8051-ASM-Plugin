Write-Output "copying server"
try {
    $process = Get-Process -Name "asm8051_lsp" 
    if($null -ne $process){
        Stop-Process -InputObject $process
    }
}
catch {}


if ($IsLinux) {
    
    Copy-Item "$PSScriptRoot/../../../server/asm8051_lsp/target/x86_64-unknown-linux-gnu/release/asm8051_lsp" "$PSScriptRoot/out/bin/asm8051_lsp" 
}
elseif ($IsWindows && [System.Environment]::Is64BitOperatingSystem) {
    Copy-Item "$PSScriptRoot/../../../server/asm8051_lsp/target/x86_64-pc-windows-gnu/release/asm8051_lsp.exe" "$PSScriptRoot/out/bin/asm8051_lsp.exe" 
}
elseif ($IsWindows) {
    Copy-Item "$PSScriptRoot/../../../server/asm8051_lsp/target/i686-pc-windows-gnu/release/asm8051_lsp.exe" "$PSScriptRoot/out/bin/asm8051_lsp.exe" 
}