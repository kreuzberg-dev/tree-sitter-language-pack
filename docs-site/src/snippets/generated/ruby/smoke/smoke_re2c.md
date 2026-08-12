---
id: fixture_ruby_smoke_re2c
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("/*!re2c\n  [a-z]+ { return; }\n*/", { 'language' => 're2c' })

```
