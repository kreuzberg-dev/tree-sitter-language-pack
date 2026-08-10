```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n", "{\"chunk_max_size\":30,\"language\":\"rust\"}");
}

```
