---
id: fixture_ruby_smoke_strace
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("open(\"/x\", O_RDONLY) = 3\n", { 'language' => 'strace' })

```
