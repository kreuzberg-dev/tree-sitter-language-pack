---
id: fixture_ruby_process_python_metrics_detail
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("\# module docstring\nimport os\n\ndef hello():\n    \# greeting\n    print('hello')\n\ndef world():\n    print('world')\n", { 'language' => 'python' })

```
