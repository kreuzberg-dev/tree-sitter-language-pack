"""Doc-vs-binding audit harness — frozen baseline for the reference-doc defect.

Measures, per language, how many documented signatures actually appear in the
emitted binding. See README.md for the columns and the pre-fix baseline.

    python3 scripts/doc_audit/audit.py --json /tmp/after.json
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import TYPE_CHECKING

import docparse
import truth

if TYPE_CHECKING:
    from collections.abc import Callable

DOCS = truth.ROOT / "docs-site/src/content/docs/reference"

LANGS = ("c", "java", "dart", "elixir", "csharp", "go")

NEGATIVE_CONTROL = "ZzzNotARealSymbol"


def _name(signature: str) -> str | None:
    """Extract the member name from a signature, if it has one."""
    match = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*\(", signature)
    return match.group(1) if match else None


def _java_contract(sig: str) -> str:
    """Drop `throws` clauses and parameter `final`, which docs never render."""
    return re.sub(r"\bfinal\s+", "", re.sub(r"\s+throws\s+[\w., ]+$", "", sig))


def _dart_contract(sig: str) -> str:
    """Unwrap `Future<>` and drop `static`/`async`: the binding is all async."""
    sig = re.sub(r"\bFuture<(.*?)>", r"\1", sig)
    return re.sub(r"\s+async$", "", re.sub(r"^static\s+", "", sig.strip()))


def _elixir_contract(sig: str) -> str:
    """Drop the leading receiver argument that NIF wrappers take."""
    match = re.match(r"^(def\s+\w+[!?]?)\((.*)\)$", sig.strip())
    if not match:
        return sig
    args = [a.strip() for a in match.group(2).split(",") if a.strip()]
    return "{}({})".format(match.group(1), ",".join(args[1:]))


def _go_contract(sig: str) -> str:
    """Drop the method receiver; docs render package functions as methods."""
    return re.sub(r"^func\s*\(\s*\w+\s+\*?\w+\s*\)\s*", "func ", sig)


MODULO: dict[str, Callable[[str], str]] = {
    "c": lambda s: s,
    "java": _java_contract,
    "dart": _dart_contract,
    "elixir": _elixir_contract,
    "csharp": lambda s: s,
    "go": _go_contract,
}

# Elixir signature blocks carry an `@spec` line and a `def` line; the `def` is
# the declaration. Every other language has a single-line signature block.
DOC_SIG: dict[str, Callable[[str], str]] = {
    "elixir": lambda code: next(
        (line.strip() for line in code.split("\n") if line.strip().startswith("def ")),
        code,
    ),
}


def audit(lang: str) -> dict[str, object]:
    """Compare every documented signature for `lang` against its binding."""
    files, rows = truth.corpus(lang)
    modulo = MODULO[lang]

    exact: dict[str, list] = {}
    loose: dict[str, list] = {}
    names: dict[str, list] = {}
    for row in rows:
        text = str(row["text"])
        exact.setdefault(docparse.tighten(text), []).append(row)
        loose.setdefault(docparse.tighten(modulo(docparse.collapse(text))), []).append(row)
        name = _name(text)
        if name:
            names.setdefault(name, []).append(row)

    pick = DOC_SIG.get(lang, lambda code: code)
    results = []
    for block in docparse.signatures(str(DOCS / f"api-{lang}.md"), lang):
        code = docparse.collapse(pick(str(block["code"])))
        hit = exact.get(docparse.tighten(code))
        near = loose.get(docparse.tighten(modulo(code)))
        name = _name(code)
        found = names.get(name) if name else None
        results.append(
            {
                "doc_line": block["line"],
                "type": block["type"],
                "member": block["member"],
                "sig": code,
                "name": name,
                "verbatim": bool(hit),
                "modulo": bool(hit or near),
                "name_present": bool(found),
                "src": [f"{r['file']}:{r['line']}" for r in (hit or near or found or [])][:3],
            }
        )
    blob = "\n".join(str(r["text"]) for r in rows)
    return {
        "lang": lang,
        "files": files,
        "decls": len(rows),
        "sigs": results,
        "negative_control_fires": NEGATIVE_CONTROL in blob,
    }


def main() -> int:
    """Run the audit for every language and print the summary table."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", help="write the full per-signature report here")
    args = parser.parse_args()

    report = {}
    failures = []
    header = ("lang", "verbatim", "modulo", "name-absent", "decls/files")
    print(f"{header[0]:<8} {header[1]:<12} {header[2]:<12} {header[3]:<12} {header[4]}")
    for lang in LANGS:
        result = audit(lang)
        sigs = result["sigs"]
        total = len(sigs)
        verbatim = sum(1 for s in sigs if s["verbatim"])
        modulo = sum(1 for s in sigs if s["modulo"])
        absent = sum(1 for s in sigs if not s["name_present"])
        report[lang] = result
        print(
            f"{lang:<8} {f'{verbatim}/{total}':<12} {f'{modulo}/{total}':<12} "
            f"{absent:<12} {result['decls']}/{len(result['files'])}"
        )
        # A zero-result sweep is indistinguishable from a broken extractor.
        if result["negative_control_fires"]:
            failures.append(f"{lang}: negative control {NEGATIVE_CONTROL} matched")
        if total - absent == 0:
            failures.append(f"{lang}: no documented name resolved at all — extractor likely broken")

    for failure in failures:
        print("CONTROL FAILURE", failure, file=sys.stderr)

    if args.json:
        Path(args.json).write_text(json.dumps(report, indent=1), encoding="utf-8")
        # Freeze the roster too: deleting fabricated members shrinks the
        # denominator, improving the ratio without fixing a single signature.
        roster = Path(args.json.replace(".json", "-members.txt"))
        roster.write_text(
            "".join(
                f"{lang}\t{sig['type']}\t{sig['member']}\t{sig['name']}\t{sig['sig']}\n"
                for lang, result in report.items()
                for sig in result["sigs"]
            ),
            encoding="utf-8",
        )
        print("\nwrote", args.json, "and", roster)

    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
