---
id: fixture_ruby_data_extraction_toml_table
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("[server]\nhost = \"localhost\"\nport = 8080\n", { 'data_extraction' => true, 'language' => 'toml' })

```
