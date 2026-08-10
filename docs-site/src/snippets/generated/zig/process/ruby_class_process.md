```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello #{name}\"\n  end\nend\n", "{\"language\":\"ruby\"}");
}

```
