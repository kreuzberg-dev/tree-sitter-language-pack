---
id: fixture_ruby_data_extraction_kdl_nested
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("server {\n  host \"localhost\"\n  port 8080\n}\n", { 'data_extraction' => true, 'language' => 'kdl' })

```
