---
id: fixture_ruby_process_python_comments
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("\# This is a comment\n\# Another comment\ndef hello():\n    \# inline comment\n    pass\n", { 'comments' => true, 'language' => 'python' })

```
