Write-Output "copying server"
try {
    $process = Get-Process -Name "lsp_server_8051_asm" 
    if($null -ne $process){
        Stop-Process -InputObject $process
    }
}
catch {}


if ($IsLinux) {
    
    Copy-Item "$PSScriptRoot/../../../server/lsp_server_8051_asm/target/x86_64-unknown-linux-gnu/release/lsp_server_8051_asm" "$PSScriptRoot/out/bin/lsp_server_8051_asm" 
}
elseif ($IsWindows && [System.Environment]::Is64BitOperatingSystem) {
    Copy-Item "$PSScriptRoot/../../../server/lsp_server_8051_asm/target/x86_64-pc-windows-gnu/release/lsp_server_8051_asm.exe" "$PSScriptRoot/out/bin/lsp_server_8051_asm.exe" 
}
elseif ($IsWindows) {
    Copy-Item "$PSScriptRoot/../../../server/lsp_server_8051_asm/target/i686-pc-windows-gnu/release/lsp_server_8051_asm.exe" "$PSScriptRoot/out/bin/lsp_server_8051_asm.exe" 
}