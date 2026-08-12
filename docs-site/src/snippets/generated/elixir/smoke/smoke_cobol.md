---
id: fixture_elixir_smoke_cobol
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cobol"}
result = TreeSitterLanguagePack.process("       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", config_value)

```
