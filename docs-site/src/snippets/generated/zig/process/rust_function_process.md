```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", "{\"language\":\"rust\"}");
}

```
