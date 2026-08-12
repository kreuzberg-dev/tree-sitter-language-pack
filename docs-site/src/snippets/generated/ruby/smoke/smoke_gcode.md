---
id: fixture_ruby_smoke_gcode
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("G0 X0\n", { 'language' => 'gcode' })

```
