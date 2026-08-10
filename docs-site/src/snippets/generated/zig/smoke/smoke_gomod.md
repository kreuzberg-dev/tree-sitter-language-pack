```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("module example.com/hello\n\ngo 1.21", "{\"language\":\"gomod\"}");
}

```
