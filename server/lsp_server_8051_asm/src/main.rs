#![deny(warnings)]
//#region imports
mod backend;
mod client_configuration;
mod diagnostics;
mod flags;
mod hover;

use std::borrow::Borrow;

use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DefaultLocalizer, LanguageLoader, Localizer,
};
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;
use tower_lsp::{LspService, Server};
//#endregion imports

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    localizer().select(&["en".parse().unwrap()]).unwrap();

    if args.contains(String::from("--use-stdio").borrow()) {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        serve(stdin, stdout).await;
    } else {
        //let stream = TcpStream::connect("127.0.0.1:8050").await.unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8050")
            .await
            .unwrap();
        let (stream, _) = listener.accept().await.unwrap();

        let (read, write) = tokio::io::split(stream);

        #[cfg(feature = "runtime-agnostic")]
        let (read, write) = (read.compat(), write.compat_write());

        serve(read, write).await;
    }
}

async fn serve<I, O>(input: I, output: O)
where
    I: tokio::io::AsyncRead + Unpin,
    O: tokio::io::AsyncWrite,
{
    let (service, socket) = LspService::build(|client| backend::Backend::new(client))
        .custom_method(
            "documentation/getAll",
            backend::Backend::get_all_documentation,
        )
        .finish();

    Server::new(input, output, socket).serve(service).await;
}

pub(crate) static LANG_ID: &str = "asm8051";
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

pub(crate) fn localizer() -> Box<dyn Localizer> {
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
