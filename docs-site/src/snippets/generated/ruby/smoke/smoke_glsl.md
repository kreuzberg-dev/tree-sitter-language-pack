---
id: fixture_ruby_smoke_glsl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('void main() { gl_Position = vec4(0.0); }', { 'language' => 'glsl' })

```
