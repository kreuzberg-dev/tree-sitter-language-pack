---
id: fixture_elixir_data_extraction_json_nested
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "json"}
result = TreeSitterLanguagePack.process("{\"server\": {\"host\": \"x\", \"port\": 8080}}", config_value)

```
