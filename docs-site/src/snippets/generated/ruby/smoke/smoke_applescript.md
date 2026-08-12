---
id: fixture_ruby_smoke_applescript
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("set x to 1\n", { 'language' => 'applescript' })

```
