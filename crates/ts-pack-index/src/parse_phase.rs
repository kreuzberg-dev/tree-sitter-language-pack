use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use rayon::prelude::*;
use tree_sitter_language_pack as ts_pack;

use crate::duplicate;
use crate::go;
use crate::pathing;
use crate::provenance;
use crate::python;
use crate::rust;
use crate::swift;
use crate::tags;
use crate::{
    CallRef, CallRefKind, CloneCandidate, ExportAliasRequest, FileNode, GoFileContext, ImportNode, ImportSymbolRequest,
    MAX_FILE_BYTES, ManifestEntry, PythonFileContext, ReExportSymbolRequest, RelRow, RustFileContext, SwiftFileContext,
    SymbolNode, WINNOW_LARGE_K, WINNOW_LARGE_W, WINNOW_MEDIUM_K, WINNOW_MEDIUM_W, WINNOW_MIN_FINGERPRINTS,
    WINNOW_MIN_TOKENS, WINNOW_SMALL_K, WINNOW_SMALL_TOKEN_THRESHOLD, WINNOW_SMALL_W,
};

pub(crate) struct FileResult {
    pub(crate) language: String,
    pub(crate) file_node: FileNode,
    pub(crate) file_facts: ts_pack::FileFacts,
    pub(crate) symbols: HashMap<&'static str, Vec<SymbolNode>>,
    pub(crate) relations: Vec<RelRow>,
    pub(crate) imports: Vec<ImportNode>,
    pub(crate) import_rels: Vec<RelRow>,
    pub(crate) call_refs: Vec<CallRef>,
    pub(crate) swift_extensions: Option<HashMap<String, HashSet<String>>>,
    pub(crate) swift_context: Option<SwiftFileContext>,
    pub(crate) python_context: Option<PythonFileContext>,
    pub(crate) rust_context: Option<RustFileContext>,
    pub(crate) go_context: Option<GoFileContext>,
    pub(crate) clone_candidates: Vec<CloneCandidate>,
    pub(crate) db_models: Vec<String>,
    pub(crate) external_urls: Vec<String>,
    pub(crate) import_symbol_requests: Vec<ImportSymbolRequest>,
    pub(crate) reexport_symbol_requests: Vec<ReExportSymbolRequest>,
    pub(crate) export_alias_requests: Vec<ExportAliasRequest>,
    pub(crate) launch_calls: Vec<String>,
    pub(crate) timings: ParseTimings,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ParseTimings {
    pub(crate) parse_tree_secs: f64,
    pub(crate) file_facts_secs: f64,
    pub(crate) process_secs: f64,
    pub(crate) tags_secs: f64,
}

struct ParsedSource {
    lang_name: &'static str,
    source: String,
    parsed_tree: Option<ts_pack::Tree>,
    file_facts: ts_pack::FileFacts,
    process_result: Option<ts_pack::ProcessResult>,
    timings: ParseTimings,
}

struct TagExtraction {
    exported_names: HashSet<String>,
    call_sites: Vec<tags::CallSite>,
    db_models: Vec<String>,
    external_urls: Vec<String>,
    launch_calls: Vec<String>,
    tags_secs: f64,
}

const SWIFT_SAFE_PAREN_NESTING_LIMIT: usize = 2048;

fn is_apple_fact_file(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    normalized.ends_with(".xcodeproj/project.pbxproj")
        || normalized.ends_with(".xcworkspace/contents.xcworkspacedata")
        || normalized.ends_with(".xcscheme")
}

fn swift_extension_qualified_name(name: &str, filepath: &str, start_line: usize) -> String {
    format!("extension {name}@{filepath}:{}", start_line + 1)
}

fn walk_item(
    item: &ts_pack::StructureItem,
    parent_id: &str,
    filepath: &str,
    project_id: Arc<str>,
    exported_names: &HashSet<String>,
    symbols: &mut HashMap<&'static str, Vec<SymbolNode>>,
    relations: &mut Vec<RelRow>,
    language: &str,
) {
    let stable_project_id = pathing::canonical_project_id(project_id.as_ref());
    let label: &'static str = match item.kind {
        ts_pack::StructureKind::Class => "Class",
        ts_pack::StructureKind::Function | ts_pack::StructureKind::Method => "Function",
        ts_pack::StructureKind::Interface => "Interface",
        ts_pack::StructureKind::Protocol => "Protocol",
        ts_pack::StructureKind::Trait => "Trait",
        ts_pack::StructureKind::Impl => "Impl",
        ts_pack::StructureKind::Struct => "Struct",
        ts_pack::StructureKind::Enum => "Enum",
        ts_pack::StructureKind::EnumCase => "EnumCase",
        ts_pack::StructureKind::Extension => "Extension",
        ts_pack::StructureKind::TypeAlias => "TypeAlias",
        ts_pack::StructureKind::AssociatedType => "AssociatedType",
        ts_pack::StructureKind::Module | ts_pack::StructureKind::Namespace => "Namespace",
        ts_pack::StructureKind::Section => "Section",
        _ => "Symbol",
    };

    let name = item.name.as_deref().unwrap_or("unnamed");
    let node_id = format!("{}:{}:{}:{}", project_id, label.to_ascii_lowercase(), filepath, name,);
    let stable_id = format!(
        "{}:{}:{}:{}",
        stable_project_id,
        label.to_ascii_lowercase(),
        filepath,
        name,
    );

    let visibility = item.visibility.as_deref().unwrap_or("").trim().to_lowercase();
    let top_level_parent = parent_id == format!("{}:file:{}", project_id, filepath);
    let name_is_public = match language {
        "go" => name.chars().next().map(|ch| ch.is_uppercase()).unwrap_or(false),
        "python" => top_level_parent && !name.starts_with('_'),
        _ => false,
    };
    let is_exported = matches!(visibility.as_str(), "public" | "open" | "pub")
        || visibility.starts_with("pub(")
        || exported_names.contains(name)
        || name_is_public;

    let qualified_name = if language == "swift" && matches!(item.kind, ts_pack::StructureKind::Extension) {
        item.name
            .as_deref()
            .map(|name| swift_extension_qualified_name(name, filepath, item.span.start_line))
    } else {
        item.qualified_name.clone()
    };

    symbols.entry(label).or_default().push(SymbolNode {
        id: node_id.clone(),
        stable_id,
        name: name.to_string(),
        kind: format!("{:?}", item.kind),
        qualified_name,
        container_name: item.container_name.clone(),
        filepath: filepath.to_string(),
        project_id: Arc::clone(&project_id),
        start_line: (item.span.start_line + 1) as u32,
        end_line: (item.span.end_line + 1) as u32,
        start_byte: item.span.start_byte,
        end_byte: item.span.end_byte,
        signature: item.signature.clone(),
        visibility: item.visibility.clone(),
        is_exported,
        doc_comment: item.doc_comment.clone(),
        swift_extended_type: item.extended_type.clone(),
        swift_inherited_types: item.inherited_types.clone(),
    });
    relations.push(RelRow {
        parent: parent_id.to_string(),
        child: node_id.clone(),
    });

    for child in &item.children {
        walk_item(
            child,
            &node_id,
            filepath,
            Arc::clone(&project_id),
            exported_names,
            symbols,
            relations,
            language,
        );
    }
}

fn label_for_symbol_kind(kind: &ts_pack::SymbolKind) -> &'static str {
    match kind {
        ts_pack::SymbolKind::Function => "Function",
        ts_pack::SymbolKind::Class => "Class",
        ts_pack::SymbolKind::Interface => "Interface",
        ts_pack::SymbolKind::Protocol => "Protocol",
        ts_pack::SymbolKind::Enum => "Enum",
        ts_pack::SymbolKind::EnumCase => "EnumCase",
        ts_pack::SymbolKind::Extension => "Extension",
        ts_pack::SymbolKind::Type | ts_pack::SymbolKind::TypeAlias => "TypeAlias",
        ts_pack::SymbolKind::AssociatedType => "AssociatedType",
        ts_pack::SymbolKind::Module => "Namespace",
        _ => "Symbol",
    }
}

fn add_symbol_info(
    sym: &ts_pack::SymbolInfo,
    parent_id: &str,
    filepath: &str,
    project_id: Arc<str>,
    exported_names: &HashSet<String>,
    symbols: &mut HashMap<&'static str, Vec<SymbolNode>>,
    relations: &mut Vec<RelRow>,
) {
    let stable_project_id = pathing::canonical_project_id(project_id.as_ref());
    let label = label_for_symbol_kind(&sym.kind);
    if label == "Symbol" {
        return;
    }

    let node_id = format!(
        "{}:{}:{}:{}",
        project_id,
        label.to_ascii_lowercase(),
        filepath,
        sym.name,
    );
    let stable_id = format!(
        "{}:{}:{}:{}",
        stable_project_id,
        label.to_ascii_lowercase(),
        filepath,
        sym.name,
    );

    let exists = symbols
        .get(label)
        .map(|items| {
            items.iter().any(|item| {
                item.name == sym.name && item.start_byte == sym.span.start_byte && item.end_byte == sym.span.end_byte
            })
        })
        .unwrap_or(false);
    if exists {
        return;
    }

    let qualified_name = if label == "Extension" {
        Some(swift_extension_qualified_name(&sym.name, filepath, sym.span.start_line))
    } else {
        None
    };

    symbols.entry(label).or_default().push(SymbolNode {
        id: node_id.clone(),
        stable_id,
        name: sym.name.clone(),
        kind: format!("{:?}", sym.kind),
        qualified_name,
        container_name: sym.container_name.clone(),
        filepath: filepath.to_string(),
        project_id: Arc::clone(&project_id),
        start_line: (sym.span.start_line + 1) as u32,
        end_line: (sym.span.end_line + 1) as u32,
        start_byte: sym.span.start_byte,
        end_byte: sym.span.end_byte,
        signature: sym.type_annotation.clone(),
        visibility: None,
        is_exported: exported_names.contains(&sym.name),
        doc_comment: sym.doc.clone(),
        swift_extended_type: sym.extended_type.clone(),
        swift_inherited_types: sym.inherited_types.clone(),
    });
    relations.push(RelRow {
        parent: parent_id.to_string(),
        child: node_id,
    });
}

fn add_synthetic_swift_main_symbol(
    symbols: &mut HashMap<&'static str, Vec<SymbolNode>>,
    relations: &mut Vec<RelRow>,
    file_id: &str,
    filepath: &str,
    project_id: Arc<str>,
    source: &str,
    call_sites: &[tags::CallSite],
) {
    if !filepath.ends_with("/main.swift") && filepath != "main.swift" {
        return;
    }

    let existing_main = symbols
        .get("Function")
        .map(|items| items.iter().any(|item| item.name == "main"))
        .unwrap_or(false);
    if existing_main {
        return;
    }

    let existing_spans: Vec<(usize, usize)> = symbols
        .values()
        .flat_map(|items| items.iter())
        .map(|item| (item.start_byte, item.end_byte))
        .collect();

    let mut uncovered_call_start: Option<usize> = None;
    for call in call_sites {
        let covered = existing_spans
            .iter()
            .any(|(sb, eb)| *sb <= call.start_byte && call.start_byte < *eb);
        if !covered {
            uncovered_call_start = Some(call.start_byte);
            break;
        }
    }

    let Some(start_byte) = uncovered_call_start else {
        return;
    };

    let stable_project_id = pathing::canonical_project_id(project_id.as_ref());
    let node_id = format!("{project_id}:function:{filepath}:main");
    let stable_id = format!("{stable_project_id}:function:{filepath}:main");
    let start_line = source[..start_byte].bytes().filter(|b| *b == b'\n').count() as u32 + 1;
    let end_line = source.lines().count().max(start_line as usize) as u32;

    symbols.entry("Function").or_default().push(SymbolNode {
        id: node_id.clone(),
        stable_id,
        name: "main".to_string(),
        kind: "Function".to_string(),
        qualified_name: Some("main".to_string()),
        container_name: None,
        filepath: filepath.to_string(),
        project_id,
        start_line,
        end_line,
        start_byte,
        end_byte: source.len(),
        signature: Some("synthetic swift entrypoint".to_string()),
        visibility: None,
        is_exported: false,
        doc_comment: None,
        swift_extended_type: None,
        swift_inherited_types: Vec::new(),
    });
    relations.push(RelRow {
        parent: file_id.to_string(),
        child: node_id,
    });
}

fn is_test_like_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/").to_lowercase();
    let basename = normalized.rsplit('/').next().unwrap_or(normalized.as_str());
    normalized.starts_with("tests/")
        || normalized.contains("/tests/")
        || normalized.contains("/__tests__/")
        || normalized.starts_with("spec/")
        || normalized.contains("/spec/")
        || normalized.contains(".test.")
        || normalized.contains(".spec.")
        || basename.starts_with("test_")
        || basename.ends_with("_test.py")
        || basename.ends_with("_test.rs")
        || basename.ends_with("_test.go")
}

fn normalized_export_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    let direct_ident = trimmed
        .chars()
        .all(|ch| ch.is_alphanumeric() || matches!(ch, '_' | '$'));
    if direct_ident {
        return Some(trimmed.to_string());
    }

    if let Some(rest) = trimmed.strip_prefix("export ") {
        for prefix in [
            "const ",
            "let ",
            "var ",
            "function ",
            "class ",
            "interface ",
            "type ",
            "enum ",
        ] {
            if let Some(after_prefix) = rest.strip_prefix(prefix) {
                let name: String = after_prefix
                    .chars()
                    .take_while(|ch| ch.is_alphanumeric() || matches!(ch, '_' | '$'))
                    .collect();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
    }

    None
}

fn build_clone_candidates(symbols: &HashMap<&'static str, Vec<SymbolNode>>, source: &str) -> Vec<CloneCandidate> {
    let mut local_clone_candidates = Vec::new();
    let Some(functions) = symbols.get("Function") else {
        return local_clone_candidates;
    };

    let source_bytes = source.as_bytes();
    for sym in functions {
        let start = sym.start_byte.min(source_bytes.len());
        let end = sym.end_byte.min(source_bytes.len());
        if end <= start {
            continue;
        }
        let tokens = duplicate::tokenize_normalized(&source_bytes[start..end]);
        if tokens.len() < WINNOW_MIN_TOKENS {
            let kgrams = duplicate::kgram_hashes(&tokens, WINNOW_SMALL_K);
            if kgrams.is_empty() {
                continue;
            }
            let token_set: HashSet<u64> = tokens.into_iter().collect();
            let span_len = sym.end_line.saturating_sub(sym.start_line);
            local_clone_candidates.push(CloneCandidate {
                symbol_id: sym.id.clone(),
                filepath: sym.filepath.clone(),
                span_len,
                token_set,
                fingerprints: vec![HashSet::new(), HashSet::new(), HashSet::new()],
                kgrams,
            });
            continue;
        }

        let mut fps_small: HashSet<u64>;
        let mut fps_medium: HashSet<u64>;
        let mut fps_large: HashSet<u64>;
        let mut kgrams: HashSet<u64>;
        if tokens.len() < WINNOW_SMALL_TOKEN_THRESHOLD {
            kgrams = duplicate::kgram_hashes(&tokens, WINNOW_SMALL_K);
            fps_small = duplicate::winnow_fingerprints(&tokens, WINNOW_SMALL_K, WINNOW_SMALL_W);
            if fps_small.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                fps_small.clear();
            }
            fps_medium = HashSet::new();
            fps_large = HashSet::new();
        } else {
            fps_small = duplicate::winnow_fingerprints(&tokens, WINNOW_SMALL_K, WINNOW_SMALL_W);
            if fps_small.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                fps_small.clear();
            }
            fps_medium = duplicate::winnow_fingerprints(&tokens, WINNOW_MEDIUM_K, WINNOW_MEDIUM_W);
            if fps_medium.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                fps_medium.clear();
            }
            fps_large = duplicate::winnow_fingerprints(&tokens, WINNOW_LARGE_K, WINNOW_LARGE_W);
            if fps_large.len() < WINNOW_MIN_FINGERPRINTS.saturating_sub(4) {
                fps_large.clear();
            }
            kgrams = HashSet::new();
            if fps_small.is_empty() && fps_medium.is_empty() && fps_large.is_empty() {
                kgrams = duplicate::kgram_hashes(&tokens, WINNOW_SMALL_K);
            }
        }

        if fps_small.is_empty() && fps_medium.is_empty() && fps_large.is_empty() && kgrams.is_empty() {
            continue;
        }
        let token_set: HashSet<u64> = tokens.into_iter().collect();
        let span_len = sym.end_line.saturating_sub(sym.start_line);
        local_clone_candidates.push(CloneCandidate {
            symbol_id: sym.id.clone(),
            filepath: sym.filepath.clone(),
            span_len,
            token_set,
            fingerprints: vec![fps_small, fps_medium, fps_large],
            kgrams,
        });
    }

    local_clone_candidates
}

fn detect_entry_language(entry: &ManifestEntry) -> Option<&'static str> {
    if is_apple_fact_file(&entry.rel_path) {
        return Some("text");
    }
    if entry.ext == "svg" {
        return Some("xml");
    }
    match ts_pack::detect_language_from_extension(&entry.ext) {
        Some(lang) => Some(lang),
        None => {
            eprintln!(
                "[ts-pack-index] detect_language_from_extension failed: {}",
                entry.rel_path
            );
            None
        }
    }
}

fn ensure_language_available(lang_name: &str) -> bool {
    if lang_name == "text" || ts_pack::has_language(lang_name) {
        return true;
    }
    match ts_pack::download(&[lang_name]) {
        Ok(_) => true,
        Err(err) => {
            eprintln!("[ts-pack-index] download failed: {lang} ({err})", lang = lang_name);
            false
        }
    }
}

fn read_entry_source(entry: &ManifestEntry) -> Option<String> {
    let source = match std::fs::read_to_string(&entry.abs_path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("[ts-pack-index] read failed: {} ({})", entry.rel_path, err);
            return None;
        }
    };
    if source.len() > MAX_FILE_BYTES {
        eprintln!(
            "[ts-pack-index] skipped oversized file: {} ({})",
            entry.rel_path,
            source.len()
        );
        return None;
    }
    Some(source)
}

fn max_delimiter_nesting(source: &str, opener: char, closer: char) -> usize {
    let mut depth = 0usize;
    let mut max_depth = 0usize;
    for ch in source.chars() {
        if ch == opener {
            depth += 1;
            if depth > max_depth {
                max_depth = depth;
            }
        } else if ch == closer && depth > 0 {
            depth -= 1;
        }
    }
    max_depth
}

fn swift_requires_parse_guard(source: &str) -> bool {
    max_delimiter_nesting(source, '(', ')') > SWIFT_SAFE_PAREN_NESTING_LIMIT
}

fn parse_source_tree(rel_path: &str, lang_name: &str, source: &str) -> Option<(Option<ts_pack::Tree>, f64)> {
    let t_parse_tree = Instant::now();
    let parsed_tree = if lang_name == "text" {
        None
    } else {
        match ts_pack::parse_string(lang_name, source.as_bytes()) {
            Ok(tree) => Some(tree),
            Err(err) => {
                eprintln!("[ts-pack-index] parse failed: {} ({})", rel_path, err);
                return None;
            }
        }
    };
    Some((parsed_tree, t_parse_tree.elapsed().as_secs_f64()))
}

fn extract_entry_file_facts(
    parsed_tree: Option<&ts_pack::Tree>,
    source: &str,
    lang_name: &str,
    rel_path: &str,
) -> (ts_pack::FileFacts, f64) {
    let t_file_facts = Instant::now();
    let file_facts = match parsed_tree {
        Some(tree) => {
            ts_pack::extract_file_facts_from_tree(tree, source, lang_name, Some(rel_path)).unwrap_or_default()
        }
        None => ts_pack::extract_file_facts(source, lang_name, Some(rel_path)).unwrap_or_default(),
    };
    (file_facts, t_file_facts.elapsed().as_secs_f64())
}

fn process_entry_source(
    rel_path: &str,
    lang_name: &str,
    source: &str,
    parsed_tree: Option<&ts_pack::Tree>,
) -> Option<(Option<ts_pack::ProcessResult>, f64)> {
    let t_process = Instant::now();
    let result = match parsed_tree {
        None => None,
        Some(tree) => {
            let mut proc_config = ts_pack::ProcessConfig::new(lang_name);
            proc_config.symbols = true;
            match ts_pack::process_with_tree(source, &proc_config, tree) {
                Ok(result) => Some(result),
                Err(err) => {
                    eprintln!("[ts-pack-index] process failed: {} ({})", rel_path, err);
                    return None;
                }
            }
        }
    };
    Some((result, t_process.elapsed().as_secs_f64()))
}

fn parse_entry_source(entry: &ManifestEntry) -> Option<ParsedSource> {
    let lang_name = detect_entry_language(entry)?;
    if !ensure_language_available(lang_name) {
        return None;
    }
    let source = read_entry_source(entry)?;
    if lang_name == "swift" && swift_requires_parse_guard(&source) {
        eprintln!(
            "[ts-pack-index] swift parse guard fallback: {} (paren_nesting>{})",
            entry.rel_path, SWIFT_SAFE_PAREN_NESTING_LIMIT
        );
        return Some(ParsedSource {
            lang_name,
            source,
            parsed_tree: None,
            file_facts: ts_pack::FileFacts::default(),
            process_result: None,
            timings: ParseTimings::default(),
        });
    }
    let (parsed_tree, parse_tree_secs) = parse_source_tree(&entry.rel_path, lang_name, &source)?;
    let (file_facts, file_facts_secs) =
        extract_entry_file_facts(parsed_tree.as_ref(), &source, lang_name, &entry.rel_path);
    let (process_result, process_secs) =
        process_entry_source(&entry.rel_path, lang_name, &source, parsed_tree.as_ref())?;
    Some(ParsedSource {
        lang_name,
        source,
        parsed_tree,
        file_facts,
        process_result,
        timings: ParseTimings {
            parse_tree_secs,
            file_facts_secs,
            process_secs,
            tags_secs: 0.0,
        },
    })
}

fn extract_tag_data(
    rel_path: &str,
    lang_name: &str,
    source: &str,
    parsed_tree: Option<&ts_pack::Tree>,
    process_result: Option<&ts_pack::ProcessResult>,
    file_facts: &ts_pack::FileFacts,
    tag_query_bundles: Option<&tags::BatchTagQueryBundles>,
) -> TagExtraction {
    let mut exported_names: HashSet<String> = process_result
        .map(|r| {
            r.exports
                .iter()
                .filter_map(|e| normalized_export_name(&e.name))
                .collect()
        })
        .unwrap_or_default();
    let t_tags = Instant::now();
    let tag_bundle = tag_query_bundles.and_then(|bundles| bundles.for_lang_and_source(lang_name, source.as_bytes()));
    let tags_result =
        parsed_tree.and_then(|tree| tags::run_tags(lang_name, tree, source.as_bytes(), rel_path, tag_bundle.as_ref()));
    let tags_secs = t_tags.elapsed().as_secs_f64();

    let (tag_exported, call_sites, mut db_models, external_calls, const_strings, launch_calls) = match tags_result {
        Some(tr) => (
            tr.exported_names,
            tr.call_sites,
            tr.db_models,
            tr.external_calls,
            tr.const_strings,
            tr.launch_calls,
        ),
        None => (
            HashSet::new(),
            Vec::new(),
            HashSet::new(),
            Vec::new(),
            HashMap::new(),
            Vec::new(),
        ),
    };
    exported_names.extend(tag_exported);
    for item in &file_facts.db_models {
        db_models.insert(item.model.clone());
    }

    let mut external_urls = Vec::new();
    for call in external_calls {
        let url = match call.arg {
            tags::ExternalCallArg::Literal(value) => Some(value),
            tags::ExternalCallArg::Identifier(name) => const_strings.get(&name).cloned(),
            tags::ExternalCallArg::ConcatIdentLiteral { ident, literal } => {
                const_strings.get(&ident).map(|base| format!("{base}{literal}"))
            }
            tags::ExternalCallArg::ConcatLiteralIdent { literal, ident } => {
                const_strings.get(&ident).map(|base| format!("{literal}{base}"))
            }
            tags::ExternalCallArg::UrlLiteral { path, base } => pathing::join_url(&base, &path),
            tags::ExternalCallArg::UrlWithBaseIdent { path, base_ident } => const_strings
                .get(&base_ident)
                .and_then(|base| pathing::join_url(base, &path)),
        };
        if let Some(url) = url {
            if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("env://") {
                external_urls.push(url);
            }
        }
    }

    TagExtraction {
        exported_names,
        call_sites,
        db_models: db_models.into_iter().collect(),
        external_urls,
        launch_calls,
        tags_secs,
    }
}

fn build_language_contexts(
    lang_name: &str,
    source: &str,
    file_id: &str,
    rel_path: &str,
    symbol_spans: &[(usize, usize, String)],
    call_sites: &[tags::CallSite],
    process_result: Option<&ts_pack::ProcessResult>,
) -> (
    Option<HashMap<String, HashSet<String>>>,
    Option<SwiftFileContext>,
    Option<PythonFileContext>,
    Option<RustFileContext>,
    Option<GoFileContext>,
) {
    let mut swift_extensions: Option<HashMap<String, HashSet<String>>> = None;
    let mut swift_context: Option<SwiftFileContext> = None;
    let mut python_context: Option<PythonFileContext> = None;
    let mut rust_context: Option<RustFileContext> = None;
    let mut go_context: Option<GoFileContext> = None;

    if lang_name == "swift" {
        let mut ext_map: HashMap<String, HashSet<String>> = HashMap::new();
        if let Some(result) = process_result {
            swift::collect_swift_extensions(&result.structure, &mut ext_map);
        }
        if !ext_map.is_empty() {
            swift_extensions = Some(ext_map);
        }

        let mut ext_spans = Vec::new();
        if let Some(result) = process_result {
            swift::collect_swift_extension_spans(&result.structure, &mut ext_spans);
        }

        let mut type_spans = Vec::new();
        if let Some(result) = process_result {
            swift::collect_swift_type_spans(&result.structure, &mut type_spans);
        }

        let var_types = swift::parse_swift_var_types(source);
        if !var_types.is_empty() || !call_sites.is_empty() {
            swift_context = Some(SwiftFileContext {
                file_id: file_id.to_string(),
                filepath: rel_path.to_string(),
                symbol_spans: symbol_spans.to_vec(),
                extension_spans: ext_spans,
                type_spans,
                call_sites: call_sites.to_vec(),
                var_types,
            });
        }
    }

    if lang_name == "python" {
        let mut module_aliases: HashMap<String, String> = HashMap::new();
        let mut imported_symbol_modules: HashMap<String, String> = HashMap::new();
        let (var_types, function_return_assignments, function_return_types) = python::parse_python_var_types(source);
        if let Some(result) = process_result {
            for imp in &result.imports {
                if imp.items.is_empty() {
                    if imp.alias.is_none() {
                        continue;
                    }
                    let Some(alias) = imp.alias.as_ref() else {
                        continue;
                    };
                    if alias.is_empty() || imp.source.is_empty() {
                        continue;
                    }
                    module_aliases.insert(alias.clone(), imp.source.clone());
                    continue;
                }
                if imp.source.is_empty() {
                    continue;
                }
                for item in &imp.items {
                    let cleaned = pathing::clean_import_name(item);
                    if cleaned.is_empty() {
                        continue;
                    }
                    imported_symbol_modules.insert(cleaned, imp.source.clone());
                }
            }
        }
        if !call_sites.is_empty()
            || !module_aliases.is_empty()
            || !imported_symbol_modules.is_empty()
            || !var_types.is_empty()
            || !function_return_assignments.is_empty()
            || !function_return_types.is_empty()
        {
            python_context = Some(PythonFileContext {
                file_id: file_id.to_string(),
                filepath: rel_path.to_string(),
                symbol_spans: symbol_spans.to_vec(),
                call_sites: call_sites.to_vec(),
                module_aliases,
                imported_symbol_modules,
                var_types,
                function_return_assignments,
                function_return_types,
            });
        }
    }

    if lang_name == "rust" {
        let var_types = rust::parse_rust_var_types(source);
        if !var_types.is_empty() {
            rust_context = Some(RustFileContext {
                filepath: rel_path.to_string(),
                var_types,
            });
        }
    }

    if lang_name == "go" {
        let (var_types, method_return_assignments, function_return_assignments) = go::parse_go_var_types(source);
        let method_return_types = go::parse_go_method_return_types(source);
        let function_return_types = go::parse_go_function_return_types(source);
        let mut import_aliases: HashMap<String, String> = HashMap::new();
        if let Some(result) = process_result {
            for imp in &result.imports {
                if let Some(alias) = imp.alias.as_ref().filter(|alias| !alias.is_empty()) {
                    import_aliases.insert(alias.clone(), imp.source.clone());
                }
            }
        }
        if !var_types.is_empty()
            || !method_return_assignments.is_empty()
            || !method_return_types.is_empty()
            || !function_return_assignments.is_empty()
            || !function_return_types.is_empty()
            || !import_aliases.is_empty()
            || !call_sites.is_empty()
        {
            go_context = Some(GoFileContext {
                file_id: file_id.to_string(),
                filepath: rel_path.to_string(),
                symbol_spans: symbol_spans.to_vec(),
                call_sites: call_sites.to_vec(),
                import_aliases,
                var_types,
                method_return_assignments,
                method_return_types,
                function_return_assignments,
                function_return_types,
            });
        }
    }

    (
        swift_extensions,
        swift_context,
        python_context,
        rust_context,
        go_context,
    )
}

fn parse_entry(
    entry: &ManifestEntry,
    pid: &Arc<str>,
    tag_query_bundles: Option<&tags::BatchTagQueryBundles>,
    clone_enrich_enabled: bool,
) -> Option<FileResult> {
    let rel_basename = entry.rel_path.rsplit('/').next().unwrap_or(entry.rel_path.as_str());
    // Producer-side standalone guard for a tiny set of repo-dotfiles that can
    // still reach ts-pack outside the rest_proxy manifest path. Broader policy
    // skips belong in the manifest builder, not here.
    if matches!(rel_basename, ".gitignore" | ".indexignore" | ".env" | ".env.example") {
        return None;
    }

    let rel_path = &entry.rel_path;
    let ParsedSource {
        lang_name,
        source,
        parsed_tree,
        file_facts,
        process_result: result,
        mut timings,
    } = parse_entry_source(entry)?;
    let debug_apple_facts = std::env::var("TS_PACK_DEBUG_APPLE_FACTS")
        .ok()
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(false);
    if debug_apple_facts && is_apple_fact_file(rel_path) {
        eprintln!(
            "[ts-pack-index] apple file facts: file={} targets={} bundled={} synced={} workspace_projects={} scheme_targets={}",
            rel_path,
            file_facts.apple_targets.len(),
            file_facts.apple_bundled_files.len(),
            file_facts.apple_synced_groups.len(),
            file_facts.apple_workspace_projects.len(),
            file_facts.apple_scheme_targets.len(),
        );
    }

    if entry.rel_path.contains("duplication_demo") {
        eprintln!(
            "[ts-pack-index] debug structure: {} (structure={}, symbols={}, imports={})",
            entry.rel_path,
            result.as_ref().map(|r| r.structure.len()).unwrap_or(0),
            result.as_ref().map(|r| r.symbols.len()).unwrap_or(0),
            result.as_ref().map(|r| r.imports.len()).unwrap_or(0),
        );
    }

    let file_name = PathBuf::from(&entry.abs_path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let canonical_pid = pathing::canonical_project_id(pid.as_ref());
    let file_id = format!("{}:file:{}", pid, rel_path);
    let stable_file_id = format!("{}:file:{}", canonical_pid, rel_path);

    let file_node = FileNode {
        id: file_id.clone(),
        stable_id: stable_file_id.clone(),
        name: file_name,
        filepath: rel_path.clone(),
        project_id: Arc::clone(pid),
        is_test: is_test_like_path(rel_path),
    };

    let mut symbols: HashMap<&'static str, Vec<SymbolNode>> = HashMap::new();
    let mut relations = Vec::new();
    let mut imports = Vec::new();
    let mut import_rels = Vec::new();
    let TagExtraction {
        exported_names,
        call_sites,
        db_models,
        external_urls,
        launch_calls,
        tags_secs,
    } = extract_tag_data(
        rel_path,
        lang_name,
        &source,
        parsed_tree.as_ref(),
        result.as_ref(),
        &file_facts,
        tag_query_bundles,
    );
    timings.tags_secs = tags_secs;

    let is_backend = rel_path.starts_with("src/api/")
        || rel_path.starts_with("src/services/")
        || rel_path.starts_with("src/webhooks/")
        || rel_path.starts_with("src/jobs/")
        || rel_path.starts_with("src/db/")
        || rel_path.starts_with("src/seed/")
        || rel_path == "src/server.ts";
    let is_public = rel_path.starts_with("src/public/");
    let is_backend = is_backend && !is_public;

    if let Some(result) = result.as_ref() {
        for item in &result.structure {
            walk_item(
                item,
                &file_id,
                rel_path,
                Arc::clone(pid),
                &exported_names,
                &mut symbols,
                &mut relations,
                lang_name,
            );
        }
        for sym in &result.symbols {
            add_symbol_info(
                sym,
                &file_id,
                rel_path,
                Arc::clone(pid),
                &exported_names,
                &mut symbols,
                &mut relations,
            );
        }
    }

    if lang_name == "swift" {
        add_synthetic_swift_main_symbol(
            &mut symbols,
            &mut relations,
            &file_id,
            rel_path,
            Arc::clone(pid),
            &source,
            &call_sites,
        );
    }

    let symbol_spans: Vec<(usize, usize, String)> = symbols
        .values()
        .flat_map(|v| v.iter())
        .map(|s| (s.start_byte, s.end_byte, s.id.clone()))
        .collect();

    // This only controls optional duplicate/clone grouping. Core symbols,
    // relations, call edges, and Swift enrichment inputs are still produced.
    let clone_candidates = if clone_enrich_enabled {
        build_clone_candidates(&symbols, &source)
    } else {
        Vec::new()
    };

    let mut seen_calls: HashSet<(String, String)> = HashSet::new();
    let allow_same_file = std::env::var("TS_PACK_INCLUDE_INTRA_FILE_CALLS")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let call_refs: Vec<CallRef> = call_sites
        .clone()
        .into_iter()
        .filter_map(|cs| {
            let caller_id = symbol_spans
                .iter()
                .filter(|(sb, eb, _)| *sb <= cs.start_byte && cs.start_byte < *eb)
                .min_by_key(|(sb, eb, _)| eb - sb)
                .map(|(_, _, id)| id.clone())
                .unwrap_or_else(|| file_id.clone());

            let dedupe_key = cs.qualified_callee.clone().unwrap_or_else(|| cs.callee.clone());
            if seen_calls.insert((caller_id.clone(), dedupe_key)) {
                let kind = if cs.qualified_callee.is_some() {
                    CallRefKind::Scoped
                } else if cs.receiver.is_some() {
                    CallRefKind::Member
                } else {
                    CallRefKind::Plain
                };
                Some(CallRef {
                    caller_id,
                    callee: cs.callee,
                    language: lang_name.to_string(),
                    caller_filepath: rel_path.clone(),
                    allow_same_file,
                    kind,
                    receiver_hint: cs.receiver,
                    qualified_hint: cs.qualified_callee,
                })
            } else {
                None
            }
        })
        .collect();

    if provenance::provenance_enabled() {
        for call_ref in &call_refs {
            if provenance::call_matches(
                &call_ref.caller_filepath,
                &call_ref.callee,
                call_ref.qualified_hint.as_deref(),
                call_ref.receiver_hint.as_deref(),
            ) {
                provenance::emit(
                    "parse",
                    "call_ref",
                    &[
                        ("caller_id", call_ref.caller_id.clone()),
                        ("caller_filepath", call_ref.caller_filepath.clone()),
                        ("callee", call_ref.callee.clone()),
                        (
                            "kind",
                            match call_ref.kind {
                                CallRefKind::Plain => "plain".to_string(),
                                CallRefKind::Member => "member".to_string(),
                                CallRefKind::Scoped => "scoped".to_string(),
                            },
                        ),
                        ("receiver_hint", call_ref.receiver_hint.clone().unwrap_or_default()),
                        ("qualified_hint", call_ref.qualified_hint.clone().unwrap_or_default()),
                    ],
                );
            }
        }
    }

    let (swift_extensions, swift_context, python_context, rust_context, go_context) = build_language_contexts(
        lang_name,
        &source,
        &file_id,
        rel_path,
        &symbol_spans,
        &call_sites,
        result.as_ref(),
    );

    let mut import_symbol_requests = Vec::new();
    let mut reexport_groups: HashMap<(String, bool), Vec<String>> = HashMap::new();
    let mut export_alias_requests = Vec::new();
    if let Some(result) = result.as_ref() {
        for imp in &result.imports {
            let import_id = format!("{}:import:{}:{}", pid, rel_path, imp.source);
            let stable_import_id = format!("{}:import:{}:{}", canonical_pid, rel_path, imp.source);
            imports.push(ImportNode {
                id: import_id.clone(),
                stable_id: stable_import_id,
                file_id: file_id.clone(),
                stable_file_id: stable_file_id.clone(),
                name: imp.source.clone(),
                source: imp.source.clone(),
                is_wildcard: imp.is_wildcard,
                project_id: Arc::clone(pid),
                filepath: rel_path.clone(),
            });
            import_rels.push(RelRow {
                parent: file_id.clone(),
                child: import_id,
            });

            if !imp.source.is_empty() {
                import_symbol_requests.push(ImportSymbolRequest {
                    src_id: file_id.clone(),
                    src_filepath: rel_path.clone(),
                    module: imp.source.clone(),
                    items: imp.items.clone(),
                });
            }
        }

        for export in &result.exports {
            if let Some(exported_as) = export.exported_as.as_ref().filter(|alias| !alias.is_empty()) {
                if let Some(item) = normalized_export_name(&export.name) {
                    export_alias_requests.push(ExportAliasRequest {
                        src_id: file_id.clone(),
                        src_filepath: rel_path.clone(),
                        module: export.source.clone(),
                        item,
                        exported_as: exported_as.clone(),
                    });
                } else if export.kind == ts_pack::ExportKind::ReExport && export.name.trim() == "*" {
                    export_alias_requests.push(ExportAliasRequest {
                        src_id: file_id.clone(),
                        src_filepath: rel_path.clone(),
                        module: export.source.clone(),
                        item: "*".to_string(),
                        exported_as: format!("{exported_as}.*"),
                    });
                }
            }
            if export.kind != ts_pack::ExportKind::ReExport {
                continue;
            }
            let Some(module) = export.source.as_ref().filter(|module| !module.is_empty()) else {
                continue;
            };
            let is_wildcard = export.name.trim() == "*";
            let key = (module.clone(), is_wildcard);
            let items = reexport_groups.entry(key).or_default();
            if !is_wildcard {
                if let Some(name) = normalized_export_name(&export.name) {
                    items.push(name);
                }
            }
        }
    }

    let reexport_symbol_requests = reexport_groups
        .into_iter()
        .map(|((module, is_wildcard), mut items)| {
            items.sort();
            items.dedup();
            ReExportSymbolRequest {
                src_id: file_id.clone(),
                src_filepath: rel_path.clone(),
                module,
                items,
                is_wildcard,
            }
        })
        .collect();

    Some(FileResult {
        language: lang_name.to_string(),
        file_node,
        file_facts,
        symbols,
        relations,
        imports,
        import_rels,
        call_refs,
        swift_extensions,
        swift_context,
        python_context,
        rust_context,
        go_context,
        clone_candidates,
        db_models: if is_backend { db_models } else { Vec::new() },
        external_urls,
        import_symbol_requests,
        reexport_symbol_requests,
        export_alias_requests,
        launch_calls,
        timings,
    })
}

pub(crate) fn parse_manifest_batch(
    batch: &[ManifestEntry],
    project_id: Arc<str>,
    clone_enrich_enabled: bool,
) -> Vec<FileResult> {
    let tag_query_bundles = Arc::new(tags::build_js_ts_query_bundles());
    let parse =
        |entry: &ManifestEntry| parse_entry(entry, &project_id, Some(tag_query_bundles.as_ref()), clone_enrich_enabled);
    if std::env::var("TS_PACK_SERIAL_PARSE").is_ok() {
        batch.iter().filter_map(parse).collect()
    } else {
        batch.par_iter().filter_map(parse).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clone_enrich_requested;

    use serde::Deserialize;
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("ts-pack-index-parse-{name}-{nanos}"))
    }

    #[test]
    fn swift_parse_guard_trips_on_pathological_paren_nesting() {
        let pathological = format!("func f() {{\nlet x = {}\n}}", "(".repeat(SWIFT_SAFE_PAREN_NESTING_LIMIT + 1));
        assert!(swift_requires_parse_guard(&pathological));
        assert!(!swift_requires_parse_guard("func f() { print(1) }"));
    }

    #[test]
    fn swift_parse_guard_returns_safe_structural_fallback() {
        let root = unique_temp_dir("swift-parse-guard-fallback");
        let rel_path = "test/Parse/structure_overflow_paren_exprs.swift";
        let abs_path = root.join(rel_path);
        fs::create_dir_all(abs_path.parent().unwrap()).unwrap();
        let source = format!(
            "func pathological() {{\nlet x = {}\nprint(x)\n}}\n",
            "(".repeat(SWIFT_SAFE_PAREN_NESTING_LIMIT + 1)
        );
        fs::write(&abs_path, source).unwrap();
        let manifest = vec![ManifestEntry {
            abs_path: abs_path.to_string_lossy().to_string(),
            rel_path: rel_path.to_string(),
            ext: "swift".to_string(),
            size: fs::metadata(&abs_path).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.file_node.filepath, rel_path);
        assert!(result.symbols.is_empty(), "guarded fallback should avoid native symbol extraction");
        assert!(result.relations.is_empty(), "guarded fallback should not emit structural relations");
        assert!(result.call_refs.is_empty(), "guarded fallback should not emit call refs");
        assert!(result.clone_candidates.is_empty(), "guarded fallback should not emit clone candidates");
    }

    #[derive(Debug, Deserialize)]
    struct GoldenExpectations {
        #[serde(default)]
        db_models: Vec<String>,
        #[serde(default)]
        excluded_db_models: Vec<String>,
        #[serde(default)]
        external_urls: Vec<String>,
        #[serde(default)]
        excluded_external_urls: Vec<String>,
        #[serde(default)]
        import_modules: Vec<String>,
        #[serde(default)]
        exported_symbols: Vec<String>,
        #[serde(default)]
        defined_symbols: Vec<String>,
        #[serde(default)]
        called_symbols: Vec<String>,
        #[serde(default)]
        required_called_symbols: Vec<String>,
    }

    fn fixture_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/ts-pack-index-goldens")
            .canonicalize()
            .unwrap()
    }

    fn copy_fixture_to_temp(group: &str, name: &str, ext: &str, rel_path: &str) -> (PathBuf, ManifestEntry) {
        let root = unique_temp_dir(&format!("{group}-{name}"));
        let rel = PathBuf::from(rel_path);
        fs::create_dir_all(root.join(rel.parent().unwrap())).unwrap();
        let fixture_path = fixture_root().join(group).join(format!("{name}.{ext}"));
        let file_abs = root.join(&rel);
        fs::copy(&fixture_path, &file_abs).unwrap();
        let manifest = ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: rel_path.to_string(),
            ext: ext.to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        };
        (root, manifest)
    }

    fn load_expectations(group: &str, name: &str) -> GoldenExpectations {
        let path = fixture_root().join(group).join(format!("{name}.expected.json"));
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
    }

    fn run_golden_fixture(group: &str, name: &str, ext: &str, rel_path: &str) {
        let (root, manifest_entry) = copy_fixture_to_temp(group, name, ext, rel_path);
        let manifest = vec![manifest_entry];
        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let result = &results[0];
        let expected = load_expectations(group, name);

        let actual_db_models: HashSet<_> = result.db_models.iter().cloned().collect();
        let expected_db_models: HashSet<_> = expected.db_models.iter().cloned().collect();
        assert_eq!(actual_db_models, expected_db_models, "db_models mismatch for {name}");

        for excluded in &expected.excluded_db_models {
            assert!(
                !actual_db_models.contains(excluded),
                "excluded db model {excluded} was present for {name}"
            );
        }

        let actual_imports: HashSet<_> = result
            .import_symbol_requests
            .iter()
            .map(|req| req.module.clone())
            .collect();
        let expected_imports: HashSet<_> = expected.import_modules.iter().cloned().collect();
        assert_eq!(actual_imports, expected_imports, "imports mismatch for {name}");

        let actual_exports: HashSet<_> = result
            .symbols
            .values()
            .flat_map(|items| items.iter())
            .filter(|sym| sym.is_exported)
            .map(|sym| sym.name.clone())
            .collect();
        let expected_exports: HashSet<_> = expected.exported_symbols.iter().cloned().collect();
        assert_eq!(actual_exports, expected_exports, "exported symbols mismatch for {name}");

        let actual_defined: HashSet<_> = result
            .symbols
            .values()
            .flat_map(|items| items.iter())
            .map(|sym| sym.name.clone())
            .collect();
        let expected_defined: HashSet<_> = expected.defined_symbols.iter().cloned().collect();
        if !expected_defined.is_empty() {
            assert_eq!(actual_defined, expected_defined, "defined symbols mismatch for {name}");
        }

        let actual_calls: HashSet<_> = result.call_refs.iter().map(|call| call.callee.clone()).collect();
        let expected_calls: HashSet<_> = expected.called_symbols.iter().cloned().collect();
        if !expected_calls.is_empty() {
            assert_eq!(actual_calls, expected_calls, "called symbols mismatch for {name}");
        }
        let required_calls: HashSet<_> = expected.required_called_symbols.iter().cloned().collect();
        if !required_calls.is_empty() {
            for call in required_calls {
                assert!(actual_calls.contains(&call), "missing required call {call} for {name}");
            }
        }

        let actual_external_urls: HashSet<_> = result.external_urls.iter().cloned().collect();
        let expected_external_urls: HashSet<_> = expected.external_urls.iter().cloned().collect();
        if !expected_external_urls.is_empty() {
            assert_eq!(
                actual_external_urls, expected_external_urls,
                "external urls mismatch for {name}"
            );
        }
        for excluded in &expected.excluded_external_urls {
            assert!(
                !actual_external_urls.contains(excluded),
                "excluded external url {excluded} was present for {name}"
            );
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn parses_python_manifest_batch_and_extracts_symbols() {
        let root = unique_temp_dir("python");
        fs::create_dir_all(root.join("pkg")).unwrap();
        let file_abs = root.join("pkg").join("main.py");
        fs::write(
            &file_abs,
            r#"
from .helpers import run

def main():
    run()
"#,
        )
        .unwrap();

        let manifest = vec![ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: "pkg/main.py".to_string(),
            ext: "py".to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.file_node.filepath, "pkg/main.py");
        assert!(!result.file_node.is_test);
        assert!(!result.symbols.is_empty());
        assert!(!result.call_refs.is_empty());
        assert_eq!(result.import_symbol_requests.len(), 1);
        assert_eq!(result.import_symbol_requests[0].module, ".helpers");
        assert!(result.reexport_symbol_requests.is_empty());
        assert!(result.export_alias_requests.is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn marks_test_like_files() {
        let root = unique_temp_dir("test-file");
        fs::create_dir_all(root.join("pkg")).unwrap();
        let file_abs = root.join("pkg").join("main.test.ts");
        fs::write(&file_abs, "export const x = 1;\n").unwrap();

        let manifest = vec![ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: "pkg/main.test.ts".to_string(),
            ext: "ts".to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        assert!(results[0].file_node.is_test);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn marks_go_exported_names_without_visibility_keyword() {
        let root = unique_temp_dir("go-exports");
        fs::create_dir_all(root.join("pkg")).unwrap();
        let file_abs = root.join("pkg").join("main.go");
        fs::write(
            &file_abs,
            "package pkg\n\nfunc PublicThing() {}\nfunc privateThing() {}\n",
        )
        .unwrap();

        let manifest = vec![ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: "pkg/main.go".to_string(),
            ext: "go".to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let funcs = results[0].symbols.get("Function").expect("functions");
        let public = funcs.iter().find(|sym| sym.name == "PublicThing").expect("PublicThing");
        let private = funcs
            .iter()
            .find(|sym| sym.name == "privateThing")
            .expect("privateThing");
        assert!(public.is_exported);
        assert!(!private.is_exported);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn normalizes_typescript_export_statement_names() {
        assert_eq!(
            normalized_export_name("export const registerFinanceAdminRoutes = (router: Router) => {"),
            Some("registerFinanceAdminRoutes".to_string())
        );
        assert_eq!(
            normalized_export_name("export function registerPublicRoutes("),
            Some("registerPublicRoutes".to_string())
        );
        assert_eq!(
            normalized_export_name("export class RouteContextBuilder {"),
            Some("RouteContextBuilder".to_string())
        );
        assert_eq!(normalized_export_name("RouteContext"), Some("RouteContext".to_string()));
        assert_eq!(normalized_export_name(""), None);
    }

    #[test]
    fn marks_typescript_exported_const_arrow_functions() {
        let root = unique_temp_dir("ts-export-const");
        fs::create_dir_all(root.join("pkg")).unwrap();
        let file_abs = root.join("pkg").join("routes.ts");
        fs::write(
            &file_abs,
            r#"
import type { Router } from "express";

export const registerFinanceAdminRoutes = (router: Router) => {
  router.get("/health", (_req, res) => {
    res.json({ ok: true });
  });
};
"#,
        )
        .unwrap();

        let manifest = vec![ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: "pkg/routes.ts".to_string(),
            ext: "ts".to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let funcs = results[0].symbols.get("Function").expect("functions");
        let exported = funcs
            .iter()
            .find(|sym| sym.name == "registerFinanceAdminRoutes")
            .expect("registerFinanceAdminRoutes");
        assert!(exported.is_exported);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn groups_typescript_reexport_requests() {
        let root = unique_temp_dir("ts-reexports");
        fs::create_dir_all(root.join("pkg")).unwrap();
        let file_abs = root.join("pkg").join("index.ts");
        fs::write(
            &file_abs,
            r#"
export { Foo, Bar as RenamedBar } from "./types";
export type { Baz } from "./types";
export * from "./helpers";
export * as routes from "./routes";
"#,
        )
        .unwrap();

        let manifest = vec![ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: "pkg/index.ts".to_string(),
            ext: "ts".to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.reexport_symbol_requests.len(), 3);

        let named = result
            .reexport_symbol_requests
            .iter()
            .find(|req| req.module == "./types")
            .expect("named reexport request");
        assert!(!named.is_wildcard);
        assert_eq!(
            named.items,
            vec!["Bar".to_string(), "Baz".to_string(), "Foo".to_string()]
        );

        let wildcard = result
            .reexport_symbol_requests
            .iter()
            .find(|req| req.module == "./helpers")
            .expect("wildcard reexport request");
        assert!(wildcard.is_wildcard);
        assert!(wildcard.items.is_empty());

        let namespace = result
            .reexport_symbol_requests
            .iter()
            .find(|req| req.module == "./routes")
            .expect("namespace reexport request");
        assert!(namespace.is_wildcard);
        assert!(namespace.items.is_empty());

        let alias_requests: Vec<_> = result.export_alias_requests.iter().collect();
        assert_eq!(alias_requests.len(), 2);

        let renamed = alias_requests
            .iter()
            .find(|req| req.module.as_deref() == Some("./types"))
            .expect("named alias request");
        assert_eq!(renamed.item, "Bar");
        assert_eq!(renamed.exported_as, "RenamedBar");

        let namespace_alias = alias_requests
            .iter()
            .find(|req| req.module.as_deref() == Some("./routes"))
            .expect("namespace alias request");
        assert_eq!(namespace_alias.item, "*");
        assert_eq!(namespace_alias.exported_as, "routes.*");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn golden_rental_accounting_sync_history_service() {
        run_golden_fixture(
            "rental",
            "accounting_sync_history_service",
            "ts",
            "src/services/accounting_sync_history_service.ts",
        );
    }

    #[test]
    fn golden_rental_tenant_credit_service() {
        run_golden_fixture(
            "rental",
            "tenant_credit_service",
            "ts",
            "src/services/tenant_credit_service.ts",
        );
    }

    #[test]
    fn golden_rest_proxy_semantic_helpers() {
        run_golden_fixture(
            "rest_proxy",
            "semantic_helpers",
            "py",
            "tools/brain/search/semantic_helpers.py",
        );
    }

    #[test]
    fn golden_rental_context() {
        run_golden_fixture("rental", "context", "ts", "src/api/routes/context.ts");
    }

    #[test]
    fn golden_rental_quickbooks_external() {
        run_golden_fixture(
            "rental",
            "quickbooks_external",
            "ts",
            "src/services/QuickBooksService.ts",
        );
    }

    #[test]
    fn golden_rest_proxy_policy() {
        run_golden_fixture("rest_proxy", "policy", "py", "tools/brain/docs/policy.py");
    }

    #[test]
    fn golden_ra_storage_server() {
        run_golden_fixture("raStorage", "server", "go", "test_parse/data/server.go");
    }

    #[test]
    fn golden_ra_storage_sample_swift() {
        run_golden_fixture("raStorage", "sample", "swift", "test_parse/data/sample.swift");
    }

    #[test]
    fn synthesizes_swift_main_symbol_for_top_level_entry_calls() {
        let root = unique_temp_dir("swift-main-entry");
        fs::create_dir_all(root.join("Examples/App")).unwrap();
        let file_abs = root.join("Examples/App/main.swift");
        fs::write(
            &file_abs,
            r#"
import Foundation

let graph = DynamicGraph()
graph.trackGrad(tensor)
"#,
        )
        .unwrap();

        let manifest = vec![ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: "Examples/App/main.swift".to_string(),
            ext: "swift".to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let result = &results[0];
        let funcs = result.symbols.get("Function").expect("functions");
        let main = funcs.iter().find(|sym| sym.name == "main").expect("synthetic main");
        assert_eq!(main.filepath, "Examples/App/main.swift");
        assert!(main.start_line >= 3);
        assert!(
            result
                .call_refs
                .iter()
                .any(|call| { call.caller_id == main.id && call.callee == "DynamicGraph" }),
            "expected top-level constructor call to belong to synthetic main symbol"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn swift_extensions_get_distinct_qualified_names() {
        let root = unique_temp_dir("swift-extension-identity");
        fs::create_dir_all(root.join("Sources/App")).unwrap();
        let file_abs = root.join("Sources/App/EventLoop.swift");
        fs::write(
            &file_abs,
            r#"
public protocol EventLoop {
    func makePromise()
}

extension EventLoop {
    public func makePromise() {}
}
"#,
        )
        .unwrap();

        let manifest = vec![ManifestEntry {
            abs_path: file_abs.to_string_lossy().to_string(),
            rel_path: "Sources/App/EventLoop.swift".to_string(),
            ext: "swift".to_string(),
            size: fs::metadata(&file_abs).unwrap().len(),
        }];

        let results = parse_manifest_batch(&manifest, Arc::from("proj"), clone_enrich_requested());
        assert_eq!(results.len(), 1);
        let result = &results[0];
        let protocols = result.symbols.get("Protocol").expect("protocols");
        let extensions = result.symbols.get("Extension").expect("extensions");
        let protocol = protocols.iter().find(|sym| sym.name == "EventLoop").expect("protocol");
        let extension = extensions
            .iter()
            .find(|sym| sym.name == "EventLoop")
            .expect("extension");

        assert_eq!(protocol.qualified_name.as_deref(), Some("EventLoop"));
        assert_ne!(extension.qualified_name.as_deref(), Some("EventLoop"));
        assert!(
            extension
                .qualified_name
                .as_deref()
                .unwrap_or_default()
                .starts_with("extension EventLoop@Sources/App/EventLoop.swift:"),
            "extension qualified name should be declaration-distinct"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn golden_ts_pack_php_validator() {
        run_golden_fixture(
            "tree-sitter-language-pack",
            "php_validator",
            "rs",
            "tools/snippet-runner/src/validators/php.rs",
        );
    }
}
