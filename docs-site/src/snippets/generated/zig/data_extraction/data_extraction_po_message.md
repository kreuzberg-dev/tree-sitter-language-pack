---
id: fixture_zig_data_extraction_po_message
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
    const _result_json = try tree_sitter_language_pack.process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", "{\"data_extraction\":true,\"language\":\"po\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
