---
id: fixture_ruby_smoke_llvm
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('define i32 @main() { ret i32 0 }', { 'language' => 'llvm' })

```
