use std::collections::{HashMap, HashSet};

use tree_sitter_language_pack as ts_pack;

pub(crate) fn normalize_swift_type(raw: &str) -> Option<String> {
    let mut s = raw.trim().trim_end_matches('?').trim_end_matches('!').to_string();
    if let Some(idx) = s.find('<') {
        s.truncate(idx);
    }
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub(crate) fn parse_swift_var_types(source: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let resolve_known_type = |candidate: String, map: &HashMap<String, String>| -> String {
        map.get(&candidate).cloned().unwrap_or(candidate)
    };
    let extract_type_from_rhs = |rhs: &str| -> Option<String> {
        let mut rhs = rhs.trim();
        if rhs.is_empty() {
            return None;
        }
        if let Some(idx) = rhs.find(" as? ") {
            rhs = rhs[idx + 5..].trim();
        } else if let Some(idx) = rhs.find(" as ") {
            rhs = rhs[idx + 4..].trim();
        }
        let mut ty = String::new();
        for ch in rhs.chars() {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '<' || ch == '>' || ch == '?' || ch == '!' {
                ty.push(ch);
            } else {
                break;
            }
        }
        if let Some(tn) = normalize_swift_type(&ty) {
            if let Some((head, _)) = tn.split_once('.') {
                return normalize_swift_type(head);
            }
            return Some(tn);
        }
        None
    };

    let extract_receiver_from_chain = |rhs: &str| -> Option<String> {
        let mut rhs = rhs.trim();
        if rhs.is_empty() {
            return None;
        }
        if let Some(idx) = rhs.find(" as? ") {
            rhs = rhs[..idx].trim();
        } else if let Some(idx) = rhs.find(" as ") {
            rhs = rhs[..idx].trim();
        }
        let mut name = String::new();
        for ch in rhs.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                name.push(ch);
            } else {
                break;
            }
        }
        if name.is_empty() { None } else { Some(name) }
    };

    for line in source.lines() {
        let trimmed = line.trim();
        let (kw, rest) = if let Some(r) = trimmed.strip_prefix("let ") {
            ("let", r)
        } else if let Some(r) = trimmed.strip_prefix("var ") {
            ("var", r)
        } else if let Some(r) = trimmed.strip_prefix("if let ") {
            ("if let", r)
        } else if let Some(r) = trimmed.strip_prefix("guard let ") {
            ("guard let", r)
        } else {
            if trimmed.contains('=')
                && !trimmed.contains("==")
                && !trimmed.contains("!=")
                && !trimmed.contains(">=")
                && !trimmed.contains("<=")
            {
                if let Some(eq_idx) = trimmed.find('=') {
                    let lhs = trimmed[..eq_idx].trim();
                    let rhs = trimmed[eq_idx + 1..].trim();
                    let mut name = String::new();
                    for ch in lhs.chars().rev() {
                        if ch.is_alphanumeric() || ch == '_' {
                            name.push(ch);
                        } else if !name.is_empty() {
                            break;
                        }
                    }
                    let name = name.chars().rev().collect::<String>();
                    if !name.is_empty() {
                        if let Some(tn) = extract_type_from_rhs(rhs) {
                            map.insert(name, resolve_known_type(tn, &map));
                        } else if rhs.contains('.') {
                            if let Some(recv) = extract_receiver_from_chain(rhs) {
                                if let Some(tn) = map.get(&recv).cloned() {
                                    map.insert(name, tn);
                                }
                            }
                        }
                    }
                }
            }
            continue;
        };
        let rest = rest.trim();
        if rest.is_empty() {
            continue;
        }

        let mut name = String::new();
        for ch in rest.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                name.push(ch);
            } else {
                break;
            }
        }
        if name.is_empty() {
            continue;
        }

        if let Some(idx) = rest.find(':') {
            let type_part = rest[idx + 1..].trim();
            let mut ty = String::new();
            for ch in type_part.chars() {
                if ch.is_alphanumeric() || ch == '_' || ch == '.' || ch == '<' || ch == '>' || ch == '?' || ch == '!' {
                    ty.push(ch);
                } else {
                    break;
                }
            }
            if let Some(tn) = normalize_swift_type(&ty) {
                map.insert(name, tn);
            }
            continue;
        }

        if let Some(eq_idx) = rest.find('=') {
            let rhs = rest[eq_idx + 1..].trim();
            if let Some(tn) = extract_type_from_rhs(rhs) {
                map.insert(name, resolve_known_type(tn, &map));
            } else if rhs.contains('.') {
                if let Some(recv) = extract_receiver_from_chain(rhs) {
                    if let Some(tn) = map.get(&recv).cloned() {
                        map.insert(name, tn);
                    }
                }
            }
        }

        let _ = kw;
    }
    map
}

pub(crate) fn collect_swift_extensions(
    items: &[ts_pack::StructureItem],
    map: &mut HashMap<String, HashSet<String>>,
) {
    for item in items {
        if item.kind == ts_pack::StructureKind::Extension {
            if let Some(type_name) = item.name.as_ref().and_then(|n| normalize_swift_type(n)) {
                let entry = map.entry(type_name).or_default();
                for child in &item.children {
                    if matches!(
                        child.kind,
                        ts_pack::StructureKind::Method | ts_pack::StructureKind::Function
                    ) {
                        if let Some(name) = child.name.as_ref() {
                            entry.insert(name.clone());
                        }
                    }
                }
            }
        }
        if !item.children.is_empty() {
            collect_swift_extensions(&item.children, map);
        }
    }
}

pub(crate) fn collect_swift_extension_spans(
    items: &[ts_pack::StructureItem],
    spans: &mut Vec<(usize, usize, String)>,
) {
    for item in items {
        if item.kind == ts_pack::StructureKind::Extension {
            if let Some(type_name) = item.name.as_ref().and_then(|n| normalize_swift_type(n)) {
                spans.push((item.span.start_byte, item.span.end_byte, type_name));
            }
        }
        if !item.children.is_empty() {
            collect_swift_extension_spans(&item.children, spans);
        }
    }
}

pub(crate) fn collect_swift_type_spans(
    items: &[ts_pack::StructureItem],
    spans: &mut Vec<(usize, usize, String)>,
) {
    for item in items {
        if matches!(
            item.kind,
            ts_pack::StructureKind::Class
                | ts_pack::StructureKind::Struct
                | ts_pack::StructureKind::Enum
                | ts_pack::StructureKind::Protocol
        ) {
            if let Some(name) = item.name.as_ref().and_then(|n| normalize_swift_type(n)) {
                spans.push((item.span.start_byte, item.span.end_byte, name));
            }
        }
        if !item.children.is_empty() {
            collect_swift_type_spans(&item.children, spans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_swift_types() {
        assert_eq!(normalize_swift_type("Service?"), Some("Service".to_string()));
        assert_eq!(normalize_swift_type("App.Service<String>"), Some("App.Service".to_string()));
        assert_eq!(normalize_swift_type(""), None);
    }

    #[test]
    fn parses_swift_variable_types_from_annotations_and_assignments() {
        let source = r#"
        let client: ApiClient = ApiClient()
        let task = ServiceTask()
        let nested = client.run()
        var response = task.execute()
        "#;

        let vars = parse_swift_var_types(source);
        assert_eq!(vars.get("client"), Some(&"ApiClient".to_string()));
        assert_eq!(vars.get("task"), Some(&"ServiceTask".to_string()));
        assert_eq!(vars.get("nested"), Some(&"ApiClient".to_string()));
        assert_eq!(vars.get("response"), Some(&"ServiceTask".to_string()));
    }
}
