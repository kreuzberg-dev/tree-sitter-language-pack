from typing import TypeAlias

from tree_sitter_language_pack._native import (
    LanguageNotFoundError,
    ParseError,
    ProcessConfig,
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
)

SupportedLanguage: TypeAlias = str

__all__ = [
    "LanguageNotFoundError",
    "ParseError",
    "ProcessConfig",
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
]
