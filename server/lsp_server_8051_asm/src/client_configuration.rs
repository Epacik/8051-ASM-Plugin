use crate::flags::{Kits, Locale};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ClientConfiguration {
    /// How many problems should we show at once
    #[serde(default, rename = "maxNumberOfProblems")]
    pub(crate) max_number_of_problems: i64,

    /// Used for selecting default set of features
    #[serde(default, rename = "kit")]
    pub(crate) kit: String,

    /// Which locale user selected for the plugin itself
    #[serde(default, rename = "language")]
    pub(crate) locale: String,

    /// locale of ui of the editor
    #[serde(default)]
    pub(crate) ui_locale: String,
}

impl ClientConfiguration {
    #[allow(dead_code)]
    pub fn kit(&self) -> Kits {
        match self.kit.as_str() {
            "DSM-51" => Kits::DSM51,
            &_       => Kits::GENERIC_8051
        }
    }

    #[allow(dead_code)]
    pub fn display_locale(&self) -> Locale {
        let locale: Locale;
        if self.locale() == Locale::DEFAULT {
            locale = self.ui_locale();
        } else {
            locale = self.locale();
        }
        locale
    }

    pub fn locale(&self) -> Locale {
        match self.locale.as_str() {
            "english" => Locale::ENGLISH,
            "polski"  => Locale::POLISH,
            &_        => Locale::DEFAULT,
        }
    }

    pub fn ui_locale(&self) -> Locale {
        match self.ui_locale.as_str() {
            "polski"  => Locale::POLISH,
            &_        => Locale::ENGLISH
        }
    }
}