---
id: fixture_zig_data_extraction_psv_rows
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
    const _result_json = try tree_sitter_language_pack.process("a|b|c\n1|2|3\n", "{\"data_extraction\":true,\"language\":\"psv\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
