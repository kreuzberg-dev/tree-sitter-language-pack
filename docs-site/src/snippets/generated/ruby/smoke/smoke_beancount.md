---
id: fixture_ruby_smoke_beancount
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('2024-01-01 open Assets:Bank USD', { 'language' => 'beancount' })

```
