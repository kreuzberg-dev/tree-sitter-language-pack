---
id: fixture_ruby_smoke_racket
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("\#lang racket\n(define x 1)", { 'language' => 'racket' })

```
