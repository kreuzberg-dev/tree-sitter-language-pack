---
id: fixture_elixir_smoke_llvm_mir
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "llvm_mir"}
result = TreeSitterLanguagePack.process("---\nname: foo\n...\n", config_value)

```
