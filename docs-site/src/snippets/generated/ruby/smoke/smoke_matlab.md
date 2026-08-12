---
id: fixture_ruby_smoke_matlab
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("function y = hello(x)\ny = x;\nend", { 'language' => 'matlab' })

```
