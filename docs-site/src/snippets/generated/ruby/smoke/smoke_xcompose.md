---
id: fixture_ruby_smoke_xcompose
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('<Multi_key> <a> : "a"', { 'language' => 'xcompose' })

```
