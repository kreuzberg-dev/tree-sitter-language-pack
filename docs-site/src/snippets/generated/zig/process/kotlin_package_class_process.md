```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", "{\"language\":\"kotlin\"}");
}

```
