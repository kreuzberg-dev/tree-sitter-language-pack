---
id: fixture_ruby_smoke_terraform
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('resource "null_resource" "main" {}', { 'language' => 'terraform' })

```
