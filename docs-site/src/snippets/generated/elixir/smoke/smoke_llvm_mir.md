```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "llvm_mir"}
result = TreeSitterLanguagePack.process("---\nname: foo\n...\n", config_value)

```
