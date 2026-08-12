---
id: fixture_ruby_data_extraction_cue_fields
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("host: \"localhost\"\nport: 8080\n", { 'data_extraction' => true, 'language' => 'cue' })

```
