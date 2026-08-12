---
id: fixture_ruby_smoke_pgn
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('1. e4 e5 *', { 'language' => 'pgn' })

```
