---
id: fixture_ruby_smoke_koka
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("fun main()\n  1\n", { 'language' => 'koka' })

```
