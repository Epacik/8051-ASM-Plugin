using System.Text.Json.Serialization;

namespace QaD8051JDE;

public class DocumentationElement
{
    [JsonPropertyName("detail")]
    public string? Detail { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }

    [JsonPropertyName("syntax")]
    public string? Syntax { get; set; }

    [JsonPropertyName("affected_flags")]
    public string? AffectedFlags { get; set; }

    [JsonPropertyName("valid_operands")]
    public string? ValidOperands { get; set; }
}

