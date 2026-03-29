from typing import Literal, TypeAlias, TypedDict

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
    "detect_language",
    "detect_language_from_content",
    "detect_language_from_extension",
    "detect_language_from_path",
    "download",
    "download_all",
    "downloaded_languages",
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
]

def get_binding(name: SupportedLanguage) -> object: ...
def get_language(name: SupportedLanguage) -> Language: ...
def get_parser(name: SupportedLanguage) -> Parser: ...
def available_languages() -> list[str]: ...
def has_language(name: str) -> bool: ...
def language_count() -> int: ...
def parse_string(language: str, source: str) -> TreeHandle: ...
def process(source: str, config: ProcessConfig) -> ProcessResult: ...
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
def extension_ambiguity(ext: str) -> AmbiguityResult | None: ...
def get_highlights_query(language: str) -> str | None: ...
def get_injections_query(language: str) -> str | None: ...
def get_locals_query(language: str) -> str | None: ...
