from __future__ import annotations

from pathlib import PurePosixPath
from typing import Any, TypeAlias
from xml.etree import ElementTree

from tree_sitter_language_pack import _native as _native

DownloadError = _native.DownloadError
LanguageNotFoundError = _native.LanguageNotFoundError
ParseError = _native.ParseError
ProcessConfig = _native.ProcessConfig
QueryError = _native.QueryError
TreeHandle = _native.TreeHandle
available_languages = _native.available_languages
cache_dir = _native.cache_dir
clean_cache = _native.clean_cache
configure = _native.configure
detect_language = _native.detect_language
download = _native.download
download_all = _native.download_all
downloaded_languages = _native.downloaded_languages
get_binding = _native.get_binding
get_language = _native.get_language
get_parser = _native.get_parser
has_language = _native.has_language
index_workspace = _native.index_workspace
init = _native.init
language_count = _native.language_count
manifest_languages = _native.manifest_languages
parse_string = _native.parse_string
process = _native.process
extract = _native.extract
validate_extraction = _native.validate_extraction
extract_swift_semantic_facts = _native.extract_swift_semantic_facts
enrich_swift_graph = _native.enrich_swift_graph
finalize_struct_graph = _native.finalize_struct_graph


def _safe_list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def _clean_pbx_ref(value: Any) -> str:
    if not isinstance(value, str):
        return ""
    return value.split("/*", 1)[0].strip()


def _merge_fact_lists(base: dict[str, Any], extra: dict[str, list[dict[str, Any]]]) -> dict[str, Any]:
    merged = dict(base)
    for key, rows in extra.items():
        if not rows:
            continue
        current = _safe_list(merged.get(key))
        seen: set[str] = set()
        deduped: list[dict[str, Any]] = []
        for row in current + rows:
            marker = repr(sorted(row.items()))
            if marker in seen:
                continue
            seen.add(marker)
            deduped.append(row)
        merged[key] = deduped
    return merged


def _xml_root(source: bytes | str) -> ElementTree.Element | None:
    try:
        data = source.decode("utf-8", errors="ignore") if isinstance(source, bytes) else source
        return ElementTree.fromstring(data)
    except Exception:
        return None


def _extract_workspace_projects(source: bytes | str, file_path: str) -> dict[str, list[dict[str, Any]]]:
    root = _xml_root(source)
    if root is None:
        return {}
    workspace_path = file_path.replace("\\", "/")
    workspace_projects: list[dict[str, Any]] = []
    for file_ref in root.findall(".//FileRef"):
        location = (file_ref.attrib.get("location") or "").strip()
        if not location:
            continue
        if location.startswith("group:"):
            project_file = location.split(":", 1)[1]
        elif location.startswith("self:"):
            project_file = location.split(":", 1)[1] or PurePosixPath(workspace_path).parent.name
        else:
            project_file = location
        if not project_file:
            continue
        workspace_projects.append(
            {
                "workspace_path": workspace_path,
                "project_file": project_file.replace("\\", "/"),
            }
        )
    return {"apple_workspace_projects": workspace_projects}


def _extract_scheme_targets(source: bytes | str, file_path: str) -> dict[str, list[dict[str, Any]]]:
    root = _xml_root(source)
    if root is None:
        return {}
    scheme_path = file_path.replace("\\", "/")
    scheme_name = PurePosixPath(scheme_path).stem
    scheme_targets: list[dict[str, Any]] = []
    for buildable in root.findall(".//BuildableReference"):
        target_id = _clean_pbx_ref(buildable.attrib.get("BlueprintIdentifier") or "")
        container = (buildable.attrib.get("ReferencedContainer") or "").strip()
        if not target_id:
            continue
        scheme_targets.append(
            {
                "scheme_path": scheme_path,
                "scheme_name": scheme_name,
                "container_path": container.replace("\\", "/"),
                "target_id": target_id,
            }
        )
    return {"apple_scheme_targets": scheme_targets}


def _extract_pbxproj_facts(file_path: str) -> dict[str, list[dict[str, Any]]]:
    try:
        from pbxproj import XcodeProject
    except Exception:
        return {}
    try:
        project = XcodeProject.load(file_path)
    except Exception:
        return {}

    project_path = file_path.replace("\\", "/")
    objects = project.objects
    targets: list[dict[str, Any]] = []
    bundled_files: list[dict[str, Any]] = []
    synced_groups: list[dict[str, Any]] = []

    for target in objects.get_targets():
        target_id = _clean_pbx_ref(target.get_id())
        target_name = (target.get("name", None) or "").strip()
        if not target_name:
            continue
        targets.append(
            {
                "target_id": target_id,
                "name": target_name,
                "project_file": project_path,
            }
        )
        for group_ref in target.get("fileSystemSynchronizedGroups", None) or []:
            cleaned_group_ref = _clean_pbx_ref(group_ref)
            group = objects[cleaned_group_ref]
            group_path = (group.get("path", None) or "").strip()
            if not group_path:
                continue
            synced_groups.append(
                {
                    "target_id": target_id,
                    "group_path": group_path.replace("\\", "/"),
                }
            )

    build_files_by_id = {
        _clean_pbx_ref(build_file.get_id()): build_file
        for build_file in objects.get_objects_in_section("PBXBuildFile")
    }
    file_refs_by_id = {
        _clean_pbx_ref(file_ref.get_id()): file_ref
        for file_ref in objects.get_objects_in_section("PBXFileReference")
    }

    for phase in objects.get_objects_in_section("PBXResourcesBuildPhase"):
        files = phase.get("files", None) or []
        owner_target_id = None
        for target in objects.get_targets():
            build_phases = [_clean_pbx_ref(ref) for ref in (target.get("buildPhases", None) or [])]
            if _clean_pbx_ref(phase.get_id()) in build_phases:
                owner_target_id = _clean_pbx_ref(target.get_id())
                break
        if owner_target_id is None:
            continue
        for build_file_ref in files:
            build_file = build_files_by_id.get(_clean_pbx_ref(build_file_ref))
            if build_file is None:
                continue
            file_ref_id = _clean_pbx_ref(build_file.get("fileRef", None) or build_file.get("productRef", None))
            file_ref = file_refs_by_id.get(file_ref_id)
            if file_ref is None:
                continue
            rel_path = (file_ref.get("path", None) or "").strip()
            if not rel_path:
                continue
            bundled_files.append(
                {
                    "target_id": owner_target_id,
                    "filepath": rel_path.replace("\\", "/"),
                }
            )

    return {
        "apple_targets": targets,
        "apple_bundled_files": bundled_files,
        "apple_synced_groups": synced_groups,
    }


def _normalize_apple_facts(facts: dict[str, Any]) -> dict[str, Any]:
    for row in _safe_list(facts.get("apple_targets")):
        row["target_id"] = _clean_pbx_ref(row.get("target_id"))
    for row in _safe_list(facts.get("apple_bundled_files")):
        row["target_id"] = _clean_pbx_ref(row.get("target_id"))
    for row in _safe_list(facts.get("apple_synced_groups")):
        row["target_id"] = _clean_pbx_ref(row.get("target_id"))
    for row in _safe_list(facts.get("apple_scheme_targets")):
        row["target_id"] = _clean_pbx_ref(row.get("target_id"))
    return facts


def extract_file_facts(source: bytes | str, file_path: str, language: str | None = None) -> dict[str, Any]:
    facts = _native.extract_file_facts(source, file_path, language) or {}
    normalized_path = file_path.replace("\\", "/")
    extras: dict[str, list[dict[str, Any]]] = {}
    if normalized_path.endswith(".xcodeproj/project.pbxproj"):
        extras = _extract_pbxproj_facts(file_path)
    elif normalized_path.endswith(".xcworkspace/contents.xcworkspacedata"):
        extras = _extract_workspace_projects(source, normalized_path)
    elif normalized_path.endswith(".xcscheme"):
        extras = _extract_scheme_targets(source, normalized_path)
    return _normalize_apple_facts(_merge_fact_lists(facts, extras))


from ._semantic_payload import (
    CODEBASE_EMBEDDINGS_UPSERT_SQL,
    build_codebase_embedding_rows,
    build_line_window_chunks as _python_build_line_window_chunks,
    build_semantic_payload,
    build_semantic_sync_plan,
    build_swift_chunks as _python_build_swift_chunks,
    execute_codebase_embedding_upsert as _python_execute_codebase_embedding_upsert,
    execute_semantic_index_driver as _python_execute_semantic_index_driver,
)

build_line_window_chunks = getattr(
    _native,
    "build_line_window_chunks",
    _python_build_line_window_chunks,
)
build_swift_chunks = getattr(
    _native,
    "build_swift_chunks",
    _python_build_swift_chunks,
)
execute_semantic_index_driver = getattr(
    _native,
    "execute_semantic_index_driver",
    _python_execute_semantic_index_driver,
)
execute_codebase_embedding_upsert = getattr(
    _native,
    "execute_codebase_embedding_upsert",
    _python_execute_codebase_embedding_upsert,
)

try:
    detect_language_from_extension = _native.detect_language_from_extension
except Exception:

    def detect_language_from_extension(_: str) -> str | None:
        return None


try:
    detect_language_from_path = _native.detect_language_from_path
except Exception:

    def detect_language_from_path(_: str) -> str | None:
        return None


try:
    detect_language_from_content = _native.detect_language_from_content
except Exception:

    def detect_language_from_content(_: bytes | str) -> str | None:
        raise NotImplementedError("detect_language_from_content is not available in this build")


SupportedLanguage: TypeAlias = str

__all__ = [
    "DownloadError",
    "LanguageNotFoundError",
    "ParseError",
    "ProcessConfig",
    "QueryError",
    "SupportedLanguage",
    "TreeHandle",
    "available_languages",
    "cache_dir",
    "clean_cache",
    "configure",
    "CODEBASE_EMBEDDINGS_UPSERT_SQL",
    "build_codebase_embedding_rows",
    "build_line_window_chunks",
    "build_semantic_payload",
    "build_semantic_sync_plan",
    "build_swift_chunks",
    "detect_language",
    "detect_language_from_content",
    "detect_language_from_extension",
    "detect_language_from_path",
    "download",
    "download_all",
    "downloaded_languages",
    "execute_codebase_embedding_upsert",
    "execute_semantic_index_driver",
    "extract",
    "extract_file_facts",
    "extract_swift_semantic_facts",
    "get_binding",
    "get_language",
    "get_parser",
    "has_language",
    "index_workspace",
    "init",
    "language_count",
    "manifest_languages",
    "parse_string",
    "process",
    "validate_extraction",
]
