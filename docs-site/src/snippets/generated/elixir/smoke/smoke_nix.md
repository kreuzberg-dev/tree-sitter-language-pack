---
id: fixture_elixir_smoke_nix
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "nix"}
result = TreeSitterLanguagePack.process("{ pkgs ? import <nixpkgs> {} }: pkgs.hello", config_value)

```
