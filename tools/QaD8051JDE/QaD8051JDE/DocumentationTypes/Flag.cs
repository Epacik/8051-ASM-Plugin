using System.Text.Json.Serialization;

namespace QaD8051JDE;

public class Flag
{
    [JsonPropertyName("flag")]
    public FlagType FlagType { get; set; }

    [JsonPropertyName("when_set")]
    public string? WhenSet { get; set; }

    [JsonPropertyName("when_unset")]
    public string? WhenUnset { get; set; }
}
