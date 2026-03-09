from __future__ import annotations

import os
from json import loads
from pathlib import Path
from typing import TYPE_CHECKING, Any, cast

import pytest
from tree_sitter import Language, Parser

from tree_sitter_language_pack import SupportedLanguage, get_binding, get_language, get_parser, get_supported_languages

if TYPE_CHECKING:
    from collections.abc import Callable


def load_language_definitions() -> dict[str, dict[str, str]]:
    possible_paths = [
        Path(__file__).parent.parent.resolve() / "sources" / "language_definitions.json",
        Path(os.environ.get("PROJECT_ROOT", ".")) / "sources" / "language_definitions.json",
        Path.cwd() / "sources" / "language_definitions.json",
    ]

    for path in possible_paths:
        if path.exists():
            return cast("dict[str, dict[str, str]]", loads(path.read_text()))

    raise AssertionError("language_definitions.json not found, using SupportedLanguage directly")


language_definitions = load_language_definitions()
language_names = sorted([*list(language_definitions.keys()), "yaml", "csharp", "embeddedtemplate"])


def test_language_names() -> None:
    supported_languages = sorted(get_supported_languages())
    assert supported_languages == language_names


def test_get_supported_languages() -> None:
    languages = get_supported_languages()
    assert isinstance(languages, tuple)
    assert len(languages) > 0
    assert all(isinstance(lang, str) for lang in languages)


@pytest.mark.parametrize("language", language_names)
def test_get_supported_languages_contains(language: str) -> None:
    assert language in get_supported_languages()


@pytest.mark.parametrize("language", language_names)
def test_get_binding(language: SupportedLanguage) -> None:
    assert type(get_binding(language)).__name__ == "PyCapsule"


@pytest.mark.parametrize("language", language_names)
def test_get_language(language: SupportedLanguage) -> None:
    assert isinstance(get_language(language), Language)


@pytest.mark.parametrize("language", language_names)
def test_get_parser(language: SupportedLanguage) -> None:
    assert isinstance(get_parser(language), Parser)


@pytest.mark.parametrize("handler", [get_language, get_parser])
def test_raises_exception_for_invalid_name(handler: Callable[[str], Any]) -> None:
    with pytest.raises(LookupError):
        handler("invalid")
