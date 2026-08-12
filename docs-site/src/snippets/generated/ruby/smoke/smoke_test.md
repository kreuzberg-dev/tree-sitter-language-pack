---
id: fixture_ruby_smoke_test
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("===========\nTest\n===========\n---\n(node)", { 'language' => 'test' })

```
