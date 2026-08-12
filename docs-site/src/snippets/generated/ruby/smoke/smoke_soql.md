---
id: fixture_ruby_smoke_soql
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("SELECT Id FROM Account\n", { 'language' => 'soql' })

```
