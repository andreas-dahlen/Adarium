mod diagnostics;
mod did_open;

use std::path::{Path, PathBuf};
use tokio::io::{stdin, stdout};
use tower_lsp_server::{
    Client, LanguageServer, LspService, Server,
    jsonrpc::Result,
    ls_types::{
        DidOpenTextDocumentParams, InitializeParams, InitializeResult, MessageType,
        ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    },
};

use crate::config::{Config, load_config};

struct Backend {
    client: Client,
    root: PathBuf,
    config: tokio::sync::RwLock<Option<Config>>,
}

impl Backend {
    fn new(client: Client, root: &Path) -> Self {
        Self {
            client,
            root: root.to_path_buf(),
            config: tokio::sync::RwLock::new(None),
        }
    }
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let config_result = load_config(&self.root).map_err(|error| error.to_string());

        match config_result {
            Ok(config) => {
                *self.config.write().await = Some(config);
            }
            Err(error) => {
                self.client
                    .show_message(
                        MessageType::ERROR,
                        format!("Depend configuration error: {error}"),
                    )
                    .await;
            }
        }
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.handle_did_open(params).await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub(super) async fn lsp_start(root: &Path) {
    let root = root.to_path_buf();

    let (service, socket) = LspService::new(move |client| Backend::new(client, &root));

    Server::new(stdin(), stdout(), socket).serve(service).await;
}

// initialize	Tell the editor what capabilities you support.
// did_open	Analyze a newly opened document.
// did_change	Re-analyze when the document changes.
// did_close	Clear diagnostics for the document.
