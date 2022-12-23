using namespace System.Collections.Generic;

param (
    [Parameter(Mandatory=$true)]
    [ValidateSet("all", "docsEditor", "server", "vscodeClient")]
    [string[]] $Launch = @("all"),

    [Parameter()]
    [ValidateSet("all", "docsEditor")]
    [string[]] $Build    
)

$ParentDir = "$PSScriptRoot/..";

if ($null -ne $Build ) {
    if ($Build.Contains("all") -or $Build.Contains("docsEditor")) {
        dotnet build "$ParentDir/tools/QaD8051JDE/QaD8051JDE.sln";
    }
}


if ($null -ne $Launch) {
    $jobs = [List[object]]::new();

    if ($Launch.Contains("all") -or $Launch.Contains("docsEditor")) {
         $job = &"$ParentDir/tools/QaD8051JDE/QaD8051JDE/bin/Debug/net6.0/QaD8051JDE.exe" "$ParentDir/server/lsp_server_8051_asm/load_documentation/json_documentation/" &;
         $jobs.Add($job);
    }
    
    if ($Launch.Contains("all") -or $Launch.Contains("server")) {
        $job = code "$ParentDir/plugins/vscode/8051-support/" &; 
        $jobs.Add($job);
    }
    
    if ($Launch.Contains("all") -or $Launch.Contains("vscodeClient")) {
        $job = code "$ParentDir/server/asm8051.code-workspace" &; 
        $jobs.Add($job);
    }

    Receive-Job $jobs -Wait -AutoRemoveJob
}