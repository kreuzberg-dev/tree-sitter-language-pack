{#
  Quick Start examples come from the alef-generated snippet corpus wherever the
  generator emits code that actually compiles.

  `crates.readme.snippets_dir` is `docs-site/src/snippets`, while the generated
  corpus lives one level down in `docs-site/src/snippets/generated/<lang>/...`,
  so the `include_snippet` filter is given `generated/<lang>` as its language
  argument. When `snippets_dir` is repointed at `docs-site/src/snippets/generated`
  the `"generated/" ~ ...` prefix here should be dropped.

  Rust, Zig, Kotlin/Android and the C FFI are inline below:
    - rust:           generated snippet has a malformed raw string literal
    - zig:            generated snippet binds an unused local const (compile error)
    - kotlin_android: generated snippet references an undefined `config`
    - ffi:            no `generated/ffi` (or `generated/c`) corpus exists at all
  Replace each with an `include_snippet` call once the generator output for that
  language compiles.
#}
{% if language == "rust" %}

```rust
use tree_sitter_language_pack::{ProcessConfig, get_parser, process};

fn main() -> Result<(), tree_sitter_language_pack::Error> {
    // Parsers are downloaded on first use and cached for every later call.
    let config = ProcessConfig::new("python").all();
    let result = process("def hello():\n    print('world')\n", &config)?;
    println!("Language: {}", result.language);
    println!("Functions: {}", result.structure.len());
    println!("Lines: {}", result.metrics.total_lines);

    // Or drop down to the tree-sitter parser directly.
    let mut parser = get_parser("python")?;
    if let Some(tree) = parser.parse("def hello(): pass") {
        println!("{}", tree.root_node().to_sexp());
    }
    Ok(())
}
```

{% elif language == "zig" %}

```zig
const std = @import("std");
const tslp = @import("tree_sitter_language_pack");

pub fn main() !void {
    // `process` returns the result as owned JSON, allocated with the C allocator.
    const result_json = try tslp.process(
        "def hello():\n    print('world')\n",
        "{\"language\":\"python\"}",
    );
    defer std.heap.c_allocator.free(result_json);

    std.debug.print("{s}\n", .{result_json});
}
```

{% elif language == "kotlin_android" %}

```kotlin
import io.xberg.tslp.android.ProcessConfig
import io.xberg.tslp.android.TreeSitterLanguagePack

fun main() {
    val config = ProcessConfig(language = "python")
    val result = TreeSitterLanguagePack.process("def hello():\n    print('world')\n", config)

    println("Language: ${result.language}")
    println("Functions: ${result.structure.size}")
}
```

{% elif language == "ffi" %}

```c
#include <stdio.h>
#include "ts_pack.h"

int main(void) {
    TS_PACKProcessConfig *config = ts_pack_process_config_from_json("{\"language\":\"python\"}");
    if (config == NULL) {
        fprintf(stderr, "config error: %s\n", ts_pack_last_error_context());
        return 1;
    }

    TS_PACKProcessResult *result = ts_pack_process("def hello():\n    print('world')\n", config);
    ts_pack_process_config_free(config);
    if (result == NULL) {
        fprintf(stderr, "process error: %s\n", ts_pack_last_error_context());
        return 1;
    }

    char *json = ts_pack_process_result_to_json(result);
    ts_pack_process_result_free(result);
    if (json == NULL) {
        fprintf(stderr, "serialize error: %s\n", ts_pack_last_error_context());
        return 1;
    }

    printf("%s\n", json);
    ts_pack_free_string(json);
    return 0;
}
```

{% elif language in ["python", "typescript", "go", "java", "csharp", "php", "ruby", "elixir", "swift", "dart", "wasm"] %}

{{ "process/config_all_python.md" | include_snippet("generated/" ~ language) }}

{% else %}
See the [language guide](https://docs.tree-sitter-language-pack.xberg.io) for `{{ language }}`-specific usage.
{% endif %}
