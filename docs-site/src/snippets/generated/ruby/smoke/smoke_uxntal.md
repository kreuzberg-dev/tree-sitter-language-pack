---
id: fixture_ruby_smoke_uxntal
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('|0100 LIT 01', { 'language' => 'uxntal' })

```
