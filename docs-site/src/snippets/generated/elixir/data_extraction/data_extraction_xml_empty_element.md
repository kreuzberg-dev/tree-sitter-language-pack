---
id: fixture_elixir_data_extraction_xml_empty_element
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "xml"}
result = TreeSitterLanguagePack.process("<br/>", config_value)

```
