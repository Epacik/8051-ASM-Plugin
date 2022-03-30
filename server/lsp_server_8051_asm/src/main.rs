#![deny(warnings)]

mod backend;
mod client_configuration;
mod diagnostics;
mod flags;
mod hover_documentation;

use crate::client_configuration::ClientConfiguration;
use i18n_embed::{LanguageLoader, Localizer, DefaultLocalizer};
use i18n_embed::fluent::{FluentLanguageLoader, fluent_language_loader};
use lspower::lsp::ClientCapabilities;
use lspower::{LspService, Server};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use tokio::sync::Mutex;
use std::sync::{Arc};
use std::collections::HashMap;

//localization macro
#[allow(unused_macros)]
#[macro_export]
macro_rules! localize {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

#[tokio::main]
async fn main() {
    localizer().select(&["en".parse().unwrap()]).unwrap();


    let listener = tokio::net::TcpListener::bind("127.0.0.1:8050")
        .await
        .unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = tokio::io::split(stream);

    let (service, messages) = LspService::new(|client| backend::Backend {
        client,
        documents: Arc::new(Mutex::new(HashMap::new())),
        //locale: RefCell::new(None),
        client_capabilities: Arc::new(Mutex::new(ClientCapabilities::default())),
        client_configuration: Arc::new(Mutex::new(ClientConfiguration::default())),
    });

    Server::new(read, write)
        .interleave(messages)
        .serve(service)
        .await;
}


#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

#[allow(dead_code)]
static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    // Load the fallback langauge by default so that users of the
    // library don't need to if they don't care about localization.
    loader
        .load_fallback_language(&Localizations)
        .expect("Error while loading fallback language");

    loader
});

pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

