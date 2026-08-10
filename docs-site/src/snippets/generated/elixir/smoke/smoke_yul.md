```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "yul"}
result = TreeSitterLanguagePack.process("object \"C\" {\n  code {\n  }\n}\n", config_value)

```
