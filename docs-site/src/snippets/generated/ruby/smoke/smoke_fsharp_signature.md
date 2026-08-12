---
id: fixture_ruby_smoke_fsharp_signature
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('val x: int', { 'language' => 'fsharp_signature' })

```
