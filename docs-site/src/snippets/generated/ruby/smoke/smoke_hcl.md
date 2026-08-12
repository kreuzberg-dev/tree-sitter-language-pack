---
id: fixture_ruby_smoke_hcl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('variable "name" { type = string }', { 'language' => 'hcl' })

```
