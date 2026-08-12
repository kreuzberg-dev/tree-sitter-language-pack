---
id: fixture_ruby_smoke_yuck
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('(defwidget main [] (label :text "hi"))', { 'language' => 'yuck' })

```
