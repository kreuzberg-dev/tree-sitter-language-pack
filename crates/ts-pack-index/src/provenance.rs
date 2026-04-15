use serde_json::{Value, json};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::tags;

const MAX_PARSE_PROVENANCE_FILES: usize = 400;
const MAX_PARSE_PROVENANCE_SAMPLES: usize = 60;

fn env_trimmed(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn contains_normalized(haystack: &str, needle: &str) -> bool {
    haystack.to_ascii_lowercase().contains(needle)
}

fn symbol_filter() -> Option<String> {
    env_trimmed("TS_PACK_DEBUG_PROVENANCE_SYMBOL").map(|value| normalize(&value))
}

fn file_filter() -> Option<String> {
    env_trimmed("TS_PACK_DEBUG_PROVENANCE_FILE").map(|value| normalize(&value.replace('\\', "/")))
}

pub fn normalize_filter(value: Option<&str>) -> Option<String> {
    value.map(normalize).filter(|value| !value.is_empty())
}

pub fn provenance_enabled() -> bool {
    symbol_filter().is_some() || file_filter().is_some()
}

pub fn call_matches(
    caller_filepath: &str,
    callee: &str,
    qualified_hint: Option<&str>,
    receiver_hint: Option<&str>,
) -> bool {
    let symbol_match = symbol_filter().is_none_or(|needle| {
        contains_normalized(callee, &needle)
            || qualified_hint.is_some_and(|value| contains_normalized(value, &needle))
            || receiver_hint.is_some_and(|value| contains_normalized(value, &needle))
    });
    let file_match =
        file_filter().is_none_or(|needle| contains_normalized(&caller_filepath.replace('\\', "/"), &needle));
    symbol_match && file_match
}

pub fn file_pair_matches(src_filepath: &str, dst_filepath: &str) -> bool {
    file_filter().is_none_or(|needle| {
        contains_normalized(&src_filepath.replace('\\', "/"), &needle)
            || contains_normalized(&dst_filepath.replace('\\', "/"), &needle)
    })
}

pub fn emit(stage: &str, event: &str, fields: &[(&str, String)]) {
    if !provenance_enabled() {
        return;
    }
    let mut parts = vec![format!("stage={stage}"), format!("event={event}")];
    for (key, value) in fields {
        parts.push(format!("{key}={value:?}"));
    }
    eprintln!("[ts-pack-provenance] {}", parts.join(" "));
}

fn should_skip_dir_name(name: &str) -> bool {
    matches!(
        name,
        ".git" | "node_modules" | "target" | ".venv" | "venv" | "dist" | "build" | ".next" | ".runtime"
    )
}

fn kind_for_call_site(site: &tags::CallSite) -> &'static str {
    if site.receiver.is_some() {
        "member"
    } else if site.qualified_callee.is_some() {
        "scoped"
    } else {
        "plain"
    }
}

pub fn collect_parse_provenance_samples(
    project_path: &Path,
    symbol_filter: Option<&str>,
    file_filter: Option<&str>,
) -> Vec<Value> {
    let normalized_symbol = normalize_filter(symbol_filter);
    let normalized_file_value = file_filter.map(|value| value.replace('\\', "/"));
    let normalized_file = normalize_filter(normalized_file_value.as_deref());
    let mut file_paths: Vec<PathBuf> = WalkDir::new(project_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            if !entry.file_type().is_dir() {
                return true;
            }
            entry
                .file_name()
                .to_str()
                .map(|name| !should_skip_dir_name(name))
                .unwrap_or(true)
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
        .collect();
    file_paths.sort();

    let mut samples = Vec::new();
    let mut files_scanned = 0usize;
    for abs_path in file_paths {
        if files_scanned >= MAX_PARSE_PROVENANCE_FILES || samples.len() >= MAX_PARSE_PROVENANCE_SAMPLES {
            break;
        }
        let Ok(rel_path) = abs_path.strip_prefix(project_path) else {
            continue;
        };
        let rel_path = rel_path.to_string_lossy().replace('\\', "/");
        if normalized_file
            .as_deref()
            .is_some_and(|needle| !contains_normalized(&rel_path, needle))
        {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(&abs_path) else {
            continue;
        };
        if source.len() > 1_000_000 {
            continue;
        }
        let Some(lang_name) = tree_sitter_language_pack::detect_language_from_path(&rel_path) else {
            continue;
        };
        let Ok(tree) = tree_sitter_language_pack::parse_string(lang_name, source.as_bytes()) else {
            continue;
        };
        let Some(tags_result) = tags::run_tags(lang_name, &tree, source.as_bytes(), &rel_path, None) else {
            continue;
        };
        files_scanned += 1;
        for site in tags_result.call_sites {
            let symbol_match = normalized_symbol.as_deref().is_none_or(|needle| {
                contains_normalized(&site.callee, needle)
                    || site
                        .qualified_callee
                        .as_deref()
                        .is_some_and(|value| contains_normalized(value, needle))
                    || site
                        .receiver
                        .as_deref()
                        .is_some_and(|value| contains_normalized(value, needle))
            });
            if !symbol_match {
                continue;
            }
            samples.push(json!({
                "caller_filepath": rel_path,
                "callee": site.callee,
                "kind": kind_for_call_site(&site),
                "receiver_hint": site.receiver.unwrap_or_default(),
                "qualified_hint": site.qualified_callee.unwrap_or_default(),
            }));
            if samples.len() >= MAX_PARSE_PROVENANCE_SAMPLES {
                break;
            }
        }
    }
    samples
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn file_pair_filter_matches_either_side() {
        let _guard = env_guard().lock().unwrap();
        unsafe {
            std::env::set_var("TS_PACK_DEBUG_PROVENANCE_FILE", "src/main.rs");
            std::env::remove_var("TS_PACK_DEBUG_PROVENANCE_SYMBOL");
        }
        assert!(file_pair_matches("src/main.rs", "src/lib.rs"));
        assert!(file_pair_matches("src/lib.rs", "src/main.rs"));
        assert!(!file_pair_matches("src/lib.rs", "src/bin.rs"));
        unsafe {
            std::env::remove_var("TS_PACK_DEBUG_PROVENANCE_FILE");
        }
    }

    #[test]
    fn call_filter_matches_symbol_and_file() {
        let _guard = env_guard().lock().unwrap();
        unsafe {
            std::env::set_var("TS_PACK_DEBUG_PROVENANCE_FILE", "src/main.rs");
            std::env::set_var("TS_PACK_DEBUG_PROVENANCE_SYMBOL", "process");
        }
        assert!(call_matches(
            "src/main.rs",
            "process",
            Some("crate::registry::process"),
            Some("registry"),
        ));
        assert!(!call_matches("src/lib.rs", "process", None, None));
        assert!(!call_matches("src/main.rs", "render", None, None));
        unsafe {
            std::env::remove_var("TS_PACK_DEBUG_PROVENANCE_FILE");
            std::env::remove_var("TS_PACK_DEBUG_PROVENANCE_SYMBOL");
        }
    }
}
