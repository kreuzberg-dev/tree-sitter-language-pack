---
id: fixture_ruby_smoke_hyprlang
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('general { border_size = 1 }', { 'language' => 'hyprlang' })

```
