---
id: fixture_ruby_smoke_hlsl
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('float4 main() : SV_Target { return 0; }', { 'language' => 'hlsl' })

```
