---
id: fixture_ruby_smoke_llvm_mir
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("---\nname: foo\n...\n", { 'language' => 'llvm_mir' })

```
