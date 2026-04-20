"""Convert tree-sitter-language-pack fixtures from the custom flat format to alef's format.

Usage: python scripts/convert_fixtures.py fixtures/
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


def convert_fixture(old: dict) -> dict:
    """Convert a single fixture from old format to alef format."""
    new: dict = {
        "id": old["id"],
        "description": old["description"],
    }

    # Category is derived from directory in alef, but we keep it explicit
    if "category" in old:
        new["category"] = old["category"]

    if "tags" in old:
        new["tags"] = old["tags"]

    # Convert skip conditions
    if "skip" in old and old["skip"]:
        skip = old["skip"]
        if "requires_language" in skip:
            new["skip"] = {"reason": f"Requires {skip['requires_language']} language"}

    assertions_obj = old.get("assertions", {})
    if not assertions_obj:
        assertions_obj = {}

    # Determine the call type and build input + assertions
    call, input_data, assertions = _classify_and_convert(old, assertions_obj)

    if call and call != "default":
        new["call"] = call
    if input_data:
        new["input"] = input_data
    if assertions:
        new["assertions"] = assertions

    return new


def _classify_and_convert(
    old: dict, a: dict
) -> tuple[str | None, dict | None, list[dict]]:
    """Determine call type and convert assertions."""
    assertions: list[dict] = []
    input_data: dict = {}
    call: str | None = None

    # --- Language detection fixtures ---
    if "detect_from_extension" in a:
        call = "detect_extension"
        input_data = {"extension": a["detect_from_extension"]}
        if "detect_result" in a:
            assertions.append({"type": "equals", "value": a["detect_result"]})
        if a.get("detect_result_none"):
            assertions.append({"type": "is_empty"})
        return call, input_data, assertions

    if "detect_from_path" in a:
        call = "detect_path"
        input_data = {"path": a["detect_from_path"]}
        if "detect_result" in a:
            assertions.append({"type": "equals", "value": a["detect_result"]})
        if a.get("detect_result_none"):
            assertions.append({"type": "is_empty"})
        return call, input_data, assertions

    if "detect_from_content" in a:
        call = "detect_content"
        input_data = {"content": a["detect_from_content"]}
        if "detect_result" in a:
            assertions.append({"type": "equals", "value": a["detect_result"]})
        if a.get("detect_result_none"):
            assertions.append({"type": "is_empty"})
        return call, input_data, assertions

    # --- Registry fixtures ---
    if "language_available" in a:
        call = "has_language"
        language = old.get("language", "python")
        input_data = {"language": language}
        assertions.append(
            {"type": "is_true" if a["language_available"] else "is_false"}
        )
        return call, input_data, assertions

    if a.get("languages_not_empty"):
        call = "list_languages"
        assertions.append({"type": "not_empty"})
        return call, input_data or None, assertions

    # --- Ambiguity fixtures ---
    if "ambiguity_extension" in a:
        call = "ambiguity"
        input_data = {"extension": a["ambiguity_extension"]}
        if "ambiguity_assigned" in a:
            assertions.append(
                {
                    "type": "equals",
                    "field": "assigned",
                    "value": a["ambiguity_assigned"],
                }
            )
        if "ambiguity_alternatives_contain" in a:
            assertions.append(
                {
                    "type": "contains",
                    "field": "alternatives",
                    "value": a["ambiguity_alternatives_contain"],
                }
            )
        if a.get("ambiguity_is_none"):
            assertions.append({"type": "is_empty"})
        return call, input_data, assertions

    # --- Highlights query fixtures ---
    if "highlights_query_not_empty" in a or "highlights_query_is_none" in a:
        call = "highlights"
        language = old.get("language", "python")
        input_data = {"language": language}
        if a.get("highlights_query_not_empty"):
            assertions.append({"type": "not_empty"})
        if a.get("highlights_query_is_none"):
            assertions.append({"type": "is_empty"})
        return call, input_data, assertions

    # --- Process fixtures (process_* assertions) ---
    has_process = any(k.startswith("process_") for k in a)
    if has_process:
        call = "default"
        language = old.get("language", "python")
        source_code = old.get("source_code", "")
        input_data = {"language": language, "source_code": source_code}

        if "process_language" in a:
            assertions.append(
                {"type": "equals", "field": "language", "value": a["process_language"]}
            )
        if "process_structure_count_min" in a:
            assertions.append(
                {
                    "type": "count_min",
                    "field": "structure",
                    "value": a["process_structure_count_min"],
                }
            )
        if "process_structure_contains_kind" in a:
            assertions.append(
                {
                    "type": "contains",
                    "field": "structure_kinds",
                    "value": a["process_structure_contains_kind"],
                }
            )
        if "process_structure_name_contains" in a:
            assertions.append(
                {
                    "type": "contains",
                    "field": "structure_names",
                    "value": a["process_structure_name_contains"],
                }
            )
        if "process_imports_count_min" in a:
            assertions.append(
                {
                    "type": "count_min",
                    "field": "imports",
                    "value": a["process_imports_count_min"],
                }
            )
        if "process_imports_contains_source" in a:
            assertions.append(
                {
                    "type": "contains",
                    "field": "import_sources",
                    "value": a["process_imports_contains_source"],
                }
            )
        if "process_exports_count_min" in a:
            assertions.append(
                {
                    "type": "count_min",
                    "field": "exports",
                    "value": a["process_exports_count_min"],
                }
            )
        if "process_comments_count_min" in a:
            assertions.append(
                {
                    "type": "count_min",
                    "field": "comments",
                    "value": a["process_comments_count_min"],
                }
            )
        if "process_metrics_total_lines_min" in a:
            assertions.append(
                {
                    "type": "greater_than_or_equal",
                    "field": "metrics.total_lines",
                    "value": a["process_metrics_total_lines_min"],
                }
            )
        if "process_metrics_code_lines_min" in a:
            assertions.append(
                {
                    "type": "greater_than_or_equal",
                    "field": "metrics.code_lines",
                    "value": a["process_metrics_code_lines_min"],
                }
            )
        if "process_metrics_comment_lines_min" in a:
            assertions.append(
                {
                    "type": "greater_than_or_equal",
                    "field": "metrics.comment_lines",
                    "value": a["process_metrics_comment_lines_min"],
                }
            )
        if "process_metrics_max_depth_min" in a:
            assertions.append(
                {
                    "type": "greater_than_or_equal",
                    "field": "metrics.max_depth",
                    "value": a["process_metrics_max_depth_min"],
                }
            )
        if "process_metrics_error_count" in a:
            assertions.append(
                {
                    "type": "equals",
                    "field": "metrics.error_count",
                    "value": a["process_metrics_error_count"],
                }
            )
        if a.get("process_diagnostics_not_empty"):
            assertions.append({"type": "not_empty", "field": "diagnostics"})
        if "process_chunk_count_min" in a:
            assertions.append(
                {
                    "type": "count_min",
                    "field": "chunks",
                    "value": a["process_chunk_count_min"],
                }
            )
        if "process_chunk_max_size" in a:
            assertions.append(
                {
                    "type": "less_than_or_equal",
                    "field": "max_chunk_size",
                    "value": a["process_chunk_max_size"],
                }
            )
        return call, input_data, assertions

    # --- Tree inspection fixtures ---
    has_tree_inspect = any(
        k
        in (
            "tree_root_node_type",
            "tree_error_count",
            "find_nodes_count_min",
            "named_children_count_min",
        )
        for k in a
    )
    if has_tree_inspect:
        call = "parse"
        language = old.get("language", "python")
        source_code = old.get("source_code", "")
        input_data = {"language": language, "source_code": source_code}

        if a.get("tree_not_null"):
            assertions.append({"type": "not_error"})
        if "tree_root_node_type" in a:
            assertions.append(
                {
                    "type": "method_result",
                    "method": "root_node_type",
                    "check": "equals",
                    "value": a["tree_root_node_type"],
                }
            )
        if "tree_error_count" in a:
            assertions.append(
                {
                    "type": "method_result",
                    "method": "error_count",
                    "check": "equals",
                    "value": a["tree_error_count"],
                }
            )
        if "find_nodes_count_min" in a:
            fncm = a["find_nodes_count_min"]
            assertions.append(
                {
                    "type": "method_result",
                    "method": "find_nodes_by_type",
                    "args": {"node_type": fncm["node_type"]},
                    "check": "count_min",
                    "value": fncm["min_count"],
                }
            )
        if "named_children_count_min" in a:
            assertions.append(
                {
                    "type": "method_result",
                    "method": "named_children_count",
                    "check": "greater_than_or_equal",
                    "value": a["named_children_count_min"],
                }
            )
        if "has_error_nodes" in a:
            assertions.append(
                {
                    "type": "method_result",
                    "method": "has_error_nodes",
                    "check": "equals",
                    "value": a["has_error_nodes"],
                }
            )
        return call, input_data, assertions

    # --- Parse/smoke fixtures (default: parse call) ---
    if "expect_error" in a and a["expect_error"]:
        call = "parse"
        language = old.get("language", "unknown")
        source_code = old.get("source_code", "")
        input_data = {"language": language, "source_code": source_code}
        assertions.append({"type": "error"})
        return call, input_data, assertions

    # Standard parse fixtures (smoke, parsing, error-handling)
    language = old.get("language")
    source_code = old.get("source_code")
    if language or source_code:
        call = "parse"
        input_data = {}
        if language:
            input_data["language"] = language
        if source_code is not None:
            input_data["source_code"] = source_code

        if a.get("tree_not_null"):
            assertions.append({"type": "not_error"})
        if "root_child_count_min" in a:
            assertions.append(
                {
                    "type": "greater_than_or_equal",
                    "field": "root_child_count",
                    "value": a["root_child_count_min"],
                }
            )
        if "root_contains_node_type" in a:
            assertions.append(
                {
                    "type": "method_result",
                    "method": "contains_node_type",
                    "args": {"node_type": a["root_contains_node_type"]},
                    "check": "is_true",
                }
            )
        if "has_error_nodes" in a:
            assertions.append(
                {
                    "type": "method_result",
                    "method": "has_error_nodes",
                    "check": "equals",
                    "value": a["has_error_nodes"],
                }
            )

        return call, input_data, assertions

    # Fallback: no recognizable pattern
    return None, None, []


def convert_file(path: Path) -> None:
    """Convert a single fixture file in-place."""
    content = path.read_text()
    data = json.loads(content)

    if isinstance(data, list):
        converted = [convert_fixture(f) for f in data]
    else:
        converted = convert_fixture(data)

    path.write_text(json.dumps(converted, indent="\t", ensure_ascii=False) + "\n")


def main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python scripts/convert_fixtures.py <fixtures_dir>")
        sys.exit(1)

    fixtures_dir = Path(sys.argv[1])
    if not fixtures_dir.is_dir():
        print(f"Not a directory: {fixtures_dir}")
        sys.exit(1)

    converted = 0
    for json_file in sorted(fixtures_dir.rglob("*.json")):
        if json_file.name == "schema.json":
            continue
        convert_file(json_file)
        converted += 1

    print(f"Converted {converted} fixture files")


if __name__ == "__main__":
    main()
