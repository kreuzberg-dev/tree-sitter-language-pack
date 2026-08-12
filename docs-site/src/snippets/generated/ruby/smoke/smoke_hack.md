---
id: fixture_ruby_smoke_hack
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("<?hh\nfunction main(): void {}", { 'language' => 'hack' })

```
