---
id: fixture_ruby_smoke_magik
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("_method object.hello\n_endmethod", { 'language' => 'magik' })

```
