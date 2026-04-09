use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use rayon::prelude::*;
use tree_sitter_language_pack as ts_pack;

use crate::duplicate;
use crate::pathing;
use crate::swift;
use crate::tags;
use crate::{
    CloneCandidate, FileNode, ImportNode, ImportSymbolRequest, MAX_FILE_BYTES, ManifestEntry, PythonFileContext,
    RelRow, SwiftFileContext, SymbolCallRow, SymbolNode, WINNOW_LARGE_K, WINNOW_LARGE_W, WINNOW_MEDIUM_K,
    WINNOW_MEDIUM_W, WINNOW_MIN_FINGERPRINTS, WINNOW_MIN_TOKENS, WINNOW_SMALL_K, WINNOW_SMALL_TOKEN_THRESHOLD,
    WINNOW_SMALL_W,
};

pub(crate) struct FileResult {
    pub(crate) file_node: FileNode,
    pub(crate) file_facts: ts_pack::FileFacts,
    pub(crate) symbols: HashMap<&'static str, Vec<SymbolNode>>,
    pub(crate) relations: Vec<RelRow>,
    pub(crate) imports: Vec<ImportNode>,
    pub(crate) import_rels: Vec<RelRow>,
    pub(crate) symbol_calls: Vec<SymbolCallRow>,
    pub(crate) swift_extensions: Option<HashMap<String, HashSet<String>>>,
    pub(crate) swift_context: Option<SwiftFileContext>,
    pub(crate) python_context: Option<PythonFileContext>,
    pub(crate) clone_candidates: Vec<CloneCandidate>,
    pub(crate) db_models: Vec<String>,
    pub(crate) external_urls: Vec<String>,
    pub(crate) import_symbol_requests: Vec<ImportSymbolRequest>,
    pub(crate) launch_calls: Vec<String>,
}

fn is_apple_fact_file(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    normalized.ends_with(".xcodeproj/project.pbxproj")
        || normalized.ends_with(".xcworkspace/contents.xcworkspacedata")
        || normalized.ends_with(".xcscheme")
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

    let visibility = item.visibility.as_deref().unwrap_or("").trim().to_lowercase();
    let name_is_public = match language {
        "go" => name.chars().next().map(|ch| ch.is_uppercase()).unwrap_or(false),
        _ => false,
    };
    let is_exported = matches!(visibility.as_str(), "public" | "open" | "pub")
        || visibility.starts_with("pub(")
        || exported_names.contains(name)
        || name_is_public;

    symbols.entry(label).or_default().push(SymbolNode {
        id: node_id.clone(),
        name: name.to_string(),
        kind: format!("{:?}", item.kind),
        qualified_name: item.qualified_name.clone(),
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

fn parse_entry(entry: &ManifestEntry, pid: &Arc<str>) -> Option<FileResult> {
    let rel_basename = entry.rel_path.rsplit('/').next().unwrap_or(entry.rel_path.as_str());
    if matches!(rel_basename, ".gitignore" | ".indexignore" | ".env" | ".env.example") {
        return None;
    }

    let lang_name = if is_apple_fact_file(&entry.rel_path) {
        "text"
    } else if entry.ext == "svg" {
        "xml"
    } else {
        match ts_pack::detect_language_from_extension(&entry.ext) {
            Some(lang) => lang,
            None => {
                eprintln!(
                    "[ts-pack-index] detect_language_from_extension failed: {}",
                    entry.rel_path
                );
                return None;
            }
        }
    };
    if lang_name != "text" && !ts_pack::has_language(lang_name) {
        if let Err(err) = ts_pack::download(&[lang_name]) {
            eprintln!("[ts-pack-index] download failed: {lang} ({err})", lang = lang_name);
            return None;
        }
    }

    let rel_path = &entry.rel_path;
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

    let file_facts = ts_pack::extract_file_facts(&source, lang_name, Some(rel_path)).unwrap_or_default();

    let result = if lang_name == "text" {
        None
    } else {
        let proc_config = ts_pack::ProcessConfig::new(lang_name).all();
        match ts_pack::process(&source, &proc_config) {
            Ok(result) => Some(result),
            Err(err) => {
                eprintln!("[ts-pack-index] process failed: {} ({})", entry.rel_path, err);
                return None;
            }
        }
    };

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
    let file_id = format!("{}:file:{}", pid, rel_path);

    let file_node = FileNode {
        id: file_id.clone(),
        name: file_name,
        filepath: rel_path.clone(),
        project_id: Arc::clone(pid),
        is_test: is_test_like_path(rel_path),
    };

    let mut symbols: HashMap<&'static str, Vec<SymbolNode>> = HashMap::new();
    let mut relations = Vec::new();
    let mut imports = Vec::new();
    let mut import_rels = Vec::new();
    let mut swift_extensions: Option<HashMap<String, HashSet<String>>> = None;
    let mut swift_context: Option<SwiftFileContext> = None;
    let mut python_context: Option<PythonFileContext> = None;

    let mut exported_names: HashSet<String> = result
        .as_ref()
        .map(|r| r.exports.iter().map(|e| e.name.clone()).collect())
        .unwrap_or_default();
    let tags_result = if lang_name == "text" {
        None
    } else {
        ts_pack::parse_string(lang_name, source.as_bytes())
            .ok()
            .and_then(|tree| tags::run_tags(lang_name, &tree, source.as_bytes()))
    };

    let (tag_exported, raw_call_sites, tag_db_models, external_calls, const_strings, launch_calls) = match tags_result {
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
    let call_sites = raw_call_sites;
    let mut db_models: HashSet<String> = tag_db_models;
    for item in &file_facts.db_models {
        db_models.insert(item.model.clone());
    }
    let db_models = db_models.into_iter().collect::<Vec<_>>();
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
    }

    let symbol_spans: Vec<(usize, usize, String)> = symbols
        .values()
        .flat_map(|v| v.iter())
        .map(|s| (s.start_byte, s.end_byte, s.id.clone()))
        .collect();

    let clone_candidates = if std::env::var("LM_PROXY_CLONE_ENRICH")
        .ok()
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
    {
        build_clone_candidates(&symbols, &source)
    } else {
        Vec::new()
    };

    let mut seen_calls: HashSet<(String, String)> = HashSet::new();
    let allow_same_file = std::env::var("TS_PACK_INCLUDE_INTRA_FILE_CALLS")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let symbol_calls: Vec<SymbolCallRow> = call_sites
        .clone()
        .into_iter()
        .filter_map(|cs| {
            let caller_id = symbol_spans
                .iter()
                .filter(|(sb, eb, _)| *sb <= cs.start_byte && cs.start_byte < *eb)
                .min_by_key(|(sb, eb, _)| eb - sb)
                .map(|(_, _, id)| id.clone())
                .unwrap_or_else(|| file_id.clone());

            if seen_calls.insert((caller_id.clone(), cs.callee.clone())) {
                Some(SymbolCallRow {
                    caller_id,
                    callee: cs.callee,
                    project_id: Arc::clone(pid),
                    caller_filepath: rel_path.clone(),
                    allow_same_file,
                })
            } else {
                None
            }
        })
        .collect();

    if lang_name == "swift" {
        let mut ext_map: HashMap<String, HashSet<String>> = HashMap::new();
        if let Some(result) = result.as_ref() {
            swift::collect_swift_extensions(&result.structure, &mut ext_map);
        }
        if !ext_map.is_empty() {
            swift_extensions = Some(ext_map);
        }

        let mut ext_spans = Vec::new();
        if let Some(result) = result.as_ref() {
            swift::collect_swift_extension_spans(&result.structure, &mut ext_spans);
        }

        let mut type_spans = Vec::new();
        if let Some(result) = result.as_ref() {
            swift::collect_swift_type_spans(&result.structure, &mut type_spans);
        }

        let var_types = swift::parse_swift_var_types(&source);
        if !var_types.is_empty() {
            swift_context = Some(SwiftFileContext {
                file_id: file_id.clone(),
                filepath: rel_path.clone(),
                symbol_spans: symbol_spans.clone(),
                extension_spans: ext_spans.clone(),
                type_spans: type_spans.clone(),
                call_sites: call_sites.clone(),
                var_types,
            });
        } else if !call_sites.is_empty() {
            swift_context = Some(SwiftFileContext {
                file_id: file_id.clone(),
                filepath: rel_path.clone(),
                symbol_spans: symbol_spans.clone(),
                extension_spans: ext_spans.clone(),
                type_spans: type_spans.clone(),
                call_sites: call_sites.clone(),
                var_types: HashMap::new(),
            });
        }
    }

    if lang_name == "python" {
        let mut module_aliases: HashMap<String, String> = HashMap::new();
        if let Some(result) = result.as_ref() {
            for imp in &result.imports {
                if imp.alias.is_none() || !imp.items.is_empty() {
                    continue;
                }
                let Some(alias) = imp.alias.as_ref() else {
                    continue;
                };
                if alias.is_empty() || imp.source.is_empty() {
                    continue;
                }
                module_aliases.insert(alias.clone(), imp.source.clone());
            }
        }
        if !call_sites.is_empty() && !module_aliases.is_empty() {
            python_context = Some(PythonFileContext {
                file_id: file_id.clone(),
                filepath: rel_path.clone(),
                symbol_spans: symbol_spans.clone(),
                call_sites: call_sites.clone(),
                module_aliases,
            });
        }
    }

    let mut import_symbol_requests = Vec::new();
    if let Some(result) = result.as_ref() {
        for imp in &result.imports {
            let import_id = format!("{}:import:{}:{}", pid, rel_path, imp.source);
            imports.push(ImportNode {
                id: import_id.clone(),
                file_id: file_id.clone(),
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
    }

    Some(FileResult {
        file_node,
        file_facts,
        symbols,
        relations,
        imports,
        import_rels,
        symbol_calls,
        swift_extensions,
        swift_context,
        python_context,
        clone_candidates,
        db_models: if is_backend { db_models } else { Vec::new() },
        external_urls,
        import_symbol_requests,
        launch_calls,
    })
}

pub(crate) fn parse_manifest_batch(batch: &[ManifestEntry], project_id: Arc<str>) -> Vec<FileResult> {
    let parse = |entry: &ManifestEntry| parse_entry(entry, &project_id);
    if std::env::var("TS_PACK_SERIAL_PARSE").is_ok() {
        batch.iter().filter_map(parse).collect()
    } else {
        batch.par_iter().filter_map(parse).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("ts-pack-index-parse-{name}-{nanos}"))
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

        let results = parse_manifest_batch(&manifest, Arc::from("proj"));
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.file_node.filepath, "pkg/main.py");
        assert!(!result.file_node.is_test);
        assert!(!result.symbols.is_empty());
        assert!(!result.symbol_calls.is_empty());
        assert_eq!(result.import_symbol_requests.len(), 1);
        assert_eq!(result.import_symbol_requests[0].module, ".helpers");

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

        let results = parse_manifest_batch(&manifest, Arc::from("proj"));
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

        let results = parse_manifest_batch(&manifest, Arc::from("proj"));
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
}
