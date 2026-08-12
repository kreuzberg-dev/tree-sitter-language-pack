---
id: fixture_ruby_smoke_nix
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process('{ pkgs ? import <nixpkgs> {} }: pkgs.hello', { 'language' => 'nix' })

```
