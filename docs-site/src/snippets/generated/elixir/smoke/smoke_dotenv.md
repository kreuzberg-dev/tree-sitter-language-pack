---
id: fixture_elixir_smoke_dotenv
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dotenv"}
result = TreeSitterLanguagePack.process("KEY=value\n", config_value)

```
