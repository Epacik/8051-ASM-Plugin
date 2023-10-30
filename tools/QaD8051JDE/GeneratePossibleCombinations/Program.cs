

using System.Reflection;

var assembly = Assembly.GetExecutingAssembly();
var assemblyPath = assembly.Location;
var repoPath = assemblyPath.Substring(
    0, 
    assemblyPath.LastIndexOf("tools"));

var docsPath = Path.Combine(repoPath, "server", "asm8051_docs", "json_documentation");
var sharedPath = Path.Combine(docsPath, ".shared");
var snglishPath = Path.Combine(docsPath, "english");

var files = Directory.GetFiles(".");

foreach (var file in files)
{

}