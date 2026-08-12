---
id: fixture_ruby_kotlin_package_class_process
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", { 'language' => 'kotlin' })

```
