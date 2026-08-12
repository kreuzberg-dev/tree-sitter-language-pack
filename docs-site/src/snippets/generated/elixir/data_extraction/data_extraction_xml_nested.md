---
id: fixture_elixir_data_extraction_xml_nested
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "xml"}
result = TreeSitterLanguagePack.process("<config><host>localhost</host><port>8080</port></config>", config_value)

```
