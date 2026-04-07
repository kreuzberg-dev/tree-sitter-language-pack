from __future__ import annotations

import json
import os
import shlex
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass
class SwiftSymbolRecord:
    filepath: str
    name: str
    base_name: str
    kind: str
    start_line: int
    end_line: int
    usr: str | None
    doc_comment: str | None
    inherited_types: list[str]


def _which(name: str) -> str | None:
    hit = shutil.which(name)
    if hit:
        return hit
    py_bin = os.path.dirname(sys.executable)
    candidate = os.path.join(py_bin, name)
    if os.path.isfile(candidate) and os.access(candidate, os.X_OK):
        return candidate
    for path in (f"/opt/homebrew/bin/{name}", f"/usr/local/bin/{name}", f"/usr/bin/{name}"):
        if os.path.isfile(path) and os.access(path, os.X_OK):
            return path
    return None


def _line_number(raw: bytes, offset: int) -> int:
    return raw[: max(0, offset)].count(b"\n") + 1


def _clean_name(name: str) -> str:
    return (name or "").strip()


def _base_name(name: str) -> str:
    return _clean_name(name).split("(", 1)[0]


def _extract_preceding_doc_comment(lines: list[str], start_line: int) -> str | None:
    if start_line <= 1:
        return None
    idx = start_line - 2
    while idx >= 0 and not lines[idx].strip():
        idx -= 1
    if idx < 0:
        return None
    line = lines[idx].strip()
    if line.startswith("///"):
        collected: list[str] = []
        while idx >= 0:
            current = lines[idx].strip()
            if not current.startswith("///"):
                break
            collected.append(current[3:].lstrip())
            idx -= 1
        collected.reverse()
        text = "\n".join(part.rstrip() for part in collected).strip()
        return text or None
    if line.endswith("*/"):
        collected = []
        while idx >= 0:
            current = lines[idx].rstrip()
            collected.append(current)
            if "/**" in current:
                break
            idx -= 1
        if collected and "/**" in collected[-1]:
            collected.reverse()
            normalized: list[str] = []
            for part in collected:
                piece = part.strip()
                piece = piece.removeprefix("/**").removesuffix("*/").strip()
                if piece.startswith("*"):
                    piece = piece[1:].lstrip()
                if piece:
                    normalized.append(piece)
            text = "\n".join(normalized).strip()
            return text or None
    return None


def _clean_inherited_type_name(name: str) -> str:
    cleaned = _clean_name(name)
    if not cleaned:
        return ""
    cleaned = cleaned.split("<", 1)[0].strip()
    cleaned = cleaned.split(".", 1)[-1].strip()
    cleaned = cleaned.split(":", 1)[0].strip()
    cleaned = cleaned.split("&", 1)[0].strip()
    return cleaned


def _extract_symbol_records_from_structure_data(
    structure: dict[str, Any], filepath: str, raw: bytes
) -> list[SwiftSymbolRecord]:
    records: list[SwiftSymbolRecord] = []
    lines = raw.decode("utf-8", errors="ignore").splitlines()

    def _walk(items: list[dict[str, Any]]) -> None:
        for item in items or []:
            kind = _clean_name(item.get("key.kind", ""))
            name = _clean_name(item.get("key.name", ""))
            offset = int(item.get("key.offset", 0) or 0)
            length = int(item.get("key.length", 0) or 0)
            start_line = _line_number(raw, offset)
            end_line = _line_number(raw, offset + max(0, length))
            inherited_types: list[str] = []
            for entry in item.get("key.inheritedtypes") or []:
                inherited = _clean_inherited_type_name((entry or {}).get("key.name", ""))
                if inherited:
                    inherited_types.append(inherited)
            if kind.startswith("source.lang.swift.decl") and name:
                records.append(
                    SwiftSymbolRecord(
                        filepath=filepath,
                        name=name,
                        base_name=_base_name(name),
                        kind=kind,
                        start_line=start_line,
                        end_line=end_line,
                        usr=_clean_name(item.get("key.usr", "")) or None,
                        doc_comment=(
                            _clean_name(item.get("key.doc.comment", "")) or
                            _extract_preceding_doc_comment(lines, start_line)
                        ),
                        inherited_types=sorted(set(inherited_types)),
                    )
                )
            _walk(item.get("key.substructure") or [])

    _walk(structure.get("key.substructure") or [])
    return records


def _extract_swift_structure_records(file_path: str) -> list[SwiftSymbolRecord]:
    sourcekitten = _which("sourcekitten")
    if not sourcekitten:
        return []
    try:
        with open(file_path, "rb") as fh:
            raw = fh.read()
        proc = subprocess.run(
            [sourcekitten, "structure", "--file", file_path],
            capture_output=True,
            text=True,
            timeout=60,
            check=True,
        )
        return _extract_symbol_records_from_structure_data(json.loads(proc.stdout), file_path, raw)
    except Exception:
        return []


def _clean_path_list(value: Any) -> list[str]:
    if not value:
        return []
    if isinstance(value, str):
        return [part for part in shlex.split(value) if part]
    if isinstance(value, list):
        return [str(part) for part in value if str(part).strip()]
    return []


def _clean_define_list(value: Any) -> list[str]:
    if not value:
        return []
    if isinstance(value, str):
        return [part for part in value.split() if part]
    if isinstance(value, list):
        return [str(part).strip() for part in value if str(part).strip()]
    return []


def _xcode_build_settings(project_file: str, scheme_name: str) -> list[dict[str, Any]]:
    xcodebuild = _which("xcodebuild")
    if not xcodebuild:
        return []
    project_bundle = project_file
    if project_bundle.endswith("/project.pbxproj"):
        project_bundle = str(Path(project_bundle).parent)
    try:
        proc = subprocess.run(
            [
                xcodebuild,
                "-project",
                project_bundle,
                "-scheme",
                scheme_name,
                "-destination",
                "platform=macOS",
                "-showBuildSettings",
                "-json",
            ],
            capture_output=True,
            text=True,
            timeout=120,
            check=True,
        )
        return json.loads(proc.stdout)
    except Exception:
        return []


def _compiler_args_from_build_settings(build_settings: dict[str, Any]) -> list[str]:
    args: list[str] = []
    sdkroot = build_settings.get("SDKROOT")
    if sdkroot:
        args.extend(["-sdk", str(sdkroot)])
    module_name = build_settings.get("PRODUCT_MODULE_NAME") or build_settings.get("TARGET_NAME")
    if module_name:
        args.extend(["-module-name", str(module_name)])
    for define in _clean_define_list(build_settings.get("SWIFT_ACTIVE_COMPILATION_CONDITIONS")):
        args.extend(["-D", define])
    for path in _clean_path_list(build_settings.get("FRAMEWORK_SEARCH_PATHS")):
        args.extend(["-F", path])
    for path in _clean_path_list(build_settings.get("HEADER_SEARCH_PATHS")):
        args.extend(["-I", path])
    for path in _clean_path_list(build_settings.get("SWIFT_INCLUDE_PATHS")):
        args.extend(["-I", path])
    args.extend(_clean_path_list(build_settings.get("OTHER_SWIFT_FLAGS")))
    return args


def _candidate_xcode_projects(project_path: str) -> list[str]:
    root = Path(project_path)
    discovered: set[str] = set()
    skip_dirs = {".git", ".build", "build", "DerivedData"}
    for path in root.rglob("*.xcodeproj"):
        if skip_dirs.intersection(path.parts):
            continue
        pbxproj = path / "project.pbxproj"
        if pbxproj.is_file():
            discovered.add(str(pbxproj))
    return sorted(discovered)


def _target_swift_files(project_root: str, target_name: str) -> list[str]:
    target_dir = os.path.join(project_root, target_name)
    if os.path.isdir(target_dir):
        return sorted(str(path) for path in Path(target_dir).rglob("*.swift"))
    return []


def _semantic_index_records(file_path: str, compiler_args: list[str], target_files: list[str]) -> list[SwiftSymbolRecord]:
    sourcekitten = _which("sourcekitten")
    if not sourcekitten or not target_files:
        return []
    try:
        proc = subprocess.run(
            ["sourcekitten", "index", "--file", file_path, "--", *compiler_args, *target_files],
            capture_output=True,
            text=True,
            timeout=180,
            check=True,
        )
        data = json.loads(proc.stdout)
    except Exception:
        return []

    records: list[SwiftSymbolRecord] = []
    seen: set[tuple[str, str]] = set()
    stack: list[Any] = [data]
    while stack:
        item = stack.pop(0)
        if isinstance(item, dict):
            name = _clean_name(item.get("key.name", ""))
            usr = _clean_name(item.get("key.usr", ""))
            if name and usr:
                key = (name, usr)
                if key not in seen:
                    seen.add(key)
                    records.append(
                        SwiftSymbolRecord(
                            filepath=file_path,
                            name=name,
                            base_name=_base_name(name),
                            kind=_clean_name(item.get("key.kind", "")),
                            start_line=0,
                            end_line=0,
                            usr=usr,
                            doc_comment=None,
                            inherited_types=[],
                        )
                    )
            for value in item.values():
                if isinstance(value, (dict, list)):
                    stack.append(value)
        elif isinstance(item, list):
            stack[:0] = item
    return records


def extract_swift_semantic_facts(project_path: str) -> dict[str, list[dict[str, Any]]]:
    """Return Swift semantic facts keyed by project-relative filepath.

    Facts are best-effort and currently target Xcode-backed Swift projects. The
    extractor combines a fast SourceKitten `structure` pass with an Xcode-aware
    SourceKitten `index` pass to attach semantic USRs where possible.
    """

    project_root = os.path.abspath(project_path)
    out: dict[str, list[dict[str, Any]]] = {}

    for project_file in _candidate_xcode_projects(project_root):
        scheme_name = Path(project_file).parent.stem
        for entry in _xcode_build_settings(project_file, scheme_name):
            build_settings = entry.get("buildSettings") or {}
            target_name = entry.get("target") or build_settings.get("TARGET_NAME")
            if not target_name:
                continue
            target_files = _target_swift_files(project_root, str(target_name))
            if not target_files:
                continue
            compiler_args = _compiler_args_from_build_settings(build_settings)
            if not compiler_args:
                continue
            for abs_path in target_files:
                rel_path = os.path.relpath(abs_path, project_root).replace(os.sep, "/")
                structure_records = _extract_swift_structure_records(abs_path)
                semantic_records = _semantic_index_records(abs_path, compiler_args, target_files)
                semantic_usr_by_base_name: dict[str, str] = {}
                for record in semantic_records:
                    if record.base_name and record.usr and record.base_name not in semantic_usr_by_base_name:
                        semantic_usr_by_base_name[record.base_name] = record.usr
                merged: list[dict[str, Any]] = []
                for record in structure_records:
                    merged.append(
                        {
                            "filepath": rel_path,
                            "name": record.name,
                            "base_name": record.base_name,
                            "kind": record.kind,
                            "start_line": record.start_line,
                            "end_line": record.end_line,
                            "usr": semantic_usr_by_base_name.get(record.base_name) or record.usr,
                            "doc_comment": record.doc_comment,
                            "inherited_types": record.inherited_types,
                        }
                    )
                if merged:
                    out[rel_path] = merged
    return out
