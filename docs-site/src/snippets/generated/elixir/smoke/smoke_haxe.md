---
id: fixture_elixir_smoke_haxe
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haxe"}
result = TreeSitterLanguagePack.process("class Main { static function main() {} }", config_value)

```
