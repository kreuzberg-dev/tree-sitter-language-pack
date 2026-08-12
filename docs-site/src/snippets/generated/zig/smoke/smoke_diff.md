---
id: fixture_zig_smoke_diff
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
    const _result_json = try tree_sitter_language_pack.process("--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", "{\"language\":\"diff\"}");
    defer std.heap.c_allocator.free(_result_json);
}

```
