---
id: fixture_ruby_smoke_tsv
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("a\tb\tc\n1\t2\t3", { 'language' => 'tsv' })

```
