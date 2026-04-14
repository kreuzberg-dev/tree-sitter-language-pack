use std::collections::HashMap;

fn normalize_rust_type_name(raw: &str) -> Option<String> {
    let mut text = raw.trim();
    if text.is_empty() {
        return None;
    }
    while let Some(stripped) = text.strip_prefix('&') {
        text = stripped.trim_start();
    }
    if let Some(stripped) = text.strip_prefix("mut ") {
        text = stripped.trim_start();
    }
    if let Some(start) = text.find('<') {
        let prefix = text[..start].trim();
        let inner = text[start + 1..].rsplit_once('>').map(|(inner, _)| inner.trim());
        if let Some(inner) = inner {
            if prefix.ends_with("LazyLock")
                || prefix.ends_with("OnceLock")
                || prefix.ends_with("Arc")
                || prefix.ends_with("Rc")
                || prefix.ends_with("Box")
            {
                text = inner;
            }
        }
    }
    let base = text.split('<').next().unwrap_or(text).trim();
    let simple = base.rsplit("::").next().unwrap_or(base).trim_start_matches('*').trim();
    if simple.is_empty() {
        None
    } else {
        Some(simple.to_string())
    }
}

pub(crate) fn parse_rust_var_types(source: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("static ")
            .or_else(|| trimmed.strip_prefix("pub static "))
            .or_else(|| trimmed.strip_prefix("pub(crate) static "))
            .or_else(|| trimmed.strip_prefix("const "))
            .or_else(|| trimmed.strip_prefix("pub const "))
            .or_else(|| trimmed.strip_prefix("pub(crate) const "))
        {
            if let Some((name_part, ty_part)) = rest.split_once(':') {
                let name = name_part.trim().trim_start_matches("mut ").trim();
                if !name.is_empty() && name.chars().all(|ch| ch.is_ascii_uppercase() || ch == '_') {
                    if let Some(ty) = normalize_rust_type_name(ty_part.split('=').next().unwrap_or(ty_part)) {
                        out.insert(name.to_string(), ty);
                    }
                }
            }
            continue;
        }
        if let Some(rest) = trimmed
            .strip_prefix("let ")
            .or_else(|| trimmed.strip_prefix("let mut "))
        {
            if let Some((name_part, ty_part)) = rest.split_once(':') {
                let name = name_part.trim();
                if !name.is_empty() {
                    if let Some(ty) = normalize_rust_type_name(ty_part.split('=').next().unwrap_or(ty_part)) {
                        out.insert(name.to_string(), ty);
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::parse_rust_var_types;

    #[test]
    fn parse_rust_var_types_extracts_static_and_let_types() {
        let source = r#"
use std::sync::LazyLock;

static REGISTRY: LazyLock<LanguageRegistry> = LazyLock::new(LanguageRegistry::new);
let registry: LanguageRegistry = LanguageRegistry::new();
"#;
        let vars = parse_rust_var_types(source);
        assert_eq!(vars.get("REGISTRY").map(String::as_str), Some("LanguageRegistry"));
        assert_eq!(vars.get("registry").map(String::as_str), Some("LanguageRegistry"));
    }
}
