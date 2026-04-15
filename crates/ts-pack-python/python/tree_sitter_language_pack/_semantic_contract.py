from __future__ import annotations

REQUIRED_SEMANTIC_CHUNK_FIELDS: tuple[str, ...] = (
    "member_usages",
    "call_like_symbols",
    "declared_symbols",
    "contains_definition",
    "contains_entrypoint",
    "chunk_role",
)
