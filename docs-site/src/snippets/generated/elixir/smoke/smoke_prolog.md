---
id: fixture_elixir_smoke_prolog
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "prolog"}
result = TreeSitterLanguagePack.process("hello :- write('hello'), nl.", config_value)

```
