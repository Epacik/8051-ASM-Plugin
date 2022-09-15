#![deny(warnings)]
//#region imports
mod backend;
mod client_configuration;
mod diagnostics;
mod flags;
mod hover;

use i18n_embed::{
    LanguageLoader, Localizer, DefaultLocalizer, fluent::{
        FluentLanguageLoader, fluent_language_loader
    }
};
use tower_lsp::{LspService, Server};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
//#endregion imports

#[tokio::main]
async fn main() {
    localizer().select(&["en".parse().unwrap()]).unwrap();

    //let stream = TcpStream::connect("127.0.0.1:8050").await.unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8050").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    
    let (read, write) = tokio::io::split(stream);

    #[cfg(feature = "runtime-agnostic")]
    let (read, write) = (read.compat(), write.compat_write());
    
    let (service, messages) = 
        LspService::build(|client| backend::Backend::new(client))
        .custom_method("documentation/getAll", backend::Backend::get_all_documentation)
        .finish();

    Server::new(read, write, messages)
        .serve(service)
        .await;
}

//#region localization
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
//#endregion localization