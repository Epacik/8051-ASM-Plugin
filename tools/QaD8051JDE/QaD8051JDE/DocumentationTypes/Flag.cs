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

    public void Deconstruct(out FlagType flagType, out string? whenSet, out string? whenUnset)
    {
        flagType = FlagType;
        whenSet = WhenSet;
        whenUnset = WhenUnset;
    }
}

public class FlagDescription
{
    [JsonPropertyName("when_set")]
    public string? WhenSet { get; set; }

    [JsonPropertyName("when_unset")]
    public string? WhenUnset { get; set; }
}
