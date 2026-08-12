---
id: fixture_ruby_data_extraction_xml_empty_element
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('<br/>', { 'data_extraction' => true, 'language' => 'xml' })

```
