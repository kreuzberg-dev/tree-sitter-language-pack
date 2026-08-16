"""Shared reference-doc parser.

One parser handles every language: the `api-<lang>.md` files are emitted from a
single IR with per-language token substitution, so they share a heading skeleton
(`### section` / `#### type` / `###### member`), the same `**Signature:**` and
`**Example:**` labels, and one fenced block per label. Five of the six are even
identical in line count. That uniformity is the defect under audit, and it is
also what makes a single extractor legitimate.
"""

from __future__ import annotations

import re
from pathlib import Path

LABELS = ("**Signature:**", "**Example:**", "**Definition:**", "**Constructor:**")

Block = dict[str, object]


def parse(path: str, fence_lang: str) -> list[Block]:
    """Return every fenced block in `path`, tagged with its heading context."""
    lines = Path(path).read_text(encoding="utf-8").split("\n")

    blocks: list[Block] = []
    section = type_ = member = label = None
    fence_open = "```" + fence_lang
    index = 0
    while index < len(lines):
        raw = lines[index]
        stripped = raw.strip()
        if raw.startswith("### ") and not raw.startswith("#### "):
            section, type_, member = raw[4:].strip(), None, None
        elif raw.startswith("#### ") and not raw.startswith("##### "):
            type_, member = raw[5:].strip(), None
        elif raw.startswith("##### ") and not raw.startswith("###### "):
            member = raw[6:].strip()
        elif raw.startswith("###### "):
            member = raw[7:].strip()
        elif stripped in LABELS:
            label = stripped.strip("*:")
        elif stripped == fence_open:
            body, end = [], index + 1
            while end < len(lines) and lines[end].strip() != "```":
                body.append(lines[end])
                end += 1
            blocks.append(
                {
                    "section": section,
                    "type": type_,
                    "member": member,
                    "label": label,
                    "line": index + 2,
                    "code": "\n".join(body),
                }
            )
            label = None
            index = end
        index += 1
    return blocks


def signatures(path: str, fence_lang: str) -> list[Block]:
    """Return only the `**Signature:**` blocks."""
    return [b for b in parse(path, fence_lang) if b["label"] == "Signature"]


def collapse(text: str) -> str:
    """Normalize whitespace and strip the body opener — the `verbatim` level.

    Removes only `{`, `;`, `=>` and elixir's `do`, which is where a declaration
    ends and a definition begins. Modifiers, return types and `async` survive,
    so a match really does mean the doc shows the declaration.
    """
    text = re.sub(r"\s*\{\s*$", "", text.strip())
    text = re.sub(r"\s*;\s*$", "", text)
    text = re.sub(r"\s*=>\s*$", "", text)
    text = re.sub(r"\s+do$", "", text)
    return re.sub(r"\s+", " ", text).strip()


def tighten(text: str) -> str:
    """`collapse()` plus punctuation spacing, so `f( a, b )` equals `f(a,b)`.

    Pointer asterisks are normalized too: `const char *name` and
    `const char* name` are the same declaration, and treating that as a
    mismatch would report a whitespace convention as a documentation defect.
    """
    return re.sub(r"\s*\*\s*", "*", re.sub(r"\s*([(),])\s*", r"\1", collapse(text)))
