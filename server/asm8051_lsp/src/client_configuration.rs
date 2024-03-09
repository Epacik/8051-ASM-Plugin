use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ClientConfiguration {
    /// How many problems should we show at once
    #[serde(default, rename = "maxNumberOfProblems")]
    pub(crate) max_number_of_problems: i64,

    /// Used for selecting default set of features
    #[serde(default, rename = "kit")]
    pub(crate) kit: String,
}

impl ClientConfiguration {
    #[allow(dead_code)]
    pub fn kit(&self) -> Kits {
        match self.kit.as_str() {
            "DSM-51" => Kits::DSM51,
            &_ => Kits::GENERIC_8051,
        }
    }
}

use bitflags::bitflags;

bitflags! {
    pub struct Kits: u32 {
        const GENERIC_8051 = 1;
        const DSM51        = 2;
    }

    pub struct Locale: u32 {
        const DEFAULT = 1;
        const ENGLISH = 2;
        const POLISH  = 3;
    }
}

#[allow(dead_code)]
impl Locale {
    pub fn lang_name(&self) -> String {
        (match self.bits {
            2 => "en",
            3 => "pl",
            _ => "",
        })
        .to_string()
    }
}

impl Kits {
    pub fn category_name(&self) -> String {
        match self.bits {
            2 => String::from("dsm51"),
            _ => String::new(),
        }
    }
}

