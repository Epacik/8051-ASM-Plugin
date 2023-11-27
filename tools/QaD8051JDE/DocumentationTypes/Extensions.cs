namespace DocumentationTypes;

public static class Extensions
{
    public static string ToLabel(this PossibleOperands operand) => ValidOperand.Labels[operand];

    public static int TokenMaxValue(this PossibleOperands operand) => operand switch
    {
        PossibleOperands.Any => 0,
        PossibleOperands.CodeAddress => 32,
        PossibleOperands.Label => 1,
        PossibleOperands.Data => 32,
        PossibleOperands.Data16 => 32,
        PossibleOperands.InternalRamAddress => 32,
        PossibleOperands.AddressInR0OrR1 => 2,
        PossibleOperands.HelperRegisters => 8,
        PossibleOperands.CarryFlag => 1,
        PossibleOperands.BitAddress => 32,
        PossibleOperands.NegatedBitAddress => 32,
        PossibleOperands.RelativeAddress => 32,
        PossibleOperands.Accumulator => 1,
        PossibleOperands.AccumulatorAndB => 1,
        PossibleOperands.AddressInAccumulatorPlusDptr => 1,
        PossibleOperands.DPTR => 1,
        PossibleOperands.AddressInDptr => 1,
        PossibleOperands.AddressInAccumulatorPlusPC => 1,
        PossibleOperands.AbsoluteAddress => 32,
        PossibleOperands.RegisterB => 1,
        PossibleOperands.DPL => 1,
        PossibleOperands.DPH => 1,
        PossibleOperands.HexNumber => 32,
        PossibleOperands.BinaryNumber => 32,
        PossibleOperands.DecimalNumber => 32,
        PossibleOperands.AsciiCharacters => 32,
    };
    public static string ToToken(this PossibleOperands operand, int value = 0) => operand switch
    {
        PossibleOperands.Any => "",
        PossibleOperands.CodeAddress => $"{(value * 8):X}H",
        PossibleOperands.Label => "LABEL",
        PossibleOperands.Data => $"#{(((value * 8) + 3) % 255):X}H",
        PossibleOperands.Data16 => $"#{(((value * 8) + 4) % 255):X}{(((value * 8) + 2) % 255):X}H",
        PossibleOperands.InternalRamAddress => $"{(((value * 8) + 2) % 255):X}H",
        PossibleOperands.AddressInR0OrR1 => value switch
        {
            0 or 1 => $"@R{value}",
            _ => throw new ArgumentOutOfRangeException(nameof(value)),
        },
        PossibleOperands.HelperRegisters => value switch
        {
            0 or 1 or 2 or 3 or 4 or 5 or 6 or 7 => $"R{value}",
            _ => throw new ArgumentOutOfRangeException(nameof(value)),
        },
        PossibleOperands.CarryFlag => "C",
        PossibleOperands.BitAddress => $"{(((value * 8) + 1) % 255):X}H",
        PossibleOperands.NegatedBitAddress => $"/{(((value * 8) + 1) % 255):X}H",
        PossibleOperands.RelativeAddress => $"-{(((value * 8) + 1) % 127):X}H",
        PossibleOperands.Accumulator => "A",
        PossibleOperands.AccumulatorAndB => "AB",
        PossibleOperands.AddressInAccumulatorPlusDptr => "@A+DPTR",
        PossibleOperands.DPTR => "DPTR",
        PossibleOperands.AddressInDptr => "@DPTR",
        PossibleOperands.AddressInAccumulatorPlusPC => "@A+PC",
        PossibleOperands.AbsoluteAddress => $"{(((value * 8) + 5) % 255):X}H",
        PossibleOperands.RegisterB => "B",
        PossibleOperands.DPL => "DPL",
        PossibleOperands.DPH => "DPL",
        PossibleOperands.HexNumber => $"#{(((value * 8) + 3) % 255):X}H",
        PossibleOperands.BinaryNumber => $"#{(((value * 8) + 3) % 255):B}B",
        PossibleOperands.DecimalNumber => $"#{(((value * 8) + 3) % 255)}",
        PossibleOperands.AsciiCharacters => $"#{string.Join("", Enumerable.Repeat('a', value).Select((x, p) => (char)p))}H",
    };
}
