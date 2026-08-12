---
id: fixture_ruby_data_extraction_dtd_element_decl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("<!ELEMENT server (host, port)>\n<!ELEMENT host (\#PCDATA)>\n", { 'data_extraction' => true, 'language' => 'dtd' })

```
