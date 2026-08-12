---
id: fixture_ruby_smoke_prisma
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('model User { id Int @id }', { 'language' => 'prisma' })

```
