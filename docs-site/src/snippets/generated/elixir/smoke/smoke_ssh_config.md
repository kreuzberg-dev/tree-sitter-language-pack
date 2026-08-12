---
id: fixture_elixir_smoke_ssh_config
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ssh_config"}
result = TreeSitterLanguagePack.process("Host example\n  HostName example.com", config_value)

```
