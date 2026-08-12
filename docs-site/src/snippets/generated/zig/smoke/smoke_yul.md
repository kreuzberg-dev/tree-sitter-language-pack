---
id: fixture_zig_smoke_yul
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
    const _result_json = try tree_sitter_language_pack.process("object \"C\" {\n  code {\n  }\n}\n", "{\"language\":\"yul\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
