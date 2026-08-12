---
id: fixture_ruby_data_extraction_csv_rows
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("a,b,c\n1,2,3\n", { 'data_extraction' => true, 'language' => 'csv' })

```
