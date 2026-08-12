---
id: fixture_ruby_smoke_cypher
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("MATCH (n) RETURN n\n", { 'language' => 'cypher' })

```
