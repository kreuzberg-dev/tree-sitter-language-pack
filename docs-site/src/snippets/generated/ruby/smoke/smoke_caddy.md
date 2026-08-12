---
id: fixture_ruby_smoke_caddy
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process(":8080 {\n\trespond \"Hello\"\n}", { 'language' => 'caddy' })

```
