---
id: fixture_zig_parse_empty_language
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
    if (tree_sitter_language_pack.process("x = 1", "{\"language\":\"\"}")) |_| {
        return error.TestUnexpectedResult;
    } else |err| { std.debug.print("call failed as expected: {s}\n", .{@errorName(err)}); }
}

```
