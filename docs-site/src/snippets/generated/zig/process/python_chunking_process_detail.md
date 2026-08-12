---
id: fixture_zig_python_chunking_process_detail
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
    const _result_json = try tree_sitter_language_pack.process("def alpha():\n    pass\n\ndef beta():\n    pass\n\ndef gamma():\n    pass\n\ndef delta():\n    pass\n", "{\"chunk_max_size\":30,\"language\":\"python\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
