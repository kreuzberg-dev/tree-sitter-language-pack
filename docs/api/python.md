---
description: "Python API reference for tree-sitter-language-pack"
---

# Python API Reference

## Installation

=== "pip"

    ```bash
    pip install tree-sitter-language-pack
    ```

=== "uv"

    ```bash
    uv add tree-sitter-language-pack
    ```

=== "poetry"

    ```bash
    poetry add tree-sitter-language-pack
    ```

## Quick Example

```python
from tree_sitter_language_pack import (
    available_languages,
    parse_string,
    process,
    ProcessConfig,
)

# List available languages
print(f"{len(available_languages())} languages available")

# Parse source code into a TreeHandle
tree = parse_string("python", "def hello(): pass")
print(tree.root_node_type())   # "module"
print(tree.has_error_nodes())  # False

# Extract code intelligence
config = ProcessConfig.all("python")
result = process("def hello(): pass", config)
print(result)
```

## Language Discovery

### `available_languages() -> list[str]`

Returns a list of all available language names, including statically compiled and dynamically loadable languages.

```python
from tree_sitter_language_pack import available_languages

langs = available_languages()
for lang in langs:
    print(lang)
```

### `has_language(name: str) -> bool`

Check whether a language is available by name or alias.

```python
from tree_sitter_language_pack import has_language

assert has_language("python")
assert has_language("shell")  # alias for "bash"
assert not has_language("nonexistent")
```

### `language_count() -> int`

Return the total number of available languages.

```python
from tree_sitter_language_pack import language_count

print(f"{language_count()} languages available")
```

### `detect_language(path: str) -> str | None`

Detect a language name from a file path or extension. Returns `None` if the extension is not recognized.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import detect_language`

```python
from tree_sitter_language_pack._native import detect_language

lang = detect_language("main.py")
print(lang)  # "python"

lang = detect_language("app.tsx")
print(lang)  # "tsx"

lang = detect_language("unknown.xyz")
print(lang)  # None
```

### `detect_language_from_content(content: str) -> str | None`

Detect a language name from file content using shebang-based detection. Returns `None` if no recognized shebang is found.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import detect_language_from_content`

```python
from tree_sitter_language_pack._native import detect_language_from_content

lang = detect_language_from_content("#!/usr/bin/env python3\nprint('hello')")
print(lang)  # "python"

lang = detect_language_from_content("no shebang here")
print(lang)  # None
```

### `detect_language_from_extension(ext: str) -> str | None`

Detect a language name from a bare file extension (without the leading dot). Returns `None` if the extension is not recognized.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import detect_language_from_extension`

```python
from tree_sitter_language_pack._native import detect_language_from_extension

lang = detect_language_from_extension("py")
print(lang)  # "python"

lang = detect_language_from_extension("tsx")
print(lang)  # "tsx"

lang = detect_language_from_extension("xyz")
print(lang)  # None
```

### `detect_language_from_path(path: str) -> str | None`

Detect a language name from a file path based on its extension. Returns `None` if the extension is not recognized.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import detect_language_from_path`

```python
from tree_sitter_language_pack._native import detect_language_from_path

lang = detect_language_from_path("/home/user/project/main.py")
print(lang)  # "python"

lang = detect_language_from_path("src/app.tsx")
print(lang)  # "tsx"

lang = detect_language_from_path("unknown.xyz")
print(lang)  # None
```

### `extension_ambiguity(ext: str) -> tuple[str, list[str]] | None`

Returns extension ambiguity information for a given file extension. If the extension maps to multiple possible languages, returns a tuple of `(assigned_language, alternative_languages)`. Returns `None` if the extension is not ambiguous.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import extension_ambiguity`

```python
from tree_sitter_language_pack._native import extension_ambiguity

result = extension_ambiguity("h")
if result:
    assigned, alternatives = result
    print(f"Assigned: {assigned}, Alternatives: {alternatives}")
```

## Queries

### `get_highlights_query(language: str) -> str | None`

Returns the bundled highlights query for the given language, or `None` if no query is available.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import get_highlights_query`

```python
from tree_sitter_language_pack._native import get_highlights_query

query = get_highlights_query("python")
if query:
    print(query[:100])  # first 100 chars of the highlights query
```

### `get_injections_query(language: str) -> str | None`

Returns the bundled injections query for the given language, or `None` if no query is available.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import get_injections_query`

```python
from tree_sitter_language_pack._native import get_injections_query

query = get_injections_query("html")
if query:
    print(query[:100])
```

### `get_locals_query(language: str) -> str | None`

Returns the bundled locals query for the given language, or `None` if no query is available.

!!! note
    This function is available from the native module: `from tree_sitter_language_pack._native import get_locals_query`

```python
from tree_sitter_language_pack._native import get_locals_query

query = get_locals_query("python")
if query:
    print(query[:100])
```

## Parsing

### `parse_string(language: str, source: str) -> TreeHandle`

Parse source code with the named language. Returns a `TreeHandle` wrapping the parsed syntax tree.

**Parameters:**

- `language` (str): Language name (first argument)
- `source` (str): Source code to parse (second argument)

**Raises:**

- `ParseError`: If parsing fails
- `LanguageNotFoundError`: If the language is not recognized

```python
from tree_sitter_language_pack import parse_string

tree = parse_string("python", "def hello(): pass")
print(tree.root_node_type())   # "module"
print(tree.root_child_count())  # 1
print(tree.has_error_nodes())   # False
```

## TreeHandle

`TreeHandle` is an opaque wrapper around a parsed tree-sitter syntax tree. It is returned by `parse_string` and provides methods for inspecting and querying the tree.

### `root_node_type() -> str`

Returns the type name of the root node.

```python
tree = parse_string("python", "x = 1")
print(tree.root_node_type())  # "module"
```

### `root_child_count() -> int`

Returns the number of named children of the root node.

```python
tree = parse_string("python", "x = 1\ny = 2")
print(tree.root_child_count())  # 2
```

### `contains_node_type(node_type: str) -> bool`

Check whether any node in the tree has the given type name.

```python
tree = parse_string("python", "def hello(): pass")
print(tree.contains_node_type("function_definition"))  # True
print(tree.contains_node_type("class_definition"))      # False
```

### `has_error_nodes() -> bool`

Check whether the tree contains any ERROR or MISSING nodes.

```python
tree = parse_string("python", "def hello(): pass")
print(tree.has_error_nodes())  # False
```

### `error_count() -> int`

Returns the count of ERROR and MISSING nodes in the tree.

```python
tree = parse_string("python", "def hello(): pass")
print(tree.error_count())  # 0
```

### `to_sexp() -> str`

Returns the S-expression representation of the tree.

```python
tree = parse_string("python", "x = 1")
print(tree.to_sexp())
# (module (expression_statement (assignment left: (identifier) right: (integer))))
```

### `root_node_info() -> dict`

Returns information about the root node as a dictionary with the following keys:

- `kind` (str): Node type name
- `is_named` (bool): Whether the node is named
- `start_byte` (int): Start byte offset
- `end_byte` (int): End byte offset
- `start_row` (int): Start row (0-indexed)
- `start_column` (int): Start column (0-indexed)
- `end_row` (int): End row (0-indexed)
- `end_column` (int): End column (0-indexed)
- `named_child_count` (int): Number of named children
- `is_error` (bool): Whether the node is an ERROR node
- `is_missing` (bool): Whether the node is a MISSING node

```python
tree = parse_string("python", "x = 1")
info = tree.root_node_info()
print(info["kind"])              # "module"
print(info["named_child_count"]) # 1
```

### `find_nodes_by_type(node_type: str) -> list[dict]`

Finds all nodes matching the given type and returns their info as a list of dictionaries. Each dictionary has the same keys as `root_node_info`.

```python
tree = parse_string("python", "x = 1\ny = 2")
nodes = tree.find_nodes_by_type("identifier")
for node in nodes:
    print(f"{node['kind']} at {node['start_row']}:{node['start_column']}")
```

### `named_children_info() -> list[dict]`

Returns info for all named children of the root node. Each dictionary has the same keys as `root_node_info`.

```python
tree = parse_string("python", "x = 1\ndef foo(): pass")
children = tree.named_children_info()
for child in children:
    print(f"{child['kind']} ({child['start_row']}:{child['start_column']})")
```

### `extract_text(start_byte: int, end_byte: int) -> str`

Extracts source text for a byte range. Use `start_byte` and `end_byte` values from node info dictionaries.

**Raises:**

- `ParseError`: If the byte range is invalid

```python
tree = parse_string("python", "x = 1")
info = tree.root_node_info()
text = tree.extract_text(info["start_byte"], info["end_byte"])
print(text)  # "x = 1"
```

### `run_query(language: str, query_source: str) -> list[dict]`

Runs a tree-sitter query against the tree and returns matches. Each match is a dictionary with:

- `pattern_index` (int): Index of the matched pattern in the query
- `captures` (list[dict]): List of captures, each with `name` (str) and `node` (dict with same keys as `root_node_info`)

**Raises:**

- `QueryError`: If the query syntax is invalid

```python
tree = parse_string("python", "def hello(): pass\ndef world(): pass")
matches = tree.run_query("python", "(function_definition name: (identifier) @fn_name)")
for match in matches:
    for capture in match["captures"]:
        text = tree.extract_text(
            capture["node"]["start_byte"],
            capture["node"]["end_byte"],
        )
        print(f"{capture['name']}: {text}")
# fn_name: hello
# fn_name: world
```

## Processing

### `process(source: str, config: ProcessConfig) -> dict`

Process source code and extract code intelligence. Parses the source with tree-sitter and extracts structure, imports, exports, comments, docstrings, symbols, diagnostics, and/or chunks based on the config flags.

**Parameters:**

- `source` (str): Source code to analyze
- `config` (ProcessConfig): Analysis configuration

**Returns:** dict containing the requested analysis results

**Raises:**

- `ParseError`: If parsing or analysis fails

```python
from tree_sitter_language_pack import process, ProcessConfig

config = ProcessConfig.all("python")
result = process("def hello(): pass", config)
print(result)
```

## ProcessConfig

Configuration for the `process` function. Controls which analysis features are enabled and chunking behavior.

### Constructor

```python
ProcessConfig(
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
)
```

`language` is the only positional argument. All other parameters are keyword-only and default to `True` (except `chunk_max_size`, which defaults to `None` for no chunking).

**Fields** (all readable and writable):

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `str` | (required) | Language name |
| `structure` | `bool` | `True` | Extract code structure |
| `imports` | `bool` | `True` | Extract imports |
| `exports` | `bool` | `True` | Extract exports |
| `comments` | `bool` | `True` | Extract comments |
| `docstrings` | `bool` | `True` | Extract docstrings |
| `symbols` | `bool` | `True` | Extract symbols |
| `diagnostics` | `bool` | `True` | Extract diagnostics |
| `chunk_max_size` | `int \| None` | `None` | Maximum chunk size (None disables chunking) |
| `extractions` | `dict \| None` | `None` | Custom extraction patterns (same shape as `extract` config `patterns`) |

### Static Methods

#### `ProcessConfig.all(language: str) -> ProcessConfig`

Create a config with all features enabled and no chunking.

```python
config = ProcessConfig.all("python")
```

#### `ProcessConfig.minimal(language: str) -> ProcessConfig`

Create a config with all features disabled (language only, no extraction).

```python
config = ProcessConfig.minimal("python")
```

### Example

```python
from tree_sitter_language_pack import process, ProcessConfig

# All features
result = process("def foo(): pass", ProcessConfig.all("python"))

# Minimal (metrics only, no extraction)
result = process("def foo(): pass", ProcessConfig.minimal("python"))

# Custom: structure and imports only, with chunking
config = ProcessConfig(
    "python",
    structure=True,
    imports=True,
    exports=False,
    comments=False,
    docstrings=False,
    symbols=False,
    diagnostics=False,
    chunk_max_size=2000,
)
result = process("import os\ndef foo(): pass", config)
```

## Extraction Queries

### `extract(source: str, config: dict) -> dict`

Run user-defined tree-sitter queries against source code and return structured results. Parses the source, executes all named patterns, and returns matches with captured nodes, text, and child fields.

**Parameters:**

- `source` (str): Source code string to parse and query
- `config` (dict): Extraction configuration with the following keys:
    - `language` (str): Language name (e.g., `"python"`)
    - `patterns` (dict): Mapping of pattern names to pattern config dicts

**Pattern config fields:**

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `str` | (required) | Tree-sitter S-expression query string |
| `capture_output` | `str` | `"Full"` | `"Text"` (text only), `"Node"` (node info only), or `"Full"` (both) |
| `child_fields` | `list[str]` | `[]` | Child field names to extract from captured nodes |
| `max_results` | `int \| None` | `None` | Maximum matches to return (None for unlimited) |
| `byte_range` | `[int, int] \| None` | `None` | Restrict matches to a `[start, end]` byte range |

**Returns:** dict with keys:

- `language` (str): The language used
- `results` (dict): Mapping of pattern names to result dicts, each containing:
    - `matches` (list[dict]): List of match dicts, each with `pattern_index` (int) and `captures` (list of capture dicts)
    - `total_count` (int): Total matches found (before `max_results` truncation)

Each capture dict contains: `name` (str), `node` (dict or None), `text` (str or None), `child_fields` (dict), `start_byte` (int).

**Raises:**

- `ParseError`: If parsing fails, the config is invalid, or serialization fails

```python
from tree_sitter_language_pack import extract

result = extract(
    "def hello(): pass\ndef world(): pass",
    {
        "language": "python",
        "patterns": {
            "functions": {
                "query": "(function_definition name: (identifier) @fn_name) @fn_def",
                "capture_output": "Full",
                "child_fields": ["name", "parameters"],
            }
        },
    },
)

for match in result["results"]["functions"]["matches"]:
    for capture in match["captures"]:
        if capture["text"]:
            print(f"{capture['name']}: {capture['text']}")
```

### `validate_extraction(config: dict) -> dict`

Validate an extraction config without executing it. Checks that the language exists and all query patterns compile successfully.

**Parameters:**

- `config` (dict): Same shape as the `config` parameter of `extract`

**Returns:** dict with keys:

- `valid` (bool): Whether all patterns are valid
- `patterns` (dict): Mapping of pattern names to validation dicts, each with:
    - `valid` (bool): Whether this pattern compiled
    - `capture_names` (list[str]): Capture names defined in the query
    - `pattern_count` (int): Number of patterns in the query
    - `warnings` (list[str]): Non-fatal warnings
    - `errors` (list[str]): Fatal errors (e.g., syntax errors)

**Raises:**

- `ParseError`: If the language cannot be loaded or the config is malformed

```python
from tree_sitter_language_pack import validate_extraction

result = validate_extraction({
    "language": "python",
    "patterns": {
        "functions": {
            "query": "(function_definition name: (identifier) @fn_name)",
        }
    },
})

assert result["valid"]
assert "fn_name" in result["patterns"]["functions"]["capture_names"]
```

## Download Management

### `init(config: dict) -> None`

Initialize the language pack with configuration. Applies cache directory settings and downloads languages/groups specified in the config dict.

**Parameters:**

- `config` (dict): Dictionary with optional keys:
    - `cache_dir` (str | None): Custom cache directory path
    - `languages` (list[str] | None): Languages to download
    - `groups` (list[str] | None): Language groups to download

**Raises:**

- `DownloadError`: If downloads fail or network is unavailable

```python
from tree_sitter_language_pack import init

# Download specific languages
init({"languages": ["python", "javascript", "rust"]})

# Set custom cache directory and download groups
init({"cache_dir": "/opt/ts-cache", "groups": ["web"]})

# Combine options
init({"languages": ["python"], "groups": ["web"], "cache_dir": "/opt/ts-cache"})
```

### `configure(*, cache_dir: str | None = None) -> None`

Set a custom cache directory without downloading anything. Pass `None` to reset to the default cache directory.

**Raises:**

- `DownloadError`: If the configuration cannot be applied

```python
from tree_sitter_language_pack import configure

configure(cache_dir="/opt/ts-pack-cache")
```

### `download(names: list[str]) -> int`

Download specific languages to the local cache. Returns the number of newly downloaded languages. Already-cached languages are skipped.

**Raises:**

- `DownloadError`: If any download fails

```python
from tree_sitter_language_pack import download

count = download(["python", "rust", "typescript"])
print(f"Downloaded {count} new languages")
```

### `download_all() -> int`

Download all available languages from the remote manifest. Returns the number of newly downloaded languages.

**Raises:**

- `DownloadError`: If the manifest cannot be fetched or downloads fail

```python
from tree_sitter_language_pack import download_all

count = download_all()
print(f"Downloaded {count} languages")
```

### `manifest_languages() -> list[str]`

Fetch all language names available in the remote manifest. Returns a sorted list.

**Raises:**

- `DownloadError`: If the manifest cannot be fetched

```python
from tree_sitter_language_pack import manifest_languages

languages = manifest_languages()
print(f"{len(languages)} languages available for download")
```

### `downloaded_languages() -> list[str]`

List languages already downloaded and cached locally. Does not perform any network requests.

```python
from tree_sitter_language_pack import downloaded_languages

cached = downloaded_languages()
print(f"Cached: {', '.join(cached)}")
```

### `clean_cache() -> None`

Delete all cached parser shared libraries.

**Raises:**

- `DownloadError`: If the cache directory cannot be removed

```python
from tree_sitter_language_pack import clean_cache

clean_cache()
```

### `cache_dir() -> str`

Return the effective cache directory path. Returns either the custom path set via `configure()` or the default.

**Raises:**

- `DownloadError`: If the cache directory cannot be determined

```python
from tree_sitter_language_pack import cache_dir

print(cache_dir())
```

## tree-sitter Interop

These functions return objects from the `tree-sitter` Python package, allowing direct use of the upstream tree-sitter API.

### `get_binding(name: str) -> PyCapsule`

Get a raw PyCapsule wrapping the `TSLanguage` pointer. This is the lowest-level interop function, compatible with the `tree_sitter.Language` constructor.

**Raises:**

- `LanguageNotFoundError`: If the language is not recognized

```python
from tree_sitter_language_pack import get_binding
import tree_sitter

capsule = get_binding("python")
language = tree_sitter.Language(capsule)
```

### `get_language(name: str) -> tree_sitter.Language`

Get a `tree_sitter.Language` instance for the given language name. Requires the `tree-sitter` package to be installed.

**Raises:**

- `LanguageNotFoundError`: If the language is not recognized

```python
from tree_sitter_language_pack import get_language
import tree_sitter

language = get_language("python")
parser = tree_sitter.Parser(language)
tree = parser.parse(b"x = 1")
print(tree.root_node.type)  # "module"
```

### `get_parser(name: str) -> tree_sitter.Parser`

Get a `tree_sitter.Parser` pre-configured for the given language. Requires the `tree-sitter` package to be installed.

**Raises:**

- `LanguageNotFoundError`: If the language is not recognized

```python
from tree_sitter_language_pack import get_parser

parser = get_parser("rust")
tree = parser.parse(b"fn main() {}")
print(tree.root_node.type)  # "source_file"
```

## Exceptions

### `LanguageNotFoundError`

Raised when a language name is not recognized. Inherits from `ValueError`.

```python
from tree_sitter_language_pack import get_language, LanguageNotFoundError

try:
    get_language("nonexistent")
except LanguageNotFoundError as e:
    print(f"Language not found: {e}")
```

### `ParseError`

Raised when parsing source code fails or when tree operations encounter errors. Inherits from `RuntimeError`.

```python
from tree_sitter_language_pack import parse_string, ParseError

try:
    tree = parse_string("python", "some code")
except ParseError as e:
    print(f"Parse error: {e}")
```

### `QueryError`

Raised when a tree-sitter query has invalid syntax. Inherits from `ValueError`.

```python
from tree_sitter_language_pack import parse_string, QueryError

tree = parse_string("python", "x = 1")
try:
    tree.run_query("python", "(invalid_query @bad")
except QueryError as e:
    print(f"Query error: {e}")
```

### `DownloadError`

Raised when download or cache operations fail. Inherits from `RuntimeError`.

```python
from tree_sitter_language_pack import download, DownloadError

try:
    download(["python"])
except DownloadError as e:
    print(f"Download failed: {e}")
```
