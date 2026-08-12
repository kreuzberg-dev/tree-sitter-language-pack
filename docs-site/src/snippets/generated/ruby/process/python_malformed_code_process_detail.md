---
id: fixture_ruby_python_malformed_code_process_detail
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("def broken(\n    return\nclass", { 'diagnostics' => true, 'language' => 'python' })

```
