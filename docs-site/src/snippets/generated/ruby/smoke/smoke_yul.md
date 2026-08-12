---
id: fixture_ruby_smoke_yul
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("object \"C\" {\n  code {\n  }\n}\n", { 'language' => 'yul' })

```
