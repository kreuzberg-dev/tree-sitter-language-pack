from __future__ import annotations

import hashlib
import re
from copy import deepcopy
from typing import Any
import json

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
    all_chunks: list[list[dict[str, Any]]],
) -> dict[str, Any]:
    cur = await conn.execute(
        "SELECT chunk_id FROM codebase_embeddings WHERE project_id = %s",
        [project_id],
    )
    existing_ids = {row[0] for row in await cur.fetchall()}
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

    rows_cursor = await conn.execute(
        "SELECT DISTINCT file_path FROM codebase_embeddings WHERE project_id = %s",
        (project_id,),
    )
    db_paths = {row[0] async for row in rows_cursor}
    manifest_path_set = {path for path in manifest_paths if path}
    orphans = db_paths - manifest_path_set
    for path in orphans:
        await conn.execute(
            "DELETE FROM codebase_embeddings WHERE project_id = %s AND file_path = %s",
            (project_id, path),
        )
        orphan_pruned += 1

    sync_plan = await execute_semantic_sync(conn, project_id, all_chunks)
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
) -> dict[str, Any]:
    window = batch_size * concurrency
    total_new = len(new_chunks)
    total_written = 0
    rounds = 0

    if total_new <= 0:
        return {"written": 0, "rounds": 0}

    n_rounds = (total_new + window - 1) // window
    for round_idx in range(n_rounds):
        group = new_chunks[round_idx * window : (round_idx + 1) * window]
        sub_batches = [group[i : i + batch_size] for i in range(0, len(group), batch_size)]
        rounds += 1

        if progress_fn is not None:
            await progress_fn(
                {
                    "round_index": round_idx,
                    "rounds": n_rounds,
                    "group_size": len(group),
                    "batch_count": len(sub_batches),
                    "written_so_far": total_written,
                    "total_new": total_new,
                    "phase": "embed_start",
                }
            )

        embedded_batches = await __import__("asyncio").gather(
            *[embed_batch_fn(batch) for batch in sub_batches]
        )

        if progress_fn is not None:
            await progress_fn(
                {
                    "round_index": round_idx,
                    "rounds": n_rounds,
                    "group_size": len(group),
                    "batch_count": len(sub_batches),
                    "written_so_far": total_written,
                    "total_new": total_new,
                    "phase": "write_start",
                }
            )

        write_counts = await __import__("asyncio").gather(
            *[write_batch_fn(batch) for batch in embedded_batches]
        )
        total_written += sum(int(count or 0) for count in write_counts)

        if progress_fn is not None:
            await progress_fn(
                {
                    "round_index": round_idx,
                    "rounds": n_rounds,
                    "group_size": len(group),
                    "batch_count": len(sub_batches),
                    "written_so_far": total_written,
                    "total_new": total_new,
                    "phase": "round_done",
                    "round_written": sum(int(count or 0) for count in write_counts),
                }
            )

    return {"written": total_written, "rounds": rounds}


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
