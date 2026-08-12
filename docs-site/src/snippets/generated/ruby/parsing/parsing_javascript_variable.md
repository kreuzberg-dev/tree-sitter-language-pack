---
id: fixture_ruby_parsing_javascript_variable
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('const x = 1;', { 'language' => 'javascript' })

```
