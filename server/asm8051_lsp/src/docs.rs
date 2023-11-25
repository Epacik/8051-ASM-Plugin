use std::collections::HashMap;

use crate::flags::Locale;
use asm8051_shared::Documentation;
use lazy_static::lazy_static;

pub(crate) fn all_documentation(locale: &Locale) -> Option<HashMap<String, Documentation>> {
    let docs = DOCUMENTATION.get(locale);

    match docs {
        Some(some_docs) => Option::Some(some_docs.clone()),
        None => Option::None,
    }
}

pub fn get_documentation(locale: &Locale, _mnemonic: &String) -> Option<Documentation> {
    let docs = match DOCUMENTATION.get(locale) {
        Some(doc) => doc,
        None => return None,
    };

    let doc = match docs.get(_mnemonic) {
        Some(d) => d,
        None => return None,
    };

    Option::Some(doc.clone())
}

lazy_static! {

    pub static ref DOCUMENTATION: HashMap<Locale, HashMap<String, Documentation>> = HashMap::from([
        (Locale::ENGLISH, load_documentation::load_documentation!(english)),
        (Locale::POLISH, load_documentation::load_documentation!(polish)),
    ]);
}


// pub enum Symbol {
//     None,
//     Number(String),
//     Label(String, u32),
//     Keyword(String),
//     Constant(String, u32),
//     Macro(String, u32),
// }