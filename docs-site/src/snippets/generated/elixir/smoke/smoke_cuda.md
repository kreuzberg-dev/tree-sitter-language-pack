---
id: fixture_elixir_smoke_cuda
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cuda"}
result = TreeSitterLanguagePack.process("__global__ void kernel() {}", config_value)

```
