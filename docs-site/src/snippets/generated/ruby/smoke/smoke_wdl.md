---
id: fixture_ruby_smoke_wdl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("version 1.0\n", { 'language' => 'wdl' })

```
