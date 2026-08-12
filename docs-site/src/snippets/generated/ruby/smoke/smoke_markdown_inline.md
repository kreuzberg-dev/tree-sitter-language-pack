---
id: fixture_ruby_smoke_markdown_inline
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('**bold** and *italic*', { 'language' => 'markdown_inline' })

```
