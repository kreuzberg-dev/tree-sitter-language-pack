```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def subtract(self, a, b):\n        return a - b\n", "{\"language\":\"python\"}");
}

```
