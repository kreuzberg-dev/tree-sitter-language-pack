---
id: fixture_zig_data_extraction_csv_single_row
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
    const _result_json = try tree_sitter_language_pack.process("x,y,z\n", "{\"data_extraction\":true,\"language\":\"csv\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
