```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("Feature: Calculator\n  Scenario: Add numbers\n    Given I have entered 1\n    When I add 2\n    Then the result should be 3\n", { 'language' => 'gherkin' })

```
