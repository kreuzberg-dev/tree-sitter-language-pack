---
id: fixture_elixir_smoke_matlab
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "matlab"}
result = TreeSitterLanguagePack.process("function y = hello(x)\ny = x;\nend", config_value)

```
