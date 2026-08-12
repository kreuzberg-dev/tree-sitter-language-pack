---
id: fixture_ruby_indents_query_unknown_language
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.get_indents_query('nonexistent_xyz')

```
