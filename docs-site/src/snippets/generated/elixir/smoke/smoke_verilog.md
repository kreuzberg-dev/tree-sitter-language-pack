---
id: fixture_elixir_smoke_verilog
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "verilog"}
result = TreeSitterLanguagePack.process("module main; endmodule", config_value)

```
