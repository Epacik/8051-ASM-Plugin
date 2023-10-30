use std::collections::HashMap;

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
    pub stack_space_needed: Option<u8>,
    pub used_registers: Option<std::vec::Vec<std::string::String>>,
    pub changed_registers: Option<std::vec::Vec<std::string::String>>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MainDocumentationElement {
    pub detail: std::string::String,
    pub description: std::string::String,
    pub prefix: std::string::String,
    pub label: Option<String>,
    pub affected_flags: HashMap<std::string::String, FlagDescription>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SharedDocumentationElement {
    pub valid_operands: std::vec::Vec<std::vec::Vec<ValidOperand>>,
    pub affected_flags: std::vec::Vec<std::string::String>,
    pub dont_generate_syntax: bool,
    pub dont_duplicate_in_all_docs: bool,
    pub prefix_required: bool,
    pub addressing_modes: Option<std::vec::Vec<std::string::String>>,
    pub stack_space_needed: Option<u8>,
    pub used_registers: Option<std::vec::Vec<std::string::String>>,
    pub changed_registers: Option<std::vec::Vec<std::string::String>>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct  FlagDescription {
    pub when_set: std::string::String,
    pub when_unset: std::string::String,
}


#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Flag {
    pub flag: std::string::String,
    pub when_set: std::string::String,
    pub when_unset: std::string::String,
}

impl Flag {
    pub fn new(flag: String, when_set: String, when_unset: String) -> Flag {
        Flag {
            flag,
            when_set,
            when_unset,
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ValidOperand {
    pub operand: std::string::String,
    pub when_first_is: std::string::String,
}


pub struct FileDescription {
    pub filename: String,
    shared_path: Option<String>,
    main_path: Option<String>,
}

impl FileDescription{
    pub fn new(filename: String, shared_path: Option<String>, main_path: Option<String>) -> FileDescription {
        FileDescription { filename, shared_path, main_path }
    }

    pub fn shared_path(&self) -> Option<String> {
        match &self.shared_path {
            Some(sp) => Some((sp.clone() + "/" + self.filename.as_str()).to_string()),
            None => None,
        }
    }

    pub fn main_path(&self) -> Option<String> {
        match &self.main_path {
            Some(mp) => Some((mp.clone() + "/" + self.filename.as_str()).to_string()),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Values {
    pub main: Option<serde_json::Value>,
    pub shared: Option<serde_json::Value>,
}

