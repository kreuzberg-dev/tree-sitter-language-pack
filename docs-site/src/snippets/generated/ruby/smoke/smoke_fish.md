---
id: fixture_ruby_smoke_fish
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('echo hello', { 'language' => 'fish' })

```
