---
id: fixture_zig_data_extraction_xml_element
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
    const _result_json = try tree_sitter_language_pack.process("<server id=\"main\"><host>localhost</host></server>", "{\"data_extraction\":true,\"language\":\"xml\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
