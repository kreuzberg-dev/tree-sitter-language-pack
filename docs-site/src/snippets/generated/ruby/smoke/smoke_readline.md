---
id: fixture_ruby_smoke_readline
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('set editing-mode vi', { 'language' => 'readline' })

```
