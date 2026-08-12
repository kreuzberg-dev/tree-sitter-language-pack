---
id: fixture_elixir_data_extraction_dtd_element_decl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "dtd"}
result = TreeSitterLanguagePack.process("<!ELEMENT server (host, port)>\n<!ELEMENT host (\#PCDATA)>\n", config_value)

```
