---
id: fixture_ruby_data_extraction_properties_dotted_key
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("server.host=localhost\nserver.port=8080\n", { 'data_extraction' => true, 'language' => 'properties' })

```
