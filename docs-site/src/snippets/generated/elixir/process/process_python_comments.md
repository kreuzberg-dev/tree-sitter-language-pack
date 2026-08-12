---
id: fixture_elixir_process_python_comments
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{comments: true, language: "python"}
result = TreeSitterLanguagePack.process("\# This is a comment\n\# Another comment\ndef hello():\n    \# inline comment\n    pass\n", config_value)

```
