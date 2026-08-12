---
id: fixture_ruby_smoke_netlinx
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("PROGRAM_NAME='hello'", { 'language' => 'netlinx' })

```
