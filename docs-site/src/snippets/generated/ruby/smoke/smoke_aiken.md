---
id: fixture_ruby_smoke_aiken
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("fn main() {\n  1\n}\n", { 'language' => 'aiken' })

```
