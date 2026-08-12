---
id: fixture_elixir_smoke_xml
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "xml"}
result = TreeSitterLanguagePack.process("<?xml version=\"1.0\"?>\n<root>hello</root>", config_value)

```
