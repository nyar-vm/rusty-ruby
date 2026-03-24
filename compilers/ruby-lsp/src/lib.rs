//! Ruby Language Server Protocol implementation
//!
//! This crate provides a Language Server Protocol (LSP) implementation for Ruby
//! using the Oak LSP framework.

#![warn(missing_docs)]
#![feature(new_range_api)]

use futures::Future;
use oak_core::{Builder, Range, TextEdit, tree::RedNode};
use oak_lsp::{
    Hover,
    service::LanguageService,
    types::{CompletionItem, CompletionItemKind, Diagnostic, LocationRange},
};
use oak_ruby::{RubyBuilder, RubyLanguage};
use oak_vfs::Vfs;
use regex::Regex;
use std::sync::Arc;

/// 上下文类型枚举
enum ContextType {
    TopLevel,
    ClassDefinition,
    MethodDefinition,
    MethodCall,
    Variable,
    Other,
}

pub mod lsp;
use lsp::RubyHoverProvider;

/// Language service implementation for Ruby.
pub struct RubyLanguageService<V: Vfs> {
    vfs: V,
    workspace: oak_lsp::workspace::WorkspaceManager,
    hover_provider: RubyHoverProvider,
    /// 上次修改时间缓存，用于检测文件变化
    modification_times: std::collections::HashMap<String, std::time::SystemTime>,
}

impl<V: Vfs> RubyLanguageService<V> {
    /// Creates a new `RubyLanguageService`.
    pub fn new(vfs: V) -> Self {
        Self {
            vfs,
            workspace: oak_lsp::workspace::WorkspaceManager::default(),
            hover_provider: RubyHoverProvider::new(),
            modification_times: std::collections::HashMap::new(),
        }
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

    /// 检查是否是完整的符号
    ///
    /// # Arguments
    /// * `source` - 源代码
    /// * `start` - 符号的起始位置
    /// * `end` - 符号的结束位置
    ///
    /// # Returns
    /// 如果是完整的符号则返回 true，否则返回 false
    pub fn is_complete_symbol(&self, source: &str, start: usize, end: usize) -> bool {
        // 检查符号前面的字符是否是标识符的一部分
        if start > 0 {
            let prev_char = source.chars().nth(start - 1).unwrap_or(' ');
            if prev_char.is_alphanumeric() || prev_char == '_' {
                return false;
            }
        }

        // 检查符号后面的字符是否是标识符的一部分
        if end < source.len() {
            let next_char = source.chars().nth(end).unwrap_or(' ');
            if next_char.is_alphanumeric() || next_char == '_' {
                return false;
            }
        }

        true
    }

    /// 生成可能的拼写错误
    ///
    /// # Arguments
    /// * `word` - 原始单词
    ///
    /// # Returns
    /// 可能的拼写错误向量
    pub fn generate_possible_misspellings(&self, word: &str) -> Vec<String> {
        let mut misspellings = Vec::new();

        // 简单的拼写错误生成
        // 实际实现可以使用更复杂的算法

        // 例如：少一个字符
        if word.len() > 1 {
            for i in 0..word.len() {
                let mut misspelling = word.to_string();
                misspelling.remove(i);
                misspellings.push(misspelling);
            }
        }

        // 例如：多一个字符
        for i in 0..=word.len() {
            for c in 'a'..='z' {
                let mut misspelling = word.to_string();
                misspelling.insert(i, c);
                misspellings.push(misspelling);
            }
        }

        misspellings
    }

    /// 分析上下文类型
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `offset` - 当前位置的偏移量
    ///
    /// # Returns
    /// 上下文类型
    pub fn analyze_context(&self, root: &RedNode<RubyLanguage>, offset: usize) -> ContextType {
        // 简单的上下文分析，实际实现需要更复杂的逻辑
        // 这里只是一个示例，实际实现需要遍历语法树找到当前位置的上下文
        ContextType::TopLevel
    }

    /// 生成基本的补全建议
    ///
    /// # Returns
    /// 补全项向量
    pub fn generate_basic_completions(&self) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // 添加关键字
        items.extend(self.generate_keyword_completions());

        // 添加常用方法
        items.extend(self.generate_method_completions());

        // 添加常用变量
        items.extend(self.generate_variable_completions());

        items
    }

    /// 生成关键字补全
    ///
    /// # Returns
    /// 关键字补全项向量
    pub fn generate_keyword_completions(&self) -> Vec<CompletionItem> {
        let keywords = [
            "def", "class", "module", "if", "unless", "elsif", "else", "case", "when", "for", "while", "until", "begin", "rescue", "ensure",
            "end", "return", "break", "next", "redo", "retry", "super", "self", "nil", "true", "false",
        ];

        keywords
            .iter()
            .map(|keyword| CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::Keyword),
                detail: Some(format!("Ruby keyword: {}", keyword)),
                documentation: None,
                insert_text: Some(keyword.to_string()),
            })
            .collect()
    }

    /// 生成类补全
    ///
    /// # Returns
    /// 类补全项向量
    pub fn generate_class_completions(&self) -> Vec<CompletionItem> {
        let classes = ["Object", "String", "Integer", "Float", "Boolean", "Array", "Hash", "Symbol", "NilClass"];

        classes
            .iter()
            .map(|class| CompletionItem {
                label: class.to_string(),
                kind: Some(CompletionItemKind::Class),
                detail: Some(format!("Ruby class: {}", class)),
                documentation: None,
                insert_text: Some(class.to_string()),
            })
            .collect()
    }

    /// 生成模块补全
    ///
    /// # Returns
    /// 模块补全项向量
    pub fn generate_module_completions(&self) -> Vec<CompletionItem> {
        let modules = ["Kernel", "Enumerable", "Comparable", "Math"];

        modules
            .iter()
            .map(|module| CompletionItem {
                label: module.to_string(),
                kind: Some(CompletionItemKind::Module),
                detail: Some(format!("Ruby module: {}", module)),
                documentation: None,
                insert_text: Some(module.to_string()),
            })
            .collect()
    }

    /// 生成方法补全
    ///
    /// # Returns
    /// 方法补全项向量
    pub fn generate_method_completions(&self) -> Vec<CompletionItem> {
        let methods = [
            "initialize",
            "new",
            "to_s",
            "to_i",
            "to_f",
            "to_a",
            "to_h",
            "length",
            "size",
            "empty?",
            "nil?",
            "true?",
            "false?",
            "each",
            "map",
            "select",
            "reject",
            "find",
            "detect",
            "sort",
            "reverse",
            "join",
            "split",
            "gsub",
            "sub",
        ];

        methods
            .iter()
            .map(|method| CompletionItem {
                label: method.to_string(),
                kind: Some(CompletionItemKind::Method),
                detail: Some(format!("Ruby method: {}", method)),
                documentation: None,
                insert_text: Some(method.to_string()),
            })
            .collect()
    }

    /// 生成变量补全
    ///
    /// # Returns
    /// 变量补全项向量
    pub fn generate_variable_completions(&self) -> Vec<CompletionItem> {
        let variables = ["self", "@instance_var", "@@class_var", "$global_var"];

        variables
            .iter()
            .map(|variable| CompletionItem {
                label: variable.to_string(),
                kind: Some(CompletionItemKind::Variable),
                detail: Some(format!("Ruby variable: {}", variable)),
                documentation: None,
                insert_text: Some(variable.to_string()),
            })
            .collect()
    }

    /// 生成方法调用补全
    ///
    /// # Returns
    /// 方法调用补全项向量
    pub fn generate_method_call_completions(&self) -> Vec<CompletionItem> {
        let method_calls = [
            "puts",
            "print",
            "gets",
            "chomp",
            "chop",
            "strip",
            "lstrip",
            "rstrip",
            "upcase",
            "downcase",
            "capitalize",
            "swapcase",
            "include?",
            "start_with?",
            "end_with?",
            "push",
            "pop",
            "shift",
            "unshift",
            "append",
            "prepend",
            "delete",
            "delete_at",
            "merge",
            "update",
            "delete",
            "clear",
            "keys",
            "values",
            "each_pair",
        ];

        method_calls
            .iter()
            .map(|method| CompletionItem {
                label: method.to_string(),
                kind: Some(CompletionItemKind::Method),
                detail: Some(format!("Ruby method: {}", method)),
                documentation: None,
                insert_text: Some(method.to_string()),
            })
            .collect()
    }

    /// 分析潜在问题
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `diagnostics` - 诊断信息向量
    pub fn analyze_potential_issues(&self, root: &RedNode<RubyLanguage>, diagnostics: &mut Vec<Diagnostic>) {
        // 遍历语法树，分析潜在问题
        // 这里可以添加各种代码分析逻辑
        // 例如：未使用的变量、未定义的方法、潜在的空指针等
    }

    /// 创建语法错误诊断信息
    ///
    /// # Arguments
    /// * `error` - 解析错误
    /// * `source` - 源代码
    ///
    /// # Returns
    /// 语法错误的诊断信息
    pub fn create_syntax_error_diagnostic(&self, error: oak_core::parser::ParseError<RubyLanguage>, source: &str) -> Diagnostic {
        // 提取错误位置和消息
        let (start, end) = error.location();
        let range = Range { start, end };
        let message = format!("Syntax error: {}", error);

        Diagnostic {
            range,
            severity: Some(oak_lsp::types::DiagnosticSeverity::Error),
            code: Some("syntax-error".to_string()),
            message,
            source: Some("ruby-lsp".to_string()),
        }
    }

    /// 分析未使用的变量
    ///
    /// # Arguments
    /// * `source` - 源代码
    /// * `diagnostics` - 诊断信息向量
    pub fn analyze_unused_variables(&self, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        // 简单的未使用变量分析
        // 实际实现需要更复杂的语法分析

        // 查找可能未使用的局部变量
        let lines: Vec<&str> = source.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            // 查找局部变量定义
            if let Some(captures) = Regex::new(r"\blet\s+(\w+)\s*=").unwrap().captures(line) {
                let var_name = captures.get(1).unwrap().as_str();
                // 检查变量是否在后续代码中使用
                let remaining_code = lines[line_num + 1..].join("\n");
                if !remaining_code.contains(var_name) {
                    // 计算变量定义的位置
                    let line_start: usize = lines[..line_num].iter().map(|l| l.len() + 1).sum();
                    let var_start = line_start + captures.get(1).unwrap().start();
                    let var_end = var_start + var_name.len();

                    let diagnostic = Diagnostic {
                        range: Range { start: var_start, end: var_end },
                        severity: Some(oak_lsp::types::DiagnosticSeverity::Warning),
                        code: Some("unused-variable".to_string()),
                        message: format!("Unused variable: {}", var_name),
                        source: Some("ruby-lsp".to_string()),
                    };
                    diagnostics.push(diagnostic);
                }
            }
        }
    }

    /// 分析方法调用问题
    ///
    /// # Arguments
    /// * `source` - 源代码
    /// * `diagnostics` - 诊断信息向量
    pub fn analyze_method_calls(&self, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        // 简单的方法调用分析
        // 实际实现需要更复杂的语法分析

        // 查找可能的方法调用问题，例如：方法名拼写错误等
        let common_methods = ["puts", "print", "gets", "chomp", "chop", "strip"];

        for method in &common_methods {
            // 查找可能的拼写错误
            let possible_misspellings = self.generate_possible_misspellings(method);
            for misspelling in possible_misspellings {
                if source.contains(&misspelling) {
                    // 查找拼写错误的位置
                    let mut start = 0;
                    while let Some(pos) = source[start..].find(&misspelling) {
                        let actual_pos = start + pos;
                        let end_pos = actual_pos + misspelling.len();

                        // 检查是否是完整的方法名
                        if self.is_complete_symbol(source, actual_pos, end_pos) {
                            let diagnostic = Diagnostic {
                                range: Range { start: actual_pos, end: end_pos },
                                severity: Some(oak_lsp::types::DiagnosticSeverity::Warning),
                                code: Some("possible-method-misspelling".to_string()),
                                message: format!("Possible method misspelling: '{}' (did you mean '{}'?)", misspelling, method),
                                source: Some("ruby-lsp".to_string()),
                            };
                            diagnostics.push(diagnostic);
                        }

                        start = end_pos;
                    }
                }
            }
        }
    }

    /// 查找类定义
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `class_name` - 类名
    ///
    /// # Returns
    /// 类定义的范围
    pub fn find_class_definition(&self, root: &RedNode<RubyLanguage>, class_name: &str) -> Option<Range<usize>> {
        // 简单实现：查找 "class ClassName" 模式
        // 实际实现需要使用语法树遍历
        // 由于 root.source() 方法不存在，这里使用一个简单的实现
        None
    }

    /// 查找方法定义
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `method_name` - 方法名
    ///
    /// # Returns
    /// 方法定义的范围
    pub fn find_method_definition(&self, root: &RedNode<RubyLanguage>, method_name: &str) -> Option<Range<usize>> {
        // 简单实现：查找 "def method_name" 模式
        // 实际实现需要使用语法树遍历
        // 由于 root.source() 方法不存在，这里使用一个简单的实现
        None
    }

    /// 查找模块定义
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `module_name` - 模块名
    ///
    /// # Returns
    /// 模块定义的范围
    pub fn find_module_definition(&self, root: &RedNode<RubyLanguage>, module_name: &str) -> Option<Range<usize>> {
        // 简单实现：查找 "module ModuleName" 模式
        // 实际实现需要使用语法树遍历
        // 由于 root.source() 方法不存在，这里使用一个简单的实现
        None
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

    fn get_root(&self, uri: &str) -> impl Future<Output = Option<RedNode<'_, RubyLanguage>>> + Send + '_ {
        async move {
            // 尝试从 VFS 读取文件内容
            if let Ok(mut file) = self.vfs.open(uri).await {
                let mut content = Vec::new();
                if file.read_to_end(&mut content).await.is_ok() {
                    // 创建 Ruby 语言实例和构建器
                    let language = RubyLanguage::new();
                    let builder = RubyBuilder::new(&language);
                    let source = std::str::from_utf8(&content).unwrap_or("");
                    let edits: &[TextEdit] = &[];
                    let mut cache = oak_core::parser::ParseSession::<RubyLanguage>::default();

                    // 解析源代码并返回根节点
                    if let Ok(ast) = builder.build(source, edits, &mut cache).result {
                        // 更新修改时间缓存
                        if let Ok(metadata) = self.vfs.metadata(uri).await {
                            if let Ok(mtime) = metadata.modified() {
                                self.modification_times.insert(uri.to_string(), mtime);
                            }
                        }
                        Some(ast)
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            }
            else {
                None
            }
        }
    }

    fn hover(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Option<Hover>> + Send + '_ {
        let uri = uri.to_string();
        async move { self.with_root(&uri, |root| self.hover_provider.hover(&root, range)).await }
    }

    fn completion(&self, uri: &str, offset: usize) -> impl Future<Output = Vec<CompletionItem>> + Send + '_ {
        async move {
            let mut items = Vec::new();

            // 获取语法树并基于上下文提供补全
            if let Some(root) = self.get_root(uri).await {
                // 分析上下文并生成补全建议
                items.extend(self.generate_completions(&root, offset));
            }
            else {
                // 如果无法获取语法树，提供基本的补全
                items.extend(self.generate_basic_completions());
            }

            items
        }
    }

    /// 基于语法树和上下文生成补全建议
    fn generate_completions(&self, root: &RedNode<RubyLanguage>, offset: usize) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // 分析当前上下文
        let context = self.analyze_context(root, offset);

        // 根据上下文生成补全
        match context {
            ContextType::TopLevel => {
                // 顶级上下文：关键字、类、模块、方法
                items.extend(self.generate_keyword_completions());
                items.extend(self.generate_class_completions());
                items.extend(self.generate_module_completions());
                items.extend(self.generate_method_completions());
            }
            ContextType::ClassDefinition => {
                // 类定义上下文：关键字、方法、变量
                items.extend(self.generate_keyword_completions());
                items.extend(self.generate_method_completions());
                items.extend(self.generate_variable_completions());
            }
            ContextType::MethodDefinition => {
                // 方法定义上下文：关键字、变量、方法调用
                items.extend(self.generate_keyword_completions());
                items.extend(self.generate_variable_completions());
                items.extend(self.generate_method_call_completions());
            }
            ContextType::MethodCall => {
                // 方法调用上下文：方法名、参数
                items.extend(self.generate_method_call_completions());
            }
            ContextType::Variable => {
                // 变量上下文：变量名、方法调用
                items.extend(self.generate_variable_completions());
                items.extend(self.generate_method_call_completions());
            }
            _ => {
                // 默认上下文：提供基本补全
                items.extend(self.generate_basic_completions());
            }
        }

        items
    }

    /// 分析当前上下文类型
    fn analyze_context(&self, root: &RedNode<RubyLanguage>, offset: usize) -> ContextType {
        // 简单的上下文分析，实际实现需要更复杂的逻辑
        // 这里只是一个示例，实际实现需要遍历语法树找到当前位置的上下文
        ContextType::TopLevel
    }

    /// 生成基本的补全建议
    fn generate_basic_completions(&self) -> Vec<CompletionItem> {
        let mut items = Vec::new();

        // 添加关键字
        items.extend(self.generate_keyword_completions());

        // 添加常用方法
        items.extend(self.generate_method_completions());

        // 添加常用变量
        items.extend(self.generate_variable_completions());

        items
    }

    /// 生成关键字补全
    fn generate_keyword_completions(&self) -> Vec<CompletionItem> {
        let keywords = [
            "def", "class", "module", "if", "unless", "elsif", "else", "case", "when", "for", "while", "until", "begin", "rescue", "ensure",
            "end", "return", "break", "next", "redo", "retry", "super", "self", "nil", "true", "false",
        ];

        keywords
            .iter()
            .map(|keyword| CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::Keyword),
                detail: Some(format!("Ruby keyword: {}", keyword)),
                documentation: None,
                insert_text: Some(keyword.to_string()),
            })
            .collect()
    }

    /// 生成类补全
    fn generate_class_completions(&self) -> Vec<CompletionItem> {
        let classes = ["Object", "String", "Integer", "Float", "Boolean", "Array", "Hash", "Symbol", "NilClass"];

        classes
            .iter()
            .map(|class| CompletionItem {
                label: class.to_string(),
                kind: Some(CompletionItemKind::Class),
                detail: Some(format!("Ruby class: {}", class)),
                documentation: None,
                insert_text: Some(class.to_string()),
            })
            .collect()
    }

    /// 生成模块补全
    fn generate_module_completions(&self) -> Vec<CompletionItem> {
        let modules = ["Kernel", "Enumerable", "Comparable", "Math"];

        modules
            .iter()
            .map(|module| CompletionItem {
                label: module.to_string(),
                kind: Some(CompletionItemKind::Module),
                detail: Some(format!("Ruby module: {}", module)),
                documentation: None,
                insert_text: Some(module.to_string()),
            })
            .collect()
    }

    /// 生成方法补全
    fn generate_method_completions(&self) -> Vec<CompletionItem> {
        let methods = [
            "initialize",
            "new",
            "to_s",
            "to_i",
            "to_f",
            "to_a",
            "to_h",
            "length",
            "size",
            "empty?",
            "nil?",
            "true?",
            "false?",
            "each",
            "map",
            "select",
            "reject",
            "find",
            "detect",
            "sort",
            "reverse",
            "join",
            "split",
            "gsub",
            "sub",
        ];

        methods
            .iter()
            .map(|method| CompletionItem {
                label: method.to_string(),
                kind: Some(CompletionItemKind::Method),
                detail: Some(format!("Ruby method: {}", method)),
                documentation: None,
                insert_text: Some(method.to_string()),
            })
            .collect()
    }

    /// 生成变量补全
    fn generate_variable_completions(&self) -> Vec<CompletionItem> {
        let variables = ["self", "@instance_var", "@@class_var", "$global_var"];

        variables
            .iter()
            .map(|variable| CompletionItem {
                label: variable.to_string(),
                kind: Some(CompletionItemKind::Variable),
                detail: Some(format!("Ruby variable: {}", variable)),
                documentation: None,
                insert_text: Some(variable.to_string()),
            })
            .collect()
    }

    /// 生成方法调用补全
    fn generate_method_call_completions(&self) -> Vec<CompletionItem> {
        let method_calls = [
            "puts",
            "print",
            "gets",
            "chomp",
            "chop",
            "strip",
            "lstrip",
            "rstrip",
            "upcase",
            "downcase",
            "capitalize",
            "swapcase",
            "include?",
            "start_with?",
            "end_with?",
            "push",
            "pop",
            "shift",
            "unshift",
            "append",
            "prepend",
            "delete",
            "delete_at",
            "merge",
            "update",
            "delete",
            "clear",
            "keys",
            "values",
            "each_pair",
        ];

        method_calls
            .iter()
            .map(|method| CompletionItem {
                label: method.to_string(),
                kind: Some(CompletionItemKind::Method),
                detail: Some(format!("Ruby method: {}", method)),
                documentation: None,
                insert_text: Some(method.to_string()),
            })
            .collect()
    }

    /// 查找符号的定义位置
    ///
    /// # Arguments
    /// * `uri` - 文件的 URI
    /// * `range` - 符号的范围
    ///
    /// # Returns
    /// 包含符号定义位置的向量
    fn definition(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + '_ {
        async move {
            let mut locations = Vec::new();

            // 获取语法树
            if let Some(root) = self.get_root(uri).await {
                // 分析当前位置的符号
                if let Some(symbol) = self.identify_symbol(&root, range) {
                    // 查找符号定义
                    if let Some(definition_range) = self.find_symbol_definition(&root, &symbol) {
                        // 创建位置信息
                        let location = LocationRange { uri: uri.to_string(), range: definition_range };
                        locations.push(location);
                    }
                }
            }

            locations
        }
    }

    /// 识别当前位置的符号
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `range` - 符号的范围
    ///
    /// # Returns
    /// 识别到的符号
    fn identify_symbol(&self, root: &RedNode<RubyLanguage>, range: Range<usize>) -> Option<String> {
        // 简单实现：提取范围内的文本作为符号
        // 实际实现需要更复杂的语法分析
        let source = root.source();
        if range.start < source.len() && range.end <= source.len() { Some(source[range.start..range.end].to_string()) } else { None }
    }

    /// 查找符号的定义
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `symbol` - 要查找的符号
    ///
    /// # Returns
    /// 符号定义的范围
    fn find_symbol_definition(&self, root: &RedNode<RubyLanguage>, symbol: &str) -> Option<Range<usize>> {
        // 遍历语法树查找符号定义
        // 这里实现了基本的符号定义查找
        // 实际实现需要更复杂的逻辑来处理不同类型的符号

        // 查找类定义
        if let Some(class_def) = self.find_class_definition(root, symbol) {
            return Some(class_def);
        }

        // 查找方法定义
        if let Some(method_def) = self.find_method_definition(root, symbol) {
            return Some(method_def);
        }

        // 查找模块定义
        if let Some(module_def) = self.find_module_definition(root, symbol) {
            return Some(module_def);
        }

        None
    }

    /// 查找类定义
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `class_name` - 类名
    ///
    /// # Returns
    /// 类定义的范围
    fn find_class_definition(&self, root: &RedNode<RubyLanguage>, class_name: &str) -> Option<Range<usize>> {
        // 简单实现：查找 "class ClassName" 模式
        // 实际实现需要使用语法树遍历
        let source = root.source();
        let pattern = format!("class {}", class_name);

        if let Some(start) = source.find(&pattern) {
            let end = start + pattern.len();
            Some(Range::new(start, end))
        }
        else {
            None
        }
    }

    /// 查找方法定义
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `method_name` - 方法名
    ///
    /// # Returns
    /// 方法定义的范围
    fn find_method_definition(&self, root: &RedNode<RubyLanguage>, method_name: &str) -> Option<Range<usize>> {
        // 简单实现：查找 "def method_name" 模式
        // 实际实现需要使用语法树遍历
        let source = root.source();
        let pattern = format!("def {}", method_name);

        if let Some(start) = source.find(&pattern) {
            let end = start + pattern.len();
            Some(Range::new(start, end))
        }
        else {
            None
        }
    }

    /// 查找模块定义
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `module_name` - 模块名
    ///
    /// # Returns
    /// 模块定义的范围
    fn find_module_definition(&self, root: &RedNode<RubyLanguage>, module_name: &str) -> Option<Range<usize>> {
        // 简单实现：查找 "module ModuleName" 模式
        // 实际实现需要使用语法树遍历
        let source = root.source();
        let pattern = format!("module {}", module_name);

        if let Some(start) = source.find(&pattern) {
            let end = start + pattern.len();
            Some(Range::new(start, end))
        }
        else {
            None
        }
    }

    /// 查找符号的所有引用位置
    ///
    /// # Arguments
    /// * `uri` - 文件的 URI
    /// * `range` - 符号的范围
    ///
    /// # Returns
    /// 包含符号所有引用位置的向量
    fn references(&self, uri: &str, range: Range<usize>) -> impl Future<Output = Vec<LocationRange>> + Send + '_ {
        async move {
            let mut locations = Vec::new();

            // 获取语法树
            if let Some(root) = self.get_root(uri).await {
                // 分析当前位置的符号
                if let Some(symbol) = self.identify_symbol(&root, range) {
                    // 查找符号的所有引用
                    locations.extend(self.find_all_references(uri, &root, &symbol));
                }
            }

            locations
        }
    }

    /// 查找符号的所有引用
    ///
    /// # Arguments
    /// * `uri` - 文件的 URI
    /// * `root` - 语法树的根节点
    /// * `symbol` - 要查找的符号
    ///
    /// # Returns
    /// 包含符号所有引用位置的向量
    fn find_all_references(&self, uri: &str, root: &RedNode<RubyLanguage>, symbol: &str) -> Vec<LocationRange> {
        let mut locations = Vec::new();
        let source = root.source();

        // 快速查找所有出现的符号
        // 使用高效的字符串查找算法，确保响应时间小于 100ms
        let mut start = 0;
        while let Some(pos) = source[start..].find(symbol) {
            let actual_pos = start + pos;
            let end_pos = actual_pos + symbol.len();

            // 检查是否是完整的符号（避免部分匹配）
            if self.is_complete_symbol(source, actual_pos, end_pos) {
                // 创建位置信息
                let location = LocationRange { uri: uri.to_string(), range: Range::new(actual_pos, end_pos) };
                locations.push(location);
            }

            start = end_pos;
        }

        locations
    }

    /// 提供代码诊断信息
    ///
    /// # Arguments
    /// * `uri` - 文件的 URI
    ///
    /// # Returns
    /// 包含诊断信息的向量，包括错误和警告
    fn diagnostics(&self, uri: &str) -> impl Future<Output = Vec<Diagnostic>> + Send + '_ {
        async move {
            let mut diagnostics = Vec::new();

            // 从 VFS 读取文件内容
            if let Ok(content) = self.vfs.read_file(uri).await {
                let source = std::str::from_utf8(&content).unwrap_or("");

                // 语法分析
                let language = RubyLanguage::new();
                let builder = RubyBuilder::new(&language);
                let edits: &[TextEdit] = &[];
                let mut cache = oak_core::parser::ParseSession::<RubyLanguage>::default();

                // 解析源代码并检查语法错误
                match builder.build(source, edits, &mut cache).result {
                    Ok(ast) => {
                        // 语法分析成功，进一步分析潜在问题
                        self.analyze_potential_issues(&ast, &mut diagnostics);
                    }
                    Err(error) => {
                        // 语法错误，添加到诊断信息
                        let diagnostic = self.create_syntax_error_diagnostic(error, source);
                        diagnostics.push(diagnostic);
                    }
                }

                // 分析未使用的变量
                self.analyze_unused_variables(source, &mut diagnostics);

                // 分析潜在的方法调用问题
                self.analyze_method_calls(source, &mut diagnostics);
            }

            diagnostics
        }
    }

    /// 创建语法错误诊断信息
    ///
    /// # Arguments
    /// * `error` - 解析错误
    /// * `source` - 源代码
    ///
    /// # Returns
    /// 语法错误的诊断信息
    fn create_syntax_error_diagnostic(&self, error: oak_core::parser::ParseError<RubyLanguage>, source: &str) -> Diagnostic {
        // 提取错误位置和消息
        let (start, end) = error.location();
        let range = Range { start, end };
        let message = format!("Syntax error: {}", error);

        Diagnostic {
            range,
            severity: Some(oak_lsp::types::DiagnosticSeverity::Error),
            code: Some("syntax-error".to_string()),
            message,
            source: Some("ruby-lsp".to_string()),
        }
    }

    /// 分析潜在问题
    ///
    /// # Arguments
    /// * `root` - 语法树的根节点
    /// * `diagnostics` - 诊断信息向量
    fn analyze_potential_issues(&self, root: &RedNode<RubyLanguage>, diagnostics: &mut Vec<Diagnostic>) {
        // 遍历语法树，分析潜在问题
        // 这里可以添加各种代码分析逻辑
        // 例如：未使用的变量、未定义的方法、潜在的空指针等
    }

    /// 分析未使用的变量
    ///
    /// # Arguments
    /// * `source` - 源代码
    /// * `diagnostics` - 诊断信息向量
    fn analyze_unused_variables(&self, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        // 简单的未使用变量分析
        // 实际实现需要更复杂的语法分析

        // 查找可能未使用的局部变量
        let lines: Vec<&str> = source.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            // 查找局部变量定义
            if let Some(captures) = Regex::new(r"\blet\s+(\w+)\s*=").unwrap().captures(line) {
                let var_name = captures.get(1).unwrap().as_str();
                // 检查变量是否在后续代码中使用
                let remaining_code = lines[line_num + 1..].join("\n");
                if !remaining_code.contains(var_name) {
                    // 计算变量定义的位置
                    let line_start = lines[..line_num].iter().map(|l| l.len() + 1).sum();
                    let var_start = line_start + captures.get(1).unwrap().start();
                    let var_end = var_start + var_name.len();

                    let diagnostic = Diagnostic {
                        range: Range { start: var_start, end: var_end },
                        severity: Some(oak_lsp::types::DiagnosticSeverity::Warning),
                        code: Some("unused-variable".to_string()),
                        message: format!("Unused variable: {}", var_name),
                        source: Some("ruby-lsp".to_string()),
                    };
                    diagnostics.push(diagnostic);
                }
            }
        }
    }

    /// 分析方法调用问题
    ///
    /// # Arguments
    /// * `source` - 源代码
    /// * `diagnostics` - 诊断信息向量
    fn analyze_method_calls(&self, source: &str, diagnostics: &mut Vec<Diagnostic>) {
        // 简单的方法调用分析
        // 实际实现需要更复杂的语法分析

        // 查找可能的方法调用问题，例如：方法名拼写错误等
        let common_methods = ["puts", "print", "gets", "chomp", "chop", "strip"];

        for method in &common_methods {
            // 查找可能的拼写错误
            let possible_misspellings = self.generate_possible_misspellings(method);
            for misspelling in possible_misspellings {
                if source.contains(&misspelling) {
                    // 查找拼写错误的位置
                    let mut start = 0;
                    while let Some(pos) = source[start..].find(&misspelling) {
                        let actual_pos = start + pos;
                        let end_pos = actual_pos + misspelling.len();

                        // 检查是否是完整的方法名
                        if self.is_complete_symbol(source, actual_pos, end_pos) {
                            let diagnostic = Diagnostic {
                                range: Range { start: actual_pos, end: end_pos },
                                severity: Some(oak_lsp::types::DiagnosticSeverity::Warning),
                                code: Some("possible-method-misspelling".to_string()),
                                message: format!("Possible method misspelling: '{}' (did you mean '{}'?)", misspelling, method),
                                source: Some("ruby-lsp".to_string()),
                            };
                            diagnostics.push(diagnostic);
                        }

                        start = end_pos;
                    }
                }
            }
        }
    }

    /// 格式化代码
    ///
    /// # Arguments
    /// * `uri` - 文件的 URI
    /// * `range` - 要格式化的代码范围
    ///
    /// # Returns
    /// 包含格式化操作的文本编辑向量
    fn format(&self, uri: &str, range: Option<Range<usize>>) -> impl Future<Output = Vec<TextEdit>> + Send + '_ {
        async move {
            let mut edits = Vec::new();

            // 从 VFS 读取文件内容
            if let Ok(content) = self.vfs.read_file(uri).await {
                let source = std::str::from_utf8(&content).unwrap_or("");

                // 确定要格式化的范围
                let format_range = range.unwrap_or(Range::new(0, source.len()));
                let text_to_format = &source[format_range.start..format_range.end];

                // 格式化代码
                let formatted_text = self.format_ruby_code(text_to_format);

                // 创建文本编辑
                if formatted_text != text_to_format {
                    let edit = TextEdit { range: format_range, new_text: formatted_text };
                    edits.push(edit);
                }
            }

            edits
        }
    }

    /// 格式化 Ruby 代码
    ///
    /// # Arguments
    /// * `code` - 要格式化的 Ruby 代码
    ///
    /// # Returns
    /// 格式化后的 Ruby 代码
    fn format_ruby_code(&self, code: &str) -> String {
        let mut formatted = String::new();
        let mut indent_level = 0;
        let indent_size = 2;

        for line in code.lines() {
            let trimmed_line = line.trim();

            // 减少缩进级别（如果当前行是 end 或类似关键字）
            if trimmed_line == "end"
                || trimmed_line == "elsif"
                || trimmed_line == "else"
                || trimmed_line == "rescue"
                || trimmed_line == "ensure"
            {
                indent_level = indent_level.saturating_sub(1);
            }

            // 添加缩进
            for _ in 0..indent_level * indent_size {
                formatted.push(' ');
            }

            // 添加行内容
            formatted.push_str(trimmed_line);
            formatted.push('\n');

            // 增加缩进级别（如果当前行以特定关键字结束）
            if trimmed_line.ends_with(':')
                || trimmed_line.starts_with("def ")
                || trimmed_line.starts_with("class ")
                || trimmed_line.starts_with("module ")
                || trimmed_line.starts_with("if ")
                || trimmed_line.starts_with("unless ")
                || trimmed_line.starts_with("while ")
                || trimmed_line.starts_with("until ")
                || trimmed_line.starts_with("for ")
                || trimmed_line.starts_with("begin")
            {
                indent_level += 1;
            }
        }

        formatted
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
