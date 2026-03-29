---
description: "Elixir API reference for tree-sitter-language-pack"
---

# Elixir API Reference

## Installation

Add to `mix.exs`:

```elixir
def deps do
  [
    {:tree_sitter_language_pack, "~> 1.2"}
  ]
end
```

Then run:

```bash
mix deps.get
```

## Quick Start

```elixir
# List available languages
languages = TreeSitterLanguagePack.available_languages()
IO.puts("#{length(languages)} languages available")

# Parse source code (returns an opaque tree reference)
tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
TreeSitterLanguagePack.tree_root_node_type(tree)
#=> "module"

TreeSitterLanguagePack.tree_contains_node_type(tree, "function_definition")
#=> true

# Extract code intelligence (config is a JSON string)
result = TreeSitterLanguagePack.process(
  "def hello(): pass",
  ~s({"language":"python"})
)
IO.inspect(result["structure"])
```

## Download Management

### `init(config_json)`

Initialize the language pack with optional pre-downloads.

**Parameters:**

- `config_json` (String): JSON string with optional fields:
    - `cache_dir` (string): Custom cache directory path
    - `languages` (list): Language names to download
    - `groups` (list): Language groups to download

**Returns:** `:ok`

**Raises:** Erlang error on invalid JSON, download failure, or network error.

This NIF runs on the DirtyIo scheduler and will not block the BEAM scheduler.

**Example:**

```elixir
TreeSitterLanguagePack.init(~s({"languages":["python","javascript","rust"]}))

TreeSitterLanguagePack.init(~s({"cache_dir":"/opt/ts-pack","languages":["python"]}))
```

### `configure(config_json)`

Apply configuration without downloading. Use to set a custom cache directory before calling `get_language_ptr/1` or any download function.

**Parameters:**

- `config_json` (String): JSON string with optional fields:
    - `cache_dir` (string): Custom cache directory path

**Returns:** `:ok`

**Raises:** Erlang error on invalid JSON or configuration failure.

**Example:**

```elixir
TreeSitterLanguagePack.configure(~s({"cache_dir":"/data/ts-pack"}))
```

### `download(names)`

Download specific languages to the local cache.

**Parameters:**

- `names` (list of String): Language names to download

**Returns:** non_neg_integer - Count of newly downloaded languages

**Raises:** Erlang error if a language is not found or download fails.

This NIF runs on the DirtyIo scheduler.

**Example:**

```elixir
count = TreeSitterLanguagePack.download(["python", "rust", "typescript"])
IO.puts("Downloaded #{count} new languages")
```

### `download_all()`

Download all available languages from the remote manifest.

**Returns:** non_neg_integer - Count of newly downloaded languages

**Raises:** Erlang error if manifest fetch fails.

This NIF runs on the DirtyIo scheduler.

**Example:**

```elixir
count = TreeSitterLanguagePack.download_all()
IO.puts("Downloaded #{count} languages")
```

### `manifest_languages()`

Get all language names available in the remote manifest.

Fetches and caches the remote manifest.

**Returns:** list of String - Sorted language names

**Raises:** Erlang error if manifest fetch fails.

This NIF runs on the DirtyIo scheduler.

**Example:**

```elixir
languages = TreeSitterLanguagePack.manifest_languages()
IO.puts("#{length(languages)} languages available for download")
```

### `downloaded_languages()`

Get languages already cached locally. Does not perform network requests.

**Returns:** list of String - Cached language names

**Example:**

```elixir
cached = TreeSitterLanguagePack.downloaded_languages()
IO.inspect(cached)
```

### `clean_cache()`

Delete all cached parser shared libraries.

**Returns:** `:ok`

**Raises:** Erlang error if cache cannot be removed.

This NIF runs on the DirtyIo scheduler.

**Example:**

```elixir
TreeSitterLanguagePack.clean_cache()
```

### `cache_dir()`

Get the effective cache directory path.

**Returns:** String

**Raises:** Erlang error if cache directory cannot be determined.

This NIF runs on the DirtyIo scheduler.

**Example:**

```elixir
dir = TreeSitterLanguagePack.cache_dir()
IO.puts("Cache at: #{dir}")
```

## Language Discovery

### `available_languages()`

List all available language names.

**Returns:** list of String - Sorted language names

**Example:**

```elixir
langs = TreeSitterLanguagePack.available_languages()
Enum.each(langs, &IO.puts/1)
```

### `has_language(name)`

Check if a language is available.

**Parameters:**

- `name` (String): Language name

**Returns:** boolean

**Example:**

```elixir
if TreeSitterLanguagePack.has_language("python") do
  IO.puts("Python available")
end
```

### `language_count()`

Get total number of available languages.

**Returns:** non_neg_integer

**Example:**

```elixir
count = TreeSitterLanguagePack.language_count()
IO.puts("#{count} languages available")
```

### `detect_language(path)`

Detect language name from a file path based on its extension.

**Parameters:**

- `path` (String): File path or extension

**Returns:** String or nil

**Example:**

```elixir
TreeSitterLanguagePack.detect_language("script.py")
#=> "python"

TreeSitterLanguagePack.detect_language("unknown.xyz")
#=> nil
```

### `detect_language_from_content(content)`

Detect language name from source code content (e.g. shebang lines).

**Parameters:**

- `content` (String): File content

**Returns:** String or nil

**Example:**

```elixir
TreeSitterLanguagePack.detect_language_from_content("#!/usr/bin/env python3\nprint('hello')")
#=> "python"
```

### `detect_language_from_extension(ext)`

Detect language name from a bare file extension (without the leading dot).

**Parameters:**

- `ext` (String): File extension without dot (e.g., `"py"`, `"js"`)

**Returns:** String or nil

**Example:**

```elixir
TreeSitterLanguagePack.detect_language_from_extension("py")
#=> "python"

TreeSitterLanguagePack.detect_language_from_extension("xyz")
#=> nil
```

### `detect_language_from_path(path)`

Detect language name from a file path based on its extension.

**Parameters:**

- `path` (String): File path

**Returns:** String or nil

**Example:**

```elixir
TreeSitterLanguagePack.detect_language_from_path("/home/user/project/main.py")
#=> "python"

TreeSitterLanguagePack.detect_language_from_path("src/app.tsx")
#=> "tsx"
```

### `extension_ambiguity(ext)`

Returns extension ambiguity information as a JSON string, or nil.

When non-nil, the JSON decodes to a map with `"assigned"` (string) and `"alternatives"` (list of strings) fields.

**Parameters:**

- `ext` (String): File extension (without dot)

**Returns:** String (JSON) or nil

**Example:**

```elixir
case TreeSitterLanguagePack.extension_ambiguity("h") do
  nil -> IO.puts("Not ambiguous")
  json -> IO.inspect(Jason.decode!(json))
end
```

### `get_language_ptr(name)`

Returns the raw `TSLanguage` pointer as a non-negative integer. Useful for interop with Elixir tree-sitter bindings that accept a language pointer.

**Parameters:**

- `name` (String): Language name

**Returns:** non_neg_integer

**Raises:** `{:error, {:language_not_found, name}}` if the language is not found.

**Example:**

```elixir
ptr = TreeSitterLanguagePack.get_language_ptr("python")
# => 140234567890
```

## Queries

### `get_highlights_query(language)`

Returns the bundled highlights query for the given language, or nil.

**Parameters:**

- `language` (String): Language name

**Returns:** String or nil

### `get_injections_query(language)`

Returns the bundled injections query for the given language, or nil.

**Parameters:**

- `language` (String): Language name

**Returns:** String or nil

### `get_locals_query(language)`

Returns the bundled locals query for the given language, or nil.

**Parameters:**

- `language` (String): Language name

**Returns:** String or nil

## Parsing

### `parse_string(language, source)`

Parse source code and return an opaque tree reference. The reference can be passed to the `tree_*` inspection functions below.

**Parameters:**

- `language` (String): Language name
- `source` (String): Source code to parse

**Returns:** reference (opaque NIF resource)

**Raises:** `{:error, {:language_not_found, name}}` or `{:error, {:parse_error, reason}}`.

**Example:**

```elixir
tree = TreeSitterLanguagePack.parse_string("python", "def foo(): pass")
TreeSitterLanguagePack.tree_root_node_type(tree)
#=> "module"
```

## Tree Inspection Functions

These functions accept the opaque tree reference returned by `parse_string/2`.

### `tree_root_node_type(tree)`

Returns the type name of the root node.

**Parameters:**

- `tree` (reference): Tree reference from `parse_string/2`

**Returns:** String

**Example:**

```elixir
tree = TreeSitterLanguagePack.parse_string("python", "x = 1")
TreeSitterLanguagePack.tree_root_node_type(tree)
#=> "module"
```

### `tree_root_child_count(tree)`

Returns the number of named children of the root node.

**Parameters:**

- `tree` (reference): Tree reference from `parse_string/2`

**Returns:** non_neg_integer

**Example:**

```elixir
tree = TreeSitterLanguagePack.parse_string("python", "x = 1\ny = 2")
TreeSitterLanguagePack.tree_root_child_count(tree)
#=> 2
```

### `tree_contains_node_type(tree, node_type)`

Checks whether any node in the tree has the given type name (depth-first traversal).

**Parameters:**

- `tree` (reference): Tree reference from `parse_string/2`
- `node_type` (String): The node type to search for

**Returns:** boolean

**Example:**

```elixir
tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
TreeSitterLanguagePack.tree_contains_node_type(tree, "function_definition")
#=> true
```

### `tree_has_error_nodes(tree)`

Checks whether the tree contains any ERROR or MISSING nodes.

**Parameters:**

- `tree` (reference): Tree reference from `parse_string/2`

**Returns:** boolean

**Example:**

```elixir
tree = TreeSitterLanguagePack.parse_string("python", "def (broken @@@ !!!")
TreeSitterLanguagePack.tree_has_error_nodes(tree)
#=> true
```

## Code Intelligence

### `process(source, config_json)`

Process source code and extract metadata as an Elixir map. The result is converted from JSON to native Elixir types (maps, lists, strings, integers, booleans, nil).

**Parameters:**

- `source` (String): Source code
- `config_json` (String): JSON string with processing configuration. Must contain at least `"language"`. Optional fields:
    - `structure` (bool, default true): Extract structural items
    - `imports` (bool, default true): Extract import statements
    - `exports` (bool, default true): Extract export statements
    - `comments` (bool, default false): Extract comments
    - `docstrings` (bool, default false): Extract docstrings
    - `symbols` (bool, default false): Extract symbol definitions
    - `diagnostics` (bool, default false): Include parse diagnostics
    - `chunk_max_size` (int or null, default null): Maximum chunk size in bytes

**Returns:** map with string keys

**Raises:** Erlang error on invalid config JSON, unknown language, or processing failure.

**Example:**

```elixir
config = Jason.encode!(%{"language" => "python", "structure" => true})
result = TreeSitterLanguagePack.process("def hello(): pass", config)

Enum.each(result["structure"], fn item ->
  IO.puts("#{item["kind"]}: #{item["name"]}")
end)
```

## Pattern Extraction

### `extract(source, config_json)`

Run tree-sitter queries against source code and return structured extraction results as an Elixir map. Unlike `process`, which uses predefined intelligence queries, `extract` lets you supply arbitrary tree-sitter query patterns.

**Parameters:**

- `source` (String): Source code to extract from
- `config_json` (String): JSON string with extraction configuration. Fields:
    - `language` (string, required): Language name
    - `patterns` (object, required): Named patterns to run. Each key maps to an object with:
        - `query` (string, required): Tree-sitter query in S-expression syntax
        - `capture_output` (string, default `"Full"`): What to capture -- `"Text"`, `"Node"`, or `"Full"`
        - `child_fields` (list of string, default `[]`): Field names to extract from child nodes
        - `max_results` (int or nil, default nil): Maximum number of matches to return
        - `byte_range` (list of two ints or nil, default nil): Restrict matches to a byte range `[start, end]`

**Returns:** map with string keys. The top-level map contains:

- `"language"` (string): The language used
- `"results"` (map): Keyed by pattern name, each value contains:
    - `"matches"` (list): Each match has `"pattern_index"` (integer) and `"captures"` (list). Each capture has `"name"` (string), `"text"` (string or nil), `"node"` (map or nil), `"child_fields"` (map), and `"start_byte"` (integer).
    - `"total_count"` (integer): Total matches before `max_results` truncation

**Raises:** Erlang error on invalid config JSON, unknown language, or extraction failure.

**Example:**

```elixir
config = Jason.encode!(%{
  "language" => "python",
  "patterns" => %{
    "functions" => %{
      "query" => "(function_definition name: (identifier) @fn_name)",
      "capture_output" => "Text"
    }
  }
})

result = TreeSitterLanguagePack.extract("def hello(): pass\ndef world(): pass", config)

for match <- result["results"]["functions"]["matches"],
    capture <- match["captures"] do
  IO.puts(capture["text"])
end
# Output:
# hello
# world
```

### `validate_extraction(config_json)`

Validate extraction patterns without running them against source code. Useful for checking query syntax before performing extraction.

**Parameters:**

- `config_json` (String): JSON string with the same shape as the config for `extract/2` (must include `language` and `patterns`)

**Returns:** map with string keys. The top-level map contains:

- `"valid"` (boolean): Whether all patterns are valid
- `"patterns"` (map): Per-pattern validation, each with:
    - `"valid"` (boolean): Whether this pattern compiled successfully
    - `"capture_names"` (list of string): Capture names defined in the query
    - `"pattern_count"` (integer): Number of patterns in the query
    - `"warnings"` (list of string): Non-fatal warnings
    - `"errors"` (list of string): Fatal errors (e.g., query syntax errors)

**Raises:** Erlang error on invalid config JSON or unknown language.

**Example:**

```elixir
config = Jason.encode!(%{
  "language" => "python",
  "patterns" => %{
    "functions" => %{
      "query" => "(function_definition name: (identifier) @fn_name)"
    }
  }
})

result = TreeSitterLanguagePack.validate_extraction(config)

if result["valid"] do
  IO.puts("All patterns valid")
else
  for {name, info} <- result["patterns"], not info["valid"] do
    Enum.each(info["errors"], &IO.puts("#{name}: #{&1}"))
  end
end
```

## Error Handling

NIF functions raise Erlang errors with tagged tuples. Use `try`/`rescue` or pattern matching:

```elixir
try do
  TreeSitterLanguagePack.get_language_ptr("nonexistent")
rescue
  ErlangError -> IO.puts("Language not found")
end
```

For functions that return tagged tuples on error:

```elixir
case TreeSitterLanguagePack.get_language_ptr("nonexistent") do
  {:error, {:language_not_found, name}} ->
    IO.puts("Language not found: #{name}")

  ptr when is_integer(ptr) ->
    IO.puts("Got pointer: #{ptr}")
end
```

## Usage Patterns

### Pre-download in Application Start

```elixir
# lib/my_app/application.ex
defmodule MyApp.Application do
  use Application

  @impl true
  def start(_type, _args) do
    TreeSitterLanguagePack.init(
      ~s({"languages":["python","rust","typescript","javascript"]})
    )

    children = []
    opts = [strategy: :one_for_one, name: MyApp.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
```

### Batch Processing with Tasks

```elixir
files = Path.wildcard("src/**/*.py")
config = ~s({"language":"python"})

files
|> Task.async_stream(fn file ->
  source = File.read!(file)
  result = TreeSitterLanguagePack.process(source, config)
  {file, length(result["structure"])}
end)
|> Enum.each(fn {:ok, {file, count}} ->
  IO.puts("#{file}: #{count} items")
end)
```

## Type Specifications

The module defines `@spec` annotations for all public functions. Use Dialyzer for static analysis:

```elixir
@spec analyze(String.t(), String.t()) :: map()
def analyze(source, language) do
  config = Jason.encode!(%{"language" => language})
  TreeSitterLanguagePack.process(source, config)
end
```
