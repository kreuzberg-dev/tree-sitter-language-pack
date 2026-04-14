use std::collections::HashMap;

use crate::PythonFunctionReturnAssignment;

fn normalize_python_type_name(raw: &str) -> Option<String> {
    let mut text = raw.trim().trim_matches('"').trim_matches('\'').trim();
    if text.is_empty() {
        return None;
    }
    if let Some((head, _)) = text.split_once('|') {
        text = head.trim();
    }
    if let Some(base) = text.strip_suffix(" | None") {
        text = base.trim();
    }
    let base = text
        .split(['[', ',', ':'])
        .next()
        .unwrap_or(text)
        .trim()
        .trim_start_matches('*');
    let short = base.rsplit('.').next().unwrap_or(base).trim();
    let first = short.chars().next()?;
    if short.is_empty() || !first.is_uppercase() {
        return None;
    }
    Some(short.to_string())
}

fn infer_python_constructor_type(expr: &str) -> Option<String> {
    let open = expr.find('(')?;
    let callee = expr[..open].trim();
    normalize_python_type_name(callee)
}

fn infer_python_function_assignment(expr: &str) -> Option<String> {
    let open = expr.find('(')?;
    let callee = expr[..open].trim();
    if callee.is_empty() || callee.contains('.') {
        return None;
    }
    let first = callee.chars().next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    Some(callee.to_string())
}

pub(crate) fn parse_python_var_types(
    source: &str,
) -> (
    HashMap<String, String>,
    Vec<PythonFunctionReturnAssignment>,
    HashMap<String, String>,
) {
    let mut var_types = HashMap::new();
    let mut function_return_assignments = Vec::new();
    let mut function_return_types = HashMap::new();

    for raw_line in source.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix("def ")
            && let Some((name_part, after_name)) = rest.split_once('(')
            && let Some((_, return_part)) = after_name.rsplit_once("->")
        {
            let function_name = name_part.trim();
            let return_part = return_part.trim().trim_end_matches(':').trim();
            if !function_name.is_empty()
                && let Some(return_type) = normalize_python_type_name(return_part)
            {
                function_return_types.insert(function_name.to_string(), return_type);
            }
            continue;
        }

        if let Some((lhs, rhs)) = line.split_once('=') {
            let name = lhs.split(':').next().unwrap_or(lhs).trim().trim_end_matches(',').trim();
            if name.is_empty() || name.contains(' ') || name.contains(',') {
                continue;
            }
            let expr = rhs.trim();
            if let Some(ty) = infer_python_constructor_type(expr) {
                var_types.insert(name.to_string(), ty);
                continue;
            }
            if let Some(function_name) = infer_python_function_assignment(expr) {
                if let Some(return_type) = function_return_types.get(&function_name) {
                    var_types.insert(name.to_string(), return_type.clone());
                } else {
                    function_return_assignments.push(PythonFunctionReturnAssignment {
                        var_name: name.to_string(),
                        function_name,
                    });
                }
            }
        }
    }

    (var_types, function_return_assignments, function_return_types)
}

#[cfg(test)]
mod tests {
    use super::parse_python_var_types;

    #[test]
    fn parse_python_var_types_extracts_constructor_and_factory_types() {
        let source = r#"
class Parser:
    def parse(self, source: bytes): ...

def make_parser() -> Parser:
    return Parser()

parser = Parser()
other = make_parser()
"#;
        let (var_types, assignments, function_return_types) = parse_python_var_types(source);
        assert_eq!(var_types.get("parser").map(String::as_str), Some("Parser"));
        assert_eq!(var_types.get("other").map(String::as_str), Some("Parser"));
        assert!(assignments.is_empty());
        assert_eq!(
            function_return_types.get("make_parser").map(String::as_str),
            Some("Parser")
        );
    }

    #[test]
    fn parse_python_var_types_captures_unresolved_factory_assignments() {
        let source = r#"
parser = build_parser()
"#;
        let (var_types, assignments, function_return_types) = parse_python_var_types(source);
        assert!(var_types.is_empty());
        assert!(function_return_types.is_empty());
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0].var_name, "parser");
        assert_eq!(assignments[0].function_name, "build_parser");
    }
}
