```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "leo"}
result = TreeSitterLanguagePack.process("program test.aleo {\n}\n", config_value)

```
