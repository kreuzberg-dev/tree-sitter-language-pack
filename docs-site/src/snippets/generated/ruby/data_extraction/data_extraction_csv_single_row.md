---
id: fixture_ruby_data_extraction_csv_single_row
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("x,y,z\n", { 'data_extraction' => true, 'language' => 'csv' })

```
