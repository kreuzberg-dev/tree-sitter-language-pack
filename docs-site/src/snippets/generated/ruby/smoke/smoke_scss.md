---
id: fixture_ruby_smoke_scss
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("$color: red;\nbody { color: $color; }", { 'language' => 'scss' })

```
