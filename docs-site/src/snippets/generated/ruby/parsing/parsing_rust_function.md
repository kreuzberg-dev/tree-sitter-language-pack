---
id: fixture_ruby_parsing_rust_function
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('fn main() {}', { 'language' => 'rust' })

```
