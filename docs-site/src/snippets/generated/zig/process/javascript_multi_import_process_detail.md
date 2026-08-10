```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("import fs from 'fs';\nimport path from 'path';\n\nfunction process(input) {\n    return input.trim();\n}\n", "{\"language\":\"javascript\"}");
}

```
