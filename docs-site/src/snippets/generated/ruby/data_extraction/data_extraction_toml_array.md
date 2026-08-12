---
id: fixture_ruby_data_extraction_toml_array
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("ports = [8080, 8081, 8082]\n", { 'data_extraction' => true, 'language' => 'toml' })

```
