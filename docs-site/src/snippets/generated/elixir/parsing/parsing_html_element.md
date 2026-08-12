---
id: fixture_elixir_parsing_html_element
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "html"}
result = TreeSitterLanguagePack.process("<div>hello</div>", config_value)

```
