---
id: fixture_ruby_smoke_t32
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("PRINT 1\n", { 'language' => 't32' })

```
