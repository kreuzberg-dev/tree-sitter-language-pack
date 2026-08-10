```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "llvm"}
result = TreeSitterLanguagePack.process("define i32 @main() { ret i32 0 }", config_value)

```
