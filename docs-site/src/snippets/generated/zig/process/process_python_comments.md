```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", "{\"comments\":true,\"language\":\"python\"}");
}

```
