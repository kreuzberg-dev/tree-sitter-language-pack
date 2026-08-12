#!/usr/bin/env python3
"""Fail if a native artifact does not export every symbol the C header declares.

The header (``crates/ts-pack-core-ffi/include/ts_pack.h``) is the ABI contract.
Any vendored or downloaded ``libts_pack_core_ffi`` that exports fewer symbols
than the header declares is stale, and every binding that resolves those symbols
(Java FFM ``SymbolLookup``, C# ``LibraryImport``, cgo) is broken against it —
usually at first use, in production, rather than at build time.

Usage::

    scripts/ci/check_native_symbols.py packages/go/.lib/linux-x86_64/libts_pack_core_ffi.so
    scripts/ci/check_native_symbols.py --discover
    scripts/ci/check_native_symbols.py --header path/to/ts_pack.h lib1 lib2 ...

Exit status is 0 only when every inspected artifact defines a superset of the
header's symbols. Anything else — a missing symbol, an unreadable artifact, a
symbol reader that cannot parse the file, or zero artifacts found — exits 1.

Windows note: PE exports are read with ``llvm-readobj --coff-exports``, which is
cross-platform and ships with the Rust toolchain's ``llvm-tools`` component. On
a Windows runner without it, the equivalent is
``dumpbin /exports ts_pack_core_ffi.dll``; that fallback is used automatically
when ``dumpbin`` is on PATH and no ``llvm-readobj`` is available. Import
libraries (``.lib``) are ``!<arch>`` archives and go through the archive reader.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

DEFAULT_HEADER = Path("crates/ts-pack-core-ffi/include/ts_pack.h")
SYMBOL_PREFIX = "ts_pack_"

# ~keep: Apple's /usr/bin/nm cannot parse the LLVM bitcode that rustc embeds in
# release rlib/staticlib members and reports `error: Unknown attribute kind` on
# stderr while still exiting 0 with a truncated symbol list. Treating that as a
# real result produces a fabricated "every symbol is missing" report, so any
# reader diagnostic is escalated to a hard failure instead.
NM_DIAGNOSTIC = re.compile(r"^\s*[^\n]*?:\s*error:", re.MULTILINE)

HEADER_DECL = re.compile(rf"\b({SYMBOL_PREFIX}[A-Za-z0-9_]+)\s*\(")
BLOCK_COMMENT = re.compile(r"/\*.*?\*/", re.DOTALL)
LINE_COMMENT = re.compile(r"//[^\n]*")

DISCOVERY_GLOBS = (
    "packages/go/.lib/*/libts_pack_core_ffi.*",
    "packages/go/.lib/*/ts_pack_core_ffi.*",
    "packages/java/src/main/resources/natives/*/libts_pack_core_ffi.*",
    "packages/java/src/main/resources/natives/*/ts_pack_core_ffi.*",
    "packages/csharp/TreeSitterLanguagePack/runtimes/*/native/libts_pack_core_ffi.*",
    "packages/csharp/TreeSitterLanguagePack/runtimes/*/native/ts_pack_core_ffi.*",
)
DISCOVERY_SUFFIXES = (".so", ".dylib", ".dll", ".a", ".lib")


class InspectionError(RuntimeError):
    """A native artifact could not be inspected. Never treated as "up to date"."""


def header_symbols(header: Path) -> set[str]:
    try:
        source = header.read_text(encoding="utf-8", errors="replace")
    except OSError as exc:
        raise InspectionError(f"reading header {header}: {exc}") from exc

    source = LINE_COMMENT.sub("", BLOCK_COMMENT.sub("", source))
    symbols = set(HEADER_DECL.findall(source))
    if not symbols:
        raise InspectionError(
            f"no {SYMBOL_PREFIX}* declarations found in {header} — the header is empty, "
            "unparseable, or the wrong file; refusing to report every library as up to date"
        )
    return symbols


def find_tool(*names: str) -> str | None:
    for name in names:
        found = shutil.which(name)
        if found:
            return found
    return None


def rust_llvm_tool(name: str) -> str | None:
    """Locate an llvm-tools binary inside the active Rust sysroot.

    Preferred over the system toolchain because it is version-matched to the
    compiler that produced the artifacts, and therefore can read their bitcode.
    """
    rustc = shutil.which("rustc")
    roots: list[Path] = []
    if rustc:
        try:
            sysroot = subprocess.run(
                [rustc, "--print", "sysroot"],
                capture_output=True,
                text=True,
                check=True,
                timeout=30,
            ).stdout.strip()
            if sysroot:
                roots.append(Path(sysroot))
        except (subprocess.SubprocessError, OSError):
            pass

    rustup_home = Path(os.environ.get("RUSTUP_HOME", Path.home() / ".rustup"))
    roots.extend(sorted(rustup_home.glob("toolchains/*")))

    for root in roots:
        for candidate in sorted(root.glob(f"lib/rustlib/*/bin/{name}")):
            if candidate.is_file() and os.access(candidate, os.X_OK):
                return str(candidate)
    return None


def run_reader(argv: list[str], path: Path) -> str:
    try:
        # ~keep check=False is deliberate: a non-zero reader exit is inspected below so the
        # ~keep failure can be reported as UNREADABLE rather than raised as a CalledProcessError.
        result = subprocess.run(argv, capture_output=True, text=True, timeout=600, check=False)
    except (OSError, subprocess.SubprocessError) as exc:
        raise InspectionError(f"running {argv[0]} on {path}: {exc}") from exc

    if result.returncode != 0:
        detail = (result.stderr or result.stdout).strip().splitlines()
        tail = detail[0] if detail else f"exit status {result.returncode}"
        raise InspectionError(f"{argv[0]} failed on {path}: {tail}")

    if NM_DIAGNOSTIC.search(result.stderr or ""):
        first = next(line for line in result.stderr.splitlines() if NM_DIAGNOSTIC.search(line))
        raise InspectionError(
            f"{argv[0]} could not fully read {path} ({first.strip()}); "
            "its symbol list is truncated and would produce a fabricated result. "
            "Use the llvm-tools binary matching the compiler that built this artifact "
            "(rustup component add llvm-tools)."
        )
    return result.stdout


def parse_nm(output: str) -> set[str]:
    symbols: set[str] = set()
    for line in output.splitlines():
        name = line.rsplit(" ", 1)[-1].strip()
        name = name.removeprefix("_")
        if name.startswith(SYMBOL_PREFIX) and name.replace("_", "").isalnum():
            symbols.add(name)
    return symbols


def detect_format(path: Path) -> str:
    try:
        with path.open("rb") as handle:
            magic = handle.read(8)
    except OSError as exc:
        raise InspectionError(f"reading {path}: {exc}") from exc

    if magic.startswith(b"!<arch>"):
        return "archive"
    if magic.startswith(b"\x7fELF"):
        return "elf"
    if magic.startswith(b"MZ"):
        return "pe"
    if magic[:4] in (
        b"\xcf\xfa\xed\xfe",
        b"\xce\xfa\xed\xfe",
        b"\xca\xfe\xba\xbe",
        b"\xbe\xba\xfe\xca",
    ):
        return "macho"
    raise InspectionError(f"unrecognised object format for {path} (magic {magic!r:.24})")


def exported_symbols(path: Path) -> set[str]:
    kind = detect_format(path)

    if kind == "pe":
        readobj = rust_llvm_tool("llvm-readobj") or find_tool("llvm-readobj")
        if readobj:
            out = run_reader([readobj, "--coff-exports", str(path)], path)
            return set(re.findall(rf"\b{SYMBOL_PREFIX}[A-Za-z0-9_]+", out))
        dumpbin = find_tool("dumpbin")
        if dumpbin:
            out = run_reader([dumpbin, "/exports", str(path)], path)
            return set(re.findall(rf"\b{SYMBOL_PREFIX}[A-Za-z0-9_]+", out))
        raise InspectionError(
            f"no PE export reader available for {path}; install llvm-tools "
            "(rustup component add llvm-tools) or run on a host with dumpbin"
        )

    nm = rust_llvm_tool("llvm-nm") or find_tool("llvm-nm", "nm")
    if not nm:
        raise InspectionError(f"no nm available to inspect {path}")

    if kind == "elf":
        # ~keep: a stripped shared object keeps only .dynsym, which plain nm does
        # not read; the unstripped case needs the non-dynamic pass as well.
        symbols = parse_nm(run_reader([nm, "-D", "--defined-only", str(path)], path))
        if not symbols:
            symbols = parse_nm(run_reader([nm, "--defined-only", "-g", str(path)], path))
    elif kind == "macho":
        symbols = parse_nm(run_reader([nm, "--defined-only", "-g", str(path)], path))
    else:
        symbols = parse_nm(run_reader([nm, "--defined-only", "-g", str(path)], path))

    if not symbols:
        raise InspectionError(
            f"{path} defines zero {SYMBOL_PREFIX}* symbols. That is a reader or format "
            "problem, not a staleness result — a real library always defines some. "
            f"Reader used: {nm} (format: {kind})."
        )
    return symbols


def discover(root: Path) -> list[Path]:
    found: list[Path] = []
    for pattern in DISCOVERY_GLOBS:
        found.extend(
            candidate
            for candidate in root.glob(pattern)
            if candidate.is_file() and candidate.suffix in DISCOVERY_SUFFIXES
        )
    return sorted(set(found))


def check(path: Path, expected: set[str]) -> list[str]:
    actual = exported_symbols(path)
    return sorted(expected - actual)


def main() -> int:
    parser = argparse.ArgumentParser(description="Verify native artifacts export every symbol the C header declares.")
    parser.add_argument("libraries", nargs="*", type=Path)
    parser.add_argument("--header", type=Path, default=None)
    parser.add_argument(
        "--discover",
        action="store_true",
        help="also inspect every known vendored artifact location under --root",
    )
    parser.add_argument("--root", type=Path, default=Path.cwd())
    args = parser.parse_args()

    root = args.root.resolve()
    header = args.header or (root / DEFAULT_HEADER)

    try:
        expected = header_symbols(header)
    except InspectionError as exc:
        sys.stderr.write(f"error: {exc}\n")
        return 1

    targets = list(args.libraries)
    if args.discover:
        targets.extend(discover(root))
    targets = sorted({Path(t).resolve() for t in targets})

    if not targets:
        sys.stderr.write(
            "error: no native artifacts to inspect. Pass paths explicitly or use --discover "
            "after the build step that produces them. An empty run is a wiring bug, not a pass.\n"
        )
        return 1

    sys.stdout.write(f"header {header} declares {len(expected)} {SYMBOL_PREFIX}* symbols\n")

    failures = 0
    for target in targets:
        try:
            missing = check(target, expected)
        except InspectionError as exc:
            failures += 1
            sys.stdout.write(f"UNREADABLE {target}\n  {exc}\n")
            continue

        if missing:
            failures += 1
            sys.stdout.write(f"STALE {target}\n  missing {len(missing)} of {len(expected)} declared symbols:\n")
            for name in missing:
                sys.stdout.write(f"    {name}\n")
        else:
            sys.stdout.write(f"OK {target} ({len(expected)}/{len(expected)})\n")

    if failures:
        sys.stderr.write(
            f"\nerror: {failures} of {len(targets)} native artifact(s) do not match "
            f"{header}. Rebuild ts-pack-core-ffi and re-vendor, or regenerate the "
            "release assets these artifacts were downloaded from.\n"
        )
        return 1

    sys.stdout.write(f"\nall {len(targets)} native artifact(s) match the header\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
