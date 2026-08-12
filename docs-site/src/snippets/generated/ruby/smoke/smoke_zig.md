---
id: fixture_ruby_smoke_zig
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('pub fn main() void {}', { 'language' => 'zig' })

```
