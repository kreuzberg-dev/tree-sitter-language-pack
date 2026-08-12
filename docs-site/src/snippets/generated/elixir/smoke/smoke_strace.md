---
id: fixture_elixir_smoke_strace
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "strace"}
result = TreeSitterLanguagePack.process("open(\"/x\", O_RDONLY) = 3\n", config_value)

```
