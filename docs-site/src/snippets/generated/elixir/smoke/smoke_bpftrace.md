```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "bpftrace"}
result = TreeSitterLanguagePack.process("BEGIN { }\n", config_value)

```
