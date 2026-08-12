---
id: fixture_ruby_smoke_clarity
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('(define-public (hello) (ok true))', { 'language' => 'clarity' })

```
