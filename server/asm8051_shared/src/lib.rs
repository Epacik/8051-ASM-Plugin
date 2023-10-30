
asm8051_localize::init!();

pub trait FromI32 {
    fn from_i32(operand: i32, when_first_is: Option<i32>) -> ValidOperand;
}

impl FromI32 for ValidOperand {
    fn from_i32(operand: i32, when_first_is: Option<i32>) -> ValidOperand {
        let when_first_is = when_first_is.unwrap_or(0);

        let operand = match operand.try_into() {
            Ok(op) => op,
            Err(_) => panic!("operand was {}", t!("error.out-of_range")),
        };

        let when_first_is = match when_first_is.try_into() {
            Ok(wfi) => wfi,
            Err(_) => panic!("when_first_is {}", t!("error.out-of_range")), 
        };

        ValidOperand {
            operand,
            when_first_is,
        }
    }
}

pub trait Label {
    fn label(&self) -> String;
    fn example(&self, i: Option<i32>)-> String {
        String::new()
    }
}

#[allow(dead_code)]
#[derive(Default, Clone, Debug)]
pub struct Documentation {
    pub detail: String,
    pub description: String,
    pub valid_operands: Vec<Vec<ValidOperand>>,
    pub affected_flags: Vec<Flag>,
    pub dont_generate_syntax: bool,
    pub dont_duplicate_in_all_docs: bool,
    pub full_key: String,
    pub category: String,
    pub prefix: String,
    pub prefix_required: bool,
    pub label: Option<String>,
    pub addressing_modes: Vec::<AddressingMode>,
    pub stack_space_needed: Option<u8>,
    pub used_registers: Vec<PossibleRegister>,
    pub changed_registers: Vec<PossibleRegister>,
}


impl Documentation {
    #[allow(dead_code)]
    fn new(
        detail: &str,
        description: &str,
        valid_operands: Vec<Vec<ValidOperand>>,
        affected_flags: Vec<Flag>,
        dont_generate_syntax: bool,
        dont_duplicate_in_all_docs: bool,
        full_key: &str,
        category: &str,
        prefix: &str,
        prefix_required: bool,
        label: Option<String>,
        addressing_modes: Vec::<AddressingMode>,
        stack_space_needed: Option<u8>,
        used_registers: Vec<PossibleRegister>,
        changed_registers: Vec<PossibleRegister>,
    ) -> Documentation {
        Documentation {
            detail: String::from(detail),
            description: String::from(description),
            valid_operands,
            affected_flags,
            dont_generate_syntax,
            category: String::from(category),
            dont_duplicate_in_all_docs,
            full_key: String::from(full_key),
            prefix: String::from(prefix),
            prefix_required,
            label,
            addressing_modes,
            stack_space_needed,
            used_registers,
            changed_registers,
        }
    }
}

#[allow(dead_code)]
#[derive(Default, Clone, Debug)]
pub struct Flag {
    pub flag: FlagType,
    pub when_set: String,
    pub when_unset: String,
}

#[allow(dead_code)]
impl Flag {
    pub fn flag(&self) -> FlagType {
        self.flag
    }

    pub fn new(flag: FlagType) -> Flag {
        Flag {
            flag: flag,
            when_set: String::from(""),
            when_unset: String::from(""),
        }
    }

    pub fn new_with_messages(flag: FlagType, when_set: &str, when_unset: &str) -> Flag {
        Flag {
            flag: flag,
            when_set: String::from(when_set),
            when_unset: String::from(when_unset),
        }
    }

    pub fn from_i32(flag: i32) -> Flag {
        Flag {
            flag: flag.try_into().unwrap(),
            when_set: String::from(""),
            when_unset: String::from(""),
        }
    }

    pub fn from_i32_with_messages(flag: i32, when_set: &str, when_unset: &str) -> Flag {
        Flag {
            flag: flag.try_into().unwrap(),
            when_set: String::from(when_set),
            when_unset: String::from(when_unset),
        }
    }
}

#[allow(dead_code)]
#[derive(Default, Clone, Copy, Debug)]
pub struct ValidOperand {
    pub operand: PossibleOperand,
    pub when_first_is: PossibleOperand,
}

#[allow(dead_code)]
impl ValidOperand {
    pub fn operand(&self) -> PossibleOperand {
        self.operand
    }

    pub fn when_first_is(&self) -> PossibleOperand {
        self.when_first_is
    }

    pub fn new(operand: PossibleOperand, when_first_is: Option<PossibleOperand>) -> ValidOperand {
        ValidOperand {
            operand: operand,
            when_first_is: when_first_is.unwrap_or(PossibleOperand::Any),
        }
    }

    pub fn equals(&self, other: &ValidOperand) -> bool {
        self.operand == other.operand && self.when_first_is == other.when_first_is
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
#[derive(Debug)]
pub enum FlagType {
    #[default]
    Parity              = 0,
    UserDefined         = 1,
    Overflow            = 2,
    RegisterBankSelect0 = 3,
    RegisterBankSelect1 = 4,
    Flag0               = 5,
    AuxiliaryCarry      = 6,
    Carry               = 7
}

impl Label for FlagType {
    fn label(&self) -> String {
        match self {
            FlagType::Parity => format!("{} [P]", t!("flag.parity")),
            FlagType::UserDefined => format!("{}", t!("flag.user_defined")),
            FlagType::Overflow => format!("{} [OV]", t!("flag.overflow")),
            FlagType::RegisterBankSelect0 => format!("{} 0 [RS0]", t!("flag.register.bank_select")),
            FlagType::RegisterBankSelect1 => format!("{} 1 [RS1]", t!("flag.register.bank_select")),
            FlagType::Flag0 => format!("{} [F0]", t!("flag.flag0")),
            FlagType::AuxiliaryCarry => format!("{} [AC]", t!("flag.auxiliary_carry")),
            FlagType::Carry => format!("{} [CY]", t!("flag.carry")),
        }
    }
}

impl TryFrom<i32> for FlagType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == FlagType::Parity              as i32 => Ok(FlagType::Parity              ),
            x if x == FlagType::UserDefined         as i32 => Ok(FlagType::UserDefined         ),
            x if x == FlagType::Overflow            as i32 => Ok(FlagType::Overflow            ),
            x if x == FlagType::RegisterBankSelect0 as i32 => Ok(FlagType::RegisterBankSelect0 ),
            x if x == FlagType::RegisterBankSelect1 as i32 => Ok(FlagType::RegisterBankSelect1 ),
            x if x == FlagType::Flag0               as i32 => Ok(FlagType::Flag0               ),
            x if x == FlagType::AuxiliaryCarry      as i32 => Ok(FlagType::AuxiliaryCarry      ),
            x if x == FlagType::Carry               as i32 => Ok(FlagType::Carry               ),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
#[derive(Debug)]
pub enum PossibleOperand {
    #[default]
    Any                          = 0,
    CodeAddress                  = 1,
    Label                        = 2,
    Data                         = 3,
    Data16                       = 4,
    InternalRamAddress           = 5,
    AddressInR0OrR1              = 6,
    HelperRegisters              = 7,
    CarryFlag                    = 8,
    BitAddress                   = 9,
    NegatedBitAddress            = 10,
    RelativeAddress              = 11,
    Accumulator                  = 12,
    AccumulatorAndB              = 13,
    AddressInAccumulatorPlusDptr = 14,
    Dptr                         = 15,
    AddressInDptr                = 16,
    AddressInAccumulatorPlusPC   = 17,
    AbsoluteAddress              = 18,
    RegisterB                    = 19,
    Dpl                          = 20,
    Dph                          = 21,

    HexNumber                    = 100,
    BinaryNumber                 = 101,
    DecimalNumber                = 102,
    AsciiCharacters              = 103,
}

impl Label for PossibleOperand {
    fn label(&self) -> String {
        match self {
            PossibleOperand::Any => t!("operand.any"),
            PossibleOperand::CodeAddress => t!("operand.codeAddress"),
            PossibleOperand::Label => t!("operand.label"),
            PossibleOperand::Data => t!("operand.byte"),
            PossibleOperand::Data16 => t!("operand.twoBytes"),
            PossibleOperand::InternalRamAddress => t!("operand.internalRamAddress"),
            PossibleOperand::AddressInR0OrR1 => t!("operand.indirectR0OrR1"),
            PossibleOperand::HelperRegisters => t!("operand.helperRegister"),
            PossibleOperand::CarryFlag => t!("operand.carryFlag"),
            PossibleOperand::BitAddress => t!("operand.bitAddress"),
            PossibleOperand::NegatedBitAddress => t!("operand.negatedBitAddress"),
            PossibleOperand::RelativeAddress => t!("operand.relativeAddress"),
            PossibleOperand::Accumulator => t!("operand.A"),
            PossibleOperand::AccumulatorAndB => t!("operand.AB"),
            PossibleOperand::AddressInAccumulatorPlusDptr => t!("operand.A_DPTR"),
            PossibleOperand::Dptr => t!("operand.DPTR"),
            PossibleOperand::AddressInDptr => t!("operand.indirectDPTR"),
            PossibleOperand::AddressInAccumulatorPlusPC => t!("operand.indirectA_PC"),
            PossibleOperand::AbsoluteAddress => t!("operand.absoluteAddress"),
            PossibleOperand::RegisterB => t!("operand.B"),
            PossibleOperand::Dpl => t!("operand.DPL"),
            PossibleOperand::Dph => t!("operand.DPH"),

            PossibleOperand::HexNumber => t!("operand.hex"),
            PossibleOperand::BinaryNumber => t!("operand.bin"),
            PossibleOperand::DecimalNumber => t!("operand.dec"),
            PossibleOperand::AsciiCharacters => t!("operand.ascii"),
        }
    }

    fn example(&self, i: Option<i32>) -> String {
        let r_address = format!("@R{}", i.unwrap_or(0));
        let r = format!("R{}", i.unwrap_or(0));
        let label = t!("operand.example_label");

        (match self {
            PossibleOperand::CodeAddress => "23H",
            PossibleOperand::Label => label.as_str(),
            PossibleOperand::Data => "#32H",
            PossibleOperand::Data16 => "#5C6H",
            PossibleOperand::InternalRamAddress => "23H",
            PossibleOperand::AddressInR0OrR1 => r_address.as_str(),
            PossibleOperand::HelperRegisters => r.as_str(),
            PossibleOperand::CarryFlag => "C",
            PossibleOperand::BitAddress => "23H",
            PossibleOperand::NegatedBitAddress => "/23H",
            PossibleOperand::RelativeAddress => "23H",
            PossibleOperand::Accumulator => "A",
            PossibleOperand::AccumulatorAndB => "AB",
            PossibleOperand::AddressInAccumulatorPlusDptr => "@A+DPTR",
            PossibleOperand::Dptr => "DPTR",
            PossibleOperand::AddressInDptr => "@DPTR",
            PossibleOperand::AddressInAccumulatorPlusPC => "@A+PC",
            PossibleOperand::AbsoluteAddress => "100h",
            PossibleOperand::RegisterB => "B",
            PossibleOperand::Dpl => "DPL",
            PossibleOperand::Dph => "DPH",

            PossibleOperand::HexNumber => "56h",
            PossibleOperand::BinaryNumber => "010101011b",
            PossibleOperand::DecimalNumber => "63",
            PossibleOperand::AsciiCharacters => "'Lorem ipsum'",
            _ => "",
        })
        .to_string()
    }
}

impl TryFrom<i32> for PossibleOperand {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == PossibleOperand::Any                          as i32 => Ok(PossibleOperand::Any                         ),
            x if x == PossibleOperand::CodeAddress                  as i32 => Ok(PossibleOperand::CodeAddress                 ),
            x if x == PossibleOperand::Label                        as i32 => Ok(PossibleOperand::Label                       ),
            x if x == PossibleOperand::Data                         as i32 => Ok(PossibleOperand::Data                        ),
            x if x == PossibleOperand::Data16                       as i32 => Ok(PossibleOperand::Data16                      ),
            x if x == PossibleOperand::InternalRamAddress           as i32 => Ok(PossibleOperand::InternalRamAddress          ),
            x if x == PossibleOperand::AddressInR0OrR1              as i32 => Ok(PossibleOperand::AddressInR0OrR1             ),
            x if x == PossibleOperand::HelperRegisters              as i32 => Ok(PossibleOperand::HelperRegisters             ),
            x if x == PossibleOperand::CarryFlag                    as i32 => Ok(PossibleOperand::CarryFlag                   ),
            x if x == PossibleOperand::BitAddress                   as i32 => Ok(PossibleOperand::BitAddress                  ),
            x if x == PossibleOperand::NegatedBitAddress            as i32 => Ok(PossibleOperand::NegatedBitAddress           ),
            x if x == PossibleOperand::RelativeAddress              as i32 => Ok(PossibleOperand::RelativeAddress             ),
            x if x == PossibleOperand::Accumulator                  as i32 => Ok(PossibleOperand::Accumulator                 ),
            x if x == PossibleOperand::AccumulatorAndB              as i32 => Ok(PossibleOperand::AccumulatorAndB             ),
            x if x == PossibleOperand::AddressInAccumulatorPlusDptr as i32 => Ok(PossibleOperand::AddressInAccumulatorPlusDptr),
            x if x == PossibleOperand::Dptr                         as i32 => Ok(PossibleOperand::Dptr                        ),
            x if x == PossibleOperand::AddressInDptr                as i32 => Ok(PossibleOperand::AddressInDptr               ),
            x if x == PossibleOperand::AddressInAccumulatorPlusPC   as i32 => Ok(PossibleOperand::AddressInAccumulatorPlusPC  ),
            x if x == PossibleOperand::AbsoluteAddress              as i32 => Ok(PossibleOperand::AbsoluteAddress             ),
            x if x == PossibleOperand::RegisterB                    as i32 => Ok(PossibleOperand::RegisterB                   ),
            x if x == PossibleOperand::Dpl                          as i32 => Ok(PossibleOperand::Dpl                         ),
            x if x == PossibleOperand::Dph                          as i32 => Ok(PossibleOperand::Dph                         ),
            x if x == PossibleOperand::HexNumber                    as i32 => Ok(PossibleOperand::HexNumber                   ),
            x if x == PossibleOperand::BinaryNumber                 as i32 => Ok(PossibleOperand::BinaryNumber                ),
            x if x == PossibleOperand::DecimalNumber                as i32 => Ok(PossibleOperand::DecimalNumber               ),
            x if x == PossibleOperand::AsciiCharacters              as i32 => Ok(PossibleOperand::AsciiCharacters             ),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
#[derive(Debug)]
pub enum AddressingMode {
    #[default]
    Implied          = 0,
    Immediate        = 1,
    Register         = 2,
    Direct           = 3,
    RegisterIndirect = 4,
    Indexed          = 5,
}

impl Label for AddressingMode {
    fn label(&self) -> String {
        match self {
            AddressingMode::Implied          => t!("addressingMode.Implied"),
            AddressingMode::Immediate        => t!("addressingMode.Immediate"),
            AddressingMode::Register         => t!("addressingMode.Register"),
            AddressingMode::Direct           => t!("addressingMode.Direct"),
            AddressingMode::RegisterIndirect => t!("addressingMode.RegisterIndirect"),
            AddressingMode::Indexed          => t!("addressingMode.Indexed"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Default)]
#[derive(Debug)]
pub enum PossibleRegister
{
    #[default]
    Accumulator = 0,
    B           = 1,
    Dptr        = 2,
    Psw         = 3,
    R0          = 4,
    R1          = 5,
    R2          = 6,
} 

impl Label for PossibleRegister{
    fn label(&self) -> String {
        match self {
            PossibleRegister::Accumulator => t!("operand.A"),
            PossibleRegister::B => t!("operand.B"),
            PossibleRegister::Dptr => t!("operand.DPTR"),
            PossibleRegister::Psw => t!("register.PSW"),
            PossibleRegister::R0 => t!("register.R0"),
            PossibleRegister::R1 => t!("register.R1"),
            PossibleRegister::R2 => t!("register.R2"),
        }
    }
}