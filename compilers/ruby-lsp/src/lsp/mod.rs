//! Ruby LSP functionality
//!
//! This module contains the LSP-related functionality for Ruby, including
//! hover providers, highlighters, and formatters.

pub mod formatter;
pub mod highlighter;

use core::range::Range;
use oak_core::tree::RedNode;
use oak_lsp::Hover;
use oak_ruby::RubyLanguage;

/// Hover provider implementation for Ruby.
pub struct RubyHoverProvider;

impl RubyHoverProvider {
    /// Creates a new `RubyHoverProvider`.
    pub fn new() -> Self {
        Self
    }
}

impl RubyHoverProvider {
    /// Provides hover information for Ruby code.
    pub fn hover(&self, node: &RedNode<RubyLanguage>, _range: Range<usize>) -> Option<Hover> {
        let kind = node.green.kind;
        // Provide context-aware hover information
        let contents = match kind {
            oak_ruby::RubyElementType::MethodDefinition => "### Ruby Method\nDefines a callable block of code.",
            oak_ruby::RubyElementType::ClassDefinition => "### Ruby Class\nDefines a blueprint for objects.",
            _ => return None,
        };
        Some(Hover { contents: contents.to_string(), range: Some(node.span()) })
    }
}
