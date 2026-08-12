---
id: fixture_elixir_smoke_tsx
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tsx"}
result = TreeSitterLanguagePack.process("const App = () => <div />;", config_value)

```
