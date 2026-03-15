from typing import Literal, TypeAlias

from tree_sitter import Language, Parser

class LanguageNotFoundError(ValueError): ...

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
    def root_node_info(self) -> dict[str, object]: ...
    def find_nodes_by_type(self, node_type: str) -> list[dict[str, object]]: ...
    def named_children_info(self) -> list[dict[str, object]]: ...
    def extract_text(self, start_byte: int, end_byte: int) -> str: ...
    def run_query(self, language: str, query_source: str) -> list[dict[str, object]]: ...

__all__ = [
    "LanguageNotFoundError",
    "ParseError",
    "ProcessConfig",
    "QueryError",
    "SupportedLanguage",
    "TreeHandle",
    "available_languages",
    "get_binding",
    "get_language",
    "get_parser",
    "has_language",
    "language_count",
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
def process(source: str, config: ProcessConfig) -> dict[str, object]: ...
