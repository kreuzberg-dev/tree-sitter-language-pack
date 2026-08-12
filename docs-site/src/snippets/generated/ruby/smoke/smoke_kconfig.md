---
id: fixture_ruby_smoke_kconfig
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("config FOO\n\tbool \"Enable foo\"", { 'language' => 'kconfig' })

```
