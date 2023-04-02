using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace QaD8051JDE.DocumentationTypes.PartialDocumentationElement;

internal class MainDocumentationElement
{
    [JsonPropertyName("detail")]
    public string? Detail { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }

    [JsonPropertyName("affected_flags")]
    public Dictionary<FlagType, FlagDescription>? AffectedFlags { get; set; }

    [JsonIgnore]
    private string? _prefix;

    [JsonPropertyName("prefix")]
    public string? Prefix
    {
        get => _prefix ?? "";
        set => _prefix = value ?? "";
    }

    [JsonPropertyName("label")]
    public string? Label { get; set; }
}
