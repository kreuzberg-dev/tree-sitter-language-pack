```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "verilog"}
result = TreeSitterLanguagePack.process("module main; endmodule", config_value)

```
