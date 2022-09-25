$location = Get-Location

Set-Location "$PSScriptRoot/../server/lsp_server_8051_asm"
cargo build -r

Set-Location "$location"