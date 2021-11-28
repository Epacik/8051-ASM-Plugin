use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tower_lsp::lsp_types::request::{GotoDeclarationParams, GotoDeclarationResponse, GotoImplementationParams, GotoImplementationResponse, GotoTypeDefinitionParams, GotoTypeDefinitionResponse};


#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {

        // time to set some capabilities, so the client knows what server can do
        let mut result = InitializeResult::default();
        result.capabilities.text_document_sync = Some(TextDocumentSyncCapability::from(TextDocumentSyncKind::Incremental));
        result.capabilities.completion_provider = Some(CompletionOptions{
            resolve_provider: Option::from(true),
            trigger_characters: None,
            all_commit_characters: None,
            work_done_progress_options: Default::default()
        });
        result.capabilities.hover_provider = Option::from(HoverProviderCapability::Simple(true));
        Ok(result)
    }

    async fn initialized(&self, _: InitializedParams) {
        print!("server initialized!");
        self.client
            .log_message(MessageType::Info, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        print!("server shutdown!");
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        print!("completion!");
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string())
        ])))
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        print!("hover!");
        Ok(Some(Hover {
            contents: HoverContents::Scalar(
                MarkedString::String("You're hovering!".to_string())
            ),
            range: None
        }))
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        print!("did change!");
        self.client
            .log_message(MessageType::Info, "file did change")
            .await;
    }



    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        print!("did open!");
        self.client
            .log_message(MessageType::Info, "file opened")
            .await;
    }
}

#[tokio::main]
async fn main() {

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8050").await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = tokio::io::split(stream);

    let (service, messages) = LspService::new(|client| Backend { client });
    Server::new(read, write)
        .interleave(messages)
        .serve(service)
        .await;
}