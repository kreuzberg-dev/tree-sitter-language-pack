---
id: fixture_ruby_smoke_cmake
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('cmake_minimum_required(VERSION 3.0)', { 'language' => 'cmake' })

```
