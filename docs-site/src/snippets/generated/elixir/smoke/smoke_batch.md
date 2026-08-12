---
id: fixture_elixir_smoke_batch
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "batch"}
result = TreeSitterLanguagePack.process("@echo off\necho hello", config_value)

```
