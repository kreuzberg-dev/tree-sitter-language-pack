---
id: fixture_elixir_smoke_asciidoc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "asciidoc"}
result = TreeSitterLanguagePack.process("= Title\n\nParagraph.", config_value)

```
