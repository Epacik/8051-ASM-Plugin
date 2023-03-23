using QaD8051JDE.DocumentationTypes;
using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace QaD8051JDE;

public class ValidOperand : IOperand<PossibleOperands>
{
    [JsonPropertyName("operand")]
    public PossibleOperands Operand { get; set; }

    [JsonPropertyName("when_first_is")]
    public PossibleOperands WhenFirstIs { get; set; } = PossibleOperands.Any;

    public void Deconstruct(out PossibleOperands operand, out PossibleOperands whenFirstIs)
    {
        operand = Operand;
        whenFirstIs = WhenFirstIs;
    }

    public static readonly Dictionary<PossibleOperands, string> Labels = new()
    {
        [PossibleOperands.Any]                          = "Any",
        [PossibleOperands.CodeAddress]                  = "Code address",
        [PossibleOperands.Label]                        = "Label",
        [PossibleOperands.Data]                         = "8 bit data (e.g. #32h)",
        [PossibleOperands.Data16]                       = "16 bit data (e.g. #2DCh)",
        [PossibleOperands.InternalRamAddress]           = "Internal RAM Address",
        [PossibleOperands.AddressInR0OrR1]              = "Address stored in @R0 or @R1",
        [PossibleOperands.HelperRegisters]              = "Registers R0 trough R7",
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
        [PossibleOperands.AbsoluteAddress]              = "Absolute Address",
        [PossibleOperands.RegisterB]                    = "Register B",
        [PossibleOperands.DPL]                          = "DPL (DPTR Low)",
        [PossibleOperands.DPH]                          = "DPH (DPTR High)",

        [PossibleOperands.HexNumber]                    = "Hexadecimal number (01h)",
        [PossibleOperands.BinaryNumber]                 = "Binary number",
        [PossibleOperands.DecimalNumber]                = "Decimal number",
        [PossibleOperands.AsciiCharacters]              = "ASCII Characters",
    };
}
