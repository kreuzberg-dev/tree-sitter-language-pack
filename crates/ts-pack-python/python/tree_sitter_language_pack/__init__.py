from typing import TypeAlias

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
    "detect_language",
    "detect_language_from_content",
    "detect_language_from_extension",
    "detect_language_from_path",
    "download",
    "download_all",
    "downloaded_languages",
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
]
