---
id: fixture_ruby_python_function_process_detail
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("def greet(name):\n    return f'Hello, {name}!'\n", { 'language' => 'python' })

```
