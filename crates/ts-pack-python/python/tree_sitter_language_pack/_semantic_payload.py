from __future__ import annotations

import hashlib
import re
from copy import deepcopy
from typing import Any

from . import extract_file_facts, process, ProcessConfig

_TS_QUERYRAW_TAGGED_TEMPLATE_RE = re.compile(r"\$queryRaw(?:Unsafe)?\s*<.+>\s*`")

_EXTRACTIONS_BY_LANG = {
    "python": {
        "calls": {
            "query": "(call function: (identifier) @name)",
            "capture_output": "Text",
        },
        "decorators": {
            "query": "(decorator (identifier) @name)",
            "capture_output": "Text",
        },
    },
    "javascript": {
        "calls": {
            "query": "(call_expression function: (identifier) @name)",
            "capture_output": "Text",
        },
        "decorators": {
            "query": "(decorator (identifier) @name)",
            "capture_output": "Text",
        },
    },
    "typescript": {
        "calls": {
            "query": "(call_expression function: (identifier) @name)",
            "capture_output": "Text",
        },
        "decorators": {
            "query": "(decorator (identifier) @name)",
            "capture_output": "Text",
        },
    },
    "tsx": {
        "calls": {
            "query": "(call_expression function: (identifier) @name)",
            "capture_output": "Text",
        },
        "decorators": {
            "query": "(decorator (identifier) @name)",
            "capture_output": "Text",
        },
    },
}


def _chunk_id(project_id: str, file_path: str, start_byte: int, text: str, version: str) -> str:
    digest = hashlib.sha256(f"{file_path}:{start_byte}:{text}".encode()).hexdigest()[:14]
    return f"{project_id}:{version}:{file_path}:{digest}"


def _chunk_id_with_header(
    project_id: str,
    file_path: str,
    start_byte: int,
    text: str,
    version: str,
) -> str:
    return _chunk_id(project_id, file_path, start_byte, text, version)


def _compact_list(items: list, limit: int) -> list:
    if not items:
        return []
    return items[:limit] if len(items) > limit else items


def _compact_imports(imports: list) -> list:
    out: list = []
    for item in imports or []:
        source = item.get("source") or item.get("module")
        names = item.get("names") or []
        if source:
            out.append({"source": source, "names": _compact_list(names, 10)})
    return _compact_list(out, 80)


def _compact_exports(exports: list) -> list:
    out: list = []
    for item in exports or []:
        if not isinstance(item, dict):
            continue
        name = item.get("name")
        if name:
            out.append({"name": name, "kind": item.get("kind")})
    return _compact_list(out, 80)


def _compact_symbols(symbols: list) -> list:
    unique: list[str] = []
    seen: set[str] = set()
    for item in symbols or []:
        if isinstance(item, dict):
            name = item.get("name") or item.get("symbol") or item.get("text")
        else:
            name = item if isinstance(item, str) else None
        if not name or name in seen:
            continue
        seen.add(name)
        unique.append(name)
        if len(unique) >= 200:
            break
    return unique


def _compact_diagnostics(diagnostics: list) -> dict:
    if not diagnostics:
        return {"count": 0, "items": []}
    items = []
    for diag in diagnostics[:10]:
        items.append(
            {
                "message": diag.get("message"),
                "start_line": diag.get("start_line") or diag.get("span", {}).get("start_row"),
                "start_col": diag.get("start_col") or diag.get("span", {}).get("start_col"),
            }
        )
    return {"count": len(diagnostics), "items": items}


def _extract_metrics(metrics: dict) -> dict:
    if not metrics:
        return {}
    return {
        "total_lines": metrics.get("total_lines") or metrics.get("totalLines"),
        "code_lines": metrics.get("code_lines") or metrics.get("codeLines"),
        "comment_lines": metrics.get("comment_lines") or metrics.get("commentLines"),
        "blank_lines": metrics.get("blank_lines") or metrics.get("blankLines"),
        "error_count": metrics.get("error_count") or metrics.get("errorCount"),
        "node_count": metrics.get("node_count") or metrics.get("nodeCount"),
        "max_depth": metrics.get("max_depth") or metrics.get("maxDepth"),
        "total_bytes": metrics.get("total_bytes") or metrics.get("totalBytes"),
    }


def _compact_extractions(extractions: dict) -> dict:
    if not isinstance(extractions, dict):
        return {}
    out: dict = {}
    for name, payload in extractions.items():
        matches = payload.get("matches") if isinstance(payload, dict) else None
        if not isinstance(matches, list):
            continue
        values: list[str] = []
        for match in matches[:50]:
            for capture in match.get("captures") or []:
                text = capture.get("text")
                if text:
                    values.append(str(text))
        if values:
            out[name] = _compact_list(values, 50)
    return out


def _normalize_ts_pack_result(source: str, lang: str | None, result: dict | None) -> dict:
    if not result:
        return {}
    normalized = deepcopy(result)
    diagnostics = list(normalized.get("diagnostics") or [])
    if not diagnostics or lang not in {"typescript", "tsx"}:
        return normalized

    lines = source.splitlines()
    filtered = []
    dropped = 0
    for diag in diagnostics:
        message = str(diag.get("message") or "")
        span = diag.get("span") or {}
        line_idx = span.get("start_line")
        line_text = lines[line_idx] if isinstance(line_idx, int) and 0 <= line_idx < len(lines) else ""
        if message == "Missing expected node: !" and _TS_QUERYRAW_TAGGED_TEMPLATE_RE.search(line_text):
            dropped += 1
            continue
        filtered.append(diag)

    if dropped:
        normalized["diagnostics"] = filtered
        metrics = dict(normalized.get("metrics") or {})
        if "error_count" in metrics and isinstance(metrics["error_count"], int):
            metrics["error_count"] = max(0, metrics["error_count"] - dropped)
        if "errorCount" in metrics and isinstance(metrics["errorCount"], int):
            metrics["errorCount"] = max(0, metrics["errorCount"] - dropped)
        normalized["metrics"] = metrics
    return normalized


def _build_process_config(language: str, *, chunk_max_size: int, chunk_overlap: int):
    kwargs = {
        "structure": True,
        "imports": True,
        "exports": True,
        "comments": True,
        "docstrings": True,
        "symbols": True,
        "diagnostics": True,
    }
    if language != "swift":
        kwargs["chunk_max_size"] = chunk_max_size
        kwargs["chunk_overlap"] = chunk_overlap
        if language in _EXTRACTIONS_BY_LANG:
            kwargs["extractions"] = _EXTRACTIONS_BY_LANG[language]
    try:
        return ProcessConfig(language, **kwargs)
    except TypeError:
        kwargs.pop("chunk_overlap", None)
        kwargs.pop("extractions", None)
        return ProcessConfig(language, **kwargs)


def build_line_window_chunks(
    source: str,
    file_path: str,
    project_id: str,
    *,
    language: str | None = None,
    file_meta: dict[str, Any] | None = None,
    chunk_id_version: str = "v6",
    chunk_lines: int = 60,
    overlap_lines: int = 10,
) -> list[dict[str, Any]]:
    file_header = f"// File: {file_path}\n"
    chunks: list[dict[str, Any]] = []
    lines = source.splitlines()
    i = 0
    metadata_base = dict(file_meta or {})
    while i < len(lines):
        block = lines[i : i + chunk_lines]
        if not block:
            break
        text = file_header + "\n".join(block)
        chunks.append(
            {
                "ref_id": _chunk_id_with_header(project_id, file_path, i, text, chunk_id_version),
                "text": text,
                "metadata": {
                    "file": file_path,
                    "project_id": project_id,
                    "language": language,
                    **metadata_base,
                },
            }
        )
        i += chunk_lines - overlap_lines
    return chunks


def build_swift_chunks(
    source: str,
    file_path: str,
    project_id: str,
    *,
    file_meta: dict[str, Any] | None = None,
    chunk_id_version: str = "v6",
    chunk_max_size: int = 4000,
    chunk_lines: int = 60,
    overlap_lines: int = 10,
) -> list[dict[str, Any]]:
    from . import get_parser

    member_types = {
        "property_declaration",
        "function_declaration",
        "subscript_declaration",
        "typealias_declaration",
        "init_declaration",
        "deinit_declaration",
        "protocol_function_declaration",
        "protocol_property_declaration",
        "enum_entry",
    }
    container_types = {
        "class_declaration",
        "struct_declaration",
        "enum_declaration",
        "protocol_declaration",
        "extension_declaration",
    }

    try:
        parser = get_parser("swift")
        src_b = source.encode("utf-8")
        tree = parser.parse(src_b)
    except Exception:
        return []

    file_header = f"// File: {file_path}\n"
    metadata_base = dict(file_meta or {})
    chunks: list[dict[str, Any]] = []

    def _name_of(node) -> str:
        for child in node.children:
            if child.type == "pattern":
                return src_b[child.start_byte : child.end_byte].decode("utf-8", errors="replace")
            if child.type in ("simple_identifier", "type_identifier"):
                return src_b[child.start_byte : child.end_byte].decode("utf-8", errors="replace")
        return ""

    def _append_chunk(
        text: str,
        start_byte: int,
        name: str,
        start_line: int,
        end_line: int,
        context_path: list[str],
    ) -> None:
        if not text:
            return
        chunks.append(
            {
                "ref_id": _chunk_id(project_id, file_path, start_byte, text, chunk_id_version),
                "text": file_header + text,
                "metadata": {
                    "file": file_path,
                    "project_id": project_id,
                    "language": "swift",
                    "symbols": [name] if name else [],
                    "start_line": start_line,
                    "end_line": end_line,
                    "context_path": context_path,
                    **metadata_base,
                },
            }
        )

    def _emit_text(
        text: str,
        start_byte: int,
        name: str,
        start_line: int,
        end_line: int,
        context_path: list[str],
    ) -> None:
        text = text.strip()
        if not text:
            return
        if len(text.encode("utf-8")) <= chunk_max_size:
            _append_chunk(text, start_byte, name, start_line, end_line, context_path)
            return

        lines = text.splitlines()
        i = 0
        while i < len(lines):
            block = "\n".join(lines[i : i + chunk_lines]).strip()
            if block:
                _append_chunk(
                    block,
                    start_byte + i,
                    name,
                    start_line + i,
                    start_line + min(i + chunk_lines, len(lines)) - 1,
                    context_path,
                )
            i += chunk_lines - overlap_lines

    def _walk(node, context_path: list[str]) -> None:
        if node.type in member_types:
            name = _name_of(node)
            text = src_b[node.start_byte : node.end_byte].decode("utf-8", errors="replace")
            next_context = context_path + ([name] if name else [])
            _emit_text(
                text,
                node.start_byte,
                name,
                node.start_point[0] + 1,
                node.end_point[0] + 1,
                next_context,
            )
            return

        if node.type in container_types:
            name = _name_of(node)
            next_context = context_path + ([name] if name else [])
            for child in node.children:
                _walk(child, next_context)
            return

        for child in node.children:
            _walk(child, context_path)

    _walk(tree.root_node, [])
    return chunks


def build_semantic_payload(
    source: str,
    language: str,
    file_path: str,
    project_id: str,
    *,
    chunk_id_version: str = "v6",
    chunk_max_size: int = 4000,
    chunk_overlap: int = 200,
) -> dict[str, Any]:
    config = _build_process_config(language, chunk_max_size=chunk_max_size, chunk_overlap=chunk_overlap)
    result = _normalize_ts_pack_result(source, language, process(source, config))
    file_facts = extract_file_facts(source, language, file_path)
    file_meta = {
        "file_imports": _compact_imports(result.get("imports", [])),
        "file_exports": _compact_exports(result.get("exports", [])),
        "file_symbols": _compact_symbols(result.get("symbols", [])),
        "file_diagnostics": _compact_diagnostics(result.get("diagnostics", [])),
        "file_metrics": _extract_metrics(result.get("metrics", {})),
        "file_extractions": _compact_extractions(result.get("extractions", {})),
    }
    if file_facts:
        file_meta["file_facts"] = file_facts

    file_header = f"// File: {file_path}\n"
    chunks: list[dict[str, Any]] = []
    for chunk in result.get("chunks", []):
        cmeta = chunk.get("metadata", {}) or {}
        if cmeta.get("has_error_nodes"):
            continue
        content = chunk.get("content", "")
        if not content or not content.strip():
            continue
        chunks.append(
            {
                "ref_id": _chunk_id(project_id, file_path, chunk.get("start_byte", 0), content, chunk_id_version),
                "text": file_header + content,
                "metadata": {
                    "file": file_path,
                    "project_id": project_id,
                    "language": language,
                    "symbols": cmeta.get("symbols_defined", []),
                    "start_line": chunk.get("start_line", 0) + 1,
                    "end_line": chunk.get("end_line", 0) + 1,
                    "docstrings": cmeta.get("docstrings", []),
                    "context_path": cmeta.get("context_path", []),
                    "node_types": cmeta.get("node_types", []),
                    "comments": cmeta.get("comments", []),
                    "has_error_nodes": bool(cmeta.get("has_error_nodes")),
                    **file_meta,
                },
            }
        )
    return {"result": result, "file_meta": file_meta, "chunks": chunks}
