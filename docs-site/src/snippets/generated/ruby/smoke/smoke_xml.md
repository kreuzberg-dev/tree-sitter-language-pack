---
id: fixture_ruby_smoke_xml
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("<?xml version=\"1.0\"?>\n<root>hello</root>", { 'language' => 'xml' })

```
