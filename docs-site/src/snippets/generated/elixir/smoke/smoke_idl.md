```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "idl"}
result = TreeSitterLanguagePack.process("module M {\n};\n", config_value)

```
