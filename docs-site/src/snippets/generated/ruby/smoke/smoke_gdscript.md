---
id: fixture_ruby_smoke_gdscript
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("extends Node\nfunc _ready():\n\tpass", { 'language' => 'gdscript' })

```
