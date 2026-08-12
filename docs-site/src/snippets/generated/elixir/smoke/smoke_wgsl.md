---
id: fixture_elixir_smoke_wgsl
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "wgsl"}
result = TreeSitterLanguagePack.process("@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }", config_value)

```
