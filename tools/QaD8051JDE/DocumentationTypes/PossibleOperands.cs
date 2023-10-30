namespace DocumentationTypes;

public enum PossibleOperands
{
    Any                          = 0,
    CodeAddress                  = 1,
    Label                        = 2,
    Data                         = 3,
    Data16                       = 4,
    publicRamAddress           = 5,
    AddressInR0OrR1              = 6,
    HelperRegisters              = 7,
    CarryFlag                    = 8,
    BitAddress                   = 9,
    NegatedBitAddress            = 10,
    RelativeAddress              = 11,
    Accumulator                  = 12,
    AccumulatorAndB              = 13,
    AddressInAccumulatorPlusDptr = 14,
    DPTR                         = 15,
    AddressInDptr                = 16,
    AddressInAccumulatorPlusPC   = 17,
    AbsoluteAddress              = 18,
    RegisterB                    = 19,
    DPL                          = 20,
    DPH                          = 21,

    HexNumber                    = 100,
    BinaryNumber                 = 101,
    DecimalNumber                = 102,
    AsciiCharacters              = 103,
}
