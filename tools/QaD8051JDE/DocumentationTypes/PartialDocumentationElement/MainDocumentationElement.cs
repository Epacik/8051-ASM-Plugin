using System.Text.Json.Serialization;

namespace DocumentationTypes.PartialDocumentationElement;

public class MainDocumentationElement
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
