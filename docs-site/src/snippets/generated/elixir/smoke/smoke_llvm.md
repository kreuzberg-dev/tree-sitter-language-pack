---
id: fixture_elixir_smoke_llvm
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "llvm"}
result = TreeSitterLanguagePack.process("define i32 @main() { ret i32 0 }", config_value)

```
