---
id: fixture_zig_smoke_psv
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
    const _result_json = try tree_sitter_language_pack.process("a|b|c\n1|2|3", "{\"language\":\"psv\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
