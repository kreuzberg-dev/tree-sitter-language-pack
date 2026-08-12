---
id: fixture_zig_download_invalid_language
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
    if (tree_sitter_language_pack.download("[\"zzz_definitely_not_a_real_language_xyz\"]")) |_| {
        return error.TestUnexpectedResult;
    } else |err| { std.debug.print("call failed as expected: {s}\n", .{@errorName(err)}); }
}

```
