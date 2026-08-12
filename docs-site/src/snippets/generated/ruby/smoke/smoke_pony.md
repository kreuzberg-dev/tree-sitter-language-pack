---
id: fixture_ruby_smoke_pony
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("actor Main\n  new create(env: Env) => None", { 'language' => 'pony' })

```
