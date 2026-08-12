---
id: fixture_ruby_smoke_idl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("module M {\n};\n", { 'language' => 'idl' })

```
