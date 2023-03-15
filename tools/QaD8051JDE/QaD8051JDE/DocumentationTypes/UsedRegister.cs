using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace QaD8051JDE.DocumentationTypes;

public class UsedRegister
{
    [JsonPropertyName("register")]
    public PossibleUsedRegister PossibleOperand { get; set; }

    [JsonPropertyName("reason")]
    public string? WhenFirstIs { get; set; }

    public static readonly Dictionary<PossibleUsedRegister, string> Labels = new()
    {
        [PossibleUsedRegister.Accumulator] = "Accumulator",
        [PossibleUsedRegister.B]           = "B Register",
        [PossibleUsedRegister.DPTR]        = "DPTR",
        [PossibleUsedRegister.PSW]         = "PSW",
        [PossibleUsedRegister.R0]          = "R0",
        [PossibleUsedRegister.R1]          = "R1",
        [PossibleUsedRegister.R2]          = "R2",
    };
}
