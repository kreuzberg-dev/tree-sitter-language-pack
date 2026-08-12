---
id: fixture_ruby_smoke_ungrammar
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("Root = Item*\nItem = 'token'", { 'language' => 'ungrammar' })

```
