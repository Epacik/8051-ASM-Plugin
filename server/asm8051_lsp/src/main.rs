#![deny(warnings)]
//#region imports
mod backend;
mod client_configuration;
mod diagnostics;
mod flags;
mod i18n;
mod hover;

use std::borrow::Borrow;
use tower_lsp::{LspService, Server};
//#endregion imports

asm8051_localize::init!();

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    i18n::change_language("en");

    if args.contains(String::from("--use-stdio").borrow()) {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();
        serve(stdin, stdout).await;
    }
    else {
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