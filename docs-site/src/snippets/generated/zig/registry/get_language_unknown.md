---
id: fixture_zig_get_language_unknown
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
    if (tree_sitter_language_pack.get_language("nonexistent_xyz")) |_| {
        return error.TestUnexpectedResult;
    } else |err| { std.debug.print("call failed as expected: {s}\n", .{@errorName(err)}); }
}

```
