---
id: fixture_ruby_ruby_class_process
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello \#{name}\"\n  end\nend\n", { 'language' => 'ruby' })

```
