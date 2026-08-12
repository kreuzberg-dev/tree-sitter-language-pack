---
id: fixture_ruby_smoke_penrose
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("type Set\n", { 'language' => 'penrose' })

```
