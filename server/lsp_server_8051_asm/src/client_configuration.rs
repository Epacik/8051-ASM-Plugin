use crate::flags::{Kits};
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
            &_       => Kits::GENERIC_8051
        }
    }
}