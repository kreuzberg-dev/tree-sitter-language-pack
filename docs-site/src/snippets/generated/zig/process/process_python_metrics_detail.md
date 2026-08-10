```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("# module docstring\nimport os\n\ndef hello():\n    # greeting\n    print('hello')\n\ndef world():\n    print('world')\n", "{\"language\":\"python\"}");
}

```
