---
id: fixture_ruby_injections_query_unknown_language
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.get_injections_query('nonexistent_xyz')

```
