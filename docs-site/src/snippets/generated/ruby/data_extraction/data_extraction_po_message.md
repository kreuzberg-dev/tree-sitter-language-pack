---
id: fixture_ruby_data_extraction_po_message
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", { 'data_extraction' => true, 'language' => 'po' })

```
