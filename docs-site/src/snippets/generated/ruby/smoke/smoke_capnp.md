---
id: fixture_ruby_smoke_capnp
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('@0xabcdef1234567890;', { 'language' => 'capnp' })

```
