---
id: fixture_elixir_data_extraction_properties_dotted_key
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "properties"}
result = TreeSitterLanguagePack.process("server.host=localhost\nserver.port=8080\n", config_value)

```
