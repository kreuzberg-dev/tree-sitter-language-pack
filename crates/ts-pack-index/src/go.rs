use std::collections::HashMap;

fn infer_ctor_return_type(expr: &str) -> Option<String> {
    let open = expr.find('(')?;
    let callee = expr[..open].trim();
    let base = callee.rsplit('.').next().unwrap_or(callee).trim();
    let stripped = base.strip_prefix("New")?;
    let mut chars = stripped.chars();
    let first = chars.next()?;
    if !first.is_uppercase() {
        return None;
    }
    Some(format!("{first}{}", chars.as_str()))
}

pub(crate) fn parse_go_var_types(source: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for raw_line in source.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if let Some((lhs, rhs)) = line.split_once(":=") {
            let names: Vec<&str> = lhs
                .split(',')
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .collect();
            let exprs: Vec<&str> = rhs
                .split(',')
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .collect();
            if let Some(name) = names.first().copied()
                && let Some(expr) = exprs.first().copied()
                && let Some(ty) = infer_ctor_return_type(expr)
            {
                out.insert(name.to_string(), ty);
            }
            continue;
        }

        if let Some(stripped) = line.strip_prefix("var ")
            && let Some((lhs, rhs)) = stripped.split_once('=')
        {
            let name = lhs.split_whitespace().next().unwrap_or("").trim();
            let expr = rhs.trim();
            if !name.is_empty()
                && let Some(ty) = infer_ctor_return_type(expr)
            {
                out.insert(name.to_string(), ty);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::parse_go_var_types;

    #[test]
    fn infers_go_short_var_constructor_types() {
        let source = r#"
            registry, err := tslp.NewRegistry()
            _, _ = registry, err
        "#;
        let vars = parse_go_var_types(source);
        assert_eq!(vars.get("registry").map(String::as_str), Some("Registry"));
    }

    #[test]
    fn infers_go_var_assignment_constructor_types() {
        let source = r#"
            var registry = tslp.NewRegistry()
        "#;
        let vars = parse_go_var_types(source);
        assert_eq!(vars.get("registry").map(String::as_str), Some("Registry"));
    }
}
