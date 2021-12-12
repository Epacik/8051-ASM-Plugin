use std::collections::HashMap;
use std::sync::Mutex;
use bitflags::bitflags;
use lspower::Client;
use lspower::lsp::{ClientCapabilities, TextDocumentItem};
use serde_json::{Map, Value};

/// Connection with a client and additional configutation
#[derive(Debug)]
pub(crate) struct Backend {
    /// connection with LSP client
    pub(crate) client: Client,

    /// documents opened in editor
    pub(crate) documents: Mutex<HashMap<String, TextDocumentItem>>,

    /// Locale of user interface
    pub(crate) locale: Mutex<Option<String>>,

    /// things that client supports
    pub(crate) client_capabilities: Mutex<Option<ClientCapabilities>>,

    /// configuration received from client
    pub(crate) client_configuration: Mutex<ClientConfiguration>,
}


#[derive(Debug)]
pub struct ClientConfiguration {
    /// How many problems should we show at once
    pub(crate) max_number_of_problems: i32,

    /// Used for selecting default set of features
    pub(crate) kit: Kits,

    /// Which locale user selected for the plugin itself
    pub(crate) locale: Locale,

    /// locale of ui of the editor
    pub(crate) ui_locale: Locale,
}

impl ClientConfiguration {
    /// Loading configuration from JSON object (Map)
    pub(crate) fn from_json_object(json_object: Option<&Map<String, Value>>) -> Option<ClientConfiguration> {
        if json_object.is_none()
        {
            return Option::None;
        }

        let map = json_object.unwrap();

        // getting default configuration to edit
        let mut config = ClientConfiguration::default();

        // check every key value pair within a map resembling a JSON object in order to update default configuration
        for kv in map {
            print!("{}", kv.0);

            if kv.0 == &String::from("kit") && kv.1.is_string() && kv.1.as_str().is_some() {
                config.kit = match kv.1.as_str().unwrap() {
                    "DSM-51" => Kits::DSM51,
                    &_ => Kits::GENERIC_8051,
                }
            }
            else if kv.0 == &String::from("language") && kv.1.is_string() && kv.1.as_str().is_some() {
                config.locale = match kv.1.as_str().unwrap() {
                    "english" => Locale::ENGLISH,
                    "polski" => Locale::POLISH,
                    &_ => Locale::DEFAULT,
                }
            }
            else if kv.0 == &String::from("maxNumberOfProblems") && kv.1.is_i64() && kv.1.as_i64().is_some() {
                config.max_number_of_problems = kv.1.as_i64().unwrap() as i32;
            }
        }

        Option::Some(config)
    }

    /// default client configuration
    pub fn default() -> ClientConfiguration {
        ClientConfiguration {
            max_number_of_problems: 100,
            kit: Kits::GENERIC_8051,
            locale: Locale::DEFAULT,
            ui_locale: Locale::ENGLISH,
        }
    }
}

pub struct Documentation {
    pub title: &'static str,
    pub detail: &'static str,
    pub description: &'static str,
    pub syntax: &'static str,
    pub affected_flags: &'static str,
    pub valid_operands: &'static str,
}



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