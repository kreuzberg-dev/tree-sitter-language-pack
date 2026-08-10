```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("void main() { gl_Position = vec4(0.0); }", "{\"language\":\"glsl\"}");
}

```
