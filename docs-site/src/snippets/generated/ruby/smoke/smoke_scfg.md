---
id: fixture_ruby_smoke_scfg
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("key value\n", { 'language' => 'scfg' })

```
