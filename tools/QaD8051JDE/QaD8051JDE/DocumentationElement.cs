using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace QaD8051JDE;

public class DocumentationElementOld
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

}

public class Flag
{
    [JsonPropertyName("flag")]
    public FlagType FlagType { get; set; }

    [JsonPropertyName("when_set")]
    public string WhenSet { get; set; }

    [JsonPropertyName("when_unset")]
    public string? WhenUnset { get; set; }
}

public class ValidOperand
{
    [JsonPropertyName("operand")]
    public PossibleOperands PossibleOperand { get; set; }

    [JsonPropertyName("when_first_is")]
    public PossibleOperands WhenFirstIs { get; set; } = PossibleOperands.Any;

    public static readonly Dictionary<PossibleOperands, string> Labels = new()
    {
        [PossibleOperands.Any]                          = "Any",
        [PossibleOperands.CodeAddress]                  = "Code address",
        [PossibleOperands.Label]                        = "Label",
        [PossibleOperands.Data]                         = "8 bit data (e.g. #32h)",
        [PossibleOperands.Data16]                       = "16 bit data (e.g. #2DCh)",
        [PossibleOperands.InternalRamAddress]           = "Internal RAM Address",
        [PossibleOperands.AddressInR0OrR1]              = "Address stored in @R0 or @R1",
        [PossibleOperands.RegistersRX]                  = "Registers R0 trough R7",
        [PossibleOperands.CarryFlag]                    = "The Carry flag",
        [PossibleOperands.BitAddress]                   = "Address to a Bit",
        [PossibleOperands.NegatedBitAddress]            = "Address to a Negated Bit",
        [PossibleOperands.RelativeAddress]              = "Relative address",
        [PossibleOperands.Accumulator]                  = "Accumulator",
        [PossibleOperands.AccumulatorAndB]              = "Accumulator And B Accumulator",
        [PossibleOperands.AddressInAccumulatorPlusDPTR] = "Address in Accumulator + DPTR (@A+DPTR)",
        [PossibleOperands.DPTR]                         = "DPTR (Data Pointer)",
        [PossibleOperands.AddressInDPTR]                = "Address in DPTR",
        [PossibleOperands.AddressInAccumulatorPlusPC]   = "Address in Accumulator + Program Counter (@A+PC)",
    };
}

public enum FlagType
{
    Parity              = 0,
    UserDefined         = 1,
    Overflow            = 2,
    RegisterBankSelect0 = 3,
    RegisterBankSelect1 = 4,
    Flag0               = 5,
    AuxiliaryCarry      = 6,
    Carry               = 7,
}

public enum PossibleOperands
{
    Any                          = 0,
    CodeAddress                  = 1,
    Label                        = 2,
    Data                         = 3,
    Data16                       = 4,
    InternalRamAddress           = 5,
    AddressInR0OrR1              = 6,
    RegistersRX                  = 7,
    CarryFlag                    = 8,
    BitAddress                   = 9,
    NegatedBitAddress            = 10,
    RelativeAddress              = 11,
    Accumulator                  = 12,
    AccumulatorAndB              = 13,
    AddressInAccumulatorPlusDPTR = 14,
    DPTR                         = 15,
    AddressInDPTR                = 16,
    AddressInAccumulatorPlusPC   = 17,
}