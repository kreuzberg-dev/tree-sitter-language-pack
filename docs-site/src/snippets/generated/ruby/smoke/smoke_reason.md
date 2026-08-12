---
id: fixture_ruby_smoke_reason
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("let x = 1;\n", { 'language' => 'reason' })

```
