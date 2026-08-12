---
id: fixture_zig_data_extraction_json_flat
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
    const _result_json = try tree_sitter_language_pack.process("{\"host\": \"localhost\", \"port\": 8080}", "{\"data_extraction\":true,\"language\":\"json\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
