---
id: fixture_ruby_config_minimal_python
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("def hello():\n    pass\n", { 'language' => 'python' })

```
