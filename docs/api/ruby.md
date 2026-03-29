---
description: "Ruby API reference for tree-sitter-language-pack"
---

# Ruby API Reference

## Installation

Add to `Gemfile`:

```ruby
gem "tree_sitter_language_pack"
```

Then run:

```bash
bundle install
```

Or install directly:

```bash
gem install tree_sitter_language_pack
```

## Quick Start

```ruby
require "tree_sitter_language_pack"

# List available languages
langs = TreeSitterLanguagePack.available_languages
puts "#{langs.length} languages available"

# Parse source code
tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
puts tree.root_node_type         # "module"
puts tree.has_error_nodes        # false
puts tree.contains_node_type("function_definition") # true

# Extract code intelligence (config is a JSON string)
result_json = TreeSitterLanguagePack.process(
  "def hello(): pass",
  '{"language":"python"}'
)
result = JSON.parse(result_json)
puts "Structure items: #{result['structure'].length}"
```

## Download Management

### `TreeSitterLanguagePack.init(config_json)`

Initialize the language pack with optional pre-downloads.

**Parameters:**

- `config_json` (String): JSON string with optional fields:
    - `cache_dir` (string): Custom cache directory
    - `languages` (array): Language names to download
    - `groups` (array): Language groups to download

**Returns:** nil

**Raises:** `RuntimeError` on invalid JSON, download failure, or network error.

**Example:**

```ruby
TreeSitterLanguagePack.init('{"languages":["python","javascript","rust"]}')

TreeSitterLanguagePack.init('{"cache_dir":"/opt/ts-pack","languages":["python"]}')
```

### `TreeSitterLanguagePack.configure(config_json)`

Apply configuration without downloading. Use to set a custom cache directory before calling `get_language_ptr` or any download function.

**Parameters:**

- `config_json` (String): JSON string with optional fields:
    - `cache_dir` (string): Custom cache directory

**Returns:** nil

**Raises:** `RuntimeError` on invalid JSON or configuration error.

**Example:**

```ruby
TreeSitterLanguagePack.configure('{"cache_dir":"/data/ts-pack"}')
```

### `TreeSitterLanguagePack.download(names)`

Download specific languages to the cache.

**Parameters:**

- `names` (Array<String>): Language names to download

**Returns:** Integer - Count of newly downloaded languages

**Raises:** `RuntimeError` if a language is not found or download fails.

**Example:**

```ruby
count = TreeSitterLanguagePack.download(["python", "rust", "typescript"])
puts "Downloaded #{count} new languages"
```

### `TreeSitterLanguagePack.download_all`

Download all available languages from the remote manifest.

**Returns:** Integer - Count of newly downloaded languages

**Raises:** `RuntimeError` if manifest fetch fails.

**Example:**

```ruby
count = TreeSitterLanguagePack.download_all
puts "Downloaded #{count} languages"
```

### `TreeSitterLanguagePack.manifest_languages`

Get all available language names from the remote manifest.

Fetches and caches the manifest.

**Returns:** Array<String> - Sorted language names

**Raises:** `RuntimeError` if manifest fetch fails.

**Example:**

```ruby
languages = TreeSitterLanguagePack.manifest_languages
puts "Available: #{languages.length} languages"
```

### `TreeSitterLanguagePack.downloaded_languages`

Get languages already cached locally. Does not perform network requests.

**Returns:** Array<String> - Cached language names

**Example:**

```ruby
cached = TreeSitterLanguagePack.downloaded_languages
cached.each { |lang| puts lang }
```

### `TreeSitterLanguagePack.clean_cache`

Delete all cached parser shared libraries.

**Returns:** nil

**Raises:** `RuntimeError` if cache cannot be removed.

**Example:**

```ruby
TreeSitterLanguagePack.clean_cache
```

### `TreeSitterLanguagePack.cache_dir`

Get the effective cache directory path.

**Returns:** String - Absolute cache directory path

**Raises:** `RuntimeError` if cache path cannot be determined or is not valid UTF-8.

**Example:**

```ruby
dir = TreeSitterLanguagePack.cache_dir
puts "Cache at: #{dir}"
```

## Language Discovery

### `TreeSitterLanguagePack.available_languages`

List all available language names.

**Returns:** Array<String> - Sorted language names

**Example:**

```ruby
langs = TreeSitterLanguagePack.available_languages
langs.each { |lang| puts lang }
```

### `TreeSitterLanguagePack.has_language(name)`

Check if a language is available.

**Parameters:**

- `name` (String): Language name

**Returns:** Boolean

**Example:**

```ruby
if TreeSitterLanguagePack.has_language("python")
  puts "Python available"
end
```

### `TreeSitterLanguagePack.language_count`

Get total number of available languages.

**Returns:** Integer

**Example:**

```ruby
count = TreeSitterLanguagePack.language_count
puts "#{count} languages available"
```

### `TreeSitterLanguagePack.detect_language(path)`

Detect language name from a file path or extension.

**Parameters:**

- `path` (String): File path or extension

**Returns:** String or nil

**Example:**

```ruby
lang = TreeSitterLanguagePack.detect_language("script.py")
puts lang # "python"
```

### `TreeSitterLanguagePack.detect_language_from_content(content)`

Detect language name from file content using shebang-based detection.

**Parameters:**

- `content` (String): File content

**Returns:** String or nil

**Example:**

```ruby
lang = TreeSitterLanguagePack.detect_language_from_content("#!/usr/bin/env python3\nprint('hello')")
puts lang # "python"
```

### `TreeSitterLanguagePack.detect_language_from_extension(ext)`

Detect language name from a bare file extension (without the leading dot).

**Parameters:**

- `ext` (String): File extension without dot (e.g., `"py"`, `"js"`)

**Returns:** String or nil

**Example:**

```ruby
lang = TreeSitterLanguagePack.detect_language_from_extension("py")
puts lang # "python"

lang = TreeSitterLanguagePack.detect_language_from_extension("xyz")
puts lang # nil
```

### `TreeSitterLanguagePack.detect_language_from_path(path)`

Detect language name from a file path based on its extension.

**Parameters:**

- `path` (String): File path

**Returns:** String or nil

**Example:**

```ruby
lang = TreeSitterLanguagePack.detect_language_from_path("/home/user/project/main.py")
puts lang # "python"

lang = TreeSitterLanguagePack.detect_language_from_path("src/app.tsx")
puts lang # "tsx"
```

### `TreeSitterLanguagePack.extension_ambiguity(ext)`

Returns extension ambiguity information as a JSON string, or nil if the extension is unambiguous.

**Parameters:**

- `ext` (String): File extension (without dot)

**Returns:** String (JSON) or nil. When non-nil, decodes to an object with `"assigned"` and `"alternatives"` fields.

**Example:**

```ruby
info = TreeSitterLanguagePack.extension_ambiguity("h")
if info
  data = JSON.parse(info)
  puts "Assigned: #{data['assigned']}"
  puts "Alternatives: #{data['alternatives'].join(', ')}"
end
```

### `TreeSitterLanguagePack.get_language_ptr(name)`

Get the raw `TSLanguage` pointer as an integer handle. Useful for interop with tree-sitter Ruby bindings that accept a language pointer.

**Parameters:**

- `name` (String): Language name

**Returns:** Integer - Raw language pointer as u64

**Raises:** `RuntimeError` if the language is not found.

**Example:**

```ruby
ptr = TreeSitterLanguagePack.get_language_ptr("python")
puts "Language pointer: #{ptr}"
```

## Queries

### `TreeSitterLanguagePack.get_highlights_query(language)`

Returns the bundled highlights query for the given language, or nil.

**Parameters:**

- `language` (String): Language name

**Returns:** String or nil

### `TreeSitterLanguagePack.get_injections_query(language)`

Returns the bundled injections query for the given language, or nil.

**Parameters:**

- `language` (String): Language name

**Returns:** String or nil

### `TreeSitterLanguagePack.get_locals_query(language)`

Returns the bundled locals query for the given language, or nil.

**Parameters:**

- `language` (String): Language name

**Returns:** String or nil

## Parsing

### `TreeSitterLanguagePack.parse_string(language, source)`

Parse source code and return a `TreeSitterLanguagePack::Tree` object.

**Parameters:**

- `language` (String): Language name
- `source` (String): Source code to parse

**Returns:** `TreeSitterLanguagePack::Tree`

**Raises:** `RuntimeError` if the language is not found or parsing fails.

**Example:**

```ruby
tree = TreeSitterLanguagePack.parse_string("python", "def foo(): pass")
puts tree.root_node_type        # "module"
puts tree.root_child_count      # 1
puts tree.has_error_nodes        # false
puts tree.contains_node_type("function_definition") # true
```

## Tree Class

`TreeSitterLanguagePack::Tree` is returned by `parse_string`. It wraps an opaque tree-sitter tree and provides the following instance methods.

### `#root_node_type`

Returns the type name of the root node as a String.

**Example:**

```ruby
tree = TreeSitterLanguagePack.parse_string("python", "x = 1")
tree.root_node_type # "module"
```

### `#root_child_count`

Returns the number of named children of the root node as an Integer.

**Example:**

```ruby
tree = TreeSitterLanguagePack.parse_string("python", "x = 1\ny = 2")
tree.root_child_count # 2
```

### `#contains_node_type(node_type)`

Checks whether any node in the tree has the given type name.

**Parameters:**

- `node_type` (String): The node type to search for

**Returns:** Boolean

**Example:**

```ruby
tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
tree.contains_node_type("function_definition") # true
tree.contains_node_type("class_definition")    # false
```

### `#has_error_nodes`

Checks whether the tree contains any ERROR or MISSING nodes.

**Returns:** Boolean

**Example:**

```ruby
tree = TreeSitterLanguagePack.parse_string("python", "def (broken @@@ !!!")
tree.has_error_nodes # true
```

## Code Intelligence

### `TreeSitterLanguagePack.process(source, config_json)`

Process source code and extract metadata as a JSON string.

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

**Returns:** String - JSON string with extraction results

**Raises:** `RuntimeError` on invalid config JSON, unknown language, or processing failure.

**Example:**

```ruby
require "json"

config = { language: "python", structure: true, imports: true }.to_json
result_json = TreeSitterLanguagePack.process("def hello(): pass", config)
result = JSON.parse(result_json)

result["structure"].each do |item|
  puts "#{item['kind']}: #{item['name']}"
end
```

## Pattern Extraction

### `TreeSitterLanguagePack.extract(source, config_json)`

Run tree-sitter queries against source code and return structured extraction results as a JSON string. Unlike `process`, which uses predefined intelligence queries, `extract` lets you supply arbitrary tree-sitter query patterns.

**Parameters:**

- `source` (String): Source code to extract from
- `config_json` (String): JSON string with extraction configuration. Fields:
    - `language` (string, required): Language name
    - `patterns` (object, required): Named patterns to run. Each key maps to an object with:
        - `query` (string, required): Tree-sitter query in S-expression syntax
        - `capture_output` (string, default `"Full"`): What to capture -- `"Text"`, `"Node"`, or `"Full"`
        - `child_fields` (array of string, default `[]`): Field names to extract from child nodes
        - `max_results` (int or null, default null): Maximum number of matches to return
        - `byte_range` (array of two ints or null, default null): Restrict matches to a byte range `[start, end]`

**Returns:** String - JSON string with extraction results. The top-level object contains:

- `language` (string): The language used
- `results` (object): Keyed by pattern name, each value contains:
    - `matches` (array): Each match has `pattern_index` (int) and `captures` (array). Each capture has `name` (string), `text` (string or null), `node` (object or null), `child_fields` (object), and `start_byte` (int).
    - `total_count` (int): Total matches before `max_results` truncation

**Raises:** `RuntimeError` on invalid config JSON, unknown language, or extraction failure.

**Example:**

```ruby
require "json"

config = {
  language: "python",
  patterns: {
    functions: {
      query: "(function_definition name: (identifier) @fn_name)",
      capture_output: "Text"
    }
  }
}.to_json

result_json = TreeSitterLanguagePack.extract("def hello(): pass\ndef world(): pass", config)
result = JSON.parse(result_json)

result["results"]["functions"]["matches"].each do |m|
  m["captures"].each { |c| puts c["text"] }
end
# Output:
# hello
# world
```

### `TreeSitterLanguagePack.validate_extraction(config_json)`

Validate extraction patterns without running them against source code. Useful for checking query syntax before performing extraction.

**Parameters:**

- `config_json` (String): JSON string with the same shape as the config for `extract` (must include `language` and `patterns`)

**Returns:** String - JSON string with validation results. The top-level object contains:

- `valid` (bool): Whether all patterns are valid
- `patterns` (object): Per-pattern validation, each with:
    - `valid` (bool): Whether this pattern compiled successfully
    - `capture_names` (array of string): Capture names defined in the query
    - `pattern_count` (int): Number of patterns in the query
    - `warnings` (array of string): Non-fatal warnings
    - `errors` (array of string): Fatal errors (e.g., query syntax errors)

**Raises:** `RuntimeError` on invalid config JSON or unknown language.

**Example:**

```ruby
require "json"

config = {
  language: "python",
  patterns: {
    functions: {
      query: "(function_definition name: (identifier) @fn_name)"
    }
  }
}.to_json

result_json = TreeSitterLanguagePack.validate_extraction(config)
result = JSON.parse(result_json)

if result["valid"]
  puts "All patterns valid"
else
  result["patterns"].each do |name, info|
    info["errors"].each { |err| puts "#{name}: #{err}" } unless info["valid"]
  end
end
```

## Error Handling

All errors from the native extension are raised as `RuntimeError`. There are no custom exception classes.

```ruby
require "tree_sitter_language_pack"

begin
  TreeSitterLanguagePack.parse_string("nonexistent_language", "code")
rescue RuntimeError => e
  puts "Error: #{e.message}"
end
```

## Usage Patterns

### Pre-download Languages at Startup

```ruby
# config/initializers/tree_sitter.rb
require "tree_sitter_language_pack"
TreeSitterLanguagePack.init('{"languages":["python","rust","typescript","javascript"]}')
```

### Custom Cache Directory

```ruby
TreeSitterLanguagePack.configure('{"cache_dir":"/data/ts-pack-cache"}')
```

### Batch Processing

```ruby
require "json"

def analyze_files(dir, language)
  config = { language: language }.to_json

  Dir.glob("#{dir}/**/*.py").each do |file|
    source = File.read(file)
    result = JSON.parse(TreeSitterLanguagePack.process(source, config))
    puts "#{file}: #{result['structure'].length} items"
  rescue RuntimeError => e
    puts "Error processing #{file}: #{e.message}"
  end
end
```
