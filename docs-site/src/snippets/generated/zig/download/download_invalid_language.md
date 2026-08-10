```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const result = tree_sitter_language_pack.download("[\"zzz_definitely_not_a_real_language_xyz\"]") catch |err| {
        std.debug.print("call failed as expected: {s}\n", .{@errorName(err)});
        return;
    };
    _ = result;
}

```
