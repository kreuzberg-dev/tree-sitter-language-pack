---
id: fixture_ruby_python_error_diagnostics
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("def broken(\n    pass\n", { 'diagnostics' => true, 'language' => 'python' })

```
