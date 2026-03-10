from typing import TypeAlias

from tree_sitter_language_pack._native import (
    LanguageNotFoundError,
    ParseError,
    QueryError,
    TreeHandle,
    available_languages,
    get_binding,
    get_language,
    get_parser,
    has_language,
    language_count,
    parse_string,
    process,
    process_and_chunk,
)

SupportedLanguage: TypeAlias = str

__all__ = [
    "LanguageNotFoundError",
    "ParseError",
    "QueryError",
    "SupportedLanguage",
    "TreeHandle",
    "available_languages",
    "get_binding",
    "get_language",
    "get_parser",
    "has_language",
    "language_count",
    "parse_string",
    "process",
    "process_and_chunk",
]
