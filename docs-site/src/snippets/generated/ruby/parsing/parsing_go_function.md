---
id: fixture_ruby_parsing_go_function
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("package main\nfunc main() {}", { 'language' => 'go' })

```
