---
id: fixture_ruby_smoke_wgsl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }', { 'language' => 'wgsl' })

```
