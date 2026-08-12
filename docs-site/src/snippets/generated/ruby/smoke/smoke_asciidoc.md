---
id: fixture_ruby_smoke_asciidoc
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("= Title\n\nParagraph.", { 'language' => 'asciidoc' })

```
