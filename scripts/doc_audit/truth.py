"""Per-language truth extractors: declaration lines from the EMITTED binding.

Every extractor reads the artifact a consumer actually compiles against — never
the Rust source, never a stub. `corpus()` returns the file list alongside the
declarations because an absence claim is only as good as the file list behind
it: grepping one file and reporting "not found" is how a real member gets filed
as a fabrication.
"""

from __future__ import annotations

import re
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Callable

ROOT = Path(__file__).resolve().parent.parent.parent

Decl = dict[str, object]

EXCLUDE = ("/target/", "/obj/", "/bin/", "/.alef/", "_test.go")


def _walk(patterns: list[str], exclude: tuple[str, ...] = ()) -> list[str]:
    """Expand repo-relative globs to a sorted, de-duplicated file list."""
    out = []
    for pattern in patterns:
        for path in ROOT.glob(pattern):
            rel = path.relative_to(ROOT).as_posix()
            if not any(x in rel for x in exclude):
                out.append(rel)
    return sorted(set(out))


def _scan(files: list[str], keep: Callable[[str], bool]) -> list[Decl]:
    """Collect declaration-shaped lines from `files`, skipping comments."""
    rows: list[Decl] = []
    for rel in files:
        content = (ROOT / rel).read_text(encoding="utf-8", errors="replace")
        for number, line in enumerate(content.split("\n"), 1):
            text = line.strip()
            if not text or text.startswith(("//", "*", "/*", "#")):
                continue
            if keep(text):
                rows.append({"file": rel, "line": number, "text": text})
    return rows


_C_PROTOTYPE = re.compile(r"^[A-Za-z_][\w \t*]*\**\s*\w+\s*\(")
_JAVA_MODIFIER = re.compile(
    r"^(public|protected|private|static|final|abstract|sealed|non-sealed|synchronized|native|default|strictfp)\b"
)
_CSHARP_MODIFIER = re.compile(r"^(public|protected|internal|private)\b")

# The cbindgen header is the only artifact a C consumer compiles against.
PATTERNS: dict[str, list[str]] = {
    "c": ["crates/ts-pack-core-ffi/include/ts_pack.h"],
    "java": ["packages/java/src/main/java/**/*.java"],
    "dart": ["packages/dart/lib/**/*.dart"],
    "elixir": ["packages/elixir/lib/**/*.ex"],
    "csharp": ["packages/csharp/**/*.cs"],
    "go": ["packages/go/*.go"],
}

KEEP: dict[str, Callable[[str], bool]] = {
    "c": lambda s: _C_PROTOTYPE.match(s) is not None,
    "java": lambda s: _JAVA_MODIFIER.match(s) is not None,
    "dart": lambda s: "(" in s or s.startswith(("abstract ", "class ", "enum ", "sealed ")),
    "elixir": lambda s: s.startswith(("def ", "defp ", "defmodule ", "defstruct", "defexception")),
    "csharp": lambda s: _CSHARP_MODIFIER.match(s) is not None,
    "go": lambda s: s.startswith(("func ", "type ", "const ", "var ")),
}


def corpus(lang: str) -> tuple[list[str], list[Decl]]:
    """Return (files searched, declarations found) for one language."""
    files = _walk(PATTERNS[lang], exclude=EXCLUDE)
    return files, _scan(files, KEEP[lang])
