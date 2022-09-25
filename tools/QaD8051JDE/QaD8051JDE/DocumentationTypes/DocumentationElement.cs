using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace QaD8051JDE;

public class DocumentationElement
{
    [JsonPropertyName("detail")]
    public string? Detail { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }

    [JsonPropertyName("valid_operands")]
    public List<List<ValidOperand>> ValidOperands { get; set; }

    [JsonPropertyName("affected_flags")]
    public List<Flag>? AffectedFlags { get; set; }

    [JsonPropertyName("dont_generate_syntax")]
    public bool DontGenerateSyntax { get; set; }

    [JsonPropertyName("dont_duplicate_in_all_docs")]
    public bool DontDuplicate { get; set; }

    [JsonIgnore]
    private string? _prefix;

    [JsonPropertyName("prefix")]
    public string? Prefix
    {
        get => _prefix ?? "";
        set => _prefix = value ?? "";
    }

    [JsonPropertyName("prefix_required")]
    public bool PrefixRequired { get; set; }

    [JsonPropertyName("label")]
    public string? Label { get; set; }
}
