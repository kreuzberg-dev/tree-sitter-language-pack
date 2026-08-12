---
id: fixture_zig_process_python_docstrings
language: zig
target: zig
level: typecheck
requires: []
side_effect: safe
---

```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("def greet(name):\n    \"\"\"Say hello to someone.\"\"\"\n    return f\"Hello {name}\"\n", "{\"docstrings\":true,\"language\":\"python\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
