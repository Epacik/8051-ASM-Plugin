param (
    [Parameter()]
    [ValidateSet("all", "docsEditor", "server", "vscodeClient")]
    [string[]] $Launch = @("all"),

    [Parameter()]
    [ValidateSet("all", "docsEditor")]
    [string[]] $Build    
)

if ($null -ne $Build ) {
    if ($Build.Contains("all") -or $Build.Contains("docsEditor")) {
        dotnet build "$PSScriptRoot/../tools/QaD8051JDE/QaD8051JDE.sln";
    }
}

if ($null -ne $Launch) {

    if ($Launch.Contains("all") -or $Launch.Contains("docsEditor")) {
        $null = &"$PSScriptRoot/../tools/QaD8051JDE/QaD8051JDE/bin/Debug/net6.0/QaD8051JDE.exe" "$PSScriptRoot/../server/lsp_server_8051_asm/load_documentation/json_documentation/" &;
    }
    
    if ($Launch.Contains("all") -or $Launch.Contains("server")) {
        $null = code "$PSScriptRoot/../plugins/vscode/8051-support/" &; 
    }
    
    if ($Launch.Contains("all") -or $Launch.Contains("vscodeClient")) {
        $null = code "$PSScriptRoot/../server/asm8051.code-workspace" &; 
    }
}