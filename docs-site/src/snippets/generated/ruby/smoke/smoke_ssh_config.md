---
id: fixture_ruby_smoke_ssh_config
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("Host example\n  HostName example.com", { 'language' => 'ssh_config' })

```
