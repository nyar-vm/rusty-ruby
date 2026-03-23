//! Ruby Language Server Protocol implementation
//!
//! This crate provides a Language Server Protocol (LSP) implementation for Ruby
//! using the Oak LSP framework.

#![warn(missing_docs)]
#![feature(new_range_api)]

use core::range::Range;
use futures::Future;
use oak_core::tree::RedNode;
use oak_lsp::{
    Hover,
    service::LanguageService,
    types::{CompletionItem, Diagnostic, LocationRange},
};
use oak_ruby::RubyLanguage;
use oak_vfs::Vfs;
use std::sync::Arc;

pub mod lsp;
use lsp::RubyHoverProvider;

/// Language service implementation for Ruby.
pub struct RubyLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RubyHoverProvider,
}

impl<V: Vfs> RubyLanguageService<V> {
    /// Creates a new `RubyLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self { vfs, workspace: oak_lsp::workspace::WorkspaceManager::default(), hover_provider: RubyHoverProvider::new() }
    }

    /// Gets the root node of the parsed tree for the given URI.
    async fn with_root<F, T>(&self, uri: &str, f: F) -> Option<T>
    where
        F: FnOnce(&RedNode<'_, RubyLanguage>) -> Option<T>,
        V: oak_vfs::WritableVfs + Send + Sync + 'static,
    {
        match self.get_root(uri).await {
            Some(root) => f(&root),
            None => None,
        }
    }
}

impl<V: Vfs + Send + Sync + 'static + oak_vfs::WritableVfs> LanguageService for RubyLanguageService<V> {
    type Lang = RubyLanguage;
    type Vfs = V;

    fn vfs(&self) -> &Self::Vfs {
        &self.vfs
    }

    fn workspace(&self) -> &oak_lsp::workspace::WorkspaceManager {
        &self.workspace
    }

    fn get_root(&self, _uri: &str) -> impl Future<Output = Option<RedNode<'_, RubyLanguage>>> + Send + '_ {
        async move {
            // TODO: Implement proper caching of parsed trees in LanguageService
            None
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await }
    }

    fn completion(&self, _uri: &str, _offset: usize) -> impl Future<Output = Vec<CompletionItem>> + Send + '_ {
        async move {
            // TODO: Implement proper completion based on context
            let mut items = Vec::new();

            // Add some basic Ruby keywords
            let keywords = ["def", "class", "module", "if", "unless", "elsif", "else", "case", "when", "for", "while", "until"];

            for keyword in keywords {
                items.push(CompletionItem {
                    label: keyword.to_string(),
                    kind: Some(oak_lsp::types::CompletionItemKind::Keyword),
                    detail: Some(format!("Ruby keyword: {}", keyword)),
                    documentation: None,
                    insert_text: Some(keyword.to_string()),
                });
            }

            items
        }
    }

    fn definition(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + '_ {
        async move {
            // TODO: Implement proper definition finding
            vec![]
        }
    }

    fn references(&self, _uri: &str, _range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + '_ {
        async move {
            // TODO: Implement proper reference finding
            vec![]
        }
    }

    fn diagnostics(&self, _uri: &str) -> impl Future<Output = Vec<Diagnostic>> + Send + '_ {
        async move {
            // TODO: Implement proper diagnostics
            vec![]
        }
    }
}

/// Starts the Ruby LSP server.
pub async fn start_server() {
    let vfs = oak_vfs::MemoryVfs::new();
    let service = Arc::new(RubyLanguageService::new(vfs));
    let server = oak_lsp::server::LspServer::new(service);

    // Get standard input and output streams
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Run the server
    match server.run(stdin, stdout).await {
        Ok(_) => println!("Ruby LSP server exited successfully"),
        Err(e) => eprintln!("Ruby LSP server error: {}", e),
    }
}
