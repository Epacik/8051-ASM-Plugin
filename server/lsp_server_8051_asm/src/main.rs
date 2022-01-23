#![deny(warnings)]

mod backend;
mod client_configuration;
mod diagnostics;
mod flags;
mod hover_documentation;

use crate::client_configuration::ClientConfiguration;
use lspower::{LspService, Server};
use std::sync::Mutex;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8050")
        .await
        .unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = tokio::io::split(stream);

    let (service, messages) = LspService::new(|client| backend::Backend {
        client,
        documents: Mutex::new(HashMap::new()),
        locale: Mutex::new(None),
        client_capabilities: Mutex::new(None),
        client_configuration: Mutex::new(ClientConfiguration::default()),
    });

    Server::new(read, write)
        .interleave(messages)
        .serve(service)
        .await;
}

// mod tests {
//     #[test]    
//     fn test_doc_loading() {
//         let _docs: std::collections::HashMap<crate::flags::Locale, std::collections::HashMap<String, crate::hover_documentation::documentation::Documentation>> = std::collections::HashMap::from([
//             (Locale::ENGLISH, load_documentation::load_documentation!(english)),
//             (Locale::POLISH, load_documentation::load_documentation!(polish)),
//         ]);
//         let locale = crate::flags::Locale::ENGLISH;
//         assert_eq!(_docs.contains_key(&locale), true);
//         let _key = String::from("ADD");
//         let _eng = _docs.get(&locale).unwrap().keys();
//         assert_eq!(true, true);
//     }
// }
