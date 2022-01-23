use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Default, Clone)]
pub struct Documentation {
    pub detail: String,
    pub description: String,
    pub syntax: String,
    pub affected_flags: String,
    pub valid_operands: String,
}