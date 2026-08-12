---
id: fixture_zig_smoke_tsv
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
    const _result_json = try tree_sitter_language_pack.process("a\tb\tc\n1\t2\t3", "{\"language\":\"tsv\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
