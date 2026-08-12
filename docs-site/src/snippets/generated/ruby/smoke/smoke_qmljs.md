---
id: fixture_ruby_smoke_qmljs
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("import QtQuick 2.0\nItem {}", { 'language' => 'qmljs' })

```
