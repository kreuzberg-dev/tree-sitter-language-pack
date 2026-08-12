---
id: fixture_elixir_smoke_pem
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pem"}
result = TreeSitterLanguagePack.process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", config_value)

```
