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

