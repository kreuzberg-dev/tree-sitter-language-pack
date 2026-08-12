---
id: fixture_ruby_data_extraction_editorconfig_section
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("[*.rs]\nindent_style = space\nindent_size = 4\n", { 'data_extraction' => true, 'language' => 'editorconfig' })

```
