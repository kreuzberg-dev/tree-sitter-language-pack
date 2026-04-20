use super::types::*;

/// Extract all intelligence from a parsed source file.
pub fn extract_intelligence(source: &str, language: &str, tree: &tree_sitter::Tree) -> ProcessResult {
    let root = tree.root_node();
    let comments = extract_comments(&root, source, language);
    let mut structure = extract_structure(&root, source, language);
    let mut symbols = extract_symbols(&root, source, language);
    attach_doc_comments_to_structure(&mut structure, &comments);
    attach_doc_comments_to_symbols(&mut symbols, &comments);
    ProcessResult {
        language: language.to_string(),
        metrics: compute_metrics(source, &root),
        structure,
        imports: extract_imports(&root, source, language),
        exports: extract_exports(&root, source, language),
        comments,
        docstrings: extract_docstrings(&root, source, language),
        symbols,
        diagnostics: extract_diagnostics(&root, source),
        chunks: Vec::new(),
        extractions: ahash::AHashMap::new(),
    }
}

fn span_from_node(node: &tree_sitter::Node) -> Span {
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

fn node_text<'a>(node: &tree_sitter::Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

fn normalized_doc_comment_text(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.starts_with("/**") || trimmed.starts_with("/*!") {
        let mut lines = Vec::new();
        for line in trimmed.lines() {
            let mut piece = line.trim().to_string();
            if let Some(rest) = piece.strip_prefix("/**") {
                piece = rest.trim().to_string();
            } else if let Some(rest) = piece.strip_prefix("/*!") {
                piece = rest.trim().to_string();
            }
            if let Some(rest) = piece.strip_suffix("*/") {
                piece = rest.trim().to_string();
            }
            if let Some(rest) = piece.strip_prefix('*') {
                piece = rest.trim_start().to_string();
            }
            if !piece.is_empty() {
                lines.push(piece);
            }
        }
        return lines.join("\n").trim().to_string();
    }
    let mut lines = Vec::new();
    for line in trimmed.lines() {
        let line = line.trim();
        let piece = line
            .strip_prefix("///")
            .or_else(|| line.strip_prefix("//!"))
            .or_else(|| line.strip_prefix("##"))
            .unwrap_or(line)
            .trim_start()
            .to_string();
        if !piece.is_empty() {
            lines.push(piece);
        }
    }
    lines.join("\n").trim().to_string()
}

fn nearest_doc_comment(
    span: &Span,
    comments: &[CommentInfo],
) -> Option<String> {
    comments
        .iter()
        .filter(|comment| comment.kind == CommentKind::Doc)
        .filter(|comment| comment.span.end_byte <= span.start_byte)
        .filter(|comment| span.start_line <= comment.span.end_line + 2)
        .min_by_key(|comment| span.start_byte - comment.span.end_byte)
        .map(|comment| normalized_doc_comment_text(&comment.text))
        .filter(|text| !text.is_empty())
}

fn attach_doc_comments_to_structure(items: &mut [StructureItem], comments: &[CommentInfo]) {
    for item in items {
        if item.doc_comment.is_none() {
            item.doc_comment = nearest_doc_comment(&item.span, comments);
        }
        if !item.children.is_empty() {
            attach_doc_comments_to_structure(&mut item.children, comments);
        }
    }
}

fn attach_doc_comments_to_symbols(symbols: &mut [SymbolInfo], comments: &[CommentInfo]) {
    for symbol in symbols {
        if symbol.doc.is_none() {
            symbol.doc = nearest_doc_comment(&symbol.span, comments);
        }
    }
}

fn strip_quoted_text(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.chars().next().unwrap_or('\0');
        let last = trimmed.chars().last().unwrap_or('\0');
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    trimmed.to_string()
}

fn swift_container_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
    let mut current = node.parent();
    for _ in 0..32 {
        let Some(parent) = current else {
            break;
        };
        match parent.kind() {
            "class_declaration"
            | "struct_declaration"
            | "enum_declaration"
            | "extension_declaration"
            | "protocol_declaration" => {
                if let Some(name) = parent
                    .child_by_field_name("name")
                    .map(|n| node_text(&n, source).trim().to_string())
                    .filter(|name| !name.is_empty())
                {
                    return Some(name);
                }
            }
            _ => {}
        }
        current = parent.parent();
    }
    None
}

fn kotlin_identifier_node<'a>(node: &'a tree_sitter::Node<'a>) -> Option<tree_sitter::Node<'a>> {
    let preferred: &[&str] = match node.kind() {
        "class_declaration" | "interface_declaration" | "enum_declaration" | "object_declaration" | "type_alias" => {
            &["type_identifier", "simple_identifier", "identifier"]
        }
        "function_declaration" | "property_declaration" => &["simple_identifier", "identifier"],
        _ => &["simple_identifier", "type_identifier", "identifier"],
    };
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).filter(|child| child.is_named()).collect();
    for kind in preferred {
        if let Some(found) = children.iter().find(|child| child.kind() == *kind) {
            return Some(*found);
        }
    }
    None
}

fn declaration_name_node<'a>(
    node: &'a tree_sitter::Node<'a>,
    language: &str,
) -> Option<tree_sitter::Node<'a>> {
    if let Some(name_node) = node.child_by_field_name("name") {
        return Some(name_node);
    }
    if language == "kotlin" {
        return kotlin_identifier_node(node);
    }
    None
}

fn kotlin_container_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
    let mut current = node.parent();
    for _ in 0..32 {
        let Some(parent) = current else {
            break;
        };
        match parent.kind() {
            "class_declaration" | "interface_declaration" | "enum_declaration" | "object_declaration" => {
                if let Some(name) = declaration_name_node(&parent, "kotlin")
                    .map(|n| node_text(&n, source).trim().to_string())
                    .filter(|name| !name.is_empty())
                {
                    return Some(name);
                }
            }
            _ => {}
        }
        current = parent.parent();
    }
    None
}

fn kotlin_qualified_name_for(node: &tree_sitter::Node, source: &str, name: &str) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = node.parent();
    for _ in 0..32 {
        let Some(parent) = current else {
            break;
        };
        match parent.kind() {
            "class_declaration" | "interface_declaration" | "enum_declaration" | "object_declaration" => {
                if let Some(pname) = declaration_name_node(&parent, "kotlin")
                    .map(|n| node_text(&n, source).trim().to_string())
                    .filter(|name| !name.is_empty())
                {
                    parts.push(pname);
                }
            }
            _ => {}
        }
        current = parent.parent();
    }
    if parts.is_empty() {
        Some(name.to_string())
    } else {
        parts.reverse();
        parts.push(name.to_string());
        Some(parts.join("."))
    }
}

fn kotlin_is_method(node: &tree_sitter::Node) -> bool {
    let mut current = node.parent();
    for _ in 0..32 {
        let Some(parent) = current else {
            break;
        };
        match parent.kind() {
            "class_declaration" | "interface_declaration" | "object_declaration" | "enum_declaration" => return true,
            "source_file" => return false,
            _ => current = parent.parent(),
        }
    }
    false
}

fn swift_inherited_types(node: &tree_sitter::Node, source: &str) -> Vec<String> {
    let mut inherited = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() != "inheritance_specifier" {
            continue;
        }
        if let Some(target) = child.child_by_field_name("inherits_from") {
            let name = node_text(&target, source).trim().to_string();
            if !name.is_empty() && seen.insert(name.clone()) {
                inherited.push(name);
            }
        }
    }
    inherited
}

fn go_type_spec_kind(node: &tree_sitter::Node) -> StructureKind {
    let ty_kind = node
        .child_by_field_name("type")
        .map(|n| n.kind().to_string())
        .unwrap_or_default();
    match ty_kind.as_str() {
        "struct_type" => StructureKind::Struct,
        "interface_type" => StructureKind::Interface,
        _ => StructureKind::TypeAlias,
    }
}

fn go_type_spec_symbol_kind(node: &tree_sitter::Node) -> SymbolKind {
    match go_type_spec_kind(node) {
        StructureKind::Struct => SymbolKind::Type,
        StructureKind::Interface => SymbolKind::Interface,
        StructureKind::TypeAlias => SymbolKind::TypeAlias,
        _ => SymbolKind::Type,
    }
}

pub(crate) fn compute_metrics(source: &str, root: &tree_sitter::Node) -> FileMetrics {
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
    total_lines = total_lines.max(1);
    let code_lines = total_lines.saturating_sub(blank_lines + comment_lines);
    let mut node_count = 0;
    let mut error_count = 0;
    let mut max_depth = 0;
    count_nodes(root, 0, &mut node_count, &mut error_count, &mut max_depth);

    FileMetrics {
        total_lines,
        code_lines,
        comment_lines,
        blank_lines,
        total_bytes: source.len(),
        node_count,
        error_count,
        max_depth,
    }
}

fn count_nodes(node: &tree_sitter::Node, depth: usize, count: &mut usize, errors: &mut usize, max_depth: &mut usize) {
    *count += 1;
    if depth > *max_depth {
        *max_depth = depth;
    }
    if node.is_error() || node.is_missing() {
        *errors += 1;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        count_nodes(&child, depth + 1, count, errors, max_depth);
    }
}

pub(crate) fn extract_comments(root: &tree_sitter::Node, source: &str, _language: &str) -> Vec<CommentInfo> {
    let mut comments = Vec::with_capacity(16);
    collect_comments(root, source, &mut comments);
    comments
}

fn collect_comments(node: &tree_sitter::Node, source: &str, comments: &mut Vec<CommentInfo>) {
    let kind = node.kind();
    if kind == "comment"
        || kind == "line_comment"
        || kind == "block_comment"
        || kind == "doc_comment"
        || kind == "documentation_comment"
    {
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
        comments.push(CommentInfo {
            text,
            kind: comment_kind,
            span: span_from_node(node),
            associated_node: node.next_named_sibling().map(|n| n.kind().to_string()),
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_comments(&child, source, comments);
    }
}

pub(crate) fn extract_docstrings(root: &tree_sitter::Node, source: &str, language: &str) -> Vec<DocstringInfo> {
    let mut docstrings = Vec::with_capacity(16);
    collect_docstrings(root, source, language, &mut docstrings);
    docstrings
}

fn collect_docstrings(node: &tree_sitter::Node, source: &str, language: &str, docstrings: &mut Vec<DocstringInfo>) {
    match language {
        "python" => {
            if node.kind() == "expression_statement"
                && let Some(child) = node.child(0)
                && (child.kind() == "string" || child.kind() == "concatenated_string")
                && let Some(parent) = node.parent()
            {
                let parent_kind = parent.kind();
                if parent_kind == "block" || parent_kind == "module" {
                    let text = node_text(&child, source).to_string();
                    docstrings.push(DocstringInfo {
                        text,
                        format: DocstringFormat::PythonTripleQuote,
                        span: span_from_node(&child),
                        associated_item: parent.parent().and_then(|gp| {
                            gp.child_by_field_name("name")
                                .map(|n| node_text(&n, source).to_string())
                        }),
                        parsed_sections: Vec::new(),
                    });
                }
            }
        }
        _ => {
            // For other languages, doc comments are already captured in extract_comments
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_docstrings(&child, source, language, docstrings);
    }
}

pub(crate) fn extract_imports(root: &tree_sitter::Node, source: &str, language: &str) -> Vec<ImportInfo> {
    let mut imports = Vec::with_capacity(16);
    collect_imports(root, source, language, &mut imports);
    imports
}

pub(crate) fn extract_index_intelligence(
    root: &tree_sitter::Node,
    source: &str,
    language: &str,
) -> (Vec<StructureItem>, Vec<ImportInfo>, Vec<ExportInfo>, Vec<SymbolInfo>) {
    let mut structure = Vec::with_capacity(32);
    let mut imports = Vec::with_capacity(16);
    let mut exports = Vec::with_capacity(16);
    let mut symbols = Vec::new();
    collect_index_intelligence(
        root,
        source,
        language,
        &mut structure,
        &mut imports,
        &mut exports,
        &mut symbols,
    );
    if language == "python" {
        let mut seen = std::collections::HashSet::new();
        exports.retain(|item| seen.insert(item.name.clone()));
    }
    (structure, imports, exports, symbols)
}

fn collect_index_intelligence(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    structure: &mut Vec<StructureItem>,
    imports: &mut Vec<ImportInfo>,
    exports: &mut Vec<ExportInfo>,
    symbols: &mut Vec<SymbolInfo>,
) {
    if node_is_structure_root(node, source, language) {
        collect_structure(node, source, language, structure);
        collect_index_semantics(node, source, language, imports, exports, symbols);
        return;
    }

    let recurse = collect_import_on_node(node, source, language, imports)
        & collect_export_on_node(node, source, language, exports)
        & collect_symbol_on_node(node, source, language, symbols);

    if recurse {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_index_intelligence(&child, source, language, structure, imports, exports, symbols);
        }
    }
}

fn node_is_structure_root(node: &tree_sitter::Node, source: &str, language: &str) -> bool {
    let kind = node.kind();
    if extract_doc_heading(node, source, language).is_some() {
        return true;
    }
    if language == "swift" && (kind.contains("enum_case") || kind == "enum_entry") {
        return true;
    }
    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "variable_declarator" {
        if let Some(value_node) = node.child_by_field_name("value") {
            return matches!(
                value_node.kind(),
                "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
            );
        }
    }
    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "assignment_expression" {
        if let Some(right_node) = node.child_by_field_name("right") {
            return matches!(
                right_node.kind(),
                "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
            );
        }
    }
    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "pair" {
        if let Some(value_node) = node.child_by_field_name("value") {
            return matches!(
                value_node.kind(),
                "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
            );
        }
    }

    matches!(
        kind,
        "function_definition"
            | "function_declaration"
            | "function_item"
            | "arrow_function"
            | "method_definition"
            | "method_declaration"
            | "class_definition"
            | "class_declaration"
            | "class"
            | "struct_item"
            | "struct_definition"
            | "struct_declaration"
            | "interface_declaration"
            | "interface_definition"
            | "protocol_declaration"
            | "enum_item"
            | "enum_definition"
            | "enum_declaration"
            | "typealias_declaration"
            | "associatedtype_declaration"
            | "module_definition"
            | "mod_item"
            | "trait_item"
            | "impl_item"
    ) || (language == "go" && kind == "type_spec")
}

fn collect_index_semantics(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    imports: &mut Vec<ImportInfo>,
    exports: &mut Vec<ExportInfo>,
    symbols: &mut Vec<SymbolInfo>,
) {
    let recurse = collect_import_on_node(node, source, language, imports)
        & collect_export_on_node(node, source, language, exports)
        & collect_symbol_on_node(node, source, language, symbols);

    if recurse {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_index_semantics(&child, source, language, imports, exports, symbols);
        }
    }
}

fn strip_quoted_import_path(raw: &str) -> String {
    let raw = raw.trim();
    if raw.len() < 2 {
        return raw.to_string();
    }
    let first = raw.chars().next().unwrap_or('\0');
    let last = raw.chars().last().unwrap_or('\0');
    if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
        return raw[1..raw.len() - 1].to_string();
    }
    raw.to_string()
}

fn parse_go_import_spec(node: &tree_sitter::Node, source: &str) -> Option<ImportInfo> {
    if node.kind() != "import_spec" {
        return None;
    }
    let text = node_text(node, source);
    let mut parts = text.split_whitespace();
    let first = parts.next()?;
    let second = parts.next();
    let (alias, source_name) = match second {
        Some(path) => {
            let alias = match first.trim() {
                "." | "_" => None,
                value if !value.is_empty() => Some(value.to_string()),
                _ => None,
            };
            (alias, strip_quoted_import_path(path))
        }
        None => (None, strip_quoted_import_path(first)),
    };
    if source_name.is_empty() {
        return None;
    }
    Some(ImportInfo {
        source: source_name,
        items: Vec::new(),
        alias,
        is_wildcard: false,
        span: span_from_node(node),
    })
}

fn collect_import_on_node(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    imports: &mut Vec<ImportInfo>,
) -> bool {
    let kind = node.kind();
    let is_import = match language {
        "python" => kind == "import_statement" || kind == "import_from_statement",
        "javascript" | "typescript" | "tsx" => kind == "import_statement",
        "rust" => kind == "use_declaration",
        "go" => kind == "import_declaration" || kind == "import_spec",
        "java" | "kotlin" => kind == "import_declaration",
        "swift" => kind == "import_declaration",
        _ => false,
    };
    let mut handled = false;

    if language == "python" && is_import {
        if kind == "import_statement" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() != "aliased_import" && child.kind() != "dotted_name" {
                    continue;
                }
                let raw = node_text(&child, source);
                let raw_trimmed = raw.trim();
                let (source_name, alias) = if let Some((left, right)) = raw_trimmed.split_once(" as ") {
                    (left.trim().to_string(), Some(right.trim().to_string()))
                } else {
                    (raw_trimmed.to_string(), None)
                };
                if source_name.is_empty() {
                    continue;
                }
                imports.push(ImportInfo {
                    source: source_name,
                    items: Vec::new(),
                    alias,
                    is_wildcard: false,
                    span: span_from_node(node),
                });
            }
            handled = true;
        } else if kind == "import_from_statement" {
            let module_node = node.child_by_field_name("module_name");
            let module_text = module_node
                .as_ref()
                .map(|n| node_text(n, source))
                .unwrap_or("")
                .trim()
                .to_string();

            let mut items: Vec<String> = Vec::new();
            let mut is_wildcard = false;
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if let Some(module_node) = module_node.as_ref() {
                    if child.start_byte() == module_node.start_byte() && child.end_byte() == module_node.end_byte() {
                        continue;
                    }
                }
                match child.kind() {
                    "wildcard_import" => is_wildcard = true,
                    "aliased_import" => {
                        let name_node = child.child_by_field_name("name");
                        let raw = if let Some(name_node) = name_node {
                            node_text(&name_node, source)
                        } else {
                            node_text(&child, source)
                        };
                        let name = raw.split(" as ").next().unwrap_or("").trim();
                        let name = name.rsplit('.').next().unwrap_or("").trim();
                        if !name.is_empty() {
                            items.push(name.to_string());
                        }
                    }
                    "dotted_name" => {
                        let raw = node_text(&child, source);
                        let name = raw.rsplit('.').next().unwrap_or("").trim();
                        if !name.is_empty() {
                            items.push(name.to_string());
                        }
                    }
                    _ => {}
                }
            }

            if !module_text.is_empty() {
                imports.push(ImportInfo {
                    source: module_text,
                    items,
                    alias: None,
                    is_wildcard,
                    span: span_from_node(node),
                });
            }
            handled = true;
        }
    }

    if language == "go" && kind == "import_spec" && !handled {
        if let Some(import_info) = parse_go_import_spec(node, source) {
            imports.push(import_info);
            handled = true;
        }
    }

    if language == "swift" && is_import && !handled {
        let text = node_text(node, source);
        let cleaned = text.trim().trim_start_matches("import").trim().to_string();
        if cleaned.is_empty() {
            return false;
        }
        let mut parts = cleaned.split_whitespace();
        let first = parts.next().unwrap_or("");
        let module_token = match first {
            "class" | "struct" | "enum" | "protocol" | "typealias" | "func" | "var" | "let" => {
                parts.next().unwrap_or("")
            }
            _ => first,
        };
        if !module_token.is_empty() {
            let module = module_token.split('.').next().unwrap_or("").trim();
            if !module.is_empty() {
                imports.push(ImportInfo {
                    source: module.to_string(),
                    items: Vec::new(),
                    alias: None,
                    is_wildcard: false,
                    span: span_from_node(node),
                });
                handled = true;
            }
        }
    }

    if language == "rust" && is_import && !handled {
        let text = node_text(node, source);
        let mut cleaned = text
            .trim()
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim()
            .to_string();

        if let Some((before, _)) = cleaned.split_once(" as ") {
            cleaned = before.trim().to_string();
        }

        if let Some((head, rest)) = cleaned.split_once('{') {
            let module_base = head.trim().trim_end_matches("::").to_string();
            let items_part = rest.split_once('}').map(|(a, _)| a).unwrap_or("");
            for raw in items_part.split(',') {
                let item_raw = raw.trim();
                if item_raw.is_empty() || item_raw == "self" {
                    continue;
                }
                if item_raw == "*" {
                    imports.push(ImportInfo {
                        source: module_base.clone(),
                        items: Vec::new(),
                        alias: None,
                        is_wildcard: true,
                        span: span_from_node(node),
                    });
                    continue;
                }
                let item_clean = item_raw.split_once(" as ").map(|(a, _)| a).unwrap_or(item_raw);
                let item_clean = item_clean.trim();
                let (module, item) = if let Some((prefix, name)) = item_clean.rsplit_once("::") {
                    (format!("{}::{}", module_base, prefix.trim_matches(':')), name.trim())
                } else {
                    (module_base.clone(), item_clean)
                };
                if !item.is_empty() {
                    imports.push(ImportInfo {
                        source: module,
                        items: vec![item.to_string()],
                        alias: None,
                        is_wildcard: false,
                        span: span_from_node(node),
                    });
                }
            }
            handled = true;
        } else {
            let mut is_wildcard = false;
            let item_part = if cleaned.ends_with("::*") {
                is_wildcard = true;
                cleaned.trim_end_matches("::*").trim().to_string()
            } else {
                cleaned.clone()
            };
            let parts: Vec<&str> = item_part.split("::").filter(|p| !p.is_empty()).collect();
            let (module, item) = if parts.len() >= 2 {
                let last = parts[parts.len() - 1];
                let module_path = parts[..parts.len() - 1].join("::");
                if last.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    (module_path, last.to_string())
                } else {
                    (parts.join("::"), String::new())
                }
            } else {
                (item_part.trim().to_string(), String::new())
            };
            imports.push(ImportInfo {
                source: module,
                items: if item.is_empty() || is_wildcard {
                    Vec::new()
                } else {
                    vec![item]
                },
                alias: None,
                is_wildcard,
                span: span_from_node(node),
            });
            handled = true;
        }
    }

    if is_import && !handled {
        let text = node_text(node, source);
        let mut source_name = text.to_string();
        let mut is_wildcard = text.contains('*');
        let mut items: Vec<String> = Vec::new();

        let strip_quotes = |raw: &str| {
            let raw = raw.trim();
            if raw.len() < 2 {
                return raw.to_string();
            }
            let first = raw.chars().next().unwrap_or('\0');
            let last = raw.chars().last().unwrap_or('\0');
            if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
                return raw[1..raw.len() - 1].to_string();
            }
            raw.to_string()
        };

        if language == "swift" {
            let mut module = None;
            let text_line = text.lines().next().unwrap_or("").trim();
            if let Some(rest) = text_line
                .strip_prefix("import ")
                .or_else(|| text_line.strip_prefix("@testable import "))
                .or_else(|| text_line.strip_prefix("@_exported import "))
            {
                let rest = rest.trim();
                let rest = if let Some(stripped) = rest.strip_prefix("typealias ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("struct ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("class ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("enum ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("protocol ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("func ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("var ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("let ") {
                    stripped
                } else {
                    rest
                };
                let mut ident = rest
                    .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.'))
                    .next()
                    .unwrap_or("")
                    .to_string();
                if let Some((first, _)) = ident.split_once('.') {
                    ident = first.to_string();
                }
                if !ident.is_empty() {
                    module = Some(ident);
                }
            }
            if let Some(mod_name) = module {
                source_name = mod_name;
            }
            is_wildcard = false;
        }

        if matches!(language, "javascript" | "typescript" | "tsx") {
            if let Some(source_node) = node.child_by_field_name("source") {
                source_name = strip_quotes(&node_text(&source_node, source));
            } else {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "string" {
                        source_name = strip_quotes(&node_text(&child, source));
                        break;
                    }
                }
            }

            let clause = node.child_by_field_name("import_clause").or_else(|| {
                let mut cursor = node.walk();
                node.children(&mut cursor).find(|c| c.kind() == "import_clause")
            });
            if let Some(clause) = clause {
                let mut cursor = clause.walk();
                for child in clause.children(&mut cursor) {
                    match child.kind() {
                        "identifier" => items.push(node_text(&child, source).to_string()),
                        "namespace_import" => is_wildcard = true,
                        "named_imports" => {
                            let mut c2 = child.walk();
                            for spec in child.children(&mut c2) {
                                if spec.kind() != "import_specifier" {
                                    continue;
                                }
                                if let Some(name_node) = spec.child_by_field_name("name") {
                                    items.push(node_text(&name_node, source).to_string());
                                    continue;
                                }
                                let mut c3 = spec.walk();
                                for n in spec.children(&mut c3) {
                                    if n.kind() == "identifier" || n.kind() == "property_identifier" {
                                        items.push(node_text(&n, source).to_string());
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        imports.push(ImportInfo {
            source: source_name,
            items,
            alias: None,
            is_wildcard,
            span: span_from_node(node),
        });
    }

    true
}

fn collect_export_on_node(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    exports: &mut Vec<ExportInfo>,
) -> bool {
    let kind = node.kind();
    let is_export = match language {
        "javascript" | "typescript" | "tsx" => kind == "export_statement",
        "python" => kind == "assignment" || kind == "expression_statement",
        _ => false,
    };
    if !is_export {
        return true;
    }
    let text = node_text(node, source);
    if language == "python" {
        if text.contains("__all__") {
            exports.push(ExportInfo {
                name: text.lines().next().unwrap_or("").to_string(),
                kind: ExportKind::Named,
                source: None,
                exported_as: None,
                span: span_from_node(node),
            });
        }
        return true;
    }
    if matches!(language, "javascript" | "typescript" | "tsx") {
        let export_kind = if node.child_by_field_name("default").is_some() {
            ExportKind::Default
        } else if node.child_by_field_name("source").is_some() {
            ExportKind::ReExport
        } else {
            ExportKind::Named
        };
        let source_module = node
            .child_by_field_name("source")
            .map(|n| strip_quoted_text(node_text(&n, source)));

        let mut names: Vec<String> = Vec::new();
        if let Some(decl) = node.child_by_field_name("declaration") {
            if let Some(name_node) = decl.child_by_field_name("name") {
                names.push(node_text(&name_node, source).to_string());
            } else if decl.kind() == "variable_declaration" {
                let mut cursor = decl.walk();
                for child in decl.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        if let Some(name_node) = child.child_by_field_name("name") {
                            let name = node_text(&name_node, source).to_string();
                            if !name.is_empty() {
                                names.push(name);
                            }
                        }
                    }
                }
            }
        }
        if names.is_empty() {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "identifier" | "property_identifier" => {
                        let name = node_text(&child, source).to_string();
                        if !name.is_empty() {
                            names.push(name);
                        }
                    }
                    "export_clause" => {
                        let mut c2 = child.walk();
                        for spec in child.children(&mut c2) {
                            if spec.kind() != "export_specifier" {
                                continue;
                            }
                            let alias = spec
                                .child_by_field_name("alias")
                                .map(|n| node_text(&n, source).to_string());
                            let name_node = spec.child_by_field_name("name").or_else(|| {
                                let mut c3 = spec.walk();
                                spec.children(&mut c3)
                                    .find(|n| matches!(n.kind(), "identifier" | "property_identifier"))
                            });
                            if let Some(name_node) = name_node {
                                let name = node_text(&name_node, source).to_string();
                                if !name.is_empty() {
                                    exports.push(ExportInfo {
                                        name,
                                        kind: export_kind.clone(),
                                        source: source_module.clone(),
                                        exported_as: alias,
                                        span: span_from_node(&spec),
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if names.is_empty() && matches!(export_kind, ExportKind::ReExport) && text.contains("export *") {
            exports.push(ExportInfo {
                name: "*".to_string(),
                kind: export_kind,
                source: source_module,
                exported_as: parse_namespace_export_alias(text),
                span: span_from_node(node),
            });
        } else {
            for name in names {
                if !name.is_empty() {
                    exports.push(ExportInfo {
                        name,
                        kind: export_kind.clone(),
                        source: source_module.clone(),
                        exported_as: None,
                        span: span_from_node(node),
                    });
                }
            }
        }
    }
    true
}

fn collect_symbol_on_node(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    symbols: &mut Vec<SymbolInfo>,
) -> bool {
    let kind = node.kind();

    fn swift_enum_case_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
        if let Some(name_node) = node.child_by_field_name("name") {
            return Some(node_text(&name_node, source).to_string());
        }
        fn find_identifier(node: &tree_sitter::Node, source: &str) -> Option<String> {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if !child.is_named() {
                    continue;
                }
                let kind = child.kind();
                if kind.contains("identifier") {
                    return Some(node_text(&child, source).to_string());
                }
                if let Some(found) = find_identifier(&child, source) {
                    return Some(found);
                }
            }
            None
        }

        find_identifier(node, source)
    }

    fn swift_enum_case_nodes<'a>(node: &'a tree_sitter::Node<'a>) -> Vec<tree_sitter::Node<'a>> {
        let kind = node.kind();
        if kind == "enum_entry" {
            return vec![*node];
        }
        if !kind.contains("enum_case") {
            return Vec::new();
        }
        let mut nodes = Vec::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if !child.is_named() {
                continue;
            }
            if child.kind().contains("enum_case_element") {
                nodes.push(child);
            }
        }
        if nodes.is_empty() {
            nodes.push(*node);
        }
        nodes
    }

    if language == "swift" && (kind.contains("enum_case") || kind == "enum_entry") {
        let mut added = false;
        for case_node in swift_enum_case_nodes(node) {
            let Some(case_name) = swift_enum_case_name(&case_node, source) else {
                continue;
            };
            symbols.push(SymbolInfo {
                name: case_name,
                kind: SymbolKind::EnumCase,
                span: span_from_node(&case_node),
                type_annotation: None,
                container_name: swift_container_name(&case_node, source),
                extended_type: None,
                inherited_types: Vec::new(),
                doc: None,
            });
            added = true;
        }
        return !added;
    }

    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "variable_declarator" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = node_text(&name_node, source).to_string();
            let mut sk = SymbolKind::Variable;
            if let Some(value_node) = node.child_by_field_name("value") {
                let vkind = value_node.kind();
                if matches!(
                    vkind,
                    "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
                ) {
                    sk = SymbolKind::Function;
                }
            }
            if !name.is_empty() {
                symbols.push(SymbolInfo {
                    name,
                    kind: sk,
                    span: span_from_node(node),
                    type_annotation: node
                        .child_by_field_name("type")
                        .map(|n| node_text(&n, source).to_string()),
                    container_name: None,
                    extended_type: None,
                    inherited_types: Vec::new(),
                    doc: None,
                });
                return false;
            }
        }
    }

    let symbol_kind = match kind {
        "function_definition" | "function_declaration" | "function_item" | "method_declaration" => {
            Some(SymbolKind::Function)
        }
        "class_definition" | "class_declaration" => {
            if language == "swift" && kind == "class_declaration" {
                match swift_classlike_kind(node, source) {
                    Some(StructureKind::Enum) => Some(SymbolKind::Enum),
                    Some(StructureKind::Struct) => Some(SymbolKind::Type),
                    Some(StructureKind::Extension) => Some(SymbolKind::Extension),
                    _ => Some(SymbolKind::Class),
                }
            } else {
                Some(SymbolKind::Class)
            }
        }
        "type_alias_declaration" | "type_item" => Some(SymbolKind::Type),
        "type_spec" if language == "go" => Some(go_type_spec_symbol_kind(node)),
        "interface_declaration" => Some(SymbolKind::Interface),
        "protocol_declaration" => Some(SymbolKind::Protocol),
        "enum_item" | "enum_declaration" => Some(SymbolKind::Enum),
        "typealias_declaration" => Some(SymbolKind::TypeAlias),
        "associatedtype_declaration" => Some(SymbolKind::AssociatedType),
        "const_item" | "const_declaration" => Some(SymbolKind::Constant),
        "let_declaration" | "variable_declaration" | "lexical_declaration" => Some(SymbolKind::Variable),
        _ => None,
    };
    if let Some(sk) = symbol_kind
        && let Some(name_node) = node.child_by_field_name("name")
    {
        let is_swift_extension = language == "swift" && sk == SymbolKind::Extension;
        symbols.push(SymbolInfo {
            name: node_text(&name_node, source).to_string(),
            kind: sk,
            span: span_from_node(node),
            type_annotation: node
                .child_by_field_name("type")
                .map(|n| node_text(&n, source).to_string()),
            container_name: if language == "swift" {
                swift_container_name(node, source)
            } else {
                None
            },
            extended_type: if is_swift_extension {
                Some(node_text(&name_node, source).trim().to_string())
            } else {
                None
            },
            inherited_types: if language == "swift" {
                swift_inherited_types(node, source)
            } else {
                Vec::new()
            },
            doc: None,
        });
    }
    true
}

fn collect_imports(node: &tree_sitter::Node, source: &str, language: &str, imports: &mut Vec<ImportInfo>) {
    let kind = node.kind();
    let is_import = match language {
        "python" => kind == "import_statement" || kind == "import_from_statement",
        "javascript" | "typescript" | "tsx" => kind == "import_statement",
        "rust" => kind == "use_declaration",
        "go" => kind == "import_declaration" || kind == "import_spec",
        "java" | "kotlin" => kind == "import_declaration",
        "swift" => kind == "import_declaration",
        _ => false,
    };
    let mut handled = false;
    if language == "python" && is_import {
        if kind == "import_statement" {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() != "aliased_import" && child.kind() != "dotted_name" {
                    continue;
                }
                let raw = node_text(&child, source);
                let raw_trimmed = raw.trim();
                let (source_name, alias) = if let Some((left, right)) = raw_trimmed.split_once(" as ") {
                    (left.trim().to_string(), Some(right.trim().to_string()))
                } else {
                    (raw_trimmed.to_string(), None)
                };
                if source_name.is_empty() {
                    continue;
                }
                imports.push(ImportInfo {
                    source: source_name,
                    items: Vec::new(),
                    alias,
                    is_wildcard: false,
                    span: span_from_node(node),
                });
            }
            handled = true;
        } else if kind == "import_from_statement" {
            let module_node = node.child_by_field_name("module_name");
            let module_text = module_node
                .as_ref()
                .map(|n| node_text(n, source))
                .unwrap_or("")
                .trim()
                .to_string();

            let mut items: Vec<String> = Vec::new();
            let mut is_wildcard = false;
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if let Some(module_node) = module_node.as_ref() {
                    if child.start_byte() == module_node.start_byte() && child.end_byte() == module_node.end_byte() {
                        continue;
                    }
                }
                match child.kind() {
                    "wildcard_import" => {
                        is_wildcard = true;
                    }
                    "aliased_import" => {
                        let name_node = child.child_by_field_name("name");
                        let raw = if let Some(name_node) = name_node {
                            node_text(&name_node, source)
                        } else {
                            node_text(&child, source)
                        };
                        let name = raw.split(" as ").next().unwrap_or("").trim();
                        let name = name.rsplit('.').next().unwrap_or("").trim();
                        if !name.is_empty() {
                            items.push(name.to_string());
                        }
                    }
                    "dotted_name" => {
                        let raw = node_text(&child, source);
                        let name = raw.rsplit('.').next().unwrap_or("").trim();
                        if !name.is_empty() {
                            items.push(name.to_string());
                        }
                    }
                    _ => {}
                }
            }

            if !module_text.is_empty() {
                imports.push(ImportInfo {
                    source: module_text,
                    items,
                    alias: None,
                    is_wildcard,
                    span: span_from_node(node),
                });
            }
            handled = true;
        }
    }

    if language == "go" && kind == "import_spec" && !handled {
        if let Some(import_info) = parse_go_import_spec(node, source) {
            imports.push(import_info);
            handled = true;
        }
    }

    if language == "swift" && is_import && !handled {
        let text = node_text(node, source);
        let cleaned = text.trim().trim_start_matches("import").trim().to_string();
        if cleaned.is_empty() {
            return;
        }
        let mut parts = cleaned.split_whitespace();
        let first = parts.next().unwrap_or("");
        let module_token = match first {
            "class" | "struct" | "enum" | "protocol" | "typealias" | "func" | "var" | "let" => {
                parts.next().unwrap_or("")
            }
            _ => first,
        };
        if !module_token.is_empty() {
            let module = module_token.split('.').next().unwrap_or("").trim();
            if !module.is_empty() {
                imports.push(ImportInfo {
                    source: module.to_string(),
                    items: Vec::new(),
                    alias: None,
                    is_wildcard: false,
                    span: span_from_node(node),
                });
                handled = true;
            }
        }
    }

    if language == "rust" && is_import && !handled {
        let text = node_text(node, source);
        let mut cleaned = text
            .trim()
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim()
            .to_string();

        if let Some((before, _)) = cleaned.split_once(" as ") {
            cleaned = before.trim().to_string();
        }

        if let Some((head, rest)) = cleaned.split_once('{') {
            let module_base = head.trim().trim_end_matches("::").to_string();
            let items_part = rest.split_once('}').map(|(a, _)| a).unwrap_or("");
            for raw in items_part.split(',') {
                let item_raw = raw.trim();
                if item_raw.is_empty() || item_raw == "self" {
                    continue;
                }
                if item_raw == "*" {
                    imports.push(ImportInfo {
                        source: module_base.clone(),
                        items: Vec::new(),
                        alias: None,
                        is_wildcard: true,
                        span: span_from_node(node),
                    });
                    continue;
                }
                let item_clean = item_raw.split_once(" as ").map(|(a, _)| a).unwrap_or(item_raw);
                let item_clean = item_clean.trim();
                let (module, item) = if let Some((prefix, name)) = item_clean.rsplit_once("::") {
                    (format!("{}::{}", module_base, prefix.trim_matches(':')), name.trim())
                } else {
                    (module_base.clone(), item_clean)
                };
                if !item.is_empty() {
                    imports.push(ImportInfo {
                        source: module,
                        items: vec![item.to_string()],
                        alias: None,
                        is_wildcard: false,
                        span: span_from_node(node),
                    });
                }
            }
            handled = true;
        } else {
            let mut is_wildcard = false;
            let item_part = if cleaned.ends_with("::*") {
                is_wildcard = true;
                cleaned.trim_end_matches("::*").trim().to_string()
            } else {
                cleaned.clone()
            };
            let parts: Vec<&str> = item_part.split("::").filter(|p| !p.is_empty()).collect();
            let (module, item) = if parts.len() >= 2 {
                let last = parts[parts.len() - 1];
                let module_path = parts[..parts.len() - 1].join("::");
                if last.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    (module_path, last.to_string())
                } else {
                    (parts.join("::"), String::new())
                }
            } else {
                (item_part.trim().to_string(), String::new())
            };
            imports.push(ImportInfo {
                source: module,
                items: if item.is_empty() || is_wildcard {
                    Vec::new()
                } else {
                    vec![item]
                },
                alias: None,
                is_wildcard,
                span: span_from_node(node),
            });
            handled = true;
        }
    }

    if is_import && !handled {
        let text = node_text(node, source);
        let mut source_name = text.to_string();
        let mut is_wildcard = text.contains('*');
        let mut items: Vec<String> = Vec::new();

        let strip_quotes = |raw: &str| {
            let raw = raw.trim();
            if raw.len() < 2 {
                return raw.to_string();
            }
            let first = raw.chars().next().unwrap_or('\0');
            let last = raw.chars().last().unwrap_or('\0');
            if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
                return raw[1..raw.len() - 1].to_string();
            }
            raw.to_string()
        };

        if language == "swift" {
            let mut module = None;
            let text_line = text.lines().next().unwrap_or("").trim();
            if let Some(rest) = text_line
                .strip_prefix("import ")
                .or_else(|| text_line.strip_prefix("@testable import "))
                .or_else(|| text_line.strip_prefix("@_exported import "))
            {
                let rest = rest.trim();
                let rest = if let Some(stripped) = rest.strip_prefix("typealias ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("struct ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("class ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("enum ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("protocol ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("func ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("var ") {
                    stripped
                } else if let Some(stripped) = rest.strip_prefix("let ") {
                    stripped
                } else {
                    rest
                };
                let mut ident = rest
                    .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.'))
                    .next()
                    .unwrap_or("")
                    .to_string();
                if let Some((first, _)) = ident.split_once('.') {
                    ident = first.to_string();
                }
                if !ident.is_empty() {
                    module = Some(ident);
                }
            }
            if let Some(mod_name) = module {
                source_name = mod_name;
            }
            is_wildcard = false;
        }

        if matches!(language, "javascript" | "typescript" | "tsx") {
            if let Some(source_node) = node.child_by_field_name("source") {
                source_name = strip_quotes(&node_text(&source_node, source));
            } else {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "string" {
                        source_name = strip_quotes(&node_text(&child, source));
                        break;
                    }
                }
            }

            let clause = node.child_by_field_name("import_clause").or_else(|| {
                let mut cursor = node.walk();
                node.children(&mut cursor).find(|c| c.kind() == "import_clause")
            });
            if let Some(clause) = clause {
                let mut cursor = clause.walk();
                for child in clause.children(&mut cursor) {
                    match child.kind() {
                        "identifier" => items.push(node_text(&child, source).to_string()),
                        "namespace_import" => {
                            is_wildcard = true;
                        }
                        "named_imports" => {
                            let mut c2 = child.walk();
                            for spec in child.children(&mut c2) {
                                if spec.kind() != "import_specifier" {
                                    continue;
                                }
                                if let Some(name_node) = spec.child_by_field_name("name") {
                                    items.push(node_text(&name_node, source).to_string());
                                    continue;
                                }
                                let mut c3 = spec.walk();
                                for n in spec.children(&mut c3) {
                                    if n.kind() == "identifier" || n.kind() == "property_identifier" {
                                        items.push(node_text(&n, source).to_string());
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        imports.push(ImportInfo {
            source: source_name,
            items,
            alias: None,
            is_wildcard,
            span: span_from_node(node),
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_imports(&child, source, language, imports);
    }
}

pub(crate) fn extract_exports(root: &tree_sitter::Node, source: &str, language: &str) -> Vec<ExportInfo> {
    let mut exports = Vec::with_capacity(16);
    collect_exports(root, source, language, &mut exports);
    if language == "python" {
        let mut seen = std::collections::HashSet::new();
        exports.retain(|item| seen.insert(item.name.clone()));
    }
    exports
}

fn parse_namespace_export_alias(text: &str) -> Option<String> {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let after_as = compact.split(" as ").nth(1)?;
    let alias = after_as.split(" from ").next()?.trim();
    if alias.is_empty() || alias == "*" {
        return None;
    }
    Some(alias.to_string())
}

fn collect_exports(node: &tree_sitter::Node, source: &str, language: &str, exports: &mut Vec<ExportInfo>) {
    let kind = node.kind();
    let is_export = match language {
        "javascript" | "typescript" | "tsx" => kind == "export_statement",
        "python" => kind == "assignment" || kind == "expression_statement",
        _ => false,
    };
    if is_export {
        let text = node_text(node, source);
        if language == "python" {
            // Only treat explicit __all__ assignments as exports.
            if text.contains("__all__") {
                exports.push(ExportInfo {
                    name: text.lines().next().unwrap_or("").to_string(),
                    kind: ExportKind::Named,
                    source: None,
                    exported_as: None,
                    span: span_from_node(node),
                });
            }
        } else if matches!(language, "javascript" | "typescript" | "tsx") {
            let export_kind = if node.child_by_field_name("default").is_some() {
                ExportKind::Default
            } else if node.child_by_field_name("source").is_some() {
                ExportKind::ReExport
            } else {
                ExportKind::Named
            };
            let source_module = node
                .child_by_field_name("source")
                .map(|n| strip_quoted_text(node_text(&n, source)));

            let mut names: Vec<String> = Vec::new();
            if let Some(decl) = node.child_by_field_name("declaration") {
                if let Some(name_node) = decl.child_by_field_name("name") {
                    names.push(node_text(&name_node, source).to_string());
                } else if decl.kind() == "variable_declaration" {
                    let mut cursor = decl.walk();
                    for child in decl.children(&mut cursor) {
                        if child.kind() == "variable_declarator" {
                            if let Some(name_node) = child.child_by_field_name("name") {
                                let name = node_text(&name_node, source).to_string();
                                if !name.is_empty() {
                                    names.push(name);
                                }
                            }
                        }
                    }
                }
            }
            if names.is_empty() {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "export_clause" {
                        let mut c2 = child.walk();
                        for spec in child.children(&mut c2) {
                            if spec.kind() != "export_specifier" {
                                continue;
                            }
                            let explicit_name = spec
                                .child_by_field_name("name")
                                .map(|n| node_text(&n, source).to_string());
                            let explicit_alias = spec
                                .child_by_field_name("alias")
                                .map(|n| node_text(&n, source).to_string());

                            let mut identifiers = Vec::new();
                            let mut c3 = spec.walk();
                            for n in spec.children(&mut c3) {
                                if n.kind() == "identifier" || n.kind() == "property_identifier" {
                                    let ident = node_text(&n, source).to_string();
                                    if !ident.is_empty() {
                                        identifiers.push(ident);
                                    }
                                }
                            }

                            let source_name = explicit_name
                                .filter(|name| !name.is_empty())
                                .or_else(|| identifiers.first().cloned());
                            let exported_as = explicit_alias.filter(|alias| !alias.is_empty()).or_else(|| {
                                if spec.kind() == "export_specifier"
                                    && node_text(&spec, source).contains(" as ")
                                    && identifiers.len() >= 2
                                {
                                    identifiers.get(1).cloned()
                                } else {
                                    None
                                }
                            });

                            if let Some(name) = source_name {
                                let public_name = exported_as.filter(|alias| alias != &name);
                                if public_name.is_some() {
                                    names.push(format!(
                                        "{name}\u{0}{public}",
                                        public = public_name.as_deref().unwrap_or("")
                                    ));
                                } else {
                                    names.push(name);
                                }
                            }
                        }
                    }
                }
            }
            if names.is_empty() && matches!(export_kind, ExportKind::ReExport) && text.contains('*') {
                if let Some(alias) = parse_namespace_export_alias(text) {
                    names.push(format!("*\u{0}{alias}"));
                } else {
                    names.push("*".to_string());
                }
            }
            if names.is_empty() {
                names.push(text.lines().next().unwrap_or("").to_string());
            }
            for name in names {
                let (name, exported_as) = match name.split_once('\0') {
                    Some((name, alias)) => (name.to_string(), Some(alias.to_string())),
                    None => (name, None),
                };
                exports.push(ExportInfo {
                    name,
                    kind: export_kind.clone(),
                    source: source_module.clone(),
                    exported_as,
                    span: span_from_node(node),
                });
            }
        } else {
            let export_kind = if node.child_by_field_name("default").is_some() {
                ExportKind::Default
            } else if node.child_by_field_name("source").is_some() {
                ExportKind::ReExport
            } else {
                ExportKind::Named
            };
            exports.push(ExportInfo {
                name: text.lines().next().unwrap_or("").to_string(),
                kind: export_kind,
                source: None,
                exported_as: None,
                span: span_from_node(node),
            });
        }
    }
    if language == "python" && (kind == "function_definition" || kind == "class_definition") {
        if let Some(parent) = node.parent() {
            if parent.kind() == "module" {
                if let Some(name_node) = node.child_by_field_name("name") {
                    let name = node_text(&name_node, source).to_string();
                    if !name.starts_with('_') {
                        exports.push(ExportInfo {
                            name,
                            kind: ExportKind::Named,
                            source: None,
                            exported_as: None,
                            span: span_from_node(node),
                        });
                    }
                }
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_exports(&child, source, language, exports);
    }
}

pub(crate) fn extract_structure(root: &tree_sitter::Node, source: &str, language: &str) -> Vec<StructureItem> {
    let mut items = Vec::with_capacity(32);
    collect_structure(root, source, language, &mut items);
    items
}

fn collect_structure(node: &tree_sitter::Node, source: &str, language: &str, items: &mut Vec<StructureItem>) {
    let kind = node.kind();

    fn extract_visibility(node: &tree_sitter::Node, source: &str, language: &str) -> Option<String> {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            match child.kind() {
                "visibility_modifier" => {
                    let text = node_text(&child, source).trim().to_string();
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
                "modifiers" => {
                    let lowered = node_text(&child, source).to_lowercase();
                    for keyword in ["public", "open", "protected", "private", "internal", "fileprivate"] {
                        if lowered.split_whitespace().any(|part| part == keyword) {
                            return Some(keyword.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        let header = node_text(node, source)
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_lowercase();
        match language {
            "rust" => {
                if header.starts_with("pub(") {
                    return header.split_whitespace().next().map(|part| part.to_string());
                }
                if header.starts_with("pub ") {
                    return Some("pub".to_string());
                }
            }
            "swift" => {
                for keyword in ["public", "open", "private", "fileprivate", "internal"] {
                    if header.starts_with(&format!("{keyword} ")) {
                        return Some(keyword.to_string());
                    }
                }
            }
            "java" | "c_sharp" | "csharp" => {
                for keyword in ["public", "protected", "private", "internal"] {
                    if header.starts_with(&format!("{keyword} ")) {
                        return Some(keyword.to_string());
                    }
                }
            }
            _ => {}
        }
        None
    }

    fn is_swift_method(node: &tree_sitter::Node) -> bool {
        let mut current = node.parent();
        for _ in 0..32 {
            let Some(parent) = current else {
                break;
            };
            match parent.kind() {
                "class_declaration"
                | "struct_declaration"
                | "enum_declaration"
                | "extension_declaration"
                | "protocol_declaration" => return true,
                _ => current = parent.parent(),
            }
        }
        false
    }

    fn swift_signature(node: &tree_sitter::Node, source: &str) -> Option<String> {
        let bytes = source.as_bytes();
        let start = node.start_byte();
        let mut end = node.end_byte();
        if start >= bytes.len() {
            return None;
        }
        if end > bytes.len() {
            end = bytes.len();
        }
        let raw = String::from_utf8_lossy(&bytes[start..end]);
        let mut sig = raw.as_ref();
        if let Some(idx) = sig.find('{') {
            sig = &sig[..idx];
        }
        let compact = sig.split_whitespace().collect::<Vec<_>>().join(" ");
        if compact.is_empty() { None } else { Some(compact) }
    }

    fn swift_qualified_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
        let name = node
            .child_by_field_name("name")
            .map(|n| node_text(&n, source).to_string())?;
        swift_qualified_name_for(node, source, &name)
    }

    fn swift_qualified_name_for(node: &tree_sitter::Node, source: &str, name: &str) -> Option<String> {
        let mut parts: Vec<String> = Vec::new();
        let mut current = node.parent();
        for _ in 0..32 {
            let Some(parent) = current else {
                break;
            };
            match parent.kind() {
                "class_declaration"
                | "struct_declaration"
                | "enum_declaration"
                | "extension_declaration"
                | "protocol_declaration" => {
                    if let Some(pname) = parent
                        .child_by_field_name("name")
                        .map(|n| node_text(&n, source).to_string())
                    {
                        parts.push(pname);
                    }
                }
                _ => {}
            }
            current = parent.parent();
        }

        if parts.is_empty() {
            Some(name.to_string())
        } else {
            parts.reverse();
            parts.push(name.to_string());
            Some(parts.join("."))
        }
    }

    fn rust_impl_target_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
        let header = rust_impl_display_name(node, source)?;
        let trimmed = header.trim();
        let after_impl = trimmed.strip_prefix("impl ")?;
        let target = after_impl.split(" for ").nth(1).unwrap_or(after_impl).trim();
        if target.is_empty() {
            None
        } else {
            Some(target.to_string())
        }
    }

    fn rust_qualified_name_for(node: &tree_sitter::Node, source: &str, name: &str) -> Option<String> {
        let mut parts: Vec<String> = Vec::new();
        let mut current = Some(*node);
        for _ in 0..32 {
            let Some(item) = current else {
                break;
            };
            match item.kind() {
                "mod_item" => {
                    if let Some(module_name) = item
                        .child_by_field_name("name")
                        .map(|n| node_text(&n, source).to_string())
                    {
                        parts.push(module_name);
                    }
                }
                "trait_item" => {
                    if let Some(trait_name) = item
                        .child_by_field_name("name")
                        .map(|n| node_text(&n, source).to_string())
                    {
                        parts.push(trait_name);
                    }
                }
                "impl_item" => {
                    if let Some(target_name) = rust_impl_target_name(&item, source) {
                        parts.push(target_name);
                    }
                }
                _ => {}
            }
            current = item.parent();
        }

        if parts.is_empty() {
            Some(name.to_string())
        } else {
            parts.reverse();
            parts.push(name.to_string());
            Some(parts.join("::"))
        }
    }

    fn go_package_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
        let mut current = Some(*node);
        for _ in 0..32 {
            let Some(item) = current else {
                break;
            };
            if item.kind() == "source_file" {
                let mut cursor = item.walk();
                for child in item.children(&mut cursor) {
                    if child.kind() != "package_clause" {
                        continue;
                    }
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let name = node_text(&name_node, source).trim().to_string();
                        if !name.is_empty() {
                            return Some(name);
                        }
                    }
                    let mut package_cursor = child.walk();
                    for grandchild in child.children(&mut package_cursor) {
                        if grandchild.kind() == "package_identifier" || grandchild.kind() == "identifier" {
                            let name = node_text(&grandchild, source).trim().to_string();
                            if !name.is_empty() {
                                return Some(name);
                            }
                        }
                    }
                }
                break;
            }
            current = item.parent();
        }
        None
    }

    fn go_receiver_type_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
        let receiver = node.child_by_field_name("receiver")?;
        let text = node_text(&receiver, source);
        let candidate = text
            .split(|c: char| !(c.is_alphanumeric() || c == '_'))
            .filter(|part| !part.is_empty())
            .last()?;
        Some(candidate.to_string())
    }

    fn go_qualified_name_for(
        node: &tree_sitter::Node,
        source: &str,
        name: &str,
        structure_kind: &StructureKind,
    ) -> Option<String> {
        let package_name = go_package_name(node, source)?;
        if structure_kind == &StructureKind::Method
            && let Some(receiver_type) = go_receiver_type_name(node, source)
        {
            return Some(format!("{package_name}.{receiver_type}.{name}"));
        }
        Some(format!("{package_name}.{name}"))
    }

    fn swift_enum_case_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
        if let Some(name_node) = node.child_by_field_name("name") {
            return Some(node_text(&name_node, source).to_string());
        }
        fn find_identifier(node: &tree_sitter::Node, source: &str) -> Option<String> {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if !child.is_named() {
                    continue;
                }
                let kind = child.kind();
                if kind.contains("identifier") {
                    return Some(node_text(&child, source).to_string());
                }
                if let Some(found) = find_identifier(&child, source) {
                    return Some(found);
                }
            }
            None
        }

        find_identifier(node, source)
    }

    fn swift_enum_case_nodes<'a>(node: &'a tree_sitter::Node<'a>) -> Vec<tree_sitter::Node<'a>> {
        let kind = node.kind();
        if kind == "enum_entry" {
            return vec![*node];
        }
        if !kind.contains("enum_case") {
            return Vec::new();
        }
        let mut nodes = Vec::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if !child.is_named() {
                continue;
            }
            if child.kind().contains("enum_case_element") {
                nodes.push(child);
            }
        }
        if nodes.is_empty() {
            nodes.push(*node);
        }
        nodes
    }

    // ── Documentation format heading extraction ──────────────────────────────
    // Each doc language uses different grammar node kinds and different child
    // nodes to hold the heading text — handle them before the generic match.
    if let Some(heading_name) = extract_doc_heading(node, source, language) {
        let mut children = Vec::new();
        // For RST/markdown `section` nodes, children are nested sections
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_structure(&child, source, language, &mut children);
        }
        items.push(StructureItem {
            kind: StructureKind::Section,
            name: Some(heading_name),
            qualified_name: None,
            container_name: None,
            visibility: None,
            extended_type: None,
            inherited_types: Vec::new(),
            span: span_from_node(node),
            children,
            decorators: Vec::new(),
            doc_comment: None,
            signature: None,
            body_span: None,
        });
        return; // don't recurse further — already walked children above
    }

    if language == "swift" && (kind.contains("enum_case") || kind == "enum_entry") {
        let mut added = false;
        for case_node in swift_enum_case_nodes(node) {
            let Some(case_name) = swift_enum_case_name(&case_node, source) else {
                continue;
            };
            let qualified_name = swift_qualified_name_for(&case_node, source, &case_name);
            items.push(StructureItem {
                kind: StructureKind::EnumCase,
                name: Some(case_name),
                qualified_name,
                container_name: swift_container_name(&case_node, source),
                visibility: None,
                extended_type: None,
                inherited_types: Vec::new(),
                span: span_from_node(&case_node),
                children: Vec::new(),
                decorators: Vec::new(),
                doc_comment: None,
                signature: None,
                body_span: None,
            });
            added = true;
        }
        if added {
            return;
        }
    }

    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "variable_declarator" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = node_text(&name_node, source).to_string();
            if !name.is_empty() {
                let mut is_func = false;
                if let Some(value_node) = node.child_by_field_name("value") {
                    let vkind = value_node.kind();
                    if matches!(
                        vkind,
                        "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
                    ) {
                        is_func = true;
                    }
                }
                if is_func {
                    items.push(StructureItem {
                        kind: StructureKind::Function,
                        name: Some(name),
                        qualified_name: None,
                        container_name: None,
                        visibility: None,
                        extended_type: None,
                        inherited_types: Vec::new(),
                        span: span_from_node(node),
                        children: Vec::new(),
                        decorators: Vec::new(),
                        doc_comment: None,
                        signature: None,
                        body_span: None,
                    });
                    return;
                }
            }
        }
    }

    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "assignment_expression" {
        if let Some(left_node) = node.child_by_field_name("left") {
            if let Some(right_node) = node.child_by_field_name("right") {
                let rkind = right_node.kind();
                if matches!(
                    rkind,
                    "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
                ) {
                    let name = resolve_js_assignment_name(&left_node, source);
                    if !name.is_empty() {
                        items.push(StructureItem {
                            kind: StructureKind::Function,
                            name: Some(name),
                            qualified_name: None,
                            container_name: None,
                            visibility: None,
                            extended_type: None,
                            inherited_types: Vec::new(),
                            span: span_from_node(node),
                            children: Vec::new(),
                            decorators: Vec::new(),
                            doc_comment: None,
                            signature: None,
                            body_span: None,
                        });
                        return;
                    }
                }
            }
        }
    }

    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "pair" {
        if let Some(key_node) = node.child_by_field_name("key") {
            if let Some(value_node) = node.child_by_field_name("value") {
                let vkind = value_node.kind();
                if matches!(
                    vkind,
                    "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
                ) {
                    let name = resolve_js_assignment_name(&key_node, source);
                    if !name.is_empty() {
                        items.push(StructureItem {
                            kind: StructureKind::Function,
                            name: Some(name),
                            qualified_name: None,
                            container_name: None,
                            visibility: None,
                            extended_type: None,
                            inherited_types: Vec::new(),
                            span: span_from_node(node),
                            children: Vec::new(),
                            decorators: Vec::new(),
                            doc_comment: None,
                            signature: None,
                            body_span: None,
                        });
                        return;
                    }
                }
            }
        }
    }

    let structure_kind = match kind {
        "function_definition" | "function_declaration" | "function_item" | "arrow_function" => {
            if language == "swift" && kind == "function_declaration" && is_swift_method(node) {
                Some(StructureKind::Method)
            } else if language == "kotlin" && kind == "function_declaration" && kotlin_is_method(node) {
                Some(StructureKind::Method)
            } else {
                Some(StructureKind::Function)
            }
        }
        "method_definition" | "method_declaration" => Some(StructureKind::Method),
        "class_definition" | "class_declaration" | "class" => {
            if language == "swift" && kind == "class_declaration" {
                swift_classlike_kind(node, source).or(Some(StructureKind::Class))
            } else {
                Some(StructureKind::Class)
            }
        }
        "type_spec" if language == "go" => Some(go_type_spec_kind(node)),
        "struct_item" | "struct_definition" | "struct_declaration" => Some(StructureKind::Struct),
        "interface_declaration" | "interface_definition" => Some(StructureKind::Interface),
        "protocol_declaration" => Some(StructureKind::Protocol),
        "enum_item" | "enum_definition" | "enum_declaration" => Some(StructureKind::Enum),
        "object_declaration" => Some(StructureKind::Class),
        "type_alias" => Some(StructureKind::TypeAlias),
        "typealias_declaration" => Some(StructureKind::TypeAlias),
        "associatedtype_declaration" => Some(StructureKind::AssociatedType),
        "module_definition" | "mod_item" => Some(StructureKind::Module),
        "trait_item" => Some(StructureKind::Trait),
        "impl_item" => Some(StructureKind::Impl),
        _ => None,
    };

    if let Some(sk) = structure_kind {
        let name = if language == "rust" && sk == StructureKind::Impl {
            rust_impl_display_name(node, source)
        } else {
            declaration_name_node(node, language)
                .map(|n| node_text(&n, source).to_string())
        };
        let qualified_name = match language {
            "swift" => swift_qualified_name(node, source),
            "rust" => name
                .as_deref()
                .and_then(|symbol_name| rust_qualified_name_for(node, source, symbol_name)),
            "go" => name
                .as_deref()
                .and_then(|symbol_name| go_qualified_name_for(node, source, symbol_name, &sk)),
            "kotlin" => name
                .as_deref()
                .and_then(|symbol_name| kotlin_qualified_name_for(node, source, symbol_name)),
            _ => None,
        };
        let (container_name, extended_type, inherited_types) = if language == "swift" {
            let inherited_types = swift_inherited_types(node, source);
            let extended_type = if sk == StructureKind::Extension {
                name.clone()
            } else {
                None
            };
            (swift_container_name(node, source), extended_type, inherited_types)
        } else if language == "kotlin" {
            (kotlin_container_name(node, source), None, Vec::new())
        } else {
            (None, None, Vec::new())
        };
        let body_span = node.child_by_field_name("body").map(|n| span_from_node(&n));
        let mut children = Vec::new();
        if let Some(body) = node.child_by_field_name("body") {
            collect_structure(&body, source, language, &mut children);
        } else if language == "swift"
            && matches!(
                sk,
                StructureKind::Class
                    | StructureKind::Struct
                    | StructureKind::Enum
                    | StructureKind::Extension
                    | StructureKind::Protocol
            )
        {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                collect_structure(&child, source, language, &mut children);
            }
        } else if language == "kotlin"
            && matches!(sk, StructureKind::Class | StructureKind::Interface | StructureKind::Enum)
        {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                collect_structure(&child, source, language, &mut children);
            }
        }
        let signature = if language == "swift" && (sk == StructureKind::Function || sk == StructureKind::Method) {
            swift_signature(node, source)
        } else {
            None
        };

        items.push(StructureItem {
            kind: sk,
            name,
            qualified_name,
            container_name,
            visibility: extract_visibility(node, source, language),
            extended_type,
            inherited_types,
            span: span_from_node(node),
            children,
            decorators: Vec::new(),
            doc_comment: None,
            signature,
            body_span,
        });
    } else {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_structure(&child, source, language, items);
        }
    }
}

fn rust_impl_display_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
    let text = node_text(node, source);
    let header = text.split('{').next()?.split_whitespace().collect::<Vec<_>>().join(" ");
    let header = header.trim();
    if header.is_empty() {
        None
    } else {
        Some(header.to_string())
    }
}

fn swift_classlike_kind(node: &tree_sitter::Node, source: &str) -> Option<StructureKind> {
    let text = node_text(node, source);
    let is_modifier = |token: &str| {
        matches!(
            token,
            "public"
                | "open"
                | "internal"
                | "fileprivate"
                | "private"
                | "final"
                | "indirect"
                | "prefix"
                | "postfix"
                | "infix"
                | "mutating"
                | "nonmutating"
                | "convenience"
                | "required"
                | "override"
                | "static"
        )
    };

    for raw_token in text.split_whitespace() {
        let token = raw_token.trim_matches(|ch: char| matches!(ch, '{' | '(' | '['));
        if token.is_empty() || token.starts_with('@') || is_modifier(token) {
            continue;
        }
        return match token {
            "extension" => Some(StructureKind::Extension),
            "enum" => Some(StructureKind::Enum),
            "struct" => Some(StructureKind::Struct),
            "class" => Some(StructureKind::Class),
            _ => None,
        };
    }
    None
}

/// Extract a heading name from a doc-format node, returning `Some(text)` if
/// this node represents a heading in the given doc language, `None` otherwise.
fn extract_doc_heading(node: &tree_sitter::Node, source: &str, language: &str) -> Option<String> {
    let kind = node.kind();
    match language {
        // ── Markdown ──────────────────────────────────────────────────────────
        // `atx_heading` node: children are marker(s) + inline text
        // e.g.  (atx_heading (atx_h2_marker) (inline "Section title"))
        "markdown" | "markdown_inline" => {
            if kind == "atx_heading" {
                let mut cursor = node.walk();
                let text = node
                    .children(&mut cursor)
                    .filter(|c| c.kind() == "inline" || c.kind() == "heading_content")
                    .map(|c| node_text(&c, source).trim().to_string())
                    .next()
                    .unwrap_or_default();
                if !text.is_empty() {
                    return Some(text);
                }
            }
            None
        }

        // ── reStructuredText ─────────────────────────────────────────────────
        // `section` node contains a `title` child whose text is the heading.
        "rst" => {
            if kind == "section" {
                let mut cursor = node.walk();
                let text = node
                    .children(&mut cursor)
                    .find(|c| c.kind() == "title")
                    .map(|t| node_text(&t, source).trim().to_string())
                    .unwrap_or_default();
                if !text.is_empty() {
                    return Some(text);
                }
            }
            None
        }

        // ── LaTeX ─────────────────────────────────────────────────────────────
        // `section` / `subsection` nodes: the curly_group child holds the title.
        // e.g.  (section (curly_group "Introduction"))
        "latex" => {
            if kind == "section"
                || kind == "subsection"
                || kind == "subsubsection"
                || kind == "chapter"
                || kind == "part"
            {
                let mut cursor = node.walk();
                let text = node
                    .children(&mut cursor)
                    .find(|c| c.kind() == "curly_group")
                    .map(|g| {
                        node_text(&g, source)
                            .trim_matches(|ch| ch == '{' || ch == '}')
                            .trim()
                            .to_string()
                    })
                    .unwrap_or_default();
                if !text.is_empty() {
                    return Some(text);
                }
            }
            None
        }

        // ── HTML ──────────────────────────────────────────────────────────────
        // `element` nodes: check if first child start_tag contains h1..h6 tag_name.
        "html" => {
            if kind == "element" {
                let mut cursor = node.walk();
                if let Some(start_tag) = node.children(&mut cursor).find(|c| c.kind() == "start_tag") {
                    let mut sc = start_tag.walk();
                    if let Some(tag_name_node) = start_tag.children(&mut sc).find(|c| c.kind() == "tag_name") {
                        let tag = node_text(&tag_name_node, source);
                        if matches!(tag, "h1" | "h2" | "h3" | "h4" | "h5" | "h6") {
                            // Collect all text children of the element (not the tags themselves)
                            let mut ec = node.walk();
                            let text: String = node
                                .children(&mut ec)
                                .filter(|c| c.kind() == "text")
                                .map(|c| node_text(&c, source).trim().to_string())
                                .collect::<Vec<_>>()
                                .join(" ");
                            if !text.is_empty() {
                                return Some(text);
                            }
                        }
                    }
                }
            }
            None
        }

        _ => None,
    }
}

pub(crate) fn extract_symbols(root: &tree_sitter::Node, source: &str, language: &str) -> Vec<SymbolInfo> {
    let mut symbols = Vec::new();
    collect_symbols(root, source, language, &mut symbols);
    symbols
}

fn collect_symbols(node: &tree_sitter::Node, source: &str, language: &str, symbols: &mut Vec<SymbolInfo>) {
    let kind = node.kind();

    fn swift_enum_case_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
        if let Some(name_node) = node.child_by_field_name("name") {
            return Some(node_text(&name_node, source).to_string());
        }
        fn find_identifier(node: &tree_sitter::Node, source: &str) -> Option<String> {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if !child.is_named() {
                    continue;
                }
                let kind = child.kind();
                if kind.contains("identifier") {
                    return Some(node_text(&child, source).to_string());
                }
                if let Some(found) = find_identifier(&child, source) {
                    return Some(found);
                }
            }
            None
        }

        find_identifier(node, source)
    }

    fn swift_enum_case_nodes<'a>(node: &'a tree_sitter::Node<'a>) -> Vec<tree_sitter::Node<'a>> {
        let kind = node.kind();
        if kind == "enum_entry" {
            return vec![*node];
        }
        if !kind.contains("enum_case") {
            return Vec::new();
        }
        let mut nodes = Vec::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if !child.is_named() {
                continue;
            }
            if child.kind().contains("enum_case_element") {
                nodes.push(child);
            }
        }
        if nodes.is_empty() {
            nodes.push(*node);
        }
        nodes
    }

    if language == "swift" && (kind.contains("enum_case") || kind == "enum_entry") {
        let mut added = false;
        for case_node in swift_enum_case_nodes(node) {
            let Some(case_name) = swift_enum_case_name(&case_node, source) else {
                continue;
            };
            symbols.push(SymbolInfo {
                name: case_name,
                kind: SymbolKind::EnumCase,
                span: span_from_node(&case_node),
                type_annotation: None,
                container_name: swift_container_name(&case_node, source),
                extended_type: None,
                inherited_types: Vec::new(),
                doc: None,
            });
            added = true;
        }
        if added {
            return;
        }
    }
    if matches!(language, "javascript" | "typescript" | "tsx") && kind == "variable_declarator" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = node_text(&name_node, source).to_string();
            let mut sk = SymbolKind::Variable;
            if let Some(value_node) = node.child_by_field_name("value") {
                let vkind = value_node.kind();
                if matches!(
                    vkind,
                    "arrow_function" | "function" | "function_expression" | "generator_function" | "async_function"
                ) {
                    sk = SymbolKind::Function;
                }
            }
            if !name.is_empty() {
                symbols.push(SymbolInfo {
                    name,
                    kind: sk,
                    span: span_from_node(node),
                    type_annotation: node
                        .child_by_field_name("type")
                        .map(|n| node_text(&n, source).to_string()),
                    container_name: None,
                    extended_type: None,
                    inherited_types: Vec::new(),
                    doc: None,
                });
                return;
            }
        }
    }

    let symbol_kind = match kind {
        "function_definition" | "function_declaration" | "function_item" => Some(SymbolKind::Function),
        "class_definition" | "class_declaration" => {
            if language == "swift" && kind == "class_declaration" {
                match swift_classlike_kind(node, source) {
                    Some(StructureKind::Enum) => Some(SymbolKind::Enum),
                    Some(StructureKind::Struct) => Some(SymbolKind::Type),
                    Some(StructureKind::Extension) => Some(SymbolKind::Extension),
                    _ => Some(SymbolKind::Class),
                }
            } else {
                Some(SymbolKind::Class)
            }
        }
        "type_alias_declaration" | "type_item" => Some(SymbolKind::Type),
        "type_alias" => Some(SymbolKind::TypeAlias),
        "type_spec" if language == "go" => Some(go_type_spec_symbol_kind(node)),
        "interface_declaration" => Some(SymbolKind::Interface),
        "protocol_declaration" => Some(SymbolKind::Protocol),
        "enum_item" | "enum_declaration" => Some(SymbolKind::Enum),
        "object_declaration" => Some(SymbolKind::Class),
        "typealias_declaration" => Some(SymbolKind::TypeAlias),
        "associatedtype_declaration" => Some(SymbolKind::AssociatedType),
        "const_item" | "const_declaration" => Some(SymbolKind::Constant),
        "let_declaration" | "variable_declaration" | "lexical_declaration" | "property_declaration" => {
            Some(SymbolKind::Variable)
        }
        _ => None,
    };
    if let Some(sk) = symbol_kind
        && let Some(name_node) = declaration_name_node(node, language)
    {
        let is_swift_extension = language == "swift" && sk == SymbolKind::Extension;
        symbols.push(SymbolInfo {
            name: node_text(&name_node, source).to_string(),
            kind: sk,
            span: span_from_node(node),
            type_annotation: node
                .child_by_field_name("type")
                .map(|n| node_text(&n, source).to_string()),
            container_name: if language == "swift" {
                swift_container_name(node, source)
            } else if language == "kotlin" {
                kotlin_container_name(node, source)
            } else {
                None
            },
            extended_type: if is_swift_extension {
                Some(node_text(&name_node, source).trim().to_string())
            } else {
                None
            },
            inherited_types: if language == "swift" {
                swift_inherited_types(node, source)
            } else {
                Vec::new()
            },
            doc: None,
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_symbols(&child, source, language, symbols);
    }
}

pub(crate) fn extract_diagnostics(root: &tree_sitter::Node, source: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::with_capacity(16);
    collect_diagnostics(root, source, &mut diags);
    diags
}

fn collect_diagnostics(node: &tree_sitter::Node, source: &str, diags: &mut Vec<Diagnostic>) {
    if node.is_error() {
        diags.push(Diagnostic {
            message: format!("Syntax error: unexpected '{}'", node_text(node, source)),
            severity: DiagnosticSeverity::Error,
            span: span_from_node(node),
        });
    } else if node.is_missing() {
        diags.push(Diagnostic {
            message: format!("Missing expected node: {}", node.kind()),
            severity: DiagnosticSeverity::Error,
            span: span_from_node(node),
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_diagnostics(&child, source, diags);
    }
}

fn resolve_js_assignment_name(node: &tree_sitter::Node, source: &str) -> String {
    match node.kind() {
        "identifier" | "property_identifier" | "shorthand_property_identifier" | "string" => node_text(node, source)
            .trim_matches(|c| c == '"' || c == '\'' || c == '`')
            .to_string(),
        "computed_property_name" => {
            // In computed properties like [key]: value, we want the inner expression
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.is_named() {
                    return resolve_js_assignment_name(&child, source);
                }
            }
            "".to_string()
        }
        "member_expression" => {
            if let Some(prop) = node.child_by_field_name("property") {
                resolve_js_assignment_name(&prop, source)
            } else {
                "".to_string()
            }
        }
        "subscript_expression" => {
            if let Some(index) = node.child_by_field_name("index") {
                resolve_js_assignment_name(&index, source)
            } else {
                "".to_string()
            }
        }
        _ => "".to_string(),
    }
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

    // -- Structure extraction tests --

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
    fn test_extract_rust_impl_display_name() {
        let source = r#"
            trait Runner { fn run(&self); }
            struct Service;
            impl Runner for Service {
                fn run(&self) {}
            }
        "#;
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Impl && item.name.as_deref() == Some("impl Runner for Service")
        }));
    }

    #[test]
    fn test_extract_rust_function_qualified_name() {
        let source = r#"
            mod parser {
                pub fn get_parser() {}
            }

            struct Service;
            impl Service {
                fn process(&self) {}
            }
        "#;
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Function
                && item.name.as_deref() == Some("get_parser")
                && item.qualified_name.as_deref() == Some("parser::get_parser")
        }));
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Function
                && item.name.as_deref() == Some("process")
                && item.qualified_name.as_deref() == Some("Service::process")
        }));
    }

    #[test]
    fn test_extract_go_function_qualified_name() {
        let source = r#"
            package languagepack

            func GetLanguage() {}

            type Registry struct{}

            func (r *Registry) Process() {}
        "#;
        let Some(tree) = parse_or_skip(source, "go") else {
            return;
        };
        let intel = extract_intelligence(source, "go", &tree);
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Function
                && item.name.as_deref() == Some("GetLanguage")
                && item.qualified_name.as_deref() == Some("languagepack.GetLanguage")
        }));
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Method
                && item.name.as_deref() == Some("Process")
                && item.qualified_name.as_deref() == Some("languagepack.Registry.Process")
        }));
    }

    #[test]
    fn test_extract_go_imports_strip_quotes_and_keep_alias() {
        let source = r#"
            package smoke

            import (
                "os"
                "path/filepath"
                j "encoding/json"
            )
        "#;
        let Some(tree) = parse_or_skip(source, "go") else {
            return;
        };
        let intel = extract_intelligence(source, "go", &tree);
        assert!(
            intel
                .imports
                .iter()
                .any(|imp| imp.source == "os" && imp.alias.is_none())
        );
        assert!(
            intel
                .imports
                .iter()
                .any(|imp| imp.source == "path/filepath" && imp.alias.is_none())
        );
        assert!(
            intel
                .imports
                .iter()
                .any(|imp| { imp.source == "encoding/json" && imp.alias.as_deref() == Some("j") })
        );
    }

    #[test]
    fn test_extract_kotlin_symbols_and_qualified_names() {
        let source = r#"
            package okhttp3.internal.http

            class RealInterceptorChain {
                fun getResponseWithInterceptorChain() {}
                val callId = 1
            }

            fun newChain() {}
        "#;
        let Some(tree) = parse_or_skip(source, "kotlin") else {
            return;
        };
        let intel = extract_intelligence(source, "kotlin", &tree);
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Class
                && item.name.as_deref() == Some("RealInterceptorChain")
        }));
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Function
                && item.name.as_deref() == Some("newChain")
                && item.qualified_name.as_deref() == Some("newChain")
        }));
        assert!(intel.structure.iter().any(|item| {
            item.kind == StructureKind::Method
                && item.name.as_deref() == Some("getResponseWithInterceptorChain")
                && item.qualified_name.as_deref() == Some("RealInterceptorChain.getResponseWithInterceptorChain")
                && item.container_name.as_deref() == Some("RealInterceptorChain")
        }));
        assert!(intel.symbols.iter().any(|item| {
            item.kind == SymbolKind::Class && item.name == "RealInterceptorChain"
        }));
        assert!(intel.symbols.iter().any(|item| {
            item.kind == SymbolKind::Function
                && item.name == "getResponseWithInterceptorChain"
                && item.container_name.as_deref() == Some("RealInterceptorChain")
        }));
        assert!(intel.symbols.iter().any(|item| {
            item.kind == SymbolKind::Variable
                && item.name == "callId"
                && item.container_name.as_deref() == Some("RealInterceptorChain")
        }));
    }

    #[test]
    fn test_extract_typescript_namespace_reexport_alias() {
        let source = r#"export * as routes from "./routes";"#;
        let Some(tree) = parse_or_skip(source, "typescript") else {
            return;
        };
        let intel = extract_intelligence(source, "typescript", &tree);
        assert_eq!(intel.exports.len(), 1);
        assert_eq!(intel.exports[0].name, "*");
        assert_eq!(intel.exports[0].exported_as.as_deref(), Some("routes"));
        assert!(matches!(intel.exports[0].kind, ExportKind::ReExport));
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
    fn test_extract_rust_visibility() {
        let source = "pub(crate) fn main() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);
        let func = &intel.structure[0];
        assert_eq!(func.visibility.as_deref(), Some("pub(crate)"));
    }

    #[test]
    fn test_extract_swift_visibility() {
        let source = "public struct Greeter {}\n";
        let Some(tree) = parse_or_skip(source, "swift") else {
            return;
        };
        let intel = extract_intelligence(source, "swift", &tree);
        let item = intel
            .structure
            .iter()
            .find(|item| item.name.as_deref() == Some("Greeter"))
            .expect("expected Greeter");
        assert_eq!(item.visibility.as_deref(), Some("public"));
    }

    #[test]
    fn test_extract_swift_struct_with_attribute() {
        let source = "@main\nstruct DrawThingsCLI: ParsableCommand {}\n";
        let Some(tree) = parse_or_skip(source, "swift") else {
            return;
        };
        let intel = extract_intelligence(source, "swift", &tree);
        let item = intel
            .structure
            .iter()
            .find(|item| item.name.as_deref() == Some("DrawThingsCLI"))
            .expect("expected DrawThingsCLI");
        assert_eq!(item.kind, StructureKind::Struct);
    }

    #[test]
    fn test_extract_swift_tree_sitter_owned_semantics() {
        let source = r#"
        public protocol EventLoop: Sendable {}

        extension EventLoop: Foo, Bar {
            /// Runs work.
            func run() {}
        }
        "#;
        let Some(tree) = parse_or_skip(source, "swift") else {
            return;
        };
        let intel = extract_intelligence(source, "swift", &tree);

        let extension = intel
            .structure
            .iter()
            .find(|item| item.kind == StructureKind::Extension)
            .expect("expected extension");
        assert_eq!(extension.name.as_deref(), Some("EventLoop"));
        assert_eq!(extension.extended_type.as_deref(), Some("EventLoop"));
        assert_eq!(
            extension.inherited_types,
            vec!["Foo".to_string(), "Bar".to_string()]
        );
        assert!(
            extension
                .qualified_name
                .as_deref()
                .is_some_and(|q| q.starts_with("extension EventLoop@"))
        );

        let method = extension
            .children
            .iter()
            .find(|item| item.kind == StructureKind::Method)
            .expect("expected method");
        assert_eq!(method.name.as_deref(), Some("run"));
        assert_eq!(method.container_name.as_deref(), Some("EventLoop"));
        assert_eq!(method.qualified_name.as_deref(), Some("EventLoop.run"));
        assert_eq!(method.doc_comment.as_deref(), Some("Runs work."));

        let method_symbol = intel
            .symbols
            .iter()
            .find(|item| item.name == "run")
            .expect("expected method symbol");
        assert_eq!(method_symbol.container_name.as_deref(), Some("EventLoop"));
        assert_eq!(method_symbol.doc.as_deref(), Some("Runs work."));
    }

    // -- Import extraction tests --

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

    // -- Comment extraction tests --

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

    // -- Metrics tests --

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

    // -- Symbol extraction tests --

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

    // -- Diagnostics tests --

    #[test]
    fn test_error_nodes_detected() {
        // Use Python with clearly invalid syntax to avoid segfault in some grammars
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

    // -- Docstring tests --

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

    // -- Language field test --

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
    fn test_extract_js_assignment_function() {
        let source = "PART_MAPPING.prop = function() {};\nPART_MAPPING['sub'] = () => {};";
        let Some(tree) = parse_or_skip(source, "tsx") else {
            return;
        };
        let intel = extract_intelligence(source, "tsx", &tree);

        assert_eq!(intel.structure.len(), 2);
        assert_eq!(intel.structure[0].name.as_deref(), Some("prop"));
        assert_eq!(intel.structure[1].name.as_deref(), Some("sub"));
    }
}
