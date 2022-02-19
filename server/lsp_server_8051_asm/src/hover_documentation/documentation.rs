use serde::{Deserialize, Serialize};
use bitflags::bitflags;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Documentation {
    pub detail: std::string::String,
    pub description: std::string::String,
    pub valid_operands: std::vec::Vec<std::vec::Vec<ValidOperand>>,
    pub affected_flags: std::vec::Vec<Flag>,
    pub dont_generate_syntax: bool,
    pub category: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Flag {
    pub flag: i32,
    pub when_set: std::string::String,
    pub when_unset: std::string::String,
}

#[allow(dead_code)]
impl Flag {
    pub fn flag(&self) -> FlagType {
        FlagType::from_bits_truncate(self.flag)
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct ValidOperand {
    pub operand: i32,
    pub when_first_is: i32,
}

#[allow(dead_code)]
impl ValidOperand {
    pub fn operand(&self) -> PossibleOperand {
        PossibleOperand::from_bits_truncate(self.operand)
    }

    pub fn when_first_is(&self) -> PossibleOperand {
        PossibleOperand::from_bits_truncate(self.when_first_is)
    }

} 

bitflags! {
    #[derive(serde::Deserialize, Default)]
    pub struct FlagType: i32 {
        const PARITY                = 0;
        const USER_DEFINED          = 1;
        const OVERFLOW              = 2;
        const REGISTER_BANK_SELECT0 = 3;
        const REGISTER_BANK_SELECT1 = 4;
        const FLAG0                 = 5;
        const AUXILIARY_CARRY       = 6;
        const CARRY                 = 7;
    }

    #[derive(serde::Deserialize, Default)]
    pub struct PossibleOperand: i32 {
        const ANY                              = 0;
        const CODE_ADDRESS                     = 1;
        const LABEL                            = 2;
        const DATA                             = 3;
        const DATA16                           = 4;
        const INTERNAL_RAM_ADDRESS             = 5;
        const ADDRESS_IN_R0_OR_R1              = 6;
        const REGISTERS_RX                     = 7;
        const CARRY_FLAG                       = 8;
        const BIT_ADDRESS                      = 9;
        const NEGATED_BIT_ADDRESS              = 10;
        const RELATIVE_ADDRESS                 = 11;
        const ACCUMULATOR                      = 12;
        const ACCUMULATOR_AND_B                = 13;
        const ADDRESS_IN_ACCUMULATOR_PLUS_DPTR = 14;
        const DPTR                             = 15;
        const ADDRESS_IN_DPTR                  = 16;
        const ADDRESS_IN_ACCUMULATOR_PLUS_PC   = 17;
    }
}
#[allow(dead_code)]

impl FlagType {
    pub fn label(&self) -> String {
        (match self.bits {
            0 => "Patity",
            1 => "User defined",
            2 => "Overflow",
            3 => "Register Bank Select 0",
            4 => "Register Bank Select 1",
            5 => "Flag 0",
            6 => "Auxiliary Carry",
            7 => "Carry",
            _ => "Unknown"
        }).to_string()
    }
}

impl PossibleOperand {
    pub fn label(&self) -> String {
        (match self.bits {
            0 => "any",
            1 => "code address",
            2 => "label",
            3 => "byte",
            4 => "two bytes",
            5 => "internal RAM address",
            6 => "@R0 or @R1",
            7 => "R0 trough R7",
            8 => "carry flag",
            9 => "bit address",
            10 => "address of negated bit",
            11 => "relative address",
            12 => "the Accumulator",
            13 => "the Accumulator an B register",
            14 => "address in the Accumulator + DPTR",
            15 => "DPTR",
            16 => "address in DPTR",
            17 => "address in the Accumulator + PC",
            _ => "Unknown"
        }).to_string()
    }

    pub fn example(&self, i: Option<i32>) -> String {
        let r_address = format!("@R{}", i.unwrap_or(0));
        let r = format!("R{}", i.unwrap_or(0));
        (match self.bits {
            1 => "23H",
            2 => "LABEL",
            3 => "#32H",
            4 => "#5C6H",
            5 => "23H",
            6 => r_address.as_str(),
            7 => r.as_str(),
            8 => "C",
            9 => "23H",
            10 => "/23H",
            11 => "23H",
            12 => "A",
            13 => "AB",
            14 => "@A+DPTR",
            15 => "DPTR",
            16 => "@DPTR",
            17 => "@A+PC",
            _ => ""
        }).to_string()
    }
}