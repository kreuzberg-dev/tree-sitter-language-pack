```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", "{\"language\":\"java\"}");
}

```
