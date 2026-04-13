use std::collections::{HashMap, HashSet};

use crate::pathing;
use crate::{
    CallRef, CallRefKind, ExternalSymbolEdgeRow, ExternalSymbolNode, ImportSymbolRequest, SymbolCallRow,
};

pub(crate) struct CallResolutionContext<'a> {
    pub(crate) callable_symbols_by_name: &'a HashMap<String, Vec<(String, String)>>,
    pub(crate) qualified_callable_symbols: &'a [(String, String, String)],
    pub(crate) caller_qualified_symbols_by_id: &'a HashMap<String, String>,
    pub(crate) symbols_by_file: &'a HashMap<String, HashMap<String, String>>,
    pub(crate) go_var_types_by_file: &'a HashMap<String, HashMap<String, String>>,
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
    if call_ref.language != "python" {
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
    !ctx
        .rust_local_module_roots_by_src_root
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

pub(crate) fn resolve_call_ref(ctx: &CallResolutionContext<'_>, call_ref: &CallRef) -> CallResolution {
    let stages: &[(&str, fn(&CallResolutionContext<'_>, &CallRef) -> Option<String>)] = match call_ref.kind {
        CallRefKind::Scoped => &[
            ("go_receiver_type", resolve_by_go_receiver_type),
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
    if is_filtered_same_file_policy(ctx, call_ref) {
        return CallResolution::Filtered("policy_same_file", None);
    }
    if is_go_test_receiver_noise(call_ref) {
        return CallResolution::Filtered("go_test_receiver", None);
    }
    if is_clearly_external_rust_scoped_call(ctx, call_ref) {
        return CallResolution::Filtered(
            "external_rust_scoped",
            call_ref.qualified_hint.as_ref().map(|qualified_name| ExternalSymbolResolution {
                name: call_ref.callee.clone(),
                qualified_name: qualified_name.clone(),
                language: call_ref.language.clone(),
            }),
        );
    }
    if is_clearly_external_go_scoped_call(ctx, call_ref) {
        return CallResolution::Filtered(
            "external_go_scoped",
            call_ref.qualified_hint.as_ref().map(|qualified_name| ExternalSymbolResolution {
                name: call_ref.callee.clone(),
                qualified_name: qualified_name.clone(),
                language: call_ref.language.clone(),
            }),
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
    matches!(
        call_ref.qualified_hint.as_deref(),
        Some("t.Helper" | "t.Fatalf" | "t.Run" | "b.Run" | "m.Run")
    )
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
        let go_var_types_by_file = HashMap::new();
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
            go_var_types_by_file: &go_var_types_by_file,
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
        let go_var_types_by_file = HashMap::new();
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
            go_var_types_by_file: &go_var_types_by_file,
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
        let go_var_types_by_file = HashMap::new();
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
            go_var_types_by_file: &go_var_types_by_file,
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
        let go_var_types_by_file = HashMap::new();
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
            go_var_types_by_file: &go_var_types_by_file,
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
        let go_var_types_by_file = HashMap::new();
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
            go_var_types_by_file: &go_var_types_by_file,
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
        let go_var_types_by_file = HashMap::new();
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
            go_var_types_by_file: &go_var_types_by_file,
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
                callee: "Run".into(),
                language: "go".into(),
                caller_filepath: "tests/smoke_test.go".into(),
                allow_same_file: true,
                kind: CallRefKind::Scoped,
                receiver_hint: Some("m".into()),
                qualified_hint: Some("m.Run".into()),
            },
        ) {
            CallResolution::Filtered(reason, None) => assert_eq!(reason, "go_test_receiver"),
            _ => panic!("expected go test receiver filter"),
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
        let go_var_types_by_file = HashMap::from([(
            "tests/test_apps/go/smoke_test.go".to_string(),
            HashMap::from([("registry".to_string(), "Registry".to_string())]),
        )]);
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
            go_var_types_by_file: &go_var_types_by_file,
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
}
