use crate::{
    dependencies::create_file_dependency_graph_from_source,
    lsp::diagnostics::violation_to_diagnostic, namespaces::resolve_namespace_file,
    violations::find_violations,
};

use super::Backend;
use tower_lsp_server::ls_types::{Diagnostic, DidOpenTextDocumentParams};

impl Backend {
    pub(super) async fn handle_did_open(&self, params: DidOpenTextDocumentParams) {
        let config = self.config.read().await;

        let Some(config) = config.as_ref() else {
            return;
        };
        let document = params.text_document;

        let Some(absolute_path) = document.uri.to_file_path() else {
            log::warn!("Document URI is not a valid file URI: {:?}", document.uri);
            return;
        };

        let Ok(relative_path) = absolute_path.strip_prefix(&self.root) else {
            log::warn!("Document is outside project root: {:?}", absolute_path);
            return;
        };

        log::debug!("Opened document: {:?}", relative_path);
        log::debug!("Document version: {}", document.version);
        log::debug!("Document length: {}", document.text.len());

        let file_graph = match create_file_dependency_graph_from_source(
            &self.root,
            relative_path,
            &document.text,
        ) {
            Ok(graph) => graph,
            Err(error) => {
                log::error!("Failed to build dependency graph: {error}");
                return;
            }
        };

        let namespaces = match resolve_namespace_file(&config.folders, &config.files, &file_graph) {
            Ok(namespaces) => namespaces,
            Err(error) => {
                log::error!("Failed to resolve namespaces: {error}");
                return;
            }
        };

        let violations = find_violations(&config.rules, &namespaces.namespace_edges);

        let diagnostics: Vec<Diagnostic> = violations.iter().map(violation_to_diagnostic).collect();

        self.client
            .publish_diagnostics(document.uri, diagnostics, Some(document.version))
            .await;
    }
}
