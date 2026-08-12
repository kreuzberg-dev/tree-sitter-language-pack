//! Language-neutral, per-node classification for the intelligence extractors.
//!
//! Every function here inspects a single node and returns what that node
//! contributes; none of them walks. Descent is owned by [`super::extract`],
//! which runs all enabled classifiers in one depth-bounded pass.

// ~keep `extract_intelligence` remains public even though `intel::process` drives `extract` directly.
#![allow(dead_code)]

use super::extract::Wanted;
use super::types::*;

/// Extract all intelligence from a parsed source file.
pub fn extract_intelligence(source: &str, language: &str, tree: &tree_sitter::Tree) -> ProcessResult {
    let root = tree.root_node();
    let mut result = ProcessResult {
        language: language.to_string(),
        metrics: compute_line_metrics(source),
        ..Default::default()
    };
    super::extract::extract_all(&root, source, language, Wanted::all(), &mut result);
    result
}

pub(super) fn span_from_node(node: &tree_sitter::Node) -> Span {
    let start = node.start_position();
    let end = node.end_position();
    Span {
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_line: start.row,
        start_column: start.column,
        end_line: end.row,
        end_column: end.column,
    }
}

pub(super) fn node_text<'a>(node: &tree_sitter::Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

fn go_type_spec_symbol_kind(node: &tree_sitter::Node) -> SymbolKind {
    let ty_kind = node
        .child_by_field_name("type")
        .map(|n| n.kind().to_string())
        .unwrap_or_default();
    match ty_kind.as_str() {
        "struct_type" => SymbolKind::Type,
        "interface_type" => SymbolKind::Interface,
        _ => SymbolKind::Type,
    }
}

/// Line-level metrics, which are derived from the source text alone.
///
/// The tree-derived fields (`node_count`, `error_count`, `max_depth`) are left
/// at zero here and filled in by [`super::extract::extract_all`], which is the
/// only traversal of the tree.
pub(crate) fn compute_line_metrics(source: &str) -> FileMetrics {
    let mut total_lines = 0usize;
    let mut blank_lines = 0;
    let mut comment_lines = 0;
    for line in source.lines() {
        total_lines += 1;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            blank_lines += 1;
        } else if trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with("/*")
            || trimmed.starts_with('*')
        {
            comment_lines += 1;
        }
    }
    let code_lines = total_lines.saturating_sub(blank_lines + comment_lines);

    FileMetrics {
        total_lines,
        code_lines,
        comment_lines,
        blank_lines,
        total_bytes: source.len(),
        node_count: 0,
        error_count: 0,
        max_depth: 0,
    }
}

/// Classify `node` as a comment, if it is one.
pub(super) fn comment_at(node: &tree_sitter::Node, source: &str) -> Option<CommentInfo> {
    let kind = node.kind();
    if !matches!(
        kind,
        "comment" | "line_comment" | "block_comment" | "doc_comment" | "documentation_comment"
    ) {
        return None;
    }
    let text = node_text(node, source).to_string();
    let comment_kind = if kind == "doc_comment" || kind == "documentation_comment" {
        CommentKind::Doc
    } else if kind == "block_comment" {
        CommentKind::Block
    } else if text.starts_with("///")
        || text.starts_with("//!")
        || text.starts_with("/**")
        || text.starts_with("/*!")
        || text.starts_with("##")
    {
        CommentKind::Doc
    } else {
        CommentKind::Line
    };
    Some(CommentInfo {
        text,
        kind: comment_kind,
        span: span_from_node(node),
        associated_node: node.next_named_sibling().map(|n| n.kind().to_string()),
    })
}

/// Classify `node` as a docstring, if it is one.
///
/// Only Python has a dedicated docstring form; every other language's doc
/// comments are already captured by [`comment_at`].
pub(super) fn docstring_at(node: &tree_sitter::Node, source: &str, language: &str) -> Option<DocstringInfo> {
    if language != "python" || node.kind() != "expression_statement" {
        return None;
    }
    let child = node.child(0)?;
    if child.kind() != "string" && child.kind() != "concatenated_string" {
        return None;
    }
    let parent = node.parent()?;
    if parent.kind() != "block" && parent.kind() != "module" {
        return None;
    }
    Some(DocstringInfo {
        text: node_text(&child, source).to_string(),
        format: DocstringFormat::PythonTripleQuote,
        span: span_from_node(&child),
        associated_item: parent.parent().and_then(|gp| {
            gp.child_by_field_name("name")
                .map(|n| node_text(&n, source).to_string())
        }),
        parsed_sections: Vec::new(),
    })
}

/// Classify `node` as an import for the language-neutral matcher.
///
/// Elixir directives are `call` nodes and are classified by
/// [`super::elixir::import_directive`] instead.
pub(super) fn import_at(node: &tree_sitter::Node, source: &str, language: &str) -> Option<ImportInfo> {
    let kind = node.kind();
    let is_import = match language {
        "python" => kind == "import_statement" || kind == "import_from_statement",
        "javascript" | "typescript" | "tsx" => kind == "import_statement",
        "rust" => kind == "use_declaration",
        "go" => kind == "import_declaration" || kind == "import_spec",
        "java" | "kotlin" => kind == "import_declaration",
        _ => false,
    };
    if !is_import {
        return None;
    }
    let text = node_text(node, source);
    Some(ImportInfo {
        source: text.to_string(),
        items: Vec::new(),
        alias: None,
        is_wildcard: text.contains('*'),
        span: span_from_node(node),
    })
}

/// Classify `node` as an export, if the language has export statements.
pub(super) fn export_at(node: &tree_sitter::Node, source: &str, language: &str) -> Option<ExportInfo> {
    let is_export = match language {
        "javascript" | "typescript" | "tsx" => node.kind() == "export_statement",
        _ => false,
    };
    if !is_export {
        return None;
    }
    let export_kind = if node.child_by_field_name("default").is_some() {
        ExportKind::Default
    } else if node.child_by_field_name("source").is_some() {
        ExportKind::ReExport
    } else {
        ExportKind::Named
    };
    let text = node_text(node, source);
    Some(ExportInfo {
        name: text.lines().next().unwrap_or("").to_string(),
        kind: export_kind,
        span: span_from_node(node),
    })
}

/// Classify `node` as a top-level structure item for the language-neutral
/// matcher. Elixir definitions are handled by [`super::elixir::definition`].
pub(super) fn structure_kind_at(node: &tree_sitter::Node, language: &str) -> Option<StructureKind> {
    match node.kind() {
        "function_definition" | "function_declaration" | "function_item" | "arrow_function" => {
            Some(StructureKind::Function)
        }
        "method_definition" | "method_declaration" => Some(StructureKind::Method),
        "method" | "singleton_method" if language == "ruby" => Some(StructureKind::Method),
        "class_definition" | "class_declaration" | "class" => Some(StructureKind::Class),
        "struct_item" | "struct_definition" | "struct_declaration" => Some(StructureKind::Struct),
        "interface_declaration" | "interface_definition" => Some(StructureKind::Interface),
        "enum_item" | "enum_definition" | "enum_declaration" => Some(StructureKind::Enum),
        "module_definition" | "mod_item" | "package_header" | "package_declaration" => Some(StructureKind::Module),
        "module" if language == "ruby" => Some(StructureKind::Module),
        "trait_item" => Some(StructureKind::Trait),
        "impl_item" => Some(StructureKind::Impl),
        _ => None,
    }
}

/// Classify `node` as a symbol, if it is a named declaration.
pub(super) fn symbol_at(node: &tree_sitter::Node, source: &str) -> Option<SymbolInfo> {
    let symbol_kind = match node.kind() {
        "function_definition" | "function_declaration" | "function_item" => SymbolKind::Function,
        "class_definition" | "class_declaration" => SymbolKind::Class,
        "type_alias_declaration" | "type_item" => SymbolKind::Type,
        "type_spec" => go_type_spec_symbol_kind(node),
        "interface_declaration" => SymbolKind::Interface,
        "enum_item" | "enum_declaration" => SymbolKind::Enum,
        "const_item" | "const_declaration" => SymbolKind::Constant,
        "let_declaration" | "variable_declaration" | "lexical_declaration" => SymbolKind::Variable,
        _ => return None,
    };
    let name_node = node.child_by_field_name("name")?;
    Some(SymbolInfo {
        name: node_text(&name_node, source).to_string(),
        kind: symbol_kind,
        span: span_from_node(node),
        type_annotation: node
            .child_by_field_name("type")
            .map(|n| node_text(&n, source).to_string()),
        doc: None,
    })
}

/// Classify `node` as a syntax diagnostic, if it is an error or missing node.
pub(super) fn diagnostic_at(node: &tree_sitter::Node, source: &str) -> Option<Diagnostic> {
    if node.is_error() {
        return Some(Diagnostic {
            message: format!("Syntax error: unexpected '{}'", node_text(node, source)),
            severity: DiagnosticSeverity::Error,
            span: span_from_node(node),
        });
    }
    if node.is_missing() {
        return Some(Diagnostic {
            message: format!("Missing expected node: {}", node.kind()),
            severity: DiagnosticSeverity::Error,
            span: span_from_node(node),
        });
    }
    None
}

/// Resolve the name of a structure node using a fallback chain.
///
/// Tries `"name"` field first (covers Python, Rust, Java classes), then finds
/// the first named child with kind `"type_identifier"` (Kotlin classes), then
/// `"identifier"` (Kotlin packages), then `"scoped_identifier"` (Java packages).
/// Returns `None` if no non-empty text is found via any strategy.
pub(super) fn resolve_structure_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
    if let Some(n) = node.child_by_field_name("name") {
        let text = node_text(&n, source);
        if !text.is_empty() {
            return Some(text.to_string());
        }
    }
    for target_kind in &["type_identifier", "identifier", "scoped_identifier"] {
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == *target_kind {
                let text = node_text(&child, source);
                if !text.is_empty() {
                    return Some(text.to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: parse source using the global registry (avoids Language lifetime issues).
    fn parse_with_language(source: &str, lang_name: &str) -> Option<(tree_sitter::Language, tree_sitter::Tree)> {
        let registry = crate::LanguageRegistry::new();
        let lang = registry.get_language(lang_name).ok()?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang).ok()?;
        let tree = parser.parse(source, None)?;
        Some((lang, tree))
    }

    fn parse_or_skip(source: &str, lang_name: &str) -> Option<tree_sitter::Tree> {
        parse_with_language(source, lang_name).map(|(_, tree)| tree)
    }

    #[test]
    fn test_extract_python_function() {
        let source = "def foo():\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.language, "python");
        assert!(!intel.structure.is_empty(), "should find at least one structure item");
        let func = &intel.structure[0];
        assert_eq!(func.kind, StructureKind::Function);
        assert_eq!(func.name.as_deref(), Some("foo"));
    }

    #[test]
    fn test_extract_python_class() {
        let source = "class MyClass:\n    def method(self):\n        pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a class");
        let class = class.unwrap();
        assert_eq!(class.name.as_deref(), Some("MyClass"));
        assert!(!class.children.is_empty(), "class should have child methods");
        assert_eq!(class.children[0].kind, StructureKind::Function);
        assert_eq!(class.children[0].name.as_deref(), Some("method"));
    }

    #[test]
    fn test_extract_ruby_module_class_and_methods() {
        let source = "module Outer\n  class Widget\n    def call\n      true\n    end\n\n    def self.build\n      new\n    end\n  end\nend\n";
        let Some(tree) = parse_or_skip(source, "ruby") else {
            return;
        };
        let intel = extract_intelligence(source, "ruby", &tree);

        let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
        assert!(module.is_some(), "should find a Ruby module entry");
        let module = module.unwrap();
        assert_eq!(module.name.as_deref(), Some("Outer"));

        let class = module.children.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a Ruby class inside the module");
        let class = class.unwrap();
        assert_eq!(class.name.as_deref(), Some("Widget"));

        let method_names = class
            .children
            .iter()
            .filter(|s| s.kind == StructureKind::Method)
            .filter_map(|s| s.name.as_deref())
            .collect::<Vec<_>>();
        assert!(method_names.contains(&"call"), "should find an instance method");
        assert!(method_names.contains(&"build"), "should find a singleton method");
    }

    #[test]
    fn test_extract_rust_function() {
        let source = "fn main() {\n    let x = 5;\n}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert!(!intel.structure.is_empty(), "should find at least one structure item");
        let func = &intel.structure[0];
        assert_eq!(func.kind, StructureKind::Function);
        assert_eq!(func.name.as_deref(), Some("main"));
    }

    #[test]
    fn test_extract_python_imports() {
        let source = "import os\nfrom sys import path\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.imports.len(), 2, "should find 2 imports");
        assert!(intel.imports[0].source.contains("import os"));
        assert!(intel.imports[1].source.contains("from sys import path"));
    }

    #[test]
    fn test_extract_rust_imports() {
        let source = "use std::collections::HashMap;\nuse std::io;\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.imports.len(), 2, "should find 2 use declarations");
    }

    #[test]
    fn test_extract_comments() {
        let source = "// This is a comment\nfn main() {}\n// Another comment\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert!(intel.comments.len() >= 2, "should find at least 2 comments");
        assert!(intel.comments[0].text.contains("This is a comment"));
    }

    #[test]
    fn test_extract_doc_comments() {
        let source = "/// Documentation comment\nfn documented() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        let doc_comments: Vec<_> = intel.comments.iter().filter(|c| c.kind == CommentKind::Doc).collect();
        assert!(!doc_comments.is_empty(), "should find doc comments");
    }

    #[test]
    fn test_metrics_counts() {
        let source = "fn foo() {}\n\n// comment\nfn bar() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert!(intel.metrics.total_lines >= 4, "should have at least 4 lines");
        assert!(intel.metrics.blank_lines >= 1, "should have at least 1 blank line");
        assert!(intel.metrics.comment_lines >= 1, "should have at least 1 comment line");
        assert!(intel.metrics.code_lines >= 2, "should have at least 2 code lines");
        assert!(intel.metrics.node_count > 0, "should have nodes");
        assert_eq!(intel.metrics.error_count, 0, "valid code should have 0 errors");
        assert!(intel.metrics.max_depth > 0, "tree should have depth > 0");
        assert_eq!(intel.metrics.total_bytes, source.len());
    }

    #[test]
    fn test_extract_symbols() {
        let source = "fn alpha() {}\nfn beta() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        let func_symbols: Vec<_> = intel
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        assert!(func_symbols.len() >= 2, "should find at least 2 function symbols");
        let names: Vec<_> = func_symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }

    #[test]
    fn test_extract_go_type_declarations_as_symbols() {
        let source = "type User struct{}\ntype Service interface{}\ntype ID string\n";
        let Some(tree) = parse_or_skip(source, "go") else {
            return;
        };
        let intel = extract_intelligence(source, "go", &tree);

        assert!(
            intel
                .symbols
                .iter()
                .any(|s| { s.kind == SymbolKind::Type && s.name == "User" })
        );
        assert!(
            intel
                .symbols
                .iter()
                .any(|s| { s.kind == SymbolKind::Interface && s.name == "Service" })
        );
        assert!(
            intel
                .symbols
                .iter()
                .any(|s| { s.kind == SymbolKind::Type && s.name == "ID" })
        );
    }

    #[test]
    fn test_error_nodes_detected() {
        let source = "def :\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert!(
            intel.metrics.error_count > 0,
            "invalid syntax should produce error nodes"
        );
        assert!(!intel.diagnostics.is_empty(), "should have diagnostics for errors");
        assert!(
            intel
                .diagnostics
                .iter()
                .any(|d| d.severity == DiagnosticSeverity::Error)
        );
    }

    #[test]
    fn test_valid_code_no_diagnostics() {
        let source = "def foo():\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.metrics.error_count, 0);
        assert!(intel.diagnostics.is_empty(), "valid code should have no diagnostics");
    }

    #[test]
    #[ignore = "Python grammar node types vary across versions; needs grammar-aware matching"]
    fn test_extract_python_docstrings() {
        let source = "def greet():\n    \"\"\"Say hello.\"\"\"\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert!(!intel.docstrings.is_empty(), "should find python docstring");
        assert_eq!(intel.docstrings[0].format, DocstringFormat::PythonTripleQuote);
    }

    #[test]
    fn test_intelligence_language_field() {
        let source = "x = 1";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);
        assert_eq!(intel.language, "python");
    }

    #[test]
    fn collect_structure_kotlin_package_and_class() {
        let source = "package foo.bar\n\nclass Widget {}\n";
        let Some(tree) = parse_or_skip(source, "kotlin") else {
            return;
        };
        let intel = extract_intelligence(source, "kotlin", &tree);

        let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
        assert!(module.is_some(), "should find a Module entry for the package header");
        assert_eq!(module.unwrap().name.as_deref(), Some("foo.bar"));

        let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a Class entry");
        assert_eq!(class.unwrap().name.as_deref(), Some("Widget"));
    }

    #[test]
    fn collect_structure_java_package_and_class() {
        let source = "package com.example;\n\npublic class Widget {}\n";
        let Some(tree) = parse_or_skip(source, "java") else {
            return;
        };
        let intel = extract_intelligence(source, "java", &tree);

        let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
        assert!(
            module.is_some(),
            "should find a Module entry for the package declaration"
        );
        assert_eq!(module.unwrap().name.as_deref(), Some("com.example"));

        let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a Class entry");
        assert_eq!(class.unwrap().name.as_deref(), Some("Widget"));
    }
}
