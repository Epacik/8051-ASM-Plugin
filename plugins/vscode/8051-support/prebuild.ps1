Set-Location "$PSScriptRoot"
New-item -ItemType Directory -Path "$PSScriptRoot/out/bin" -Force
&"$PSScriptRoot/../../../scripts/buildReleaseServer.ps1"

Copy-Item "$PSScriptRoot/../../../server/lsp_server_8051_asm/target/release/lsp_server_8051_asm.exe" -Destination "$PSScriptRoot/out/bin/lsp_server_8051_asm.exe" 