```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "lean"}
result = TreeSitterLanguagePack.process("def main : IO Unit := pure ()", config_value)

```
