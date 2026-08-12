---
id: fixture_elixir_data_extraction_editorconfig_section
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "editorconfig"}
result = TreeSitterLanguagePack.process("[*.rs]\nindent_style = space\nindent_size = 4\n", config_value)

```
