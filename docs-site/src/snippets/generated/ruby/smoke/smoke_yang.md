---
id: fixture_ruby_smoke_yang
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("module m {\n}\n", { 'language' => 'yang' })

```
