---
id: fixture_ruby_smoke_requirements
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('flask>=2.0', { 'language' => 'requirements' })

```
