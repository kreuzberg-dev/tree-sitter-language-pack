---
id: fixture_elixir_data_extraction_yaml_nested
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "yaml"}
result = TreeSitterLanguagePack.process("server:\n  host: localhost\n  port: 8080\n", config_value)

```
