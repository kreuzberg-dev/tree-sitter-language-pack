---
description: "PHP API reference for tree-sitter-language-pack"
---

# PHP API Reference

## Installation

Install via Composer:

```bash
composer require kreuzberg/tree-sitter-language-pack
```

The package requires the `tree_sitter_language_pack` PHP extension (a native Rust extension built with ext-php-rs). The Composer package provides a PHP wrapper class around the procedural extension functions.

## Quick Start

```php
<?php
declare(strict_types=1);

require_once 'vendor/autoload.php';

use TreeSitterLanguagePack\TreeSitterLanguagePack;
use TreeSitterLanguagePack\ProcessConfig;

// List available languages
$langs = TreeSitterLanguagePack::availableLanguages();
echo count($langs) . " languages available\n";

// Parse source code (returns S-expression string)
$sexp = TreeSitterLanguagePack::parseString("python", "def hello(): pass");
echo "Tree: $sexp\n";

// Extract code intelligence
$config = new ProcessConfig("python");
$result = TreeSitterLanguagePack::process("def hello(): pass", $config);
echo count($result['structure']) . " structure items\n";
```

## Architecture

The PHP binding has two layers:

1. **Extension functions** (`ts_pack_*`): Procedural functions exposed directly by the Rust native extension via ext-php-rs.
2. **PHP wrapper class** (`TreeSitterLanguagePack\TreeSitterLanguagePack`): A thin OOP wrapper in `packages/php/src/` that calls the extension functions.

You can use either layer. The wrapper class is the recommended interface.

## Wrapper Class API

### `TreeSitterLanguagePack::version(): string`

Get the library version.

**Returns:** string - Version in semver format (e.g., "1.2.0")

**Example:**

```php
echo TreeSitterLanguagePack::version();
```

### `TreeSitterLanguagePack::availableLanguages(): array`

List all available language names.

**Returns:** string[] - Sorted language names

**Example:**

```php
$langs = TreeSitterLanguagePack::availableLanguages();
foreach ($langs as $lang) {
    echo "$lang\n";
}
```

### `TreeSitterLanguagePack::hasLanguage(string $name): bool`

Check if a language is available.

**Parameters:**

- `$name` (string): Language name

**Returns:** bool

**Example:**

```php
if (TreeSitterLanguagePack::hasLanguage('python')) {
    echo "Python available\n";
}
```

### `TreeSitterLanguagePack::languageCount(): int`

Get total number of available languages.

**Returns:** int

**Example:**

```php
echo TreeSitterLanguagePack::languageCount() . " languages\n";
```

### `TreeSitterLanguagePack::getLanguage(string $name): int`

Get the raw `TSLanguage` pointer as an integer handle. Useful for interop with PHP tree-sitter bindings that accept a language pointer.

**Parameters:**

- `$name` (string): Language name

**Returns:** int - Raw language pointer

**Throws:** `\Exception` if the language is not available.

**Example:**

```php
$ptr = TreeSitterLanguagePack::getLanguage('python');
echo "Language pointer: $ptr\n";
```

### `TreeSitterLanguagePack::parseString(string $language, string $source): string`

Parse source code and return an S-expression representation of the syntax tree.

**Parameters:**

- `$language` (string): Language name
- `$source` (string): Source code to parse

**Returns:** string - S-expression of the parsed tree

**Throws:** `\Exception` if the language is not available or parsing fails.

**Example:**

```php
$sexp = TreeSitterLanguagePack::parseString('python', 'def foo(): pass');
echo "Tree: $sexp\n";
```

### `TreeSitterLanguagePack::process(string $source, ProcessConfig|array $config): array`

Process source code and extract metadata and chunks.

**Parameters:**

- `$source` (string): Source code
- `$config` (ProcessConfig or array): Configuration. Must contain at least `language`. Can be a `ProcessConfig` object or an associative array.

**Returns:** array - Extraction results with string keys

**Throws:** `\RuntimeException` on invalid config, unknown language, or processing failure.

**Example:**

```php
$config = new ProcessConfig('python', structure: true, comments: true);
$result = TreeSitterLanguagePack::process('def hello(): pass', $config);

foreach ($result['structure'] as $item) {
    echo "{$item['kind']}: {$item['name']}\n";
}
```

## ProcessConfig

Configuration for source code processing. Uses PHP 8.2 readonly constructor promotion.

**Constructor:**

```php
new ProcessConfig(
    string $language,        // Required: language name
    bool $structure = true,  // Extract structural items (functions, classes, etc.)
    bool $imports = true,    // Extract import statements
    bool $exports = true,    // Extract export statements
    bool $comments = false,  // Extract comments
    bool $docstrings = false, // Extract docstrings
    bool $symbols = false,   // Extract symbol definitions
    bool $diagnostics = false, // Include parse diagnostics
    ?int $chunkMaxSize = null, // Maximum chunk size in bytes (null disables chunking)
)
```

All properties are `public readonly`.

**Methods:**

- `toArray(): array` - Convert to associative array for JSON encoding

**Examples:**

```php
// Defaults: structure + imports + exports enabled
$config = new ProcessConfig('python');

// Enable everything
$config = new ProcessConfig(
    'python',
    structure: true,
    imports: true,
    exports: true,
    comments: true,
    docstrings: true,
    symbols: true,
    diagnostics: true,
    chunkMaxSize: 2000,
);

// Using an array directly
$result = TreeSitterLanguagePack::process($source, [
    'language' => 'python',
    'structure' => true,
    'comments' => true,
]);
```

## Pattern Extraction

### `TreeSitterLanguagePack::extract(string $source, string $configJson): array`

Run tree-sitter queries against source code and return structured extraction results. Unlike `process`, which uses predefined intelligence queries, `extract` lets you supply arbitrary tree-sitter query patterns.

The wrapper class calls `ts_pack_extract` internally and JSON-decodes the result into an associative array.

**Parameters:**

- `$source` (string): Source code to extract from
- `$configJson` (string): JSON string with extraction configuration. Fields:
  - `language` (string, required): Language name
  - `patterns` (object, required): Named patterns to run. Each key maps to an object with:
    - `query` (string, required): tree-sitter query in S-expression syntax
    - `capture_output` (string, default `"Full"`): What to capture -- `"Text"`, `"Node"`, or `"Full"`
    - `child_fields` (string[], default `[]`): Field names to extract from child nodes
    - `max_results` (int|null, default null): Maximum number of matches to return
    - `byte_range` ([int, int]|null, default null): Restrict matches to a byte range

**Returns:** array - Associative array with extraction results. The top-level array contains:

- `language` (string): The language used
- `results` (array): Keyed by pattern name, each value contains:
  - `matches` (array): Each match has `pattern_index` (int) and `captures` (array). Each capture has `name` (string), `text` (string|null), `node` (array|null), `child_fields` (array), and `start_byte` (int).
  - `total_count` (int): Total matches before `max_results` truncation

**Throws:** `\RuntimeException` on invalid config JSON, unknown language, or extraction failure.

**Example:**

```php
$config = json_encode([
    'language' => 'python',
    'patterns' => [
        'functions' => [
            'query' => '(function_definition name: (identifier) @fn_name)',
            'capture_output' => 'Text',
        ],
    ],
]);

$result = TreeSitterLanguagePack::extract('def hello(): pass', $config);

foreach ($result['results']['functions']['matches'] as $match) {
    foreach ($match['captures'] as $capture) {
        echo $capture['text'] . "\n";
    }
}
```

### `TreeSitterLanguagePack::validateExtraction(string $configJson): array`

Validate extraction patterns without running them against source code. Useful for checking query syntax before performing extraction.

**Parameters:**

- `$configJson` (string): JSON string with the same shape as the config for `extract` (must include `language` and `patterns`)

**Returns:** array - Associative array with validation results. The top-level array contains:

- `valid` (bool): Whether all patterns are valid
- `patterns` (array): Per-pattern validation, each with:
  - `valid` (bool): Whether this pattern compiled successfully
  - `capture_names` (string[]): Capture names defined in the query
  - `pattern_count` (int): Number of patterns in the query
  - `warnings` (string[]): Non-fatal warnings
  - `errors` (string[]): Fatal errors (e.g., query syntax errors)

**Throws:** `\RuntimeException` on invalid config JSON or unknown language.

**Example:**

```php
$config = json_encode([
    'language' => 'python',
    'patterns' => [
        'functions' => [
            'query' => '(function_definition name: (identifier) @fn_name)',
        ],
    ],
]);

$result = TreeSitterLanguagePack::validateExtraction($config);

if ($result['valid']) {
    echo "All patterns valid\n";
} else {
    foreach ($result['patterns'] as $name => $info) {
        if (!$info['valid']) {
            foreach ($info['errors'] as $error) {
                echo "$name: $error\n";
            }
        }
    }
}
```

## Extension Functions (Procedural API)

These are the raw functions exposed by the native extension. The wrapper class calls these internally.

### Registry and Parsing

| Function | Parameters | Returns |
|----------|-----------|---------|
| `ts_pack_version()` | none | string |
| `ts_pack_available_languages()` | none | string[] |
| `ts_pack_has_language(string $name)` | language name | bool |
| `ts_pack_language_count()` | none | int |
| `ts_pack_detect_language(string $path)` | file path | string or null |
| `ts_pack_detect_language_from_content(string $content)` | file content | string or null |
| `ts_pack_detect_language_from_extension(string $ext)` | bare extension (no dot) | string or null |
| `ts_pack_detect_language_from_path(string $path)` | file path | string or null |
| `ts_pack_extension_ambiguity(string $ext)` | extension | string (JSON) or null |
| `ts_pack_get_highlights_query(string $language)` | language name | string or null |
| `ts_pack_get_injections_query(string $language)` | language name | string or null |
| `ts_pack_get_locals_query(string $language)` | language name | string or null |
| `ts_pack_get_language(string $name)` | language name | int (pointer) |
| `ts_pack_parse_string(string $language, string $source)` | language, source | string (S-expression) |
| `ts_pack_process(string $source, string $config_json)` | source, JSON config | string (JSON result) |
| `ts_pack_extract(string $source, string $config_json)` | source, JSON config | string (JSON result) |
| `ts_pack_validate_extraction(string $config_json)` | JSON config | string (JSON result) |

### Download Management

| Function | Parameters | Returns |
|----------|-----------|---------|
| `ts_pack_init(string $config_json)` | JSON config | void |
| `ts_pack_configure(string $config_json)` | JSON config | void |
| `ts_pack_download(string[] $names)` | language names | int |
| `ts_pack_download_all()` | none | int |
| `ts_pack_manifest_languages()` | none | string[] |
| `ts_pack_downloaded_languages()` | none | string[] |
| `ts_pack_clean_cache()` | none | void |
| `ts_pack_cache_dir()` | none | string |

### Extension Function Examples

```php
// Using procedural functions directly
$langs = ts_pack_available_languages();
$sexp = ts_pack_parse_string('python', 'x = 1');
$result_json = ts_pack_process('def hello(): pass', '{"language":"python"}');
$result = json_decode($result_json, true);

// Download management
ts_pack_init('{"languages":["python","rust"]}');
ts_pack_configure('{"cache_dir":"/tmp/parsers"}');
$count = ts_pack_download(['python', 'rust']);
$cached = ts_pack_downloaded_languages();
$dir = ts_pack_cache_dir();
ts_pack_clean_cache();
```

### Detection Functions

```php
$lang = ts_pack_detect_language('script.py');     // "python"
$lang = ts_pack_detect_language_from_content("#!/usr/bin/env python3\n");  // "python"
$lang = ts_pack_detect_language_from_extension('py');  // "python"
$lang = ts_pack_detect_language_from_path('/home/user/project/main.py');  // "python"

$info = ts_pack_extension_ambiguity('h');
if ($info !== null) {
    $data = json_decode($info, true);
    echo "Assigned: " . $data['assigned'] . "\n";
}
```

## Error Handling

The native extension throws `\Exception` on errors. The wrapper class may throw `\RuntimeException` for JSON encoding/decoding failures.

```php
try {
    $sexp = TreeSitterLanguagePack::parseString('nonexistent', 'code');
} catch (\Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
```

## Usage Patterns

### Pre-download Languages

```php
// bootstrap.php
ts_pack_init('{"languages":["python","rust","typescript","javascript"]}');
```

### Custom Cache Directory

```php
ts_pack_configure('{"cache_dir":"/data/ts-pack-cache"}');
```

### Batch Processing

```php
use TreeSitterLanguagePack\TreeSitterLanguagePack;
use TreeSitterLanguagePack\ProcessConfig;

$config = new ProcessConfig('python');

foreach (glob('src/**/*.py') as $file) {
    try {
        $source = file_get_contents($file);
        $result = TreeSitterLanguagePack::process($source, $config);
        echo "$file: " . count($result['structure']) . " items\n";
    } catch (\Exception $e) {
        echo "Error: {$e->getMessage()}\n";
    }
}
```
