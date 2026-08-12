---
id: fixture_ruby_smoke_gosum
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('example.com/pkg v1.0.0 h1:abc=', { 'language' => 'gosum' })

```
