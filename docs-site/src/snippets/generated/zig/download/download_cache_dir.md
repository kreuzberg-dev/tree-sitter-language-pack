---
id: fixture_zig_download_cache_dir
language: zig
target: zig
level: typecheck
requires: []
side_effect: safe
---

```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    _ = try tree_sitter_language_pack.cache_dir();
}

```
