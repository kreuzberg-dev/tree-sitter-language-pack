```zig title="Zig"
const std = @import("std");
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

pub fn main() !void {
    const _result_json = try tree_sitter_language_pack.process("Feature: Calculator\n  Scenario: Add numbers\n    Given I have entered 1\n    When I add 2\n    Then the result should be 3\n", "{\"language\":\"gherkin\"}");
}

```
