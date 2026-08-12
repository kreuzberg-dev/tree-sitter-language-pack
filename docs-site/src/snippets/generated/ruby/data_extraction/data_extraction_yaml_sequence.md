---
id: fixture_ruby_data_extraction_yaml_sequence
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("ports:\n  - 8080\n  - 8081\n", { 'data_extraction' => true, 'language' => 'yaml' })

```
