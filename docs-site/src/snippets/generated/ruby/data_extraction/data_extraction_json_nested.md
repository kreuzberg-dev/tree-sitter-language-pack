---
id: fixture_ruby_data_extraction_json_nested
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('{"server": {"host": "x", "port": 8080}}', { 'data_extraction' => true, 'language' => 'json' })

```
