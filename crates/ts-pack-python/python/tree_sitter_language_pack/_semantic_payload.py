from __future__ import annotations

import hashlib
import re
from copy import deepcopy
from typing import Any
import json

from . import extract_file_facts, process, ProcessConfig
from . import _native as _native

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

_DECLARATION_ANCHOR_RADIUS = 20
_MAX_DECLARATION_ANCHORS = 6
_DECLARATION_PATTERNS: list[tuple[re.Pattern[str], str]] = [
    (re.compile(r"^\s*@interface\s+([A-Za-z_][A-Za-z0-9_]*)\b"), "type"),
    (re.compile(r"^\s*@implementation\s+([A-Za-z_][A-Za-z0-9_]*)\b"), "type"),
    (re.compile(r"^\s*pub\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("), "function"),
    (re.compile(r"^\s*fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("), "function"),
    (re.compile(r"^\s*struct\s+([A-Za-z_][A-Za-z0-9_]*)\b"), "type"),
    (re.compile(r"^\s*enum\s+([A-Za-z_][A-Za-z0-9_]*)\b"), "type"),
    (re.compile(r"^\s*trait\s+([A-Za-z_][A-Za-z0-9_]*)\b"), "type"),
    (re.compile(r"^\s*mod\s+([A-Za-z_][A-Za-z0-9_]*)\b"), "module"),
    (re.compile(r"^\s*(?:async\s+)?def\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("), "function"),
    (re.compile(r"^\s*class\s+([A-Za-z_][A-Za-z0-9_]*)\b"), "type"),
    (
        re.compile(
            r"^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:public|open|internal|fileprivate|private|final)?\s*class\s+([A-Za-z_][A-Za-z0-9_]*)\b"
        ),
        "type",
    ),
    (
        re.compile(
            r"^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:public|open|internal|fileprivate|private|final)?\s*struct\s+([A-Za-z_][A-Za-z0-9_]*)\b"
        ),
        "type",
    ),
    (
        re.compile(
            r"^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:public|open|internal|fileprivate|private|final|indirect)?\s*enum\s+([A-Za-z_][A-Za-z0-9_]*)\b"
        ),
        "type",
    ),
    (
        re.compile(
            r"^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:public|open|internal|fileprivate|private)?\s*protocol\s+([A-Za-z_][A-Za-z0-9_]*)\b"
        ),
        "type",
    ),
    (
        re.compile(
            r"^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:public|open|internal|fileprivate|private)?\s*extension\s+([A-Za-z_][A-Za-z0-9_<>.]*)\b"
        ),
        "type",
    ),
    (
        re.compile(
            r"^\s*(?:@\w+(?:\([^)]*\))?\s+)*(?:public|open|internal|fileprivate|private)?\s*typealias\s+([A-Za-z_][A-Za-z0-9_]*)\b"
        ),
        "type",
    ),
    (
        re.compile(
            r"^\s*export\s+(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("
        ),
        "function",
    ),
    (re.compile(r"^\s*function\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("), "function"),
]
_DECLARATION_NODE_TYPES = {
    "function_definition",
    "function_declaration",
    "function_item",
    "method_definition",
    "method_declaration",
    "class_definition",
    "class_declaration",
    "struct_item",
    "enum_item",
    "trait_item",
    "module",
    "mod_item",
    "impl_item",
}
_CALLSITE_NODE_TYPES = {
    "call",
    "call_expression",
    "invocation_expression",
    "method_call_expression",
    "await_expression",
    "expression_statement",
}
_PATH_LIKE_CHUNK_ROLES: list[tuple[str, str]] = [
    ("/tests/", "test_usage"),
    ("/test/", "test_usage"),
    ("/spec/", "test_usage"),
    ("/__tests__/", "test_usage"),
    ("/e2e/", "test_usage"),
    ("/examples/", "example_usage"),
    ("/example/", "example_usage"),
    ("examples/", "example_usage"),
    ("example/", "example_usage"),
]
_SUPPORT_PATH_SEGMENTS = {
    "/scripts/",
    "/tools/",
    "/vendor/",
    "/generated/",
    "/node_modules/",
    "/dist/",
    "/release/",
}
_FALLBACK_EXTS = {
    "yaml",
    "yml",
    "toml",
    "json",
    "pbxproj",
    "xcscheme",
    "xcworkspacedata",
    "plist",
    "md",
    "txt",
    "sh",
    "bash",
    "zsh",
    "fish",
    "sql",
    "graphql",
    "tf",
    "hcl",
    "r",
    "jl",
}
_FALLBACK_FILENAMES = {
    ".env",
    ".env.example",
    ".gitignore",
    ".indexignore",
}
_SWIFT_SAFE_PAREN_NESTING_LIMIT = 2048


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


def _chunk_content_body(text: str) -> str:
    if not text:
        return ""
    lines = text.splitlines()
    if lines and lines[0].startswith("// File: "):
        return "\n".join(lines[1:])
    return text


def _utf8_byte_len(text: str) -> int:
    return len(text.encode("utf-8"))


def _max_delimiter_nesting(source: str, opener: str, closer: str) -> int:
    depth = 0
    max_depth = 0
    for ch in source:
        if ch == opener:
            depth += 1
            if depth > max_depth:
                max_depth = depth
        elif ch == closer and depth:
            depth -= 1
    return max_depth


def _swift_requires_line_fallback(source: str) -> bool:
    return _max_delimiter_nesting(source, "(", ")") > _SWIFT_SAFE_PAREN_NESTING_LIMIT


def _split_text_at_utf8_boundaries(text: str, max_chunk_size: int) -> list[tuple[int, str]]:
    if max_chunk_size <= 0:
        return [(0, text)] if text else []
    encoded = text.encode("utf-8")
    if len(encoded) <= max_chunk_size:
        return [(0, text)]

    parts: list[tuple[int, str]] = []
    start = 0
    while start < len(encoded):
        end = min(start + max_chunk_size, len(encoded))
        while end > start:
            try:
                piece = encoded[start:end].decode("utf-8")
                break
            except UnicodeDecodeError:
                end -= 1
        if end <= start:
            end = min(start + 1, len(encoded))
            while end < len(encoded):
                try:
                    piece = encoded[start:end].decode("utf-8")
                    break
                except UnicodeDecodeError:
                    end += 1
            else:
                piece = encoded[start:].decode("utf-8", errors="ignore")
                end = len(encoded)
        parts.append((start, piece))
        start = end
    return parts


def _extract_chunk_member_usages(text: str) -> list[str]:
    body = _chunk_content_body(text)
    if not body:
        return []
    seen: set[str] = set()
    values: list[str] = []
    for receiver, member in re.findall(
        r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\.\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(",
        body,
    ):
        expr = f"{receiver}.{member}".lower()
        if expr in seen:
            continue
        seen.add(expr)
        values.append(expr)
        if len(values) >= 24:
            break
    return values


def _extract_chunk_call_like_symbols(text: str, member_usages: list[str]) -> list[str]:
    body = _chunk_content_body(text)
    seen = {expr.lower() for expr in member_usages}
    values = list(member_usages)
    for symbol in re.findall(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(", body):
        normalized = symbol.lower()
        if normalized in seen:
            continue
        seen.add(normalized)
        values.append(normalized)
        if len(values) >= 32:
            break
    return values


def _extract_chunk_declared_symbols(text: str) -> list[str]:
    body = _chunk_content_body(text)
    if not body:
        return []
    seen: set[str] = set()
    values: list[str] = []
    for raw_line in body.splitlines():
        for pattern, _kind in _DECLARATION_PATTERNS:
            match = pattern.search(raw_line)
            if not match:
                continue
            symbol = (match.group(1) or "").strip()
            if not symbol:
                continue
            normalized = symbol.lower()
            if normalized in seen:
                continue
            seen.add(normalized)
            values.append(symbol)
            break
        if len(values) >= 16:
            break
    return values


def _chunk_contains_entrypoint(file_path: str, declared_symbols: list[str]) -> bool:
    if not file_path or not declared_symbols:
        return False
    norm = (file_path or "").replace("\\", "/").lower()
    lowered = {str(symbol).strip().lower() for symbol in declared_symbols if str(symbol).strip()}
    if "main" not in lowered:
        return False
    if norm.endswith(
        (
            "/src/main.rs",
            "/src/main.py",
            "/src/main.ts",
            "/src/main.tsx",
            "/src/main.js",
            "/src/main.jsx",
        )
    ):
        return True
    return norm.endswith("/main.go") and ("/cmd/" in norm or norm.startswith("cmd/"))


def _declaration_anchor_candidates(source: str) -> list[tuple[int, str, str]]:
    candidates: list[tuple[int, str, str]] = []
    for idx, raw_line in enumerate(source.splitlines(), start=1):
        for pattern, kind in _DECLARATION_PATTERNS:
            match = pattern.search(raw_line)
            if not match:
                continue
            symbol = (match.group(1) or "").strip()
            if symbol:
                candidates.append((idx, symbol, kind))
            break
    return candidates


def _infer_chunk_role(file_path: str, metadata: dict[str, Any]) -> str:
    norm = (file_path or "").replace("\\", "/").lower()
    for segment, role in _PATH_LIKE_CHUNK_ROLES:
        if segment in norm or norm.startswith(segment.lstrip("/")):
            return role
    if any(segment in norm for segment in _SUPPORT_PATH_SEGMENTS) or norm.startswith(("scripts/", "tools/")):
        return "script_support"
    if metadata.get("contains_definition") or metadata.get("declared_symbols"):
        return "definition"

    lowered = {
        str(node_type).strip().lower()
        for node_type in (metadata.get("node_types") or [])
        if str(node_type).strip()
    }
    if lowered & _DECLARATION_NODE_TYPES:
        return "definition"
    if lowered & _CALLSITE_NODE_TYPES:
        return "usage"
    return "context"


def _enrich_chunk_metadata(chunk: dict[str, Any], file_path: str) -> dict[str, Any]:
    metadata = chunk.get("metadata")
    if not isinstance(metadata, dict):
        metadata = {}
        chunk["metadata"] = metadata

    text = str(chunk.get("text") or chunk.get("content") or "")
    if not isinstance(metadata.get("member_usages"), list):
        metadata["member_usages"] = _extract_chunk_member_usages(text)
    if not isinstance(metadata.get("call_like_symbols"), list):
        metadata["call_like_symbols"] = _extract_chunk_call_like_symbols(
            text,
            metadata.get("member_usages") or [],
        )
    if not isinstance(metadata.get("declared_symbols"), list):
        metadata["declared_symbols"] = _extract_chunk_declared_symbols(text)
    if "contains_definition" not in metadata:
        lowered = {
            str(node_type).strip().lower()
            for node_type in (metadata.get("node_types") or [])
            if str(node_type).strip()
        }
        metadata["contains_definition"] = bool(
            metadata.get("declared_symbols") or (lowered & _DECLARATION_NODE_TYPES)
        )
    if "contains_entrypoint" not in metadata:
        metadata["contains_entrypoint"] = _chunk_contains_entrypoint(
            file_path,
            metadata.get("declared_symbols") or [],
        )
    chunk_role = metadata.get("chunk_role")
    if not isinstance(chunk_role, str) or not chunk_role.strip():
        metadata["chunk_role"] = _infer_chunk_role(file_path, metadata)
    return chunk


def _build_declaration_anchor_chunks(
    source: str,
    file_path: str,
    project_id: str,
    file_meta: dict[str, Any],
    existing_chunks: list[dict[str, Any]],
    *,
    chunk_id_version: str,
) -> list[dict[str, Any]]:
    lines = source.splitlines()
    if not lines:
        return []
    existing_bodies = {_chunk_content_body(str(chunk.get("text") or chunk.get("content") or "")) for chunk in existing_chunks}
    existing_declared = {
        str(symbol).strip().lower()
        for chunk in existing_chunks
        for symbol in ((chunk.get("metadata") or {}).get("declared_symbols") or [])
        if str(symbol).strip()
    }
    file_symbols = list(file_meta.get("file_symbols") or [])
    language = file_meta.get("language")
    anchors: list[dict[str, Any]] = []
    seen_anchor_keys: set[tuple[int, str]] = set()
    for line_no, symbol, kind in _declaration_anchor_candidates(source):
        normalized_symbol = symbol.lower()
        if normalized_symbol in existing_declared or (line_no, normalized_symbol) in seen_anchor_keys:
            continue
        start_line = max(1, line_no - _DECLARATION_ANCHOR_RADIUS)
        end_line = min(len(lines), line_no + _DECLARATION_ANCHOR_RADIUS)
        snippet_body = "\n".join(lines[start_line - 1 : end_line]).strip()
        if not snippet_body or snippet_body in existing_bodies:
            continue
        seen_anchor_keys.add((line_no, normalized_symbol))
        snippet_text = f"// File: {file_path}\n{snippet_body}"
        anchor_id = hashlib.sha256(
            f"{project_id}:{chunk_id_version}:{file_path}:decl:{line_no}:{symbol}".encode("utf-8")
        ).hexdigest()[:14]
        anchors.append(
            {
                "ref_id": f"{project_id}:{chunk_id_version}:{file_path}:decl-{anchor_id}",
                "text": snippet_text,
                "metadata": {
                    "file": file_path,
                    "project_id": project_id,
                    "language": language,
                    "symbols": [symbol],
                    "file_symbols": file_symbols,
                    "start_line": start_line,
                    "end_line": end_line,
                    "declared_symbols": [symbol],
                    "contains_definition": True,
                    "contains_entrypoint": _chunk_contains_entrypoint(file_path, [symbol]),
                    "chunk_role": "definition",
                    "node_types": ["function_item" if kind == "function" else "module"],
                    "anchor_kind": kind,
                },
            }
        )
        if len(anchors) >= _MAX_DECLARATION_ANCHORS:
            break
    return anchors


def _finalize_semantic_chunks(
    source: str,
    file_path: str,
    project_id: str,
    file_meta: dict[str, Any],
    chunks: list[dict[str, Any]],
    *,
    chunk_id_version: str,
) -> list[dict[str, Any]]:
    enriched_chunks = [_enrich_chunk_metadata(chunk, file_path) for chunk in chunks]
    anchor_chunks = [
        _enrich_chunk_metadata(chunk, file_path)
        for chunk in _build_declaration_anchor_chunks(
            source,
            file_path,
            project_id,
            file_meta,
            enriched_chunks,
            chunk_id_version=chunk_id_version,
        )
    ]
    if anchor_chunks:
        enriched_chunks.extend(anchor_chunks)
    return enriched_chunks


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


def should_use_line_window_fallback(file_path: str) -> bool:
    basename = (file_path or "").replace("\\", "/").rsplit("/", 1)[-1]
    if basename in _FALLBACK_FILENAMES:
        return True
    if "." not in basename:
        return False
    ext = basename.rsplit(".", 1)[-1].lower()
    return ext in _FALLBACK_EXTS


def _strip_nul_bytes(text: str) -> str:
    return text.replace("\x00", "")


def _sanitize_chunk_payload(value: Any) -> Any:
    if isinstance(value, str):
        return _strip_nul_bytes(value)
    if isinstance(value, list):
        return [_sanitize_chunk_payload(item) for item in value]
    if isinstance(value, dict):
        return {key: _sanitize_chunk_payload(item) for key, item in value.items()}
    return value


def _sanitize_chunk_list(chunks: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [_sanitize_chunk_payload(chunk) for chunk in chunks]


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
    source = _strip_nul_bytes(source)
    file_header = f"// File: {file_path}\n"
    chunks: list[dict[str, Any]] = []
    lines = source.splitlines()
    i = 0
    metadata_base = dict(file_meta or {})
    max_chunk_size = 4000
    while i < len(lines):
        block = lines[i : i + chunk_lines]
        if not block:
            break
        block_text = "\n".join(block)
        for offset, piece in _split_text_at_utf8_boundaries(block_text, max_chunk_size):
            body = piece.rstrip("\n")
            if not body.strip():
                continue
            text = file_header + body
            chunks.append(
                {
                    "ref_id": _chunk_id_with_header(project_id, file_path, i + offset, text, chunk_id_version),
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
    return _sanitize_chunk_list(
        _finalize_semantic_chunks(
            source,
            file_path,
            project_id,
            metadata_base,
            chunks,
            chunk_id_version=chunk_id_version,
        )
    )


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
    source = _strip_nul_bytes(source)
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
        if not text or not text.strip():
            return
        if _utf8_byte_len(text) <= chunk_max_size:
            _append_chunk(text.rstrip("\n"), start_byte, name, start_line, end_line, context_path)
            return

        raw_lines = text.splitlines(keepends=True)
        current_parts: list[str] = []
        current_bytes = 0
        current_start_byte = start_byte
        current_start_line = start_line
        current_end_line = start_line - 1
        rel_byte = 0

        def _flush_current() -> None:
            nonlocal current_parts, current_bytes, current_start_byte, current_start_line, current_end_line
            if not current_parts:
                return
            body = "".join(current_parts).rstrip("\n")
            if body.strip():
                _append_chunk(
                    body,
                    current_start_byte,
                    name,
                    current_start_line,
                    max(current_start_line, current_end_line),
                    context_path,
                )
            current_parts = []
            current_bytes = 0
            current_end_line = current_start_line - 1

        for line_index, raw_line in enumerate(raw_lines):
            line_start_byte = start_byte + rel_byte
            line_start_line = start_line + line_index
            line_bytes = _utf8_byte_len(raw_line)

            if line_bytes > chunk_max_size:
                _flush_current()
                for piece_offset, piece in _split_text_at_utf8_boundaries(raw_line, chunk_max_size):
                    body = piece.rstrip("\n")
                    if not body.strip():
                        continue
                    _append_chunk(
                        body,
                        line_start_byte + piece_offset,
                        name,
                        line_start_line,
                        line_start_line,
                        context_path,
                    )
                rel_byte += line_bytes
                current_start_byte = start_byte + rel_byte
                current_start_line = line_start_line + 1
                continue

            if current_parts and (
                len(current_parts) >= chunk_lines or current_bytes + line_bytes > chunk_max_size
            ):
                _flush_current()
                current_start_byte = line_start_byte
                current_start_line = line_start_line

            if not current_parts:
                current_start_byte = line_start_byte
                current_start_line = line_start_line

            current_parts.append(raw_line)
            current_bytes += line_bytes
            current_end_line = line_start_line
            rel_byte += line_bytes

            if len(current_parts) >= chunk_lines:
                _flush_current()
                current_start_byte = start_byte + rel_byte
                current_start_line = line_start_line + 1

        _flush_current()

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
            text = src_b[node.start_byte : node.end_byte].decode("utf-8", errors="replace")
            _emit_text(
                text,
                node.start_byte,
                name,
                node.start_point[0] + 1,
                node.end_point[0] + 1,
                next_context,
            )
            for child in node.children:
                _walk(child, next_context)
            return

        for child in node.children:
            _walk(child, context_path)

    _walk(tree.root_node, [])
    return _sanitize_chunk_list(
        _finalize_semantic_chunks(
            source,
            file_path,
            project_id,
            metadata_base,
            chunks,
            chunk_id_version=chunk_id_version,
        )
    )


def build_indexing_chunks(
    source: str,
    file_path: str,
    project_id: str,
    *,
    language: str | None = None,
    chunk_id_version: str = "v6",
    chunk_max_size: int = 4000,
    chunk_overlap: int = 200,
    chunk_lines: int = 60,
    overlap_lines: int = 10,
) -> dict[str, Any]:
    source = _strip_nul_bytes(source)
    file_meta: dict[str, Any] = {}
    chunks: list[dict[str, Any]] = []

    if language == "swift":
        if _swift_requires_line_fallback(source):
            return {
                "language": language,
                "file_meta": file_meta,
                "chunks": build_line_window_chunks(
                    source,
                    file_path,
                    project_id,
                    language=language,
                    file_meta=file_meta,
                    chunk_id_version=chunk_id_version,
                    chunk_lines=chunk_lines,
                    overlap_lines=overlap_lines,
                ),
            }
        try:
            payload = build_semantic_payload(
                source,
                "swift",
                file_path,
                project_id,
                chunk_id_version=chunk_id_version,
                chunk_max_size=chunk_max_size,
                chunk_overlap=chunk_overlap,
            )
            file_meta = payload.get("file_meta") or {}
        except Exception:
            file_meta = {}

        chunks = build_swift_chunks(
            source,
            file_path,
            project_id,
            file_meta=file_meta,
            chunk_id_version=chunk_id_version,
            chunk_max_size=chunk_max_size,
            chunk_lines=chunk_lines,
            overlap_lines=overlap_lines,
        )
        if chunks:
            return {"language": language, "file_meta": file_meta, "chunks": chunks}

    elif language:
        payload = build_semantic_payload(
            source,
            language,
            file_path,
            project_id,
            chunk_id_version=chunk_id_version,
            chunk_max_size=chunk_max_size,
            chunk_overlap=chunk_overlap,
        )
        file_meta = payload.get("file_meta") or {}
        chunks = _sanitize_chunk_list(payload.get("chunks") or [])
        if language == "objc" and (file_meta.get("file_diagnostics") or {}).get("count", 0) > 0:
            chunks = []
        if chunks:
            return {"language": language, "file_meta": file_meta, "chunks": chunks}

    if language is None and not should_use_line_window_fallback(file_path):
        return {"language": language, "file_meta": file_meta, "chunks": []}

    chunks = build_line_window_chunks(
        source,
        file_path,
        project_id,
        language=language,
        file_meta=file_meta,
        chunk_id_version=chunk_id_version,
        chunk_lines=chunk_lines,
        overlap_lines=overlap_lines,
    )
    return {"language": language, "file_meta": file_meta, "chunks": chunks}


def build_semantic_sync_plan(
    all_chunks: list[list[dict[str, Any]]],
    existing_ids: set[str] | None = None,
) -> dict[str, Any]:
    existing_ids = existing_ids or set()
    new_chunks: list[dict[str, Any]] = []
    prune_targets: list[dict[str, Any]] = []
    total_chunks = 0

    for file_chunks in all_chunks:
        if not file_chunks:
            continue
        total_chunks += len(file_chunks)
        file_path = file_chunks[0].get("metadata", {}).get("file")
        chunk_ids = [chunk.get("ref_id") for chunk in file_chunks if chunk.get("ref_id")]
        if file_path and chunk_ids:
            prune_targets.append({"file_path": file_path, "chunk_ids": chunk_ids})
        for chunk in file_chunks:
            if chunk.get("ref_id") not in existing_ids:
                new_chunks.append(chunk)

    return {
        "new_chunks": new_chunks,
        "skipped_chunks": total_chunks - len(new_chunks),
        "prune_targets": prune_targets,
        "total_chunks": total_chunks,
    }


def build_codebase_embedding_rows(
    batch: list[dict[str, Any]],
    project_id: str,
    *,
    expected_dim: int | None = None,
    created_at: float | None = None,
) -> list[tuple[Any, ...]]:
    rows: list[tuple[Any, ...]] = []
    now = created_at
    for item in batch:
        chunk_id = item.get("ref_id")
        text = item.get("text")
        meta = item.get("metadata", {})
        vec = item.get("vector", [])
        if not chunk_id or not isinstance(text, str):
            continue
        if not isinstance(vec, list):
            continue
        if expected_dim is not None and len(vec) != expected_dim:
            continue
        if now is None:
            import time as _time

            now = _time.time()
        file_path = meta.get("file", "") if isinstance(meta, dict) else ""
        chunk_index = 0
        if isinstance(meta, dict):
            chunk_index = int(meta.get("chunk_index") or meta.get("start_line") or 0)
        rows.append(
            (
                chunk_id,
                project_id,
                file_path,
                item.get("ref_type", "code_chunk"),
                chunk_index,
                text,
                "[" + ",".join(str(v) for v in vec) + "]",
                json.dumps(meta if isinstance(meta, dict) else {}),
                now,
            )
        )
    return rows


_native_build_semantic_sync_plan = getattr(_native, "build_semantic_sync_plan", None)
if _native_build_semantic_sync_plan is not None:
    _python_build_semantic_sync_plan = build_semantic_sync_plan

    def build_semantic_sync_plan(
        all_chunks: list[list[dict[str, Any]]],
        existing_ids: set[str] | None = None,
    ) -> dict[str, Any]:
        payload = _native_build_semantic_sync_plan(
            all_chunks,
            sorted(existing_ids) if existing_ids else None,
        )
        return dict(payload or {})


_native_build_codebase_embedding_rows = getattr(_native, "build_codebase_embedding_rows", None)
if _native_build_codebase_embedding_rows is not None:
    _python_build_codebase_embedding_rows = build_codebase_embedding_rows

    def build_codebase_embedding_rows(
        batch: list[dict[str, Any]],
        project_id: str,
        *,
        expected_dim: int | None = None,
        created_at: float | None = None,
    ) -> list[tuple[Any, ...]]:
        rows = _native_build_codebase_embedding_rows(
            batch,
            project_id,
            expected_dim=expected_dim,
            created_at=created_at,
        )
        return list(rows or [])


CODEBASE_EMBEDDINGS_UPSERT_SQL = """\
INSERT INTO codebase_embeddings
  (chunk_id, project_id, file_path, ref_type, chunk_index,
   content, embedding, metadata, created_at)
VALUES (%s, %s, %s, %s, %s, %s, %s::vector, %s::jsonb, to_timestamp(%s))
ON CONFLICT (chunk_id) DO NOTHING
"""


async def execute_codebase_embedding_upsert(
    cursor: Any,
    batch: list[dict[str, Any]],
    project_id: str,
    *,
    expected_dim: int | None = None,
    created_at: float | None = None,
) -> int:
    rows = build_codebase_embedding_rows(
        batch,
        project_id,
        expected_dim=expected_dim,
        created_at=created_at,
    )
    if not rows:
        return 0
    await cursor.executemany(CODEBASE_EMBEDDINGS_UPSERT_SQL, rows)
    return len(rows)


async def execute_semantic_sync(
    conn: Any,
    project_id: str,
    manifest_paths: list[str],
    all_chunks: list[list[dict[str, Any]]],
    *,
    existing_ids_override: set[str] | None = None,
) -> dict[str, Any]:
    if existing_ids_override is None:
        manifest_path_set = [path for path in manifest_paths if path]
        if manifest_path_set:
            cur = await conn.execute(
                """
                SELECT chunk_id
                FROM codebase_embeddings
                WHERE project_id = %s
                  AND file_path = ANY(%s)
                """,
                (project_id, manifest_path_set),
            )
        else:
            cur = await conn.execute("SELECT chunk_id FROM codebase_embeddings WHERE 1=0", ())
        existing_ids = {row[0] for row in await cur.fetchall()}
    else:
        existing_ids = set(existing_ids_override)
    sync_plan = build_semantic_sync_plan(all_chunks, existing_ids)

    pruned_total = 0
    prune_targets = sync_plan.get("prune_targets") or []
    if prune_targets:
        async with conn.cursor() as prune_cursor:
            for target in prune_targets:
                file_path = target.get("file_path")
                chunk_ids = target.get("chunk_ids") or []
                if not file_path or not chunk_ids:
                    continue
                await prune_cursor.execute(
                    """
                    DELETE FROM codebase_embeddings
                    WHERE project_id = %s
                      AND file_path = %s
                      AND NOT (chunk_id = ANY(%s))
                    """,
                    (project_id, file_path, chunk_ids),
                )
                pruned_total += getattr(prune_cursor, "rowcount", 0) or 0

    sync_plan["existing_ids"] = existing_ids
    sync_plan["pruned_total"] = pruned_total
    return sync_plan


async def execute_semantic_index_prepare(
    conn: Any,
    project_id: str,
    manifest_paths: list[str],
    all_chunks: list[list[dict[str, Any]]],
    *,
    rebuild: bool = False,
) -> dict[str, Any]:
    wiped = False
    orphan_pruned = 0

    if rebuild:
        await conn.execute(
            "DELETE FROM codebase_embeddings WHERE project_id = %s",
            (project_id,),
        )
        wiped = True
    else:
        manifest_path_set = [path for path in manifest_paths if path]
        if manifest_path_set:
            delete_cur = await conn.execute(
                """
                DELETE FROM codebase_embeddings
                WHERE project_id = %s
                  AND NOT (file_path = ANY(%s))
                """,
                (project_id, manifest_path_set),
            )
        else:
            delete_cur = await conn.execute(
                "DELETE FROM codebase_embeddings WHERE project_id = %s",
                (project_id,),
            )
        orphan_pruned = getattr(delete_cur, "rowcount", 0) or 0

    sync_plan = await execute_semantic_sync(
        conn,
        project_id,
        manifest_paths,
        all_chunks,
        existing_ids_override=set() if rebuild else None,
    )
    sync_plan["wiped"] = wiped
    sync_plan["orphan_pruned"] = orphan_pruned
    return sync_plan


async def execute_semantic_index_rounds(
    new_chunks: list[dict[str, Any]],
    *,
    batch_size: int,
    concurrency: int,
    embed_batch_fn: Any,
    write_batch_fn: Any,
    progress_fn: Any | None = None,
    round_plan: list[dict[str, Any]] | None = None,
) -> dict[str, Any]:
    import time

    total_new = len(new_chunks)
    total_written = 0

    if total_new <= 0:
        return {"written": 0, "rounds": 0}

    if round_plan is not None:
        round_plan = list(round_plan)
    else:
        round_plan_builder = getattr(_native, "build_semantic_index_round_plan", None)
        if round_plan_builder is not None:
            round_plan = list(round_plan_builder(new_chunks, batch_size, concurrency) or [])
        else:
            safe_batch_size = max(1, batch_size)
            safe_concurrency = max(1, concurrency)
            window = safe_batch_size * safe_concurrency
            n_rounds = (total_new + window - 1) // window
            round_plan = []
            for round_idx in range(n_rounds):
                group = new_chunks[round_idx * window : (round_idx + 1) * window]
                sub_batches = [group[i : i + safe_batch_size] for i in range(0, len(group), safe_batch_size)]
                round_plan.append(
                    {
                        "round_index": round_idx,
                        "rounds": n_rounds,
                        "group_size": len(group),
                        "batch_count": len(sub_batches),
                        "sub_batches": sub_batches,
                    }
                )

    rounds = len(round_plan)
    for round_info in round_plan:
        round_started = time.perf_counter()
        round_idx = int(round_info.get("round_index") or 0)
        n_rounds = int(round_info.get("rounds") or rounds or 1)
        sub_batches = list(round_info.get("sub_batches") or [])
        group_size = int(round_info.get("group_size") or sum(len(batch) for batch in sub_batches))
        batch_count = int(round_info.get("batch_count") or len(sub_batches))

        if progress_fn is not None:
            await progress_fn(
                {
                    "round_index": round_idx,
                    "rounds": n_rounds,
                    "group_size": group_size,
                    "batch_count": batch_count,
                    "written_so_far": total_written,
                    "total_new": total_new,
                    "phase": "embed_start",
                }
            )

        embed_started = time.perf_counter()
        embedded_batches = await __import__("asyncio").gather(
            *[embed_batch_fn(batch) for batch in sub_batches]
        )
        embed_seconds = time.perf_counter() - embed_started

        write_groups: list[list[dict[str, Any]]] = []
        combined_group: list[dict[str, Any]] = []
        for embedded in embedded_batches:
            if embedded:
                combined_group.extend(embedded)
        if combined_group:
            write_groups.append(combined_group)

        if progress_fn is not None:
            await progress_fn(
                {
                    "round_index": round_idx,
                    "rounds": n_rounds,
                    "group_size": group_size,
                    "batch_count": len(write_groups),
                    "written_so_far": total_written,
                    "total_new": total_new,
                    "phase": "write_start",
                }
            )

        write_started = time.perf_counter()
        write_counts = await __import__("asyncio").gather(
            *[write_batch_fn(batch) for batch in write_groups]
        )
        write_seconds = time.perf_counter() - write_started
        total_written += sum(int(count or 0) for count in write_counts)

        if progress_fn is not None:
            await progress_fn(
                {
                    "round_index": round_idx,
                    "rounds": n_rounds,
                    "group_size": group_size,
                    "batch_count": batch_count,
                    "written_so_far": total_written,
                    "total_new": total_new,
                    "phase": "round_done",
                    "round_written": sum(int(count or 0) for count in write_counts),
                    "embed_seconds": embed_seconds,
                    "write_seconds": write_seconds,
                    "round_seconds": time.perf_counter() - round_started,
                }
            )

    return {"written": total_written, "rounds": rounds}


async def _maybe_commit_connection(conn: Any) -> None:
    commit = getattr(conn, "commit", None)
    if commit is None:
        return
    result = commit()
    if hasattr(result, "__await__"):
        await result


async def execute_semantic_index_driver(
    conn: Any,
    project_id: str,
    manifest_paths: list[str],
    all_chunks: list[list[dict[str, Any]]],
    *,
    rebuild: bool = False,
    batch_size: int,
    concurrency: int,
    embed_batch_fn: Any,
    write_batch_fn: Any,
    progress_fn: Any | None = None,
) -> dict[str, Any]:
    import time as _time

    prepare_started = _time.perf_counter()
    driver_plan_builder = getattr(_native, "build_semantic_index_driver_plan", None)

    existing_ids: set[str] = set()
    db_paths: set[str] = set()
    if driver_plan_builder is not None:
        manifest_path_set = [path for path in manifest_paths if path]
        if rebuild:
            existing_ids = set()
        else:
            if manifest_path_set:
                cur = await conn.execute(
                    """
                    SELECT chunk_id
                    FROM codebase_embeddings
                    WHERE project_id = %s
                      AND file_path = ANY(%s)
                    """,
                    (project_id, manifest_path_set),
                )
            else:
                cur = await conn.execute("SELECT chunk_id FROM codebase_embeddings WHERE 1=0", ())
            existing_ids = {row[0] for row in await cur.fetchall()}
        db_paths = set(manifest_path_set)

        sync_plan = dict(
            driver_plan_builder(
                all_chunks,
                sorted(existing_ids),
                manifest_paths,
                sorted(db_paths),
                rebuild=rebuild,
                batch_size=batch_size,
                concurrency=concurrency,
            )
            or {}
        )
        if rebuild:
            await conn.execute(
                "DELETE FROM codebase_embeddings WHERE project_id = %s",
                (project_id,),
            )
            orphan_pruned = 0
        else:
            if manifest_path_set:
                delete_cur = await conn.execute(
                    """
                    DELETE FROM codebase_embeddings
                    WHERE project_id = %s
                      AND NOT (file_path = ANY(%s))
                    """,
                    (project_id, manifest_path_set),
                )
            else:
                delete_cur = await conn.execute(
                    "DELETE FROM codebase_embeddings WHERE project_id = %s",
                    (project_id,),
                )
            orphan_pruned = getattr(delete_cur, "rowcount", 0) or 0

        pruned_total = 0
        prune_targets = sync_plan.get("prune_targets") or []
        if prune_targets:
            async with conn.cursor() as prune_cursor:
                for target in prune_targets:
                    file_path = target.get("file_path")
                    chunk_ids = target.get("chunk_ids") or []
                    if not file_path or not chunk_ids:
                        continue
                    await prune_cursor.execute(
                        """
                        DELETE FROM codebase_embeddings
                        WHERE project_id = %s
                          AND file_path = %s
                          AND NOT (chunk_id = ANY(%s))
                        """,
                        (project_id, file_path, chunk_ids),
                    )
                    pruned_total += getattr(prune_cursor, "rowcount", 0) or 0

        sync_plan["existing_ids"] = existing_ids
        sync_plan["orphan_pruned"] = orphan_pruned
        sync_plan["pruned_total"] = pruned_total
    else:
        sync_plan = await execute_semantic_index_prepare(
            conn,
            project_id,
            manifest_paths,
            all_chunks,
            rebuild=rebuild,
        )

    # End the preparation transaction before the long-running embedding loop.
    # The round writes use the caller's write_batch_fn, typically on separate
    # pool connections, so keeping the prepare connection open here invites
    # idle-in-transaction timeouts on large indexes.
    await _maybe_commit_connection(conn)

    if progress_fn is not None:
        await progress_fn(
            {
                "phase": "prepare_done",
                "prepare_seconds": _time.perf_counter() - prepare_started,
                "existing_count": len(sync_plan.get("existing_ids") or set()),
                "orphan_pruned": int(sync_plan.get("orphan_pruned") or 0),
                "pruned_total": int(sync_plan.get("pruned_total") or 0),
                "total_new": len(sync_plan.get("new_chunks") or []),
            }
        )

    round_result = await execute_semantic_index_rounds(
        sync_plan.get("new_chunks") or [],
        batch_size=batch_size,
        concurrency=concurrency,
        embed_batch_fn=embed_batch_fn,
        write_batch_fn=write_batch_fn,
        progress_fn=progress_fn,
        round_plan=sync_plan.get("round_plan"),
    )
    return {
        **sync_plan,
        **round_result,
    }


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
    source = _strip_nul_bytes(source)
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
    chunks = _sanitize_chunk_list(
        _finalize_semantic_chunks(
            source,
            file_path,
            project_id,
            file_meta,
            chunks,
            chunk_id_version=chunk_id_version,
        )
    )
    return {"result": result, "file_meta": file_meta, "chunks": chunks}
