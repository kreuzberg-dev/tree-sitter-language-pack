from typing import Any, Literal, TypeAlias, TypedDict

from tree_sitter import Language, Parser

class LanguageNotFoundError(ValueError): ...
class DownloadError(RuntimeError): ...

SupportedLanguage: TypeAlias = Literal[
    "actionscript",
    "ada",
    "agda",
    "apex",
    "arduino",
    "asm",
    "astro",
    "bash",
    "batch",
    "bazel",
    "beancount",
    "bibtex",
    "bicep",
    "bitbake",
    "bsl",
    "c",
    "cairo",
    "capnp",
    "chatito",
    "clarity",
    "clojure",
    "cmake",
    "cobol",
    "comment",
    "commonlisp",
    "cpon",
    "cpp",
    "css",
    "csv",
    "cuda",
    "d",
    "dart",
    "diff",
    "dockerfile",
    "doxygen",
    "dtd",
    "elisp",
    "elixir",
    "elm",
    "erlang",
    "fennel",
    "firrtl",
    "fish",
    "fortran",
    "fsharp",
    "fsharp_signature",
    "func",
    "gdscript",
    "gitattributes",
    "gitcommit",
    "gitignore",
    "gleam",
    "glsl",
    "gn",
    "go",
    "gomod",
    "gosum",
    "gradle",
    "graphql",
    "groovy",
    "gstlaunch",
    "hack",
    "hare",
    "haskell",
    "haxe",
    "hcl",
    "heex",
    "hlsl",
    "html",
    "hyprlang",
    "ignorefile",
    "ini",
    "ispc",
    "janet",
    "java",
    "javascript",
    "jsdoc",
    "json",
    "jsonnet",
    "julia",
    "kconfig",
    "kdl",
    "kotlin",
    "latex",
    "linkerscript",
    "lisp",
    "llvm",
    "lua",
    "luadoc",
    "luap",
    "luau",
    "magik",
    "make",
    "makefile",
    "markdown",
    "markdown_inline",
    "matlab",
    "mermaid",
    "meson",
    "netlinx",
    "nim",
    "ninja",
    "nix",
    "nqc",
    "objc",
    "ocaml",
    "ocaml_interface",
    "odin",
    "org",
    "pascal",
    "pem",
    "perl",
    "pgn",
    "php",
    "pkl",
    "po",
    "pony",
    "powershell",
    "printf",
    "prisma",
    "properties",
    "proto",
    "psv",
    "puppet",
    "purescript",
    "pymanifest",
    "python",
    "qmldir",
    "qmljs",
    "query",
    "r",
    "racket",
    "re2c",
    "readline",
    "rego",
    "requirements",
    "ron",
    "rst",
    "ruby",
    "rust",
    "scala",
    "scheme",
    "scss",
    "shell",
    "smali",
    "smithy",
    "solidity",
    "sparql",
    "sql",
    "squirrel",
    "starlark",
    "svelte",
    "swift",
    "tablegen",
    "tcl",
    "terraform",
    "test",
    "thrift",
    "toml",
    "tsv",
    "tsx",
    "twig",
    "typescript",
    "typst",
    "udev",
    "ungrammar",
    "uxntal",
    "v",
    "verilog",
    "vhdl",
    "vim",
    "vue",
    "wast",
    "wat",
    "wgsl",
    "xcompose",
    "xml",
    "yuck",
    "zig",
]

class ParseError(RuntimeError): ...
class QueryError(ValueError): ...

class Span(TypedDict):
    start_byte: int
    end_byte: int
    start_row: int
    start_col: int
    end_row: int
    end_col: int

class NodeInfo(TypedDict):
    kind: str
    is_named: bool
    start_byte: int
    end_byte: int
    start_row: int
    start_col: int
    end_row: int
    end_col: int
    named_child_count: int
    is_error: bool
    is_missing: bool

class FileMetrics(TypedDict):
    total_lines: int
    total_bytes: int
    blank_lines: int
    comment_lines: int
    code_lines: int
    error_count: int

class RouteDefFact(TypedDict):
    framework: str
    method: str
    path: str

class HttpCallFact(TypedDict):
    client: str
    method: str
    path: str

class ResourceRefFact(TypedDict):
    kind: str
    name: str
    callee: str

class AppleTargetFact(TypedDict):
    target_id: str
    name: str
    project_file: str

class AppleBundledFileFact(TypedDict):
    target_id: str
    filepath: str

class AppleSyncedGroupFact(TypedDict):
    target_id: str
    group_path: str

class AppleWorkspaceProjectFact(TypedDict):
    workspace_path: str
    project_file: str

class AppleSchemeTargetFact(TypedDict):
    scheme_path: str
    scheme_name: str
    container_path: str
    target_id: str

class SwiftSemanticFact(TypedDict):
    filepath: str
    name: str
    base_name: str
    kind: str
    start_line: int
    end_line: int
    usr: str | None
    doc_comment: str | None
    inherited_types: list[str]

class FileFacts(TypedDict, total=False):
    route_defs: list[RouteDefFact]
    http_calls: list[HttpCallFact]
    resource_refs: list[ResourceRefFact]
    apple_targets: list[AppleTargetFact]
    apple_bundled_files: list[AppleBundledFileFact]
    apple_synced_groups: list[AppleSyncedGroupFact]
    apple_workspace_projects: list[AppleWorkspaceProjectFact]
    apple_scheme_targets: list[AppleSchemeTargetFact]

class StructureItem(TypedDict):
    kind: str  # "Function", "Class", "Method", etc.
    name: str
    span: Span
    parent: str | None

class ImportInfo(TypedDict):
    module: str
    names: list[str]
    span: Span

class ExportInfo(TypedDict):
    name: str
    kind: str
    span: Span

class CommentInfo(TypedDict):
    text: str
    kind: str  # "Line", "Block", "Doc"
    span: Span
    associated_node: str | None

class DocstringInfo(TypedDict):
    text: str
    format: str
    span: Span
    associated_item: str | None
    sections: list[dict[str, str]]

class SymbolInfo(TypedDict):
    name: str
    kind: str
    span: Span
    type_annotation: str | None

class Diagnostic(TypedDict):
    message: str
    severity: str
    span: Span

class ChunkContext(TypedDict):
    language: str
    chunk_index: int
    total_chunks: int
    start_line: int
    end_line: int
    node_types: list[str]
    symbols_defined: list[str]
    comments: list[str]
    docstrings: list[str]
    has_error_nodes: bool
    context_path: list[str]

class CodeChunk(TypedDict):
    content: str
    start_byte: int
    end_byte: int
    metadata: ChunkContext

class ProcessResult(TypedDict):
    language: str
    metrics: FileMetrics
    structure: list[StructureItem]
    imports: list[ImportInfo]
    exports: list[ExportInfo]
    comments: list[CommentInfo]
    docstrings: list[DocstringInfo]
    symbols: list[SymbolInfo]
    diagnostics: list[Diagnostic]
    chunks: list[CodeChunk]
    extractions: dict[str, Any]

class QueryCapture(TypedDict):
    capture_name: str
    node: NodeInfo

class QueryMatch(TypedDict):
    pattern_index: int
    captures: list[QueryCapture]

class AmbiguityResult(TypedDict):
    assigned: str
    alternatives: list[str]

class ProcessConfig:
    language: str
    structure: bool
    imports: bool
    exports: bool
    comments: bool
    docstrings: bool
    symbols: bool
    diagnostics: bool
    chunk_max_size: int | None

    def __init__(
        self,
        language: str,
        *,
        structure: bool = True,
        imports: bool = True,
        exports: bool = True,
        comments: bool = True,
        docstrings: bool = True,
        symbols: bool = True,
        diagnostics: bool = True,
        chunk_max_size: int | None = None,
    ) -> None: ...
    @staticmethod
    def all(language: str) -> ProcessConfig: ...
    @staticmethod
    def minimal(language: str) -> ProcessConfig: ...

class TreeHandle:
    def root_node_type(self) -> str: ...
    def root_child_count(self) -> int: ...
    def contains_node_type(self, node_type: str) -> bool: ...
    def has_error_nodes(self) -> bool: ...
    def to_sexp(self) -> str: ...
    def error_count(self) -> int: ...
    def root_node_info(self) -> NodeInfo: ...
    def find_nodes_by_type(self, node_type: str) -> list[NodeInfo]: ...
    def named_children_info(self) -> list[NodeInfo]: ...
    def extract_text(self, start_byte: int, end_byte: int) -> str: ...
    def run_query(self, language: str, query_source: str) -> list[QueryMatch]: ...

__all__ = [
    "AmbiguityResult",
    "ChunkContext",
    "CodeChunk",
    "CommentInfo",
    "CODEBASE_EMBEDDINGS_UPSERT_SQL",
    "Diagnostic",
    "DocstringInfo",
    "DownloadError",
    "ExportInfo",
    "FileMetrics",
    "ImportInfo",
    "LanguageNotFoundError",
    "NodeInfo",
    "ParseError",
    "ProcessConfig",
    "ProcessResult",
    "QueryCapture",
    "QueryError",
    "QueryMatch",
    "Span",
    "StructureItem",
    "SupportedLanguage",
    "SymbolInfo",
    "TreeHandle",
    "available_languages",
    "cache_dir",
    "clean_cache",
    "configure",
    "build_codebase_embedding_rows",
    "build_indexing_chunks",
    "build_line_window_chunks",
    "should_use_line_window_fallback",
    "build_swift_chunks",
    "build_semantic_sync_plan",
    "detect_language",
    "detect_language_from_content",
    "detect_language_from_extension",
    "detect_language_from_path",
    "download",
    "download_all",
    "downloaded_languages",
    "build_semantic_payload",
    "execute_codebase_embedding_upsert",
    "execute_semantic_index_driver",
    "extract",
    "enrich_swift_graph",
    "extract_swift_semantic_facts",
    "finalize_struct_graph",
    "extension_ambiguity",
    "get_binding",
    "get_highlights_query",
    "get_injections_query",
    "get_language",
    "get_locals_query",
    "get_parser",
    "has_language",
    "init",
    "language_count",
    "manifest_languages",
    "parse_string",
    "process",
    "validate_extraction",
]

def get_binding(name: SupportedLanguage) -> object: ...
def get_language(name: SupportedLanguage) -> Language: ...
def get_parser(name: SupportedLanguage) -> Parser: ...
def available_languages() -> list[str]: ...
def has_language(name: str) -> bool: ...
def language_count() -> int: ...
def parse_string(language: str, source: str) -> TreeHandle: ...
def process(source: str, config: ProcessConfig) -> ProcessResult: ...
def build_semantic_payload(
    source: str,
    language: str,
    file_path: str,
    project_id: str,
    *,
    chunk_id_version: str = "v6",
    chunk_max_size: int = 4000,
    chunk_overlap: int = 200,
) -> dict[str, Any]: ...
def build_line_window_chunks(
    source: str,
    file_path: str,
    project_id: str,
    *,
    language: str | None = None,
    file_meta: dict[str, Any] | None = None,
    chunk_id_version: str = "v6",
    chunk_lines: int = 60,
    overlap_lines: int = 10,
) -> list[dict[str, Any]]: ...
def build_swift_chunks(
    source: str,
    file_path: str,
    project_id: str,
    *,
    file_meta: dict[str, Any] | None = None,
    chunk_id_version: str = "v6",
    chunk_max_size: int = 4000,
    chunk_lines: int = 60,
    overlap_lines: int = 10,
) -> list[dict[str, Any]]: ...
def should_use_line_window_fallback(file_path: str) -> bool: ...
def build_indexing_chunks(
    source: str,
    file_path: str,
    project_id: str,
    *,
    language: str | None = None,
    chunk_id_version: str = "v6",
    chunk_max_size: int = 4000,
    chunk_overlap: int = 200,
    chunk_lines: int = 60,
    overlap_lines: int = 10,
) -> dict[str, Any]: ...
def build_semantic_sync_plan(
    all_chunks: list[list[dict[str, Any]]],
    existing_ids: set[str] | None = None,
) -> dict[str, Any]: ...
def build_codebase_embedding_rows(
    batch: list[dict[str, Any]],
    project_id: str,
    *,
    expected_dim: int | None = None,
    created_at: float | None = None,
) -> list[tuple[Any, ...]]: ...
CODEBASE_EMBEDDINGS_UPSERT_SQL: str
async def execute_codebase_embedding_upsert(
    cursor: Any,
    batch: list[dict[str, Any]],
    project_id: str,
    *,
    expected_dim: int | None = None,
    created_at: float | None = None,
) -> int: ...
async def execute_semantic_index_driver(
    conn: Any,
    project_id: str,
    manifest_paths: list[str],
    all_chunks: list[list[dict[str, Any]]],
    *,
    rebuild: bool = False,
    batch_size: int,
    concurrency: int,
    embed_batch_fn: Any,
    write_batch_fn: Any,
    progress_fn: Any | None = None,
) -> dict[str, Any]: ...
def extract(source: str, config: dict[str, object]) -> dict[str, Any]: ...
def validate_extraction(config: dict[str, object]) -> dict[str, Any]: ...
def extract_file_facts(source: str, language: str, file_path: str | None = None) -> FileFacts: ...
def extract_swift_semantic_facts(project_path: str) -> dict[str, list[SwiftSemanticFact]]: ...
def enrich_swift_graph(
    project_path: str,
    project_id: str,
    indexed_files: list[str],
    neo4j_uri: str,
    neo4j_user: str,
    neo4j_pass: str,
    neo4j_db: str = "proxy",
) -> dict[str, Any]: ...
def finalize_struct_graph(
    project_path: str,
    project_id: str,
    manifest_file: str,
    indexed_files: list[str],
    neo4j_uri: str,
    neo4j_user: str,
    neo4j_pass: str,
    neo4j_db: str = "proxy",
) -> dict[str, Any]: ...
def init(config: dict[str, object]) -> None: ...
def configure(*, cache_dir: str | None = None) -> None: ...
def download(names: list[str]) -> int: ...
def download_all() -> int: ...
def manifest_languages() -> list[str]: ...
def downloaded_languages() -> list[str]: ...
def clean_cache() -> None: ...
def cache_dir() -> str: ...
def detect_language(path: str) -> str | None: ...
def detect_language_from_content(content: str) -> str | None: ...
def detect_language_from_extension(ext: str) -> str | None: ...
def detect_language_from_path(path: str) -> str | None: ...
def index_workspace(
    *,
    path: str,
    project_id: str,
    neo4j_uri: str,
    neo4j_user: str,
    neo4j_pass: str,
    manifest_file: str,
) -> list[str]: ...
def extension_ambiguity(ext: str) -> AmbiguityResult | None: ...
def get_highlights_query(language: str) -> str | None: ...
def get_injections_query(language: str) -> str | None: ...
def get_locals_query(language: str) -> str | None: ...
