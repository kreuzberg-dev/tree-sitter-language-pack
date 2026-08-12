---
id: fixture_ruby_smoke_prolog
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("hello :- write('hello'), nl.", { 'language' => 'prolog' })

```
