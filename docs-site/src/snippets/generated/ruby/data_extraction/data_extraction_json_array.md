---
id: fixture_ruby_data_extraction_json_array
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('[1, 2, 3]', { 'data_extraction' => true, 'language' => 'json' })

```
