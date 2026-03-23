//! Ruby Tools 库
//! 提供 Ruby 语言相关的工具链

#![warn(missing_docs)]

use oak_core::{Builder, TextEdit, builder::BuildOutput};
use oak_ruby::{RubyBuilder, RubyLanguage, ast::RubyRoot};

/// Rusty Ruby 前端
pub struct RustyRubyFrontend {
    language: RubyLanguage,
}

impl Default for RustyRubyFrontend {
    fn default() -> Self {
        Self::new()
    }
}

impl RustyRubyFrontend {
    /// 创建新的前端实例
    pub fn new() -> Self {
        Self { language: RubyLanguage::new() }
    }

    /// 解析 Ruby 代码
    pub fn parse(&self, source: &str) -> Result<RubyRoot, String> {
        let builder = RubyBuilder::new(&self.language);
        let edits: &[TextEdit] = &[];
        let mut cache = oak_core::parser::ParseSession::default();
        let output: BuildOutput<RubyLanguage> = builder.build(source, edits, &mut cache);
        output.result.map_err(|e| e.to_string())
    }
}
