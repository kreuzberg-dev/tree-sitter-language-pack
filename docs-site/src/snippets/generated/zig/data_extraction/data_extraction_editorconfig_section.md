---
id: fixture_zig_data_extraction_editorconfig_section
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
    const _result_json = try tree_sitter_language_pack.process("[*.rs]\nindent_style = space\nindent_size = 4\n", "{\"data_extraction\":true,\"language\":\"editorconfig\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
