use std::collections::{HashMap, HashSet};

use crate::pathing;
use crate::{CallRef, CallRefKind, ExternalSymbolEdgeRow, ExternalSymbolNode, ImportSymbolRequest, SymbolCallRow};

pub(crate) struct CallResolutionContext<'a> {
    pub(crate) callable_symbols_by_name: &'a HashMap<String, Vec<(String, String)>>,
    pub(crate) qualified_callable_symbols: &'a [(String, String, String)],
    pub(crate) caller_qualified_symbols_by_id: &'a HashMap<String, String>,
    pub(crate) symbols_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) go_import_aliases_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) go_var_types_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) rust_var_types_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) python_var_types_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) python_module_aliases_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) python_imported_symbol_modules_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) imported_target_files_by_src: &'a HashMap<String, HashSet<String>>,
    pub(crate) import_symbol_requests: &'a [ImportSymbolRequest],
    pub(crate) exported_symbols_by_file: &'a HashMap<String, Vec<String>>,
    pub(crate) files_set: &'a HashSet<String>,
    pub(crate) rust_local_module_roots_by_src_root: &'a HashMap<String, HashSet<String>>,
}

pub(crate) enum CallResolution {
    ResolvedInternal(String, &'static str),
    Filtered(&'static str, Option<ExternalSymbolResolution>),
    Unresolved(&'static str),
}

pub(crate) struct ExternalSymbolResolution {
    pub(crate) name: String,
    pub(crate) qualified_name: String,
    pub(crate) language: String,
}

pub(crate) struct CallResolutionOutputs {
    pub(crate) symbol_call_rows: Vec<SymbolCallRow>,
    pub(crate) external_symbol_nodes: Vec<ExternalSymbolNode>,
    pub(crate) external_symbol_edges: Vec<ExternalSymbolEdgeRow>,
    pub(crate) resolved_call_rows: usize,
    pub(crate) unresolved_internal_call_rows: usize,
    pub(crate) resolution_stage_counts: HashMap<&'static str, usize>,
    pub(crate) filtered_stage_counts: HashMap<&'static str, usize>,
    pub(crate) unresolved_name_counts: HashMap<String, usize>,
    pub(crate) unresolved_bucket_counts: HashMap<String, usize>,
    pub(crate) unresolved_bucket_samples: HashMap<String, Vec<String>>,
    pub(crate) unresolved_rust_plain_attribution: HashMap<(String, String), usize>,
    pub(crate) skipped_external_call_rows: usize,
}

fn strip_generic_segments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut depth = 0usize;
    for ch in text.chars() {
        match ch {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out
}

fn rust_path_variants(text: &str) -> Vec<String> {
    let mut variants = Vec::new();
    variants.push(text.to_string());

    let mut current = text;
    while let Some(rest) = current
        .strip_prefix("crate::")
        .or_else(|| current.strip_prefix("self::"))
        .or_else(|| current.strip_prefix("super::"))
    {
        variants.push(rest.to_string());
        current = rest;
    }

    variants
}

pub(crate) fn normalize_qualified_variants(text: &str, language: &str) -> Vec<String> {
    let compact = text.trim().replace(":: <", "::<").replace(' ', "");
    let compact = compact.trim_start_matches('&').trim_start_matches("mut ");
    let no_generics = strip_generic_segments(compact);
    let base = if no_generics.is_empty() {
        compact.to_string()
    } else {
        no_generics
    };

    let mut variants = match language {
        "rust" => rust_path_variants(&base),
        _ => vec![base],
    };
    variants.sort();
    variants.dedup();
    variants
}

pub(crate) fn normalize_qualified_hint(text: &str) -> String {
    normalize_qualified_variants(text, "")
        .into_iter()
        .next()
        .unwrap_or_default()
}

fn receiver_qualified_candidates(call_ref: &CallRef) -> Vec<String> {
    let Some(receiver) = call_ref.receiver_hint.as_ref() else {
        return Vec::new();
    };
    let receiver = receiver.trim();
    if receiver.is_empty() {
        return Vec::new();
    }

    let mut candidates = Vec::new();
    match call_ref.language.as_str() {
        "rust" => {
            candidates.push(format!("{receiver}::{}", call_ref.callee));
            candidates.push(format!("{receiver}.{}", call_ref.callee));
        }
        "go" | "javascript" | "typescript" | "tsx" | "jsx" | "python" => {
            candidates.push(format!("{receiver}.{}", call_ref.callee));
        }
        _ => {
            candidates.push(format!("{receiver}.{}", call_ref.callee));
            candidates.push(format!("{receiver}::{}", call_ref.callee));
        }
    }
    candidates
        .into_iter()
        .flat_map(|text| normalize_qualified_variants(&text, &call_ref.language))
        .collect()
}

fn self_qualified_candidates(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Vec<String> {
    if call_ref.language != "rust" {
        return Vec::new();
    }
    let Some(qualified_hint) = call_ref.qualified_hint.as_deref() else {
        return Vec::new();
    };
    let Some(rest) = qualified_hint.strip_prefix("Self::") else {
        return Vec::new();
    };
    let Some(caller_qualified) = ctx.caller_qualified_symbols_by_id.get(&call_ref.caller_id) else {
        return Vec::new();
    };

    let mut candidates = Vec::new();
    let mut parts: Vec<&str> = caller_qualified
        .split("::")
        .filter(|segment| !segment.is_empty())
        .collect();
    if parts.len() >= 2 {
        parts.pop();
        let type_path = parts.join("::");
        if !type_path.is_empty() {
            candidates.push(format!("{type_path}::{rest}"));
            if let Some(short_type) = type_path.rsplit("::").next() {
                candidates.push(format!("{short_type}::{rest}"));
            }
        }
    }

    candidates
        .into_iter()
        .flat_map(|text| normalize_qualified_variants(&text, &call_ref.language))
        .collect()
}

fn resolve_by_global_unique(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    let candidates = ctx.callable_symbols_by_name.get(&call_ref.callee)?;
    let mut matches = candidates
        .iter()
        .filter(|(_, filepath)| call_ref.allow_same_file || filepath != &call_ref.caller_filepath)
        .map(|(id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_same_file(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    if !call_ref.allow_same_file {
        return None;
    }
    ctx.symbols_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|sym_map| sym_map.get(&call_ref.callee).cloned())
}

fn resolve_by_import_symbol_request(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    for req in ctx
        .import_symbol_requests
        .iter()
        .filter(|req| req.src_filepath == call_ref.caller_filepath)
    {
        let target_fp = pathing::resolve_module_path(&req.src_filepath, &req.module, ctx.files_set);
        let sym_map = target_fp.as_ref().and_then(|fp| ctx.symbols_by_file.get(fp));
        if req.items.is_empty() {
            if call_ref.language == "rust"
                && let Some((module_path, imported_name)) = req.module.rsplit_once("::")
                && imported_name == call_ref.callee
            {
                let imported_target = pathing::resolve_module_path(&req.src_filepath, module_path, ctx.files_set);
                let imported_sym_map = imported_target.as_ref().and_then(|fp| ctx.symbols_by_file.get(fp));
                if let Some(imported_sym_map) = imported_sym_map
                    && let Some(sym_id) = imported_sym_map.get(imported_name)
                {
                    return Some(sym_id.clone());
                }
            }
            if let Some(fp) = target_fp.as_ref() {
                if let Some(sym_map) = sym_map {
                    if let Some(sym_id) = sym_map.get(&call_ref.callee) {
                        return Some(sym_id.clone());
                    }
                } else if let Some(exported) = ctx.exported_symbols_by_file.get(fp) {
                    if let Some(sym_id) = exported.first() {
                        return Some(sym_id.clone());
                    }
                }
            }
            continue;
        }
        if !req
            .items
            .iter()
            .any(|item| pathing::clean_import_name(item) == call_ref.callee)
        {
            continue;
        }
        if let Some(sym_map) = sym_map {
            if let Some(sym_id) = sym_map.get(&call_ref.callee) {
                return Some(sym_id.clone());
            }
        }
    }
    None
}

fn resolve_by_imported_target_unique(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    let candidates = ctx.callable_symbols_by_name.get(&call_ref.callee)?;
    let imported_files = ctx.imported_target_files_by_src.get(&call_ref.caller_filepath)?;
    let mut matches = candidates
        .iter()
        .filter(|(_, filepath)| imported_files.contains(filepath))
        .map(|(id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_python_module_receiver(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return None;
    }
    let receiver = call_ref.receiver_hint.as_ref()?;
    let module = ctx
        .python_module_aliases_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(receiver))?;
    let target_fp = pathing::resolve_module_path(&call_ref.caller_filepath, module, ctx.files_set)?;
    if !call_ref.allow_same_file && target_fp == call_ref.caller_filepath {
        return None;
    }
    ctx.symbols_by_file
        .get(&target_fp)
        .and_then(|sym_map| sym_map.get(&call_ref.callee).cloned())
}

fn resolve_by_python_imported_symbol(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Plain) {
        return None;
    }
    let module = ctx
        .python_imported_symbol_modules_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(&call_ref.callee))?;
    let target_fp = pathing::resolve_module_path(&call_ref.caller_filepath, module, ctx.files_set)?;
    if !call_ref.allow_same_file && target_fp == call_ref.caller_filepath {
        return None;
    }
    ctx.symbols_by_file
        .get(&target_fp)
        .and_then(|sym_map| sym_map.get(&call_ref.callee).cloned())
}

fn resolve_by_python_receiver_type(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return None;
    }
    let receiver = call_ref.receiver_hint.as_deref()?;
    let receiver_type = ctx
        .python_var_types_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(receiver))?;
    let normalized_type = receiver_type.rsplit('.').next().unwrap_or(receiver_type).trim();
    if normalized_type.is_empty() {
        return None;
    }
    let exact = format!("{normalized_type}.{}", call_ref.callee);
    let suffix = format!(".{normalized_type}.{}", call_ref.callee);
    let mut matches = ctx
        .qualified_callable_symbols
        .iter()
        .filter(|(qualified_name, _, filepath)| {
            (qualified_name == &exact || qualified_name.ends_with(&suffix))
                && (call_ref.allow_same_file || filepath != &call_ref.caller_filepath)
        })
        .map(|(_, id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_go_import_receiver(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    if call_ref.language != "go" || !matches!(call_ref.kind, CallRefKind::Scoped) {
        return None;
    }
    let receiver = call_ref.receiver_hint.as_deref()?;
    let module = ctx
        .go_import_aliases_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(receiver))?;

    let mut module_prefixes = Vec::new();
    let module = module.trim().trim_matches('"');
    if !module.is_empty() {
        module_prefixes.push(module.to_string());
        if let Some(idx) = module.find("packages/") {
            module_prefixes.push(module[idx..].to_string());
        }
        let parts: Vec<&str> = module.split('/').filter(|part| !part.is_empty()).collect();
        if parts.len() >= 2 {
            module_prefixes.push(format!("{}/{}", parts[parts.len() - 2], parts[parts.len() - 1]));
        }
        if let Some(last) = parts.last() {
            module_prefixes.push((*last).to_string());
        }
    }
    module_prefixes.sort();
    module_prefixes.dedup();

    let mut matches = ctx
        .callable_symbols_by_name
        .get(&call_ref.callee)?
        .iter()
        .filter(|(_, filepath)| {
            module_prefixes.iter().any(|prefix| {
                filepath.starts_with(prefix)
                    || filepath.contains(&format!("/{prefix}/"))
                    || filepath.contains(&format!("/{prefix}."))
            })
        })
        .filter(|(_, filepath)| call_ref.allow_same_file || *filepath != call_ref.caller_filepath)
        .map(|(id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_go_receiver_type(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    if call_ref.language != "go" || !matches!(call_ref.kind, CallRefKind::Scoped) {
        return None;
    }
    let receiver = call_ref.receiver_hint.as_deref()?;
    let receiver_type = ctx
        .go_var_types_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(receiver))?;
    let normalized_type = receiver_type
        .split('.')
        .next_back()
        .unwrap_or(receiver_type)
        .trim_start_matches('*')
        .trim();
    if normalized_type.is_empty() {
        return None;
    }
    let suffix = format!(".{normalized_type}.{}", call_ref.callee);
    let exact = format!("{normalized_type}.{}", call_ref.callee);
    let mut matches = ctx
        .qualified_callable_symbols
        .iter()
        .filter(|(qualified_name, _, filepath)| {
            (qualified_name == &exact || qualified_name.ends_with(&suffix))
                && (call_ref.allow_same_file || filepath != &call_ref.caller_filepath)
        })
        .map(|(_, id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_rust_receiver_type(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    if call_ref.language != "rust" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return None;
    }
    let receiver = call_ref.receiver_hint.as_deref()?;
    let receiver_type = ctx
        .rust_var_types_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(receiver))?;
    let normalized_type = receiver_type
        .rsplit("::")
        .next()
        .unwrap_or(receiver_type)
        .trim_start_matches('*')
        .trim();
    if normalized_type.is_empty() {
        return None;
    }
    let rust_exact = format!("{normalized_type}::{}", call_ref.callee);
    let dot_exact = format!("{normalized_type}.{}", call_ref.callee);
    let rust_suffix = format!("::{normalized_type}::{}", call_ref.callee);
    let dot_suffix = format!(".{normalized_type}.{}", call_ref.callee);
    let mut matches = ctx
        .qualified_callable_symbols
        .iter()
        .filter(|(qualified_name, _, filepath)| {
            (qualified_name == &rust_exact
                || qualified_name == &dot_exact
                || qualified_name.ends_with(&rust_suffix)
                || qualified_name.ends_with(&dot_suffix))
                && (call_ref.allow_same_file || filepath != &call_ref.caller_filepath)
        })
        .map(|(_, id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_local_directory_unique(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    let candidates = ctx.callable_symbols_by_name.get(&call_ref.callee)?;
    let caller_dir = std::path::Path::new(&call_ref.caller_filepath)
        .parent()
        .and_then(|p| p.to_str())
        .map(|s| s.trim_end_matches('/').to_string())?;
    let dir_prefix = if caller_dir.is_empty() {
        String::new()
    } else {
        format!("{caller_dir}/")
    };
    let mut matches = candidates
        .iter()
        .filter(|(_, filepath)| call_ref.allow_same_file || *filepath != call_ref.caller_filepath)
        .filter(|(_, filepath)| dir_prefix.is_empty() || filepath.starts_with(&dir_prefix))
        .map(|(id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_qualified_hint(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    let hint = call_ref.qualified_hint.as_ref()?;
    let normalized = normalize_qualified_variants(hint, &call_ref.language);
    let mut matches = ctx
        .qualified_callable_symbols
        .iter()
        .filter(|(qualified_name, _, filepath)| {
            normalized.iter().any(|candidate| {
                (qualified_name == candidate
                    || qualified_name.ends_with(candidate)
                    || candidate.ends_with(qualified_name.as_str()))
                    && (call_ref.allow_same_file || filepath != &call_ref.caller_filepath)
            })
        })
        .map(|(_, id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_self_qualified(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    let candidates = self_qualified_candidates(ctx, call_ref);
    if candidates.is_empty() {
        return None;
    }

    let mut matches = ctx
        .qualified_callable_symbols
        .iter()
        .filter(|(qualified_name, _, filepath)| {
            candidates.iter().any(|candidate| {
                (qualified_name == candidate
                    || qualified_name.ends_with(candidate)
                    || candidate.ends_with(qualified_name.as_str()))
                    && (call_ref.allow_same_file || filepath != &call_ref.caller_filepath)
            })
        })
        .map(|(_, id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn resolve_by_receiver_qualified(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> Option<String> {
    let candidates = receiver_qualified_candidates(call_ref);
    if candidates.is_empty() {
        return None;
    }

    let mut matches = ctx
        .qualified_callable_symbols
        .iter()
        .filter(|(qualified_name, _, filepath)| {
            candidates.iter().any(|candidate| {
                (qualified_name == candidate
                    || qualified_name.ends_with(candidate)
                    || candidate.ends_with(qualified_name.as_str()))
                    && (call_ref.allow_same_file || filepath != &call_ref.caller_filepath)
            })
        })
        .map(|(_, id, _)| id.clone());
    let first = matches.next();
    let second = matches.next();
    if second.is_none() { first } else { None }
}

fn rust_source_root(filepath: &str) -> Option<String> {
    let marker = "/src/";
    let idx = filepath.find(marker)?;
    Some(filepath[..idx + marker.len() - 1].to_string())
}

fn rust_qualified_root(text: &str) -> Option<&str> {
    let normalized = text.trim();
    if normalized.is_empty() {
        return None;
    }
    normalized.split("::").find(|segment| !segment.is_empty())
}

fn is_clearly_external_rust_scoped_call(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> bool {
    if call_ref.language != "rust" || !matches!(call_ref.kind, CallRefKind::Scoped) {
        return false;
    }
    let Some(hint) = call_ref.qualified_hint.as_deref() else {
        return false;
    };
    let Some(root) = rust_qualified_root(hint) else {
        return false;
    };
    if matches!(root, "crate" | "self" | "super" | "Self") {
        return false;
    }
    if let Some(src_root) = rust_source_root(&call_ref.caller_filepath) {
        return match ctx.rust_local_module_roots_by_src_root.get(&src_root) {
            Some(local_roots) => !local_roots.contains(root),
            None => true,
        };
    }
    !ctx.rust_local_module_roots_by_src_root
        .values()
        .any(|local_roots| local_roots.contains(root))
}

fn import_module_alias(module: &str) -> Option<&str> {
    module.rsplit('/').find(|segment| !segment.is_empty())
}

fn is_clearly_external_go_scoped_call(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> bool {
    if call_ref.language != "go" || !matches!(call_ref.kind, CallRefKind::Scoped) {
        return false;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    ctx.import_symbol_requests
        .iter()
        .filter(|req| req.src_filepath == call_ref.caller_filepath)
        .filter(|req| req.items.is_empty())
        .filter(|req| import_module_alias(&req.module) == Some(receiver))
        .any(|req| pathing::resolve_module_path(&req.src_filepath, &req.module, ctx.files_set).is_none())
}

fn is_python_builtin_noise(call_ref: &CallRef) -> bool {
    call_ref.language == "python"
        && matches!(call_ref.kind, CallRefKind::Plain)
        && matches!(
            call_ref.callee.as_str(),
            "len"
                | "isinstance"
                | "set"
                | "list"
                | "dict"
                | "tuple"
                | "str"
                | "int"
                | "float"
                | "bool"
                | "min"
                | "max"
                | "sum"
                | "sorted"
                | "any"
                | "all"
                | "enumerate"
                | "zip"
                | "range"
                | "abs"
                | "getattr"
                | "deepcopy"
        )
}

fn is_python_container_method_noise(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> bool {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return false;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    if !receiver
        .chars()
        .next()
        .map(|ch| ch.is_ascii_lowercase())
        .unwrap_or(false)
    {
        return false;
    }
    if receiver.contains('.') || receiver.contains("::") {
        return false;
    }
    if ctx
        .python_module_aliases_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(receiver))
        .is_some()
    {
        return false;
    }
    matches!(
        call_ref.callee.as_str(),
        "append" | "add" | "extend" | "update" | "discard" | "remove" | "pop" | "clear" | "items" | "splitlines"
    )
}

fn is_python_regex_method_noise(call_ref: &CallRef) -> bool {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return false;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    matches!(call_ref.callee.as_str(), "search" | "match" | "findall" | "sub")
        && (receiver.starts_with('_')
            || receiver
                .chars()
                .all(|ch| !ch.is_ascii_alphabetic() || ch.is_ascii_uppercase() || ch == '_'))
}

fn is_python_semantic_payload_support_plain_noise(call_ref: &CallRef) -> bool {
    call_ref.language == "python"
        && call_ref
            .caller_filepath
            .ends_with("python/tree_sitter_language_pack/_semantic_payload.py")
        && matches!(call_ref.kind, CallRefKind::Plain)
        && matches!(
            call_ref.callee.as_str(),
            "_native_build_semantic_sync_plan"
                | "_native_build_codebase_embedding_rows"
                | "round_plan_builder"
                | "progress_fn"
                | "__import__"
                | "embed_batch_fn"
                | "write_batch_fn"
                | "driver_plan_builder"
        )
}

fn is_python_semantic_payload_support_member_noise(call_ref: &CallRef) -> bool {
    if call_ref.language != "python"
        || !call_ref
            .caller_filepath
            .ends_with("python/tree_sitter_language_pack/_semantic_payload.py")
        || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped)
    {
        return false;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    match call_ref.callee.as_str() {
        "encode" | "strip" => matches!(receiver, "source" | "text" | "content"),
        "parse" => receiver == "parser",
        "sha256" => receiver == "hashlib",
        "dumps" => receiver == "json",
        "execute" => matches!(receiver, "conn" | "prune_cursor"),
        "cursor" => receiver == "conn",
        "executemany" => receiver == "cursor",
        "fetchall" => receiver == "cur",
        _ => false,
    }
}

fn is_python_init_wrapper_plain_noise(call_ref: &CallRef) -> bool {
    call_ref.language == "python"
        && call_ref
            .caller_filepath
            .ends_with("python/tree_sitter_language_pack/__init__.py")
        && matches!(call_ref.kind, CallRefKind::Plain)
        && matches!(
            call_ref.callee.as_str(),
            "repr" | "PurePosixPath" | "NotImplementedError"
        )
}

fn is_python_init_wrapper_member_noise(call_ref: &CallRef) -> bool {
    if call_ref.language != "python"
        || !call_ref
            .caller_filepath
            .ends_with("python/tree_sitter_language_pack/__init__.py")
        || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped)
    {
        return false;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    match call_ref.callee.as_str() {
        "split" => matches!(receiver, "value" | "location"),
        "decode" => receiver == "source",
        "replace" => matches!(
            receiver,
            "file_path" | "project_file" | "container" | "group_path" | "rel_path"
        ),
        "startswith" | "endswith" => matches!(receiver, "location" | "normalized_path"),
        "findall" => receiver == "root",
        "get_targets" => receiver == "objects",
        "get_id" => matches!(receiver, "target" | "build_file" | "file_ref" | "phase"),
        "get_objects_in_section" => receiver == "objects",
        _ => false,
    }
}

fn is_python_script_support_member_noise(call_ref: &CallRef) -> bool {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return false;
    }
    let path = call_ref.caller_filepath.as_str();
    let is_script = path.starts_with("scripts/") || path.contains("/scripts/");
    if !is_script {
        return false;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    let callee = call_ref.callee.as_str();
    let lower_receiver = receiver.to_ascii_lowercase();
    let pathish_receiver = lower_receiver.ends_with("_path")
        || lower_receiver.ends_with("_dir")
        || lower_receiver.ends_with("_file")
        || lower_receiver.contains("path")
        || lower_receiver.contains("dir")
        || lower_receiver.contains("file")
        || matches!(
            receiver,
            "CACHE_MANIFEST_FILE"
                | "DEFINITIONS_PATH"
                | "_cache_path"
                | "ffi_lib"
                | "header"
                | "clone_target"
                | "target_src"
                | "target_common"
                | "target_queries"
                | "vendor_directory"
                | "stale_dir"
                | "parsers_directory"
                | "parser_dir"
                | "target_source_dir"
                | "replacement_path"
                | "config_path"
                | "output_path"
                | "definitions_path"
                | "cargo_toml"
                | "file_path"
                | "path"
                | "dest_lib"
                | "core_vendor"
                | "vendor_cargo"
                | "core_toml"
                | "vendor_base"
                | "src"
                | "artifact"
                | "php_toml"
                | "f"
                | "ruby_toml"
        );
    let stringish_receiver = matches!(
        receiver,
        "text"
            | "line"
            | "stripped"
            | "output"
            | "lang"
            | "lang_id"
            | "repo_url"
            | "key"
            | "version"
            | "content"
            | "word"
            | "field"
            | "current"
            | "url"
            | "val"
            | "base_spec"
    );
    let containerish_receiver = lower_receiver.contains("language")
        || matches!(receiver, "languages" | "language_def" | "language_definitions");

    match callee {
        "add_argument" | "parse_args" => receiver == "parser",
        "debug" | "info" | "warning" | "error" | "exception" | "setLevel" => receiver == "logger",
        "exists" | "open" | "write_text" | "mkdir" | "glob" | "rglob" | "iterdir" | "relative_to" | "stat"
        | "unlink" => pathish_receiver,
        "replace" => pathish_receiver || matches!(receiver, "lang_id" | "version" | "content"),
        "split" | "strip" | "rstrip" | "removesuffix" | "startswith" | "capitalize" => stringish_receiver,
        "keys" | "copy" => containerish_receiver,
        "render" => receiver == "template",
        "group" => matches!(receiver, "match" | "m"),
        _ => false,
    }
}

fn resolve_python_init_external_receiver(call_ref: &CallRef) -> Option<ExternalSymbolResolution> {
    if call_ref.language != "python"
        || !call_ref
            .caller_filepath
            .ends_with("python/tree_sitter_language_pack/__init__.py")
        || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped)
    {
        return None;
    }
    let receiver = call_ref.receiver_hint.as_deref()?;
    match (receiver, call_ref.callee.as_str()) {
        ("ElementTree", "fromstring") => Some(ExternalSymbolResolution {
            name: call_ref.callee.clone(),
            qualified_name: "xml.etree.ElementTree.fromstring".to_string(),
            language: call_ref.language.clone(),
        }),
        ("XcodeProject", "load") => Some(ExternalSymbolResolution {
            name: call_ref.callee.clone(),
            qualified_name: "pbxproj.XcodeProject.load".to_string(),
            language: call_ref.language.clone(),
        }),
        _ => None,
    }
}

fn resolve_explicit_python_external_receiver(call_ref: &CallRef) -> Option<ExternalSymbolResolution> {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return None;
    }
    let qualified = call_ref.qualified_hint.as_deref().unwrap_or("");
    let path = call_ref.caller_filepath.as_str();
    let is_script = path.starts_with("scripts/") || path.contains("/scripts/");
    if is_script
        && [
            "json.",
            "yaml.",
            "re.",
            "hashlib.",
            "shutil.",
            "os.",
            "platform.",
            "asyncio.",
            "argparse.",
        ]
        .iter()
        .any(|prefix| qualified.starts_with(prefix))
    {
        return Some(ExternalSymbolResolution {
            name: call_ref.callee.clone(),
            qualified_name: qualified.to_string(),
            language: call_ref.language.clone(),
        });
    }
    let resolved = match qualified {
        "json.loads" => "json.loads",
        "sys.exit" => "sys.exit",
        "logging.getLogger" => "logging.getLogger",
        "yaml.safe_load" => "yaml.safe_load",
        "re.sub" => "re.sub",
        "argparse.ArgumentParser" => "argparse.ArgumentParser",
        "json.load" => "json.load",
        "json.dumps" => "json.dumps",
        "os.cpu_count" => "os.cpu_count",
        "asyncio.Semaphore" => "asyncio.Semaphore",
        "asyncio.gather" => "asyncio.gather",
        "hashlib.sha256" => "hashlib.sha256",
        "platform.system" => "platform.system",
        "re.search" => "re.search",
        "re.match" => "re.match",
        "shutil.copy2" => "shutil.copy2",
        "shutil.rmtree" => "shutil.rmtree",
        _ => return None,
    };
    Some(ExternalSymbolResolution {
        name: call_ref.callee.clone(),
        qualified_name: resolved.to_string(),
        language: call_ref.language.clone(),
    })
}

fn resolve_external_python_module_receiver(
    ctx: &CallResolutionContext<'_>,
    call_ref: &CallRef,
) -> Option<ExternalSymbolResolution> {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return None;
    }
    let receiver = call_ref.receiver_hint.as_deref()?;
    let module = ctx
        .python_module_aliases_by_file
        .get(&call_ref.caller_filepath)
        .and_then(|m| m.get(receiver))?;
    if pathing::resolve_module_path(&call_ref.caller_filepath, module, ctx.files_set).is_some() {
        return None;
    }
    Some(ExternalSymbolResolution {
        name: call_ref.callee.clone(),
        qualified_name: format!("{module}.{}", call_ref.callee),
        language: call_ref.language.clone(),
    })
}

pub(crate) fn resolve_call_ref(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> CallResolution {
    let stages: &[(&str, fn(&CallResolutionContext<'_>, &CallRef) -> Option<String>)] = match call_ref.kind {
        CallRefKind::Scoped if call_ref.language == "python" => &[
            ("python_module_receiver", resolve_by_python_module_receiver),
            ("python_receiver_type", resolve_by_python_receiver_type),
            ("qualified", resolve_by_qualified_hint),
            ("receiver_qualified", resolve_by_receiver_qualified),
            ("import_symbol", resolve_by_import_symbol_request),
            ("imported_target", resolve_by_imported_target_unique),
            ("local_directory", resolve_by_local_directory_unique),
            ("global_unique", resolve_by_global_unique),
        ],
        CallRefKind::Scoped => &[
            ("go_import_receiver", resolve_by_go_import_receiver),
            ("go_receiver_type", resolve_by_go_receiver_type),
            ("rust_receiver_type", resolve_by_rust_receiver_type),
            ("self_qualified", resolve_by_self_qualified),
            ("qualified", resolve_by_qualified_hint),
            ("receiver_qualified", resolve_by_receiver_qualified),
            ("import_symbol", resolve_by_import_symbol_request),
            ("imported_target", resolve_by_imported_target_unique),
            ("local_directory", resolve_by_local_directory_unique),
            ("global_unique", resolve_by_global_unique),
        ],
        CallRefKind::Member => &[
            ("python_module_receiver", resolve_by_python_module_receiver),
            ("python_receiver_type", resolve_by_python_receiver_type),
            ("rust_receiver_type", resolve_by_rust_receiver_type),
            ("receiver_qualified", resolve_by_receiver_qualified),
            ("import_symbol", resolve_by_import_symbol_request),
            ("imported_target", resolve_by_imported_target_unique),
            ("local_directory", resolve_by_local_directory_unique),
            ("global_unique", resolve_by_global_unique),
        ],
        CallRefKind::Plain => &[
            ("python_imported_symbol", resolve_by_python_imported_symbol),
            ("global_unique", resolve_by_global_unique),
            ("same_file", resolve_by_same_file),
            ("import_symbol", resolve_by_import_symbol_request),
            ("imported_target", resolve_by_imported_target_unique),
            ("local_directory", resolve_by_local_directory_unique),
        ],
    };

    for (stage, resolver) in stages {
        if let Some(id) = resolver(ctx, call_ref) {
            return CallResolution::ResolvedInternal(id, stage);
        }
    }
    if is_rust_constructor_noise(call_ref) {
        return CallResolution::Filtered("constructor_noise", None);
    }
    if is_python_builtin_noise(call_ref) {
        return CallResolution::Filtered("python_builtin", None);
    }
    if is_python_semantic_payload_support_plain_noise(call_ref) {
        return CallResolution::Filtered("python_payload_support_plain", None);
    }
    if is_python_init_wrapper_plain_noise(call_ref) {
        return CallResolution::Filtered("python_init_wrapper_plain", None);
    }
    if is_python_container_method_noise(ctx, call_ref) {
        return CallResolution::Filtered("python_container_method", None);
    }
    if is_python_semantic_payload_support_member_noise(call_ref) {
        return CallResolution::Filtered("python_payload_support_member", None);
    }
    if is_python_init_wrapper_member_noise(call_ref) {
        return CallResolution::Filtered("python_init_wrapper_member", None);
    }
    if is_python_script_support_member_noise(call_ref) {
        return CallResolution::Filtered("python_script_support_member", None);
    }
    if is_python_regex_method_noise(call_ref) {
        return CallResolution::Filtered("python_regex_method", None);
    }
    if is_filtered_same_file_policy(ctx, call_ref) {
        return CallResolution::Filtered("policy_same_file", None);
    }
    if is_go_test_receiver_noise(call_ref) {
        return CallResolution::Filtered("go_test_receiver", None);
    }
    if is_python_test_receiver_noise(call_ref) {
        return CallResolution::Filtered("python_test_receiver", None);
    }
    if is_clearly_external_rust_scoped_call(ctx, call_ref) {
        return CallResolution::Filtered(
            "external_rust_scoped",
            call_ref
                .qualified_hint
                .as_ref()
                .map(|qualified_name| ExternalSymbolResolution {
                    name: call_ref.callee.clone(),
                    qualified_name: qualified_name.clone(),
                    language: call_ref.language.clone(),
                }),
        );
    }
    if is_clearly_external_go_scoped_call(ctx, call_ref) {
        return CallResolution::Filtered(
            "external_go_scoped",
            call_ref
                .qualified_hint
                .as_ref()
                .map(|qualified_name| ExternalSymbolResolution {
                    name: call_ref.callee.clone(),
                    qualified_name: qualified_name.clone(),
                    language: call_ref.language.clone(),
                }),
        );
    }
    if let Some(external_symbol) = resolve_external_python_module_receiver(ctx, call_ref) {
        return CallResolution::Filtered("external_python_module", Some(external_symbol));
    }
    if let Some(external_symbol) = resolve_python_init_external_receiver(call_ref) {
        return CallResolution::Filtered("external_python_module", Some(external_symbol));
    }
    if let Some(external_symbol) = resolve_explicit_python_external_receiver(call_ref) {
        return CallResolution::Filtered("external_python_module", Some(external_symbol));
    }
    if std::env::var("TS_PACK_DEBUG_PYTHON_SCOPED_TRACE")
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
        && call_ref.language == "python"
        && matches!(call_ref.kind, CallRefKind::Scoped)
        && (call_ref
            .caller_filepath
            .ends_with("python/tree_sitter_language_pack/_semantic_payload.py")
            || call_ref
                .caller_filepath
                .ends_with("python/tree_sitter_language_pack/__init__.py")
            || call_ref.caller_filepath.contains("/tests/")
            || call_ref.caller_filepath.starts_with("tests/"))
    {
        eprintln!(
            "[ts-pack-index] PY TRACE unresolved scoped — file={} callee={} recv={} qualified={} payload_member={} init_member={} container={} regex={} py_test={} ext_py_module={}",
            call_ref.caller_filepath,
            call_ref.callee,
            call_ref.receiver_hint.as_deref().unwrap_or("-"),
            call_ref.qualified_hint.as_deref().unwrap_or("-"),
            is_python_semantic_payload_support_member_noise(call_ref),
            is_python_init_wrapper_member_noise(call_ref),
            is_python_container_method_noise(ctx, call_ref),
            is_python_regex_method_noise(call_ref),
            is_python_test_receiver_noise(call_ref),
            resolve_external_python_module_receiver(ctx, call_ref).is_some()
                || resolve_python_init_external_receiver(call_ref).is_some()
                || resolve_explicit_python_external_receiver(call_ref).is_some(),
        );
    }
    CallResolution::Unresolved("no_internal_resolution")
}

fn is_rust_constructor_noise(call_ref: &CallRef) -> bool {
    call_ref.language == "rust"
        && matches!(call_ref.kind, CallRefKind::Plain)
        && matches!(call_ref.callee.as_str(), "Ok" | "Err" | "Some" | "None")
}

fn is_filtered_same_file_policy(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> bool {
    !call_ref.allow_same_file
        && ctx
            .symbols_by_file
            .get(&call_ref.caller_filepath)
            .and_then(|sym_map| sym_map.get(&call_ref.callee))
            .is_some()
}

fn is_go_test_receiver_noise(call_ref: &CallRef) -> bool {
    if call_ref.language != "go" || !matches!(call_ref.kind, CallRefKind::Scoped) {
        return false;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    if !matches!(receiver, "t" | "b" | "m") {
        return false;
    }
    let path = call_ref.caller_filepath.as_str();
    if !(path.contains("/test") || path.ends_with("_test.go")) {
        return false;
    }
    match receiver {
        "t" => matches!(
            call_ref.callee.as_str(),
            "Helper" | "Fatal" | "Fatalf" | "Error" | "Errorf" | "Log" | "Logf" | "Run" | "Cleanup" | "Skipf"
        ),
        "b" => matches!(call_ref.callee.as_str(), "Run"),
        "m" => matches!(call_ref.callee.as_str(), "Run"),
        _ => false,
    }
}

fn is_python_test_receiver_noise(call_ref: &CallRef) -> bool {
    if call_ref.language != "python" || !matches!(call_ref.kind, CallRefKind::Member | CallRefKind::Scoped) {
        return false;
    }
    let path = call_ref.caller_filepath.as_str();
    if !(path.contains("/tests/") || path.starts_with("tests/") || path.contains("test_")) {
        return false;
    }
    let qualified = call_ref.qualified_hint.as_deref().unwrap_or("");
    if matches!(
        qualified,
        "unittest.main" | "pytest.fixture" | "pytest.mark" | "pytest.fail" | "pytest.raises"
    ) {
        return true;
    }
    let Some(receiver) = call_ref.receiver_hint.as_deref() else {
        return false;
    };
    match receiver {
        "self" => matches!(
            call_ref.callee.as_str(),
            "skipTest"
                | "assertIn"
                | "assertEqual"
                | "assertTrue"
                | "assertFalse"
                | "assertRaises"
                | "assertIsNone"
                | "assertIsNotNone"
        ),
        "unittest" => matches!(call_ref.callee.as_str(), "main"),
        "pytest" => matches!(call_ref.callee.as_str(), "fixture" | "mark" | "fail" | "raises"),
        _ => false,
    }
}

pub(crate) fn build_symbol_call_rows(
    call_refs: Vec<CallRef>,
    resolution_ctx: &CallResolutionContext<'_>,
    project_id: &std::sync::Arc<str>,
    debug_call_resolution: bool,
) -> CallResolutionOutputs {
    let mut symbol_call_rows = Vec::with_capacity(call_refs.len());
    let mut external_symbol_nodes = Vec::new();
    let mut external_symbol_edges = Vec::new();
    let mut resolved_call_rows = 0usize;
    let mut unresolved_internal_call_rows = 0usize;
    let mut resolution_stage_counts: HashMap<&'static str, usize> = HashMap::new();
    let mut filtered_stage_counts: HashMap<&'static str, usize> = HashMap::new();
    let mut unresolved_name_counts: HashMap<String, usize> = HashMap::new();
    let mut unresolved_bucket_counts: HashMap<String, usize> = HashMap::new();
    let mut unresolved_bucket_samples: HashMap<String, Vec<String>> = HashMap::new();
    let mut unresolved_rust_plain_attribution: HashMap<(String, String), usize> = HashMap::new();
    let mut skipped_external_call_rows = 0usize;

    for call_ref in call_refs {
        match resolve_call_ref(resolution_ctx, &call_ref) {
            CallResolution::ResolvedInternal(id, stage) => {
                resolved_call_rows += 1;
                *resolution_stage_counts.entry(stage).or_insert(0) += 1;
                symbol_call_rows.push(SymbolCallRow {
                    caller_id: call_ref.caller_id,
                    callee: call_ref.callee,
                    callee_id: Some(id),
                    project_id: std::sync::Arc::clone(project_id),
                    caller_filepath: call_ref.caller_filepath,
                    allow_same_file: call_ref.allow_same_file,
                });
            }
            CallResolution::Filtered(reason, external_symbol) => {
                skipped_external_call_rows += 1;
                *filtered_stage_counts.entry(reason).or_insert(0) += 1;
                if let Some(external_symbol) = external_symbol {
                    let external_id = crate::external_symbol_id(
                        project_id.as_ref(),
                        &external_symbol.language,
                        &external_symbol.qualified_name,
                    );
                    external_symbol_nodes.push(ExternalSymbolNode {
                        id: external_id.clone(),
                        name: external_symbol.name,
                        qualified_name: external_symbol.qualified_name,
                        language: external_symbol.language,
                        project_id: std::sync::Arc::clone(project_id),
                    });
                    external_symbol_edges.push(ExternalSymbolEdgeRow {
                        src: call_ref.caller_id,
                        tgt: external_id,
                    });
                }
            }
            CallResolution::Unresolved(reason) => {
                unresolved_internal_call_rows += 1;
                if debug_call_resolution {
                    *unresolved_name_counts.entry(call_ref.callee.clone()).or_insert(0) += 1;
                    let kind = match call_ref.kind {
                        CallRefKind::Plain => "plain",
                        CallRefKind::Member => "member",
                        CallRefKind::Scoped => "scoped",
                    };
                    let bucket = format!(
                        "{}:{}:{}",
                        call_ref.language,
                        kind,
                        if call_ref.receiver_hint.is_some() {
                            "recv"
                        } else {
                            "norecv"
                        }
                    );
                    *unresolved_bucket_counts.entry(bucket.clone()).or_insert(0) += 1;
                    if bucket == "rust:plain:norecv" {
                        *unresolved_rust_plain_attribution
                            .entry((call_ref.callee.clone(), call_ref.caller_filepath.clone()))
                            .or_insert(0) += 1;
                    }
                    let samples = unresolved_bucket_samples.entry(bucket).or_default();
                    if samples.len() < 5 {
                        let qualified = call_ref.qualified_hint.as_deref().unwrap_or("-");
                        let receiver = call_ref.receiver_hint.as_deref().unwrap_or("-");
                        samples.push(format!(
                            "{} @ {} (qualified={}, recv={}, reason={})",
                            call_ref.callee, call_ref.caller_filepath, qualified, receiver, reason
                        ));
                    }
                }
            }
        }
    }

    CallResolutionOutputs {
        symbol_call_rows,
        external_symbol_nodes,
        external_symbol_edges,
        resolved_call_rows,
        unresolved_internal_call_rows,
        resolution_stage_counts,
        filtered_stage_counts,
        unresolved_name_counts,
        unresolved_bucket_counts,
        unresolved_bucket_samples,
        unresolved_rust_plain_attribution,
        skipped_external_call_rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unresolved_calls_do_not_enter_canonical_rows() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };
        let project_id: std::sync::Arc<str> = std::sync::Arc::from("proj");
        let outputs = build_symbol_call_rows(
            vec![CallRef {
                caller_id: "caller".into(),
                callee: "mystery_helper".into(),
                language: "rust".into(),
                caller_filepath: "crates/foo/src/lib.rs".into(),
                allow_same_file: false,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            }],
            &ctx,
            &project_id,
            true,
        );

        assert_eq!(outputs.symbol_call_rows.len(), 0);
        assert_eq!(outputs.resolved_call_rows, 0);
        assert_eq!(outputs.unresolved_internal_call_rows, 1);
    }

    #[test]
    fn constructor_noise_is_filtered_before_canonical_rows() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };
        let project_id: std::sync::Arc<str> = std::sync::Arc::from("proj");
        let outputs = build_symbol_call_rows(
            vec![CallRef {
                caller_id: "caller".into(),
                callee: "Ok".into(),
                language: "rust".into(),
                caller_filepath: "crates/foo/src/lib.rs".into(),
                allow_same_file: false,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            }],
            &ctx,
            &project_id,
            true,
        );

        assert_eq!(outputs.symbol_call_rows.len(), 0);
        assert_eq!(outputs.skipped_external_call_rows, 1);
        assert_eq!(outputs.filtered_stage_counts.get("constructor_noise").copied(), Some(1));
    }

    #[test]
    fn self_qualified_rust_calls_resolve_from_caller_context() {
        let callable_symbols_by_name = HashMap::from([(
            "new".to_string(),
            vec![("sym:new".to_string(), "src/validator.rs".to_string())],
        )]);
        let qualified_callable_symbols = vec![(
            "validators::JavaValidator::new".to_string(),
            "sym:new".to_string(),
            "src/validator.rs".to_string(),
        )];
        let caller_qualified_symbols_by_id =
            HashMap::from([("caller".to_string(), "validators::JavaValidator::validate".to_string())]);
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &qualified_callable_symbols,
            caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "new".into(),
                language: "rust".into(),
                caller_filepath: "src/validator.rs".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: None,
                qualified_hint: Some("Self::new".into()),
            },
        ) {
            CallResolution::ResolvedInternal(id, stage) => {
                assert_eq!(id, "sym:new");
                assert_eq!(stage, "self_qualified");
            }
            _ => panic!("expected self-qualified resolution"),
        }
    }

    #[test]
    fn python_imported_symbol_calls_resolve_exactly() {
        let callable_symbols_by_name = HashMap::from([(
            "run".to_string(),
            vec![("sym:run".to_string(), "pkg/helpers.py".to_string())],
        )]);
        let symbols_by_file = HashMap::from([(
            "pkg/helpers.py".to_string(),
            HashMap::from([("run".to_string(), "sym:run".to_string())]),
        )]);
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::from([(
            "pkg/main.py".to_string(),
            HashMap::from([("run".to_string(), ".helpers".to_string())]),
        )]);
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::from(["pkg/main.py".to_string(), "pkg/helpers.py".to_string()]);
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "run".into(),
                language: "python".into(),
                caller_filepath: "pkg/main.py".into(),
                allow_same_file: false,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            },
        ) {
            CallResolution::ResolvedInternal(id, stage) => {
                assert_eq!(id, "sym:run");
                assert_eq!(stage, "python_imported_symbol");
            }
            _ => panic!("expected python imported-symbol resolution"),
        }
    }

    #[test]
    fn rust_build_script_stdlib_scoped_calls_are_filtered_as_external() {
        let callable_symbols_by_name = HashMap::new();
        let qualified_callable_symbols = vec![];
        let caller_qualified_symbols_by_id = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root =
            HashMap::from([("/workspace/src".to_string(), HashSet::from(["localmod".to_string()]))]);
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &qualified_callable_symbols,
            caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "var".into(),
                language: "rust".into(),
                caller_filepath: "build.rs".into(),
                allow_same_file: false,
                kind: CallRefKind::Scoped,
                receiver_hint: None,
                qualified_hint: Some("env::var".into()),
            },
        ) {
            CallResolution::Filtered(reason, external_symbol) => {
                assert_eq!(reason, "external_rust_scoped");
                let external_symbol = external_symbol.expect("expected external symbol payload");
                assert_eq!(external_symbol.qualified_name, "env::var");
            }
            _ => panic!("expected external rust scoped classification"),
        }
    }

    #[test]
    fn go_test_receiver_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let qualified_callable_symbols = vec![];
        let caller_qualified_symbols_by_id = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &qualified_callable_symbols,
            caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "Cleanup".into(),
                language: "go".into(),
                caller_filepath: "tests/smoke_test.go".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("t".into()),
                qualified_hint: Some("t.Cleanup".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "go_test_receiver"),
            _ => panic!("expected go test receiver filter"),
        }
    }

    #[test]
    fn python_test_receiver_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let qualified_callable_symbols = vec![];
        let caller_qualified_symbols_by_id = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &qualified_callable_symbols,
            caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "skipTest".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/tests/test_extract_file_facts_wrapper.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("self".into()),
                qualified_hint: Some("self.skipTest".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_test_receiver"),
            _ => panic!("expected python test receiver filter"),
        }
    }

    #[test]
    fn go_import_receiver_calls_resolve_exactly() {
        let callable_symbols_by_name = HashMap::from([(
            "DownloadedLanguages".to_string(),
            vec![
                ("sym:dl".to_string(), "packages/go/v1/tspack.go".to_string()),
                ("sym:other".to_string(), "packages/csharp/TsPackClient.cs".to_string()),
            ],
        )]);
        let qualified_callable_symbols = vec![(
            "tspack.DownloadedLanguages".to_string(),
            "sym:dl".to_string(),
            "packages/go/v1/tspack.go".to_string(),
        )];
        let caller_qualified_symbols_by_id = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::from([(
            "tests/test_apps/go/smoke_test.go".to_string(),
            HashMap::from([(
                "tslp".to_string(),
                "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go".to_string(),
            )]),
        )]);
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::from([
            "tests/test_apps/go/smoke_test.go".to_string(),
            "packages/go/v1/tspack.go".to_string(),
            "packages/go/doc.go".to_string(),
        ]);
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &qualified_callable_symbols,
            caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "DownloadedLanguages".into(),
                language: "go".into(),
                caller_filepath: "tests/test_apps/go/smoke_test.go".into(),
                allow_same_file: false,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("tslp".into()),
                qualified_hint: Some("tslp.DownloadedLanguages".into()),
            },
        ) {
            CallResolution::ResolvedInternal(id, stage) => {
                assert_eq!(id, "sym:dl");
                assert_eq!(stage, "go_import_receiver");
            }
            _ => panic!("expected go import-receiver resolution"),
        }
    }

    #[test]
    fn go_receiver_type_calls_resolve_exactly() {
        let callable_symbols_by_name = HashMap::from([(
            "LanguageCount".to_string(),
            vec![("sym:count".to_string(), "packages/go/v1/tspack.go".to_string())],
        )]);
        let qualified_callable_symbols = vec![(
            "tspack.Registry.LanguageCount".to_string(),
            "sym:count".to_string(),
            "packages/go/v1/tspack.go".to_string(),
        )];
        let caller_qualified_symbols_by_id = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::from([(
            "tests/test_apps/go/smoke_test.go".to_string(),
            HashMap::from([("registry".to_string(), "Registry".to_string())]),
        )]);
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &qualified_callable_symbols,
            caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "LanguageCount".into(),
                language: "go".into(),
                caller_filepath: "tests/test_apps/go/smoke_test.go".into(),
                allow_same_file: false,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("registry".into()),
                qualified_hint: Some("registry.LanguageCount".into()),
            },
        ) {
            CallResolution::ResolvedInternal(id, stage) => {
                assert_eq!(id, "sym:count");
                assert_eq!(stage, "go_receiver_type");
            }
            _ => panic!("expected go receiver-type resolution"),
        }
    }

    #[test]
    fn rust_receiver_type_calls_resolve_exactly() {
        let callable_symbols_by_name = HashMap::from([(
            "process".to_string(),
            vec![(
                "sym:process".to_string(),
                "crates/ts-pack-core/src/registry.rs".to_string(),
            )],
        )]);
        let qualified_callable_symbols = vec![(
            "LanguageRegistry::process".to_string(),
            "sym:process".to_string(),
            "crates/ts-pack-core/src/registry.rs".to_string(),
        )];
        let caller_qualified_symbols_by_id = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::from([(
            "crates/ts-pack-core/src/lib.rs".to_string(),
            HashMap::from([("REGISTRY".to_string(), "LanguageRegistry".to_string())]),
        )]);
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &qualified_callable_symbols,
            caller_qualified_symbols_by_id: &caller_qualified_symbols_by_id,
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "sym:wrapper".into(),
                callee: "process".into(),
                language: "rust".into(),
                caller_filepath: "crates/ts-pack-core/src/lib.rs".into(),
                allow_same_file: false,
                kind: CallRefKind::Member,
                receiver_hint: Some("REGISTRY".into()),
                qualified_hint: Some("REGISTRY.process".into()),
            },
        ) {
            CallResolution::ResolvedInternal(id, stage) => {
                assert_eq!(id, "sym:process");
                assert_eq!(stage, "rust_receiver_type");
            }
            _ => panic!("expected rust receiver-type resolution"),
        }
    }

    #[test]
    fn python_builtin_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "len".into(),
                language: "python".into(),
                caller_filepath: "pkg/helpers.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_builtin"),
            _ => panic!("expected python builtin filter"),
        }
    }

    #[test]
    fn python_container_methods_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "append".into(),
                language: "python".into(),
                caller_filepath: "pkg/helpers.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Member,
                receiver_hint: Some("out".into()),
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_container_method"),
            _ => panic!("expected python container method filter"),
        }
    }

    #[test]
    fn python_external_module_receivers_are_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::from([(
            "pkg/helpers.py".to_string(),
            HashMap::from([("hashlib".to_string(), "hashlib".to_string())]),
        )]);
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::from(["pkg/helpers.py".to_string()]);
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "sha256".into(),
                language: "python".into(),
                caller_filepath: "pkg/helpers.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Member,
                receiver_hint: Some("hashlib".into()),
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "hashlib.sha256");
            }
            _ => panic!("expected external python module classification"),
        }
    }

    #[test]
    fn explicit_python_stdlib_receivers_are_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "loads".into(),
                language: "python".into(),
                caller_filepath: "tests/test_apps/python/smoke_test.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("json".into()),
                qualified_hint: Some("json.loads".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "json.loads");
            }
            _ => panic!("expected explicit python stdlib classification"),
        }
    }

    #[test]
    fn explicit_python_yaml_receiver_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "safe_load".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("yaml".into()),
                qualified_hint: Some("yaml.safe_load".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "yaml.safe_load");
            }
            _ => panic!("expected explicit python yaml classification"),
        }
    }

    #[test]
    fn explicit_python_re_receiver_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "sub".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("re".into()),
                qualified_hint: Some("re.sub".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "re.sub");
            }
            _ => panic!("expected explicit python re classification"),
        }
    }

    #[test]
    fn explicit_python_argparse_receiver_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "ArgumentParser".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("argparse".into()),
                qualified_hint: Some("argparse.ArgumentParser".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "argparse.ArgumentParser");
            }
            _ => panic!("expected explicit python argparse classification"),
        }
    }

    #[test]
    fn explicit_python_asyncio_and_os_receivers_are_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        for (callee, qualified) in [
            ("cpu_count", "os.cpu_count"),
            ("Semaphore", "asyncio.Semaphore"),
            ("gather", "asyncio.gather"),
        ] {
            match resolve_call_ref(
                &ctx,
                &CallRef {
                    caller_id: "caller".into(),
                    callee: callee.into(),
                    language: "python".into(),
                    caller_filepath: "scripts/pin_vendors.py".into(),
                    allow_same_file: true,
                    kind: CallRefKind::Scoped,
                    receiver_hint: Some(qualified.split('.').next().unwrap().into()),
                    qualified_hint: Some(qualified.into()),
                },
            ) {
                CallResolution::Filtered(reason, Some(external_symbol)) => {
                    assert_eq!(reason, "external_python_module");
                    assert_eq!(external_symbol.qualified_name, qualified);
                }
                _ => panic!("expected explicit python stdlib classification"),
            }
        }
    }

    #[test]
    fn explicit_python_json_receiver_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "dumps".into(),
                language: "python".into(),
                caller_filepath: "scripts/pin_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("json".into()),
                qualified_hint: Some("json.dumps".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "json.dumps");
            }
            _ => panic!("expected explicit python json classification"),
        }
    }

    #[test]
    fn explicit_python_json_load_receiver_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "load".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("json".into()),
                qualified_hint: Some("json.load".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "json.load");
            }
            _ => panic!("expected explicit python json.load classification"),
        }
    }

    #[test]
    fn explicit_python_hashlib_receiver_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "sha256".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("hashlib".into()),
                qualified_hint: Some("hashlib.sha256".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "hashlib.sha256");
            }
            _ => panic!("expected explicit python hashlib classification"),
        }
    }

    #[test]
    fn explicit_python_platform_receiver_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "system".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("platform".into()),
                qualified_hint: Some("platform.system".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "platform.system");
            }
            _ => panic!("expected explicit python platform classification"),
        }
    }

    #[test]
    fn explicit_python_re_search_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "search".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("re".into()),
                qualified_hint: Some("re.search".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "re.search");
            }
            _ => panic!("expected explicit python re.search classification"),
        }
    }

    #[test]
    fn explicit_python_re_match_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "match".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("re".into()),
                qualified_hint: Some("re.match".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "re.match");
            }
            _ => panic!("expected explicit python re.match classification"),
        }
    }

    #[test]
    fn explicit_python_shutil_copy2_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "copy2".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("shutil".into()),
                qualified_hint: Some("shutil.copy2".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "shutil.copy2");
            }
            _ => panic!("expected explicit python shutil.copy2 classification"),
        }
    }

    #[test]
    fn explicit_python_shutil_rmtree_is_classified() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "rmtree".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("shutil".into()),
                qualified_hint: Some("shutil.rmtree".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "shutil.rmtree");
            }
            _ => panic!("expected explicit python shutil.rmtree classification"),
        }
    }

    #[test]
    fn python_script_support_member_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "open".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("config_path".into()),
                qualified_hint: Some("config_path.open".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script support member filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "debug".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("logger".into()),
                qualified_hint: Some("logger.debug".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script logger debug filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("output_path".into()),
                qualified_hint: Some("output_path.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script output_path write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "warning".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("logger".into()),
                qualified_hint: Some("logger.warning".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script logger warning filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exception".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("logger".into()),
                qualified_hint: Some("logger.exception".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script logger exception filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "relative_to".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("file_path".into()),
                qualified_hint: Some("file_path.relative_to".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script file_path relative_to filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "keys".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("languages".into()),
                qualified_hint: Some("languages.keys".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script languages keys filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "add_argument".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.add_argument".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script parser add_argument filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "setLevel".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_readme.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("logger".into()),
                qualified_hint: Some("logger.setLevel".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected python script logger setLevel filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "split".into(),
                language: "python".into(),
                caller_filepath: "scripts/pin_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("output".into()),
                qualified_hint: Some("output.split".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected pin_vendors output split filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "copy".into(),
                language: "python".into(),
                caller_filepath: "scripts/pin_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("language_def".into()),
                qualified_hint: Some("language_def.copy".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected pin_vendors language_def copy filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/pin_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("definitions_path".into()),
                qualified_hint: Some("definitions_path.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected pin_vendors definitions_path write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "add_argument".into(),
                language: "python".into(),
                caller_filepath: "scripts/pin_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.add_argument".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected pin_vendors parser add_argument filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "parse_args".into(),
                language: "python".into(),
                caller_filepath: "scripts/pin_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.parse_args".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected pin_vendors parser parse_args filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("CACHE_MANIFEST_FILE".into()),
                qualified_hint: Some("CACHE_MANIFEST_FILE.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors cache manifest exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("CACHE_MANIFEST_FILE".into()),
                qualified_hint: Some("CACHE_MANIFEST_FILE.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors cache manifest write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "iterdir".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser_dir".into()),
                qualified_hint: Some("parser_dir.iterdir".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors parser_dir iterdir filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "keys".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("language_definitions".into()),
                qualified_hint: Some("language_definitions.keys".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors language_definitions keys filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("clone_target".into()),
                qualified_hint: Some("clone_target.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors clone_target exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("target_src".into()),
                qualified_hint: Some("target_src.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors target_src exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "glob".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("target_source_dir".into()),
                qualified_hint: Some("target_source_dir.glob".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors target_source_dir glob filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "replace".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("replacement_path".into()),
                qualified_hint: Some("replacement_path.replace".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors replacement_path replace filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("target_queries".into()),
                qualified_hint: Some("target_queries.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors target_queries exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "mkdir".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parsers_directory".into()),
                qualified_hint: Some("parsers_directory.mkdir".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors parsers_directory mkdir filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("stale_dir".into()),
                qualified_hint: Some("stale_dir.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors stale_dir exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("vendor_directory".into()),
                qualified_hint: Some("vendor_directory.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors vendor_directory exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/clone_vendors.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parsers_directory".into()),
                qualified_hint: Some("parsers_directory.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected clone_vendors parsers_directory exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("cargo_toml".into()),
                qualified_hint: Some("cargo_toml.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions cargo_toml exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "group".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("match".into()),
                qualified_hint: Some("match.group".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions match.group filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "group".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("m".into()),
                qualified_hint: Some("m.group".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions m.group filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("file_path".into()),
                qualified_hint: Some("file_path.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions file_path write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "startswith".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("key".into()),
                qualified_hint: Some("key.startswith".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions key.startswith filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "replace".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("version".into()),
                qualified_hint: Some("version.replace".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions version.replace filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "replace".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("content".into()),
                qualified_hint: Some("content.replace".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions content.replace filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("file_path".into()),
                qualified_hint: Some("file_path.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions file_path exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "relative_to".into(),
                language: "python".into(),
                caller_filepath: "scripts/sync_versions.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("file_path".into()),
                qualified_hint: Some("file_path.relative_to".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected sync_versions file_path relative_to filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "split".into(),
                language: "python".into(),
                caller_filepath: "scripts/check_grammar_updates.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("output".into()),
                qualified_hint: Some("output.split".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected check_grammar_updates output.split filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "strip".into(),
                language: "python".into(),
                caller_filepath: "scripts/check_grammar_updates.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("lang".into()),
                qualified_hint: Some("lang.strip".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected check_grammar_updates lang.strip filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/check_grammar_updates.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("DEFINITIONS_PATH".into()),
                qualified_hint: Some("DEFINITIONS_PATH.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected check_grammar_updates definitions path write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/check_grammar_updates.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("report_path".into()),
                qualified_hint: Some("report_path.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected check_grammar_updates report path write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "add_argument".into(),
                language: "python".into(),
                caller_filepath: "scripts/check_grammar_updates.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.add_argument".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected check_grammar_updates parser add_argument filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "parse_args".into(),
                language: "python".into(),
                caller_filepath: "scripts/check_grammar_updates.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.parse_args".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected check_grammar_updates parser parse_args filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "capitalize".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("word".into()),
                qualified_hint: Some("word.capitalize".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table word.capitalize filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "replace".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("lang_id".into()),
                qualified_hint: Some("lang_id.replace".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table lang_id.replace filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "rstrip".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("repo_url".into()),
                qualified_hint: Some("repo_url.rstrip".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table repo_url.rstrip filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("definitions_path".into()),
                qualified_hint: Some("definitions_path.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table definitions_path.exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "open".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("definitions_path".into()),
                qualified_hint: Some("definitions_path.open".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table definitions_path.open filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "add_argument".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.add_argument".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table parser.add_argument filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("output_path".into()),
                qualified_hint: Some("output_path.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table output_path.write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "relative_to".into(),
                language: "python".into(),
                caller_filepath: "scripts/generate_grammar_table.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("output_path".into()),
                qualified_hint: Some("output_path.relative_to".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected generate_grammar_table output_path.relative_to filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "rstrip".into(),
                language: "python".into(),
                caller_filepath: "scripts/lint_grammar_licenses.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("url".into()),
                qualified_hint: Some("url.rstrip".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected lint_grammar_licenses url.rstrip filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "removesuffix".into(),
                language: "python".into(),
                caller_filepath: "scripts/lint_grammar_licenses.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("url".into()),
                qualified_hint: Some("url.removesuffix".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected lint_grammar_licenses url.removesuffix filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "startswith".into(),
                language: "python".into(),
                caller_filepath: "scripts/lint_grammar_licenses.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("url".into()),
                qualified_hint: Some("url.startswith".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected lint_grammar_licenses url.startswith filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/lint_grammar_licenses.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("_cache_path".into()),
                qualified_hint: Some("_cache_path.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected lint_grammar_licenses _cache_path.exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "write_text".into(),
                language: "python".into(),
                caller_filepath: "scripts/lint_grammar_licenses.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("_cache_path".into()),
                qualified_hint: Some("_cache_path.write_text".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected lint_grammar_licenses _cache_path.write_text filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("header".into()),
                qualified_hint: Some("header.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected vendor-ffi header.exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("ffi_lib".into()),
                qualified_hint: Some("ffi_lib.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected vendor-ffi ffi_lib.exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "mkdir".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("include_dir".into()),
                qualified_hint: Some("include_dir.mkdir".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected vendor-ffi include_dir.mkdir filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "mkdir".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("lib_dir".into()),
                qualified_hint: Some("lib_dir.mkdir".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected vendor-ffi lib_dir.mkdir filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "stat".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("dest_lib".into()),
                qualified_hint: Some("dest_lib.stat".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected vendor-ffi dest_lib.stat filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "exists".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("path".into()),
                qualified_hint: Some("path.exists".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected vendor-ffi path.exists filter"),
        }

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "add_argument".into(),
                language: "python".into(),
                caller_filepath: "scripts/ci/go/vendor-ffi.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.add_argument".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => {
                assert_eq!(reason, "python_script_support_member")
            }
            _ => panic!("expected vendor-ffi parser.add_argument filter"),
        }
    }

    #[test]
    fn python_support_plain_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "deepcopy".into(),
                language: "python".into(),
                caller_filepath: "pkg/helpers.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_builtin"),
            _ => panic!("expected python support plain builtin filter"),
        }
    }

    #[test]
    fn python_support_member_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "items".into(),
                language: "python".into(),
                caller_filepath: "pkg/helpers.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Member,
                receiver_hint: Some("extractions".into()),
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_container_method"),
            _ => panic!("expected python support member filter"),
        }
    }

    #[test]
    fn python_regex_methods_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "search".into(),
                language: "python".into(),
                caller_filepath: "pkg/helpers.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Member,
                receiver_hint: Some("_TS_QUERYRAW_TAGGED_TEMPLATE_RE".into()),
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_regex_method"),
            _ => panic!("expected python regex method filter"),
        }
    }

    #[test]
    fn python_semantic_payload_support_plain_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "__import__".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/python/tree_sitter_language_pack/_semantic_payload.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_payload_support_plain"),
            _ => panic!("expected python semantic payload support plain filter"),
        }
    }

    #[test]
    fn python_semantic_payload_support_member_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "parse".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/python/tree_sitter_language_pack/_semantic_payload.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("parser".into()),
                qualified_hint: Some("parser.parse".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_payload_support_member"),
            _ => panic!("expected python semantic payload support member filter"),
        }
    }

    #[test]
    fn python_semantic_payload_db_json_member_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "execute".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/python/tree_sitter_language_pack/_semantic_payload.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("conn".into()),
                qualified_hint: Some("conn.execute".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_payload_support_member"),
            _ => panic!("expected python semantic payload db/json member filter"),
        }
    }

    #[test]
    fn python_semantic_payload_callback_plain_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "embed_batch_fn".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/python/tree_sitter_language_pack/_semantic_payload.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_payload_support_plain"),
            _ => panic!("expected python semantic payload callback plain filter"),
        }
    }

    #[test]
    fn python_init_wrapper_plain_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "PurePosixPath".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/python/tree_sitter_language_pack/__init__.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Plain,
                receiver_hint: None,
                qualified_hint: None,
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_init_wrapper_plain"),
            _ => panic!("expected python init wrapper plain filter"),
        }
    }

    #[test]
    fn python_init_wrapper_member_calls_are_filtered() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "decode".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/python/tree_sitter_language_pack/__init__.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("source".into()),
                qualified_hint: Some("source.decode".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "python_init_wrapper_member"),
            _ => panic!("expected python init wrapper member filter"),
        }
    }

    #[test]
    fn python_init_elementtree_calls_are_external() {
        let callable_symbols_by_name = HashMap::new();
        let symbols_by_file = HashMap::new();
        let go_import_aliases_by_file = HashMap::new();
        let go_var_types_by_file = HashMap::new();
        let rust_var_types_by_file = HashMap::new();
        let python_var_types_by_file = HashMap::new();
        let python_module_aliases_by_file = HashMap::new();
        let python_imported_symbol_modules_by_file = HashMap::new();
        let imported_target_files_by_src = HashMap::new();
        let exported_symbols_by_file = HashMap::new();
        let files_set = HashSet::new();
        let rust_local_module_roots_by_src_root = HashMap::new();
        let ctx = CallResolutionContext {
            callable_symbols_by_name: &callable_symbols_by_name,
            qualified_callable_symbols: &[],
            caller_qualified_symbols_by_id: &HashMap::new(),
            symbols_by_file: &symbols_by_file,
            go_import_aliases_by_file: &go_import_aliases_by_file,
            go_var_types_by_file: &go_var_types_by_file,
            rust_var_types_by_file: &rust_var_types_by_file,
            python_var_types_by_file: &python_var_types_by_file,
            python_module_aliases_by_file: &python_module_aliases_by_file,
            python_imported_symbol_modules_by_file: &python_imported_symbol_modules_by_file,
            imported_target_files_by_src: &imported_target_files_by_src,
            import_symbol_requests: &[],
            exported_symbols_by_file: &exported_symbols_by_file,
            files_set: &files_set,
            rust_local_module_roots_by_src_root: &rust_local_module_roots_by_src_root,
        };

        match resolve_call_ref(
            &ctx,
            &CallRef {
                caller_id: "caller".into(),
                callee: "fromstring".into(),
                language: "python".into(),
                caller_filepath: "crates/ts-pack-python/python/tree_sitter_language_pack/__init__.py".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("ElementTree".into()),
                qualified_hint: Some("ElementTree.fromstring".into()),
            },
        ) {
            CallResolution::Filtered(reason, Some(external_symbol)) => {
                assert_eq!(reason, "external_python_module");
                assert_eq!(external_symbol.qualified_name, "xml.etree.ElementTree.fromstring");
            }
            _ => panic!("expected python init external module classification"),
        }
    }
}
