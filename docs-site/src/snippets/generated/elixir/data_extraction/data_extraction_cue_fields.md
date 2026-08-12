---
id: fixture_elixir_data_extraction_cue_fields
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "cue"}
result = TreeSitterLanguagePack.process("host: \"localhost\"\nport: 8080\n", config_value)

```
