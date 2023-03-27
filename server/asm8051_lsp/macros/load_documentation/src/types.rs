use serde::{Deserialize, Serialize};


#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Documentation {
    pub detail: std::string::String,
    pub description: std::string::String,
    pub valid_operands: std::vec::Vec<std::vec::Vec<ValidOperand>>,
    pub affected_flags: std::vec::Vec<Flag>,
    pub dont_generate_syntax: bool,
    pub dont_duplicate_in_all_docs: bool,
    pub prefix: std::string::String,
    pub prefix_required: bool,
    pub label: Option<String>,
    pub addressing_modes: Option<std::vec::Vec<std::string::String>>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Flag {
    pub flag: std::string::String,
    pub when_set: std::string::String,
    pub when_unset: std::string::String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ValidOperand {
    pub operand: std::string::String,
    pub when_first_is: std::string::String,
}
