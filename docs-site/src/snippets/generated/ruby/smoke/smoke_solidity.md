---
id: fixture_ruby_smoke_solidity
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("pragma solidity ^0.8.0;\ncontract Main {}", { 'language' => 'solidity' })

```
