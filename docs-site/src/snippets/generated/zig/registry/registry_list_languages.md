```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    _ = try tree_sitter_language_pack.available_languages();
}

```
