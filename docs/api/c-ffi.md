---
description: "C FFI API reference for tree-sitter-language-pack"
---

# C / FFI API Reference

## Overview

The C FFI layer provides a stable C API for tree-sitter-language-pack, enabling integration with languages like Go (cgo), Java (Panama FFM), and C# (P/Invoke).

The header file is located at `crates/ts-pack-ffi/include/ts_pack.h`.

## Installation

### C Header

```c
#include "ts_pack.h"
```

Link against the compiled FFI library:

```bash
gcc -o program program.c -L. -lts_pack_ffi
```

## Opaque Handles

The API uses two opaque pointer types. Callers must never dereference or inspect the internals of these handles.

- `TsPackRegistry*` -- language registry created with `ts_pack_registry_new`, freed with `ts_pack_registry_free`.
- `TsPackTree*` -- parsed syntax tree created with `ts_pack_parse_string`, freed with `ts_pack_tree_free`.

## Error Handling

Errors are reported through a thread-local string. After any function returns a failure indicator (null pointer, 0 count, false, or -1), call `ts_pack_last_error()` to retrieve a human-readable message.

### `const char* ts_pack_last_error(void)`

Get the last error message for the current thread, or null if no error occurred.

**Returns:** `const char*` -- error message string, valid until the next FFI call on the same thread. Do NOT free this pointer.

### `void ts_pack_clear_error(void)`

Clear the thread-local error state.

**Example:**

```c
TsPackRegistry* reg = ts_pack_registry_new();
if (!reg) {
    const char* err = ts_pack_last_error();
    fprintf(stderr, "Error: %s\n", err ? err : "unknown");
    return 1;
}
```

## Registry

### `TsPackRegistry* ts_pack_registry_new(void)`

Create a new language registry containing all compiled-in grammars.

**Returns:** `TsPackRegistry*` -- opaque handle, or null on failure.

**Note:** The caller must free the registry with `ts_pack_registry_free`.

### `void ts_pack_registry_free(TsPackRegistry* registry)`

Free a registry previously created with `ts_pack_registry_new`. Passing null is a safe no-op.

**Parameters:**

- `registry` (`TsPackRegistry*`): registry to free, or null.

**Example:**

```c
TsPackRegistry* reg = ts_pack_registry_new();
// ... use registry ...
ts_pack_registry_free(reg);
```

## Language Discovery

### `size_t ts_pack_language_count(const TsPackRegistry* registry)`

Get the number of available languages in the registry.

**Parameters:**

- `registry` (`const TsPackRegistry*`): registry handle.

**Returns:** `size_t` -- language count, or 0 if registry is null.

### `const char* ts_pack_language_name_at(const TsPackRegistry* registry, size_t index)`

Get the language name at the given index.

**Parameters:**

- `registry` (`const TsPackRegistry*`): registry handle.
- `index` (`size_t`): zero-based index into the sorted language list.

**Returns:** `const char*` -- newly-allocated C string, or null if out of bounds. The caller must free the returned string with `ts_pack_free_string`.

**Example:**

```c
TsPackRegistry* reg = ts_pack_registry_new();
size_t count = ts_pack_language_count(reg);
for (size_t i = 0; i < count; i++) {
    char* name = (char*)ts_pack_language_name_at(reg, i);
    if (name) {
        printf("%s\n", name);
        ts_pack_free_string(name);
    }
}
ts_pack_registry_free(reg);
```

### `bool ts_pack_has_language(const TsPackRegistry* registry, const char* name)`

Check whether the registry contains a language with the given name.

**Parameters:**

- `registry` (`const TsPackRegistry*`): registry handle.
- `name` (`const char*`): null-terminated language name.

**Returns:** `bool` -- true if available, false otherwise (or if either pointer is null).

**Example:**

```c
if (ts_pack_has_language(reg, "python")) {
    printf("Python is available\n");
}
```

## Language Detection

### `char* ts_pack_detect_language(const char* path)`

Detect language name from a file path based on its extension.

**Parameters:**

- `path` (`const char*`): null-terminated file path.

**Returns:** `char*` -- newly-allocated language name string, or null if the extension is not recognized. The caller must free the returned string with `ts_pack_free_string`.

### `char* ts_pack_detect_language_from_extension(const char* ext)`

Detect language name from a bare file extension (without the leading dot).

**Parameters:**

- `ext` (`const char*`): null-terminated file extension (e.g., `"rs"`, `"py"`).

**Returns:** `char*` -- newly-allocated language name string, or null if the extension is not recognized. The caller must free the returned string with `ts_pack_free_string`.

**Example:**

```c
char* lang = ts_pack_detect_language_from_extension("rs");
if (lang) {
    printf("Language: %s\n", lang); // "rust"
    ts_pack_free_string(lang);
}
```

### `char* ts_pack_detect_language_from_path(const char* path)`

Detect language name from a file path.

**Parameters:**

- `path` (`const char*`): null-terminated file path.

**Returns:** `char*` -- newly-allocated language name string, or null if the path's extension is not recognized. The caller must free the returned string with `ts_pack_free_string`.

**Example:**

```c
char* lang = ts_pack_detect_language_from_path("/home/user/project/main.py");
if (lang) {
    printf("Language: %s\n", lang); // "python"
    ts_pack_free_string(lang);
}
```

### `char* ts_pack_detect_language_from_content(const char* content)`

Detect language name from file content using shebang-based detection.

**Parameters:**

- `content` (`const char*`): null-terminated file content.

**Returns:** `char*` -- newly-allocated language name string, or null if no shebang is recognized. The caller must free the returned string with `ts_pack_free_string`.

### `char* ts_pack_extension_ambiguity(const char* ext)`

Get extension ambiguity information as a JSON string.

**Parameters:**

- `ext` (`const char*`): null-terminated file extension (without dot).

**Returns:** `char*` -- newly-allocated JSON string with `"assigned"` (string) and `"alternatives"` (string array) fields, or null if the extension is not ambiguous. The caller must free the returned string with `ts_pack_free_string`.

## Language Pointers

### `const TSLanguage* ts_pack_get_language(const TsPackRegistry* registry, const char* name)`

Get a raw tree-sitter `TSLanguage` pointer for the given language name.

**Parameters:**

- `registry` (`const TsPackRegistry*`): registry handle.
- `name` (`const char*`): null-terminated language name.

**Returns:** `const TSLanguage*` -- language pointer valid for the lifetime of the registry, or null on error. Check `ts_pack_last_error()` on null.

**Example:**

```c
const TSLanguage* lang = ts_pack_get_language(reg, "python");
if (!lang) {
    fprintf(stderr, "Error: %s\n", ts_pack_last_error());
    return;
}
// Use with tree-sitter's ts_parser_set_language()
```

## Queries

### `char* ts_pack_get_highlights_query(const char* language)`

Get the bundled highlights query for a language.

**Parameters:**

- `language` (`const char*`): null-terminated language name.

**Returns:** `char*` -- newly-allocated query string, or null if no highlights query is bundled. Free with `ts_pack_free_string`.

### `char* ts_pack_get_injections_query(const char* language)`

Get the bundled injections query for a language.

**Parameters:**

- `language` (`const char*`): null-terminated language name.

**Returns:** `char*` -- newly-allocated query string, or null if unavailable. Free with `ts_pack_free_string`.

### `char* ts_pack_get_locals_query(const char* language)`

Get the bundled locals query for a language.

**Parameters:**

- `language` (`const char*`): null-terminated language name.

**Returns:** `char*` -- newly-allocated query string, or null if unavailable. Free with `ts_pack_free_string`.

## Parsing

### `TsPackTree* ts_pack_parse_string(const TsPackRegistry* registry, const char* name, const char* source, size_t source_len)`

Parse source code into an opaque syntax tree.

**Parameters:**

- `registry` (`const TsPackRegistry*`): registry handle.
- `name` (`const char*`): null-terminated language name.
- `source` (`const char*`): source code buffer (does not need to be null-terminated).
- `source_len` (`size_t`): length of the source buffer in bytes.

**Returns:** `TsPackTree*` -- opaque tree handle, or null on error. Check `ts_pack_last_error()` on null. The caller must free the tree with `ts_pack_tree_free`.

**Example:**

```c
const char* code = "def hello(): pass";
TsPackTree* tree = ts_pack_parse_string(reg, "python", code, strlen(code));
if (!tree) {
    fprintf(stderr, "Parse error: %s\n", ts_pack_last_error());
    return;
}
// ... use tree ...
ts_pack_tree_free(tree);
```

### `void ts_pack_tree_free(TsPackTree* tree)`

Free a tree previously created with `ts_pack_parse_string`. Passing null is a safe no-op.

## Tree Inspection

### `char* ts_pack_tree_root_node_type(const TsPackTree* tree)`

Get the type name of the root node.

**Parameters:**

- `tree` (`const TsPackTree*`): tree handle.

**Returns:** `char*` -- newly-allocated type name string, or null if tree is null. Free with `ts_pack_free_string`.

### `uint32_t ts_pack_tree_root_child_count(const TsPackTree* tree)`

Get the number of named children of the root node.

**Parameters:**

- `tree` (`const TsPackTree*`): tree handle.

**Returns:** `uint32_t` -- child count, or 0 if tree is null.

### `bool ts_pack_tree_contains_node_type(const TsPackTree* tree, const char* node_type)`

Check whether any node in the tree has the given type name. Uses depth-first traversal.

**Parameters:**

- `tree` (`const TsPackTree*`): tree handle.
- `node_type` (`const char*`): null-terminated node type name to search for.

**Returns:** `bool` -- true if a node with the given type exists.

### `bool ts_pack_tree_has_error_nodes(const TsPackTree* tree)`

Check whether the tree contains any ERROR or MISSING nodes.

**Parameters:**

- `tree` (`const TsPackTree*`): tree handle.

**Returns:** `bool` -- true if the tree has error nodes.

### `size_t ts_pack_tree_error_count(const TsPackTree* tree)`

Return the count of ERROR and MISSING nodes in the tree.

**Parameters:**

- `tree` (`const TsPackTree*`): tree handle.

**Returns:** `size_t` -- error node count, or 0 if tree is null.

### `char* ts_pack_tree_to_sexp(const TsPackTree* tree)`

Get the S-expression representation of the tree.

**Parameters:**

- `tree` (`const TsPackTree*`): tree handle.

**Returns:** `char*` -- newly-allocated S-expression string, or null if tree is null. Free with `ts_pack_free_string`.

**Example:**

```c
char* sexp = ts_pack_tree_to_sexp(tree);
if (sexp) {
    printf("%s\n", sexp);
    ts_pack_free_string(sexp);
}
```

## Code Intelligence (Process)

### `char* ts_pack_process(const TsPackRegistry* registry, const char* source, size_t source_len, const char* config_json)`

Process source code and extract metadata and chunks as a JSON string.

**Parameters:**

- `registry` (`const TsPackRegistry*`): registry handle.
- `source` (`const char*`): source code buffer.
- `source_len` (`size_t`): length of the source buffer in bytes.
- `config_json` (`const char*`): null-terminated JSON configuration string.

**Config JSON fields:**

- `language` (string, required): the language name.
- `structure` (bool): extract functions, classes, etc.
- `imports` (bool): extract imports.
- `exports` (bool): extract exports.
- `comments` (bool): extract comments.
- `symbols` (bool): extract symbols.
- `docstrings` (bool): extract docstrings.
- `diagnostics` (bool): include diagnostics.
- `chunk_max_size` (number): maximum chunk size in bytes.

**Returns:** `char*` -- newly-allocated JSON string containing the process result, or null on error. Check `ts_pack_last_error()` on null. Free with `ts_pack_free_string`.

**Example:**

```c
const char* config = "{\"language\":\"python\",\"structure\":true,\"imports\":true}";
const char* code = "import os\ndef hello(): pass";
char* result = ts_pack_process(reg, code, strlen(code), config);
if (!result) {
    fprintf(stderr, "Process error: %s\n", ts_pack_last_error());
} else {
    printf("%s\n", result);
    ts_pack_free_string(result);
}
```

## Extraction Queries

### `char* ts_pack_extract(const char* source, size_t source_len, const char* config_json)`

Run extraction queries against source code using tree-sitter query patterns.

**Parameters:**

- `source` (`const char*`): source code buffer (does not need to be null-terminated).
- `source_len` (`size_t`): length of the source buffer in bytes.
- `config_json` (`const char*`): null-terminated JSON configuration string.

**Config JSON fields:**

- `language` (string, required): the language name.
- `patterns` (object, required): a map of pattern names to pattern definitions. Each pattern definition contains:
    - `query` (string): a tree-sitter query string.
    - `capture_output` (object): output format configuration.
    - `child_fields` (array): child field names to extract.
    - `max_results` (number or null): optional limit on matches.
    - `byte_range` (array or null): optional `[start, end]` byte range to restrict matching.

**Returns:** `char*` -- newly-allocated JSON string containing the extraction results, or null on error. Check `ts_pack_last_error()` on null. Free with `ts_pack_free_string`.

**Example:**

```c
const char* code = "def hello(): pass\ndef world(): pass";
const char* config = "{\"language\":\"python\",\"patterns\":{\"fns\":{\"query\":\"(function_definition name: (identifier) @fn_name)\",\"capture_output\":{},\"child_fields\":[],\"max_results\":null,\"byte_range\":null}}}";
char* result = ts_pack_extract(code, strlen(code), config);
if (!result) {
    fprintf(stderr, "Extract error: %s\n", ts_pack_last_error());
} else {
    printf("%s\n", result);
    ts_pack_free_string(result);
}
```

### `char* ts_pack_validate_extraction(const char* config_json)`

Validate extraction patterns without running them against source code. Useful for checking query syntax before executing.

**Parameters:**

- `config_json` (`const char*`): null-terminated JSON configuration string with the same shape as for `ts_pack_extract` (language + patterns).

**Returns:** `char*` -- newly-allocated JSON string containing validation results, or null on error. Check `ts_pack_last_error()` on null. Free with `ts_pack_free_string`.

**Example:**

```c
const char* config = "{\"language\":\"python\",\"patterns\":{\"fns\":{\"query\":\"(function_definition name: (identifier) @fn_name)\",\"capture_output\":{},\"child_fields\":[],\"max_results\":null,\"byte_range\":null}}}";
char* result = ts_pack_validate_extraction(config);
if (!result) {
    fprintf(stderr, "Validation error: %s\n", ts_pack_last_error());
} else {
    printf("Validation result: %s\n", result);
    ts_pack_free_string(result);
}
```

## Download API

These functions require the `download` feature to be enabled at compile time.

### `int32_t ts_pack_init(const char* config_json)`

Initialize the language pack with configuration, downloading parsers as needed.

**Parameters:**

- `config_json` (`const char*`): null-terminated JSON string, or null for defaults.

**Config JSON fields:**

- `cache_dir` (string): override default cache directory.
- `languages` (array of strings): languages to pre-download.
- `groups` (array of strings): language groups to pre-download.

**Returns:** `0` on success, `-1` on error.

### `int32_t ts_pack_configure(const char* config_json)`

Configure the language pack without downloading. Accepts the same JSON fields as `ts_pack_init` (only `cache_dir` is meaningful here).

**Returns:** `0` on success, `-1` on error.

### `int32_t ts_pack_download(const char** names, size_t count)`

Download specific languages to the cache.

**Parameters:**

- `names` (`const char**`): array of null-terminated language name strings.
- `count` (`size_t`): number of strings in the array.

**Returns:** number of newly downloaded languages on success, or `-1` on error.

### `int32_t ts_pack_download_all(void)`

Download all available languages from the remote manifest.

**Returns:** number of newly downloaded languages on success, or `-1` on error.

### `const char** ts_pack_manifest_languages(size_t* out_count)`

Get all language names available in the remote manifest.

**Parameters:**

- `out_count` (`size_t*`): receives the number of languages in the returned array.

**Returns:** newly-allocated array of language name strings, or null on error. Free each string with `ts_pack_free_string`, then free the array with `ts_pack_free_string_array`.

### `const char** ts_pack_downloaded_languages(size_t* out_count)`

Get all languages that are already downloaded and cached locally.

**Parameters:**

- `out_count` (`size_t*`): receives the number of languages in the returned array.

**Returns:** newly-allocated array of language name strings. Free each string with `ts_pack_free_string`, then free the array with `ts_pack_free_string_array`.

### `int32_t ts_pack_clean_cache(void)`

Delete all cached parser shared libraries.

**Returns:** `0` on success, `-1` on error.

### `char* ts_pack_cache_dir(void)`

Get the effective cache directory path.

**Returns:** `char*` -- newly-allocated path string, or null on error. Free with `ts_pack_free_string`.

## Memory Management

### `void ts_pack_free_string(char* s)`

Free a string returned by the FFI (e.g., from `ts_pack_language_name_at`, `ts_pack_tree_to_sexp`, `ts_pack_process`). Passing null is a safe no-op.

### `void ts_pack_free_string_array(const char** arr)`

Free a string array wrapper returned by the FFI (e.g., from `ts_pack_manifest_languages`). This frees only the array itself, not the individual strings. Free each string with `ts_pack_free_string` before calling this. Passing null is a safe no-op. Requires the `download` feature.

## Complete Example

```c
#include <stdio.h>
#include <string.h>
#include "ts_pack.h"

int main(void) {
    /* Create registry */
    TsPackRegistry* reg = ts_pack_registry_new();
    if (!reg) {
        fprintf(stderr, "Failed to create registry: %s\n", ts_pack_last_error());
        return 1;
    }

    /* List languages */
    size_t count = ts_pack_language_count(reg);
    printf("%zu languages available\n", count);

    /* Check for Python */
    if (!ts_pack_has_language(reg, "python")) {
        fprintf(stderr, "Python not available\n");
        ts_pack_registry_free(reg);
        return 1;
    }

    /* Parse source code */
    const char* code = "def hello(name):\n    print(f'Hello {name}')";
    TsPackTree* tree = ts_pack_parse_string(reg, "python", code, strlen(code));
    if (!tree) {
        fprintf(stderr, "Parse failed: %s\n", ts_pack_last_error());
        ts_pack_registry_free(reg);
        return 1;
    }

    /* Inspect tree */
    char* root_type = ts_pack_tree_root_node_type(tree);
    if (root_type) {
        printf("Root node type: %s\n", root_type);
        ts_pack_free_string(root_type);
    }

    printf("Root children: %u\n", ts_pack_tree_root_child_count(tree));
    printf("Has errors: %s\n", ts_pack_tree_has_error_nodes(tree) ? "yes" : "no");
    printf("Contains function_definition: %s\n",
           ts_pack_tree_contains_node_type(tree, "function_definition") ? "yes" : "no");

    /* S-expression */
    char* sexp = ts_pack_tree_to_sexp(tree);
    if (sexp) {
        printf("S-expression:\n%s\n", sexp);
        ts_pack_free_string(sexp);
    }

    /* Process for code intelligence */
    const char* config = "{\"language\":\"python\",\"structure\":true,\"imports\":true}";
    char* result = ts_pack_process(reg, code, strlen(code), config);
    if (result) {
        printf("Process result:\n%s\n", result);
        ts_pack_free_string(result);
    }

    /* Cleanup */
    ts_pack_tree_free(tree);
    ts_pack_registry_free(reg);
    return 0;
}
```

## Linking

### Static Library

```bash
gcc -o program program.c -L. -l:libts_pack_ffi.a
```

### Dynamic Library

```bash
gcc -o program program.c -L. -lts_pack_ffi
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
./program
```

### CMake Integration

```cmake
find_library(TS_PACK_FFI ts_pack_ffi REQUIRED)
add_executable(program program.c)
target_link_libraries(program ${TS_PACK_FFI})
target_include_directories(program PRIVATE /path/to/crates/ts-pack-ffi/include)
```
