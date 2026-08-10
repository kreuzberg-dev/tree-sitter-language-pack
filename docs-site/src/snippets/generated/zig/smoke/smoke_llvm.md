```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("define i32 @main() { ret i32 0 }", "{\"language\":\"llvm\"}");
}

```
