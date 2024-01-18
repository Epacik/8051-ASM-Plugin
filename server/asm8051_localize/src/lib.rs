use std::{collections::HashMap, sync::RwLock};
use once_cell::sync::Lazy;

use serde_json::Value;

#[macro_use]
extern crate lazy_static;

static CURRENT_LOCALE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("en")));

/// Inspired by
/// https://github.com/longbridgeapp/rust-i18n/blob/main/src/lib.rs
/// Licensed under MIT License https://github.com/longbridgeapp/rust-i18n/blob/main/LICENSE
pub fn set_locale(locale: &str) {
    let mut current_locale = CURRENT_LOCALE.write().unwrap();
    *current_locale = locale.to_string();
}

/// Inspired by
/// https://github.com/longbridgeapp/rust-i18n/blob/main/src/lib.rs
/// Licensed under MIT License https://github.com/longbridgeapp/rust-i18n/blob/main/LICENSE
pub fn locale() -> String {
    CURRENT_LOCALE.read().unwrap().to_string()
}

/// Inspired by
/// https://github.com/longbridgeapp/rust-i18n/blob/main/src/lib.rs
/// Licensed under MIT License https://github.com/longbridgeapp/rust-i18n/blob/main/LICENSE
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! t {
    // t!("foo")
    ($key:expr) => {
        asm8051_localize::translate(asm8051_localize::locale().as_str(), $key)
    };

    // t!("foo", locale = "en")
    ($key:expr, locale = $locale:expr) => {
        asm8051_localize::translate($locale, $key)
    };

    // t!("foo", locale = "en", a = 1, b = "Foo")
    ($key:expr, locale = $locale:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            let mut message = asm8051_localize::translate($locale, $key);

            $(
                // Get the variable name as a string, and remove quotes surrounding the variable name
                let var_name = stringify!($var_name).trim_matches('"');
                // Make a holder string to replace the variable name with: %{var_name}
                let holder = format!("%{{{var_name}}}");

                message = message.replace(&holder, &format!("{}", $var_val));
            )+
            message
        }
    };

    // t!("foo %{a} %{b}", a = "bar", b = "baz")
    ($key:expr, $($var_name:tt = $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &asm8051_localize::locale(), $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", locale = "en", "a" => "bar", "b" => "baz")
    ($key:expr, locale = $locale:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = $locale, $($var_name = $var_val),*)
        }
    };

    // t!("foo %{a} %{b}", "a" => "bar", "b" => "baz")
    ($key:expr, $($var_name:tt => $var_val:expr),+ $(,)?) => {
        {
            t!($key, locale = &asm8051_localize::locale(), $($var_name = $var_val),*)
        }
    };
}

#[inline]
// Inspired by 
// https://github.com/longbridgeapp/rust-i18n/blob/main/crates/macro/src/lib.rs
// https://github.com/longbridgeapp/rust-i18n/blob/main/crates/support/src/lib.rs
// Licensed under MIT license https://github.com/longbridgeapp/rust-i18n/blob/main/LICENSE
pub fn translate(locale: &str, key: &str) -> String
{
    let backend = &TRANSLATE_BACKEND;

    if let Some(value) = backend.translate(locale, key) {
        return value.to_string();
    }

    if let Some(value) = backend.translate("en", key) {
        return value.to_string();
    }

    if locale.is_empty() {
        return key.to_string();
    }
    return format!("{}.{}", locale, key);
}

lazy_static! {
    static ref RAW_TRANSLATIONS: String = String::from(include_str!("../locales/app.yaml"));
    static ref TRANSLATION_MAP: HashMap::<String, HashMap<String, String>> = {
        let mut map = HashMap::<String, HashMap<String, String>>::new();
        let parsed_yaml = serde_yaml::from_str::<Value>(&RAW_TRANSLATIONS);
    
        if let Ok(ok) = parsed_yaml {
            if let Value::Object(obj) = ok {
                for (key, value) in obj {
                    if let Value::Object(langs) = value {
                        for (lang, translation) in langs {
                            if let Value::String(string) = translation {
                                if !map.contains_key(&lang) {
                                    map.insert(lang.clone(), HashMap::<String, String>::new());
                                }
    
                                let mut vec = map.get_mut(&lang);
    
                                if let Some(vec) = vec {
                                    vec.insert(key.clone(), string.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        map
    };
    static ref TRANSLATE_BACKEND: Box<dyn rust_i18n::Backend> = {
        let mut backend = rust_i18n::SimpleBackend::new();

        let translations = &TRANSLATION_MAP;
    
        for (locale, trs) in translations.iter() {
            let map = trs
                .into_iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            backend.add_translations(locale.as_str(), &map);
        }
        
        Box::new(backend)
    };

}

