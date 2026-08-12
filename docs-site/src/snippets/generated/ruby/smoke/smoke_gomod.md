---
id: fixture_ruby_smoke_gomod
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("module example.com/hello\n\ngo 1.21", { 'language' => 'gomod' })

```
