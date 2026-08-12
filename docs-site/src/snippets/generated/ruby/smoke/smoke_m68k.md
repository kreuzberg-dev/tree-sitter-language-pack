---
id: fixture_ruby_smoke_m68k
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process(" move.l d0,d1\n", { 'language' => 'm68k' })

```
