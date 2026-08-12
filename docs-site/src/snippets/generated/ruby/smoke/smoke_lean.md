---
id: fixture_ruby_smoke_lean
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('def main : IO Unit := pure ()', { 'language' => 'lean' })

```
