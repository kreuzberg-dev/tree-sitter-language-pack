---
id: fixture_ruby_error_process_empty_source
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('', { 'language' => 'python' })

```
