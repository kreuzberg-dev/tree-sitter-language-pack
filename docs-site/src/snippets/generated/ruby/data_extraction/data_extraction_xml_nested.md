---
id: fixture_ruby_data_extraction_xml_nested
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('<config><host>localhost</host><port>8080</port></config>', { 'data_extraction' => true, 'language' => 'xml' })

```
